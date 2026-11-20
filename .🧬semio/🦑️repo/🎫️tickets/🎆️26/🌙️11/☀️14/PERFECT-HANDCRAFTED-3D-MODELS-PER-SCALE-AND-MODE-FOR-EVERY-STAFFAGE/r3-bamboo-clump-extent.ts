/** 📏️ Per-culm extents of the round-3 bamboo model at a scale, from the candidate rig: which culm sets the width, depth and height of the clump. Usage: `bun r3-bamboo-clump-extent.ts [scale]` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, STAFFAGE, type Frame, type Json } from "./volumes-lib.ts";
import { tubePolygons, blobOutline, project } from "./r3-date-palm-kit.ts";
import { buildPlant, type Scale } from "./r3-bamboo-clump-plant.ts";

const { solveChain } = (await import(join(STAFFAGE, "🔨️modules/🦴️rig/🟦️.ts"))) as { solveChain: (rig: Json, offsets: Json, root: Json) => Record<string, { frame: Frame }> };
const asset = JSON.parse(readFileSync(join(GENERATED, "r3-bamboo-clump", "🔣️.json"), "utf8")).asset;
const pose = asset.poses.find((p: Json) => p.id === asset.presentation.defaultPose);
const F = Object.fromEntries(Object.entries(solveChain(asset.rig, pose.bones, { x: 0, y: 0, angleDeg: 0 })).map(([k, v]) => [k, v.frame])) as Record<string, Frame>;
const scale = Number(process.argv[2] ?? 50) as Scale;
const P = buildPlant(scale, F, asset.rig);
for (const k of [1, 2, 3, 4, 5, 6]) {
  const parts = P.parts.filter((p) => p.body && (p.bone.startsWith(`culm-${k}-`) || p.bone === `fol-${k}`));
  const el = parts.flatMap((p) => (p.tube ? tubePolygons("elevation", p.tube) : p.blob ? [blobOutline("elevation", p.blob)] : [])).flat();
  const fr = parts.flatMap((p) => (p.tube ? tubePolygons("front", p.tube) : p.blob ? [blobOutline("front", p.blob)] : [])).flat();
  const xs = el.map((p) => p.x), ys = el.map((p) => p.y), zs = fr.map((p) => p.x);
  const tip = P.culms[k - 1]!.at(P.culms[k - 1]!.L);
  console.log(`culm ${k}: elevation-x ${Math.min(...xs).toFixed(3)}..${Math.max(...xs).toFixed(3)} top ${Math.max(...ys).toFixed(3)} front-x ${Math.min(...zs).toFixed(3)}..${Math.max(...zs).toFixed(3)} tip (${tip.map((v) => v.toFixed(3)).join(", ")}) front-x of tip ${project("front", tip).x.toFixed(3)}`);
}
