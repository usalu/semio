/** 🔺️ Triangles per scene of every asset of a shard: tri.ts <out.json> <index/count> [id…] (all assets without ids) writes `{asset: {scene: triangles}}` through the kernel's own tessellation at `qualityOf`. */
import { writeFileSync } from "node:fs";
import { scenesOf, tessellateSolid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { modelOfAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
import { ROUNDNESS, maxSegmentsAt, qualityOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";

const old = (asset: any, scale: number) => {
  const largest = Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM);
  const featureM = ROUNDNESS.paperFeatureM * scale;
  return { chordM: Math.min(largest / ROUNDNESS.largestPerChord, ROUNDNESS.chordPerFeature * featureM), minSegments: ROUNDNESS.minSegments, maxSegments: maxSegmentsAt(scale), round: { radiusM: featureM, segments: ROUNDNESS.roundSegments } };
};
const [out, shard, ...ids] = process.argv.slice(2);
const [index, count] = shard!.split("/").map(Number) as [number, number];
const entries = (await assetPathsFromFiles()).filter((e, i) => i % count! === index && (ids.length === 0 || ids.includes(e.folder)));
const result: Record<string, { before: Record<string, number>; after: Record<string, number> } | string> = {};
for (const entry of entries) {
  try {
    const { asset } = await readStreamedAsset(entry);
    const document = await modelOfAsset(asset, { poses: [asset.presentation.defaultPose], clips: [] });
    const before: Record<string, number> = {};
    const after: Record<string, number> = {};
    for (const scene of scenesOf(document)) {
      const scale = Number(/^s(\d+)-/u.exec(scene.id)![1]);
      const members = new Set(scene.nodes);
      let n = 0;
      let m = 0;
      for (const solid of document.solids) if (members.has(solid.node)) {
        n += tessellateSolid(solid, old(asset, scale)).indices.length / 3;
        m += tessellateSolid(solid, qualityOf(asset, scale)).indices.length / 3;
      }
      before[scene.id] = n;
      after[scene.id] = m;
    }
    result[entry.folder] = { before, after };
  } catch (error) {
    result[entry.folder] = `ERROR ${String(error).slice(0, 300)}`;
  }
  writeFileSync(out!, JSON.stringify(result));
}
