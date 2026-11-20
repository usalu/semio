/** ⛳️ The lowest point and the highest point of the 1:50 body of the weeping willow candidate in every pose and every quarter second of every clip (the hem must stay above the ground in every frame).
 * Usage: bun r3-weeping-willow-ground.ts [scale] */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED } from "./volumes-lib.ts";
import { listsIn, rigAt, type Query } from "./r3-lib.ts";

const scale = Number(process.argv[2] ?? 50);
const asset = JSON.parse(readFileSync(join(GENERATED, "r3-weeping-willow", "🔣️.json"), "utf8")).asset;
const queries: [string, Query][] = [...asset.poses.map((p: { id: string }) => [p.id, { kind: "pose", pose: p.id }] as [string, Query])];
for (const c of asset.clips) for (let t = 0; t <= c.durationS + 1e-9; t += 0.25) queries.push([`${c.id}@${t.toFixed(2)}`, { kind: "clip", clip: c.id, timeS: t }]);
let worst = { name: "", y: Infinity };
for (const [name, q] of queries) {
  const rig = rigAt(asset, scale, q);
  const lists = listsIn(asset, scale, rig.frames);
  let lo = Infinity, who = "";
  for (const m of lists.body) for (let i = 1; i < m.world.length; i += 3) if (m.world[i]! < lo) { lo = m.world[i]!; who = m.id; }
  if (lo < worst.y) worst = { name: `${name} ${who}`, y: lo };
  if (!name.includes("@") || lo < 0.05) console.log(name.padEnd(28), "lowest", lo.toFixed(3), who);
}
console.log("worst", worst.name, worst.y.toFixed(3));
