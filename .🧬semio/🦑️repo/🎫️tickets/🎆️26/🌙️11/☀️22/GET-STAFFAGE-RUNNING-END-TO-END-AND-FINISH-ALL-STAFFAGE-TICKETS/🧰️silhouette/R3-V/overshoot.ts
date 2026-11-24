/** 📏️ How far the centre line the guard finds in slender capsules runs past their end points, in 3D: per solid the length of the compared stroke (from its quads in the three views) against the length between the capsule centres, and the summary: bun overshoot.ts <id|file.json> <scale> [id-substring] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { strokesOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./people.ts";
const [name, sc, sub] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
const byId = new Map((asset.volumes[sc!].body as any[]).map((s) => [s.id, s]));
const out: number[] = [];
for (const w of worldSolids(asset, state.bones as never, Number(sc)) as any[]) {
  const s = byId.get(w.id);
  if (w.slender !== true || s?.kind !== "capsule" || !w.id.includes(sub ?? "")) continue;
  const mid = (view: string) => { const q = (strokesOf(view as never, w, undefined) as { x: number; y: number }[][])[0]!; return [{ x: (q[0]!.x + q[3]!.x) / 2, y: (q[0]!.y + q[3]!.y) / 2 }, { x: (q[1]!.x + q[2]!.x) / 2, y: (q[1]!.y + q[2]!.y) / 2 }]; };
  const [e, p] = [mid("elevation"), mid("plan")];
  const len2 = Math.max(Math.hypot(e[1]!.x - e[0]!.x, e[1]!.y - e[0]!.y) ** 2, 0) + (Math.abs(p[1]!.y - p[0]!.y)) ** 2;
  const true3 = Math.hypot(s.p1[0] - s.p0[0], s.p1[1] - s.p0[1], s.p1[2] - s.p0[2]);
  const over = (Math.sqrt(len2) - true3) / 2;
  out.push(over);
  if (process.env.ALL !== undefined) console.log(`${w.id} length ${true3.toFixed(4)} stroke ${Math.sqrt(len2).toFixed(4)} past each end ${(over * 1000).toFixed(1)} mm`);
}
out.sort((a, b) => a - b);
console.log(`${out.length} capsules: past each end min ${(out[0]! * 1000).toFixed(1)} median ${(out[Math.floor(out.length / 2)]! * 1000).toFixed(1)} max ${(out[out.length - 1]! * 1000).toFixed(1)} mm`);
