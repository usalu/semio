import { readFileSync } from "node:fs";
import { exportCast } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";

const manifestPath = process.argv[2]!;
const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
const ids = process.argv[3] ? process.argv[3].split(",") : Object.keys(manifest.casts);
try {
  const out = await exportCast(manifest, ids, process.argv[4]!, {});
  console.log(`ok ${out.length}`);
} catch (error) {
  console.log((error as Error).stack);
  process.exitCode = 1;
}
