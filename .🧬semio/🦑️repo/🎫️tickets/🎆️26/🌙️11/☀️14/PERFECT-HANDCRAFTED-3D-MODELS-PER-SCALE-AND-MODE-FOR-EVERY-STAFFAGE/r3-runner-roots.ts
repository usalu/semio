/** 🧬️ Writes the candidate document of `humans/runner-with-blade-prosthesis` with every root fix outside `volumes` (rig, anchors, poses, clip, default pose, dimensions, summary, and the redrawn 2D geometry and volumes when their files exist) to `🗑️generated/r3-runner-with-blade-prosthesis/🔣️.json`. Every value is absolute, so the script is re-runnable on its own result.
 * Usage: `bun r3-runner-roots.ts` then `bun r3-validate.ts --file <candidate>` then `python3 r3-runner-splice.py <members>`.
 * @see ./r3-runner-rig.ts */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { fixPoses, fixRig } from "./r3-runner-rig.ts";

export const ASSET_FILE = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/runner-with-blade-prosthesis/🔣️.json";
export const OUT = join(import.meta.dir, "🗑️generated", "r3-runner-with-blade-prosthesis");
export const DIMENSIONS = { heightM: 1.81, widthM: 0.41, depthM: 0.3 };

if (import.meta.main) {
  const doc = JSON.parse(readFileSync(ASSET_FILE, "utf8"));
  const asset = doc.asset;
  const log = fixRig(asset);
  fixPoses(asset);
  const dims = existsSync(join(OUT, "dimensions.json")) ? JSON.parse(readFileSync(join(OUT, "dimensions.json"), "utf8")) : DIMENSIONS;
  Object.assign(asset.dimensions, dims);
  asset.summary.en = asset.summary.en.replace("a J-shaped carbon blade attached below a carbon socket", "a C-shaped carbon blade bolted below a carbon socket").replace("Poses: standing, recovering, forefoot strike, blade mid-stance and flight.", "Poses: standing, recovering, forefoot strike, blade mid-stance, blade push-off and flight.");
  asset.summary.de = asset.summary.de.replace("einer J-förmigen Karbonfeder unter einem Karbonschaft", "einer C-förmigen Karbonfeder, die unter einem Karbonschaft verschraubt ist,").replace("Posen: Stehen, Erholen, Vorfußaufsatz, Feder im Stütz und Flug.", "Posen: Stehen, Erholen, Vorfußaufsatz, Feder im Stütz, Federabstoß und Flug.");
  if (existsSync(join(OUT, "geometry.json"))) {
    const geometry = JSON.parse(readFileSync(join(OUT, "geometry.json"), "utf8"));
    for (const [scale, views] of Object.entries(geometry) as [string, Record<string, unknown>][]) for (const [view, shapes] of Object.entries(views)) asset.representations[scale].geometry[view] = shapes;
  }
  asset.representations["500"].validRange.maxDenominator = 800;
  const source = join(import.meta.dir, "volumes", "humans", "runner-with-blade-prosthesis.json");
  if (process.argv.includes("--volumes")) asset.volumes = JSON.parse(readFileSync(source, "utf8")).volumes;
  mkdirSync(OUT, { recursive: true });
  writeFileSync(join(OUT, "🔣️.json"), `${JSON.stringify(doc, null, 2)}\n`);
  console.log(log.join("\n"));
  console.log(join(OUT, "🔣️.json"));
}
