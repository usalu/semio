// [DEBUG] scratch sketch renderer for humans-a: bun humans-a-sketch.ts <file> <asset> <out.svg> [pose:<id>|clip:<id>:<n>] ...
import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [file, id, out, ...specs] = process.argv.slice(2);
const doc = JSON.parse(readFileSync(file!, "utf8"));
const asset = (doc.assets ?? [doc]).find((a: any) => a.id === id);
const queries: any[] = [];
for (const s of specs) {
  const [k, n, c] = s.split(":");
  if (k === "pose") queries.push({ kind: "pose", pose: n });
  else if (k === "allposes") asset.poses.forEach((p: any) => queries.push({ kind: "pose", pose: p.id }));
  else { const clip = asset.clips.find((x: any) => x.id === n); const N = Number(c); for (let i = 0; i < N; i++) queries.push({ kind: "clip", clip: n, timeS: (clip.durationS * i) / N }); }
}
const S = 300; // px per metre
const pad = 0.4;
let cells = ""; let xOff = 0; const H = 2.3;
const info: string[] = [];
for (const q of queries) {
  const st = evaluate(asset, q);
  const w = Math.max(1.4, st.bounds.maxX - st.bounds.minX + 2 * pad);
  const cx = (st.bounds.minX + st.bounds.maxX) / 2;
  const tx = (x: number) => ((x - cx + w / 2) + xOff) * S;
  const ty = (y: number) => (H - y) * S;
  let g = "";
  const shapes = [...st.shapes.elevation].sort((a, b) => a.shape.z - b.shape.z);
  for (const e of shapes) {
    const sh = e.shape; if (sh.role === "ground") continue;
    const d = e.rings.map((r) => r.map((p, i) => `${i ? "L" : "M"}${tx(p.x).toFixed(1)} ${ty(p.y).toFixed(1)}`).join("") + (e.closed ? "Z" : "")).join(" ");
    const gray = Math.round(255 * sh.tone);
    const fill = sh.filled ? (sh.role === "glazing" ? "rgb(170,210,235)" : `rgb(${gray},${gray},${gray})`) : "none";
    const sw = sh.role === "body" ? 1.6 : sh.role === "texture" ? 0.5 : 0.9;
    g += `<path d="${d}" fill="${fill}" fill-opacity="${sh.role === "glazing" ? 0.6 : 1}" stroke="#111" stroke-width="${sw}" stroke-linejoin="round"/>`;
  }
  for (const [aid, a] of Object.entries(st.anchors) as any) {
    const isC = st.contacts.includes(aid);
    g += `<circle cx="${tx(a.at.x)}" cy="${ty(a.at.y)}" r="${isC ? 4 : 2}" fill="${isC ? "red" : a.anchor.kind === "emitter" ? "blue" : "none"}" stroke="${isC ? "red" : "#08c"}" stroke-width="0.8"/>`;
  }
  g += `<line x1="${xOff * S}" x2="${(xOff + w) * S}" y1="${ty(0)}" y2="${ty(0)}" stroke="#555" stroke-width="1.5"/>`;
  g += `<text x="${(xOff + 0.05) * S}" y="14" font-size="12" font-family="sans-serif">${q.kind === "pose" ? q.pose : q.clip + " t=" + q.timeS.toFixed(3)}</text>`;
  cells += g;
  info.push(`${q.kind === "pose" ? q.pose : q.clip + "@" + q.timeS.toFixed(3)} h=${(st.bounds.maxY - st.bounds.minY).toFixed(3)} w=${(st.bounds.maxX - st.bounds.minX).toFixed(3)} minY=${st.bounds.minY.toFixed(3)} contacts=[${st.contacts}]`);
  xOff += w;
}
writeFileSync(out!, `<svg xmlns="http://www.w3.org/2000/svg" width="${xOff * S}" height="${H * S}" viewBox="0 0 ${xOff * S} ${H * S}"><rect width="100%" height="100%" fill="white"/>${cells}</svg>`);
console.log(info.join("\n"));
