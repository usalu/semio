
import { readFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = process.cwd();
const { createRustSourceAnalysisV1, readRustSourceClosureV1 } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const graph = JSON.parse(await readFile(process.argv[2], "utf8"));
const select = process.argv.slice(3);
const analysis = createRustSourceAnalysisV1();
for (const unit of graph.units) {
  const name = unit.target.name;
  if (!unit.pkg_id.startsWith("path+") || unit.platform !== "wasm32-wasip2" || !select.some(prefix => name.startsWith(prefix))) continue;
  const manifestDirectory = fileURLToPath(unit.pkg_id.slice(5).split("#", 1)[0]);
  const entry = resolve(unit.target.src_path);
  const cfg = [
    'target_arch="wasm32"', 'target_os="wasi"', 'target_env="p2"', 'target_family="wasm"',
    'target_vendor="unknown"', 'target_pointer_width="32"', 'target_endian="little"',
  ];
  if (unit.profile.debug_assertions) cfg.push("debug_assertions");
  cfg.push(...unit.features.map(feature => `feature=${JSON.stringify(feature)}`));
  const files = await readRustSourceClosureV1([entry], root, cfg, path =>
    readFile(path).catch(error => { if (error.code === "ENOENT") return null; throw error; }),
    { manifestDirectory, generated: () => undefined }, analysis);
  const bytes = files.reduce((sum, file) => sum + file.bytes, 0);
  const lines = files.reduce((sum, file) => sum + file.activeLines, 0);
  const extent = `${files.length} files, ${bytes} bytes, ${lines} active lines`;
  console.log(`${name} (${unit.platform ?? "host"}): ${extent}`);
  if ((files.length > 32 || bytes > 262_144 || lines > 4_000) && select.length === 1) {
    for (const file of [...files].sort((a, b) => b.bytes - a.bytes).slice(0, 12)) {
      console.log(`  ${file.bytes} ${file.path}`);
    }
  }
}
