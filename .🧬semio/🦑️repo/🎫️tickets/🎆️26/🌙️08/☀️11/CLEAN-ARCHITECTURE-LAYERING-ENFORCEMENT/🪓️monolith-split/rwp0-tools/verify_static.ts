// ✅️ Static verification of the WP0 skeleton: manifests, dependency DAG and ranks, registries (members, owners, launch routes), path lengths, emoji identities, top-crate block.
// usage: bun verify_static.ts   (reads ../🗑️generated/rwp0-crates.json written by crates.py)
import {existsSync, readFileSync, readdirSync, statSync} from "node:fs";
import {dirname, join, resolve, relative} from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const GEN = import.meta.dir + "/../🗑️generated";
const crates: Record<string, any> = JSON.parse(readFileSync(GEN + "/rwp0-crates.json", "utf8"));
const T = ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu";
const keys = Object.keys(crates);
let failures = 0;
const check = (ok: boolean, what: string) => { if (!ok) { failures++; console.log("FAIL", what); } };
const toml = (path: string) => Bun.TOML.parse(readFileSync(path, "utf8")) as any;
const bytes = (s: string) => Buffer.byteLength(s, "utf8");

const byName = new Map<string, string>();
for (const k of keys) byName.set(crates[k].name, k);
const edges = new Map<string, string[]>();
let manifests = 0, depsChecked = 0;
for (const k of keys) {
  const c = crates[k];
  const dir = ROOT + "/" + c.pkg, file = dir + "/Cargo.toml";
  check(existsSync(file), `${k}: manifest exists`);
  const m = toml(file);
  manifests++;
  check(m.package.name === c.name, `${k}: package.name`);
  check(m.package.metadata.semio.id === c.id && m.package.metadata.semio.role === "framework", `${k}: semio metadata`);
  const boundary = m.package.metadata.semio["rust-source-boundary"];
  check(JSON.stringify(Object.keys(boundary).sort()) === JSON.stringify(["maxActiveLines", "maxSourceBytes", "maxSourceFiles"]) && boundary.maxSourceBytes === 262144 && boundary.maxSourceFiles === 32 && boundary.maxActiveLines === 4000, `${k}: boundary keys`);
  check(resolve(dir, m.package.workspace) === ROOT && existsSync(ROOT + "/Cargo.toml"), `${k}: workspace points at the repository root`);
  check(m.lints?.workspace === true && m.package["rust-version"]?.workspace === true, `${k}: lints and rust-version inherit`);
  check(resolve(dir, m.lib.path) === ROOT + "/" + c.dir + "/🦀️.rs" && existsSync(resolve(dir, m.lib.path)), `${k}: lib path`);
  const tables: any[] = [m.dependencies, ...Object.values(m.target ?? {}).map((t: any) => t.dependencies)];
  const first: string[] = [];
  for (const table of tables) for (const [dep, spec] of Object.entries(table ?? {}) as [string, any][]) {
    if (typeof spec === "object" && spec.path) {
      const target = resolve(dir, spec.path);
      depsChecked++;
      check(existsSync(target + "/Cargo.toml"), `${k}: dependency ${dep} path resolves`);
      if (existsSync(target + "/Cargo.toml")) {
        const dm = toml(target + "/Cargo.toml");
        check(dm.package.name === (spec.package ?? dep), `${k}: dependency ${dep} names package ${dm.package.name}`);
      }
      if (dep.startsWith("semio-framework-os-renderer-wgpu-")) first.push(dep);
    }
  }
  check(first.length === new Set(first).size, `${k}: duplicate first-party dependency`);
  const expected = [...c.deps.map((d: string) => crates[d].name)].sort();
  check(JSON.stringify([...first].sort()) === JSON.stringify(expected), `${k}: first-party dependency set equals the frozen table`);
  edges.set(k, c.deps);
  const lib = ROOT + "/" + c.dir + "/🦀️.rs";
  const text = readFileSync(lib, "utf8");
  check(/^\/\/! \p{Extended_Pictographic}/u.test(text), `${k}: lib root docstring starts with an emoji`);
  if (c.kind === "lib" || c.kind === "shell") check(text.includes('#[path = "🧪️tests/🦀️.rs"]') && existsSync(ROOT + "/" + c.dir + "/🧪️tests/🦀️.rs"), `${k}: test mount and stub`);
  for (const f of [file, dir + "/📋️project.json", lib]) check(bytes(relative(ROOT, f)) <= 240, `${k}: path length ${bytes(relative(ROOT, f))} of ${relative(ROOT, f)}`);
  const pj = JSON.parse(readFileSync(dir + "/📋️project.json", "utf8"));
  check(pj.name === c.project && JSON.stringify(pj.tags) === JSON.stringify(["scope:framework", "language:rust"]), `${k}: project.json`);
}
// DAG: no cycles, strictly decreasing rank
const state = new Map<string, number>();
const visit = (k: string, trail: string[]) => {
  if (state.get(k) === 2) return;
  if (state.get(k) === 1) { check(false, "cycle " + [...trail, k].join(" -> ")); return; }
  state.set(k, 1);
  for (const d of edges.get(k) ?? []) { check(crates[d].rank < crates[k].rank, `${k} -> ${d} violates the rank order`); visit(d, [...trail, k]); }
  state.set(k, 2);
};
for (const k of keys) visit(k, []);
// members
const rootToml = toml(ROOT + "/Cargo.toml");
const members: string[] = rootToml.workspace.members;
const memberSet = new Set(members);
check(memberSet.size === members.length, "root workspace members are unique");
for (const k of keys) check(memberSet.has(crates[k].pkg) && existsSync(ROOT + "/" + crates[k].pkg + "/Cargo.toml"), `${k}: workspace member`);
const prefix = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/";
const block = members.filter(m => m.startsWith(prefix));
check(JSON.stringify(block) === JSON.stringify([...block].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)))), "wgpu member block is byte-sorted");
check(new Set(members.map(m => m.normalize("NFC"))).size === members.length, "members unique after NFC");
// owners
const owners = JSON.parse(readFileSync(ROOT + "/📋️build-owners.json", "utf8")).owners as any[];
const ownerNames = owners.map(o => o.name);
check(new Set(ownerNames).size === ownerNames.length, "owner names unique");
for (const k of keys) {
  const row = owners.find(o => o.name === crates[k].project);
  check(!!row && row.root === crates[k].pkg && row.manifest === crates[k].pkg + "/📋️project.json" && Array.isArray(row.packages) && row.packages.length === 0, `${k}: owner row`);
  if (row) check(existsSync(ROOT + "/" + row.manifest), `${k}: owner manifest exists`);
}
const newOwners = ownerNames.filter(n => n.startsWith("@semio-tech/os-renderer-wgpu-"));
check(newOwners.length === keys.length, `owner rows ${newOwners.length} equal crates ${keys.length}`);
check(JSON.stringify(newOwners) === JSON.stringify([...newOwners].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)))), "new owner rows are byte-sorted");
// launch
const files = [ROOT + "/.vscode/launch.json", ROOT + "/.vscode/🧩️launch.seed.jsonc"];
const combos = [["🏗️build 🛠️prototype", "build", "prototype", "4_build"], ["🏗️build 🚀️release", "build", "release", "4_build"], ["🔎️check 🛠️prototype", "check", "prototype", "4_gate"], ["🔎️check 🚀️release", "check", "release", "4_gate"], ["🧪️test 🛠️prototype", "test", "prototype", "4_test"], ["🧪️test 🚀️release", "test", "release", "4_test"]];
const ordersByFile: number[][] = [];
for (const f of files) {
  const cfgs = JSON.parse(readFileSync(f, "utf8")).configurations as any[];
  const orders = cfgs.map(c => c.presentation?.order).filter((o: any) => typeof o === "number");
  const seen = new Map<string, number>();
  for (const c of cfgs) seen.set(c.name, (seen.get(c.name) ?? 0) + 1);
  for (const k of keys) for (const [label, target, conf, group] of combos) {
    const name = `🦀️ ${crates[k].name} ${label}`;
    const rows = cfgs.filter(c => c.name === name);
    check(rows.length === 1, `${f.split("/").pop()}: ${name} x${rows.length}`);
    const r = rows[0];
    if (r) {
      check(r.command === `bun repo run ${crates[k].project}:${target} --configuration=${conf} --parallel=1 -- --lib --offline` && r.type === "node-terminal" && r.request === "launch" && r.cwd === "${workspaceFolder}" && r.presentation.group === group, `${name}: shape`);
      check(JSON.stringify(Object.keys(r).filter(key => key !== "env")) === JSON.stringify(["name", "type", "request", "command", "cwd", "presentation"]) && (!("env" in r) || Object.keys(r).at(-1) === "env"), `${name}: key order (an optional trailing env is added by the launch normalizer of the peers)`);
    }
  }
  const mine = cfgs.filter(c => (c.name ?? "").startsWith("🦀️ semio-framework-os-renderer-wgpu-")).map(c => c.presentation.order);
  check(mine.length === keys.length * 6, `${f.split("/").pop()}: ${mine.length} routes`);
  check(new Set(mine).size === mine.length, `${f.split("/").pop()}: new order keys unique`);
  const others = new Map<number, number>();
  for (const c of cfgs) if (!(c.name ?? "").startsWith("🦀️ semio-framework-os-renderer-wgpu-") && typeof c.presentation?.order === "number") others.set(c.presentation.order, 1);
  check(mine.every((o: number) => !others.has(o)), `${f.split("/").pop()}: new order keys unused by other routes`);
  ordersByFile.push(mine);
}
check(JSON.stringify(ordersByFile[0]) === JSON.stringify(ordersByFile[1]), "both launch files carry the same order keys in the same order");
// emoji identities among the siblings of the target folder
const first = (name: string) => [...new Intl.Segmenter("en", {granularity: "grapheme"}).segment(name)][0]!.segment.replaceAll("️", "");
const siblings = readdirSync(T).filter(n => n !== "node_modules");
const counts = new Map<string, string[]>();
for (const n of siblings) counts.set(first(n), [...(counts.get(first(n)) ?? []), n]);
for (const k of keys) {
  const emoji = crates[k].folder.slice(0, crates[k].folder.indexOf("️") + 1);
  check(/^\p{RGI_Emoji}$/v.test(emoji) || /^\p{Extended_Pictographic}️$/u.test(emoji), `${k}: folder emoji is RGI or Extended_Pictographic+FE0F`);
  check(counts.get(first(crates[k].folder))!.length === 1, `${k}: folder emoji unique among siblings: ${counts.get(first(crates[k].folder))}`);
  check(bytes(crates[k].folder) <= 28, `${k}: folder name ${bytes(crates[k].folder)} bytes`);
}
const dup = [...counts.entries()].filter(([, v]) => v.length > 1).map(([e, v]) => e + ":" + v.join("|"));
console.log("pre-existing sibling emoji duplicates (not ours):", dup.filter(d => !keys.some(k => d.includes(crates[k].folder))).join(" ; "));
// top crate block
const top = toml(T + "/📦️packages/🦀️rust/Cargo.toml");
const topTables: Record<string, any> = {plain: top.dependencies, notwasi: top.target['cfg(not(target_os = "wasi"))'].dependencies, native: top.target['cfg(not(target_arch = "wasm32"))'].dependencies, browser: top.target['cfg(all(target_arch = "wasm32", target_os = "unknown"))'].dependencies};
for (const k of keys) {
  const c = crates[k];
  if (c.kind !== "lib" && c.kind !== "shell") { check(!Object.values(topTables).some(t => t[c.name]), `${k}: companion must not be a top-crate dependency`); continue; }
  const table = c.cfg === "native" ? "native" : c.cfg === "wasm" ? "browser" : c.cfg === "all" ? "plain" : "notwasi";
  const spec = topTables[table][c.name];
  check(!!spec && resolve(T + "/📦️packages/🦀️rust", spec.path) === ROOT + "/" + c.pkg, `${k}: top-crate dependency in the ${table} table`);
  for (const [t, tbl] of Object.entries(topTables)) if (t !== table) check(!tbl[c.name], `${k}: also listed in the ${t} table`);
}
check(JSON.stringify(Object.keys(top.package.metadata.semio)) === JSON.stringify(["role", "id"]) && top.package.name === "semio-framework-os-renderer-wgpu", "top crate identity unchanged");
console.log(`manifests ${manifests}, first-party/external path dependencies resolved ${depsChecked}, crates ${keys.length}, failures ${failures}`);
console.log(failures === 0 ? "ALL STATIC CHECKS PASSED" : "STATIC CHECKS FAILED");
process.exit(failures === 0 ? 0 : 1);
