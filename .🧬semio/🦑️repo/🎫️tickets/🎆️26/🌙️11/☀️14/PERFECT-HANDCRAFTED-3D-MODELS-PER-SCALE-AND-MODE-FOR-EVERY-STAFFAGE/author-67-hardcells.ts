/** Which hard (non-foliage) boundary cells are farther than `limit` metres from the other mask? usage: bun author-67-hardcells.ts <asset> <scale> <view> <limit_m> [foliage-region-regex] */
import { Mask, VIEWS } from "./volumes-lib.ts";
import { Fitter } from "./kit-fitter.ts";
import { drawn, openCtx, type Json } from "./kit-lib.ts";
const [spec, sc, view, lim, fre = "^(lobe|fill|crown|canopy|pad)"] = process.argv.slice(2) as string[];
const c = openCtx(spec!, Number(sc));
const regions: Record<string, Json> = Object.fromEntries(c.m.thermal.regions.map((r: Json) => [r.id, r]));
const isFol = (id?: string) => id !== undefined && regions[id]?.kind === "foliage";
const fit = new Fitter(c, 2400);
const body: Json[] = c.vol.body;
const meshes = body.map((s) => fit.mesh(s));
const cells = meshes.map((m) => fit.cellsOf(m));
const v = view as any;
const s = fit.views.get(v)!;
const { width: w, height: h } = s.truth;
const vol = new Uint8Array(s.count.length);
const soft = new Uint8Array(s.count.length);
body.forEach((sd, i) => {
  if (sd.slender === true || sd.texture === true) return;
  for (const k of cells[i]!.get(v)!) { vol[k] = 1; if (new RegExp(fre).test(sd.id) || isFol(sd.region)) soft[k] = 1; }
});
const sm = new Mask(s.box, s.cell);
for (const sh of drawn(c, v)) if (isFol(sh.region)) sm.fill(sh.rings);
for (let k = 0; k < soft.length; k++) if (sm.cells[k]) soft[k] = 1;
const drawnM = s.truth.cells;
const bnd = (m: Uint8Array) => { const o = new Uint8Array(m.length); for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) { const k = j * w + i; if (!m[k]) continue; if (i === 0 || j === 0 || i === w - 1 || j === h - 1 || !m[k - 1] || !m[k + 1] || !m[k - w] || !m[k + w]) o[k] = 1; } return o; };
function dist(mask: Uint8Array): Float32Array {
  const d = new Float32Array(w * h).fill(1e9); for (let i = 0; i < mask.length; i++) if (mask[i]) d[i] = 0;
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) { const k = j * w + i; let x = d[k]!; if (i > 0) x = Math.min(x, d[k - 1]! + 1); if (j > 0) { x = Math.min(x, d[k - w]! + 1); if (i > 0) x = Math.min(x, d[k - w - 1]! + 1.4142); if (i < w - 1) x = Math.min(x, d[k - w + 1]! + 1.4142); } d[k] = x; }
  for (let j = h - 1; j >= 0; j--) for (let i = w - 1; i >= 0; i--) { const k = j * w + i; let x = d[k]!; if (i < w - 1) x = Math.min(x, d[k + 1]! + 1); if (j < h - 1) { x = Math.min(x, d[k + w]! + 1); if (i < w - 1) x = Math.min(x, d[k + w + 1]! + 1.4142); if (i > 0) x = Math.min(x, d[k + w - 1]! + 1.4142); } d[k] = x; }
  return d;
}
const bd = bnd(drawnM), bv = bnd(vol);
const dd = dist(bd), dv = dist(bv), ds = dist(soft);
const L = Number(lim) / s.cell;
const far: { k: number; d: number; side: string }[] = [];
let totalHard = 0;
for (let k = 0; k < bd.length; k++) {
  if (bd[k] && ds[k]! > 3) { totalHard++; if (dv[k]! > L) far.push({ k, d: dv[k]!, side: "drawn" }); }
  if (bv[k] && ds[k]! > 3) { totalHard++; if (dd[k]! > L) far.push({ k, d: dd[k]!, side: "3D" }); }
}
console.log(`cell ${(s.cell * 1000).toFixed(1)} mm, hard boundary cells ${totalHard}, farther than ${lim} m: ${far.length} (${((100 * far.length) / totalHard).toFixed(1)} %)`);
const grid = new Map<string, { n: number; side: string; x: number; y: number; dmax: number }>();
for (const f of far) { const x = s.box.minX + ((f.k % w) + 0.5) * s.cell, y = s.box.minY + (Math.floor(f.k / w) + 0.5) * s.cell; const key = `${f.side} ${Math.round(x / 0.5)} ${Math.round(y / 0.5)}`; const g = grid.get(key) ?? { n: 0, side: f.side, x: 0, y: 0, dmax: 0 }; g.n++; g.x += x; g.y += y; g.dmax = Math.max(g.dmax, f.d * s.cell); grid.set(key, g); }
const near = (x: number, y: number) => {
  const shapes = drawn(c, v).filter((sh) => sh.rings.some((r) => { let mn = 1e9; for (const p of r) mn = Math.min(mn, Math.hypot(p.x - x, p.y - y)); return mn < 0.35; })).map((sh) => sh.id);
  const sol = body.filter((_, i) => { const l = cells[i]!.get(v)!; const ci = Math.round((x - s.box.minX) / s.cell), cj = Math.round((y - s.box.minY) / s.cell); for (const k of l) if (Math.abs((k % w) - ci) < 20 && Math.abs(Math.floor(k / w) - cj) < 20) return true; return false; }).map((sd) => sd.id);
  return `shapes[${shapes.slice(0, 5)}] solids[${sol.slice(0, 5)}]`;
};
for (const g of [...grid.values()].sort((a, b) => b.n - a.n).slice(0, 14)) console.log(g.side, "n", g.n, "at", (g.x / g.n).toFixed(2), (g.y / g.n).toFixed(2), "dmax mm", (g.dmax * 1000).toFixed(0), near(g.x / g.n, g.y / g.n));
