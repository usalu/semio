type Vec2 = [number, number];
const cross2 = (o: Vec2, a: Vec2, b: Vec2): number => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);

const inTriangle = (p: Vec2, a: Vec2, b: Vec2, c: Vec2, slack = 0): boolean => cross2(a, b, p) >= -slack && cross2(b, c, p) >= -slack && cross2(c, a, p) >= -slack;

/** 🔺️ Triangle index triples into `[...outer, ...holes]`, counter-clockwise; holes must be clockwise and the outer ring counter-clockwise. */
function bridged(outer: readonly Vec2[], holes: readonly (readonly Vec2[])[]): { points: Vec2[]; poly: number[] } {
  const pace = { tick(_: number) {} };
  const points: Vec2[] = [...outer, ...holes.flat()];
  let poly: number[] = outer.map((_, i) => i);
  const ranges: number[][] = [];
  let offset = outer.length;
  for (const hole of holes) {
    ranges.push(hole.map((_, i) => offset + i));
    offset += hole.length;
  }
  ranges.sort((a, b) => Math.max(...b.map((i) => points[i]![0])) - Math.max(...a.map((i) => points[i]![0])));
  for (const hole of ranges) {
    pace.tick(poly.length);
    let m = hole[0]!;
    for (const i of hole) if (points[i]![0] > points[m]![0]) m = i;
    const M = points[m]!;
    let best = Infinity, bridge = -1, hitX = 0;
    for (let k = 0; k < poly.length; k++) {
      const a = points[poly[k]!]!, b = points[poly[(k + 1) % poly.length]!]!;
      if ((a[1] - M[1]) * (b[1] - M[1]) > 0 || a[1] === b[1]) continue;
      const x = a[0] + ((M[1] - a[1]) / (b[1] - a[1])) * (b[0] - a[0]);
      if (x >= M[0] && x < best) {
        best = x;
        hitX = x;
        bridge = a[0] > b[0] ? k : (k + 1) % poly.length;
      }
    }
    if (bridge < 0) continue;
    const P = points[poly[bridge]!]!;
    const I: Vec2 = [hitX, M[1]];
    let bestAngle = Infinity;
    for (let k = 0; k < poly.length; k++) {
      const q = points[poly[k]!]!;
      if (poly[k] === poly[bridge] || q[0] < M[0]) continue;
      const [ta, tb, tc]: [Vec2, Vec2, Vec2] = I[1] < P[1] || (I[1] === P[1] && I[0] < P[0]) ? [M, I, P] : [M, P, I];
      if (!inTriangle(q, ta, tb, tc)) continue;
      const angle = Math.abs(Math.atan2(q[1] - M[1], q[0] - M[0]));
      if (angle < bestAngle) {
        bestAngle = angle;
        bridge = k;
      }
    }
    const at = hole.indexOf(m);
    const ring = [...hole.slice(at), ...hole.slice(0, at)];
    poly = [...poly.slice(0, bridge + 1), ...ring, m, poly[bridge]!, ...poly.slice(bridge + 1)];
  }
  return { points, poly };
}

const outer: Vec2[] = [[-2, 0.06], [2, 0.06], [2, 1.26], [-2, 1.26]];
const h1: Vec2[] = ([[-1.965, 0.095], [-0.015, 0.095], [-0.015, 1.225], [-1.965, 1.225]] as Vec2[]).reverse();
const h2: Vec2[] = ([[0.015, 0.095], [1.965, 0.095], [1.965, 1.225], [0.015, 1.225]] as Vec2[]).reverse();
const r = bridged(outer, [h1, h2]);
console.log(r.poly.map((i) => `${i}:${r.points[i]!.join(",")}`).join("\n"));
