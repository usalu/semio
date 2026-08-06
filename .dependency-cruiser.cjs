/** @type {import('dependency-cruiser').IConfiguration} */
const fs = require("fs");
const path = require("path");

const TECHNOLOGIES = ["compose", "🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"];

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
  fs.readFileSync(path.join(__dirname, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/🔣️taxonomy.json"), "utf8"),
);

function crossTechnologyRules() {
  const rules = [];
  for (const from of TECHNOLOGIES) {
    for (const to of TECHNOLOGIES) {
      if (from === to) continue;
      rules.push({
        name: `no-cross-technology-${from}-to-${to}`,
        severity: "error",
        comment: "Relative imports must not cross top-level technology folders; use @semio-tech packages",
        from: { path: `^${from}/` },
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
 * relative-imports repo-lib (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/…`) across family boundaries —
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
    {
      // 🧭️ Was stale since before this rule's own introduction: it matched
      // `📺️renderer/⚡️implementations/🟦️typescript/🧑️‍🎨️engine/⚛️react/`, but the real dir order has always
      // been `🧑️‍🎨️engine/⚛️react/⚡️implementations/🟦️typescript/` — zero paths ever matched `from`, so
      // this rule has never fired. Repointed (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE) at the
      // FUTURE co-located shape (`📦️packages/🟦️typescript/🎯️targets/⚛️react/` + `🧱️elements/`) rather
      // than the current dead path — stays a deliberate no-op until that restructure's W4 move lands,
      // at which point it starts enforcing for real. Do not "fix" it back to the current dir; that would
      // just trade one dead path for another about to be deleted.
      name: "renderer-hosts-only-ui",
      severity: "error",
      comment: "the react renderer host may depend only on ui/styling, framework-core protocol types, react, and itself — never os-shell, ui-interpreter, or app packages",
      from: { path: "^🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/(📦️packages/🟦️typescript/🎯️targets/⚛️react|🧱️elements)/" },
      to: {
        pathNot: [
          "^🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/(📦️packages/🟦️typescript/🎯️targets/⚛️react|🧱️elements)/",
          "^@semio-tech/ui-",
          "^@semio-tech/framework-core",
          "^react$",
          "^react/",
          "^react-dom$",
          "^react-dom/",
          "^node:",
        ],
      },
    },
    {
      name: "no-generated-edits-upstream",
      severity: "error",
      comment: "only the plugin registry itself may import its generated plugin catalog directly — other consumers must go through generated/🟦️plugins.ts",
      from: { pathNot: "^🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/" },
      to: { path: "^🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/🤖️generated/🔣️plugins\\.json$" },
    },
    ...crossTechnologyRules(),
    ...crossPluginRules(),
    noImplSegmentRule(),
    crossPackageRelativeRule(),
  ],
  options: {
    doNotFollow: {
      path: "node_modules|dist|target|storybook-static|\\.git|\\.nx|\\.🦑️repo|\\.repo",
    },
    tsPreCompilationDeps: true,
    combinedDependencies: true,
    enhancedResolveOptions: {
      exportsFields: ["exports"],
      conditionNames: ["import", "require", "node", "default"],
    },
  },
};
