/** 📏️ Measures each owned-runtime crate's authored native production closure against the global source admission ceiling (4,000 active lines / 262,144 bytes / 32 files). */
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const workspace = resolve(import.meta.dir, "../../../../../../../.."), cfg = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix"];
let failed = false;
for (const entry of process.argv.slice(2)) {
  const rows = await readRustSourceClosureV1([resolve(workspace, entry)], workspace, cfg, async path => await readFile(path).catch(error => (error.code === "ENOENT" ? null : Promise.reject(error))));
  const lines = rows.reduce((sum, row) => sum + row.activeLines, 0), bytes = rows.reduce((sum, row) => sum + row.bytes, 0), ok = lines <= 4000 && bytes <= 262144 && rows.length <= 32;
  failed ||= !ok;
  console.log(`${ok ? "PASS" : "FAIL"} files=${rows.length} activeLines=${lines} bytes=${bytes} ${entry}`);
  if (process.env.SOURCE_ADMISSION_ROWS) for (const row of rows) console.log(`  ${row.activeLines}\t${row.bytes}\t${row.path}`);
}
process.exit(failed ? 1 : 0);
