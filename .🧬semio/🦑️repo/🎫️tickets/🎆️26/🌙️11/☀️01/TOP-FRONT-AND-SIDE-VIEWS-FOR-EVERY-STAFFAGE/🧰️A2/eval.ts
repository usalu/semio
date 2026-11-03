import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const [file, scaleArg, out, samplesArg = "5"] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const scale = Number(scaleArg);
const samples = Number(samplesArg);
const selector = { kind: "scale", scale: { denominator: scale } } as const;
const queries: { label: string; query: any }[] = [];
for (const pose of asset.poses) queries.push({ label: pose.id, query: { kind: "pose", pose: pose.id } });
for (const clip of asset.clips) for (let i = 0; i < samples; i++) queries.push({ label: `${clip.id}@${((clip.durationS * i) / samples).toFixed(2)}`, query: { kind: "clip", clip: clip.id, timeS: (clip.durationS * i) / samples } });
const num = (v: number) => Math.round(v * 1e4) / 1e4;
const result: any[] = [];
for (const { label, query } of queries) {
  try {
    const st: any = evaluate(asset, query, selector as any);
    const views: any = {};
    for (const vp of ["elevation", "front", "plan"]) {
      views[vp] = (st.shapes[vp] ?? []).map((s: any) => ({ id: s.shape.id, bone: s.shape.bone, role: s.shape.role, z: s.shape.z, filled: s.shape.filled, tone: s.shape.tone, closed: s.closed, rings: s.rings.map((r: any[]) => r.map((p) => [num(p.x), num(p.y)])) }));
    }
    const bones: any = {};
    for (const [id, b] of Object.entries<any>(st.bones)) bones[id] = [num(b.origin.x), num(b.origin.y), num(b.tip.x), num(b.tip.y)];
    result.push({ label, bones, views });
  } catch (e: any) {
    result.push({ label, error: String(e?.message ?? e) });
  }
}
writeFileSync(out, JSON.stringify(result));
console.log("wrote", out, result.length);
