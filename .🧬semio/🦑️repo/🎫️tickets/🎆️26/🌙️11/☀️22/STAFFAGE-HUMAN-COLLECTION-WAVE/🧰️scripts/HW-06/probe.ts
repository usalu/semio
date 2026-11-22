/** 📏️ Default-pose extents of every authored scale and view, total power and the rig mass of one asset file (for `dimensions`, the manifest row and the report).
 *  bun probe.ts <asset.json> [pose] */
import { evaluate, powerOf, totalPowerW, viewExtent } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const [file, poseArg] = process.argv.slice(2);
const asset = (await Bun.file(file!).json()).asset;
const pose = poseArg ?? asset.presentation.defaultPose;
const rows: Record<string, unknown> = {};
for (const key of ["500", "200", "100", "50"]) {
  if (asset.representations[key].status !== "authored") continue;
  const s = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: Number(key) } });
  const r = (v: number) => Math.round(v * 1000) / 1000;
  rows[key] = { height: r(s.bounds.maxY - s.bounds.minY), width: r(s.bounds.maxX - s.bounds.minX), minX: r(s.bounds.minX), minY: r(s.bounds.minY), front: viewExtent(s, "front"), plan: viewExtent(s, "plan") };
}
const mass = asset.rig.bones.reduce((sum: number, b: { mass?: { kg: number } }) => sum + (b.mass?.kg ?? 0), 0);
console.log(JSON.stringify({ pose, rows, powerW: totalPowerW(asset), power: powerOf(asset), rigMassKg: mass, dimensions: asset.dimensions }, null, 1));
