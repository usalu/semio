type P = [number, number];
const r4 = (n: number) => +n.toFixed(4);
function dp(pts: P[], eps: number): P[] {
  if (pts.length < 3) return pts;
  const [a, b] = [pts[0]!, pts[pts.length - 1]!];
  let idx = -1, dmax = 0;
  for (let i = 1; i < pts.length - 1; i++) {
    const p = pts[i]!; const dx = b[0] - a[0], dy = b[1] - a[1]; const L = Math.hypot(dx, dy) || 1e-12;
    const d = Math.abs(dy * p[0] - dx * p[1] + b[0] * a[1] - b[1] * a[0]) / L;
    if (d > dmax) { dmax = d; idx = i; }
  }
  if (dmax > eps) return [...dp(pts.slice(0, idx + 1), eps).slice(0, -1), ...dp(pts.slice(idx), eps)];
  return [a, b];
}
export function simplifyClosed(pts: P[], eps: number): P[] {
  if (pts.length < 5) return pts.map((p) => [r4(p[0]), r4(p[1])]);
  let far = 0, fd = -1; for (let i = 1; i < pts.length; i++) { const d = Math.hypot(pts[i]![0] - pts[0]![0], pts[i]![1] - pts[0]![1]); if (d > fd) { fd = d; far = i; } }
  const h1 = dp(pts.slice(0, far + 1), eps), h2 = dp([...pts.slice(far), pts[0]!], eps);
  return [...h1.slice(0, -1), ...h2.slice(0, -1)].map((p) => [r4(p[0]), r4(p[1])]);
}
export function simplifyOpen(pts: P[], eps: number): P[] { return dp(pts, eps).map((p) => [r4(p[0]), r4(p[1])]); }
export function flattenPath(d: string): { rings: P[][]; closed: boolean } {
  const t = d.match(/[MLCZHVmlczhv]|-?\d*\.?\d+(?:e-?\d+)?/g) ?? []; const rings: P[][] = []; let cur: P[] = []; let i = 0; let cmd = ""; let x = 0, y = 0; let closed = false;
  const num = () => parseFloat(t[i++]!);
  while (i < t.length) {
    if (/[A-Za-z]/.test(t[i]!)) cmd = t[i++]!;
    if (cmd === "M") { if (cur.length) rings.push(cur); x = num(); y = num(); cur = [[x, y]]; cmd = "L"; }
    else if (cmd === "L") { x = num(); y = num(); cur.push([x, y]); }
    else if (cmd === "H") { x = num(); cur.push([x, y]); }
    else if (cmd === "V") { y = num(); cur.push([x, y]); }
    else if (cmd === "C") { const x1 = num(), y1 = num(), x2 = num(), y2 = num(), x3 = num(), y3 = num(); for (let k = 1; k <= 8; k++) { const u = k / 8, v = 1 - u; cur.push([v * v * v * x + 3 * v * v * u * x1 + 3 * v * u * u * x2 + u * u * u * x3, v * v * v * y + 3 * v * v * u * y1 + 3 * v * u * u * y2 + u * u * u * y3]); } x = x3; y = y3; }
    else if (cmd === "Z") { closed = true; i += 0; cmd = ""; if (i < t.length && !/[A-Za-z]/.test(t[i]!)) break; }
    else break;
  }
  if (cur.length) rings.push(cur);
  return { rings, closed };
}
export function simplifyShape(s: any, eps: number): void {
  if (s.kind === "poly") { s.points = s.closed ? simplifyClosed(s.points, eps) : simplifyOpen(s.points, eps); return; }
  if (s.kind === "path") {
    const { rings, closed } = flattenPath(s.d);
    s.d = rings.map((ring) => { const q = closed ? simplifyClosed(ring, eps) : simplifyOpen(ring, eps); return "M" + q.map((p) => `${p[0]} ${p[1]}`).join(" L") + (closed ? " Z" : ""); }).join(" ");
  }
}
