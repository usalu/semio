/** 🌿️ Where the triangles of an asset's scenes go, by group of solids: per scene the total, then per group (the solid id with every number replaced by `#`, and its kind) the solids, the triangles and the triangles per solid, heaviest first; the budget findings of `🧊️model-catalog` at the end. groups.ts <asset id | file.json> [--top <n>] [--scene <id>]…
 * @see ../CL-3D/profile.ts */
import { readFileSync } from "node:fs";
import { scenesOf, tessellateSolid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { modelOfAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
import { qualityOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";

const [target, ...rest] = process.argv.slice(2);
const top = rest.includes("--top") ? Number(rest[rest.indexOf("--top") + 1]) : 12;
const wanted = rest.filter((a, i) => rest[i - 1] === "--scene");
const asset = target!.endsWith(".json") ? JSON.parse(readFileSync(target!, "utf8")).asset : (await readStreamedAsset((await assetPathsFromFiles()).find((e) => e.folder === target)!)).asset;
const document = await modelOfAsset(asset, { poses: "all", clips: [] });
const totals: Record<string, Record<number, number>> = {};
for (const scene of scenesOf(document)) {
  const scale = Number(/^s(\d+)-/u.exec(scene.id)![1]);
  const mode = scene.id.replace(/^s\d+-/u, "");
  const quality = qualityOf(asset, scale);
  const members = new Set(scene.nodes);
  const groups: Record<string, { solids: number; triangles: number }> = {};
  let total = 0;
  let count = 0;
  for (const solid of document.solids.filter((s) => members.has(s.node))) {
    const n = tessellateSolid(solid, quality).indices.length / 3;
    const g = (groups[`${solid.id.replace(/^s\d+-[a-z]+-/u, "").replace(/\d+/gu, "#")}:${solid.kind}${solid.tags?.includes("glyph") ? "/glyph" : ""}${solid.tags?.includes("flow") ? "/flow" : ""}`] ??= { solids: 0, triangles: 0 });
    g.solids += 1;
    g.triangles += n;
    total += n;
    count += 1;
  }
  (totals[mode] ??= {})[scale] = total;
  if (wanted.length > 0 && !wanted.includes(scene.id)) continue;
  console.log(`${scene.id} ${total} triangles, ${count} solids, quality ${JSON.stringify(quality)}`);
  for (const [key, g] of Object.entries(groups).sort((a, b) => b[1].triangles - a[1].triangles).slice(0, top)) console.log(`  ${g.triangles} = ${g.solids} x ${Math.round(g.triangles / g.solids)}  ${key}`);
}
let bad = 0;
for (const [mode, row] of Object.entries(totals)) {
  const scales = Object.keys(row).map(Number).sort((a, b) => a - b);
  console.log(`${asset.id} ${mode}: ${scales.map((s) => `1:${s} ${row[s]}`).join(", ")}`);
  for (let i = 1; i < scales.length; i++) if (row[scales[i]!]! > row[scales[i - 1]!]!) { bad++; console.log(`  ! ${mode} triangles grow from 1:${scales[i - 1]} (${row[scales[i - 1]!]}) to 1:${scales[i]} (${row[scales[i]!]})`); }
  if (row[scales[0]!]! > 400_000) { bad++; console.log(`  ! ${row[scales[0]!]} ${mode} triangles at 1:${scales[0]} (budget 400000)`); }
}
console.log(`BUDGET ${asset.id}: ${bad === 0 ? "OK" : `${bad} findings`}`);
