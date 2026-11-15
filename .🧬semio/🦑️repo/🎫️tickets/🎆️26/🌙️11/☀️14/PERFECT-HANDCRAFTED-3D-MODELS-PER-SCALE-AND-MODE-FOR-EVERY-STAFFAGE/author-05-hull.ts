import { Mask, distanceField, entryOf, featureM, poseState, type Json } from "./volumes-lib.ts";
const [spec, scaleS] = process.argv.slice(2);
const scale = Number(scaleS);
const { asset } = entryOf(spec!);
const st = poseState(asset, scale);
const feature = featureM(scale, asset);
const c = Math.max(feature * 0.5, 0.0025);
const shapes = (v: "elevation" | "front" | "plan") => st[v].filter((e: Json) => e.closed && ["body", "detail", "glazing"].includes(e.shape.role) && e.rings.length > 0);
const ext = (v: "elevation" | "front" | "plan") => {
  const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const e of shapes(v)) for (const r of e.rings) for (const p of r) { b.minX = Math.min(b.minX, p.x); b.maxX = Math.max(b.maxX, p.x); b.minY = Math.min(b.minY, p.y); b.maxY = Math.max(b.maxY, p.y); }
  return b;
};
const [be, bf, bp] = [ext("elevation"), ext("front"), ext("plan")];
const X0 = Math.min(be.minX, bp.minX) - 2 * c, X1 = Math.max(be.maxX, bp.maxX) + 2 * c;
const Y0 = Math.min(be.minY, bf.minY) - 2 * c, Y1 = Math.max(be.maxY, bf.maxY) + 2 * c;
const Z0 = Math.min(bf.minX, bp.minY) - 2 * c, Z1 = Math.max(bf.maxX, bp.maxY) + 2 * c;
const nx = Math.ceil((X1 - X0) / c), ny = Math.ceil((Y1 - Y0) / c), nz = Math.ceil((Z1 - Z0) / c);
const mk = (v: "elevation" | "front" | "plan", b: { minX: number; minY: number }, w: number, h: number) => {
  const m = new Mask({ minX: b.minX, minY: b.minY, maxX: b.minX + w * c, maxY: b.minY + h * c }, c);
  for (const e of shapes(v)) m.fill(e.rings);
  return m;
};
const E = mk("elevation", { minX: X0, minY: Y0 }, nx, ny); // x,y
const P = mk("plan", { minX: X0, minY: Z0 }, nx, nz); // x,z
const F = mk("front", { minX: Z0, minY: Y0 }, nz, ny); // z,y
const e = (i: number, j: number) => E.cells[j * E.width + i]!;
const p = (i: number, k: number) => P.cells[k * P.width + i]!;
const f = (k: number, j: number) => F.cells[j * F.width + k]!;
const E2 = new Mask(E.box, c), P2 = new Mask(P.box, c), F2 = new Mask(F.box, c);
for (let i = 0; i < nx; i++) for (let j = 0; j < ny; j++) if (e(i, j)) for (let k = 0; k < nz; k++) if (p(i, k) && f(k, j)) {
  E2.cells[j * E2.width + i] = 1;
  P2.cells[k * P2.width + i] = 1;
  F2.cells[j * F2.width + k] = 1;
}
const rep = (name: string, T: Mask, R: Mask) => {
  const d = distanceField(R);
  let miss = 0, tot = 0, worst = 0, wi = 0, wj = 0;
  for (let j = 0; j < T.height; j++) for (let i = 0; i < T.width; i++) if (T.cells[j * T.width + i]) {
    tot++;
    if (!R.cells[j * R.width + i]) {
      miss++;
      const dd = d[j * R.width + i]!;
      if (dd > worst) { worst = dd; wi = i; wj = j; }
    }
  }
  console.log(name.padEnd(10), "unrealizable cells", ((100 * miss) / tot).toFixed(1) + "%", "floor Hausdorff", ((worst - c) * 1000).toFixed(0) + "mm", "limit", (2 * feature * 1000).toFixed(0) + "mm", `at (${(T.box.minX + (wi + 0.5) * c).toFixed(2)},${(T.box.minY + (wj + 0.5) * c).toFixed(2)})`);
};
rep("elevation", E, E2);
rep("front", F, F2);
rep("plan", P, P2);
