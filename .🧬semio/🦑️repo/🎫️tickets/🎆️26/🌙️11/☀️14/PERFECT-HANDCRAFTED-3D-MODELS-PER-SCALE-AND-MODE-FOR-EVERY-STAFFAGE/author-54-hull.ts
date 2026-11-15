import { openCtx, drawn } from "./kit-lib.ts";
import { Mask } from "./volumes-lib.ts";
const c = openCtx("vegetation/agave", 500);
const cell = 0.025;
const bx = { minX: -1.5, maxX: 1.5, minY: -1.5, maxY: 1.5 };
const M: Record<string, Mask> = {};
for (const v of ["elevation", "front", "plan"] as const) { M[v] = new Mask(bx, cell); for (const d of drawn(c, v)) M[v]!.fill(d.rings); }
const idx = (m: Mask, x: number, y: number) => { const i = Math.floor((x - bx.minX) / cell), j = Math.floor((y - bx.minY) / cell); return i < 0 || j < 0 || i >= m.width || j >= m.height ? 0 : m.cells[j * m.width + i]!; };
// paper axes: elevation (x,y), front (z,y), plan (x,z)
const proj: Record<string, Set<string>> = { elevation: new Set(), front: new Set(), plan: new Set() };
let n = 0;
for (let x = -1.5; x < 1.5; x += cell) for (let y = 0; y < 1.5; y += cell) for (let z = -1.5; z < 1.5; z += cell) {
  if (idx(M.elevation!, x, y) && idx(M.front!, z, y) && idx(M.plan!, x, z)) { n++; proj.elevation!.add(`${Math.floor(x / cell)},${Math.floor(y / cell)}`); proj.front!.add(`${Math.floor(z / cell)},${Math.floor(y / cell)}`); proj.plan!.add(`${Math.floor(x / cell)},${Math.floor(z / cell)}`); }
}
console.log("voxels", n);
const axes: Record<string, [string, string]> = { elevation: ["x", "y"], front: ["z", "y"], plan: ["x", "z"] };
for (const v of ["elevation", "front", "plan"] as const) {
  let drawnN = 0, both = 0;
  const m = M[v]!;
  for (let j = 0; j < m.height; j++) for (let i = 0; i < m.width; i++) if (m.cells[j * m.width + i]) { drawnN++; const key = `${Math.floor((bx.minX + i * cell) / cell + 1e-6)},${Math.floor((bx.minY + j * cell) / cell + 1e-6)}`; if (proj[v]!.has(key)) both++; }
  const p = proj[v]!.size;
  console.log(v, "drawn cells", drawnN, "hull proj", p, "IoU", (both / (drawnN + p - both)).toFixed(3));
}
const vox: [number, number, number][] = [];
for (let x = -1.5; x < 1.5; x += cell) for (let y = 0; y < 1.5; y += cell) for (let z = -1.5; z < 1.5; z += cell) if (idx(M.elevation!, x, y) && idx(M.front!, z, y) && idx(M.plan!, x, z)) vox.push([x + cell / 2, y + cell / 2, z + cell / 2]);
const bins = 24;
const clusters: Record<number, [number, number, number][]> = {};
for (const v of vox) { if (Math.hypot(v[0], v[2]) < 0.12) continue; const th = Math.atan2(v[2], v[0]); const b = Math.floor(((th + Math.PI) / (2 * Math.PI)) * bins); (clusters[b] ??= []).push(v); }
for (const [b, vs] of Object.entries(clusters)) {
  const far = vs.reduce((p, q) => (Math.hypot(q[0], q[2]) > Math.hypot(p[0], p[2]) ? q : p));
  const ys = vs.map((v) => v[1]);
  console.log("bin", b, "ang", ((Number(b) + 0.5) * 360 / bins - 180).toFixed(0), "n", vs.length, "farthest", far.map((x) => x.toFixed(2)).join(","), "y", Math.min(...ys).toFixed(2), Math.max(...ys).toFixed(2));
}
