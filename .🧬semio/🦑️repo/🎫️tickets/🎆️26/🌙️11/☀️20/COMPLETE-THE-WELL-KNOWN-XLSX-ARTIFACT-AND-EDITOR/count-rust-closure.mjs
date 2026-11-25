import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
const root = process.cwd();
const mod = await import(resolve(root, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const cfg = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix"];
for (const entry of process.argv.slice(2)) {
  try {
    const rows = await mod.readRustSourceClosureV1([resolve(root, entry)], root, cfg, p => readFile(p).catch(e => e.code === "ENOENT" ? null : Promise.reject(e)));
    const lines = rows.reduce((s, r) => s + r.activeLines, 0);
    console.log(lines, rows.length, entry);
    if (process.env.DETAIL) for (const r of rows) console.log("   ", r.activeLines, r.path);
  } catch (e) { console.log("ERR", entry, String(e.message ?? e).slice(0, 200)); }
}
