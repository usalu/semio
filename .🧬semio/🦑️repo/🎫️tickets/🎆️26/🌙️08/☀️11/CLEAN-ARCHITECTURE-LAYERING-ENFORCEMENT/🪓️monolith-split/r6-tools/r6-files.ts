// 📋️ Lists the source-closure files of one crate root (lib or test cfg) with bytes and active lines, largest first.
import {readFileSync} from "node:fs";
import {dirname, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const [manifestArg, mode] = process.argv.slice(2);
const manifest = resolve(ROOT, manifestArg!, "Cargo.toml");
const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
const entry = resolve(dirname(manifest), parsed.lib?.path ?? "src/lib.rs");
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const cfg = [...hostCfg, "debug_assertions", ...(mode === "test" ? ["test"] : [])];
const read = async (p: string) => { try { return readFileSync(p); } catch { return null; } };
const inputs = await readRustSourceClosureV1([entry], ROOT, cfg, read, {manifestDirectory: dirname(manifest), generated: () => undefined});
const rows = inputs.map(i => ({path: i.path, kind: i.kind, bytes: i.bytes, lines: i.activeLines})).sort((a, b) => b.bytes - a.bytes);
let tb = 0, tl = 0;
for (const r of rows) { tb += r.bytes; tl += r.lines; console.log(`${r.bytes}\t${r.lines}\t${r.kind}\t${r.path.replace(/^.*🪆️subsets\/♾️any\//u, "")}`); }
console.log(`TOTAL files=${rows.length} bytes=${tb} lines=${tl}`);
