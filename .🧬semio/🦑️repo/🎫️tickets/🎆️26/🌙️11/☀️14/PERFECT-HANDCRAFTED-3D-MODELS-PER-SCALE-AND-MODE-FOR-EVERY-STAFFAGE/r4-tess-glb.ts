/** 📦️ The GLB of the default pose of catalog assets at the product's tessellation: bytes and triangles against the oracle budget of 8 MB.
 *
 * Usage: `$BUN r4-tess-glb.ts category/slug ...`; prints one line per asset. */
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { writeGlbDetailed } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";

const only = process.argv.slice(2).map((id) => id.split("/").pop()!);
for (const entry of await assetPathsFromFiles()) {
  if (!only.includes(entry.folder)) continue;
  const { asset } = await readStreamedAsset(entry);
  const started = Date.now();
  const { bytes, triangles } = await writeGlbDetailed(asset, { pose: asset.presentation.defaultPose });
  console.log(`${entry.folder.padEnd(40)} ${triangles} triangles ${(bytes.length / 1048576).toFixed(2)} MB ${(bytes.length / triangles).toFixed(1)} B/triangle (${Date.now() - started} ms)`);
}
