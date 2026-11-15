/** 🖼️ Author 61: the missing elevation outlines of the willow's curtain sheets (every `f-c*`/`p-c*` pair without an elevation shape): x extent from the sheet's plan footprint, height and width profile from its own front outline. Writes JSON to 🗑️generated/author-61/sheets2d.<scale>.json. Usage: `bun author-61-sheets2d.ts <scale>`. */
import { writeFileSync } from "node:fs";
import { Kit, bboxOfShapes, ivalsAll } from "./author-02-lib.ts";
import { TICKET } from "./volumes-lib.ts";

const S = Number(process.argv[2]);
const k = new Kit("vegetation/weeping-willow", S);
const out: unknown[] = [];
const have = new Set(k.shapes("elevation").map((e) => e.shape.id as string));
for (const f of k.shapes("front", (s) => /^f-c\d/.test(s.id))) {
  const key = (f.shape.id as string).replace(/^f-/, "");
  const id = `e-${key}`;
  if (have.has(id) || /-(0|180)$/.test(key)) continue;
  const Pk = k.P((s) => s.id === `p-${key}`);
  const Fk = k.F((s) => s.id === f.shape.id);
  if (!Pk.length) continue;
  const pb = bboxOfShapes(Pk), fb = bboxOfShapes(Fk);
  const cx = (pb.minX + pb.maxX) / 2, hx = (pb.maxX - pb.minX) / 2;
  const widths: number[] = [];
  const N = 40;
  const ys: number[] = [];
  for (let i = 0; i <= N; i++) {
    const y = fb.minY + 1e-3 + ((fb.maxY - fb.minY - 2e-3) * i) / N;
    const iv = ivalsAll(Fk, "y", y);
    ys.push(y);
    widths.push(iv.length ? iv[iv.length - 1]![1] - iv[0]![0] : 0);
  }
  const wmax = Math.max(...widths);
  const left = ys.map((y, i) => [Math.round((cx - (hx * widths[i]!) / wmax) * 1e4) / 1e4, Math.round(y * 1e4) / 1e4]);
  const right = ys.map((y, i) => [Math.round((cx + (hx * widths[i]!) / wmax) * 1e4) / 1e4, Math.round(y * 1e4) / 1e4]).reverse();
  const fr = k.fr[f.shape.bone as string]!;
  const local = [...left, ...right].map(([x, y]) => { const dx = x! - fr.o[0], dy = y! - fr.o[1]; return [Math.round((dx * fr.u[0] + dy * fr.u[1]) * 1e4) / 1e4, Math.round((dx * fr.v[0] + dy * fr.v[1]) * 1e4) / 1e4]; });
  out.push({ id, bone: f.shape.bone, role: "body", z: 4.5, filled: true, tone: f.shape.tone ?? 0.5, kind: "poly", points: local, closed: true, followTip: true, region: f.shape.region, material: f.shape.material });
}
const file = `${TICKET}/🗑️generated/author-61/sheets2d.${S}.json`;
writeFileSync(file, JSON.stringify(out));
console.log(S, out.length, "elevation sheets to add");
