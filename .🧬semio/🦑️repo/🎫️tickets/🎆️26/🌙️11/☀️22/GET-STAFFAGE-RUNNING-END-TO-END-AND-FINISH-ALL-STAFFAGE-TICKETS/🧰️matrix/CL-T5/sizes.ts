/** 📏️ The GLB bytes of every pose that is not a key pose of assets or trial copies with the live writer, against the budget of 8 MiB of `🧊️model-catalog` less the margin of this lane (0.5 MiB): sizes.ts <asset id | file.json>…
 * @see ../CX-FLOW/sizes.ts */
import { readFileSync } from "node:fs";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { writeGlb } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";

for (const target of process.argv.slice(2)) {
  const asset = target.endsWith(".json") ? JSON.parse(readFileSync(target, "utf8")).asset : (await readStreamedAsset((await assetPathsFromFiles()).find((e) => e.folder === target)!)).asset;
  const sizes: [string, number][] = [];
  for (const pose of asset.poses.filter((p: { key?: boolean }) => p.key !== true)) sizes.push([pose.id, (await writeGlb(asset, { pose: pose.id })).length]);
  const max = Math.max(...sizes.map(([, n]) => n));
  console.log(`SIZES ${asset.id} max ${max} bytes (${(max / 1048576).toFixed(2)} MiB) ${max <= 7.5 * 1048576 ? "ok" : max <= 8 * 1048576 ? "WITHIN 8 BUT NOT 7.5" : "OVER"} | ${sizes.map(([p, n]) => `${p}=${(n / 1048576).toFixed(2)}`).join(" ")}`);
}
