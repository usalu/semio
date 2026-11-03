import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const ROOT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture";
// usage: render.ts <asset> <scale> <outsvg> <q1> <q2> ...   q = pose:id | clip:id@t | clipN:id@count | allposes | allclips@count
const [, , id, scaleArg, out, ...qs] = process.argv;
const asset = JSON.parse(readFileSync(`${ROOT}/${id}/🔣️.json`, "utf8")).asset;
const scale = Number(scaleArg);
const sel = { kind: "scale", scale: { denominator: scale } } as any;
const queries: { label: string; q: any }[] = [];
const addClip = (c: string, n: number) => {
  const clip = asset.clips.find((x: any) => x.id === c);
  for (let i = 0; i < n; i++) { const t = (clip.durationS * i) / n; queries.push({ label: `${c}@${t.toFixed(2)}`, q: { kind: "clip", clip: c, timeS: t } }); }
};
for (const s of qs) {
  const [kind, rest] = s.split(":");
  if (kind === "pose") queries.push({ label: rest!, q: { kind: "pose", pose: rest } });
  else if (kind === "allposes") for (const p of asset.poses) queries.push({ label: p.id, q: { kind: "pose", pose: p.id } });
  else if (kind === "clip") { const [c, t] = rest!.split("@"); queries.push({ label: `${c}@${t}`, q: { kind: "clip", clip: c, timeS: Number(t) } }); }
  else if (kind === "clipN") { const [c, n] = rest!.split("@"); addClip(c!, Number(n)); }
  else if (kind === "allclips") for (const c of asset.clips) addClip(c.id, Number(rest));
}
type P = { x: number; y: number };
const states = queries.map((x) => ({ ...x, st: evaluate(asset, x.q, sel) }));
const views = ["elevation", "front", "plan"] as const;
const boundsOf = (list: any[]) => {
  const e = { x0: 1e9, x1: -1e9, y0: 1e9, y1: -1e9 };
  for (const sh of list) { if (sh.shape.role === "ground") continue; for (const r of sh.rings) for (const p of r as P[]) { e.x0 = Math.min(e.x0, p.x); e.x1 = Math.max(e.x1, p.x); e.y0 = Math.min(e.y0, p.y); e.y1 = Math.max(e.y1, p.y); } }
  return e;
};
const b: Record<string, { x0: number; x1: number; y0: number; y1: number }> = {};
for (const v of views) {
  const e = { x0: 1e9, x1: -1e9, y0: 1e9, y1: -1e9 };
  for (const s of states) { const q = boundsOf(s.st.shapes[v]); e.x0 = Math.min(e.x0, q.x0); e.x1 = Math.max(e.x1, q.x1); e.y0 = Math.min(e.y0, q.y0); e.y1 = Math.max(e.y1, q.y1); }
  b[v] = e;
}
const PPM = scale >= 100 ? 330 : 250;
const m = 0.08;
const hue = (s: string) => { let h = 0; for (const c of s) h = (h * 31 + c.charCodeAt(0)) % 360; return h; };
const hsl = (h: number, sa: number, l: number) => { const a = (sa / 100) * Math.min(l / 100, 1 - l / 100); const f = (n: number) => { const k = (n + h / 30) % 12; const c = l / 100 - a * Math.max(-1, Math.min(k - 3, 9 - k, 1)); return Math.round(255 * c).toString(16).padStart(2, "0"); }; return "#" + f(0) + f(8) + f(4); };
let svg = "";
let y = 10;
const cellW = (v: string) => Math.max(40, (b[v]!.x1 - b[v]!.x0 + 2 * m) * PPM);
const cellH = (v: string) => Math.max(40, (b[v]!.y1 - b[v]!.y0 + 2 * m) * PPM);
const rowH = Math.max(...views.map(cellH));
const dims = asset.dimensions;
const def = asset.presentation.defaultPose;
const tol = (d: number) => Math.max(0.02, 0.03 * d);
const bad: string[] = [];
for (const s of states) {
  let x = 10;
  svg += `<text x="${x}" y="${y + 12}" font-size="13" font-family="monospace">${s.label}  1:${scale}</text>`;
  const bb: Record<string, any> = {};
  for (const v of views) {
    bb[v] = boundsOf(s.st.shapes[v]);
    if (s.st.shapes[v].length === 0 || !Object.values(bb[v]).every((n) => Number.isFinite(n))) bad.push(`${s.label}: ${v} empty or non-finite`);
    const w = cellW(v), h = cellH(v);
    const ox = x - (b[v]!.x0 - m) * PPM, oy = y + 16 + h + (b[v]!.y0 - m) * PPM;
    svg += `<rect x="${x}" y="${y + 16}" width="${w}" height="${h}" fill="#fff" stroke="#bbb" stroke-width="0.5"/>`;
    svg += `<text x="${x + 3}" y="${y + 28}" font-size="10" fill="#888" font-family="monospace">${v}</text>`;
    svg += `<line x1="${x}" y1="${oy}" x2="${x + w}" y2="${oy}" stroke="#e66" stroke-width="0.4" stroke-dasharray="3 3"/>`;
    svg += `<line x1="${ox}" y1="${y + 16}" x2="${ox}" y2="${y + 16 + h}" stroke="#69e" stroke-width="0.4" stroke-dasharray="3 3"/>`;
    const list = [...s.st.shapes[v]].sort((a, c) => a.shape.z - c.shape.z);
    for (const sh of list) {
      const closed = sh.closed; const d = sh.rings.map((r: P[]) => r.map((p, i) => `${i ? "L" : "M"}${(ox + p.x * PPM).toFixed(1)} ${(oy - p.y * PPM).toFixed(1)}`).join(" ") + (closed ? "Z" : "")).join(" ");
      const key = (sh.shape as any).region ?? ((sh.shape as any).regions?.[0]) ?? sh.shape.id;
      const filled = sh.shape.filled && closed;
      const fill = filled ? hsl(hue(String(key)), 45, sh.shape.role === "detail" ? 55 : 72) : "none";
      svg += `<path d="${d}" fill="${fill}" fill-opacity="0.92" stroke="#222" stroke-width="${sh.shape.role === "detail" ? 0.6 : 1}"><title>${sh.shape.id}</title></path>`;
    }
    x += w + 12;
  }
  if (s.label === def) {
    const f = (n: number) => n.toFixed(3);
    const e = bb.elevation, fr = bb.front, pl = bb.plan;
    const cmp = (name: string, a: number, c: number, t: number) => { if (Math.abs(a - c) > t) bad.push(`DEFAULT ${name}: ${f(a)} vs ${f(c)} (tol ${f(t)})`); };
    const D = Math.max(dims.widthM, dims.depthM, dims.heightM);
    const t = tol(D);
    cmp("elev.x0 vs plan.x0", e.x0, pl.x0, t); cmp("elev.x1 vs plan.x1", e.x1, pl.x1, t);
    cmp("elev.y0 vs front.y0", e.y0, fr.y0, t); cmp("elev.y1 vs front.y1", e.y1, fr.y1, t);
    cmp("front.x0 vs plan.y0", fr.x0, pl.y0, t); cmp("front.x1 vs plan.y1", fr.x1, pl.y1, t);
    cmp("width", e.x1 - e.x0, dims.widthM, t); cmp("height", e.y1 - e.y0, dims.heightM, t); cmp("depth", pl.y1 - pl.y0, dims.depthM, t);
    console.log(`default ${s.label}: elev x[${f(e.x0)},${f(e.x1)}] y[${f(e.y0)},${f(e.y1)}] | front x[${f(fr.x0)},${f(fr.x1)}] y[${f(fr.y0)},${f(fr.y1)}] | plan x[${f(pl.x0)},${f(pl.x1)}] y[${f(pl.y0)},${f(pl.y1)}] dims w${dims.widthM} d${dims.depthM} h${dims.heightM}`);
  }
  y += rowH + 30;
}
const W = 10 + views.reduce((a, v) => a + cellW(v) + 12, 0);
mkdirSync(out.replace(/[\\/][^\\/]*$/, ""), { recursive: true });
writeFileSync(out, `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${y}" viewBox="0 0 ${W} ${y}"><rect width="100%" height="100%" fill="#fff"/>${svg}</svg>`);
console.log("wrote", out, Math.round(W), Math.round(y), states.length, "states");
for (const line of bad) console.log("PROBLEM", line);
