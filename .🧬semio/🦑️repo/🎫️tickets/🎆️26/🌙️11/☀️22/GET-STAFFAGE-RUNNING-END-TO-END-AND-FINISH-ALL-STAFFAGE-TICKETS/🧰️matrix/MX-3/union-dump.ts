/** 📥️ The convex bodies that `loft3` unifies for every unweighted `loft` shape of a document (default pose, every authored scale and view), with the area of the engine's outline: union-dump.ts <json> <out json> */
import { readFileSync, writeFileSync } from "node:fs";
import { evaluate, loftRings } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🔭️views/🟦️.ts";
type P = { x: number; y: number };
const area = (r: readonly P[]): number => r.reduce((s, p, k) => s + p.x * r[(k + 1) % r.length]!.y - r[(k + 1) % r.length]!.x * p.y, 0) / 2;
function hull(points: P[]): P[] {
  const s = [...points].sort((a, b) => a.x - b.x || a.y - b.y);
  const turn = (o: P, a: P, b: P) => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const build = (input: P[]) => { const out: P[] = []; for (const p of input) { while (out.length >= 2 && turn(out.at(-2)!, out.at(-1)!, p) <= 0) out.pop(); out.push(p); } out.pop(); return out; };
  return [...build(s), ...build([...s].reverse())];
}
const [file, out] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const cases: unknown[] = [];
for (const scale of (process.env.SCALES ?? "50,100,200,500").split(",").map(Number)) for (const pose of (process.env.POSES === "all" ? asset.poses.map((q: any) => q.id) : [asset.presentation.defaultPose]) as string[]) {
  if (asset.representations[String(scale)]?.status !== "authored") continue;
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } });
  for (const view of ["elevation", "front", "plan"] as const) for (const e of state.shapes[view]) {
    const shape = e.shape as any;
    if (shape.kind !== "loft" || shape.weights?.length) continue;
    const f = state.bones[shape.bone]!.frame;
    const rings = loftRings(shape).map((ring) => ring.map((p) => project(view, { x: f.o.x + p[0] * f.u.x + p[1] * f.v.x + p[2] * f.w.x, y: f.o.y + p[0] * f.u.y + p[1] * f.v.y + p[2] * f.w.y, z: f.o.z + p[0] * f.u.z + p[1] * f.v.z + p[2] * f.w.z } as never)));
    const bodies: P[][] = [];
    for (let i = 0; i + 1 < rings.length; i++) { const b = hull([...rings[i]!, ...rings[i + 1]!]); if (b.length >= 3 && Math.abs(area(b)) > 1e-12) bodies.push(b); }
    cases.push({ scale, view: view + " " + pose, id: shape.id, bone: shape.bone, engineArea: e.rings.reduce((s, r) => s + area(r), 0), engineBox: [Math.min(...e.rings.flat().map((p) => p.x)), Math.min(...e.rings.flat().map((p) => p.y)), Math.max(...e.rings.flat().map((p) => p.x)), Math.max(...e.rings.flat().map((p) => p.y))], bodies: bodies.map((b) => b.map((p) => [p.x, p.y])) });
  }
}
writeFileSync(out!, JSON.stringify(cases));
console.log(cases.length, "lofts dumped");
