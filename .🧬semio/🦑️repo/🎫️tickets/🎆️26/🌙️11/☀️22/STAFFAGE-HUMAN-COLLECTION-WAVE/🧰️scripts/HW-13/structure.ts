/** 🏗️ Structural residuals and reactions of an HW-13 asset over its poses and clip samples (the check CLI's equilibrium test).
 *  bun structure.ts <slug> [samples=9] */
import { evaluate, solveStructure } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const [slug, n] = process.argv.slice(2);
const a = (await Bun.file(`/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/${slug}/🔣️.json`).json()).asset;
const queries: any[] = [...a.poses.map((p: any) => ({ kind: "pose", pose: p.id }))];
for (const c of a.clips) for (let i = 0; i <= Number(n ?? 9); i++) queries.push({ kind: "clip", clip: c.id, timeS: (c.durationS * i) / Number(n ?? 9) });
for (const q of queries) {
  const s: any = solveStructure(evaluate(a, q));
  const r = s.residual;
  const bad = Math.hypot(r.fxN, r.fyN) > s.totalMassKg * 9.81e-6;
  console.log(`${bad ? "✗" : "✓"} ${q.pose ?? `${q.clip}@${q.timeS.toFixed(2)}`} residual ${r.fxN.toFixed(2)} ${r.fyN.toFixed(2)} | ${s.reactions.filter((x: any) => x.active).map((x: any) => `${x.anchor ?? x.id}:${(x.fxN ?? 0).toFixed(0)},${(x.fyN ?? 0).toFixed(0)}`).join(" ")}${bad ? " | " + s.notes.map((x: any) => x.en).join(" / ").slice(0, 300) : ""}`);
}
