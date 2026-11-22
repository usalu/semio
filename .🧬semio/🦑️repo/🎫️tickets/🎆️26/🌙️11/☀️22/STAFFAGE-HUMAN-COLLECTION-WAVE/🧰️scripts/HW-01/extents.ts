/** 📏️ Per-bone extents of each view of one pose at one scale: elevation x/y, front x(=z)/y, plan x/y(=z).
 *  bun extents.ts <asset.json> <pose> <scale> [bone ...] */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const [path, pose, scale, ...only] = process.argv.slice(2);
const asset = (await Bun.file(path!).json()).asset;
const state = evaluate(asset, { kind: "pose", pose: pose! }, { kind: "scale", scale: { denominator: Number(scale) } } as never);
const f = (n: number) => n.toFixed(3);
for (const vp of ["elevation", "front", "plan"] as const) {
  const by = new Map<string, number[]>();
  for (const s of state.shapes[vp]) {
    if (only.length && !only.includes(s.shape.bone)) continue;
    const e = by.get(s.shape.bone + ":" + s.shape.id) ?? [Infinity, -Infinity, Infinity, -Infinity];
    for (const r of s.rings) for (const p of r) { e[0] = Math.min(e[0]!, p.x); e[1] = Math.max(e[1]!, p.x); e[2] = Math.min(e[2]!, p.y); e[3] = Math.max(e[3]!, p.y); }
    by.set(s.shape.bone + ":" + s.shape.id, e);
  }
  for (const [k, e] of by) console.log(`${vp.padEnd(9)} ${k.padEnd(36)} x ${f(e[0]!)}..${f(e[1]!)} y ${f(e[2]!)}..${f(e[3]!)}`);
}
