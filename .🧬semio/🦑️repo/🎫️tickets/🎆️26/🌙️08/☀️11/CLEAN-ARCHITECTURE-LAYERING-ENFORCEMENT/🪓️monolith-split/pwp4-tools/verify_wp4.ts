// ✅️ Static verification of the WP4 crates: manifests, first-party dependency DAG against Appendix H ranks, path lengths, project files.
// usage: bun verify_wp4.ts   (reads ../🗑️generated/pwp0-crates.json)
import {existsSync, readFileSync, readdirSync, statSync} from "node:fs";
import {resolve, relative} from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const GEN = import.meta.dir + "/../🗑️generated";
const crates: Record<string, any> = JSON.parse(readFileSync(GEN + "/pwp0-crates.json", "utf8"));
const PLUG = ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const mine = Object.keys(crates).filter(k => crates[k].wp === 4 && crates[k].kind === "lower").concat(["tests-07"]);
let failures = 0;
const check = (ok: boolean, what: string) => { if (!ok) { failures++; console.log("FAIL", what); } };
const toml = (path: string) => Bun.TOML.parse(readFileSync(path, "utf8")) as any;
const bytes = (s: string) => Buffer.byteLength(s, "utf8");
const byName = new Map(Object.keys(crates).map(k => [crates[k].name, k] as [string, string]));
let deltas: string[] = [];
const edges = new Map<string, string[]>();
for (const k of mine) {
  const c = crates[k], dir = ROOT + "/" + c.pkg, file = dir + "/Cargo.toml";
  check(existsSync(file), `${k}: manifest exists`);
  const m = toml(file);
  check(m.package.name === c.name, `${k}: package.name`);
  check(m.package.metadata.semio.id === c.id && m.package.metadata.semio.role === "framework", `${k}: semio metadata`);
  const b = m.package.metadata.semio["rust-source-boundary"];
  check(b.maxSourceBytes === 262144 && b.maxSourceFiles === 32 && b.maxActiveLines === 4000, `${k}: boundary triple`);
  check(resolve(dir, m.package.workspace) === ROOT, `${k}: workspace root`);
  check(m.lints?.workspace === true && m.package["rust-version"]?.workspace === true, `${k}: lints and rust-version inherit`);
  const libFile = resolve(dir, m.lib.path);
  check(existsSync(libFile), `${k}: lib path ${relative(ROOT, libFile)} exists`);
  const first: string[] = [];
  const tables: any[] = [m.dependencies, ...Object.values(m.target ?? {}).map((t: any) => t.dependencies)];
  for (const table of tables) for (const [dep, spec] of Object.entries(table ?? {}) as [string, any][]) {
    if (typeof spec === "object" && spec.path) {
      const target = resolve(dir, spec.path);
      check(existsSync(target + "/Cargo.toml"), `${k}: dependency ${dep} resolves`);
      if (byName.has(dep)) first.push(byName.get(dep)!);
    }
  }
  edges.set(k, first);
  for (const d of first) check(crates[d].rank < c.rank, `${k} -> ${d} violates the rank order (${c.rank} -> ${crates[d].rank})`);
  const frozen = new Set<string>(c.deps);
  const added = first.filter(d => !frozen.has(d)), removed = [...frozen].filter(d => !first.includes(d));
  if (added.length || removed.length) deltas.push(`${k}: +${added.join(",") || "-"} -${removed.join(",") || "-"} versus the frozen table`);
  check(existsSync(dir + "/📋️project.json"), `${k}: project.json exists`);
}
const state = new Map<string, number>();
const visit = (k: string, trail: string[]) => {
  if (state.get(k) === 2) return;
  if (state.get(k) === 1) { check(false, "cycle " + [...trail, k].join(" -> ")); return; }
  state.set(k, 1);
  for (const d of edges.get(k) ?? []) if (edges.has(d)) visit(d, [...trail, k]);
  state.set(k, 2);
};
for (const k of mine) visit(k, []);
let longest = 0, files = 0, legacy = 0;
const touched = new Set(readFileSync(GEN + "/pwp4-touched.txt", "utf8").split("\n").filter(Boolean).map(r => "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/" + r));
const walk = (d: string) => { for (const n of readdirSync(d)) { if (n === "node_modules") continue; const p = d + "/" + n; const s = statSync(p); if (s.isDirectory()) walk(p); else { files++; const n = bytes(relative(ROOT, p)); if (touched.has(relative(ROOT, p))) longest = Math.max(longest, n); else legacy = Math.max(legacy, n); } } };
for (const d of ["⚛️reactor", "🚪️lifetime"]) walk(PLUG + "/" + d);
check(longest <= 240, `longest path ${longest} bytes`);
console.log(`crates ${mine.length}, files walked ${files}, longest path of a file written by WP4 ${longest} bytes, longest legacy path ${legacy} bytes, failures ${failures}`);
for (const d of deltas) console.log("delta", d);
console.log(failures === 0 ? "WP4 STATIC CHECKS PASSED" : "WP4 STATIC CHECKS FAILED");
process.exit(failures === 0 ? 0 : 1);
