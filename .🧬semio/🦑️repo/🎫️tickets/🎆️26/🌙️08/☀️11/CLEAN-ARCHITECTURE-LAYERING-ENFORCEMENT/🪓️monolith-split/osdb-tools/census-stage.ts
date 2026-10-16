// 🧮️ Source-closure rows (lib and test selection) of every staged db crate, via the canonical counter. Usage: bun census-stage.ts <sp dir> [cfg-mode]
import {readFileSync, existsSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const SPD = process.argv[2]!;
const STAGE_ROOT = SPD + "/stage";
const STAGE = STAGE_ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db";
const crates: {lib: string; root: string; files: string[]}[] = JSON.parse(readFileSync(SPD + "/crates.json", "utf8"));
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p.startsWith(STAGE_ROOT + "/") && !existsSync(p) ? "/Users/ueli/Documents/semio/" + p.slice(STAGE_ROOT.length + 1) : p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const rows: string[] = [];
let bad = 0;
for (const c of crates) {
  const entry = STAGE + "/" + c.root;
  const dir = entry.replace(/\/🦀️\.rs$/u, "") + "/📦️packages/🦀️rust";
  for (const [label, extra] of [["lib", []], ["test", ["test", "feature=testkit", "feature=claimed-backend-laws"]]] as [string, string[]][]) {
    try {
      const inputs = await readRustSourceClosureV1([entry], STAGE_ROOT, [...host, "debug_assertions", ...extra], read, {manifestDirectory: dir, generated: () => undefined});
      const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
      const over = inputs.length > 32 || bytes > 262144 || lines > 4000;
      const target = inputs.length > 28 || bytes > 230000 || lines > 3500;
      if (over) bad++;
      rows.push([c.lib, label, inputs.length, bytes, lines, over ? "OVER" : target ? "above-target" : "ok"].join("\t"));
    } catch (e) { rows.push([c.lib, label, "ERROR", String((e as Error).message).slice(0, 160)].join("\t")); bad++; }
  }
}
console.log(rows.join("\n"));
console.log("over-limit rows:", bad);
