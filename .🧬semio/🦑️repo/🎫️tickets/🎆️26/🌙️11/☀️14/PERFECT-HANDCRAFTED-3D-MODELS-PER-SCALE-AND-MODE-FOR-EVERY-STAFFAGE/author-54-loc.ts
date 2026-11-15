import { openCtx } from "./kit-lib.ts";
import { localGeometry, qualityOfAsset, Mask } from "./volumes-lib.ts";
const [spec, sc, id] = [process.argv[2]!, Number(process.argv[3]), process.argv[4]!];
const c = openCtx(spec, sc);
const s = c.vol.body.find((x: any) => x.id === id);
const g = localGeometry(s, qualityOfAsset(c.m));
const box = { minX: -0.2, maxX: 1.3, minY: -0.4, maxY: 0.4 }, cell = 0.002;
const m = new Mask(box, cell);
const P = (i: number, a: number, b: number) => ({ x: g.positions[i * 3 + a]!, y: g.positions[i * 3 + b]! });
for (let t = 0; t < g.indices.length; t += 3) m.fill([[0, 1, 2].map((k) => P(g.indices[t + k]!, 0, 2))]);
let A = 0; for (const x of m.cells) A += x;
console.log("local (u,w) projected area", (A * cell * cell).toFixed(4));
const geo = c.raw.representations[String(sc)].geometry.elevation.find((x: any) => x.id === id);
let ar = 0; const pts = geo.points; for (let i = 0, j = pts.length - 1; i < pts.length; j = i++) ar += pts[j][0] * pts[i][1] - pts[i][0] * pts[j][1];
console.log("polygon area", Math.abs(ar / 2).toFixed(4));
let S = 0;
for (let t = 0; t < g.indices.length; t += 3) { const [p, q, r] = [0, 1, 2].map((k) => P(g.indices[t + k]!, 0, 2)); S += Math.abs((q.x - p.x) * (r.y - p.y) - (r.x - p.x) * (q.y - p.y)) / 2; }
console.log("half of summed projected triangle areas", (S / 2).toFixed(4), "triangles", g.indices.length / 3);
import { spanAt } from "./author-54-lib.ts";
for (let u = 0.02; u < 0.95; u += 0.06) {
  const i = Math.round((u - box.minX) / cell);
  let lo = 1e9, hi = -1e9, n = 0;
  for (let j = 0; j < m.height; j++) if (m.cells[j * m.width + i]) { const w = box.minY + (j + 0.5) * cell; lo = Math.min(lo, w); hi = Math.max(hi, w); n++; }
  const sp = spanAt(geo.points, 0, u);
  console.log(u.toFixed(2), "mesh", lo.toFixed(3), hi.toFixed(3), "filled", (n * cell).toFixed(3), "poly", sp ? sp.map((x) => x.toFixed(3)).join(" ") : "-");
}
