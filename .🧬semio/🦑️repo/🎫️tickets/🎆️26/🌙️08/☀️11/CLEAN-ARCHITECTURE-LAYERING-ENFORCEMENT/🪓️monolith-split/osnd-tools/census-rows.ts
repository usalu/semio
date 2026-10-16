// 🧮️ Source-closure rows (lib and cfg-test) for crates given as `name|entry|manifestDir` arguments, via the canonical counter.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const targets = (process.env.TARGETS_CFG ?? "").split(",").filter(Boolean);
for (const arg of process.argv.slice(2)) {
  const [name, entry, dir, features = ""] = arg.split("|");
  const feat = features.split(",").filter(Boolean).map(f => "feature=" + f);
  for (const [label, extra] of [["lib", []], ["test", ["test"]], ["wasm", ['target_arch=wasm32', 'target_os=unknown', 'target_family=wasm']], ["wasip2", ['target_arch=wasm32', 'target_os=wasi', 'target_env=p2', 'target_family=wasm']]] as [string, string[]][]) {
    if ((label === "wasm" || label === "wasip2") && !process.env.WASM) continue;
    try {
      const base = host.filter(v => !extra.some(e => e.startsWith("target_arch")) || !/^(target_arch|target_os|target_family|target_env|unix|target_pointer_width|target_endian)/.test(v));
      const inputs = await readRustSourceClosureV1([entry], ROOT, [...base, "debug_assertions", ...feat, ...extra], read, {manifestDirectory: dir, generated: () => undefined});
      const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0), emb = inputs.filter(i => i.kind === "embedded");
      console.log([name, label, "files", inputs.length, "bytes", bytes, "lines", lines, "embedded", emb.length, emb.reduce((s, i) => s + i.bytes, 0), (inputs.length > 32 || bytes > 262144 || lines > 4000) ? "OVER" : (inputs.length > 28 || bytes > 230000 || lines > 3500) ? "over-target" : "ok"].join("\t"));
    } catch (e) { console.log([name, label, "ERROR", String((e as Error).message).slice(0, 200)].join("\t")); }
  }
}
