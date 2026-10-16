// 🔬️ Static checks of the carve wiring: TOML/JSON parse, workspace members, owners, launch routes, dependency DAG and path lengths.
import {readFileSync, existsSync} from "node:fs";
import {join, resolve, relative, dirname} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const clusters: Record<string, any> = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
let fail = 0;
const bad = (m: string) => { fail++; console.log("FAIL", m); };
const read = (p: string) => readFileSync(join(ROOT, p), "utf8");
const pkgDir = (node: string) => node + "/📦️packages/🦀️rust";
const fwToml = Bun.TOML.parse(read("🧰️framework/Cargo.toml")) as any;
const members: string[] = fwToml.workspace.members;
const owners = JSON.parse(read("📋️build-owners.json")).owners as any[];
const launch = Bun.JSONC.parse(read(".vscode/launch.json")) as any;
const seed = Bun.JSONC.parse(read(".vscode/🧩️launch.seed.jsonc")) as any;
const names = new Map<string, string>();
const edges = new Map<string, string[]>();
for (const [id, c] of Object.entries(clusters)) {
  const dir = pkgDir(c.node);
  for (const f of ["Cargo.toml", "📋️project.json"]) if (!existsSync(join(ROOT, dir, f))) bad(`${id} missing ${f}`);
  if (Buffer.byteLength(dir + "/📋️project.json") > 240) bad(`${id} path too long`);
  const toml = Bun.TOML.parse(read(dir + "/Cargo.toml")) as any;
  if (toml.package.name !== c.pkg) bad(`${id} name ${toml.package.name}`);
  const ws = resolve(ROOT, dir, toml.package.workspace);
  if (ws !== join(ROOT, "🧰️framework")) bad(`${id} workspace ${ws}`);
  const b = toml.package.metadata.semio["rust-source-boundary"];
  if (b.maxSourceBytes !== 262144 || b.maxSourceFiles !== 32 || b.maxActiveLines !== 4000 || Object.keys(b).length !== 3) bad(`${id} boundary keys`);
  const lib = resolve(ROOT, dir, toml.lib.path);
  if (lib !== join(ROOT, c.root)) bad(`${id} lib path ${lib}`);
  if (!existsSync(lib)) bad(`${id} lib missing`);
  const rel = relative(join(ROOT, "🧰️framework"), join(ROOT, dir));
  if (!members.includes(rel)) bad(`${id} not a workspace member ${rel}`);
  const deps: string[] = [];
  for (const table of [toml.dependencies, toml["dev-dependencies"]]) for (const [k, v] of Object.entries(table ?? {}) as any) {
    if (v && typeof v === "object" && typeof v.path === "string") {
      const target = resolve(ROOT, dir, v.path, "Cargo.toml");
      if (!existsSync(target)) bad(`${id} dep ${k} path missing ${v.path}`);
      else { const t = Bun.TOML.parse(readFileSync(target, "utf8")) as any; if ((v.package ?? k) !== t.package.name) bad(`${id} dep ${k} resolves to ${t.package.name}`); }
      for (const [oid, oc] of Object.entries(clusters)) if (resolve(ROOT, dir, v.path) === join(ROOT, pkgDir(oc.node))) deps.push(oid);
    }
  }
  edges.set(id, deps);
  const row = owners.filter(o => o.name === `@semio-tech/${id}-rs`);
  if (row.length !== 1) bad(`${id} owner rows ${row.length}`);
  else { if (row[0].root !== dir || row[0].manifest !== dir + "/📋️project.json") bad(`${id} owner row fields`); }
  const pj = JSON.parse(read(dir + "/📋️project.json"));
  if (pj.name !== `@semio-tech/${id}-rs`) bad(`${id} project name`);
  for (const [label, file] of [["launch", launch], ["seed", seed]] as const) {
    for (const verb of ["🏗️build", "🔎️check", "🧪️test"]) for (const mode of ["🛠️prototype", "🚀️release"]) {
      const name = `🦀️ ${c.pkg} ${verb} ${mode}`;
      const hit = file.configurations.filter((x: any) => x.name === name);
      if (hit.length !== 1) { bad(`${label} ${name} x${hit.length}`); continue; }
      const target = verb.includes("build") ? "build" : verb.includes("check") ? "check" : "test";
      const conf = mode.includes("prototype") ? "prototype" : "release";
      const want = `bun repo run @semio-tech/${id}-rs:${target} --configuration=${conf} --parallel=1 -- --lib --offline`;
      if (hit[0].command !== want) bad(`${label} ${name} command`);
    }
  }
}
// order keys unique per group in both files
for (const [label, file] of [["launch", launch], ["seed", seed]] as const) {
  const seen = new Map<string, string>();
  for (const x of file.configurations) { const g = x.presentation?.group, o = x.presentation?.order; if (g === undefined || o === undefined) continue; const k = `${g}|${o}`; if (seen.has(k) && x.name.includes("semio-framework-") && Object.values(clusters).some((c: any) => x.name.includes(c.pkg + " "))) bad(`${label} duplicate order ${k}: ${x.name} / ${seen.get(k)}`); seen.set(k, x.name); }
}
// DAG
const state = new Map<string, number>();
const dfs = (n: string, path: string[]) => { if (state.get(n) === 1) { bad(`cycle ${[...path, n].join(" > ")}`); return; } if (state.get(n) === 2) return; state.set(n, 1); for (const m of edges.get(n) ?? []) dfs(m, [...path, n]); state.set(n, 2); };
for (const id of edges.keys()) dfs(id, []);
// facade
const facade = Bun.TOML.parse(read("🧰️framework/📦️packages/🦀️rust/Cargo.toml")) as any;
for (const [id, c] of Object.entries(clusters)) if (!facade.dependencies[c.pkg]) bad(`facade lacks dependency ${c.pkg}`);
// Cargo.toml sorted members check not required (existing list is not sorted); only uniqueness
if (new Set(members).size !== members.length) bad("duplicate workspace members");
console.log(fail === 0 ? "STATIC CHECKS PASSED" : `STATIC CHECKS FAILED ${fail}`);
