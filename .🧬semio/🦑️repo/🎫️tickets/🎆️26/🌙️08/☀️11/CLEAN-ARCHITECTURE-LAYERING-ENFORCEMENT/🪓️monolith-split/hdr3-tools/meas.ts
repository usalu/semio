// 📏️ meas.ts <crate-name-or-manifest>... : lib and test closure ratios (all targets, default features) like the census
import {readFileSync, existsSync, readdirSync} from "node:fs";
import {dirname, relative, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const F = `${ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`;
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const readFile = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const featureSet = (parsed: any): Set<string> => {
  const table: Record<string, string[]> = parsed.features ?? {};
  const out = new Set<string>(["default"]), queue = [...(table.default ?? [])];
  while (queue.length) { const token = queue.pop()!; if (out.has(token)) continue; out.add(token); if (token.startsWith("dep:")) continue; const [head, rest] = token.split("/") as [string, string | undefined]; const name = head.replace(/\?$/u, ""); if (table[name]) queue.push(...table[name]); if (rest !== undefined && !head.endsWith("?")) out.add(name); }
  return out;
};
const manifestOfName = (name: string): string => {
  const lines = readFileSync(`${F}/🗑️generated/hdr3-crate-census.csv`, "utf8").split("\n");
  for (const l of lines) if (l.startsWith(name + ",")) return l.split(",")[1]!;
  throw new Error("unknown crate " + name);
};
const out: string[] = [];
for (const arg of process.argv.slice(2)) {
  const manifest = resolve(ROOT, arg.endsWith("Cargo.toml") ? arg : manifestOfName(arg));
  const dir = dirname(manifest);
  const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  const lim = parsed.package?.metadata?.semio?.["rust-source-boundary"] ?? {};
  const limit = {files: Math.min(32, lim.maxSourceFiles ?? 32), bytes: Math.min(262144, lim.maxSourceBytes ?? 262144), lines: Math.min(4000, lim.maxActiveLines ?? 4000)};
  const feats = [...featureSet(parsed)].filter(f => !f.startsWith("dep:") && !f.includes("/") && !f.endsWith("?")).map(f => "feature=" + f);
  const libEntry = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
  const itests: string[] = [];
  for (const t of parsed.test ?? []) itests.push(resolve(dir, t.path ?? `tests/${t.name}.rs`));
  if (parsed.autotests !== false) { const td = resolve(dir, "tests"); if (existsSync(td)) for (const row of readdirSync(td, {withFileTypes: true})) { if (row.isFile() && row.name.endsWith(".rs")) itests.push(resolve(td, row.name)); else if (row.isDirectory() && existsSync(resolve(td, row.name, "main.rs"))) itests.push(resolve(td, row.name, "main.rs")); } }
  const name = parsed.package.name;
  for (const test of [false, true]) {
    const cfg = [...hostCfg, "debug_assertions", ...(test ? ["test"] : []), ...feats];
    const entries = test ? [libEntry, ...itests] : [libEntry];
    let best = {r: -1, s: ""};
    for (const entry of entries.filter(e => existsSync(e))) {
      try {
        const got = await readRustSourceClosureV1([entry], ROOT, cfg, readFile, {manifestDirectory: dir, generated: () => undefined});
        const f = got.length, b = got.reduce((s, i) => s + i.bytes, 0), l = got.reduce((s, i) => s + i.activeLines, 0);
        const r = Math.max(f / limit.files, b / limit.bytes, l / limit.lines);
        if (r > best.r) best = {r, s: `${f}f/${b}B/${l}L [${relative(dir, entry).slice(-40)}]`};
      } catch (e) { best = {r: 99, s: "ERROR " + String((e as Error).message).slice(0, 120)}; }
    }
    out.push(`${name} ${test ? "test" : "lib "} ${best.r.toFixed(3)} ${best.s}`);
  }
}
console.log(out.join("\n"));
