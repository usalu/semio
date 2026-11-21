import { readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { parse } from "smol-toml";

const root = process.cwd();
const counterUrl = pathToFileURL(resolve(root, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const { readRustSourceClosureV1 } = await import(counterUrl.href);
const scopes = process.argv.length > 2 ? process.argv.slice(2) : [
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
const unresolved = [];
for (const scope of scopes) {
  for (const relative of new Bun.Glob("**/Cargo.toml").scanSync({ cwd: scope, followSymlinks: false })) {
    const manifestPath = resolve(root, scope, relative);
    const manifestDirectory = dirname(manifestPath);
    try {
      const manifest = parse(await Bun.file(manifestPath).text()) as {
        package?: { name: string };
        lib?: { path?: string };
      };
      if (!manifest.package || !manifest.lib?.path) continue;
      const entry = resolve(manifestDirectory, manifest.lib.path);
      const files = await readRustSourceClosureV1([entry], root, cfg, read, { manifestDirectory, generated: () => undefined });
      measured.push({
        crate: manifest.package.name,
        files: files.length,
        lines: files.reduce((sum: number, file: { activeLines: number }) => sum + file.activeLines, 0),
        bytes: files.reduce((sum: number, file: { bytes: number }) => sum + file.bytes, 0),
      });
    } catch (error) {
      unresolved.push({ manifest: `${scope}/${relative}`, error: error instanceof Error ? error.message : String(error) });
    }
  }
}
const exceeds = ({ files, lines, bytes }: (typeof measured)[number]) => files > 32 || lines > 4000 || bytes > 262144;
const closeToLimit = measured.filter(row => row.files >= 30 || row.lines >= 3700 || row.bytes >= 240000);
console.log("[DEBUG] Production library closures", JSON.stringify({
  measured: measured.length,
  closeToLimit: closeToLimit.sort((a, b) => Number(exceeds(b)) - Number(exceeds(a)) || b.lines - a.lines),
  unresolved,
}, null, 2));
if (measured.some(exceeds) || unresolved.length) process.exitCode = 1;
