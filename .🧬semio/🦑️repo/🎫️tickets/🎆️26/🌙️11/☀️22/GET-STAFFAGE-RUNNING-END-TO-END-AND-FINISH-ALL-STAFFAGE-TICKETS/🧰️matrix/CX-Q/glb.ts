/** 📦️ GLB bytes and triangles of the pose files named in a list (`<MB> <asset> <pose>` per line): glb.ts <list> <out.json> */
import { readFileSync, writeFileSync } from "node:fs";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { writeGlbDetailed } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";

const [list, out] = process.argv.slice(2);
const entries = await assetPathsFromFiles();
const result: Record<string, { bytes: number; triangles: number; ms: number }> = {};
for (const line of readFileSync(list!, "utf8").split("\n").filter((l) => l.trim() !== "")) {
  const [, id, pose] = line.trim().split(/\s+/u);
  const { asset } = await readStreamedAsset(entries.find((e) => e.folder === id)!);
  const t = Date.now();
  const r = await writeGlbDetailed(asset, { pose });
  result[`${id}/${pose}`] = { bytes: r.bytes.length, triangles: r.triangles, ms: Date.now() - t };
  writeFileSync(out!, JSON.stringify(result));
}
