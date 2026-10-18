// 🔭️ Closure lister with file age: bun cl.ts <test|lib> <crate-name> ... (manifest looked up in hdr3-crate-census.csv)
import {readFileSync, existsSync, statSync, readdirSync} from "node:fs";
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
const mode = process.argv[2]!;
const now = Date.now();
for (const name of process.argv.slice(3)) {
  const manifest = resolve(ROOT, manifestOfName(name));
  const dir = dirname(manifest);
  const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  const feats = [...featureSet(parsed)].filter(f => !f.startsWith("dep:") && !f.includes("/") && !f.endsWith("?")).map(f => "feature=" + f);
  const libEntry = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
  const entries: string[] = [];
  if (mode === "lib") entries.push(libEntry);
  else {
    if (existsSync(libEntry)) entries.push(libEntry);
    for (const t of parsed.test ?? []) entries.push(resolve(dir, t.path ?? `tests/${t.name}.rs`));
    if (parsed.autotests !== false) { const td = resolve(dir, "tests"); if (existsSync(td)) for (const row of readdirSync(td, {withFileTypes: true})) { if (row.isFile() && row.name.endsWith(".rs")) entries.push(resolve(td, row.name)); else if (row.isDirectory() && existsSync(resolve(td, row.name, "main.rs"))) entries.push(resolve(td, row.name, "main.rs")); } }
  }
  console.log(`## ${name}  ${relative(ROOT, manifest)}`);
  const cfg = [...hostCfg, "debug_assertions", ...(mode === "lib" ? [] : ["test"]), ...feats];
  for (const entry of entries.filter(e => existsSync(e))) {
    let got: any[];
    try { got = await readRustSourceClosureV1([entry], ROOT, cfg, readFile, {manifestDirectory: dir, generated: () => undefined}); } catch (e) { console.log("# ERROR " + String((e as Error).message).slice(0, 160)); continue; }
    console.log(`# entry ${relative(ROOT, entry)}: ${got.length} files ${got.reduce((s, i) => s + i.bytes, 0)} bytes ${got.reduce((s, i) => s + i.activeLines, 0)} lines`);
    if (got.length < 4 && entries.length > 1 && entry !== libEntry && got.length < 3) continue;
    for (const i of got as any[]) {
      const p = resolve(ROOT, i.path ?? ""); let age = "?"; try { age = String(Math.round((now - statSync(p).mtimeMs) / 60000)); } catch {}
      console.log(`${i.kind}\t${i.bytes}\t${i.activeLines}\t${age}m\t${relative(ROOT, p)}`);
    }
  }
}
