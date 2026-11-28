/** 📏️ GLB bytes of every non-key pose (or only the default pose with --default) of the given assets: sizes.ts [--default] <id>… */
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { writeGlb } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";
const args = process.argv.slice(2); const only = args.includes("--default"); const ids = args.filter((a) => !a.startsWith("--"));
const entries = await assetPathsFromFiles();
for (const id of ids) {
  const { asset } = await readStreamedAsset(entries.find((e) => e.folder === id)!);
  const poses = asset.poses.filter((p: any) => p.key !== true).map((p: any) => p.id).filter((p: string) => !only || p === asset.presentation.defaultPose);
  let max = 0; const sizes: string[] = [];
  for (const pose of poses) { const n = (await writeGlb(asset, { pose })).length; max = Math.max(max, n); sizes.push(`${pose}=${(n / 1048576).toFixed(2)}`); }
  console.log(`RESULT ${id} max ${(max / 1048576).toFixed(2)} MiB ${max <= 8 * 1048576 ? "ok" : "OVER"} | ${sizes.join(" ")}`);
}
