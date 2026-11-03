import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const ROOT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans";
// usage: render.ts <asset> <scale> <outsvg> <q1> <q2> ...   q = pose:id | clip:id@t | clipN:id@count (count evenly spaced instants)
const [, , id, scaleArg, out, ...qs] = process.argv;
const asset = JSON.parse(readFileSync(`${ROOT}/${id}/🔣️.json`, "utf8")).asset;
const scale = Number(scaleArg);
const sel = { kind: "scale", scale: { denominator: scale } } as any;
const queries: { label: string; q: any }[] = [];
for (const s of qs) {
  const [kind, rest] = s.split(":");
  if (kind === "pose") queries.push({ label: rest!, q: { kind: "pose", pose: rest } });
  else if (kind === "clip") { const [c, t] = rest!.split("@"); queries.push({ label: `${c}@${t}`, q: { kind: "clip", clip: c, timeS: Number(t) } }); }
  else if (kind === "clipN") {
    const [c, n] = rest!.split("@"); const clip = asset.clips.find((x: any) => x.id === c);
    for (let i = 0; i < Number(n); i++) { const t = (clip.durationS * i) / Number(n); queries.push({ label: `${c}@${t.toFixed(2)}`, q: { kind: "clip", clip: c, timeS: t } }); }
  }
}
type P = { x: number; y: number };
const states = queries.map((x) => ({ ...x, st: evaluate(asset, x.q, sel) }));
const views = ["elevation", "front", "plan"] as const;
const b: Record<string, { x0: number; x1: number; y0: number; y1: number }> = {};
for (const v of views) {
  const e = { x0: 1e9, x1: -1e9, y0: 1e9, y1: -1e9 };
  for (const s of states) for (const sh of s.st.shapes[v]) { if (sh.shape.role === "ground") continue; for (const r of sh.rings) for (const p of r as P[]) { e.x0 = Math.min(e.x0, p.x); e.x1 = Math.max(e.x1, p.x); e.y0 = Math.min(e.y0, p.y); e.y1 = Math.max(e.y1, p.y); } }
  b[v] = e;
}
// common vertical range for elevation and front; plan has its own
const PPM = Number(process.env.PPM ?? (scale >= 100 ? 330 : 250));
const m = 0.08;
const hue = (s: string) => { let h = 0; for (const c of s) h = (h * 31 + c.charCodeAt(0)) % 360; return h; };
const hsl = (h: number, sa: number, l: number) => { const a = (sa / 100) * Math.min(l / 100, 1 - l / 100); const f = (n: number) => { const k = (n + h / 30) % 12; const c = l / 100 - a * Math.max(-1, Math.min(k - 3, 9 - k, 1)); return Math.round(255 * c).toString(16).padStart(2, "0"); }; return "#" + f(0) + f(8) + f(4); };
let svg = "";
let y = 10;
const cellW = (v: string) => Math.max(40, (b[v]!.x1 - b[v]!.x0 + 2 * m) * PPM);
const cellH = (v: string) => Math.max(40, (b[v]!.y1 - b[v]!.y0 + 2 * m) * PPM);
const rowH = Math.max(...views.map(cellH));
for (const s of states) {
  let x = 10;
  svg += `<text x="${x}" y="${y + 12}" font-size="13" font-family="monospace">${s.label}  1:${scale}</text>`;
  for (const v of views) {
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
  y += rowH + 30;
}
const W = 10 + views.reduce((a, v) => a + cellW(v) + 12, 0);
mkdirSync(out.replace(/[\\/][^\\/]*$/, ""), { recursive: true });
writeFileSync(out, `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${y}" viewBox="0 0 ${W} ${y}"><rect width="100%" height="100%" fill="#fff"/>${svg}</svg>`);
console.log("wrote", out, W, y);
