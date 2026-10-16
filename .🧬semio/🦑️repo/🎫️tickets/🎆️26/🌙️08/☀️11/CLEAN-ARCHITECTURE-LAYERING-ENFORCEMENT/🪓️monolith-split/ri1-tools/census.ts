// 🧮️ Source-closure rows (canonical counter) of the carved renderer-wgpu crates and the monolith top crate: lib and test closures, native and wasm32-unknown-unknown.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const T = ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu";
const split = process.env.SPLIT!;
const crates: Record<string, string> = JSON.parse(process.env.CRATES!);
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const wasm = [...host.filter(c => !/^(target_arch|target_os|target_family|target_vendor|target_env|target_endian|unix|windows|target_has_atomic|target_feature|target_thread_local|target_pointer_width)/u.test(c)), "target_arch=wasm32", "target_os=unknown", "target_family=wasm", "target_pointer_width=32", "target_endian=little"];
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const rows: string[] = [];
for (const [name, folder] of Object.entries(crates)) {
  const entry = folder === "<top>" ? T + "/📦️packages/🦀️rust/📚️library/🦀️.rs" : T + "/" + folder + "/🦀️.rs";
  const dir = folder === "<top>" ? T + "/📦️packages/🦀️rust" : T + "/" + folder + "/📦️packages/🦀️rust";
  for (const [target, cfg] of [["native", host], ["wasm", wasm]] as [string, string[]][]) {
    for (const [label, extra] of [["lib", []], ["test", ["test"]]] as [string, string[]][]) {
      try {
        const inputs = await readRustSourceClosureV1([entry], ROOT, [...cfg, "debug_assertions", ...extra], read, {manifestDirectory: dir, generated: () => undefined});
        const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0), emb = inputs.filter(i => i.kind === "embedded");
        const over = inputs.length > 32 || bytes > 262144 || lines > 4000 ? "OVER-GATE" : inputs.length > 28 || bytes > 230000 || lines > 3500 ? "over-target" : "ok";
        rows.push([name, target, label, inputs.length, bytes, lines, emb.length, over].join("\t"));
      } catch (e) { rows.push([name, target, label, "ERROR", String((e as Error).message).slice(0, 160).replaceAll("\n", " ")].join("\t")); }
    }
  }
}
console.log(rows.join("\n"));
