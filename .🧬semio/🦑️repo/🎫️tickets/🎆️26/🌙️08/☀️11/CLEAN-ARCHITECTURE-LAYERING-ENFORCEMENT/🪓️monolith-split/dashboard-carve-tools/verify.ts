// ✅ Static checks of the dashboard carve: manifests, workspace membership, owner rows, launch routes, mounted paths, ownership and the dependency DAG.
import {existsSync, readFileSync, readdirSync, statSync} from "node:fs";
import {join, relative, resolve, dirname} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const D = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard";
const crates: Record<string, {node: string[]; id: string}> = {
  "semio-framework-repo-dashboard-core": {node: ["🧱️core"], id: "core"},
  "semio-framework-repo-dashboard-launch": {node: ["🧾️launch"], id: "launch"},
  "semio-framework-repo-dashboard-repo-domain": {node: ["🏛️repo-domain"], id: "repo-domain"},
  "semio-framework-repo-dashboard-registry": {node: ["🎮️registry"], id: "registry"},
  "semio-framework-repo-dashboard-inventory": {node: ["📚️inventory"], id: "inventory"},
  "semio-framework-repo-dashboard-wire": {node: ["🌀️daemon", "🧵️wire"], id: "wire"},
  "semio-framework-repo-dashboard-labels": {node: ["🖥️terminal", "🌐️labels"], id: "labels"},
  "semio-framework-repo-dashboard-preferences": {node: ["⚙️preferences"], id: "preferences"},
  "semio-framework-repo-dashboard-daemon": {node: ["🌀️daemon"], id: "daemon"},
  "semio-framework-repo-dashboard-terminal": {node: ["🖥️terminal"], id: "terminal"},
  "semio-framework-repo-dashboard-cli": {node: ["🧭️cli"], id: "cli"},
};
let failures = 0;
const check = (ok: boolean, what: string) => { if (!ok) { failures++; console.log("FAIL", what); } };
const manifestOf = (node: string[]) => join(D, ...node, "📦️packages", "🦀️rust", "Cargo.toml");
const graph = new Map<string, string[]>();
const packages = new Map<string, string>();
const all: [string, string[]][] = [...Object.entries(crates).map(([name, c]) => [name, c.node] as [string, string[]]), ["semio-framework-repo-dashboard", []]];
for (const [name, node] of all) {
  const file = manifestOf(node);
  const toml = Bun.TOML.parse(readFileSync(file, "utf8")) as any;
  check(toml.package.name === name, `${name}: package name`);
  packages.set(resolve(dirname(file)), name);
  const pkgDir = dirname(file);
  check(resolve(pkgDir, toml.package.workspace) === ROOT, `${name}: package.workspace resolves to the repo root (${resolve(pkgDir, toml.package.workspace)})`);
  const lib = resolve(pkgDir, toml.lib.path);
  check(existsSync(lib), `${name}: lib path ${relative(ROOT, lib)}`);
  if (name !== "semio-framework-repo-dashboard") {
    const meta = toml.package.metadata.semio;
    check(meta.id === "repo-dashboard-" + crates[name]!.id && meta.role === "tool", `${name}: metadata`);
    check(JSON.stringify(Object.keys(meta["rust-source-boundary"]).sort()) === JSON.stringify(["maxActiveLines", "maxSourceBytes", "maxSourceFiles"]) && meta["rust-source-boundary"].maxSourceBytes === 262144 && meta["rust-source-boundary"].maxSourceFiles === 32 && meta["rust-source-boundary"].maxActiveLines === 4000, `${name}: boundary keys`);
    check(toml.lints?.workspace === true, `${name}: lints`);
  }
  const edges: string[] = [];
  for (const table of [toml.dependencies, toml["dev-dependencies"]]) for (const [key, spec] of Object.entries(table ?? {}) as [string, any][]) {
    if (typeof spec.path !== "string") continue;
    const target = resolve(pkgDir, spec.path, "Cargo.toml");
    check(existsSync(target), `${name}: dependency ${key} path`);
    if (!existsSync(target)) continue;
    const dep = Bun.TOML.parse(readFileSync(target, "utf8")) as any;
    check(dep.package.name === (spec.package ?? key), `${name}: dependency ${key} names ${dep.package.name}`);
    edges.push(dep.package.name);
  }
  graph.set(name, [...new Set(edges)]);
}
// DAG
const visiting = new Set<string>(), done = new Set<string>();
const dfs = (name: string, path: string[]) => {
  if (done.has(name)) return;
  check(!visiting.has(name), `cycle ${[...path, name].join(" -> ")}`);
  if (visiting.has(name)) return;
  visiting.add(name);
  for (const dep of graph.get(name) ?? []) if (graph.has(dep)) dfs(dep, [...path, name]);
  visiting.delete(name); done.add(name);
};
for (const name of graph.keys()) dfs(name, []);
// workspace members
const rootToml = Bun.TOML.parse(readFileSync(ROOT + "/Cargo.toml", "utf8")) as any;
const members: string[] = rootToml.workspace.members;
for (const [name, node] of all) {
  const path = relative(ROOT, dirname(manifestOf(node)));
  check(members.includes(path), `${name}: workspace member ${path}`);
  check(members.filter(m => m === path).length === 1, `${name}: member once`);
}
const dashboardMembers = members.filter(m => m.includes("🎛️dashboard"));
check(JSON.stringify(dashboardMembers) === JSON.stringify([...dashboardMembers].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)))), "dashboard members are byte-sorted");
const encoder = new TextEncoder();
for (let i = 1; i < members.length; i++) if (Buffer.compare(Buffer.from(members[i - 1]!), Buffer.from(members[i]!)) > 0 && members[i]!.includes("🎛️dashboard")) check(false, `member order at ${members[i]}`);
// owners
const owners = JSON.parse(readFileSync(ROOT + "/📋️build-owners.json", "utf8")).owners as {name: string; root: string; manifest: string; packages: unknown[]}[];
const ownerNames = owners.map(o => o.name);
check(new Set(ownerNames).size === ownerNames.length, "owner names unique");
for (const [name, node] of all) {
  const project = name === "semio-framework-repo-dashboard" ? "@semio-tech/repo-dashboard-rs" : "@semio-tech/repo-dashboard-" + crates[name]!.id + "-rs";
  const row = owners.find(o => o.name === project);
  check(!!row, `${project}: owner row`);
  if (!row) continue;
  const root = relative(ROOT, dirname(manifestOf(node)));
  check(row.root === root && row.manifest === root + "/📋️project.json" && existsSync(join(ROOT, row.manifest)), `${project}: owner paths`);
  const pj = JSON.parse(readFileSync(join(ROOT, row.manifest), "utf8"));
  check(pj.name === project, `${project}: project.json name`);
}
const dashOwners = ownerNames.filter(n => n.startsWith("@semio-tech/repo-dashboard-"));
check(JSON.stringify(dashOwners) === JSON.stringify([...dashOwners].sort()), "dashboard owner rows are name-sorted: " + dashOwners.join(","));
// launch routes
for (const file of [ROOT + "/.vscode/launch.json", ROOT + "/.vscode/🧩️launch.seed.jsonc"]) {
  const doc = JSON.parse(readFileSync(file, "utf8")).configurations as any[];
  const orders = new Map<number, number>();
  for (const c of doc) if (typeof c?.presentation?.order === "number") orders.set(c.presentation.order, (orders.get(c.presentation.order) ?? 0) + 1);
  for (const name of Object.keys(crates)) {
    const project = "@semio-tech/repo-dashboard-" + crates[name]!.id + "-rs";
    for (const target of ["build", "check", "test"]) for (const conf of ["prototype", "release"]) {
      const command = `bun repo run ${project}:${target} --configuration=${conf} --parallel=1 -- --lib --offline`;
      const rows = doc.filter(c => c?.command === command);
      check(rows.length === 1, `${file.split("/").pop()}: ${project}:${target}:${conf} x${rows.length}`);
      if (rows[0]) check(orders.get(rows[0].presentation.order) === 1, `${file.split("/").pop()}: order ${rows[0].presentation.order} unique`);
    }
  }
}
// mounted paths and embedded files
let mounts = 0, embeds = 0;
const walk = (dir: string, out: string[]) => { for (const e of readdirSync(dir, {withFileTypes: true})) { if (e.name === "node_modules" || e.name === "🧭️journeys") continue; const p = join(dir, e.name); if (e.isDirectory()) walk(p, out); else if (e.name.endsWith(".rs")) out.push(p); } };
const rs: string[] = []; walk(D, rs);
let longest = 0;
for (const file of rs) {
  const text = readFileSync(file, "utf8");
  for (const m of text.matchAll(/#\[path = "([^"]+)"\]/g)) { mounts++; check(existsSync(resolve(dirname(file), m[1]!)), `${relative(D, file)}: #[path] ${m[1]}`); }
  for (const m of text.matchAll(/include_(?:str|bytes)!\("([^"]+)"\)/g)) { embeds++; check(existsSync(resolve(dirname(file), m[1]!)), `${relative(D, file)}: include ${m[1]}`); }
  longest = Math.max(longest, Buffer.byteLength(relative(ROOT, file)));
}
console.log(`manifests ${all.length} crates, mounts ${mounts}, embeds ${embeds}, rs files ${rs.length}, longest path ${longest} bytes`);
check(longest <= 240, "path length");
console.log(failures === 0 ? "ALL STATIC CHECKS PASSED" : `FAILURES: ${failures}`);
