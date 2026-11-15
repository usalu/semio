import { openCtx } from "./kit-lib.ts";
import { localGeometry, qualityOfAsset, Mask } from "./volumes-lib.ts";
import { pathPoints, spanAt } from "./author-54-lib.ts";
const [spec, sc, id] = [process.argv[2]!, Number(process.argv[3]), process.argv[4]!];
const c = openCtx(spec, sc);
const s = c.vol.body.find((x: any) => x.id === id);
const g = localGeometry(s, qualityOfAsset(c.m));
const box = { minX: -0.1, maxX: 0.5, minY: -0.2, maxY: 0.2 }, cell = 0.002;
const m = new Mask(box, cell);
for (let t = 0; t < g.indices.length; t += 3) m.fill([[0, 1, 2].map((k) => ({ x: g.positions[g.indices[t + k]! * 3]!, y: g.positions[g.indices[t + k]! * 3 + 2]! }))]);
const geo = c.raw.representations[String(sc)].geometry.elevation.find((x: any) => x.id === id);
const poly = pathPoints(geo.d, 8);
const pm = new Mask(box, cell); pm.fill([poly.map((p) => ({ x: p[0], y: p[1] }))]);
let A = 0, B = 0, I = 0; for (let k = 0; k < m.cells.length; k++) { A += m.cells[k]!; B += pm.cells[k]!; if (m.cells[k] && pm.cells[k]) I++; }
console.log("mesh", (A * cell * cell).toFixed(4), "poly(evenodd)", (B * cell * cell).toFixed(4), "IoU", (I / (A + B - I)).toFixed(3), "npoly", poly.length);
for (let u = 0.0; u < 0.32; u += 0.03) { const sp = spanAt(poly, 0, u + 1e-4); const i = Math.round((u - box.minX) / cell); let lo = 1e9, hi = -1e9; for (let j = 0; j < m.height; j++) if (m.cells[j * m.width + i]) { const w = box.minY + (j + 0.5) * cell; lo = Math.min(lo, w); hi = Math.max(hi, w); } console.log(u.toFixed(2), "mesh", lo.toFixed(3), hi.toFixed(3), "poly", sp ? sp.map((x) => x.toFixed(3)).join(" ") : "-"); }
