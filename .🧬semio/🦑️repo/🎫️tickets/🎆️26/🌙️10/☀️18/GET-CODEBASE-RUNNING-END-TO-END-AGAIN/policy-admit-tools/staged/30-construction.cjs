/** 🧱️ Constructs every boundary selector from admitted immutable policy inputs.
 * @param {import("./🟦️.ts").DependencyPolicyConstructionInputs} inputs
 * @returns {import("../🚀️bootstrap/🟦️.ts").DependencyPolicySnapshot["policy"]}
 */
function buildDependencyDirectionPolicy({ taxonomy: TAXONOMY, plugins: PLUGINS, workspacePackages, nodeBuiltins }) {
const api = TAXONOMY?.dependencyDirections?.publicApi;
if (!api || Object.keys(api).sort().join("|") !== "rulePrefix|sdkFoundationPackages|sdkPackage|toolingRoles|version" || api.version !== 1 || api.rulePrefix !== "no-cross-package-relative-" || typeof api.sdkPackage !== "string" || !api.sdkPackage || /\s/u.test(api.sdkPackage) || !Array.isArray(api.toolingRoles) || api.toolingRoles.length) throw Error("Dependency policy requires canonical authored public API authority");
if (!Array.isArray(api.sdkFoundationPackages) || !api.sdkFoundationPackages.length || new Set(api.sdkFoundationPackages).size !== api.sdkFoundationPackages.length || api.sdkFoundationPackages.some(name => typeof name !== "string" || !/^@[^/\s]+\/[^/\s]+$/u.test(name) || name === api.sdkPackage)) throw Error("Dependency policy requires unique admitted SDK foundation package names");
const packageOwners = new Map(), packageNames = new Map();
for (const pkg of workspacePackages) {
  if (typeof pkg.owner !== "string" || !pkg.owner || pkg.sourceOwner !== (pkg.owner.includes("/📦️packages/") ? pkg.owner.split("/📦️packages/")[0] : pkg.owner) || !Array.isArray(pkg.exportTargets) || !Array.isArray(pkg.exports)) throw Error("Dependency policy requires complete authored package authority");
  if (typeof pkg.name !== "string" || !pkg.name || pkg.exportTargets.some(row => !row || typeof row.subpath !== "string" || (row.target !== null && typeof row.target !== "string") || Object.keys(row).sort().join("|") !== "subpath|target")) throw Error("Dependency policy requires readable authored package authority");
  const targets = new Map();
  for (const row of pkg.exportTargets) { if (!targets.has(row.subpath)) targets.set(row.subpath,[]); targets.get(row.subpath).push(row.target); }
  const canonical = authoredPackageExportAuthority(pkg.owner,{ exports: Object.fromEntries(targets) });
  if (new Set(pkg.exportTargets.map(row => JSON.stringify(row))).size !== pkg.exportTargets.length || pkg.exports.length !== canonical.exports.length || pkg.exports.some(key => !canonical.exports.includes(key))) throw Error("Dependency policy has contradictory public export authority");
  const previous = packageOwners.get(pkg.owner), named = packageNames.get(pkg.name);
  if (previous && (previous.name !== pkg.name || previous.dependencyRole !== pkg.dependencyRole || previous.exports.length !== pkg.exports.length || previous.exports.some((value,index) => value !== pkg.exports[index]) || previous.sourceOwner !== pkg.sourceOwner || JSON.stringify(previous.exportTargets) !== JSON.stringify(pkg.exportTargets))) throw Error("Dependency policy owner has conflicting package authority: "+pkg.owner);
  if (named !== undefined && named !== pkg.owner) throw Error("Dependency policy package name has distinct owners: "+pkg.name+" ("+named+", "+pkg.owner+")");
  packageOwners.set(pkg.owner,pkg); packageNames.set(pkg.name,pkg.owner);
}
workspacePackages = [...packageOwners.values()];
const TECHNOLOGIES = ["compose", "🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"];
const BOOTSTRAP_TOOLING_ENTRY_PATH = "(^|/)(?:📜️script\\.ts|🏗️builder/🌐️vite/🟦️\\.ts|🧪️tests/🎚️config/🟦️\\.ts|(?:⚙️|🧪️)?(?:vite|vitest)\\.config\\.[cm]?[jt]s)$";
const IN_SOURCE_TEST_PATH = "(^|/)🧪️tests/";
const BUILDER_SOURCE_PATH = "(^|/)🏗️builder/";
const UI_TEST_SOURCE_PATH = "(^|/)🧪️(?:tests|testing)/|\\.(?:test|spec|story)\\.[cm]?[jt]sx?$";
const RENDERER_HOST_ROOT = "^🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/(🎯️targets/⚛️react|🧱️elements)/";
/** 🖥️ The renderer host's UI contract with the OS package: only the declared `elements` subpaths of `@semio-tech/framework-os` (the published entries of the host's own element modules), never any other OS internals. */
const RENDERER_HOST_UI_CONTRACT = Object.freeze({ package: "@semio-tech/framework-os", subpathPrefixes: Object.freeze(["./modules/renderer/engine/elements/", "./testkit/renderer-host/"]) });
/** 🎬️ The media window-kit contract the media transport host and the window kit share: the only plugin-owned subpaths a host may read. */
const RENDERER_HOST_WINDOW_KIT_CONTRACT = Object.freeze({ package: "@semio-tech/plugin-window-kits", subpathPrefixes: Object.freeze(["./media"]) });
/** 🧫️ Language-neutral corpora a host test reads through declared exports: published fixture modules and schema JSON modules. */
const RENDERER_HOST_CORPUS_PATHS = Object.freeze(["(?:^|/)🧪️testing/🧫️fixtures/", "(?:^|/)🧬️schema/🔣️json/🟦️\\.ts$"]);
const RENDERER_HOST_UI_CONTRACT_TARGETS = exportTargetPatterns(workspacePackages.filter(pkg => pkg.name === RENDERER_HOST_UI_CONTRACT.package).map(pkg => ({ ...pkg, exportTargets: pkg.exportTargets.filter(row => RENDERER_HOST_UI_CONTRACT.subpathPrefixes.some(prefix => row.subpath.startsWith(prefix))) })));
const RENDERER_HOST_WINDOW_KIT_TARGETS = exportTargetPatterns(workspacePackages.filter(pkg => pkg.name === RENDERER_HOST_WINDOW_KIT_CONTRACT.package).map(pkg => ({ ...pkg, exportTargets: pkg.exportTargets.filter(row => RENDERER_HOST_WINDOW_KIT_CONTRACT.subpathPrefixes.some(prefix => row.subpath === prefix || row.subpath.startsWith(prefix + "/"))) })));
/** 🗝️ The authored public targets of the admitted SDK foundation packages (`publicApi.sdkFoundationPackages`): neutral modules that carry keys once published by the SDK package, shared by plugins and renderer hosts. */
const SDK_FOUNDATION_TARGETS = exportTargetPatterns(workspacePackages.filter(pkg => api.sdkFoundationPackages.includes(pkg.name)));
const RESOLVED_NODE_BUILTIN_PATH = `^(?:node:)?(?:${[...new Set(nodeBuiltins.map((name) => name.replace(/^node:/, "")).filter((name) => !name.includes(":")))].map(escapeRegex).join("|")})(?:$|/)`;
const RENDERER_HOST_ALLOWED_RESOLVED_PATHS = [
  RENDERER_HOST_ROOT,
  ...RENDERER_HOST_UI_CONTRACT_TARGETS,
  ...RENDERER_HOST_WINDOW_KIT_TARGETS,
  ...SDK_FOUNDATION_TARGETS,
  ...RENDERER_HOST_CORPUS_PATHS,
  "^🧰️framework/🔨️modules/🖱️ui/",
  "^🧰️framework/📦️packages/",
  installedPackagePath(["react", "react-dom"]),
  "^(?:node:|vitest/)",
  "^bun:test$",
  installedPackagePath(["vite", "vitest"]),
  RESOLVED_NODE_BUILTIN_PATH,
];

/** ⚙️ Escapes a literal string for embedding inside a `RegExp` alternation. */
function escapeRegex(literal) {
  return literal.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/** 📦️ Matches an installed package by its resolved path whether it is linked at `node_modules/<name>` or realpathed into the bun store (`<store>/node_modules/.bun/<name>@<version>/node_modules/<name>`), independent of the store location. Written without an optional group around the version repetition so the resolver's safe-regex admission holds. */
function installedPackagePath(names) {
  const alternation = names.map(escapeRegex).join("|");
  return `(?:^|/)node_modules/(?:(?:${alternation})/|\\.bun/(?:${alternation})@[^/]+/node_modules/(?:${alternation})/)`;
}

/** 🌳️ Factors literal prefixes without introducing captures or changing their accepted strings. */
function literalPrefixPattern(values) {
  const root = { terminal: false, children: new Map() };
  for (const value of values) {
    let node = root;
    for (const part of value) {
      if (!node.children.has(part)) node.children.set(part, { terminal: false, children: new Map() });
      node = node.children.get(part);
    }
    node.terminal = true;
  }
  const render = (node) => {
    const branches = [];
    for (const [head, child] of node.children) {
      let prefix = head, tail = child;
      while (!tail.terminal && tail.children.size === 1) {
        const [part, next] = tail.children.entries().next().value;
        prefix += part; tail = next;
      }
      branches.push(escapeRegex(prefix) + render(tail));
    }
    const suffix = branches.length > 1 ? `(?:${branches.join("|")})` : branches[0] ?? "";
    return node.terminal && suffix ? `(?:${suffix}|)` : suffix;
  };
  return render(root);
}

/** 🏷️ Physical source ownership ends before installed package storage. */
function sourceOwnerPattern(owner) {
  return `^${escapeRegex(owner)}/(?!node_modules(?:/|$)|.*/node_modules(?:/|$))`;
}

/** 🏷️ Groups installed aliases that share the same direction while retaining exact package boundaries. */
function packageOwnerPatterns(names) {
  const packages = [...new Set(names)].sort();
  return packages.length ? [`(?:^(?:node_modules/)?|/node_modules/)(?:${literalPrefixPattern(packages)})(?:$|/)`] : [];
}

/** 🗂️ Removes only redundant descendants already covered by an identical physical owner direction. */
function sourceOwnerPatterns(owners) {
  const roots = [...new Set(owners)], index = new Set(roots);
  return roots.filter(owner => {
    for (let position = owner.indexOf("/"); position >= 0; position = owner.indexOf("/", position + 1)) if (index.has(owner.slice(0, position))) return false;
    return true;
  }).map(sourceOwnerPattern);
}

/** 🧪️ Fails config loading if the two resolver-boundary allowlists regress into broad source or vendor exclusions. */
function assertFocusedBoundarySemantics() {
  const bootstrap = new RegExp(BOOTSTRAP_TOOLING_ENTRY_PATH, "u");
  const approvedBootstrap = [
    "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📜️script.ts",
    "compose/client/lib/sketchpad/doc/js/vite.config.ts",
    "♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts",
    "♻️mit-bestand/🎤️präsentation/📦️packages/🟦️typescript/🧪️vitest.config.ts",
  ];
  const runtimeSources = [
    "compose/client/lib/sketchpad/js/boot.tsx",
    "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🟦️.ts",
    "🌎️hub/🔨️modules/example/🟦️.ts",
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
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts",
    "🧰️framework/📦️packages/🟦️typescript/🟦️.ts",
    "node_modules/react/index.js",
    "node_modules/react-dom/client.js",
    "node_modules/vitest/dist/index.js",
    ".cache/b/d/m/j/0123456789abcdef0123/node_modules/.bun/react@19.3.1/node_modules/react/index.js",
    ".cache/b/d/m/j/0123456789abcdef0123/node_modules/.bun/react-dom@19.3.1+peer/node_modules/react-dom/client.js",
    "vitest/importMeta",
    "bun:test",
    "fs",
    "node:path",
  ];
  const forbiddenRendererTargets = [
    "node_modules/three/build/three.module.js",
    ".cache/b/d/m/j/0123456789abcdef0123/node_modules/.bun/three@0.170.0/node_modules/three/build/three.module.js",
    ".cache/b/d/m/j/0123456789abcdef0123/node_modules/.bun/react@19.3.1/node_modules/three/index.js",
    "bun:sqlite",
    "🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/🟦️.ts",
    "🧰️framework/🔨️modules/🎠️kernel/🎛️activation-registry/🟦️.ts",
    "🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🔨️modules/🏪️store/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🔨️modules/📺️renderer/🧑‍🎨engine/🔮️oracles/🎨️world3d-scene-shading/📜️script.ts",
    "♻️mit-bestand/🧺️demonstrator/🪧️brand.ts",
  ];
  const allowed = (candidate) => rendererAllows.some((pattern) => pattern.test(candidate));
  if (allowedRendererTargets.some((candidate) => !allowed(candidate)) || forbiddenRendererTargets.some(allowed)) {
    throw new Error("dependency-cruiser renderer-host allowlist no longer distinguishes presentation dependencies from runtime ownership breaches");
  }
}

assertFocusedBoundarySemantics();

const AREA_LAYERS = Object.entries(TAXONOMY.areaLayers);
const OPAQUE_PATHS = Object.values(TAXONOMY.pathExclusions).map(({ path: prefix }) => prefix.replace(/\/$/u, ""));
const WORKSPACE_PACKAGES = workspacePackages.map(({ owner, ...pkg }) => ({ dir: owner, ...pkg }));
const S_PACKAGES = WORKSPACE_PACKAGES.filter(({ dir }) => dir.startsWith("✏️s/"));
const FRAMEWORK_PACKAGES = WORKSPACE_PACKAGES.filter(({ dir }) => dir.startsWith("🧰️framework/"));
const VENDORED_PATHS = TAXONOMY.dependencyDirections.vendoredPaths;
if (!Array.isArray(VENDORED_PATHS) || new Set(VENDORED_PATHS).size !== VENDORED_PATHS.length || VENDORED_PATHS.some((path) => typeof path !== "string" || !path || path.startsWith("/") || path.endsWith("/") || path.split("/").some((segment) => !segment || segment === "." || segment === ".."))) throw new Error("Dependency policy requires unique normalized repository-relative vendored paths");
const IGNORED_GRAPH_PATHS = TAXONOMY.implementationLeafPolicy.ignoredPathPatterns.filter((pattern) => pattern !== "**/node_modules").map((pattern) => `(^|/)${escapeRegex(pattern.replace(/^\*\*\//u, ""))}(/|$)`)
  .concat(OPAQUE_PATHS.concat(VENDORED_PATHS).map((prefix) => `^${escapeRegex(prefix)}(/|$)`));

/** 🧱️ Matches framework and implementation owners identically through resolved paths and public package names. */
function frameworkNoImplementationRule() {
  const frameworkAreas = AREA_LAYERS.filter(([, layer]) => layer === "framework").map(([area]) => area);
  const implementationAreas = AREA_LAYERS.filter(([, layer]) => layer === "implementation").map(([area]) => area);
  const targets = sourceOwnerPatterns(implementationAreas).concat(packageOwnerPatterns(WORKSPACE_PACKAGES.filter((pkg) => implementationAreas.some((area) => pkg.dir === area || pkg.dir.startsWith(`${area}/`))).map((pkg) => pkg.name)));
  return {
    name: "framework-no-implementation",
    severity: "error",
    comment: "Taxonomy framework areas must remain independent of implementation areas, including type imports, dynamic imports, tests, tooling and package subpaths",
    from: { path: frameworkAreas.map(sourceOwnerPattern) },
    to: { path: targets },
  };
}

/** 🧬️ Resolves declared roles to resolved-path and package-alias selectors from taxonomy owners and workspace package contributions. */
function dependencyRolePatterns(roleIds) {
  const { roles } = TAXONOMY.dependencyDirections;
  return roleIds.flatMap((id) => {
    const role = roles[id];
    if (!role) throw new Error(`Unknown dependency direction role ${id}`);
    const packages = WORKSPACE_PACKAGES.filter((pkg) => pkg.dependencyRole === id || role.ownerPaths.some((owner) => pkg.dir === owner || pkg.dir.startsWith(`${owner}/`)));
    return sourceOwnerPatterns(role.ownerPaths.concat(packages.map((pkg) => pkg.dir))).concat(packageOwnerPatterns(packages.map((pkg) => pkg.name).concat(role.externalPackages)));
  });
}

/** 🧬️ Builds declared semantic role rules from taxonomy owners and workspace package contributions. */
function declaredDependencyDirectionRules() {
  const { roles, rules } = TAXONOMY.dependencyDirections;
  for (const pkg of WORKSPACE_PACKAGES) if (pkg.dependencyRole && !roles[pkg.dependencyRole]) throw new Error(`Unknown dependency role ${pkg.dependencyRole} in ${pkg.dir}`);
  return Object.entries(rules).map(([name, rule]) => ({ name, severity: "error", comment: "Semantic owners must follow their declared dependency direction through resolved paths, package aliases and subpaths", from: { path: dependencyRolePatterns(rule.fromRoles) }, to: { path: dependencyRolePatterns(rule.toRoles) } }));
}

/** 🖱️ The ui boundary: ui sources may depend on the declared foundation roles at runtime, on the declared test-support roles only from test sources, and on nothing else of the framework; bootstrap entry points are the composition roots and stay exempt. */
function uiBoundaryRules() {
  const boundary = TAXONOMY.dependencyDirections.uiBoundary, { roles } = TAXONOMY.dependencyDirections;
  if (!boundary || boundary.version !== 1 || !roles[boundary.subjectRole] || !Array.isArray(boundary.foundationRoles) || !Array.isArray(boundary.testSupportRoles)) throw new Error("Dependency policy requires a canonical ui boundary declaration");
  const subject = sourceOwnerPatterns(roles[boundary.subjectRole].ownerPaths);
  const foundation = dependencyRolePatterns(boundary.foundationRoles), testSupport = dependencyRolePatterns(boundary.testSupportRoles);
  return [
    {
      name: "ui-no-framework-packages",
      severity: "error",
      comment: "🧰️framework/🔨️modules/🖱️ui must stay presentational and business-logic free — it may use only the declared ui-foundation modules, and the declared ui-test-support modules from tests; everything else is injected through a ui-owned port",
      from: { path: subject, pathNot: BOOTSTRAP_TOOLING_ENTRY_PATH },
      to: { path: ["^🧰️framework/", "^@semio-tech/framework-"], pathNot: subject.concat(["^@semio-tech/ui-"], foundation, testSupport) },
    },
    {
      name: "ui-foundation-closure",
      severity: "error",
      comment: "ui-foundation modules are the closed set the ui may depend on: their runtime sources may depend only on the ui and on other foundation modules; scripts, tests and builders are composition roots",
      from: { path: foundation, pathNot: [BOOTSTRAP_TOOLING_ENTRY_PATH, UI_TEST_SOURCE_PATH, BUILDER_SOURCE_PATH] },
      to: { path: ["^🧰️framework/", "^@semio-tech/framework-"], pathNot: subject.concat(["^@semio-tech/ui-"], foundation) },
    },
    {
      name: "ui-runtime-no-test-support",
      severity: "error",
      comment: "ui-test-support modules are test infrastructure and corpora: only test sources and bootstrap entry points may use them",
      from: { path: subject, pathNot: [BOOTSTRAP_TOOLING_ENTRY_PATH, UI_TEST_SOURCE_PATH] },
      to: { path: testSupport, pathNot: foundation },
    },
  ];
}

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
          dependencyTypesNot: ["aliased-workspace"],
        },
      });
    }
  }
  return rules;
}

/** 🔌️ Every distinct plugin pair is forbidden through physical ownership and authored package aliases. */
function crossPluginRules() {
  const patterns = (plugin) => {
    const owner = `✏️s/🔌️plugins/${plugin}`;
    const packages = WORKSPACE_PACKAGES.filter((pkg) => pkg.dir.startsWith(`${owner}/`));
    return [sourceOwnerPattern(owner), ...packageOwnerPatterns(packages.map((pkg) => pkg.name))];
  };
  const rules = [];
  for (const from of PLUGINS) {
    for (const to of PLUGINS) {
      if (from === to) continue;
      rules.push({
        name: `no-cross-plugin-${from}-to-${to}`,
        severity: "error",
        comment: "Plugin owners consume neutral public contracts; no source, test, tooling or package alias may depend on a sibling plugin",
        from: { path: patterns(from) },
        to: { path: patterns(to) },
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

/** 🚫️ Bans dependency paths with a segment whose emoji-stripped, extension-stripped name is a banned stem (`core`, `shared`, …), exactly like the filesystem naming law `policyBannedNameStemBreaches`.
 * Node builtins (`core`) and unresolvable specifiers (`unknown`, for example `bun:test` or an uninstalled npm package) are not repository paths and never count.
 * Stems listed in `bannedNameStemExemptions.subsetDirectories` stay legal as the direct child of a `subsetsDirName` owner, where the standard's own subset vocabulary (for example `base`) names the directory.
 * Files listed in `bannedNameStemExemptions.vendoredFiles` are installed third-party modules with a fixed internal import identity and never count. */
function noCorePathRule() {
  const { bannedNameStems: banned, bannedNameStemExemptions: exemptions, subsetsDirName: subsets } = TAXONOMY;
  if (!Array.isArray(banned) || !banned.length || banned.some((stem) => typeof stem !== "string" || !/^[a-z0-9][a-z0-9-]*$/u.test(stem))) throw new Error("Dependency policy requires canonical bannedNameStems authority");
  if (!exemptions || Object.keys(exemptions).join() !== "subsetDirectories,vendoredFiles" || !Array.isArray(exemptions.subsetDirectories) || exemptions.subsetDirectories.some((stem) => !banned.includes(stem))) throw new Error("Dependency policy requires bannedNameStemExemptions.subsetDirectories drawn from bannedNameStems");
  if (!Array.isArray(exemptions.vendoredFiles) || exemptions.vendoredFiles.some((file) => typeof file !== "string" || !file || file.startsWith("/") || file.includes("..") || file.endsWith("/"))) throw new Error("Dependency policy requires bannedNameStemExemptions.vendoredFiles as repository-relative file paths");
  if (typeof subsets !== "string" || !subsets || subsets.includes("/")) throw new Error("Dependency policy requires canonical subsetsDirName authority");
  const exempt = new Set(exemptions.subsetDirectories), alternation = (stems) => stems.map(escapeRegex).join("|");
  const emoji = "[^\\x00-\\x7f]*";
  const everywhere = banned.filter((stem) => !exempt.has(stem)), subsetOnly = banned.filter((stem) => exempt.has(stem));
  const boundary = "(?:\\.(?:rs|ts|tsx|js|mjs|cjs|go|py|cs))?(?=/|$)";
  const branches = [...(everywhere.length ? [`(?:^|/)${emoji}(?:${alternation(everywhere)})`] : []), ...(subsetOnly.length ? [`^${emoji}(?:${alternation(subsetOnly)})`, `(?:^|/)(?!${escapeRegex(subsets)}/)[^/]+/${emoji}(?:${alternation(subsetOnly)})`] : [])];
  return {
    name: "no-core-path",
    severity: "error",
    comment: "Clean mechanism: no dependency may resolve through a banned name stem folder (core/shared/util/…) — ERROR after Wave 4 core dissolve",
    from: {
      path: "^(✏️s/|🧰️framework/|🌎️hub/|♻️mit-bestand/)",
    },
    to: {
      path: branches.map((branch) => branch + boundary),
      pathNot: ["(?:^|/)(?:node_modules|target|pkg)(?:/|$)", ...(exemptions.vendoredFiles.length ? [`^(?:${exemptions.vendoredFiles.map(escapeRegex).join("|")})$`] : [])],
      dependencyTypesNot: ["core", "unknown"],
    },
  };
}

/** 🛍️ Selects the closest admitted semantic owner and its actual authored public targets. */
function crossPackageRelativeRules() {
  const siblings = new Map(), descendants = new Map(), selectors = new Map();
  for (const pkg of workspacePackages) {
    if (!siblings.has(pkg.sourceOwner)) { siblings.set(pkg.sourceOwner, []); descendants.set(pkg.sourceOwner, []); }
    siblings.get(pkg.sourceOwner).push(pkg);
  }
  for (const owner of siblings.keys()) {
    for (let position = owner.indexOf("/"); position >= 0; position = owner.indexOf("/", position + 1)) {
      const parent = owner.slice(0, position);
      if (descendants.has(parent)) descendants.get(parent).push(owner.slice(parent.length + 1) + "/");
    }
  }
  for (const [owner, packages] of siblings) {
    const nested = literalPrefixPattern(descendants.get(owner));
    const exclusions = ["node_modules(?:/|$)", ".*/node_modules(?:/|$)", ...(nested ? [nested] : [])];
    const owned = `^${escapeRegex(owner)}/(?!(?:${exclusions.join("|")}))`;
    selectors.set(owner, { from: { path: "^", pathNot: owned }, to: { path: [owned, ...packageOwnerPatterns(packages.map(row => row.name))], pathNot: exportTargetPatterns(packages) } });
  }
  return workspacePackages.map(pkg => {
    const selector = selectors.get(pkg.sourceOwner);
    return { name: TAXONOMY.dependencyDirections.publicApi.rulePrefix + pkg.name, severity: "error", comment: "Cross-owner dependencies resolve only to authored package export targets, including scripts, types and literal dynamic imports", from: { ...selector.from }, to: { path: [...selector.to.path], pathNot: [...selector.to.pathNot] } };
  });
}

/** 📮️ Matches authored targets after more specific public keys displace wildcard captures. */
function exportTargetPatterns(packages) {
  return [...new Set(packages.flatMap(pkg => pkg.exportTargets.filter(row => row.target !== null).flatMap(row => {
    const parts = row.target.slice(2).split("*");
    const target = parts.map((part,index) => escapeRegex(part) + (index < parts.length - 1 ? index === 0 ? "(.*)" : "\\1" : "")).join("");
    const blocked = parts.length > 1 ? [...new Set(pkg.exportTargets.map(item => item.subpath))].flatMap(key => shadowCaptures(row.subpath,key)).map(capture => parts.map(escapeRegex).join(`(?:${capture})`)) : [];
    const allowed = (blocked.length ? `(?!(?:${blocked.join("|")})$)` : "") + target + "$";
    return [`^${escapeRegex(pkg.owner)}/${allowed}`, `(?:^(?:node_modules/)?|/node_modules/)${escapeRegex(pkg.name)}/${allowed}`];
  })))];
}

/** 🕳️ Computes values whose authored wildcard key is displaced by an exact or more specific key. */
function shadowCaptures(key, override) {
  const index = key.indexOf("*");
  if (index < 0 || key === override) return [];
  const prefix = key.slice(0,index), suffix = key.slice(index + 1), otherIndex = override.indexOf("*");
  if (otherIndex < 0) return override.startsWith(prefix) && override.endsWith(suffix) && override.length >= prefix.length + suffix.length ? [escapeRegex(override.slice(prefix.length,override.length - suffix.length))] : [];
  const otherPrefix = override.slice(0,otherIndex), otherSuffix = override.slice(otherIndex + 1);
  if (otherPrefix.length < prefix.length || (otherPrefix.length === prefix.length && override.length <= key.length) || !otherPrefix.startsWith(prefix)) return [];
  if (!otherSuffix.endsWith(suffix) && !suffix.endsWith(otherSuffix)) return [];
  const extraPrefix = otherPrefix.slice(prefix.length), extraSuffix = otherSuffix.endsWith(suffix) ? otherSuffix.slice(0,otherSuffix.length - suffix.length) : "";
  const captures = [escapeRegex(extraPrefix) + ".*" + escapeRegex(extraSuffix)];
  for (let length = 0; length < extraPrefix.length; length++) {
    const capture = extraPrefix.slice(0,length), candidate = prefix + capture + suffix;
    if (candidate.length >= otherPrefix.length + otherSuffix.length && candidate.startsWith(otherPrefix) && candidate.endsWith(otherSuffix)) captures.push(escapeRegex(capture));
  }
  return captures;
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
      path: [sourceOwnerPattern("✏️s")].concat(S_PACKAGES.map((p) => `^${escapeRegex(p.name)}$`)),
    },
  };
}

/** 🧱️ `s-modules-no-plugins` (W1 of `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`): `✏️s/🔨️modules`
 * are shared building blocks and must not depend on `✏️s/🔌️plugins`, the app layer built from them — the
 * audit for this ticket found zero real violations of this direction, so it's safe to enforce as ERROR
 * immediately rather than staging it through `warn`. */
function sModulesNoPluginsRule() {
  const pluginPackageNames = packageOwnerPatterns(S_PACKAGES.filter((p) => p.dir.startsWith("✏️s/🔌️plugins/")).map((p) => p.name));
  return {
    name: "s-modules-no-plugins",
    severity: "error",
    comment: "✏️s/🔨️modules must not import ✏️s/🔌️plugins — modules are shared substrate for plugins, not the reverse",
    from: { path: "^✏️s/🔨️modules/" },
    to: {
      path: [sourceOwnerPattern("✏️s/🔌️plugins")].concat(pluginPackageNames),
    },
  };
}

/** 🧩️ Optional extensions and artifacts consume their plugin owner, never the reverse. */
function pluginNoExtensionOrArtifactRules() {
  const ownerPaths = ["🧩️extensions", "🗿️artifacts"];
  const packagePatterns = packageOwnerPatterns(S_PACKAGES.filter((pkg) => ownerPaths.some((owner) => pkg.dir.includes(`/${owner}/`))).map((pkg) => pkg.name));
  return PLUGINS.map((plugin) => ({
    name: `plugin-no-extension-or-artifact-${plugin}`,
    severity: "error",
    comment: "Plugin owners must remain independent of optional extensions and artifacts through every import form",
    from: { path: `^✏️s/🔌️plugins/${escapeRegex(plugin)}/`, pathNot: `^✏️s/🔌️plugins/${escapeRegex(plugin)}/(?:${ownerPaths.join("|")})/` },
    to: { path: ["^✏️s/(?!node_modules(?:/|$)|.*/node_modules(?:/|$))🔌️plugins/[^/]+/(?:🧩️extensions|🗿️artifacts)/"].concat(packagePatterns) },
  }));
}

/** 🔌️ Every plugin source owner consumes only the admitted neutral public SDK and its foundation packages. */
function pluginsFrameworkSdkOnlyRule() {
  const sdk = workspacePackages.filter(pkg => pkg.name === TAXONOMY.dependencyDirections.publicApi.sdkPackage);
  return { name: "plugins-framework-sdk-only", severity: "error", comment: "Plugin runtime and executable owners use the neutral authored public SDK and the admitted SDK foundation packages; private framework and product APIs are forbidden", from: { path: "^✏️s/🔌️plugins/" }, to: { path: [sourceOwnerPattern("🧰️framework"), ...packageOwnerPatterns(FRAMEWORK_PACKAGES.map(pkg => pkg.name))], pathNot: [...exportTargetPatterns(sdk), ...SDK_FOUNDATION_TARGETS] } };
}

/** 🖥️ Enforces the renderer host's presentation-only dependency boundary against dependency-cruiser's canonical resolved paths. */
function rendererHostsOnlyUiRule() {
  return {
    name: "renderer-hosts-only-ui",
    severity: "error",
    comment: "the react renderer host may depend only on ui/styling, the admitted SDK foundation packages, framework-core protocol types, react, and itself — never os-shell, ui-interpreter, or app packages",
    from: { path: RENDERER_HOST_ROOT },
    to: { pathNot: RENDERER_HOST_ALLOWED_RESOLVED_PATHS },
  };
}

return {
  forbidden: [
    {
      name: "no-circular",
      severity: "error",
      comment: "No circular dependencies; a module's own in-source tests (registered behind `import.meta.vitest`, dead code in every production bundle) are leaves of that module and never members of its production graph",
      from: { pathNot: IN_SOURCE_TEST_PATH },
      to: { circular: true, pathNot: IN_SOURCE_TEST_PATH },
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
        path: ["^✏️s/🔌️plugins/"].concat(S_PACKAGES.filter((pkg) => pkg.dir.startsWith("✏️s/🔌️plugins/")).map((pkg) => `(?:^(?:node_modules/)?|/node_modules/)${escapeRegex(pkg.name)}(?:$|/)`)),
      },
    },
    ...uiBoundaryRules(),
    rendererHostsOnlyUiRule(),
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
        dependencyTypesNot: ["aliased-workspace"],
      },
    },
    ...crossTechnologyRules(),
    ...crossPluginRules(),
    noImplSegmentRule(),
    noCorePathRule(),
    ...crossPackageRelativeRules(),
    frameworkNoSRule(),
    frameworkNoImplementationRule(),
    { ...frameworkNoImplementationRule(), name: "repo-no-implementation", comment: "Repository-wide source must remain independent of deletable implementation owners", from: { path: ["^[^/]+$"] } },
    ...declaredDependencyDirectionRules(),
    sModulesNoPluginsRule(),
    ...pluginNoExtensionOrArtifactRules(),
    pluginsFrameworkSdkOnlyRule(),
  ],
  options: {
    exclude: { path: IGNORED_GRAPH_PATHS },
    doNotFollow: {
      path: IGNORED_GRAPH_PATHS.concat("(^|/)node_modules(/|$)").join("|"),
    },
    tsPreCompilationDeps: true,
    combinedDependencies: true,
    enhancedResolveOptions: {
      exportsFields: ["exports"],
      conditionNames: ["semio-source", "import", "require", "node", "default"],
    },
  },
};

}
/** 🧾️ Captures authored public keys and target leaves without implicit main or compatibility exports. */
function authoredPackageExportAuthority(owner, manifest) {
  if (typeof owner !== "string" || /[\\\0]/u.test(owner) || owner.split("/").some(segment => !segment || segment === "." || segment === "..")) throw Error("Canonical authored package owner required");
  const sourceOwner = owner.includes("/📦️packages/") ? owner.split("/📦️packages/")[0] : owner;
  const rows = [];
  const collect = (subpath, value) => {
    if (!/^\.(?:\/.*)?$/u.test(subpath) || /[\\\0]/u.test(subpath) || (subpath.match(/\*/g)?.length ?? 0) > 1 || (subpath !== "." && subpath.slice(2).split("/").some(segment => !segment || segment === "." || segment === ".."))) throw Error("Invalid authored package export key: " + subpath);
    if (value === null) rows.push({ subpath, target: null });
    else if (typeof value === "string") {
      if (!value.startsWith("./") || /[\\\0]/u.test(value) || value.slice(2).split("/").some(segment => !segment || segment === "." || segment === "..") || (value.includes("*") && !subpath.includes("*"))) throw Error("Invalid authored package export target: " + value);
      rows.push({ subpath, target: value });
    } else if (Array.isArray(value)) value.forEach(item => collect(subpath, item));
    else if (value && typeof value === "object") Object.values(value).forEach(item => collect(subpath, item));
    else throw Error("Unreadable authored package export target");
  };
  const value = manifest.exports;
  if (value !== undefined) {
    if (value && typeof value === "object" && !Array.isArray(value) && Object.keys(value).some(key => key.startsWith("."))) Object.entries(value).forEach(([key, target]) => collect(key, target));
    else collect(".", value);
  }
  const exportTargets = [...new Map(rows.map(row => [JSON.stringify(row), row])).values()].sort((a,b) => Buffer.compare(Buffer.from(a.subpath),Buffer.from(b.subpath)) || Buffer.compare(Buffer.from(String(a.target)),Buffer.from(String(b.target))));
  return { sourceOwner, exports: [...new Set(exportTargets.filter(row => row.target !== null).map(row => row.subpath))], exportTargets };
}
module.exports = { buildDependencyDirectionPolicy, authoredPackageExportAuthority };
