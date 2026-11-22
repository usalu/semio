/** 🏗️ Structural residual, active reactions with their friction use and the centre of mass of an asset document over its poses and clip samples, or at the given instants.
 *  bun struct.ts <slug|file.json> [samples=8] [clip@t ...] */
import { join, resolve } from "node:path";
import { evaluate, solveStructure } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const [arg, n, ...at] = process.argv.slice(2);
const a = (await Bun.file(arg!.endsWith(".json") ? arg! : join(catalog, arg!, "🔣️.json")).json()).asset;
const samples = Number(n ?? 8);
const queries: any[] = at.length > 0 ? at.map((s) => ({ kind: "clip", clip: s.split("@")[0], timeS: Number(s.split("@")[1]) })) : [...a.poses.filter((p: any) => !p.key).map((p: any) => ({ kind: "pose", pose: p.id }))];
if (at.length === 0) for (const c of a.clips) for (let i = 0; i <= samples; i++) queries.push({ kind: "clip", clip: c.id, timeS: (c.durationS * i) / samples });
for (const q of queries) {
  const s: any = solveStructure(evaluate(a, q));
  const r = s.residual;
  const bad = Math.hypot(r.fxN, r.fyN) > s.totalMassKg * 9.81e-6;
  const reactions = s.reactions.filter((x: any) => x.active).map((x: any) => `${x.anchor ?? x.id}:${(x.fxN ?? 0).toFixed(1)},${(x.fyN ?? 0).toFixed(1)},${(x.fzN ?? 0).toFixed(1)}`).join(" ");
  const inertia = s.inertialForces.reduce((t: number[], f: any) => [t[0]! + f.fxN, t[1]! + f.fyN, t[2]! + (f.fzN ?? 0)], [0, 0, 0]).map((v: number) => v.toFixed(1)).join(",");
  console.log(`${bad ? "✗" : "✓"} ${q.pose ?? `${q.clip}@${q.timeS.toFixed(3)}`} residual ${r.fxN.toFixed(2)},${r.fyN.toFixed(2)},${r.fzN.toFixed(2)} inertia ${inertia} | ${reactions}`);
}
