// 🧪️ Static verification of the repo CLI carve: manifests, workspace membership, owners, launch routes, paths, DAG.
import {existsSync, readFileSync, readdirSync, statSync} from "node:fs";
import {dirname, join, resolve, relative} from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const CLI = join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli");
const nodes: Record<string, string> = {
  "semio-framework-repo-cli-event-stream": "📡️event-stream",
  "semio-framework-repo-cli-command-framework": "🧭️repo-cli/🎮️command-framework",
  "semio-framework-repo-cli-tree-source-and-search": "🧭️repo-cli/🗂️tree-source-and-search",
  "semio-framework-repo-cli-mcp-verb": "🔌️mcp-verb",
  "semio-framework-repo-cli-verbs": "🧭️repo-cli/🚦️verbs",
  "semio-framework-repo-cli": "",
};
let fails = 0;
const check = (ok: boolean, msg: string) => { if (!ok) { fails++; console.log("FAIL", msg); } };
const edges = new Map<string, string[]>();
for (const [name, node] of Object.entries(nodes)) {
  const pkg = join(CLI, node, "📦️packages/🦀️rust");
  const toml = Bun.TOML.parse(readFileSync(join(pkg, "Cargo.toml"), "utf8")) as any;
  check(toml.package.name === name, `${name}: package name`);
  check(resolve(pkg, toml.package.workspace) === ROOT, `${name}: workspace resolves to repo root (${resolve(pkg, toml.package.workspace)})`);
  const lib = resolve(pkg, toml.lib.path);
  check(existsSync(lib) && lib === join(CLI, node, "🦀️.rs"), `${name}: lib path ${lib}`);
  if (name !== "semio-framework-repo-cli") {
    const b = toml.package.metadata.semio["rust-source-boundary"];
    check(Object.keys(b).sort().join() === "maxActiveLines,maxSourceBytes,maxSourceFiles" && b.maxSourceBytes === 262144 && b.maxSourceFiles === 32 && b.maxActiveLines === 4000, `${name}: boundary keys`);
    check(toml.package.metadata.semio.id === name.replace("semio-framework-", ""), `${name}: semio id`);
    const proj = JSON.parse(readFileSync(join(pkg, "📋️project.json"), "utf8"));
    check(proj.name === "@semio-tech/" + name.replace("semio-framework-", "") + "-rs", `${name}: project name ${proj.name}`);
  }
  const deps: string[] = [];
  for (const [dep, spec] of Object.entries(toml.dependencies ?? {}) as [string, any][]) {
    if (spec.path) {
      const target = resolve(pkg, spec.path);
      const manifest = join(target, "Cargo.toml");
      check(existsSync(manifest), `${name}: dep ${dep} path exists`);
      if (existsSync(manifest)) check((Bun.TOML.parse(readFileSync(manifest, "utf8")) as any).package.name === dep, `${name}: dep ${dep} package name`);
      if (dep.startsWith("semio-framework-repo-cli")) deps.push(dep);
    } else check(spec.workspace === true, `${name}: dep ${dep} is path or workspace`);
  }
  edges.set(name, deps);
  const proj2 = join(pkg, "📋️project.json");
  check(existsSync(proj2) || name === "semio-framework-repo-cli", `${name}: project.json`);
}
const seen = new Map<string, number>();
const visit = (n: string, stack: string[]) => {
  if (seen.get(n) === 2) return;
  check(seen.get(n) !== 1, `cycle through ${[...stack, n].join(" -> ")}`);
  if (seen.get(n) === 1) return;
  seen.set(n, 1);
  for (const d of edges.get(n) ?? []) visit(d, [...stack, n]);
  seen.set(n, 2);
};
for (const n of edges.keys()) visit(n, []);
for (const [n, d] of edges) console.log("edge", n, "->", d.join(", ") || "-");

const rootToml = Bun.TOML.parse(readFileSync(join(ROOT, "Cargo.toml"), "utf8")) as any;
const members: string[] = rootToml.workspace.members;
for (const [name, node] of Object.entries(nodes)) {
  const rel = relative(ROOT, join(CLI, node, "📦️packages/🦀️rust"));
  check(members.filter(m => m === rel).length === 1, `${name}: exactly one workspace member ${rel}`);
}
const cliMembers = members.filter(m => m.includes("🦑️repo/🔨️modules/⌨️cli/"));
const enc = new TextEncoder();
const cmp = (a: string, b: string) => { const x = enc.encode(a), y = enc.encode(b); for (let i = 0; i < Math.min(x.length, y.length); i++) if (x[i] !== y[i]) return x[i] - y[i]; return x.length - y.length; };
check(cliMembers.every((m, i) => i === 0 || cmp(cliMembers[i - 1], m) < 0), "cli members byte-sorted");
const idx = members.map((m, i) => [m, i] as const).filter(([m]) => m.includes("🦑️repo/🔨️modules/⌨️cli/")).map(([, i]) => i);
check(idx.every((v, i) => i === 0 || v === idx[i - 1] + 1), "cli members contiguous");
check(cmp(members[idx[idx.length - 1]], members[idx[idx.length - 1] + 1]) < 0, "member after the cli block sorts after it");
check(cmp(members[idx[0] - 1], members[idx[0]]) < 0, "member before the cli block sorts before it");

const owners = JSON.parse(readFileSync(join(ROOT, "📋️build-owners.json"), "utf8")).owners as any[];
const onames = owners.map(o => o.name);
check(new Set(onames).size === onames.length, "owner names unique");
for (const [name, node] of Object.entries(nodes)) {
  if (name === "semio-framework-repo-cli") continue;
  const pn = "@semio-tech/" + name.replace("semio-framework-", "") + "-rs";
  const o = owners.find(x => x.name === pn);
  check(!!o && o.root === relative(ROOT, join(CLI, node, "📦️packages/🦀️rust")) && o.manifest === o.root + "/📋️project.json" && existsSync(join(ROOT, o.manifest)) && o.packages.length === 0, `owner row ${pn}`);
}
const sorted = onames.slice().sort((a, b) => cmp(a, b));
const cliIdx = onames.map((n, i) => [n, i] as const).filter(([n]) => n.startsWith("@semio-tech/repo-cli-")).map(([, i]) => i);
check(cliIdx.every((v, i) => i === 0 || v === cliIdx[i - 1] + 1) && cliIdx.every((v, i) => i === 0 || cmp(onames[cliIdx[i - 1]], onames[v]) < 0), "cli owner rows contiguous and sorted");

const launch = JSON.parse(readFileSync(join(ROOT, ".vscode/launch.json"), "utf8")).configurations as any[];
const seed = (Bun as any).JSONC.parse(readFileSync(join(ROOT, ".vscode/🧩️launch.seed.jsonc"), "utf8")).configurations as any[];
for (const [label, cfgs] of [["launch.json", launch], ["seed", seed]] as const) {
  for (const [name] of Object.entries(nodes)) {
    if (name === "semio-framework-repo-cli") continue;
    const id = name.replace("semio-framework-", "");
    for (const [t, tl, g] of [["build", "🏗️build", "4_build"], ["check", "🔎️check", "4_gate"], ["test", "🧪️test", "4_test"]] as const) {
      for (const [c, cl] of [["prototype", "🛠️prototype"], ["release", "🚀️release"]] as const) {
        const rows = cfgs.filter(x => x.name === `🦀️ ${name} ${tl} ${cl}`);
        check(rows.length === 1 && rows[0].command === `bun repo run @semio-tech/${id}-rs:${t} --configuration=${c} --parallel=1 -- --lib --offline` && rows[0].presentation.group === g && rows[0].type === "node-terminal" && rows[0].cwd === "${workspaceFolder}" && Object.keys(rows[0]).join() === "name,type,request,command,cwd,presentation", `${label}: route ${name} ${t} ${c}`);
      }
    }
  }
  const mine = cfgs.filter(x => typeof x.name === "string" && x.name.startsWith("🦀️ semio-framework-repo-cli-"));
  check(mine.length === 30, `${label}: 30 routes (${mine.length})`);
  for (const g of ["4_build", "4_gate", "4_test"]) { const o = mine.filter(x => x.presentation.group === g).map(x => x.presentation.order); check(new Set(o).size === o.length, `${label}: unique orders in ${g}`); }
}
console.log("launch configs", launch.length, "seed configs", seed.length);

let longest = 0, longestPath = "";
const walk = (d: string) => { for (const e of readdirSync(d, {withFileTypes: true})) { const p = join(d, e.name); const rel = relative(ROOT, p); const len = new TextEncoder().encode(rel).length; if (len > longest) { longest = len; longestPath = rel; } if (e.isDirectory()) walk(p); } };
walk(CLI);
console.log("longest path bytes", longest, longestPath);
check(longest <= 240, "path length");

const rsFiles: string[] = [];
const walkRs = (d: string) => { for (const e of readdirSync(d, {withFileTypes: true})) { const p = join(d, e.name); if (e.isDirectory()) walkRs(p); else if (e.name.endsWith(".rs")) rsFiles.push(p); } };
walkRs(CLI);
let pathAttrs = 0;
for (const f of rsFiles) {
  const t = readFileSync(f, "utf8");
  for (const m of t.matchAll(/^\s*#\[path = "([^"]+)"\]/gm)) { pathAttrs++; check(existsSync(resolve(dirname(f), m[1])), `#[path] ${m[1]} from ${relative(ROOT, f)}`); }
  for (const m of t.matchAll(/include_(?:str|bytes)!\("([^"]+)"\)/g)) check(existsSync(resolve(dirname(f), m[1])), `include ${m[1]}`);
}
console.log("path attrs", pathAttrs);
const owned = new Map<string, string[]>();
for (const f of rsFiles) for (const m of readFileSync(f, "utf8").matchAll(/^\s*#\[path = "([^"]+)"\]/gm)) { const t = resolve(dirname(f), m[1]); owned.set(t, [...(owned.get(t) ?? []), f]); }
for (const [t, o] of owned) check(o.length === 1, `module file mounted once: ${relative(ROOT, t)} by ${o.length}`);
console.log(fails === 0 ? "ALL STATIC CHECKS PASSED" : `${fails} FAILURES`);
