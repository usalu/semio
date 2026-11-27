/** 🧪️ Trial of the staged roundness levels (`stage/job.ts`, written by `land.py --stage`; the live tree is untouched): per asset the model level, the level and cap of every authored scale, the nominal triangles on the full level and on its levels, the triangles the kernel really tessellates on its levels (every authored solid once), their ratio, the nominal triangles of the three lists per scale and the time the levels took. trial.ts <ids…> */
import { tessellateSolid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { listOf, solidOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🧊️solids/🟦️.ts";
import { ROUNDNESS, nominalTriangles, qualityOf, roundnessLevels, volumeTriangles } from "./stage/job.ts";

const entries = await assetPathsFromFiles();
for (const id of process.argv.slice(2)) {
  const { asset } = await readStreamedAsset(entries.find((e) => e.folder === id)!);
  const started = performance.now();
  const levels = roundnessLevels(asset);
  const took = performance.now() - started;
  let tessellated = 0;
  let nominal = 0;
  let full = 0;
  const rows: string[] = [];
  for (const volume of Object.values(asset.volumes)) {
    if (volume.status !== "authored") continue;
    const quality = qualityOf(asset, volume.denominator);
    nominal += volumeTriangles(asset, volume, levels.scales.get(volume.denominator)!);
    full += volumeTriangles(asset, volume, ROUNDNESS.fullLevel);
    for (const solid of [...volume.body, ...volume.thermal, ...volume.structure]) if ("kind" in solid) tessellated += tessellateSolid(solidOf(solid as never, "trial"), quality).indices.length / 3;
    rows.push(`1:${volume.denominator} level ${levels.scales.get(volume.denominator)} cap ${quality.maxSegments} lists ${(["body", "thermal", "structure"] as const).map((list) => listOf(volume, list).reduce((n, s) => n + nominalTriangles(s, quality), 0)).join("/")}`);
  }
  console.log(`${id}: model level ${levels.model}; nominal ${full} -> ${nominal}, tessellated ${tessellated} (${(tessellated / nominal).toFixed(3)} of nominal); levels in ${took.toFixed(1)} ms\n   ${rows.join("\n   ")}`);
}
