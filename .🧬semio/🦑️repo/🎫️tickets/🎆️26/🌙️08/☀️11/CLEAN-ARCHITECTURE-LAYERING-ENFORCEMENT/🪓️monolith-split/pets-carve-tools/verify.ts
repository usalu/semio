// 🔍️ Static checks of the pets carve: TOML parse, dependency paths and DAG, lib roots, #[path] ownership, path lengths, JSON parse.
import {readFileSync, existsSync, readdirSync, statSync} from "node:fs";
import {join, resolve, dirname, relative} from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const P = join(ROOT, "🧰️framework/🛍️products/🐾️pets");
const pkgDirs: Record<string, string> = {
  "semio-framework-pets": join(P, "📦️packages/🦀️rust"),
  "semio-framework-pets-contract": join(P, "🔨️modules/📜️contract/📦️packages/🦀️rust"),
  "semio-framework-pets-kinematics": join(P, "🔨️modules/⚙️kinematics/📦️packages/🦀️rust"),
  "semio-framework-pets-clearance": join(P, "🔨️modules/🚧️clearance/📦️packages/🦀️rust"),
  "semio-framework-pets-traversal": join(P, "🔨️modules/🧭️traversal/📦️packages/🦀️rust"),
  "semio-framework-pets-interaction": join(P, "🔨️modules/🖱️interaction/📦️packages/🦀️rust"),
  "semio-framework-pets-character": join(P, "🔨️modules/🎭️character/📦️packages/🦀️rust"),
  "semio-framework-pets-staging": join(P, "🔨️modules/🎬️staging/📦️packages/🦀️rust"),
};
let failures = 0;
const fail = (m: string) => { failures++; console.log("FAIL", m); };
const edges = new Map<string, string[]>();
const roots = new Map<string, string>();
const ws = Bun.TOML.parse(readFileSync(join(ROOT, "Cargo.toml"), "utf8")) as any;
const members: string[] = ws.workspace.members;
for (const [name, dir] of Object.entries(pkgDirs)) {
  const m = Bun.TOML.parse(readFileSync(join(dir, "Cargo.toml"), "utf8")) as any;
  if (m.package.name !== name) fail(`${name}: package.name ${m.package.name}`);
  const lib = resolve(dir, m.lib?.path ?? "src/lib.rs");
  if (!existsSync(lib)) fail(`${name}: lib root ${lib} missing`); else roots.set(name, lib);
  const rel = relative(ROOT, dir);
  if (!members.includes(rel)) fail(`${name}: not a workspace member`);
  const up = resolve(dir, m.package.workspace);
  if (up !== ROOT) fail(`${name}: package.workspace resolves to ${up}`);
  const b = m.package.metadata?.semio?.["rust-source-boundary"];
  if (!b || Object.keys(b).sort().join() !== "maxActiveLines,maxSourceBytes,maxSourceFiles" || b.maxActiveLines !== 4000 || b.maxSourceBytes !== 262144 || b.maxSourceFiles !== 32) fail(`${name}: boundary keys`);
  const out: string[] = [];
  for (const table of ["dependencies", "dev-dependencies"]) for (const [dep, spec] of Object.entries(m[table] ?? {}) as [string, any][]) {
    if (spec.path) {
      const target = resolve(dir, spec.path);
      const tm = join(target, "Cargo.toml");
      if (!existsSync(tm)) { fail(`${name}: ${table} ${dep} path ${spec.path} has no manifest`); continue; }
      const tn = (Bun.TOML.parse(readFileSync(tm, "utf8")) as any).package.name;
      if (tn !== dep) fail(`${name}: ${dep} resolves to package ${tn}`);
      if (tn.startsWith("semio-framework-pets")) out.push(tn);
    } else if (spec.workspace !== true) fail(`${name}: ${dep} is neither path nor workspace`);
  }
  edges.set(name, [...new Set(out)]);
  console.log("ok", name, "->", [...new Set(out)].map(x => x.replace("semio-framework-", "")).join(", ") || "-");
}
const state = new Map<string, number>();
const visit = (n: string, stack: string[]) => {
  if (state.get(n) === 2) return;
  if (state.get(n) === 1) { fail("cycle " + [...stack, n].join(" -> ")); return; }
  state.set(n, 1);
  for (const d of edges.get(n) ?? []) visit(d, [...stack, n]);
  state.set(n, 2);
};
for (const n of edges.keys()) visit(n, []);
console.log("DAG check done");

const owned = new Map<string, string[]>();
const mount = /#\[path = "([^"]+)"\]\s*(?:pub\s+)?mod\s+\w+;/g;
const moduleFiles: string[] = [join(P, "🧬️schema/🦀️.rs"), ...readdirSync(join(P, "🔨️modules")).map(d => join(P, "🔨️modules", d, "🦀️.rs")).filter(existsSync)];
const entryCrates = new Set(roots.keys());
for (const [name, root] of roots) {
  const text = readFileSync(root, "utf8");
  const own = [root];
  for (const m of text.matchAll(/#\[(?:cfg\([^\]]*\)\]\s*#\[)?path = "([^"]+)"\]\s*(?:pub\s+)?mod\s+\w+;/g)) {
    const target = resolve(dirname(root), m[1]!);
    if (!existsSync(target)) fail(`${name}: #[path] ${m[1]} missing`);
    own.push(target);
  }
  for (const f of own) owned.set(f, [...(owned.get(f) ?? []), name]);
}
for (const f of moduleFiles) {
  const o = owned.get(f) ?? [];
  if (o.length !== 1) fail(`${relative(P, f)} owned by ${o.length}: ${o.join(",")}`);
}
console.log("module files owned exactly once:", moduleFiles.length, "files checked");

let maxLen = 0, maxPath = "";
const walk = (d: string) => { for (const e of readdirSync(d, {withFileTypes: true})) { if (e.name === "node_modules") continue; const p = join(d, e.name); const l = Buffer.byteLength(relative(ROOT, p)); if (l > maxLen) { maxLen = l; maxPath = p; } if (e.isDirectory()) walk(p); } };
walk(P);
console.log("longest path in pets tree:", maxLen, "bytes");
if (maxLen > 240) fail("path > 240 bytes " + maxPath);

for (const f of [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc", "📋️build-owners.json", ...Object.values(pkgDirs).map(d => relative(ROOT, join(d, "📋️project.json")))]) {
  try { JSON.parse(readFileSync(join(ROOT, f), "utf8")); console.log("json ok", f); } catch (e) { fail("json " + f + " " + e); }
}
const launch = JSON.parse(readFileSync(join(ROOT, ".vscode/launch.json"), "utf8")).configurations as any[];
const seed = JSON.parse(readFileSync(join(ROOT, ".vscode/🧩️launch.seed.jsonc"), "utf8")).configurations as any[];
const owners = (JSON.parse(readFileSync(join(ROOT, "📋️build-owners.json"), "utf8")).owners as any[]);
for (const [name, dir] of Object.entries(pkgDirs)) {
  if (name === "semio-framework-pets") continue;
  const id = name.replace("semio-framework-", "");
  const project = `@semio-tech/${id}-rs`;
  const pj = JSON.parse(readFileSync(join(dir, "📋️project.json"), "utf8"));
  if (pj.name !== project) fail(`${name}: project.json name ${pj.name}`);
  const row = owners.filter(o => o.name === project);
  if (row.length !== 1 || row[0].root !== relative(ROOT, dir) || row[0].manifest !== relative(ROOT, join(dir, "📋️project.json"))) fail(`${name}: owner row`);
  for (const [file, rows] of [["launch.json", launch], ["seed", seed]] as [string, any[]][]) {
    for (const kind of ["build", "check", "test"]) for (const conf of ["prototype", "release"]) {
      const cmd = `bun repo run ${project}:${kind} --configuration=${conf} --parallel=1 -- --lib --offline`;
      const hits = rows.filter(r => r.command === cmd);
      if (hits.length !== 1) fail(`${file}: ${cmd} x${hits.length}`);
    }
  }
}
const orders = new Map<number, string[]>();
for (const [file, rows] of [["launch.json", launch], ["seed", seed]] as [string, any[]][]) {
  const seen = new Map<number, number>();
  for (const r of rows) { const o = r.presentation?.order; if (typeof o === "number") seen.set(o, (seen.get(o) ?? 0) + 1); }
  for (const r of rows) if (typeof r.command === "string" && r.command.includes("@semio-tech/pets-") && r.command.includes("--parallel=1 -- --lib --offline")) {
    const o = r.presentation.order; if ((seen.get(o) ?? 0) !== 1) fail(`${file}: order ${o} reused ${seen.get(o)}x`);
  }
}
console.log(failures === 0 ? "ALL STATIC CHECKS PASSED" : `${failures} FAILURES`);
