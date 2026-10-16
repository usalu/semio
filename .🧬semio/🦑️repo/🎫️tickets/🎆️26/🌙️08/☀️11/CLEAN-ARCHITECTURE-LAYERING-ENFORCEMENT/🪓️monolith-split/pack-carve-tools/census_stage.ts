// 🧮️ Source-closure rows (lib and test selections) of the carved pack crates, computed on the base+stage overlay with the canonical counter.
import {readFileSync, existsSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = process.env.OVERLAY ?? "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/S3";
const P = "🧰️framework/🔨️modules/🎒️pack/";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const crates: [string, string][] = JSON.parse(process.env.CRATES!);
const wasm = (execFileSync(rustc, ["--print", "cfg", "--target", "wasm32-unknown-unknown"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q))));
for (const [name, node] of crates) {
  const entry = ROOT + "/" + P + node + "/🦀️.rs", dir = ROOT + "/" + P + node + "/📦️packages/🦀️rust";
  for (const [label, cfg] of [["lib", [...host, "debug_assertions", 'feature=deflate']], ["test", [...host, "debug_assertions", "test", "feature=deflate"]], ["wasm", [...wasm, "debug_assertions", "feature=deflate"]]] as [string, string[]][]) {
    try {
      const inputs = await readRustSourceClosureV1([entry], ROOT, cfg, read, {manifestDirectory: dir, generated: () => undefined});
      const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0), emb = inputs.filter(i => i.kind === "embedded");
      const over = inputs.length > 32 || bytes > 262144 || lines > 4000 ? "OVER-GATE" : (inputs.length > 28 || bytes > 230000 || lines > 3500 ? "over-target" : "ok");
      console.log([name, label, "files", inputs.length, "bytes", bytes, "lines", lines, "embedded", emb.length, over].join("\t"));
    } catch (e) { console.log([name, label, "ERROR", String((e as Error).message).slice(0, 200)].join("\t")); }
  }
}
