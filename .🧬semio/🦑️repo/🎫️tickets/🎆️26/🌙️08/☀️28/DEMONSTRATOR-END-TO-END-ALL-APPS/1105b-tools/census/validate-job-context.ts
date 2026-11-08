import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { statSync, readFileSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
const dir = ROOT + "/🧰️framework/🔨️modules/🧵️job/⏱️context/📦️packages/🦀️rust";
const entry = dir + "/🦀️.rs";
const cfgs: Record<string, string[]> = {
  "dev-default": ["debug_assertions", "fmt_debug=full", "overflow_checks", "panic=unwind", "target_os=macos", "target_family=unix", "target_arch=aarch64"],
  "rel-default": ["fmt_debug=full", "overflow_checks", "panic=unwind", "target_os=macos", "target_family=unix", "target_arch=aarch64"],
};
for (const [name, cfg] of Object.entries(cfgs)) {
  const reads: [string, number][] = [];
  const read = async (p: string) => { try { const b = new Uint8Array(readFileSync(p)); if (!reads.some(x => x[0] === p)) reads.push([p, b.byteLength]); return b; } catch { return null; } };
  const inputs = await readRustSourceClosureV1([entry], ROOT, cfg, read, { manifestDirectory: dir, generated: () => undefined });
  console.log(name, "files", inputs.length, "bytes", inputs.reduce((s, i) => s + i.bytes, 0), "lines", inputs.reduce((s, i) => s + i.activeLines, 0));
  if (name === "dev-default") for (const i of inputs) console.log("  ", i.kind, i.bytes, i.activeLines, i.path, new Date(statSync(ROOT + "/" + i.path).mtimeMs).toISOString().slice(11, 19));
}
