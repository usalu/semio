import { writeFileSync, mkdirSync } from "node:fs";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { writeGlbDetailed } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";
const [out, ...ids] = process.argv.slice(2);
mkdirSync(out!, { recursive: true });
const entries = await assetPathsFromFiles();
for (const id of ids) {
  const [name, poseArg] = id.split(":");
  const { asset } = await readStreamedAsset(entries.find((e) => e.folder === name)!);
  const pose = poseArg || asset.poses.find((p: any) => p.key !== true)!.id;
  const r = await writeGlbDetailed(asset, { pose });
  writeFileSync(`${out}/${name}.glb`, r.bytes);
  console.log("[DEBUG]", name, pose, r.bytes.length, r.triangles);
  (globalThis as any).Bun?.gc(true);
}
