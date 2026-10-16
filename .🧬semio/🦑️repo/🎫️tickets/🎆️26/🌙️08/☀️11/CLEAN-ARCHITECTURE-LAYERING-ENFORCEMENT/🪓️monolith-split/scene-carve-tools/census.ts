// 🧮️ Source-closure rows (lib and unit-test selection) of the ui-scene crates via the canonical counter. Usage: bun census.ts <entry.rs>... or no args for the carved crates.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const entries = process.argv.slice(2);
const kit = (process.env.FEATURES ?? "").split(",").filter(Boolean).map(f => `feature=${f}`);
for (const entry of entries) {
  const dir = entry.replace(/\/[^/]+$/u, "");
  for (const [label, extra] of [["lib", []], ["test", ["test"]]] as [string, string[]][]) {
    try {
      const inputs = await readRustSourceClosureV1([entry], ROOT, [...host, "debug_assertions", ...kit, ...extra], read, {manifestDirectory: dir, generated: () => undefined});
      const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0), emb = inputs.filter(i => i.kind === "embedded");
      console.log([entry.replace(ROOT + "/🧰️framework/🔨️modules/🖱️ui/", ""), label, "files", inputs.length, "bytes", bytes, "lines", lines, "embedded", emb.length, emb.reduce((s, i) => s + i.bytes, 0), (inputs.length > 28 || bytes > 230000 || lines > 3500) ? "OVER-TARGET" : "ok"].join("\t"));
    } catch (e) { console.log([entry, label, "ERROR", String((e as Error).message).slice(0, 200)].join("\t")); }
  }
}
