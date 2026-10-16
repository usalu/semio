// 🧮️ Union of canonical closures of several entry files per planned crate (lib and test cfg). usage: bun plan_sizes.ts plan.json
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = process.env.ROOT_DIR ?? "/Users/ueli/Documents/semio";
const FL = ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/";
const plan = JSON.parse(readFileSync(process.argv[2], "utf8")) as Record<string, string[]>;
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
for (const [crate, entries] of Object.entries(plan)) {
  for (const [label, extra] of [["lib", []], ["test", ["test"]]] as [string, string[]][]) {
    const seen = new Map<string, {bytes: number; lines: number}>();
    for (const e of entries) {
      try {
        const inputs = await readRustSourceClosureV1([FL + e], ROOT, [...host, "debug_assertions", ...extra], read, {manifestDirectory: FL, generated: () => undefined});
        for (const i of inputs) seen.set(i.path, {bytes: i.bytes, lines: i.activeLines});
      } catch (err) { console.log(crate, label, "ERROR", e, String((err as Error).message).slice(0, 100)); }
    }
    let b = 0, l = 0; for (const v of seen.values()) { b += v.bytes; l += v.lines; }
    console.log([crate, label, "files", seen.size, "bytes", b, "lines", l, (seen.size > 32 || b > 262144 || l > 4000) ? "OVER" : (seen.size > 28 || b > 230000 || l > 3500) ? "over-target" : "ok"].join("\t"));
  }
}
