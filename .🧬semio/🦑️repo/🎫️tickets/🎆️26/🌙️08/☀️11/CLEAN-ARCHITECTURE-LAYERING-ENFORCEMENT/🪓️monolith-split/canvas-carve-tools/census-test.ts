// 🧮️ Source-closure rows of the canvas crates for the lib target and for the unit-test selection (cfg test, testkit), via the canonical counter.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const C = ROOT + "/🧰️framework/🔨️modules/🖼️canvas";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const crates: [string, string, string][] = [
  ["semio-framework-canvas", C + "/🦀️.rs", C + "/📦️packages/🦀️rust"],
  ["semio-framework-canvas-assets", C + "/🔌️runtime-assets/🦀️.rs", C + "/🔌️runtime-assets/📦️packages/🦀️rust"],
  ["semio-framework-canvas-camera", C + "/📷️camera/🦀️.rs", C + "/📷️camera/📦️packages/🦀️rust"],
  ["semio-framework-canvas-icons", C + "/🪪️icon-codec/🦀️.rs", C + "/🪪️icon-codec/📦️packages/🦀️rust"],
  ["semio-framework-canvas-scene", C + "/🎬️scene/🦀️.rs", C + "/🎬️scene/📦️packages/🦀️rust"],
];
const targets: [string, string[]][] = [["native", []], ["wasm", ['target_arch=wasm32']], ["wasip2", ['target_arch=wasm32', 'target_env=p2']]];
for (const [name, entry, dir] of crates) {
  for (const [label, extra] of [["lib", []], ["test", ["test", "feature=testkit"]]] as [string, string[]][]) {
    try {
      const inputs = await readRustSourceClosureV1([entry], ROOT, [...host, "debug_assertions", ...extra], read, {manifestDirectory: dir, generated: () => undefined});
      const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0), emb = inputs.filter(i => i.kind === "embedded");
      console.log([name, label, "files", inputs.length, "bytes", bytes, "lines", lines, "embedded", emb.length, emb.reduce((s, i) => s + i.bytes, 0), (inputs.length > 32 || bytes > 262144 || lines > 4000) ? "OVER" : (inputs.length > 28 || bytes > 230000 || lines > 3500) ? "over-target" : "ok"].join("\t"));
    } catch (e) { console.log([name, label, "ERROR", String((e as Error).message).slice(0, 200)].join("\t")); }
  }
}
