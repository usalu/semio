import { readFileSync, readdirSync, rmSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { exportCast } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";

const manifest = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const root = process.argv[3]!;
const only = process.argv[4] ? process.argv[4].split(",") : Object.keys(manifest.casts);
const walk = (dir: string): string[] => readdirSync(dir, { withFileTypes: true }).flatMap((e) => (e.isDirectory() ? walk(join(dir, e.name)) : [join(dir, e.name)]));
for (const id of only) {
  const out = join(root, id);
  rmSync(out, { recursive: true, force: true });
  mkdirSync(out, { recursive: true });
  try {
    await exportCast(manifest, [id], out, {});
    const bad = walk(out).filter((f) => /\.(svg|json)$/.test(f) && readFileSync(f, "utf8").includes("NaN"));
    console.log(bad.length > 0 ? `NAN ${id} ${bad.length} files e.g. ${bad[0]}` : `ok ${id}`);
  } catch (error) {
    console.log(`ERR ${id} ${(error as Error).message}`);
  }
  if (!process.argv[5]) rmSync(out, { recursive: true, force: true });
}
