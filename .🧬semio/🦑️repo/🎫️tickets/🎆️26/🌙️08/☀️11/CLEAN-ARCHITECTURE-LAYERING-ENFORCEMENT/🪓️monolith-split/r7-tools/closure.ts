// 📋️ Lists the source closure files (lines, bytes) of one crate lib root for a given cfg: usage `bun closure.ts <root> <libRootFile> [--test]`.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve, dirname} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const [root, entry, ...flags] = process.argv.slice(2);
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const cfg = [...host, "debug_assertions", ...(flags.includes("--test") ? ["test"] : [])];
const read = async (p: string) => { try { return readFileSync(p); } catch { return null; } };
const inputs = await readRustSourceClosureV1([resolve(root!, entry!)], root!, cfg, read, {manifestDirectory: dirname(resolve(root!, entry!)), generated: () => undefined});
for (const i of inputs.sort((a, b) => b.activeLines - a.activeLines)) console.log(i.activeLines, i.bytes, i.kind, i.path ?? (i as any).file ?? "");
console.log("total", inputs.length, inputs.reduce((s, i) => s + i.bytes, 0), inputs.reduce((s, i) => s + i.activeLines, 0));
