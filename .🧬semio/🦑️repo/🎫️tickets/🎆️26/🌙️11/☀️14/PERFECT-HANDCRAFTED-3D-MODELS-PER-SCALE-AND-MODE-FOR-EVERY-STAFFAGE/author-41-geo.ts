/** 📐️ Author 41: small 2D geometry helpers for repairing and reading drawn outlines (convex hull, Douglas-Peucker, station outlines).
 * @see ./author-41.md */
export type P = [number, number];

export const cross2 = (o: P, a: P, b: P): number => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);

/** 🔗️ Convex hull (counter-clockwise) of a point set. */
export function hull(points: readonly P[]): P[] {
  const p = [...points].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const lo: P[] = [];
  for (const q of p) {
    while (lo.length >= 2 && cross2(lo[lo.length - 2]!, lo[lo.length - 1]!, q) <= 0) lo.pop();
    lo.push(q);
  }
  const up: P[] = [];
  for (const q of [...p].reverse()) {
    while (up.length >= 2 && cross2(up[up.length - 2]!, up[up.length - 1]!, q) <= 0) up.pop();
    up.push(q);
  }
  return [...lo.slice(0, -1), ...up.slice(0, -1)];
}

/** ✂️ Douglas-Peucker simplification of a closed ring with tolerance `tol`. */
export function simplifyRing(ring: readonly P[], tol: number): P[] {
  if (ring.length <= 4) return [...ring];
  let far = 0;
  let best = -1;
  for (let i = 1; i < ring.length; i++) {
    const d = Math.hypot(ring[i]![0] - ring[0]![0], ring[i]![1] - ring[0]![1]);
    if (d > best) { best = d; far = i; }
  }
  const dp = (a: number, b: number, pts: readonly P[]): P[] => {
    let m = -1;
    let k = -1;
    const [ax, ay] = pts[a]!;
    const [bx, by] = pts[b]!;
    const L = Math.hypot(bx - ax, by - ay) || 1e-9;
    for (let i = a + 1; i < b; i++) {
      const d = Math.abs((bx - ax) * (ay - pts[i]![1]) - (ax - pts[i]![0]) * (by - ay)) / L;
      if (d > m) { m = d; k = i; }
    }
    if (m <= tol) return [pts[a]!];
    return [...dp(a, k, pts), ...dp(k, b, pts)];
  };
  const first = ring.slice(0, far + 1);
  const second = [...ring.slice(far), ring[0]!];
  return [...dp(0, first.length - 1, first), ...dp(0, second.length - 1, second)];
}

/** 🧭️ Span of the union of rings along the line through `at` with direction `d` (unit): `[tmin, tmax]` of the intersection parameters measured along the perpendicular `n`, or null. */
export function lineSpan(rings: readonly (readonly P[])[], at: P, d: P): [number, number] | null {
  const n: P = [-d[1], d[0]];
  let lo = Infinity;
  let hi = -Infinity;
  for (const r of rings) for (let i = 0, k = r.length - 1; i < r.length; k = i++) {
    const a = r[k]!;
    const b = r[i]!;
    const sa = (a[0] - at[0]) * d[0] + (a[1] - at[1]) * d[1];
    const sb = (b[0] - at[0]) * d[0] + (b[1] - at[1]) * d[1];
    if (sa <= 0 === sb <= 0) continue;
    const t = sa / (sa - sb);
    const x = a[0] + (b[0] - a[0]) * t - at[0];
    const y = a[1] + (b[1] - a[1]) * t - at[1];
    const v = x * n[0] + y * n[1];
    lo = Math.min(lo, v);
    hi = Math.max(hi, v);
  }
  return Number.isFinite(lo) ? [lo, hi] : null;
}

/** 🧱️ Outline polygon of the union of rings seen as a function of x: upper points left to right, lower points right to left. */
export function stationOutline(rings: readonly (readonly P[])[], xs: readonly number[]): P[] {
  const top: P[] = [];
  const bot: P[] = [];
  for (const x of xs) {
    const s = lineSpan(rings, [x, 0], [1, 0]);
    if (s === null) continue;
    bot.push([x, s[0]]);
    top.push([x, s[1]]);
  }
  return [...bot, ...top.reverse()].map((p) => [Math.round(p[0] * 1e5) / 1e5, Math.round(p[1] * 1e5) / 1e5] as P);
}

export const range = (a: number, b: number, n: number): number[] => Array.from({ length: n }, (_, i) => a + ((b - a) * i) / (n - 1));

/** 🦌️ Medial path of a thin curved strip (horn, tail, antler): splits the ring at its two farthest points, resamples both chains by arc length and returns `n` stations `{ c, half }` (centre and half distance between the chains), ordered from the end nearer to `from`. */
export function medialPath(ring: readonly P[], n: number, from: P): { c: P; half: number }[] {
  let ia = 0;
  let ib = 1;
  let best = -1;
  for (let i = 0; i < ring.length; i++) for (let j = i + 1; j < ring.length; j++) {
    const d = Math.hypot(ring[i]![0] - ring[j]![0], ring[i]![1] - ring[j]![1]);
    if (d > best) { best = d; ia = i; ib = j; }
  }
  const chain = (a: number, b: number): P[] => { const out: P[] = []; for (let i = a; i !== b; i = (i + 1) % ring.length) out.push(ring[i]!); out.push(ring[b]!); return out; };
  const resample = (c: readonly P[]): P[] => {
    const L = [0];
    for (let i = 1; i < c.length; i++) L.push(L[i - 1]! + Math.hypot(c[i]![0] - c[i - 1]![0], c[i]![1] - c[i - 1]![1]));
    const tot = L[L.length - 1]!;
    return Array.from({ length: n }, (_, k) => {
      const t = (tot * k) / (n - 1);
      let i = 1;
      while (i < L.length - 1 && L[i]! < t) i++;
      const u = (t - L[i - 1]!) / Math.max(L[i]! - L[i - 1]!, 1e-12);
      return [c[i - 1]![0] + (c[i]![0] - c[i - 1]![0]) * u, c[i - 1]![1] + (c[i]![1] - c[i - 1]![1]) * u] as P;
    });
  };
  const c1 = resample(chain(ia, ib));
  const c2 = resample([...chain(ib, ia)].reverse());
  const out = c1.map((p, i) => ({ c: [(p[0] + c2[i]![0]) / 2, (p[1] + c2[i]![1]) / 2] as P, half: Math.hypot(p[0] - c2[i]![0], p[1] - c2[i]![1]) / 2 }));
  const d0 = Math.hypot(out[0]!.c[0] - from[0], out[0]!.c[1] - from[1]);
  const d1 = Math.hypot(out[n - 1]!.c[0] - from[0], out[n - 1]!.c[1] - from[1]);
  return d1 < d0 ? out.reverse() : out;
}
