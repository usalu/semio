import { readFileSync, mkdirSync, writeFileSync } from "node:fs";
import sharp from "../../../../../../../../node_modules/sharp/lib/index.js";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { runPreview } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
import { checkAsset } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";

const root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery";
const out = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/M2";
const plan: Record<string, { scales: number[]; only: string[] }> = {
  "scroll-compressor": { scales: [], only: [] },
  "solar-thermal-collector": { scales: [], only: [] },
  "plate-heat-exchanger": { scales: [50, 200], only: ["partial", "bypass"] },
  "scissor-lift": { scales: [50, 200], only: ["mid-raise", "raised-working", "raise-cycle"] },
  "wheel-loader": { scales: [100, 500], only: ["dump-high", "raising", "load-cycle"] },
  "oil-pumpjack": { scales: [50, 200], only: ["bottom-of-stroke", "pump-cycle"] },
  "tower-crane": { scales: [100, 500], only: ["max-reach", "skip-at-ground", "lift-and-traverse"] },
  "sorption-wheel": { scales: [50, 200], only: ["rated", "spin"] },
};
const sel = (s: number) => ({ kind: "scale", scale: { denominator: s } }) as const;
type Box = { x0: number; x1: number; y0: number; y1: number };
function box(shapes: any[]): Box | null {
  let b: Box | null = null;
  for (const s of shapes) {
    if (s.shape.role === "ground") continue;
    for (const ring of s.rings) for (const p of ring) {
      if (!Number.isFinite(p.x) || !Number.isFinite(p.y)) return { x0: NaN, x1: NaN, y0: NaN, y1: NaN };
      b = b ? { x0: Math.min(b.x0, p.x), x1: Math.max(b.x1, p.x), y0: Math.min(b.y0, p.y), y1: Math.max(b.y1, p.y) } : { x0: p.x, x1: p.x, y0: p.y, y1: p.y };
    }
  }
  return b;
}
const f = (v: number) => v.toFixed(3);
const fmt = (b: Box | null) => (b ? `x[${f(b.x0)},${f(b.x1)}] y[${f(b.y0)},${f(b.y1)}]` : "EMPTY");
const lines: string[] = [];
const log = (t: string) => { lines.push(t); console.log(t); };
for (const [id, cfg] of Object.entries(plan)) {
  try {
    const file = `${root}/${id}/🔣️.json`;
    const asset = JSON.parse(readFileSync(file, "utf8")).asset;
    const authored = ["50", "100", "200", "500"].filter((s) => asset.representations[s]?.status === "authored").map(Number);
    log(`=== ${id} authored ${authored.join(",")}`);
    const queries: any[] = [];
    for (const p of asset.poses) queries.push({ kind: "pose", pose: p.id });
    for (const c of asset.clips) for (const k of [0, 0.2, 0.4, 0.6, 0.8, 0.999]) queries.push({ kind: "clip", clip: c.id, k, timeS: Math.round(k * c.durationS * 1000) / 1000 });
    const dims = asset.dimensions;
    for (const s of authored) {
      const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, sel(s));
      const e = box(st.shapes.elevation), fr = box(st.shapes.front), pl = box(st.shapes.plan);
      const tol = Math.max(0.02, 0.03 * Math.max(dims.widthM, dims.depthM, dims.heightM));
      const bad: string[] = [];
      const d = (a: number, b: number, t: string) => { if (Math.abs(a - b) > tol) bad.push(`${t} ${f(a)} vs ${f(b)}`); };
      if (e && fr && pl) {
        d(e.x0, pl.x0, "elevX0/planX0"); d(e.x1, pl.x1, "elevX1/planX1"); d(e.y0, fr.y0, "y0"); d(e.y1, fr.y1, "y1"); d(fr.x0, pl.y0, "frontX0/planY0"); d(fr.x1, pl.y1, "frontX1/planY1");
        d(e.x1 - e.x0, dims.widthM, "width"); d(fr.x1 - fr.x0, dims.depthM, "depth"); d(e.y1 - e.y0, dims.heightM, "height");
      } else bad.push("EMPTY view");
      log(`1:${s} tol=${f(tol)} ${bad.length ? "MISMATCH " + bad.join("; ") : "consistent"} | E ${fmt(e)} F ${fmt(fr)} P ${fmt(pl)}`);
      for (const q of queries) {
        try {
          const t = evaluate(asset, q, sel(s));
          const bb = (["elevation", "front", "plan"] as const).map((v) => box(t.shapes[v]));
          if (bb.some((b) => !b || !Number.isFinite(b.x0))) log(`  EMPTY/NONFINITE 1:${s} ${q.pose ?? q.clip + "@" + q.timeS}`);
        } catch (err) { log(`  THROW 1:${s} ${q.pose ?? q.clip + "@" + q.timeS}: ${(err as Error).message}`); }
      }
    }
    try { for (const l of checkAsset(asset, 6)) log("check: " + l.slice(0, 400)); } catch (err) { log(`check threw ${(err as Error).message}`); }
    for (const s of cfg.scales) for (const q of queries) {
      const name = q.kind === "pose" ? q.pose : q.clip + "@" + q.timeS;
      if (q.kind === "pose" ? !cfg.only.includes(q.pose) : !(cfg.only.includes(q.clip) && q.k === 0.4)) continue;
      const dir = `${out}/${id}/${s}`;
      mkdirSync(dir, { recursive: true });
      const argv = ["--file", file, "--scale", String(s), "--views", "--no-png", "--out", dir];
      if (q.kind === "pose") argv.push("--pose", q.pose); else argv.push("--clip", q.clip, "--time", String(q.timeS));
      for (const svg of await runPreview(argv)) await sharp(svg, { density: 110 }).png().toFile(svg.replace(/\.svg$/, ".png"));
    }
  } catch (err) { log(`ASSET FAILED ${id}: ${(err as Error).stack?.split("\n").slice(0, 4).join(" | ")}`); }
}
writeFileSync(`${out}/all-result.txt`, lines.join("\n"));
