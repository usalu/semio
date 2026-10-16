// 🧮️ Source-closure rows (lib and test selections, native, wasm32 and deflate) of the carved replication crates on the base+stage overlay with the canonical counter.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = process.env.OVERLAY!;
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfgOf = (args: string[]) => execFileSync(rustc, ["--print", "cfg", ...args], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const host = cfgOf([]), unknown = cfgOf(["--target", "wasm32-unknown-unknown"]), wasip2 = cfgOf(["--target", "wasm32-wasip2"]);
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const crates: [string, string, string][] = JSON.parse(process.env.CRATES!);
let worst = 0;
for (const [name, entryRel, dirRel] of crates) {
  const entry = ROOT + "/" + entryRel, dir = ROOT + "/" + dirRel;
  const rows: [string, string[]][] = [
    ["lib", [...host, "debug_assertions"]], ["lib+deflate", [...host, "debug_assertions", "feature=deflate"]], ["test", [...host, "debug_assertions", "test"]], ["test+deflate", [...host, "debug_assertions", "test", "feature=deflate"]],
    ["wasm-unknown", [...unknown, "debug_assertions"]], ["wasm-unknown test", [...unknown, "debug_assertions", "test"]], ["wasip2", [...wasip2, "debug_assertions"]],
  ];
  for (const [label, cfg] of rows) {
    try {
      const inputs = await readRustSourceClosureV1([entry], ROOT, cfg, read, {manifestDirectory: dir, generated: () => undefined});
      const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
      const verdict = inputs.length > 32 || bytes > 262144 || lines > 4000 ? "OVER-GATE" : (inputs.length > 28 || bytes > 230000 || lines > 3500 ? "over-target" : "ok");
      if (verdict !== "ok") worst++;
      console.log([name, label, "files", inputs.length, "bytes", bytes, "lines", lines, verdict].join("\t"));
    } catch (e) { worst++; console.log([name, label, "ERROR", String((e as Error).message).slice(0, 200)].join("\t")); }
  }
}
console.log("NON-OK ROWS", worst);
