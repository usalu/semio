/** 🔬️ Measures the 3D body of one bone in the bone's own frame, in lateral bands: `bun tv1-local.ts <id>|--file <json> <scale> <bone> [bands] [id prefix]` prints, per band of |w| (half width), the range of u (along the bone) and of v (its normal) of the solids of the bone (only those whose id starts with the prefix), the numbers a hand-written extruded 2D solid needs. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const staffage = join(resolve(import.meta.dir, "../../../../../../../.."), "🎓️teaching/🏛️architecture/🧍️staffage");
const { catalog } = await import(join(staffage, "🧪️tests/🧊️three-views/🧰️.ts"));
const { evaluate } = await import(join(staffage, "🔨️modules/🧮️evaluation/🟦️.ts"));
const { worldSolids } = await import(join(staffage, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const args = process.argv.slice(2);
const asset = args[0] === "--file" ? JSON.parse(readFileSync(resolve(args[1]!), "utf8")).asset : catalog().find((candidate: { id: string }) => candidate.id === args[0]);
const [scale, bone, bands = "6", prefix = ""] = args.slice(args[0] === "--file" ? 2 : 1);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } });
const { o, u, v, w } = state.bones[bone!].frame as Record<string, { x: number; y: number; z: number }>;
const solids = (worldSolids(asset, state.bones, Number(scale)) as { id: string; bone?: string; positions: Float64Array; texture?: boolean }[]).filter((s) => s.bone === bone && s.texture !== true && s.id.startsWith(prefix));
const local: [number, number, number][] = [];
for (const s of solids) for (let i = 0; i < s.positions.length; i += 3) {
  const d = [s.positions[i]! - o!.x, s.positions[i + 1]! - o!.y, s.positions[i + 2]! - o!.z];
  local.push([d[0]! * u!.x + d[1]! * u!.y + d[2]! * u!.z, d[0]! * v!.x + d[1]! * v!.y + d[2]! * v!.z, d[0]! * w!.x + d[1]! * w!.y + d[2]! * w!.z]);
}
const half = Math.max(...local.map((p) => Math.abs(p[2])));
console.log(`${asset.id} 1:${scale} ${bone} (${solids.map((s) => s.id).join(", ")}): half width ${half.toFixed(4)}, w [${Math.min(...local.map((p) => p[2])).toFixed(4)}, ${Math.max(...local.map((p) => p[2])).toFixed(4)}]`);
for (let b = 0; b < Number(bands); b++) {
  const lo = (half * b) / Number(bands);
  const hi = (half * (b + 1)) / Number(bands);
  const inside = local.filter((p) => Math.abs(p[2]) >= lo && Math.abs(p[2]) <= hi + 1e-9);
  if (inside.length === 0) continue;
  const r = (k: number): string => `[${Math.min(...inside.map((p) => p[k]!)).toFixed(4)}, ${Math.max(...inside.map((p) => p[k]!)).toFixed(4)}]`;
  console.log(`  |w| ${lo.toFixed(3)}..${hi.toFixed(3)}: u ${r(0)} v ${r(1)} (${inside.length} vertices)`);
}
