/** 🧊️ Runs `modelOf`, the kernel's `validateModel` and `tessellateSolid` over every catalog asset: lists each asset that still fails with the message. Usage: bun catalog-model-check.ts */
import { validateModel } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { volumeIssues } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volume/🟦️.ts";
import { modelOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
import { catalogEntries, loadAsset, meshes } from "./volume-lib.ts";
let ok = 0;
const failures: string[] = [];
const entries = catalogEntries().filter((e) => !["environments", "print-profiles"].includes(e.category));
for (const entry of entries) {
  const asset = loadAsset(entry.path);
  if (asset?.rig === undefined) continue;
  try {
    const issues = volumeIssues(asset).map((i) => `${i.code} ${i.path}`);
    if (issues.length) throw new Error(`volumeIssues: ${issues.slice(0, 3).join("; ")}`);
    meshes(asset, asset.volume.solids, Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM));
    const document = await modelOf(asset, { scale: 50, views: ["light-solid"], poses: [], clips: [] } as never);
    const errors = validateModel(document).filter((i) => i.severity === "error");
    if (errors.length) throw new Error(`validateModel: ${errors.slice(0, 3).map((e) => JSON.stringify(e)).join("; ")}`);
    ok++;
  } catch (error) { failures.push(`${entry.category}/${entry.slug}: ${String(error).slice(0, 400)}`); }
}
console.log(`${ok} of ${ok + failures.length} assets pass modelOf + validateModel + tessellate`);
for (const f of failures) console.log(f);
