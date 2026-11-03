import { readFileSync, writeFileSync, mkdirSync, readdirSync, existsSync } from "node:fs";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const [, , outFile, ...ids] = process.argv;
const catalogDirs = ["👤️humans"];
const find = (id: string): string => {
  const cat = readdirSync(`${S}`).find((n) => n.includes("catalog"))!;
  for (const c of readdirSync(`${S}/${cat}`)) { const d = `${S}/${cat}/${c}/${id}`; if (existsSync(d)) return `${d}/${readdirSync(d).find((n) => n.endsWith(".json"))}`; }
  throw new Error(id);
};
const round = (v: number) => Math.round(v * 1e4) / 1e4;
const VIEWS = ["elevation", "front", "plan"] as const;
const result: Record<string, unknown> = {};
for (const id of ids) {
  const asset = JSON.parse(readFileSync(find(id), "utf8")).asset;
  const out: Record<string, unknown> = { dimensions: asset.dimensions, defaultPose: asset.presentation.defaultPose };
  const rest = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose });
  out.bones = Object.fromEntries(Object.entries(rest.bones).map(([k, b]: [string, any]) => [k, { o: [round(b.origin.x), round(b.origin.y)], t: [round(b.tip.x), round(b.tip.y)] }]));
  const queries: any[] = [
    ...asset.poses.map((p: any) => ({ kind: "pose", pose: p.id })),
    ...asset.clips.flatMap((c: any) => [0, 0.25, 0.5, 0.75].map((f) => ({ kind: "clip", clip: c.id, timeS: c.durationS * f }))),
  ];
  const scales: Record<string, unknown> = {};
  for (const [key, rep] of Object.entries<any>(asset.representations)) {
    if (rep.status !== "authored") continue;
    const per: Record<string, unknown> = {};
    const bounds = (state: any, v: string, ground = false) => {
      let x0 = 1e9, x1 = -1e9, y0 = 1e9, y1 = -1e9;
      for (const s of state.shapes[v]) {
        if (s.shape.role === "ground") continue;
        for (const ring of s.rings) for (const p of ring) { x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); }
      }
      return [round(x0), round(x1), round(y0), round(y1)];
    };
    const d = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(key) } });
    for (const v of VIEWS) per[v] = bounds(d, v);
    const all: Record<string, unknown> = {};
    for (const q of queries) {
      const label = q.kind === "pose" ? q.pose : `${q.clip}@${q.timeS.toFixed(2)}`;
      const st = evaluate(asset, q, { kind: "scale", scale: { denominator: Number(key) } });
      all[label] = Object.fromEntries(VIEWS.map((v) => [v, bounds(st, v)]));
    }
    per.all = all;
    scales[key] = per;
  }
  out.scales = scales;
  result[id] = out;
}
mkdirSync(outFile.replace(/[\/][^\/]*$/, ""), { recursive: true });
writeFileSync(outFile, JSON.stringify(result, null, 1));
console.log("[DEBUG] probe written", outFile);
