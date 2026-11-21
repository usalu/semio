/** 📐 Model-against-reference measurements of a human asset at one scale: joint heights of a standing pose (world, metres and shares of the barefoot stature), segment lengths of the rig, and the extents of named body solids in their own bone frames.
 * Usage: `bun r5-measure.ts <io module> <scale> <standing pose> <barefoot stature m> <sole m> [label=solid,solid:axis ...]` (axis u, v or w of the bone frame)
 * @see ./r5-brief.md */
import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";

const [io, scale, pose, stature, sole, ...specs] = process.argv.slice(2);
const { candidate } = await import(io!);
const asset = candidate(true);
const H = Number(stature);
const lift = Number(sole);
const rig = rigAt(asset, Number(scale), { kind: "pose", pose: pose! });
const f3 = (x: number): string => x.toFixed(3);
const row = (label: string, m: number, share = true): void => console.log(`${label.padEnd(28)} ${f3(m)} m${share ? `  ${(m / H).toFixed(3)} H` : ""}`);
const at = (id: string): number[] => rig.frames[id]!.o as number[];
const bone = (re: RegExp): string | undefined => rig.bones.map((b) => b.id as string).find((id) => re.test(id));
for (const [label, re] of [["hip joint", /^thigh-l$/], ["knee", /^shank-l$/], ["ankle", /^foot-l$/], ["shoulder joint", /^(upper-arm|arm)-l$/], ["elbow", /^forearm-l$/], ["wrist", /^hand-l$/]] as const) {
  const id = bone(re);
  if (id) row(`${label} height (barefoot)`, at(id)[1]! - lift);
}
const head = rig.bones.find((b) => b.id === "head")!;
row("vertex (head bone tip, barefoot)", head.tip[1] - lift);
const arm = bone(/^(upper-arm|arm)-l$/)!;
row("shoulder joint spacing", Math.abs(at(arm)[2]! - at(arm.replace("-l", "-r"))[2]!));
row("hip joint spacing", Math.abs(at("thigh-l")[2]! - at("thigh-r")[2]!));
for (const b of asset.rig.bones) if (/^(torso|neck|head|upper-arm-l|arm-l|forearm-l|hand-l|thigh-l|shank-l)$/.test(b.id)) row(`bone ${b.id}`, b.lengthM);
const IDENT: Record<string, Frame> = Object.fromEntries((asset.rig.bones as any[]).map((b) => [b.id, { o: [0, 0, 0], u: [1, 0, 0], v: [0, 1, 0], w: [0, 0, 1] } as Frame]));
const body = asset.volumes[scale!].body as any[];
for (const spec of specs) {
  const [label, rest] = spec.split("=");
  const [ids, axis] = rest!.split(":");
  const k = { u: 0, v: 1, w: 2 }[axis as "u" | "v" | "w"];
  const solids = body.filter((s) => ids!.split(",").some((p) => (p.endsWith("*") ? s.id.startsWith(p.slice(0, -1)) : s.id === p)));
  let lo = Infinity, hi = -Infinity;
  for (const m of meshesIn(asset, IDENT, solids)) for (let i = k; i < m.world.length; i += 3) { lo = Math.min(lo, m.world[i]!); hi = Math.max(hi, m.world[i]!); }
  row(`${label} (${axis})`, hi - lo);
}
