// 🕸️ Static reconstruction of the build graph (BuildGraphV1) from tracked project manifests, nx.json named inputs, package.json edges and the playground grammar; fixture-free input for the Bazel adapter generator.
import { mkdirSync, readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const TICKET = join(import.meta.dir, "..");
const OUT_DIR = join(TICKET, "🗑️generated", "bzl-lazy");
const strip = (s: string) => s.replace(/️/g, "");
const PROJECT_BASENAME = "\u{1F4CB}️project.json";
const POLICY = JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"), "utf8"));
const NX = JSON.parse(readFileSync(join(ROOT, "nx.json"), "utf8"));
const NX_IGNORED_SEGMENTS = new Set(["🧫️fixtures", "🧫️examples", "🧪️fixtures", "🧪️examples"].map(strip));

const git = Bun.spawnSync(["git", "-c", "core.quotepath=false", "ls-files", "-z"], { cwd: ROOT });
const tracked: string[] = new TextDecoder().decode(git.stdout).split("\0").filter((f) => f && existsSync(join(ROOT, f)));

const matchesCommand = (name: string, commands: string[]) => commands.some((c) => name === c || name.startsWith(c + "-"));
const matchesUncached = (name: string) => POLICY.uncachedExact.includes(name) || matchesCommand(name, POLICY.uncached);
const mutatingName = (name: string) => /(?:^|-)(?:clean|gc|prune|setup|fuzz)(?:-|$)/.test(name);
const liveName = (name: string) => /(?:^|-)(?:e2e|live)(?:-|$)/.test(name);
const cacheableFamily = (name: string) =>
  /^(test(?:-|$)|check(?:-|$)|lint(?:-|$)|typecheck(?:-|$)|format-check(?:-|$)|generate(?:-|$)|schema(?:-|$)|verify(?:-|$)|stdio(?:-|$)|build(?:-|$)|wasm$|native-build$|package$|extension-package$|cpp-(?:configure|build|test)$|graph-check$|policy-check$|artifact-check$|artifact-package-contract$|toolchain$)/.test(name) ||
  /(?:^|-)contract(?:-|$)/.test(name) ||
  (/^generator(?:-|$)/.test(name) && !name.includes("generator-inputs"));
const verifyCommand = (t: any) => /(?:^|[\s"'])verify(?:[\s"']|$)/.test(t?.options?.command ?? "");

function effective(name: string, target: any): { cache: boolean; continuous: boolean } {
  if (matchesCommand(name, POLICY.continuous)) return { cache: false, continuous: true };
  if (target.cache === false) return { cache: false, continuous: target.continuous === true };
  if (matchesUncached(name)) return { cache: false, continuous: target.continuous === true };
  if (POLICY.cachedExact.includes(name)) return { cache: true, continuous: target.continuous === true };
  if (mutatingName(name) || liveName(name)) return { cache: false, continuous: target.continuous === true };
  if (cacheableFamily(name) || verifyCommand(target)) return { cache: true, continuous: target.continuous === true };
  return { cache: target.cache !== false, continuous: target.continuous === true };
}

function fileStatus(file: string): string {
  const segs = file.split("/").map(strip);
  if (segs[0] === strip(".🧬semio")) return "ticket-excluded";
  if (segs.includes("pkg")) return "nxignore-pkg";
  if (segs.slice(0, -1).some((s) => NX_IGNORED_SEGMENTS.has(s))) return "nxignore-fixture-or-example";
  if (segs[segs.length - 1] !== strip(PROJECT_BASENAME)) return "not-plugin-include-basename";
  return "nx-project";
}

const execution = (options: any, kind: "command" | "noop"): any => {
  if (kind === "noop") return { kind: "noop" };
  const e: any = { kind: "command" };
  if (typeof options.command === "string") e.command = options.command;
  else if (Array.isArray(options.commands)) e.commands = options.commands;
  else e.command = "true";
  if (typeof options.cwd === "string") e.cwd = options.cwd;
  if (options.env && typeof options.env === "object") e.env = options.env;
  if (typeof options.forwardAllArgs === "boolean") e.forwardAllArgs = options.forwardAllArgs;
  if (typeof options.parallel === "boolean") e.parallel = options.parallel;
  if (Array.isArray(options.args)) e.args = options.args;
  return e;
};

const decision = (klass: string, reason: string) => ({ semio: { cache: { version: 1, class: klass, reason } } });

const projects: Record<string, any> = {};
const roots = new Map<string, string>();
const packageOwner = new Map<string, string>();
const problems: Record<string, string[]> = { parseErrors: [], unnamed: [], duplicateName: [], duplicateRoot: [] };
const projectFiles = tracked.filter((f) => f.endsWith("project.json") && fileStatus(f) === "nx-project").sort();
const packages = new Map<string, any>();

for (const file of projectFiles) {
  let json: any;
  try { json = JSON.parse(readFileSync(join(ROOT, file), "utf8")); } catch { problems.parseErrors.push(file); continue; }
  const root = dirname(file) || ".";
  let pkg: any;
  const pkgPath = join(ROOT, root, "package.json");
  if (existsSync(pkgPath)) { try { pkg = JSON.parse(readFileSync(pkgPath, "utf8")); } catch {} }
  const name: string | undefined = json.name ?? pkg?.name;
  if (!name) { problems.unnamed.push(file); continue; }
  if (projects[name]) { problems.duplicateName.push(name); continue; }
  if (roots.has(root)) { problems.duplicateRoot.push(root); continue; }
  const targets: Record<string, any> = {};
  for (const [tname, t] of Object.entries<any>(json.targets ?? {})) {
    const defaults = POLICY.targetDefaults?.[tname] ?? {};
    const merged = { ...defaults, ...t };
    const eff = effective(tname, merged);
    const noop = t.executor === "nx:noop" || t.executor === "repo:noop";
    const target: any = { execution: execution(t.options ?? {}, noop ? "noop" : "command") };
    const inputs = t.inputs ?? defaults.inputs ?? (eff.cache === false && t.cache === false ? ["default"] : undefined);
    if (inputs !== undefined) target.inputs = inputs;
    const outputs = t.outputs ?? defaults.outputs;
    if (outputs !== undefined) target.outputs = outputs;
    const dependsOn = t.dependsOn ?? defaults.dependsOn;
    if (Array.isArray(dependsOn) && dependsOn.length) target.dependsOn = dependsOn;
    if (t.cache !== undefined || defaults.cache !== undefined || eff.cache) target.cache = t.cache !== undefined ? t.cache : eff.cache;
    if (eff.cache === false && t.cache === undefined) target.cache = false;
    if (eff.continuous) target.continuous = true;
    if (typeof t.parallelism === "boolean") target.parallelism = t.parallelism;
    if (t.metadata) target.metadata = t.metadata;
    if (t.configurations) {
      target.configurations = Object.fromEntries(Object.entries<any>(t.configurations).map(([c, o]) => [c, { execution: Object.fromEntries(Object.entries(execution(o, "command")).filter(([k]) => ["args", "env", "cwd", "forwardAllArgs"].includes(k)).concat(typeof o.command === "string" ? [["command", o.command]] : [])) }]));
      if (t.defaultConfiguration) target.defaultConfiguration = t.defaultConfiguration;
    }
    targets[tname] = target;
  }
  const project: any = { name, root, targets };
  if (json.tags) project.tags = json.tags;
  if (json.namedInputs) project.namedInputs = json.namedInputs;
  if (json.implicitDependencies) project.implicitDependencies = json.implicitDependencies;
  projects[name] = project;
  roots.set(root, name);
  if (pkg) { packages.set(name, pkg); if (pkg.name) packageOwner.set(pkg.name, name); }
}

const dependencies: { source: string; target: string; type: string }[] = [];
const seen = new Set<string>();
const addEdge = (s: string, t: string, type: string) => { if (s === t || !projects[s] || !projects[t]) return; const k = s + "\0" + t; if (seen.has(k)) return; seen.add(k); dependencies.push({ source: s, target: t, type }); };
for (const [name, pkg] of packages) for (const field of ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"]) for (const dep of Object.keys(pkg[field] ?? {})) { const owner = packageOwner.get(dep); if (owner) addEdge(name, owner, "static"); }
for (const [name, p] of Object.entries<any>(projects)) for (const dep of p.implicitDependencies ?? []) if (typeof dep === "string") addEdge(name, dep, "implicit");
for (const p of Object.values<any>(projects)) delete p.implicitDependencies;

const cargoFiles = tracked.filter((f) => f.endsWith("/Cargo.toml") && !f.startsWith(".🧬semio") && fileStatus(f.replace(/Cargo\.toml$/, PROJECT_BASENAME)) === "nx-project");
const rows: { variant: string; crate: string; mcp: boolean; dist: boolean }[] = [];
for (const f of cargoFiles) {
  const text = readFileSync(join(ROOT, f), "utf8");
  if (!/component-kind\s*=\s*"(plugin|extension)"/.test(text) || !/\[package\.metadata\.component\][\s\S]*?\bpackage\s*=/.test(text)) continue;
  for (const block of text.split(/\[\[package\.metadata\.semio\.playground\]\]/).slice(1)) {
    const body = block.split(/\n\[/)[0];
    const variant = body.match(/variant\s*=\s*"([^"]+)"/)?.[1];
    if (variant) rows.push({ variant, crate: dirname(f), mcp: /mcpHost/.test(body), dist: /distDir/.test(body) });
  }
}
const OSDEV = "@semio-tech/framework-os-dev", REGISTRY = "@semio-tech/plugin-registry";
let synthesized = 0;
const synthTarget = (owner: string, name: string, target: any, klass: [string, string]) => {
  const project = projects[owner];
  if (!project || project.targets[name]) return;
  target.metadata = { ...(target.metadata ?? {}), ...decision(klass[0], klass[1]) };
  project.targets[name] = target;
  synthesized++;
};
const cmd = (command: string, extra: any = {}) => ({ kind: "command", command, forwardAllArgs: false, ...extra });
const variants = [...new Set(rows.map((r) => r.variant))].sort();
for (const variant of variants) {
  const row = rows.find((r) => r.variant === variant)!;
  const catalog = `${REGISTRY}:session-${variant}`;
  synthTarget(REGISTRY, `session-${variant}`, { execution: cmd(`bun ./📜️script.ts session ${variant}`), cache: true, inputs: ["default", { dependentTasksOutputFiles: "**/*", transitive: true }], outputs: [`{workspaceRoot}/.cache/b/p/m/browser/{options.env.SEMIO_BUILD_MODE}/sessions/${variant}`] }, ["deterministic", "pure-function-of-inputs"]);
  for (const profile of ["prototype", "release"]) {
    for (const engine of ["react", "wgpu"]) {
      const prepare = `prepare-${variant}-${engine}-${profile}`, activate = `activate-${variant}-${engine}-${profile}`;
      synthTarget(OSDEV, prepare, { execution: cmd(`bun ./📜️script.ts prepare ${variant} ${engine} ${profile}`), cache: false, outputs: [`{workspaceRoot}/.cache/b/p/m/browser/${profile}/modules/${variant}`], dependsOn: [catalog] }, ["driver-owned", "freshness-owned-by-driver"]);
      synthTarget(OSDEV, activate, { execution: cmd(`bun ./📜️script.ts activate ${variant} ${engine} ${profile}`), cache: engine === "react", outputs: [`{workspaceRoot}/.cache/b/p/m/browser/${profile}/runtime/${engine}/${variant}`], inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }], dependsOn: [prepare], parallelism: false } as any, engine === "react" ? ["deterministic", "pure-function-of-inputs"] : ["side-effecting", "publishes-live-state"]);
      synthTarget(OSDEV, `serve-${variant}-${engine}-${profile}`, { execution: cmd(`bun ./📜️script.ts serve ${variant} ${engine} ${profile}`), cache: false, continuous: true, outputs: [], dependsOn: [engine === "react" ? catalog : activate] }, ["continuous", "serves-until-stopped"]);
      synthTarget(OSDEV, `dev-${variant}-${engine}-${profile}`, { execution: cmd(`bun ./📜️script.ts serve ${variant} ${engine} ${profile}`), cache: false, continuous: true, outputs: [], dependsOn: [activate] }, ["continuous", "serves-until-stopped"]);
    }
    synthTarget(OSDEV, `prepare-${variant}-native-${profile}`, { execution: cmd(`bun ./📜️script.ts publish ${variant} ${profile}`), cache: false, outputs: [`{workspaceRoot}/.cache/b/p/m/browser/${profile}/native/${variant}`], dependsOn: [catalog] }, ["driver-owned", "freshness-owned-by-driver"]);
    for (const op of ["run", "smoke"]) synthTarget(OSDEV, `${op}-${variant}-native-${profile}`, { execution: cmd(`bun ./📜️script.ts run ${variant} ${profile}${op === "smoke" ? " --smoke" : ""}`, { forwardAllArgs: true }), cache: false, outputs: [], dependsOn: [`prepare-${variant}-native-${profile}`] }, ["live", "drives-running-process"]);
    if (row.mcp) for (const transport of ["stdio", "http"]) synthTarget(OSDEV, `mcp-${variant}-${transport}-${profile}`, { execution: cmd(`bun ./📜️script.ts mcp ${variant} ${profile} ${transport}`, { forwardAllArgs: true }), cache: false, continuous: true, outputs: [] }, ["continuous", "serves-until-stopped"]);
  }
  const build = `build-${variant}-react-release`;
  synthTarget(OSDEV, build, { execution: cmd(`bun ../../🚚️distribution/📜️script.ts build ${variant} react release`, { forwardAllArgs: true }), cache: false, inputs: ["production", "^production", { dependentTasksOutputFiles: "**/*", transitive: true }, { env: "S_HUB_URL" }, { env: "S_DATA_DIR" }], outputs: [`{workspaceRoot}/.cache/b/p/m/vite/release/${variant}/out`], dependsOn: [catalog, `prepare-${variant}-react-release`] }, ["driver-owned", "freshness-owned-by-driver"]);
  synthTarget(OSDEV, `build-${variant}-react`, { execution: cmd(`bun ../../🚚️distribution/📜️script.ts build ${variant} react release`, { forwardAllArgs: true }), cache: false, outputs: [`{workspaceRoot}/.cache/b/p/m/vite/{options.env.SEMIO_BUILD_MODE}/${variant}/out`], dependsOn: [catalog, `prepare-${variant}-react-release`], configurations: { prototype: { execution: { command: `bun ../../🚚️distribution/📜️script.ts build ${variant} react prototype` } }, release: { execution: { command: `bun ../../🚚️distribution/📜️script.ts build ${variant} react release` } } } }, ["driver-owned", "freshness-owned-by-driver"]);
  for (const engine of ["react", "wgpu"]) synthTarget(OSDEV, `prepare-${variant}-${engine}`, { execution: cmd(`bun ./📜️script.ts prepare ${variant} ${engine} prototype`), cache: false, outputs: [], dependsOn: [catalog], configurations: { prototype: { execution: { command: `bun ./📜️script.ts prepare ${variant} ${engine} prototype` } }, release: { execution: { command: `bun ./📜️script.ts prepare ${variant} ${engine} release` } } } }, ["driver-owned", "freshness-owned-by-driver"]);
}
const crates = new Map<string, typeof rows>();
for (const r of rows) crates.set(r.crate, [...(crates.get(r.crate) ?? []), r]);
for (const [crate, crateRows] of crates) {
  const owner = roots.get(crate);
  if (!owner) continue;
  for (const r of crateRows) synthTarget(owner, `build-${r.variant}-site`, { execution: { kind: "noop" }, cache: false, outputs: [`{workspaceRoot}/.cache/b/p/m/vite/{options.env.SEMIO_BUILD_MODE}/${r.variant}/out`], dependsOn: [`${OSDEV}:build-${r.variant}-react`] }, ["aggregate", "orders-prerequisites"]);
  synthTarget(owner, "build", { execution: { kind: "noop" }, cache: false, outputs: [], dependsOn: crateRows.map((r) => `build-${r.variant}-site`) }, ["aggregate", "orders-prerequisites"]);
}
if (rows.some((r) => r.dist)) synthTarget(OSDEV, "build-all-playground-cdn-sites", { execution: cmd("bun ../../🚚️distribution/📜️script.ts complete"), cache: false, outputs: [], dependsOn: rows.filter((r) => r.dist).map((r) => `build-${r.variant}-react`) }, ["side-effecting", "publishes-remote"]);

const graph = { version: 1, projects, dependencies, namedInputs: NX.namedInputs };
const targetCount = Object.values<any>(projects).reduce((a, p) => a + Object.keys(p.targets).length, 0);
mkdirSync(OUT_DIR, { recursive: true });
writeFileSync(join(OUT_DIR, "graph.json"), JSON.stringify(graph));
const summary = { projectFiles: projectFiles.length, projects: Object.keys(projects).length, targets: targetCount, synthesized, variants: variants.length, playgroundRows: rows.length, edges: dependencies.length, problems: Object.fromEntries(Object.entries(problems).map(([k, v]) => [k, v.length])) };
writeFileSync(join(OUT_DIR, "graph-summary.json"), JSON.stringify({ ...summary, problemDetail: problems }, null, 1));
console.log(JSON.stringify(summary));
