/** 📐️ Hand-chosen numbers and pure geometry of the used 16-inch viola in its dart case: outlines, arching caps, lofts and their silhouettes, all in world metres of the rest pose (x ahead to the scroll end, y up, z left).
 *
 * @see ./author-viola.ts — the one authoritative script that writes the asset document from these numbers
 * @see ../../reports/used-viola.md — the reference table the numbers come from */

export type P = [number, number];
export type V = [number, number, number];

/** 🔢️ Rounds to a fixed number of decimals so the document stays readable. */
export const q = (n: number, d = 5): number => Math.round(n * 10 ** d) / 10 ** d;

/** 🎯️ Rounds a point. */
export const qp = (p: readonly number[], d = 5): any => p.map((x) => q(x, d));

/** 🧷️ The case: a dart shell of two tangent circles, base and lid split at the parting line. */
export const CASE = {
  wideX: 0.135,
  wideR: 0.135,
  narrowX: 0.735,
  narrowR: 0.065,
  length: 0.8,
  floorTop: 0.012,
  parting: 0.092,
  lidTop: 0.15,
  wall: 0.01,
  lidSkin: 0.01,
  liningLid: 0.0075,
  bedTop: 0.0195,
  openDeg: 95,
};

/** 🎻️ The viola: 16-inch (406 mm) body, string length 364 mm, lying on its back in the case. */
export const VIOLA = {
  tailX: 0.046,
  body: 0.406,
  origin: [0.04, 0.054, 0] as V,
  backApex: 0.019,
  backEdge: 0.0375,
  ribTop: 0.0755,
  plateEdge: 0.0035,
  topArch: 0.016,
  backArch: 0.015,
  overhang: 0.0025,
  ribWall: 0.0012,
  centreS: 0.205,
  bridgeX: 0.238,
  bridgeTop: 0.1315,
  nutX: 0.602,
  fbEndX: 0.312,
  neckRootX: 0.452,
  scrollEndX: 0.717,
  endButtonX: 0.04,
  stringBridgeY: 0.1305,
  stringNutY: 0.0706,
};

/** 🏹️ The viola bow: 740 mm, 70 g, hair ribbon 11 mm, lying in the lid with the hair towards the viola. */
export const BOW = {
  button: -0.065,
  head: 0.675,
  hair: 0.648,
  frog: [0.1099, 0.104, -0.0461] as V,
  dir: [0.9982, 0, 0.0607] as V,
};

/** 🔁️ A closed Catmull-Rom spline through control points, `per` samples per segment. */
export function catmull(points: P[], per: number): P[] {
  const out: P[] = [];
  const n = points.length;
  for (let i = 0; i < n; i++) {
    const p0 = points[(i - 1 + n) % n]!, p1 = points[i]!, p2 = points[(i + 1) % n]!, p3 = points[(i + 2) % n]!;
    for (let k = 0; k < per; k++) {
      const t = k / per, t2 = t * t, t3 = t2 * t;
      out.push([
        0.5 * (2 * p1[0] + (-p0[0] + p2[0]) * t + (2 * p0[0] - 5 * p1[0] + 4 * p2[0] - p3[0]) * t2 + (-p0[0] + 3 * p1[0] - 3 * p2[0] + p3[0]) * t3),
        0.5 * (2 * p1[1] + (-p0[1] + p2[1]) * t + (2 * p0[1] - 5 * p1[1] + 4 * p2[1] - p3[1]) * t2 + (-p0[1] + 3 * p1[1] - 3 * p2[1] + p3[1]) * t3),
      ]);
    }
  }
  return out;
}

/** 📏️ Signed area of a ring (positive counter-clockwise in its own plane). */
export function area(ring: P[]): number {
  let a = 0;
  for (let i = 0; i < ring.length; i++) {
    const p = ring[i]!, n = ring[(i + 1) % ring.length]!;
    a += p[0] * n[1] - n[0] * p[1];
  }
  return a / 2;
}

/** ⚖️ Area centroid of a ring. */
export function centroid(ring: P[]): P {
  let a = 0, cx = 0, cy = 0;
  for (let i = 0; i < ring.length; i++) {
    const p = ring[i]!, n = ring[(i + 1) % ring.length]!;
    const c = p[0] * n[1] - n[0] * p[1];
    a += c;
    cx += (p[0] + n[0]) * c;
    cy += (p[1] + n[1]) * c;
  }
  return [cx / (3 * a), cy / (3 * a)];
}

/** 🔄️ The ring counter-clockwise. */
export function ccw(ring: P[]): P[] {
  return area(ring) < 0 ? [...ring].reverse() : ring;
}

/** ↘️ A smooth ring moved inwards by `d` along the vertex normals (the ring counter-clockwise). */
export function inset(ring: P[], d: number): P[] {
  const r = ccw(ring);
  return r.map((p, i) => {
    const a = r[(i - 1 + r.length) % r.length]!, b = r[(i + 1) % r.length]!;
    const tx = b[0] - a[0], ty = b[1] - a[1];
    const l = Math.hypot(tx, ty);
    return [p[0] - (ty / l) * d, p[1] + (tx / l) * d] as P;
  });
}

/** 🎯️ The dart outline of the case in (x, z), counter-clockwise, `d` inside the outer skin: the hull of the two end circles. */
export function dart(d: number, n1 = 36, n2 = 20): P[] {
  const r1 = CASE.wideR - d, r2 = CASE.narrowR - d;
  const alpha = Math.asin((CASE.wideR - CASE.narrowR) / (CASE.narrowX - CASE.wideX));
  const out: P[] = [];
  const a0 = -(Math.PI / 2 - alpha), a1 = Math.PI / 2 - alpha;
  for (let i = 0; i <= n2; i++) {
    const a = a0 + ((a1 - a0) * i) / n2;
    out.push([CASE.narrowX + r2 * Math.cos(a), r2 * Math.sin(a)]);
  }
  const b0 = Math.PI / 2 - alpha, b1 = (3 * Math.PI) / 2 + alpha;
  for (let i = 0; i <= n1; i++) {
    const a = b0 + ((b1 - b0) * i) / n1;
    out.push([CASE.wideX + r1 * Math.cos(a), r1 * Math.sin(a)]);
  }
  return out;
}

/** 📐️ The hinge: the upper tangent of the dart at the parting line, from the wide to the narrow circle. */
export function hinge(): { origin: V; dir: V; deltaDeg: number } {
  const alpha = Math.asin((CASE.wideR - CASE.narrowR) / (CASE.narrowX - CASE.wideX));
  const origin: V = [CASE.wideX + CASE.wideR * Math.sin(alpha), CASE.parting, CASE.wideR * Math.cos(alpha)];
  return { origin, dir: [Math.cos(alpha), 0, -Math.sin(alpha)], deltaDeg: (alpha * 180) / Math.PI };
}

/** ↔️ Half width of the dart at x on the lateral side `side` (+1 left, -1 right), at inset d. */
export function dartHalf(x: number, d = 0): number {
  const ring = dart(d, 72, 40);
  let best = 0;
  for (let i = 0; i < ring.length; i++) {
    const a = ring[i]!, b = ring[(i + 1) % ring.length]!;
    if ((a[0] - x) * (b[0] - x) <= 0 && a[0] !== b[0]) {
      const t = (x - a[0]) / (b[0] - a[0]);
      best = Math.max(best, Math.abs(a[1] + (b[1] - a[1]) * t));
    }
  }
  return best;
}

/** 🎻️ Half-outline control points of the 406 mm body: (s from the tail, half width). */
const HALF: P[] = [
  [0.0, 0.0], [0.0015, 0.03], [0.006, 0.055], [0.016, 0.078], [0.032, 0.097], [0.055, 0.11], [0.082, 0.1168], [0.105, 0.1175], [0.13, 0.1145], [0.152, 0.106],
  [0.165, 0.096], [0.172, 0.0885], [0.18, 0.076], [0.195, 0.0685], [0.215, 0.065], [0.235, 0.067], [0.249, 0.0735], [0.257, 0.082], [0.267, 0.088], [0.29, 0.093],
  [0.32, 0.095], [0.345, 0.0935], [0.368, 0.087], [0.386, 0.075], [0.398, 0.057], [0.4045, 0.032], [0.406, 0.0],
];

/** 🎻️ The plate outline of the viola body in world (x, z), counter-clockwise, `per` samples per control segment, `d` inside. */
export function violaOutline(per = 3, d = 0): P[] {
  const right = HALF.map(([s, h]) => [VIOLA.tailX + s, -h] as P);
  const left = HALF.slice(1, -1).reverse().map(([s, h]) => [VIOLA.tailX + s, h] as P);
  const ring = ccw(catmull([...right, ...left], per));
  return d === 0 ? ring : inset(ring, d);
}

/** 🏔️ The arching profile: 0 at the edge ring, 1 at the apex (full in the middle, steep near the edge). */
export const arch = (t: number): number => 1 - (1 - t) ** 2.4;

/** 🏔️ A closed arched plate as a polyhedron over the outline: flat face at y0, edge of te, arch of ha in direction dir (+1 up for the top, -1 down for the back), K rings scaled to the centre c. */
export function capMesh(outline: P[], c: P, y0: number, te: number, ha: number, dir: 1 | -1, K: number): { vertices: V[]; triangles: [number, number, number][]; rings: { scale: number; y: number }[] } {
  const N = outline.length;
  const vertices: V[] = [[c[0], y0, c[1]]];
  for (const p of outline) vertices.push([p[0], y0, p[1]]);
  const rings: { scale: number; y: number }[] = [];
  for (let k = 0; k < K; k++) {
    const s = 1 - k / K;
    const y = y0 + dir * (te + ha * arch(k / K));
    rings.push({ scale: s, y });
    for (const p of outline) vertices.push([c[0] + (p[0] - c[0]) * s, y, c[1] + (p[1] - c[1]) * s]);
  }
  vertices.push([c[0], y0 + dir * (te + ha), c[1]]);
  const apex = vertices.length - 1;
  const base = (i: number) => 1 + (i % N);
  const ring = (k: number, i: number) => 1 + N + k * N + (i % N);
  const tris: [number, number, number][] = [];
  for (let i = 0; i < N; i++) tris.push([0, base(i), base(i + 1)]);
  for (let i = 0; i < N; i++) {
    tris.push([base(i), ring(0, i), ring(0, i + 1)]);
    tris.push([base(i), ring(0, i + 1), base(i + 1)]);
  }
  for (let k = 0; k < K - 1; k++)
    for (let i = 0; i < N; i++) {
      tris.push([ring(k, i), ring(k + 1, i), ring(k + 1, i + 1)]);
      tris.push([ring(k, i), ring(k + 1, i + 1), ring(k, i + 1)]);
    }
  for (let i = 0; i < N; i++) tris.push([ring(K - 1, i), apex, ring(K - 1, i + 1)]);
  let vol = 0;
  for (const [a, b, d] of tris) {
    const A = vertices[a]!, B = vertices[b]!, D = vertices[d]!;
    vol += A[0] * (B[1] * D[2] - B[2] * D[1]) - A[1] * (B[0] * D[2] - B[2] * D[0]) + A[2] * (B[0] * D[1] - B[1] * D[0]);
  }
  const triangles = vol > 0 ? tris : tris.map(([a, b, d]) => [a, d, b] as [number, number, number]);
  return { vertices, triangles, rings };
}

/** 🔦️ The upper (dir +1) or lower (dir -1) envelope of a point cloud along one axis, binned at `step`, as a closed polygon down (or up) to `flat`. */
export function envelope(points: P[], step: number, dir: 1 | -1, flat: number): P[] {
  const bins = new Map<number, number>();
  for (const [a, b] of points) {
    const k = Math.round(a / step);
    const prev = bins.get(k);
    if (prev === undefined || (dir > 0 ? b > prev : b < prev)) bins.set(k, b);
  }
  const keys = [...bins.keys()].sort((x, y) => x - y);
  const top: P[] = keys.map((k) => [k * step, bins.get(k)!]);
  return [[top[0]![0], flat], ...top, [top[top.length - 1]![0], flat]];
}

/** 🕸️ Dense surface samples of a mesh (vertices and points inside every triangle). */
export function samples(mesh: { vertices: V[]; triangles: [number, number, number][] }): V[] {
  const out: V[] = [...mesh.vertices];
  for (const [a, b, c] of mesh.triangles) {
    const A = mesh.vertices[a]!, B = mesh.vertices[b]!, C = mesh.vertices[c]!;
    for (let i = 0; i <= 4; i++)
      for (let j = 0; j <= 4 - i; j++) {
        const u = i / 4, v = j / 4, w = 1 - u - v;
        out.push([A[0] * w + B[0] * u + C[0] * v, A[1] * w + B[1] * u + C[1] * v, A[2] * w + B[2] * u + C[2] * v]);
      }
  }
  return out;
}

/** 🏔️ Height of the top arching at (x, z) on the rest pose (the upper surface of the top cap), from the same rings the mesh uses. */
export function topHeight(x: number, z: number, outline: P[]): number {
  const c: P = [VIOLA.tailX + VIOLA.centreS, 0];
  const dx = x - c[0], dz = z - c[1];
  const reach = rayReach(outline, c, Math.atan2(dz, dx));
  const t = reach > 0 ? Math.max(0, 1 - Math.hypot(dx, dz) / reach) : 0;
  return VIOLA.ribTop + VIOLA.plateEdge + VIOLA.topArch * arch(t);
}

/** 📡️ Distance from c to the outline along the direction `ang`. */
export function rayReach(outline: P[], c: P, ang: number): number {
  const dx = Math.cos(ang), dz = Math.sin(ang);
  let best = 0;
  for (let i = 0; i < outline.length; i++) {
    const a = outline[i]!, b = outline[(i + 1) % outline.length]!;
    const ex = b[0] - a[0], ez = b[1] - a[1];
    const den = dx * ez - dz * ex;
    if (Math.abs(den) < 1e-14) continue;
    const t = ((a[0] - c[0]) * ez - (a[1] - c[1]) * ex) / den;
    const s = ((a[0] - c[0]) * dz - (a[1] - c[1]) * dx) / den;
    if (t > 0 && s >= -1e-9 && s <= 1 + 1e-9) best = Math.max(best, t);
  }
  return best;
}

/** 🧵️ A superelliptic loft section along u (rx half depth along v, ry half width along w). */
export type Section = { x: number; y: number; z?: number; rx: number; ry: number; n: number };

/** 🧵️ The loft of sections, in the frame shifted by `o`. */
export function loft(sections: Section[], o: V): any {
  return {
    kind: "loft",
    sections: sections.map((s) => ({ at: qp([s.x - o[0], s.y - o[1], (s.z ?? 0) - o[2]]), rx: q(s.rx), ry: q(s.ry), exponent: s.n, rotation: [0.5, 0.5, 0.5, 0.5] })),
    capStart: true,
    capEnd: true,
  };
}

/** ✏️ Side outline (x, y) of a loft: upper edges then lower edges. */
export function loftSide(sections: Section[]): P[] {
  return [...sections.map((s) => [s.x, s.y + s.rx] as P), ...[...sections].reverse().map((s) => [s.x, s.y - s.rx] as P)];
}

/** ✏️ Plan outline (x, z) of a loft. */
export function loftPlan(sections: Section[]): P[] {
  return [...sections.map((s) => [s.x, (s.z ?? 0) - s.ry] as P), ...[...sections].reverse().map((s) => [s.x, (s.z ?? 0) + s.ry] as P)];
}

/** ⭕️ A superellipse ring (a along the first, b along the second axis) about (cx, cy). */
export function superellipse(cx: number, cy: number, a: number, b: number, n: number, m = 32): P[] {
  const out: P[] = [];
  for (let i = 0; i < m; i++) {
    const t = (2 * Math.PI * i) / m;
    const c = Math.cos(t), s = Math.sin(t);
    out.push([cx + a * Math.sign(c) * Math.abs(c) ** (2 / n), cy + b * Math.sign(s) * Math.abs(s) ** (2 / n)]);
  }
  return out;
}

/** 🪢️ Convex hull (monotone chain) of points. */
export function hull(points: P[]): P[] {
  const p = [...points].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const cross = (o: P, a: P, b: P) => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const lower: P[] = [];
  for (const pt of p) {
    while (lower.length >= 2 && cross(lower[lower.length - 2]!, lower[lower.length - 1]!, pt) <= 0) lower.pop();
    lower.push(pt);
  }
  const upper: P[] = [];
  for (const pt of [...p].reverse()) {
    while (upper.length >= 2 && cross(upper[upper.length - 2]!, upper[upper.length - 1]!, pt) <= 0) upper.pop();
    upper.push(pt);
  }
  return [...lower.slice(0, -1), ...upper.slice(0, -1)];
}

/** ✏️ Front outline (z, y) of a loft: the hull of its sections seen along x. */
export function loftFront(sections: Section[]): P[] {
  return hull(sections.flatMap((s) => superellipse(s.z ?? 0, s.y, s.ry, s.rx, s.n, 24)));
}

/** 🧭️ Removes near-duplicate consecutive points of a ring. */
export function clean(ring: P[], eps = 2e-4): P[] {
  const out: P[] = [];
  for (const p of ring) {
    const last = out[out.length - 1];
    if (last === undefined || Math.hypot(p[0] - last[0], p[1] - last[1]) > eps) out.push(p);
  }
  if (out.length > 2 && Math.hypot(out[0]![0] - out[out.length - 1]![0], out[0]![1] - out[out.length - 1]![1]) <= eps) out.pop();
  return out;
}

/** 📉️ Douglas-Peucker simplification of a closed ring. */
export function simplify(ring: P[], tol: number): P[] {
  if (ring.length < 8) return ring;
  const dp = (pts: P[]): P[] => {
    if (pts.length < 3) return pts;
    const a = pts[0]!, b = pts[pts.length - 1]!;
    let best = -1, idx = 0;
    for (let i = 1; i < pts.length - 1; i++) {
      const p = pts[i]!;
      const l = Math.hypot(b[0] - a[0], b[1] - a[1]) || 1e-12;
      const d = Math.abs((b[0] - a[0]) * (a[1] - p[1]) - (a[0] - p[0]) * (b[1] - a[1])) / l;
      if (d > best) { best = d; idx = i; }
    }
    if (best <= tol) return [a, b];
    return [...dp(pts.slice(0, idx + 1)).slice(0, -1), ...dp(pts.slice(idx))];
  };
  const half = Math.floor(ring.length / 2);
  const first = dp([...ring.slice(0, half + 1)]);
  const second = dp([...ring.slice(half), ring[0]!]);
  return clean([...first.slice(0, -1), ...second.slice(0, -1)]);
}
