/** @type {import('dependency-cruiser').IConfiguration} */
const fs = require("fs");
const { builtinModules } = require("module");
const path = require("path");

const TECHNOLOGIES = ["compose", "🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"];
const BOOTSTRAP_TOOLING_ENTRY_PATH = "(^|/)(?:📜️script\\.ts|(?:⚙️|🧪️)?(?:vite|vitest)\\.config\\.[cm]?[jt]s)$";
const RENDERER_HOST_ROOT = "^🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/(📦️packages/🟦️typescript/🎯️targets/⚛️react|🧱️elements)/";
const RESOLVED_NODE_BUILTIN_PATH = `^(?:${[...new Set(builtinModules.map((name) => name.replace(/^node:/, "")))].map(escapeRegex).join("|")})(?:$|/)`;
const RENDERER_HOST_ALLOWED_RESOLVED_PATHS = [
  RENDERER_HOST_ROOT,
  "^🧰️framework/🔨️modules/🖱️ui/",
  "^🧰️framework/📦️packages/",
  "^node_modules/react(?:-dom)?/",
  "^(?:node:|vitest/)",
  "^node_modules/(?:vite|vitest)/",
  RESOLVED_NODE_BUILTIN_PATH,
];

/** 🔌️ Derived from the live `✏️s/🔌️plugins` directory listing rather than hardcoded, so the
 * cross-plugin isolation matrix below self-corrects as plugins are added, renamed, or removed. */
const PLUGINS = fs
  .readdirSync(path.join(__dirname, "✏️s/🔌️plugins"), { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .map((entry) => entry.name);

/** 🔣️ `forbiddenPathSegments` (both `⚡️implementations` spellings) read from the M1 shared vocabulary
 * (`26/08/06/MECHANISM-VOCABULARY-AND-DISCOVERY-LIBRARY`) rather than re-hardcoded here, so this config
 * and the registry/root-policy scripts can never drift on which spellings are banned. Plain JSON require —
 * no TS toolchain needed from this plain `.cjs` config. */
const TAXONOMY = JSON.parse(
  fs.readFileSync(path.join(__dirname, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"),
);

/** ⚙️ Escapes a literal string for embedding inside a `RegExp` alternation. */
function escapeRegex(literal) {
  return literal.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/** 🧪️ Fails config loading if the two resolver-boundary allowlists regress into broad source or vendor exclusions. */
function assertFocusedBoundarySemantics() {
  const bootstrap = new RegExp(BOOTSTRAP_TOOLING_ENTRY_PATH, "u");
  const approvedBootstrap = [
    "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📜️script.ts",
    "compose/client/lib/sketchpad/doc/js/vite.config.ts",
    "♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts",
    "♻️mit-bestand/🎤️präsentation/📦️packages/🟦️typescript/🧪️vitest.config.ts",
  ];
  const runtimeSources = [
    "compose/client/lib/sketchpad/js/boot.tsx",
    "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📦️index.ts",
    "🌎️hub/🔨️modules/example/🟦️component.ts",
    "♻️mit-bestand/🧺️demonstrator/🤖️generated/🟦️plugins.ts",
    "♻️mit-bestand/🧺️demonstrator/runtime.config.ts",
  ];
  if (approvedBootstrap.some((candidate) => !bootstrap.test(candidate)) || runtimeSources.some((candidate) => bootstrap.test(candidate))) {
    throw new Error("dependency-cruiser bootstrap-tooling boundary is broader or narrower than its approved entry points");
  }
  const crossRules = crossTechnologyRules();
  if (crossRules.length !== TECHNOLOGIES.length * (TECHNOLOGIES.length - 1) || crossRules.some((rule) => rule.from.pathNot !== BOOTSTRAP_TOOLING_ENTRY_PATH)) {
    throw new Error("dependency-cruiser bootstrap-tooling boundary is not applied to every cross-technology direction");
  }

  const rendererRule = rendererHostsOnlyUiRule();
  if (rendererRule.from.path !== RENDERER_HOST_ROOT || rendererRule.to.pathNot !== RENDERER_HOST_ALLOWED_RESOLVED_PATHS) {
    throw new Error("dependency-cruiser renderer-host rule is not wired to its resolved-path allowlist");
  }
  const rendererAllows = rendererRule.to.pathNot.map((pattern) => new RegExp(pattern, "u"));
  const allowedRendererTargets = [
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🟦️component.tsx",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📦️index.ts",
    "🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts",
    "node_modules/react/index.js",
    "node_modules/react-dom/client.js",
    "node_modules/vitest/dist/index.js",
    "vitest/importMeta",
    "fs",
    "node:path",
  ];
  const forbiddenRendererTargets = [
    "node_modules/three/build/three.module.js",
    "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts",
    "🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️glue.ts",
    "♻️mit-bestand/🧺️demonstrator/🟦️brand.ts",
  ];
  const allowed = (candidate) => rendererAllows.some((pattern) => pattern.test(candidate));
  if (allowedRendererTargets.some((candidate) => !allowed(candidate)) || forbiddenRendererTargets.some(allowed)) {
    throw new Error("dependency-cruiser renderer-host allowlist no longer distinguishes presentation dependencies from runtime ownership breaches");
  }
}

assertFocusedBoundarySemantics();

/** 📦️ Recursively scans a directory for `package.json` files, skipping the same build-artifact/vendor
 * directories `options.doNotFollow` below already excludes (plus `pkg/`, a wasm-pack output dir that
 * duplicates its owning Rust package's name — see `noCorePathRule`'s `pathNot` for the same exclusion).
 * Self-deriving, like `PLUGINS` above, so the name-based layering rules below never hardcode a package
 * list that can drift from the real tree. Returns `{ dir, name }` pairs where `dir` is the package.json's
 * containing directory as a repo-relative POSIX path. */
function scanPackageJsonFiles(rootAbsDir) {
  const SKIP_DIRS = new Set(["node_modules", "dist", "target", "pkg", "storybook-static", ".git", ".nx", "🦑️repo", "repo"]);
  const results = [];
  if (!fs.existsSync(rootAbsDir)) return results;
  const stack = [rootAbsDir];
  while (stack.length) {
    const dir = stack.pop();
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      if (entry.isDirectory()) {
        if (SKIP_DIRS.has(entry.name)) continue;
        stack.push(path.join(dir, entry.name));
        continue;
      }
      if (entry.name !== "package.json") continue;
      const pkg = JSON.parse(fs.readFileSync(path.join(dir, entry.name), "utf8"));
      if (!pkg.name) continue;
      results.push({ dir: path.relative(__dirname, dir).split(path.sep).join("/"), name: pkg.name });
    }
  }
  return results;
}

/** ✏️ Every `package.json` found anywhere under `✏️s`, used to derive the `@semio-tech/*` package-name
 * equivalents for the path-based layering rules below (`framework-no-s`, `s-modules-no-plugins`,
 * `no-plugin-to-extension`) — a relative import and an npm-name import of the same code must both trip
 * the rule, so each rule's `to` combines a path pattern with these derived name patterns. */
const S_PACKAGES = scanPackageJsonFiles(path.join(__dirname, "✏️s"));

/** 🧰️ Every `package.json` found anywhere under `🧰️framework`, used the same way `S_PACKAGES` is above —
 * package-name equivalents for `pluginsFrameworkSdkOnlyRule`'s `to`, so a bare `@semio-tech/framework-os`
 * (etc.) import trips the rule even where dependency-cruiser doesn't resolve it down to the real file path. */
const FRAMEWORK_PACKAGES = scanPackageJsonFiles(path.join(__dirname, "🧰️framework"));

/** 🥾️ Keeps product-runtime technology edges forbidden while exempting only executable bootstrap and Vite/Vitest configuration entry points. */
function crossTechnologyRules() {
  const rules = [];
  for (const from of TECHNOLOGIES) {
    for (const to of TECHNOLOGIES) {
      if (from === to) continue;
      rules.push({
        name: `no-cross-technology-${from}-to-${to}`,
        severity: "error",
        comment: "Runtime relative imports must not cross top-level technology folders; bootstrap scripts and approved Vite/Vitest config entry points may consume repo tooling",
        from: { path: `^${from}/`, pathNot: BOOTSTRAP_TOOLING_ENTRY_PATH },
        to: {
          path: `^${to}/`,
          dependencyTypes: ["local"],
        },
      });
    }
  }
  return rules;
}

/** 🔌️ Plugins must not relative-import a sibling plugin's implementation — cross-plugin sharing goes
 * through @semio-tech packages or a framework/s module. `flow` is exempt (media-graph canvas embed). */
function crossPluginRules() {
  const rules = [];
  for (const from of PLUGINS) {
    for (const to of PLUGINS) {
      if (from === to || to === "🌊️flow") continue;
      rules.push({
        name: `no-cross-plugin-${from}-to-${to}`,
        severity: "error",
        comment: "Relative imports must not cross plugin folders; use @semio-tech packages",
        from: { path: `^✏️s/🔌️plugins/${from}/` },
        to: {
          path: `^✏️s/🔌️plugins/${to}/`,
          dependencyTypes: ["local"],
        },
      });
    }
  }
  return rules;
}

/** 🌳️ Step 7 of the spicy-umbrella mechanism wave (`26/08/06/DEPENDENCY-CRUISER-CONFIG-MODERNIZATION-FOR-TAXONOMY-SHAPE`):
 * forbids any dependency whose RESOLVED path still carries a `⚡️implementations`/`⚡️implementation`
 * segment (Shape V2 tree purity, both spellings, from `🔣️taxonomy.json`'s `forbiddenPathSegments`).
 * WARN, not error — plugins are fully retrofitted but framework/hub/mit-bestand haven't been touched by
 * this initiative yet, so real hits are EXPECTED here; promotion to error is W10 finalization's job, not
 * this ticket's. */
function noImplSegmentRule() {
  const alternation = TAXONOMY.forbiddenPathSegments.map((segment) => segment.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("|");
  return {
    name: "no-impl-segment",
    severity: "warn",
    comment: "Shape V2 tree purity: no dependency path may carry a ⚡️implementations/⚡️implementation segment — WARN until the W10 finalization flip promotes this to error",
    from: {},
    to: { path: `(^|/)(${alternation})(/|$)` },
  };
}

/** 🚫️ Bans dependency paths whose emoji-stripped segment is a banned stem (`core`, `shared`, …). */
function noCorePathRule() {
  const stems = (TAXONOMY.bannedNameStems || ["core"]).map((segment) => segment.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("|");
  return {
    name: "no-core-path",
    severity: "error",
    comment: "Clean mechanism: no dependency may resolve through a banned name stem folder (core/shared/util/…) — ERROR after Wave 4 core dissolve",
    from: {
      path: "^(✏️s/|🧰️framework/|🌎️hub/|♻️mit-bestand/)",
    },
    to: {
      path: `(^|/)([^/]*?)(${stems})(/|$)`,
      pathNot: "node_modules|target|/pkg/",
    },
  };
}

/** 📦️ Step 7's "`$1`-capture rule": a relative (`local`) import may freely reach anywhere inside its OWN
 * package/module family — same directory tree, any depth — but must not resolve into a SIBLING family via
 * a deep relative path; cross-family reuse goes through a `@semio-tech/…` package-name import instead.
 * "Family" is approximated with a path-segment heuristic (chosen over wiring up M1's `discoverPackages()`
 * here: that library is an ESM/TS module meant for the registry/root-policy TS scripts, and importing it
 * into this plain `.cjs` config would need a build step for no real gain — the taxonomy's actual package
 * unit, `<owner>/📦️packages/<lang>/`, is Shape V2 end-state and most of these areas are still legacy
 * sandwiches today, so a `📦️packages`-anchored capture would simply fail to match almost anything yet;
 * the directory-family heuristic below already covers the real, present-day gap: `✏️s/🔌️plugins/*` cross
 * imports are already an ERROR via `crossPluginRules`, so plugins are deliberately left out here to avoid
 * a redundant WARN — the gap this rule actually closes is *within* 🧰️framework (product-to-product,
 * module-to-module), ✏️s/🔨️modules (s-module-to-s-module), 🌎️hub/🔨️modules, and ♻️mit-bestand
 * (item-to-item), none of which any existing rule reaches).
 * `📜️script.ts` itself is exempt (`pathNot` below): every such bootstrap script across the repo already
 * relative-imports repo-lib (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/…`) across family boundaries —
 * the SAME sanctioned pattern `no-escaping-relative-imports` above already carves out ("do not fix it into
 * a specifier-depth rule, that would break every script.ts in the repo") — flagging it here would just be
 * ~30 files of noise on an already-litigated non-issue, not a new real finding. */
function crossPackageRelativeRule() {
  const familyPattern = "🧰️framework/(?:🛍️products|🔨️modules)/[^/]+|✏️s/🔨️modules/[^/]+|🌎️hub/🔨️modules/[^/]+|♻️mit-bestand/[^/]+";
  return {
    name: "no-cross-package-relative",
    severity: "warn",
    comment:
      "Deep relative imports must not cross package/module family boundaries in favor of @semio-tech/… package-name imports — WARN until package-name imports are the norm repo-wide, then promote at finalization",
    from: { path: `^(${familyPattern})/`, pathNot: "(^|/)📜️script\\.ts$" },
    to: {
      dependencyTypes: ["local"],
      pathNot: "^$1/",
    },
  };
}

/** 🧱️ `framework-no-s` (W1 of `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`): `🧰️framework` must not
 * import `✏️s` app/plugin/module code — framework is the substrate `✏️s` builds on, never the reverse.
 * ERROR (W7 of the same ticket): a full `bunx dependency-cruiser … --output-type err` sweep across
 * `compose 🧰️framework ✏️s 🌎️hub ♻️mit-bestand` found zero real hits for this rule — the pre-existing
 * violations it was staged to wait out never materialized on this (TS/JS import graph) surface, so there
 * is nothing left to clear before promoting. */
function frameworkNoSRule() {
  return {
    name: "framework-no-s",
    severity: "error",
    comment: "🧰️framework must not import ✏️s app/plugin/module code — apps consume framework, never the reverse",
    from: { path: "^🧰️framework/" },
    to: {
      path: ["^✏️s/"].concat(S_PACKAGES.map((p) => `^${escapeRegex(p.name)}$`)),
    },
  };
}

/** 🧱️ `s-modules-no-plugins` (W1 of `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`): `✏️s/🔨️modules`
 * are shared building blocks and must not depend on `✏️s/🔌️plugins`, the app layer built from them — the
 * audit for this ticket found zero real violations of this direction, so it's safe to enforce as ERROR
 * immediately rather than staging it through `warn`. */
function sModulesNoPluginsRule() {
  const pluginPackageNames = S_PACKAGES.filter((p) => p.dir.startsWith("✏️s/🔌️plugins/")).map((p) => `^${escapeRegex(p.name)}$`);
  return {
    name: "s-modules-no-plugins",
    severity: "error",
    comment: "✏️s/🔨️modules must not import ✏️s/🔌️plugins — modules are shared substrate for plugins, not the reverse",
    from: { path: "^✏️s/🔨️modules/" },
    to: {
      path: ["^✏️s/🔌️plugins/"].concat(pluginPackageNames),
    },
  };
}

/** 🧱️ `no-plugin-to-extension` (W1 of `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`): a plugin's core
 * code (i.e. everything under `✏️s/🔌️plugins/{p}/` outside its own `🧩️extensions/`) must not depend on
 * ANY plugin's `🧩️extensions/` tree — extensions are optional add-ons layered on top of a plugin's core,
 * so the dependency must run extension → core, never core → extension. `from` excludes `{p}/🧩️extensions/`
 * itself so extension-to-extension imports (including within the same plugin) are exempt. ERROR (W7 of
 * the same ticket) for every plugin except `🌀️procedural` and `📐️cad` — a full depcruise sweep of the
 * TS/JS import graph found zero real hits for every OTHER per-plugin rule, so there is nothing left to
 * clear before promoting those.
 *
 * `🌀️procedural` is the documented C2 exception (see `📓️w5b-c2-verdict.md`): 7 REAL dependencies on
 * `🌊️flow`'s extension crates, but they are Cargo (Rust) edges, invisible to this TS/JS-only scan either
 * way — promoting this rule here would be cosmetic, not a real gate on C2. C2 stays enforced (WARN-only,
 * populated allowlist) by the cargo-metadata layering lint instead (`CapabilityLayeringLintScript`,
 * `KNOWN_LAYERING_VIOLATIONS`), which actually sees the Cargo graph. Unlinking it for real needs new
 * runtime infrastructure that doesn't exist yet — a follow-up ticket, not mechanical cleanup.
 *
 * `📐️cad` is a SECOND, newly-discovered real violation (W7, not previously investigated by any earlier
 * wave of this ticket): `🔨️modules/🏃️runtime/🟦️component.ts` and `🔨️modules/📐️brepjs/🟦️component.ts`
 * (both plugin-core, outside `🧩️extensions/`) statically `import`/`import()` all 4 of cad's own
 * extensions (`@semio-tech/cad-js-module-{spatial-shape,aec-building,aec-building-energy,aec-building-
 * structure}`) to build `CAD_MODULE_REGISTRARS` — a composition-root that registers each installed
 * extension module. This is a real, structural core→extension edge, not noise, but "fix" here means
 * redesigning how cad's extensions register themselves (e.g. self-registration into a runtime-populated
 * table instead of the core statically importing every extension by name) — an architecture change, not
 * a lint-severity flip; out of scope for this pass. Left at WARN pending a dedicated follow-up. */
function noPluginToExtensionRules() {
  const extensionPackageNamePatterns = S_PACKAGES.filter((p) => /(^|\/)🧩️extensions\//.test(p.dir)).map((p) => `^${escapeRegex(p.name)}$`);
  const GRANDFATHERED_PLUGINS = new Set(["🌀️procedural", "📐️cad"]);
  return PLUGINS.map((p) => ({
    name: `no-plugin-to-extension-${p}`,
    severity: GRANDFATHERED_PLUGINS.has(p) ? "warn" : "error",
    comment: GRANDFATHERED_PLUGINS.has(p)
      ? `a plugin's core must not depend on any plugin's extensions tree — extensions depend on core, not the reverse — WARN: ${p === "🌀️procedural" ? "🌀️procedural's real violation (C2) is a Cargo dependency edge, invisible to this TS/JS import-graph scan; enforced instead by the cargo-metadata layering lint's KNOWN_LAYERING_VIOLATIONS allowlist, see 📓️w5b-c2-verdict.md" : "📐️cad's real violation is its runtime/brepjs composition-root statically importing all 4 of its own extensions to register them — an architecture change, not a lint flip; see this rule's own docstring above"}`
      : "a plugin's core must not depend on any plugin's extensions tree — extensions depend on core, not the reverse",
    from: {
      path: `^✏️s/🔌️plugins/${p}/`,
      pathNot: `^✏️s/🔌️plugins/${p}/🧩️extensions/`,
    },
    to: {
      path: ["^✏️s/🔌️plugins/[^/]+/🧩️extensions/"].concat(extensionPackageNamePatterns),
    },
  }));
}

/** 🔌️ `plugins-framework-sdk-only` (`26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`): TS/JS mirror of
 * `PluginCapabilityLintScript`'s Cargo-side `semio-framework-os` ban (framework-os-dev's `📜️script.ts`,
 * `depRules`) — a plugin may depend on the plugin SDK (`@semio-tech/framework`, the product-neutral
 * package rooted at `🧰️framework/📦️packages/`) but must not depend on any OTHER `🧰️framework` package: the
 * OS host (`@semio-tech/framework-os`), a renderer target, or anything else product-specific that isn't
 * the SDK. `from` excludes `📜️script.ts` for the exact reason `crossPackageRelativeRule` above already
 * does: every plugin's own build/dev script relative-imports repo-lib
 * (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/…`), a sanctioned dev-tooling pattern, not a runtime
 * coupling — leaving it in would have drowned the real findings below in 108 identical script.ts hits. WARN,
 * not error, for the same reason this ticket's other policy rules stay report-mode (see
 * `noImplSegmentRule`/`crossPackageRelativeRule` above): do not shift the shared verify gate under the two
 * other sessions concurrently editing this tree. A dry sweep at seed time DID find 7 real runtime hits after
 * the `📜️script.ts` exclusion (`bunx dependency-cruiser --config .dependency-cruiser.cjs --output-type
 * err-long ✏️s`, grep `plugins-framework-sdk-only` minus `📜️script\.ts →` lines) — `📐️cad`'s renderer/brepjs
 * components reaching `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f`,
 * `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/pkg` (a wasm build output), and three plugins'
 * `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react` direct-renderer imports — real,
 * pre-existing, left as WARN backlog for a future wave to triage rather than silenced or hastily excepted. */
function pluginsFrameworkSdkOnlyRule() {
  const otherFrameworkPackageNames = FRAMEWORK_PACKAGES.filter((p) => /^@semio-tech\/framework($|-)/.test(p.name) && p.name !== "@semio-tech/framework").map(
    (p) => `^${escapeRegex(p.name)}$`,
  );
  return {
    name: "plugins-framework-sdk-only",
    severity: "warn",
    comment: "✏️s/🔌️plugins/** may depend on the plugin SDK (@semio-tech/framework) but not any other 🧰️framework package — mirrors the Cargo capability lint's semio-framework-os ban",
    from: { path: "^✏️s/🔌️plugins/", pathNot: "(^|/)📜️script\\.ts$" },
    to: {
      path: ["^🧰️framework/"].concat(otherFrameworkPackageNames),
      pathNot: ["^🧰️framework/📦️packages/", "^@semio-tech/framework$"],
    },
  };
}

/** 🖥️ Enforces the renderer host's presentation-only dependency boundary against dependency-cruiser's canonical resolved paths. */
function rendererHostsOnlyUiRule() {
  return {
    name: "renderer-hosts-only-ui",
    severity: "error",
    comment: "the react renderer host may depend only on ui/styling, framework-core protocol types, react, and itself — never os-shell, ui-interpreter, or app packages",
    from: { path: RENDERER_HOST_ROOT },
    to: { pathNot: RENDERER_HOST_ALLOWED_RESOLVED_PATHS },
  };
}

module.exports = {
  forbidden: [
    {
      name: "no-circular",
      severity: "error",
      comment: "No circular dependencies",
      from: {},
      to: { circular: true },
    },
    {
      name: "not-to-unlisted",
      severity: "error",
      comment: "Only depend on packages declared in the nearest package.json",
      from: {},
      to: {
        dependencyTypes: ["npm-no-pkg", "npm-unknown"],
      },
    },
    {
      // 🧭️ Matches the RESOLVED dependency path, not the raw import specifier — dependency-cruiser is
      // invoked from the repo root, so this only fires when a resolved local import lands 4+ levels
      // above the repo root, i.e. actually escapes the checkout. A `📜️script.ts`'s own relative
      // specifier (`../../../../../../../🧰️framework/…`, 6-8 `../` segments to reach repo-lib) resolves
      // to an ordinary in-repo path and never trips this — do not "fix" it into a specifier-depth rule,
      // that would break every script.ts in the repo (see ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE).
      name: "no-escaping-relative-imports",
      severity: "error",
      comment: "Relative imports must not resolve to a path outside the repo checkout",
      from: {},
      to: {
        dependencyTypes: ["local"],
        path: "^\\.\\./\\.\\./\\.\\./\\.\\./",
      },
    },
    {
      name: "framework-no-plugin-packages",
      severity: "error",
      comment: "🧰️framework must not import plugin app packages — shells derive from app contributions",
      from: { path: "^🧰️framework/" },
      to: {
        path: PLUGINS.map((p) => `^✏️s/🔌️plugins/${p}/`).concat(PLUGINS.map((p) => `^@semio-tech/${p.replace(/^[^a-zA-Z]+/, "")}-`)),
      },
    },
    {
      name: "ui-no-framework-packages",
      severity: "error",
      comment: "🧰️framework/🔨️modules/🖱️ui must stay presentational and business-logic free — no OS/plugin/framework coupling",
      from: { path: "^🧰️framework/🔨️modules/🖱️ui/" },
      to: {
        path: ["^🧰️framework/", "^@semio-tech/framework-"],
        pathNot: ["^🧰️framework/🔨️modules/🖱️ui/", "^@semio-tech/ui-"],
      },
    },
    rendererHostsOnlyUiRule(),
    {
      name: "no-generated-edits-upstream",
      severity: "error",
      comment: "only the plugin registry itself may import its generated plugin catalog directly — other consumers must go through generated/🟦️plugins.ts",
      from: { pathNot: "^🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/" },
      to: { path: "^🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🔣️plugins\\.json$" },
    },
    {
      name: "no-state-outside-os",
      severity: "error",
      comment:
        "OS-exclusive state authority: ✏️s and non-OS framework must not import OS host DocumentStore/session internals via deep relative paths — go through @semio-tech packages / public host APIs",
      from: {
        path: "^(✏️s/|🧰️framework/)",
        pathNot: "^🧰️framework/🛍️products/💻️os/",
      },
      to: {
        path: "^🧰️framework/🛍️products/💻️os/.*/(🏪️store|🖥️host)/",
        dependencyTypes: ["local"],
      },
    },
    ...crossTechnologyRules(),
    ...crossPluginRules(),
    noImplSegmentRule(),
    noCorePathRule(),
    crossPackageRelativeRule(),
    frameworkNoSRule(),
    sModulesNoPluginsRule(),
    ...noPluginToExtensionRules(),
    pluginsFrameworkSdkOnlyRule(),
  ],
  options: {
    doNotFollow: {
      path: "node_modules|dist|target|storybook-static|\\.git|\\.nx|\\.🧬semio|\\.repo",
    },
    tsPreCompilationDeps: true,
    combinedDependencies: true,
    enhancedResolveOptions: {
      exportsFields: ["exports"],
      conditionNames: ["import", "require", "node", "default"],
    },
  },
};
