# Current Duplicate Package Owner Authority Audit

Read-only source inspection, no production writes/test/native run. Membership provider already deduplicates discovered canonical directory paths through workspaceCandidates found Set, per-scope byDirectory, and global all Set. Overlapping workspace globs selecting the SAME real owner should therefore yield one package and are a required positive case.

Public dependencyDirectionWorkspacePackages currently flatMaps each present discovered manifest to {name,owner,exports}; it does not reject duplicate names across distinct owners. Shared bootstrap snapshot similarly maps membership.packages then passes them to constructor. Later graphScope validator rejects duplicate package names, but that does not protect callers consuming inventory/snapshot/policy before graph validation. Constructor maps workspacePackages directly to WORKSPACE_PACKAGES and creates ownership patterns; a last-wins name Map elsewhere can discard authority if inventory was already ambiguous.

## Genuine boundary

First dedup discovery by canonical owner directory, not package name. Then admit manifest identities and assert name→owner is one-to-one before constructing any policy/name map. Two distinct present owners with the same package name must throw with name and both owners, even identical exports/roles; publication identity is ambiguous regardless of byte similarity. Same owner discovered through overlapping globs is one canonical owner and is accepted. Same normalized owner presented twice with inconsistent names/exports/role is malformed input; do not let first/last selection hide contradictory authority. Symlink aliases are already independently forbidden by authority census and must not become a way to collapse distinct logical owners silently.

Guard public inventory function and canonical snapshot boundary. Constructor accepting direct untrusted workspacePackages should also validate owner/name uniqueness before mapping/pattern generation, or require a genuinely validated owner type boundary; do not rely on callers knowing to invoke graphScope afterward. Share the canonical owner admission implementation behind existing Repo interface where feasible; do not introduce external framework/runtime dependencies or test helper facade.

## Closed portable test-first vectors

- overlapping-globs-same-owner: two positive patterns find one manifest; inventory/snapshot/policy report exactly one canonical package.
- multiple-scopes-same-owner: equivalent discovery of same canonical owner dedups, only if existing scope semantics permits it.
- distinct-owners-same-name: refuse inventory, snapshot and direct constructor before returning any policy.
- distinct-owners-same-name-different-role and distinct-owners-same-name-different-exports: refuse without overwriting either authority.
- same-owner-conflicting-name: direct construction input refused.
- same-owner-conflicting-role/exports: direct construction input refused.
- distinct-owners-distinct-names: accept, exact both owners/roles/exports remain.
- deleted-optional-owner: absence accepted; surviving owner retained exactly.
- present-malformed/linked-owner: refusal retained, never dedup or skip into acceptance.

Retain schema additionalProperties:false and unique IDs; malformed roles/manifests stay hostile independent cases. Compare candidate path roster against independent fast-glob and use actual shared policy loader for snapshot/constructor behavior. Duplicate rejection is owner-level invariance, not package visibility adjustment; private command-only/public artifact tests remain independently strict.

## Full current source receipts

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🟦️.ts

SHA-256 `8ff23493a16d33f4d7546de08d6b8534263542b7faaef5af63114ebcd8b9bd69`; 12745 bytes.

```
import { lstatSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { bunRepositoryMembership } from "../../🗂️workspaces/🟦️bun/🟦️.ts";

export type DependencyDirectionRule = Readonly<{ name: string; severity: string; from: { path: readonly string[]; pathNot?: readonly string[] }; to: { path: readonly string[]; pathNot?: readonly string[] } }>;
export type DependencyDirectionEdge = Readonly<{ rule: string; from: string; to: string }>;
export type DependencyDirectionGraphScope = Readonly<{ workspaceRoots: readonly string[]; workspacePackages: readonly Readonly<{ name: string; owner: string; exports: readonly string[] }>[]; excludedPaths: readonly string[]; nonFollowedPaths: readonly string[]; expectedSources: readonly string[] }>;

const sourceExtension = /\.(?:[cm]?[jt]s|[jt]sx)$/u;
const pathPatterns = new Map<string, RegExp>();
const pathMatches = (patterns: readonly string[], path: string): boolean => patterns.some((pattern) => {
  let expression = pathPatterns.get(pattern);
  if (!expression) { expression = new RegExp(pattern, "u"); pathPatterns.set(pattern, expression); }
  return expression.test(path);
});
const terminalPath = (scope: DependencyDirectionGraphScope, path: string): boolean => pathMatches([...scope.excludedPaths, ...scope.nonFollowedPaths], path);
const workspacePath = (scope: DependencyDirectionGraphScope, path: string): boolean => scope.workspaceRoots.some((root) => path === root || path.startsWith(`${root}/`));

function validateScope(scope: DependencyDirectionGraphScope): void {
  if (!scope || typeof scope !== "object") throw new Error("Dependency direction requires readable scope metadata");
  for (const key of ["workspaceRoots", "excludedPaths", "nonFollowedPaths", "expectedSources"] as const) {
    const paths = scope[key];
    if (!Array.isArray(paths) || paths.some((path) => typeof path !== "string" || !path)) throw new Error(`Dependency direction requires readable scope metadata: ${key}`);
    if (new Set(paths).size !== paths.length) throw new Error(`Dependency direction scope repeats a path: ${key}`);
  }
  if (!scope.workspaceRoots.length) throw new Error("Dependency direction requires declared workspace roots");
  if (!Array.isArray(scope.workspacePackages) || scope.workspacePackages.some((pkg) => !pkg || typeof pkg.name !== "string" || !pkg.name || typeof pkg.owner !== "string" || !workspacePath(scope, pkg.owner))) throw new Error("Dependency direction requires readable workspace package ownership");
  if (scope.workspacePackages.some((pkg) => !Array.isArray(pkg.exports) || !pkg.exports.length || pkg.exports.some((path: unknown) => typeof path !== "string" || !/^\.(?:\/.*)?$/u.test(path)) || new Set(pkg.exports).size !== pkg.exports.length)) throw new Error("Dependency direction requires readable authored package exports");
  if (new Set(scope.workspacePackages.map((pkg) => pkg.name)).size !== scope.workspacePackages.length) throw new Error("Dependency direction scope repeats a package name");
  for (const pattern of [...scope.excludedPaths, ...scope.nonFollowedPaths]) {
    try { new RegExp(pattern, "u"); } catch { throw new Error(`Dependency direction scope has an invalid path pattern: ${pattern}`); }
  }
}

/** 📦️ Reads present authored workspace owners; deleted owners contribute no packages. */
export function dependencyDirectionWorkspacePackages(root: string, excludedPaths: readonly string[]): DependencyDirectionGraphScope["workspacePackages"] {
  const membership = bunRepositoryMembership(root);
  for (const scope of membership.scopes) for (const path of scope.declaration.members) {
    if (path.startsWith("!") || /[*?]/u.test(path)) continue;
    const owner = join(root, scope.directory, path);
    let entry;
    try { entry = lstatSync(owner); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") continue; throw error; }
    if (entry.isDirectory()) lstatSync(join(owner, "package.json"));
  }
  const workspaces = membership.packages;
  return workspaces.filter((path) => !pathMatches(excludedPaths, path)).flatMap((owner) => {
    let directory;
    try { directory = lstatSync(join(root, owner)); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return []; throw error; }
    if (!directory.isDirectory()) throw new Error(`Dependency direction workspace owner must be a directory: ${owner}`);
    const manifest = JSON.parse(readFileSync(join(root, owner, "package.json"), "utf8"));
    if (typeof manifest.name !== "string" || !manifest.name) throw new Error(`Dependency direction requires an authored package name: ${owner}`);
    const exports = manifest.exports && typeof manifest.exports === "object" && !Array.isArray(manifest.exports) && Object.keys(manifest.exports).some((key) => key.startsWith(".")) ? Object.keys(manifest.exports).filter((key) => manifest.exports[key] !== null) : ["."];
    return [{ name: manifest.name, owner, exports }];
  });
}

/** 🗂️ Inventories every followed TypeScript/JavaScript root independently of resolver graph output. */
export function dependencyDirectionSourceInventory(root: string, roots: readonly string[], scope: DependencyDirectionGraphScope): readonly string[] {
  validateScope(scope);
  if (!roots.length || roots.some((path) => !workspacePath(scope, path) && (path.includes("/") || !sourceExtension.test(path)))) throw new Error("Dependency direction inventory requires declared workspace roots");
  const found = new Set<string>();
  const walk = (path: string): void => {
    if (terminalPath(scope, path)) return;
    for (const entry of readdirSync(join(root, path), { withFileTypes: true })) {
      const child = `${path}/${entry.name}`;
      if (terminalPath(scope, child)) continue;
      if (entry.isSymbolicLink()) {
        if (sourceExtension.test(child) || statSync(join(root, child)).isDirectory()) throw new Error(`Dependency direction cannot inventory a linked source: ${child}`);
        continue;
      }
      if (entry.isDirectory()) walk(child);
      else if (entry.isFile() && sourceExtension.test(child)) found.add(child);
    }
  };
  roots.forEach((path) => {
    const entry = lstatSync(join(root, path));
    if (entry.isSymbolicLink()) throw new Error(`Dependency direction cannot inventory a linked root: ${path}`);
    if (entry.isDirectory()) walk(path);
    else if (entry.isFile() && sourceExtension.test(path) && !terminalPath(scope, path)) found.add(path);
    else throw new Error(`Dependency direction inventory requires a source root: ${path}`);
  });
  return [...found].sort();
}

/** 📊️ Validates a complete resolver result and independently checks its framework dependency verdicts. */
export function dependencyDirectionEdges(report: unknown, rules: readonly DependencyDirectionRule[], scope: DependencyDirectionGraphScope): readonly DependencyDirectionEdge[] {
  const record = (value: unknown): Record<string, unknown> => {
    if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("Dependency direction graph requires an object");
    return value as Record<string, unknown>;
  };
  const array = (value: unknown): unknown[] => {
    if (!Array.isArray(value)) throw new Error("Dependency direction graph requires an array");
    return value;
  };
  const text = (value: unknown): string => {
    if (typeof value !== "string" || !value) throw new Error("Dependency direction graph requires a nonempty path or rule name");
    return value;
  };
  const matches = pathMatches;
  validateScope(scope);
  if (!scope.expectedSources.length) throw new Error("Dependency direction requires source inventory");
  const graph = record(report), modules = array(graph.modules), summary = record(graph.summary);
  const names = rules.map((rule) => rule.name);
  if (!names.length || new Set(names).size !== names.length || rules.some((rule) => rule.severity !== "error")) throw new Error("Dependency direction requires distinct strict rules");
  const usedRules = array(record(summary.ruleSetUsed).forbidden).map((value) => text(record(value).name));
  if (JSON.stringify([...usedRules].sort()) !== JSON.stringify([...names].sort())) throw new Error("Dependency direction graph used a different policy");
  if (!modules.length || summary.totalCruised !== modules.length) throw new Error("Dependency direction graph is empty or incomplete");
  const edges: DependencyDirectionEdge[] = [], problems: string[] = [], sources = new Map<string, string>();
  let dependencyCount = 0;
  for (const value of modules) {
    const module = record(value), from = text(module.source), dependencies = array(module.dependencies);
    const previous = sources.get(from), signature = JSON.stringify(dependencies);
    if (previous !== undefined && previous !== signature) throw new Error(`Dependency direction graph repeats a source with different dependencies: ${from}`);
    sources.set(from, signature);
    dependencyCount += dependencies.length;
    const applicable = rules.filter((rule) => matches(rule.from.path, from) && !matches(rule.from.pathNot ?? [], from));
    for (const value of dependencies) {
      const dependency = record(value), to = text(dependency.resolved);
      const specifier = text(dependency.module), pkg = scope.workspacePackages.find((pkg) => specifier === pkg.name || specifier.startsWith(`${pkg.name}/`));
      if (pkg) {
        const subpath = specifier === pkg.name ? "." : `.${specifier.slice(pkg.name.length)}`;
        if (subpath !== "." && subpath.slice(2).split(/[\\/]/u).some((segment) => !segment || segment === "." || segment === "..")) throw new Error(`Dependency direction graph uses an invalid authored package subpath: ${from} → ${specifier}`);
        const exported = pkg.exports.some((path) => new RegExp(`^${path.split("*").map((part) => part.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join(".*")}$`, "u").test(subpath));
        if (!exported) problems.push(`Dependency direction graph uses an undeclared authored package export: ${from} → ${specifier}`);
      }
      const installed = to.split("/").lastIndexOf("node_modules"), ownership = installed < 0 ? to : to.split("/").slice(installed).join("/");
      for (const rule of applicable) if (!matches(rule.to.pathNot ?? [], to) && (matches(rule.to.path, ownership) || (pkg && (matches(rule.to.path, specifier) || matches(rule.to.path, `${pkg.owner}/`))))) edges.push({ rule: rule.name, from, to });
    }
  }
  if (summary.totalDependenciesCruised !== dependencyCount) throw new Error("Dependency direction graph omits framework sources or dependencies");
  const missing = scope.expectedSources.filter((source) => !sources.has(source));
  if (missing.length) throw new Error(`Dependency direction graph omits inventoried sources: ${missing.join(", ")}`);
  for (const value of modules) {
    const module = record(value);
    for (const value of array(module.dependencies)) {
      const dependency = record(value), target = text(dependency.resolved);
      if (terminalPath(scope, target)) continue;
      const specifier = text(dependency.module);
      const authored = scope.workspacePackages.some((pkg) => specifier === pkg.name || specifier.startsWith(`${pkg.name}/`)) || specifier.startsWith("@semio-tech/");
      const local = /^(?:\.{1,2}(?:\/|$)|\/|[A-Za-z]:[\\/])/u.test(specifier) || workspacePath(scope, target);
      if (dependency.couldNotResolve === true && (local || authored)) problems.push(`Dependency direction graph has an unresolved workspace dependency: ${module.source} → ${specifier}`);
      if (workspacePath(scope, target) && (sourceExtension.test(target) || dependency.followable === true) && !sources.has(target)) problems.push(`Dependency direction graph omits followed workspace source: ${module.source} → ${target}`);
    }
  }
  if (problems.length) throw new Error([...new Set(problems)].join("\n"));
  const key = (edge: DependencyDirectionEdge): string => JSON.stringify([edge.rule, edge.from, edge.to]);
  const actual = [...new Set(edges.map(key))].sort();
  const reported = [...new Set(array(summary.violations).map((value) => {
    const violation = record(value);
    const rule = text(record(violation.rule).name);
    if (!names.includes(rule)) throw new Error("Dependency direction graph reports a different policy");
    return key({ rule, from: text(violation.from), to: text(violation.to) });
  }))].sort();
  if (JSON.stringify(actual) !== JSON.stringify(reported) || summary.error !== reported.length) throw new Error("Dependency direction resolver verdict disagrees with the source graph");
  return actual.map((value) => { const [rule, from, to] = JSON.parse(value) as [string, string, string]; return { rule, from, to }; });
}

```

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🚀️bootstrap/🟨️.cjs

SHA-256 `cf61f55c647cf505b27aa9641d9ec9c71ee657ae8fd9565ef2c60c0dada45eec`; 8238 bytes.

```
const fs = require("node:fs");
const { builtinModules } = require("node:module");
const { createHash } = require("node:crypto");
const { dirname, isAbsolute, join, relative, resolve } = require("node:path");
const { readBunRepositoryMembership } = require("../../../🗂️workspaces/🟦️bun/🟨️.cjs");
const { buildDependencyDirectionPolicy } = require("../🏗️construction/🟨️.cjs");
const slash = path => path.replaceAll("\\", "/");
const digest = text => createHash("sha256").update(text).digest("hex");
function freeze(value) {
  if (value && typeof value === "object" && !Object.isFrozen(value)) { Object.values(value).forEach(freeze); Object.freeze(value); }
  return value;
}
function rootPath(input) {
  if (typeof input !== "string" || !input || input.includes("\0") || /^[A-Za-z]:(?:$|[^\\/])/u.test(input) || input.split(/[\\/]/u).some(segment => segment === "." || segment === "..")) throw Error(`Dependency policy requires an unnormalized safe root: ${JSON.stringify(input)}`);
  const root = resolve(input), ancestry = [];
  for (let current = root; ; current = dirname(current)) { ancestry.push(current); if (current === dirname(current)) break; }
  for (const current of ancestry.reverse()) if (!fs.lstatSync(current).isDirectory() || fs.lstatSync(current).isSymbolicLink()) throw Error(`Dependency policy root ancestry must be a real directory: ${current}`);
  return root;
}
function locator(value) {
  const path = value?.metadata?.semio?.taxonomy;
  if (typeof path !== "string" || !path || /[\\\0]/u.test(path) || path.startsWith("/") || /^[A-Za-z]:/u.test(path) || path.split("/").some(segment => !segment || segment === "." || segment === "..")) throw Error("Dependency policy requires canonical metadata.semio.taxonomy");
  return path;
}

/** 🛡️ Captures fresh no-follow authority and builds policy without module-cached input.
 * @param {string} input
 * @param {import("./🟦️.ts").DependencyPolicyCaptureOptions} [options]
 * @returns {import("./🟦️.ts").DependencyPolicySnapshot}
 */
function loadDependencyDirectionPolicy(input, options = {}) {
  const check = () => { if (options.signal?.aborted) throw Error("Dependency policy authority capture cancelled"); };
  check();
  const root = rootPath(input), sources = new Map(), texts = new Map(), verifiedDirectories = new Set([root]);
  const record = (path, receipt) => { sources.set(path, { path, ...receipt }); if (sources.size % 300 === 0) options.onProgress?.({ phase: "capture", sources: sources.size }); check(); };
  const local = path => {
    const value = slash(relative(root, path));
    if (value === ".." || value.startsWith("../") || isAbsolute(value)) throw Error(`Dependency policy authority escapes root: ${path}`);
    return value;
  };
  const state = (path, freshAncestry = false) => {
    check();
    const parts = local(path).split("/").filter(Boolean); let current = root;
    for (const [index, part] of parts.entries()) {
      current = join(current, part);
      if (!freshAncestry && verifiedDirectories.has(current)) { if (index === parts.length - 1) return "directory"; continue; }
      let entry;
      try { entry = fs.lstatSync(current); } catch (error) { if (error.code === "ENOENT") { record(local(current), { kind: "missing" }); return "missing"; } throw error; }
      if (entry.isSymbolicLink()) throw Error(`Dependency policy authority must not follow a link: ${local(current)}`);
      if (index < parts.length - 1 && !entry.isDirectory()) throw Error(`Dependency policy authority ancestor must be a directory: ${local(current)}`);
      if (entry.isDirectory()) verifiedDirectories.add(current);
      if (index === parts.length - 1) return entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other";
    }
    return "directory";
  };
  const readText = path => {
    const name = local(path);
    if (texts.has(name)) return texts.get(name);
    if (state(path, true) !== "file") throw Error(`Dependency policy requires a regular authority file: ${name}`);
    const before = fs.lstatSync(path), descriptor = fs.openSync(path, fs.constants.O_RDONLY | (fs.constants.O_NOFOLLOW ?? 0));
    try {
      const opened = fs.fstatSync(descriptor);
      if (!opened.isFile() || opened.dev !== before.dev || opened.ino !== before.ino) throw Error(`Dependency policy authority changed while opening: ${name}`);
      const bytes = fs.readFileSync(descriptor), text = bytes.toString("utf8");
      if (state(path, true) !== "file") throw Error(`Dependency policy authority changed while reading: ${name}`);
      const after = fs.lstatSync(path);
      if (after.dev !== opened.dev || after.ino !== opened.ino || !Buffer.from(text).equals(bytes)) throw Error(`Dependency policy authority is not stable UTF-8: ${name}`);
      texts.set(name, text); record(name, { kind: "file", sha256: digest(bytes), bytes: bytes.length }); return text;
    } finally { fs.closeSync(descriptor); }
  };
  const list = path => {
    const name = local(path);
    if (state(path) !== "directory") throw Error(`Dependency policy requires a real authority directory: ${name}`);
    const entries = fs.readdirSync(path, { withFileTypes: true }).map(entry => ({ name: entry.name, kind: entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other" })).sort((a, b) => Buffer.compare(Buffer.from(a.name), Buffer.from(b.name)));
    record(name || ".", { kind: "directory", entries, sha256: digest(JSON.stringify(entries)) }); return entries;
  };
  const document = path => JSON.parse(readText(join(root, path)));
  document("nx.json");
  const project = document("📋️project.json"), taxonomyPath = locator(project), taxonomy = document(taxonomyPath);
  if (!taxonomy || typeof taxonomy !== "object" || Array.isArray(taxonomy) || !taxonomy.areaLayers || !taxonomy.dependencyDirections || !taxonomy.pathExclusions || !taxonomy.implementationLeafPolicy || !Array.isArray(taxonomy.forbiddenPathSegments)) throw Error("Dependency policy requires readable taxonomy authorities");
  const membership = readBunRepositoryMembership(root, { state, readText, list });
  for (const scope of membership.scopes) for (const path of scope.declaration.members) {
    if (path.startsWith("!") || /[*?]/u.test(path)) continue;
    const owner = join(root, scope.directory, path);
    if (state(owner) === "directory") readText(join(owner, "package.json"));
  }
  const excluded = Object.values(taxonomy.pathExclusions).map(row => row.path.replace(/\/$/u, ""));
  const workspacePackages = membership.packages.filter(owner => !excluded.some(prefix => owner === prefix || owner.startsWith(prefix + "/"))).map(owner => {
    const row = document(owner + "/package.json");
    if (typeof row.name !== "string" || !row.name) throw Error(`Workspace owner requires an authored package name: ${owner}`);
    const exports = row.exports && typeof row.exports === "object" && !Array.isArray(row.exports) && Object.keys(row.exports).some(key => key.startsWith(".")) ? Object.keys(row.exports).filter(key => row.exports[key] !== null) : ["."];
    return { owner, name: row.name, exports, ...(row.semio?.dependencyRole ? { dependencyRole: row.semio.dependencyRole } : {}) };
  });
  const pluginRoot = join(root, "✏️s/🔌️plugins"), pluginState = state(pluginRoot);
  if (pluginState !== "directory" && pluginState !== "missing") throw Error("Dependency policy plugin owner must be a real directory");
  const plugins = pluginState === "missing" ? [] : list(pluginRoot).filter(entry => { if (entry.kind === "symlink") throw Error(`Dependency policy plugin must not be linked: ${entry.name}`); return entry.kind === "directory"; }).map(entry => entry.name).sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
  const policy = buildDependencyDirectionPolicy({ taxonomy, plugins, workspacePackages, nodeBuiltins: builtinModules });
  options.onProgress?.({ phase: "complete", sources: sources.size }); check();
  return freeze({ workspaceRoot: root, taxonomyPath, taxonomy, workspacePackages, plugins, policy, sources: [...sources.values()].sort((a, b) => Buffer.compare(Buffer.from(a.path), Buffer.from(b.path))) });
}
module.exports = { loadDependencyDirectionPolicy };

```

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs

SHA-256 `3d7227450c702b34c7b9f9cebb61f82f23907e9d4cf2f2e9c3367c6f87c4fad4`; 25203 bytes.

```
/** 🧱️ Constructs every boundary selector from admitted immutable policy inputs.
 * @param {import("./🟦️.ts").DependencyPolicyConstructionInputs} inputs
 * @returns {import("../🚀️bootstrap/🟦️.ts").DependencyPolicySnapshot["policy"]}
 */
function buildDependencyDirectionPolicy({ taxonomy: TAXONOMY, plugins: PLUGINS, workspacePackages, nodeBuiltins }) {
const TECHNOLOGIES = ["compose", "🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"];
const BOOTSTRAP_TOOLING_ENTRY_PATH = "(^|/)(?:📜️script\\.ts|🏗️builder/🌐️vite/🟦️\\.ts|🧪️tests/🎚️config/🟦️\\.ts|(?:⚙️|🧪️)?(?:vite|vitest)\\.config\\.[cm]?[jt]s)$";
const RENDERER_HOST_ROOT = "^🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/(🎯️targets/⚛️react|🧱️elements)/";
const RESOLVED_NODE_BUILTIN_PATH = `^(?:${[...new Set(nodeBuiltins.map((name) => name.replace(/^node:/, "")))].map(escapeRegex).join("|")})(?:$|/)`;
const RENDERER_HOST_ALLOWED_RESOLVED_PATHS = [
  RENDERER_HOST_ROOT,
  "^🧰️framework/🔨️modules/🖱️ui/",
  "^🧰️framework/📦️packages/",
  "^node_modules/react(?:-dom)?/",
  "^(?:node:|vitest/)",
  "^node_modules/(?:vite|vitest)/",
  RESOLVED_NODE_BUILTIN_PATH,
];

/** ⚙️ Escapes a literal string for embedding inside a `RegExp` alternation. */
function escapeRegex(literal) {
  return literal.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/** 🏷️ Physical source ownership ends before installed package storage. */
function sourceOwnerPattern(owner) {
  return `^${escapeRegex(owner)}/(?!node_modules(?:/|$)|.*/node_modules(?:/|$))`;
}

/** 🏷️ Groups installed aliases that share the same direction while retaining exact package boundaries. */
function packageOwnerPatterns(names) {
  const packages = [...new Set(names)].sort();
  return packages.length ? [`(?:^(?:node_modules/)?|/node_modules/)(?:${packages.map(escapeRegex).join("|")})(?:$|/)`] : [];
}

/** 🗂️ Removes only redundant descendants already covered by an identical physical owner direction. */
function sourceOwnerPatterns(owners) {
  const roots = [...new Set(owners)];
  return roots.filter((owner) => !roots.some((root) => owner !== root && owner.startsWith(`${root}/`))).map(sourceOwnerPattern);
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
    "vitest/importMeta",
    "fs",
    "node:path",
  ];
  const forbiddenRendererTargets = [
    "node_modules/three/build/three.module.js",
    "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts",
    "🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts",
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
const IGNORED_GRAPH_PATHS = TAXONOMY.implementationLeafPolicy.ignoredPathPatterns.filter((pattern) => pattern !== "**/node_modules").map((pattern) => `(^|/)${escapeRegex(pattern.replace(/^\*\*\//u, ""))}(/|$)`)
  .concat(OPAQUE_PATHS.map((prefix) => `^${escapeRegex(prefix)}(/|$)`));

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

/** 🧬️ Builds declared semantic role rules from taxonomy owners and workspace package contributions. */
function declaredDependencyDirectionRules() {
  const { roles, rules } = TAXONOMY.dependencyDirections;
  for (const pkg of WORKSPACE_PACKAGES) if (pkg.dependencyRole && !roles[pkg.dependencyRole]) throw new Error(`Unknown dependency role ${pkg.dependencyRole} in ${pkg.dir}`);
  const patterns = (roleIds) => roleIds.flatMap((id) => {
    const role = roles[id];
    if (!role) throw new Error(`Unknown dependency direction role ${id}`);
    const packages = WORKSPACE_PACKAGES.filter((pkg) => pkg.dependencyRole === id || role.ownerPaths.some((owner) => pkg.dir === owner || pkg.dir.startsWith(`${owner}/`)));
    return sourceOwnerPatterns(role.ownerPaths.concat(packages.map((pkg) => pkg.dir))).concat(packageOwnerPatterns(packages.map((pkg) => pkg.name).concat(role.externalPackages)));
  });
  return Object.entries(rules).map(([name, rule]) => ({ name, severity: "error", comment: "Semantic owners must follow their declared dependency direction through resolved paths, package aliases and subpaths", from: { path: patterns(rule.fromRoles) }, to: { path: patterns(rule.toRoles) } }));
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
 * `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript` direct-renderer imports — real,
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

return {
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
        path: ["^✏️s/🔌️plugins/"].concat(S_PACKAGES.filter((pkg) => pkg.dir.startsWith("✏️s/🔌️plugins/")).map((pkg) => `(?:^(?:node_modules/)?|/node_modules/)${escapeRegex(pkg.name)}(?:$|/)`)),
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
module.exports = { buildDependencyDirectionPolicy };

```

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs

SHA-256 `f479f2ae459474df68b1c15da4ad2de4fbb7e517ed3b7496ac2bae9f32075ae1`; 9192 bytes.

```
const fs = require("node:fs");
const { dirname, isAbsolute, join, relative, resolve } = require("node:path");
const { ownsPayload } = require("../📦️payload/🟨️.cjs");
const slash = (path) => path.replaceAll("\\", "/");
const opaque = new Set(["🗑️generated", "node_modules", "target", "dist", "build", "storybook-static", "temp", "coverage", "🔌️plugin-modules", "compose", "🧫️fixtures"]);

/** 🧬️ Admits authored native patterns independently of package identity or product roles.
 * @returns {import("./🟦️.ts").BunWorkspaceDeclarationV1}
 */
function parseBunWorkspaceDeclaration(value) {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw Error("Bun workspace declaration requires an object");
  if (Object.keys(value).some(key => !["schemaVersion", "members", "owners"].includes(key)) || value.schemaVersion !== 1 || !Array.isArray(value.members) || !value.members.length || !Array.isArray(value.owners)) throw Error("Invalid Bun workspace declaration");
  for (const [key, values] of [["members", value.members], ["owners", value.owners]]) if (values.some(path => typeof path !== "string" || !path || path === "!" || /^!?\//u.test(path) || /^!?[A-Za-z]:/u.test(path) || path.includes("\\") || path.includes("\0") || key === "owners" && path !== "*/package.json") || new Set(values).size !== values.length) throw Error("Invalid Bun workspace patterns");
  if (!value.members.some(path => !path.startsWith("!"))) throw Error("Bun workspace needs a positive source pattern");
  return { schemaVersion: 1, members: [...value.members], owners: [...value.owners] };
}
const match = (pattern, path) => {
  const parts = pattern.split("/"), source = parts.map((part, index) => part === "**" ? index === parts.length - 1 ? ".*" : "(?:[^/]+/)*" : part.split("").map(c => c === "*" ? "[^/]*" : c === "?" ? "[^/]" : c.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("") + (index === parts.length - 1 ? "" : "/")).join("");
  return new RegExp("^" + source + "$").test(path);
};
const bounded = (root, path) => {
  const full = resolve(root, path), local = relative(root, full);
  if (local === ".." || local.startsWith("../") || local.startsWith("..\\") || isAbsolute(local)) throw Error(`Bun workspace escapes repository: ${path}`);
  return full;
};

/** 📁️ Expands current physical candidates without cached package membership. */
function workspaceCandidates(root, scope, operations, inventory) {
  const positive = scope.declaration.members.filter(path => !path.startsWith("!")), negative = scope.declaration.members.filter(path => path.startsWith("!")).map(path => path.slice(1)), found = new Set();
  for (const pattern of positive) {
    const prefix = pattern.split(/[*?]/u)[0].replace(/\/$/u, ""), start = bounded(root, join(scope.directory, prefix));
    const state = operations.state(start);
    if (state === "missing") continue;
    if (state !== "directory") throw Error(`Bun source owner must be a real directory: ${start}`);
    const walk = (directory) => {
      const known = inventory.get(directory);
      if (known) return known;
      const entries = operations.list(directory), paths = [];
      if (entries.some(entry => entry.name === "package.json")) paths.push(directory);
      for (const entry of entries) if (!entry.name.startsWith(".") && !opaque.has(entry.name)) {
        if (entry.kind === "symlink") throw Error(`Bun source directory is a symlink: ${join(directory, entry.name)}`);
        if (entry.kind === "directory") paths.push(...walk(join(directory, entry.name)));
      }
      inventory.set(directory, paths);
      return paths;
    };
    for (const directory of walk(start)) {
      const local = slash(relative(resolve(root, scope.directory), directory));
      if (match(pattern, local) && !negative.some(path => match(path, local + "/") || match(path, local))) found.add(slash(relative(root, directory)));
    }
  }
  return [...found].sort((a, b) => a.localeCompare(b));
}
function workspacePackages(root, scope, operations, inventory, documents) {
  const document = (path) => {
    if (!documents.has(path)) documents.set(path, JSON.parse(operations.readText(bounded(root, path))));
    return documents.get(path);
  };
  const paths = workspaceCandidates(root, scope, operations, inventory), candidates = paths.map(relDir => ({ relDir, absDir: resolve(root, relDir), ...document(relDir + "/package.json") })), byDirectory = new Map(candidates.map(candidate => [candidate.absDir, candidate]));
  return candidates.filter(candidate => {
    for (let parent = dirname(candidate.absDir); parent !== root && parent !== dirname(parent); parent = dirname(parent)) {
      const owner = byDirectory.get(parent);
      if (owner) return !ownsPayload(owner, candidate, operations);
    }
    return true;
  }).map(candidate => candidate.relDir);
}

/** 🗂️ Resolves source membership through an explicit physical operations interface.
 * @returns {import("./🟦️.ts").BunRepositoryMembership}
 */
function readBunRepositoryMembership(root, operations) {
  const document = JSON.parse(operations.readText(join(root, "package.json"))), declaration = parseBunWorkspaceDeclaration(document.semio?.workspace), paths = new Set(["package.json"]);
  for (const pattern of declaration.owners) {
    if (pattern !== "*/package.json") throw Error(`Unsupported physical owner recipe: ${pattern}`);
    for (const entry of operations.list(root)) if (!entry.name.startsWith(".") && !opaque.has(entry.name)) {
      if (entry.kind === "symlink") throw Error(`Bun installation owner is a symlink: ${entry.name}`);
      if (entry.kind === "directory" && operations.state(join(root, entry.name, "package.json")) !== "missing") paths.add(entry.name + "/package.json");
    }
  }
  const scopes = [], inventory = new Map(), documents = new Map(), all = new Set();
  for (const path of [...paths].sort()) {
    const row = JSON.parse(operations.readText(join(root, path)));
    if (row.semio?.workspace === undefined) continue;
    const d = dirname(path), directory = d === "." ? "." : slash(d), admitted = parseBunWorkspaceDeclaration(row.semio.workspace);
    if (!Array.isArray(row.workspaces) || JSON.stringify(row.workspaces.slice(0, admitted.members.length)) !== JSON.stringify(admitted.members) || row.workspaces.slice(admitted.members.length).some(path => typeof path !== "string" || !path.startsWith("!"))) throw Error(`Bun native source patterns drift: ${path}`);
    for (const pattern of admitted.members) bounded(root, join(directory, pattern.replace(/^!/u, "").split(/[*?]/u)[0]));
    const scope = { directory, manifest: path, lock: directory === "." ? "bun.lock" : directory + "/bun.lock", declaration: admitted };
    const packages = workspacePackages(root, scope, operations, inventory, documents);
    for (const exclusion of row.workspaces.slice(admitted.members.length)) if (packages.some(path => match(exclusion.slice(1), slash(relative(resolve(root, directory), resolve(root, path)))))) throw Error(`Bun native exclusion hides an independent owner: ${path} ${exclusion}`);
    packages.forEach(path => all.add(path)); scopes.push(scope);
  }
  return { scopes, packages: [...all].sort((a, b) => a.localeCompare(b)) };
}
const nativeOperations = {
  state(path) {
    try {
      const entry = fs.lstatSync(path);
      return entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other";
    } catch (error) { if (error.code === "ENOENT") return "missing"; throw error; }
  },
  list: path => fs.readdirSync(path, { withFileTypes: true }).map(entry => ({ name: entry.name, kind: entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other" })),
  readText(path) {
    const ancestry = [];
    for (let current = dirname(path); ; current = dirname(current)) { ancestry.push(current); if (current === dirname(current)) break; }
    for (const current of ancestry.reverse()) if (this.state(current) !== "directory") throw Error(`Bun workspace ancestor must be a real directory: ${current}`);
    if (this.state(path) !== "file") throw Error(`Bun workspace requires a regular manifest: ${path}`);
    return fs.readFileSync(path, "utf8");
  },
};
const bunRepositoryMembership = root => readBunRepositoryMembership(root, nativeOperations);
const bunWorkspacePackages = (root, scope) => workspacePackages(root, scope, nativeOperations, new Map(), new Map());
const bunWorkspaceNativePatterns = (root, scope) => {
  const inventory = new Map(), selected = new Set(workspacePackages(root, scope, nativeOperations, inventory, new Map()));
  return [...scope.declaration.members, ...workspaceCandidates(root, scope, nativeOperations, inventory).filter(path => !selected.has(path)).map(path => "!" + slash(relative(resolve(root, scope.directory), resolve(root, path))).replace(/[?*\[\]{}]/g, "\\$&"))];
};
module.exports = { parseBunWorkspaceDeclaration, readBunRepositoryMembership, bunRepositoryMembership, bunWorkspacePackages, bunWorkspaceNativePatterns, discoverBunWorkspaces: root => bunRepositoryMembership(root).scopes, bunRepositoryPackages: root => bunRepositoryMembership(root).packages };

```

## Independent current constructor runtime observation

Executed Bun inline direct existing CJS buildDependencyDirectionPolicy with actual current taxonomy, node builtin list, plugins=[] and two in-memory manifests; no files/generated/native output. Distinct synthetic owners ✏️s/a and ✏️s/b using the SAME @semio-test/shared name were accepted and returned38rules. Identical same owner/name repeated also accepted38rules. This confirms current constructor ambiguity admission at runtime before High guard publication. Positive overlapping discovery itself was not executed by this lane; existing membership Set semantics are retained source evidence. This observation narrows the earlier no-runtime statement: no native/compiler tests ran, but the pure constructor was exercised explicitly.
