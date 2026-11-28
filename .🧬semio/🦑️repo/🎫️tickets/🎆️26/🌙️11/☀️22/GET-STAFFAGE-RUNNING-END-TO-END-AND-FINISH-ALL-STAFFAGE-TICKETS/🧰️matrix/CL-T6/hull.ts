/** 🥚️ The convex hull of points in space as outward triangles over the indices of the points (incremental, with a horizon per added point); a point within `eps` of the hull so far is left out. Used for a foliage lobe that keeps a chosen part of the corner points of its reference facets.
 * @see ./facet.ts */
export function hull3(points: readonly (readonly number[])[], eps = 1e-9): number[][] {
  const sub = (a: readonly number[], b: readonly number[]): number[] => [a[0]! - b[0]!, a[1]! - b[1]!, a[2]! - b[2]!];
  const cross = (a: readonly number[], b: readonly number[]): number[] => [a[1]! * b[2]! - a[2]! * b[1]!, a[2]! * b[0]! - a[0]! * b[2]!, a[0]! * b[1]! - a[1]! * b[0]!];
  const dot = (a: readonly number[], b: readonly number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;
  const n = points.length;
  if (n < 4) throw new Error("hull3: fewer than four points");
  let i1 = 1, best = 0;
  for (let i = 1; i < n; i++) { const d = dot(sub(points[i]!, points[0]!), sub(points[i]!, points[0]!)); if (d > best) { best = d; i1 = i; } }
  let i2 = -1; best = 0;
  for (let i = 0; i < n; i++) { const c = cross(sub(points[i1]!, points[0]!), sub(points[i]!, points[0]!)); const d = dot(c, c); if (d > best) { best = d; i2 = i; } }
  const normal = cross(sub(points[i1]!, points[0]!), sub(points[i2]!, points[0]!));
  let i3 = -1; best = 0;
  for (let i = 0; i < n; i++) { const d = Math.abs(dot(normal, sub(points[i]!, points[0]!))); if (d > best) { best = d; i3 = i; } }
  if (i2 < 0 || i3 < 0 || !(best > 0)) throw new Error("hull3: the points lie in one plane");
  const inner = [0, 1, 2].map((k) => (points[0]![k]! + points[i1]![k]! + points[i2]![k]! + points[i3]![k]!) / 4);
  type Face = { v: [number, number, number]; n: number[]; d: number; alive: boolean };
  const faces: Face[] = [];
  const edges = new Map<number, number>();
  const add = (a: number, b: number, c: number): void => {
    let nn = cross(sub(points[b]!, points[a]!), sub(points[c]!, points[a]!));
    const length = Math.hypot(nn[0]!, nn[1]!, nn[2]!) || 1;
    nn = nn.map((x) => x / length);
    let v: [number, number, number] = [a, b, c];
    if (dot(nn, inner) - dot(nn, points[a]!) > 0) { nn = nn.map((x) => -x); v = [a, c, b]; }
    faces.push({ v, n: nn, d: dot(nn, points[v[0]]!), alive: true });
    for (const [p, q] of [[v[0], v[1]], [v[1], v[2]], [v[2], v[0]]] as const) edges.set(p * n + q, faces.length - 1);
  };
  add(0, i1, i2); add(0, i1, i3); add(0, i2, i3); add(i1, i2, i3);
  const used = new Set([0, i1, i2, i3]);
  for (let p = 0; p < n; p++) {
    if (used.has(p)) continue;
    const seen = faces.map((f, at) => (f.alive && dot(f.n, points[p]!) - f.d > eps ? at : -1)).filter((at) => at >= 0);
    if (seen.length === 0) continue;
    const gone = new Set(seen);
    const horizon: [number, number][] = [];
    for (const at of seen) {
      const v = faces[at]!.v;
      for (const [a, b] of [[v[0], v[1]], [v[1], v[2]], [v[2], v[0]]] as const) if (!gone.has(edges.get(b * n + a)!)) horizon.push([a, b]);
    }
    for (const at of seen) faces[at]!.alive = false;
    for (const [a, b] of horizon) add(a, b, p);
  }
  return faces.filter((f) => f.alive).map((f) => [...f.v]);
}
