// 📏️ Single-crate canonical source-gate measure: bun one.ts <manifest-dir-or-Cargo.toml>... ; lists files when VERBOSE=1.
import { readFileSync } from "node:fs";
import { join, posix, dirname } from "node:path";
import { readRustSourceClosureV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { crateTreeHostCfg } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const read = async (abs: string) => { try { return readFileSync(abs); } catch { return null; } };
const host = [...crateTreeHostCfg()];
for (const arg of process.argv.slice(2)) {
  const manifest = arg.endsWith("Cargo.toml") ? arg : posix.join(arg, "Cargo.toml");
  const dir = dirname(manifest);
  const text = readFileSync(join(ROOT, manifest), "utf8");
  const name = /^name\s*=\s*"([^"]+)"/mu.exec(text)?.[1];
  const lib = /^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"/mu.exec(text)?.[1] ?? "src/lib.rs";
  for (const [label, cfg] of [["prod", host], ["test", [...host, "test"]]] as const) {
    try {
      const inputs = await readRustSourceClosureV1([join(ROOT, dir, lib)], ROOT, cfg, read, { manifestDirectory: join(ROOT, dir), generated: () => undefined });
      const files = inputs.length, bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
      const ratio = Math.max(files / 32, bytes / 262144, lines / 4000);
      console.log(name, label, `${files}f ${bytes}b ${lines}l ratio=${ratio.toFixed(3)}`);
      if (process.env.VERBOSE) for (const i of inputs) console.log("   ", i.activeLines, i.bytes, i.path.replace(ROOT + "/", ""));
    } catch (e) { console.log(name, label, "REFUSED", String((e as Error).message)); }
  }
}
