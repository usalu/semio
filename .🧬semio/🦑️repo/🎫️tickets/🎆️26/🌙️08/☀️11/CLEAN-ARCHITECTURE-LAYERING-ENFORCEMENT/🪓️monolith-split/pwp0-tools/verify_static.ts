// ✅️ Static verification of the plugin WP0 skeleton: manifests, dependency DAG and ranks, registries (members, owners, launch routes), path lengths, emoji identities, facade block.
// usage: bun verify_static.ts   (reads ../🗑️generated/pwp0-crates.json written by crates.py)
import {existsSync, readFileSync, readdirSync} from "node:fs";
import {resolve, relative} from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const GEN = import.meta.dir + "/../🗑️generated";
const crates: Record<string, any> = JSON.parse(readFileSync(GEN + "/pwp0-crates.json", "utf8"));
const PLUG = ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const keys = Object.keys(crates);
let failures = 0;
const check = (ok: boolean, what: string) => { if (!ok) { failures++; console.log("FAIL", what); } };
const toml = (path: string) => Bun.TOML.parse(readFileSync(path, "utf8")) as any;
const bytes = (s: string) => Buffer.byteLength(s, "utf8");
const PREFIX = "semio-framework-plugin-";
const byName = new Set(keys.map(k => crates[k].name));
const edges = new Map<string, string[]>();
let manifests = 0, depsChecked = 0;
for (const k of keys) {
  const c = crates[k];
  const dir = ROOT + "/" + c.pkg, file = dir + "/Cargo.toml";
  check(existsSync(file), `${k}: manifest exists`);
  const m = toml(file);
  manifests++;
  check(m.package.name === c.name && c.name === PREFIX + k, `${k}: package.name`);
  check(m.package.metadata.semio.id === c.id && m.package.metadata.semio.role === "framework", `${k}: semio metadata`);
  const boundary = m.package.metadata.semio["rust-source-boundary"];
  check(JSON.stringify(Object.keys(boundary).sort()) === JSON.stringify(["maxActiveLines", "maxSourceBytes", "maxSourceFiles"]) && boundary.maxSourceBytes === 262144 && boundary.maxSourceFiles === 32 && boundary.maxActiveLines === 4000, `${k}: boundary keys`);
  check(resolve(dir, m.package.workspace) === ROOT && existsSync(ROOT + "/Cargo.toml"), `${k}: workspace points at the repository root`);
  check(m.lints?.workspace === true && m.package["rust-version"]?.workspace === true, `${k}: lints and rust-version inherit`);
  const libFile = c.class === "A" ? ROOT + "/" + c.dir + "/🦀️.rs" : dir + "/🦀️.rs";
  check(resolve(dir, m.lib.path) === libFile && existsSync(libFile), `${k}: lib path (class ${c.class})`);
  if (c.class === "B") check(existsSync(ROOT + "/" + c.dir + "/🦀️.rs"), `${k}: class B root keeps the monolith module file`);
  const tables: any[] = [m.dependencies, ...Object.values(m.target ?? {}).map((t: any) => t.dependencies)];
  const first: string[] = [];
  for (const table of tables) for (const [dep, spec] of Object.entries(table ?? {}) as [string, any][]) {
    if (typeof spec === "object" && spec.path) {
      const target = resolve(dir, spec.path);
      depsChecked++;
      check(existsSync(target + "/Cargo.toml"), `${k}: dependency ${dep} path resolves`);
      if (existsSync(target + "/Cargo.toml")) check(toml(target + "/Cargo.toml").package.name === (spec.package ?? dep), `${k}: dependency ${dep} names its package`);
      if (byName.has(dep)) first.push(dep);
    }
  }
  check(first.length === new Set(first).size, `${k}: duplicate first-party dependency`);
  const expected = [...c.deps.map((d: string) => crates[d].name)].sort();
  check(JSON.stringify([...first].sort()) === JSON.stringify(expected), `${k}: first-party dependency set equals the frozen table`);
  edges.set(k, c.deps);
  const text = readFileSync(libFile, "utf8");
  check(/^\/\/! \p{Extended_Pictographic}/u.test(text), `${k}: lib root docstring starts with an emoji`);
  for (const f of [file, dir + "/📋️project.json", libFile]) check(bytes(relative(ROOT, f)) <= 240, `${k}: path length ${bytes(relative(ROOT, f))} of ${relative(ROOT, f)}`);
  const pj = JSON.parse(readFileSync(dir + "/📋️project.json", "utf8"));
  check(pj.name === c.project && JSON.stringify(pj.tags) === JSON.stringify(["scope:framework", "language:rust"]), `${k}: project.json`);
  for (const f of c.features ?? []) check(m.features && f in m.features, `${k}: feature ${f} declared`);
}
const state = new Map<string, number>();
const visit = (k: string, trail: string[]) => {
  if (state.get(k) === 2) return;
  if (state.get(k) === 1) { check(false, "cycle " + [...trail, k].join(" -> ")); return; }
  state.set(k, 1);
  for (const d of edges.get(k) ?? []) { check(crates[d].rank < crates[k].rank, `${k} -> ${d} violates the rank order`); visit(d, [...trail, k]); }
  state.set(k, 2);
};
for (const k of keys) visit(k, []);
const members: string[] = toml(ROOT + "/Cargo.toml").workspace.members;
const memberSet = new Set(members);
check(memberSet.size === members.length, "root workspace members are unique");
check(JSON.stringify(members) === JSON.stringify([...members].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)))), "root workspace members are byte-sorted");
for (const k of keys) check(memberSet.has(crates[k].pkg), `${k}: workspace member`);
const owners = JSON.parse(readFileSync(ROOT + "/📋️build-owners.json", "utf8")).owners as any[];
const ownerNames = owners.map(o => o.name);
check(new Set(ownerNames).size === ownerNames.length, "owner names unique");
check(JSON.stringify(ownerNames) === JSON.stringify([...ownerNames].sort()), "owner rows are sorted by name");
for (const k of keys) {
  const row = owners.find(o => o.name === crates[k].project);
  check(!!row && row.root === crates[k].pkg && row.manifest === crates[k].pkg + "/📋️project.json" && Array.isArray(row.packages) && row.packages.length === 0, `${k}: owner row`);
  if (row) check(existsSync(ROOT + "/" + row.manifest), `${k}: owner manifest exists`);
}
const files = [ROOT + "/.vscode/launch.json", ROOT + "/.vscode/🧩️launch.seed.jsonc"];
const combos = [["🏗️build 🛠️prototype", "build", "prototype", "4_build"], ["🏗️build 🚀️release", "build", "release", "4_build"], ["🔎️check 🛠️prototype", "check", "prototype", "4_gate"], ["🔎️check 🚀️release", "check", "release", "4_gate"], ["🧪️test 🛠️prototype", "test", "prototype", "4_test"], ["🧪️test 🚀️release", "test", "release", "4_test"]];
const ordersByFile: number[][] = [];
const mineNames = new Set(keys.map(k => "🦀️ " + crates[k].name + " "));
const isMine = (n: string) => { for (const p of mineNames) if (n.startsWith(p)) return true; return false; };
for (const f of files) {
  const text = readFileSync(f, "utf8");
  const cfgs = (f.endsWith(".json") ? JSON.parse(text) : Bun.JSONC.parse(text)).configurations as any[];
  for (const k of keys) for (const [label, target, conf, group] of combos) {
    const name = `🦀️ ${crates[k].name} ${label}`;
    const rows = cfgs.filter(c => c.name === name);
    check(rows.length === 1, `${f.split("/").pop()}: ${name} x${rows.length}`);
    const r = rows[0];
    if (r) {
      check(r.command === `bun repo run ${crates[k].project}:${target} --configuration=${conf} --parallel=1 -- --lib --offline` && r.type === "node-terminal" && r.request === "launch" && r.cwd === "${workspaceFolder}" && r.presentation.group === group, `${name}: shape`);
      check(JSON.stringify(Object.keys(r).filter(key => key !== "env")) === JSON.stringify(["name", "type", "request", "command", "cwd", "presentation"]), `${name}: key order`);
    }
  }
  const mine = cfgs.filter(c => isMine(c.name ?? "")).map(c => c.presentation.order);
  check(mine.length === keys.length * 6, `${f.split("/").pop()}: ${mine.length} routes`);
  check(new Set(mine).size === mine.length, `${f.split("/").pop()}: new order keys unique`);
  const others = new Set<number>();
  for (const c of cfgs) if (!isMine(c.name ?? "") && typeof c.presentation?.order === "number") others.add(c.presentation.order);
  check(mine.every((o: number) => !others.has(o)), `${f.split("/").pop()}: new order keys unused by other routes`);
  ordersByFile.push(mine);
}
check(JSON.stringify(ordersByFile[0]) === JSON.stringify(ordersByFile[1]), "both launch files carry the same order keys in the same order");
// emoji identities of every directory this WP created, among its siblings
const first = (name: string) => [...new Intl.Segmenter("en", {granularity: "grapheme"}).segment(name)][0]!.segment.replaceAll("️", "");
const created = new Set<string>();
const tracked = (d: string) => Bun.spawnSync(["git", "-c", "core.quotepath=false", "ls-files", "--", d], {cwd: ROOT}).stdout.toString().trim().length > 0;
for (const k of keys) {
  const parts = crates[k].root.split("/"); let cur = PLUG;
  for (const p of parts) { cur += "/" + p; if (!tracked(cur)) created.add(cur); }
}
created.add(PLUG + "/🪝️mounts");
for (const d of readdirSync(PLUG + "/🪝️mounts")) created.add(PLUG + "/🪝️mounts/" + d);
for (const d of created) {
  const parent = d.slice(0, d.lastIndexOf("/")), name = d.slice(d.lastIndexOf("/") + 1);
  if (!existsSync(parent)) continue;
  const sibs = readdirSync(parent).filter(n => n !== "node_modules" && first(n) === first(name));
  const emoji = name.slice(0, name.indexOf("️") + 1);
  check(/^\p{RGI_Emoji}$/v.test(emoji) || /^\p{Extended_Pictographic}\ufe0f$/u.test(emoji) || /^\p{RGI_Emoji}\ufe0f$/v.test(emoji), `${name}: folder emoji is RGI (bare or with U+FE0F)`);
  check(sibs.length === 1, `${d.slice(PLUG.length + 1)}: emoji unique among siblings: ${sibs}`);
}
// facade
const facade = toml(PLUG + "/📦️packages/🦀️rust/Cargo.toml");
const wit = facade.target?.['cfg(all(target_arch = "wasm32", target_env = "p2"))']?.dependencies ?? {};
for (const k of keys) {
  const c = crates[k];
  if (c.kind !== "lower" && c.kind !== "instance") { check(!facade.dependencies[c.name], `${k}: companion must not be a facade dependency`); continue; }
  const spec = k === "reactor-wit-bridge" ? wit[c.name] : facade.dependencies[c.name];
  check(!!spec && resolve(PLUG + "/📦️packages/🦀️rust", spec.path) === ROOT + "/" + c.pkg, `${k}: facade dependency`);
  check(!!spec?.optional === (k === "testing-laws" || k === "testing-acceptance"), `${k}: facade optional flag`);
  for (const f of c.features ?? []) if (f !== "canvas-runtime-assets") check((facade.features[f] ?? []).includes(c.name + "/" + f), `${k}: facade forwards ${f}`);
}
const b = facade.package.metadata.semio["rust-source-boundary"];
check(b.maxSourceBytes === 262144 && b.maxSourceFiles === 32 && b.maxActiveLines === 4000, "facade boundary triple at the ceiling");
check(!("ui_wgpu" in facade.dependencies), "facade has no ui_wgpu dependency");
check(facade.package.name === "semio-framework-plugin" && JSON.stringify(Object.keys(facade.package.metadata.semio)) === JSON.stringify(["role", "id", "rust-source-boundary"]), "facade identity unchanged");
console.log(`manifests ${manifests}, path dependencies resolved ${depsChecked}, crates ${keys.length}, failures ${failures}`);
console.log(failures === 0 ? "ALL STATIC CHECKS PASSED" : "STATIC CHECKS FAILED");
process.exit(failures === 0 ? 0 : 1);
