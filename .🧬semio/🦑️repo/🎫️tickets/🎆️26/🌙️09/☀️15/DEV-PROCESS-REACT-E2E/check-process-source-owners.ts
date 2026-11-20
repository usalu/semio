import { readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { parse } from "smol-toml";

const root = process.cwd();
const counterUrl = pathToFileURL(resolve(root, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const { readRustSourceClosureV1 } = await import(counterUrl.href);
const scopes = [
  "🧰️framework/🔨️modules/🧊️3d/📐️brep",
  "✏️s/🔌️plugins/🏭️process",
];
const cfg = [process.platform === "win32" ? "windows" : "unix"];
const read = async (path: string) => {
  try { return await readFile(path); }
  catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return null;
    throw error;
  }
};
const measured = [];
for (const scope of scopes) {
  for (const relative of new Bun.Glob("**/Cargo.toml").scanSync({ cwd: scope })) {
    const manifestPath = resolve(root, scope, relative);
    const manifestDirectory = dirname(manifestPath);
    const manifest = parse(await Bun.file(manifestPath).text()) as {
      package?: { name: string };
      lib?: { path?: string };
    };
    if (!manifest.package || !manifest.lib?.path) continue;
    const entry = resolve(manifestDirectory, manifest.lib.path);
    const files = await readRustSourceClosureV1([entry], root, cfg, read, { manifestDirectory, generated: () => undefined });
    measured.push({
      crate: manifest.package.name,
      lines: files.reduce((sum: number, file: { activeLines: number }) => sum + file.activeLines, 0),
      bytes: files.reduce((sum: number, file: { bytes: number }) => sum + file.bytes, 0),
    });
  }
}
const closeToLimit = measured.filter(row => row.lines >= 3700 || row.bytes >= 240000).sort((a, b) => b.lines - a.lines);
console.log("[DEBUG] Measured", measured.length, "declared production library closures", JSON.stringify(closeToLimit, null, 2));
if (measured.some(row => row.lines > 4000 || row.bytes > 262144)) process.exitCode = 1;
