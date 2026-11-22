/** 🪚️ PW-08 geometry helpers shared by the three species: interpolated profiles, stems with root flares, and envelope sleeves that generalise a set of foliage masses for a coarser scale.
 *
 * @see ./lib.ts */
import { blob, tube, type Mesh, type Station, type V3 } from "./lib.ts";

/** 📈️ A Catmull-Rom profile through `(x, y)` knots, clamped at the ends. */
export function profile(knots: readonly [number, number][]): (x: number) => number {
  return (x: number) => {
    if (x <= knots[0]![0]) return knots[0]![1];
    if (x >= knots[knots.length - 1]![0]) return knots[knots.length - 1]![1];
    let i = 0;
    while (knots[i + 1]![0] < x) i++;
    const p0 = knots[Math.max(0, i - 1)]!, p1 = knots[i]!, p2 = knots[i + 1]!, p3 = knots[Math.min(knots.length - 1, i + 2)]!;
    const u = (x - p1[0]) / (p2[0] - p1[0]);
    const m1 = ((p2[1] - p0[1]) / (p2[0] - p0[0] || 1)) * (p2[0] - p1[0]);
    const m2 = ((p3[1] - p1[1]) / (p3[0] - p1[0] || 1)) * (p2[0] - p1[0]);
    const u2 = u * u, u3 = u2 * u;
    return (2 * u3 - 3 * u2 + 1) * p1[1] + (u3 - 2 * u2 + u) * m1 + (-2 * u3 + 3 * u2) * p2[1] + (u3 - u2) * m2;
  };
}

/** 🪵️ A vertical stem from `y0` to `y1` along the world y axis through `(x, z)`: radius `r(y)` times an azimuthal factor `k(θ, y)` (flutes, buttresses), `n` points per ring, a flat foot and an apex or flat top. */
export function stem(x: number, z: number, ys: readonly number[], r: (y: number) => number, k: (theta: number, y: number) => number, n: number, apexTop: boolean): Mesh {
  const st: Station[] = ys.map((y, i) => ({ c: [x, y, z] as V3, r: apexTop && i === ys.length - 1 ? 0 : (th: number) => r(y) * k(th, y) }));
  return tube(st, n, [1, 0, 0]);
}

/** 🎚️ Evenly spaced values from `a` to `b` with steps of at most `step`. */
export function steps(a: number, b: number, step: number): number[] {
  const n = Math.max(1, Math.ceil((b - a) / step));
  return Array.from({ length: n + 1 }, (_, i) => a + ((b - a) * i) / n);
}

/** 🧥️ The envelope sleeve of a set of meshes about the vertical axis through `(cx, cz)`: per height band of `dy` and azimuth bin of `2π/nTheta` the largest radius any vertex reaches, smoothed over `smoothBins` neighbours, never below `minR`, closed by apexes on the axis at the lowest and highest vertex. Used where a coarser scale merges many foliage masses into one generalised mass that keeps their scallops. */
export function sleeve(ms: readonly Mesh[], cx: number, cz: number, dy: number, nTheta: number, smoothBins: number, minR: number, inflate = 1): Mesh {
  let y0 = Infinity, y1 = -Infinity;
  for (const m of ms) for (const q of m.p) { y0 = Math.min(y0, q[1]); y1 = Math.max(y1, q[1]); }
  const nb = Math.max(2, Math.ceil((y1 - y0) / dy));
  const h = (y1 - y0) / nb;
  const R = Array.from({ length: nb + 1 }, () => new Float64Array(nTheta));
  for (const m of ms) for (const q of m.p) {
    const j = Math.round((q[1] - y0) / h);
    const a = Math.atan2(q[2] - cz, q[0] - cx);
    const i = ((Math.round((a / (2 * Math.PI)) * nTheta) % nTheta) + nTheta) % nTheta;
    const rr = Math.hypot(q[0] - cx, q[2] - cz);
    for (const jj of [j - 1, j, j + 1]) if (jj >= 0 && jj <= nb) R[jj]![i] = Math.max(R[jj]![i]!, jj === j ? rr : rr * 0.97);
  }
  const smoothRow = (row: Float64Array): Float64Array => {
    const o = new Float64Array(nTheta);
    for (let i = 0; i < nTheta; i++) {
      let m = 0;
      for (let k = -smoothBins; k <= smoothBins; k++) m = Math.max(m, row[(i + k + nTheta) % nTheta]! * (k === 0 ? 1 : 0.985));
      o[i] = m;
    }
    return o;
  };
  const rows = R.map(smoothRow);
  const st: Station[] = [{ c: [cx, y0, cz], r: 0 }];
  for (let j = 1; j < nb; j++) {
    const row = rows[j]!;
    const prev = rows[j - 1]!, next = rows[j + 1]!;
    const fn = (th: number): number => {
      const u = (((th / (2 * Math.PI)) * nTheta) % nTheta + nTheta) % nTheta;
      const i0 = Math.floor(u) % nTheta, i1 = (i0 + 1) % nTheta, f = u - Math.floor(u);
      const v = Math.max(row[i0]! * (1 - f) + row[i1]! * f, 0.97 * Math.max(prev[i0]! * (1 - f) + prev[i1]! * f, next[i0]! * (1 - f) + next[i1]! * f));
      return Math.max(minR, v * inflate);
    };
    st.push({ c: [cx, y0 + j * h, cz], r: fn });
  }
  st.push({ c: [cx, y1, cz], r: 0 });
  return tube(st, nTheta, [1, 0, 0]);
}

/** 🫧️ A lumpy foliage mass: an ellipsoid with `lobes` bumps of relative height `amp` in random directions (seeded), flattened below by `flat` (0 none, 1 flat base). */
export function lumpy(c: V3, radii: V3, lobes: readonly { dir: V3; amp: number; width: number }[], flat: number, nLat: number, nLon: number): Mesh {
  return blob(c, radii, (th, ph) => {
    const d: V3 = [Math.sin(ph) * Math.cos(th), Math.cos(ph), Math.sin(ph) * Math.sin(th)];
    let k = 1;
    for (const l of lobes) {
      const cosang = d[0] * l.dir[0] + d[1] * l.dir[1] + d[2] * l.dir[2];
      k += l.amp * Math.exp(-((1 - cosang) / l.width));
    }
    const below = Math.max(0, -d[1]);
    return k * (1 - flat * 0.35 * below * below);
  }, nLat, nLon);
}

/** 📏️ The parts' meshes stretched about the stem axis, each side of x and z on its own, and in the upper half of the height, so that their union reaches exactly the envelope `target` of the finest body: a coarser scale generalises the form but keeps the true extent. */
export function fitExtent<T extends { mesh: Mesh }>(parts: readonly T[], target: { min: V3; max: V3 }): T[] {
  const min: V3 = [Infinity, Infinity, Infinity], max: V3 = [-Infinity, -Infinity, -Infinity];
  for (const p of parts) for (const q of p.mesh.p) for (const i of [0, 1, 2]) { min[i] = Math.min(min[i]!, q[i]!); max[i] = Math.max(max[i]!, q[i]!); }
  const k = (i: number, v: number): number => (v >= 0 ? v * (target.max[i]! / max[i]!) : v * (target.min[i]! / min[i]!));
  const half = max[1]! / 2;
  const up = (y: number): number => (y <= half ? y : half + ((y - half) * (target.max[1]! - half)) / (max[1]! - half));
  return parts.map((p) => ({ ...p, mesh: { p: p.mesh.p.map((q): V3 => [k(0, q[0]), up(q[1]), k(2, q[2])]), t: p.mesh.t } }));
}
