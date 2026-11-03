import { readFileSync, mkdirSync } from "node:fs";
import sharp from "../../../../../../../../node_modules/sharp/lib/index.js";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { runPreview } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery";
const out = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/M2";
const [id, mode, ...rest] = process.argv.slice(2);
const file = `${root}/${id}/🔣️.json`;
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const scales = (rest.find((a) => a.startsWith("scales="))?.slice(7) ?? "50,100,200,500").split(",").map(Number);
const authored = scales.filter((s) => asset.representations[String(s)]?.status === "authored");
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
const queries: any[] = [];
for (const p of asset.poses) queries.push({ kind: "pose", pose: p.id });
for (const c of asset.clips) for (const k of [0, 0.25, 0.5, 0.75, 1]) queries.push({ kind: "clip", clip: c.id, timeS: Math.round(k * c.durationS * 1000) / 1000 });
if (mode === "check") {
  const dims = asset.dimensions;
  for (const s of authored) {
    const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, sel(s));
    const e = box(st.shapes.elevation), fr = box(st.shapes.front), pl = box(st.shapes.plan);
    const tol = Math.max(0.02, 0.03 * Math.max(dims.widthM, dims.depthM, dims.heightM));
    const bad: string[] = [];
    const d = (a: number, b: number, t: string) => { if (Math.abs(a - b) > tol) bad.push(`${t} ${f(a)} vs ${f(b)}`); };
    if (e && fr && pl) {
      d(e.x1 - e.x0, pl.x1 - pl.x0, "elevX/planX len"); d(e.x0, pl.x0, "elevX0/planX0"); d(e.x1, pl.x1, "elevX1/planX1");
      d(e.y0, fr.y0, "y0"); d(e.y1, fr.y1, "y1");
      d(fr.x0, pl.y0, "frontX0/planY0"); d(fr.x1, pl.y1, "frontX1/planY1");
      d(e.x1 - e.x0, dims.widthM, "width"); d(fr.x1 - fr.x0, dims.depthM, "depth"); d(e.y1 - e.y0, dims.heightM, "height");
    } else bad.push("EMPTY view");
    console.log(`1:${s} tol=${f(tol)} ${bad.length ? "MISMATCH " + bad.join("; ") : "consistent"}\n  E ${fmt(e)}\n  F ${fmt(fr)}\n  P ${fmt(pl)}`);
    for (const q of queries) {
      const t = evaluate(asset, q, sel(s));
      const bb = (["elevation", "front", "plan"] as const).map((v) => box(t.shapes[v]));
      const flag = bb.some((b) => !b || !Number.isFinite(b.x0)) ? " <<< EMPTY/NONFINITE" : "";
      if (flag || process.argv.includes("verbose")) console.log(`  ${q.kind === "pose" ? q.pose : q.clip + "@" + q.timeS}: E ${fmt(bb[0])} | F ${fmt(bb[1])} | P ${fmt(bb[2])}${flag}`);
    }
  }
} else if (mode === "render") {
  const dir = `${out}/${id}`;
  mkdirSync(dir, { recursive: true });
  const only = rest.find((a) => a.startsWith("only="))?.slice(5).split(",");
  for (const s of authored) for (const q of queries) {
    const name = q.kind === "pose" ? q.pose : q.clip + "@" + q.timeS;
    if (only && !only.some((o) => name.startsWith(o))) continue;
    const dpi = Number(rest.find((a) => a.startsWith("dpi="))?.slice(4) ?? "200");
    const argv = ["--file", file, "--scale", String(s), "--views", "--no-png", "--out", `${dir}/${s}`];
    if (q.kind === "pose") argv.push("--pose", q.pose); else argv.push("--clip", q.clip, "--time", String(q.timeS));
    for (const svg of await runPreview(argv)) await sharp(svg, { density: dpi }).png().toFile(svg.replace(/\.svg$/, ".png"));
  }
  console.log("rendered to", dir);
}
