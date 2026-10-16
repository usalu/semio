// 🧮️ Source-closure rows (lib and cfg-test) for given entry/package-dir pairs via the canonical counter. Usage: bun closure.ts <entry> <pkgDir> [list]
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const [entry, dir, list, feats] = process.argv.slice(2);
for (const [label, extra] of [["lib", []], ["test", ["test", ...(feats ? feats.split(",").map(f => "feature=" + f) : [])]]] as [string, string[]][]) {
  try {
    const inputs = await readRustSourceClosureV1([entry!], ROOT, [...host, "debug_assertions", ...extra], read, {manifestDirectory: dir!, generated: () => undefined});
    const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0), emb = inputs.filter(i => i.kind === "embedded");
    console.log([label, "files", inputs.length, "bytes", bytes, "lines", lines, "embedded", emb.length, emb.reduce((s, i) => s + i.bytes, 0), (inputs.length > 32 || bytes > 262144 || lines > 4000) ? "OVER" : "ok"].join("\t"));
    if (list) for (const i of inputs) console.log("   ", i.kind, i.bytes, i.activeLines, i.path.replace(/^.*🎛️dashboard\//, ""));
  } catch (e) { console.log(label, "ERROR", String((e as Error).message).slice(0, 200)); }
}
