/** 📐️ Fauna: world <-> local mapping of any 2D `poly` shape (front, plan or elevation) fitted from its own points and drawn ring (least-squares affine), so a repair can be written in world coordinates for every view.
 * @example const m = polyMap(ctx, "front", "tail"); m.toLocal([[0.04, 0.05]]) */
import { drawn, type Ctx } from "./kit-lib.ts";
export type P = [number, number];
export function polyMap(c: Ctx, view: "elevation" | "front" | "plan", id: string, points: P[]): { toLocal: (w: P[]) => P[]; err: number } {
  const sh = drawn(c, view).find((s) => s.id === id);
  if (sh === undefined) throw new Error(`no ${view} shape ${id}`);
  const ring = sh.rings[0]!;
  if (ring.length !== points.length) throw new Error(`${view} ${id}: ring ${ring.length} != points ${points.length}`);
  const n = points.length;
  const solve = (rhs: number[]): number[] => {
    const A = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    const b = [0, 0, 0];
    for (let i = 0; i < n; i++) { const r = [points[i]![0], points[i]![1], 1]; for (let a = 0; a < 3; a++) { for (let k = 0; k < 3; k++) A[a]![k]! += r[a]! * r[k]!; b[a]! += r[a]! * rhs[i]!; } }
    for (let i = 0; i < 3; i++) { let p = i; for (let k = i + 1; k < 3; k++) if (Math.abs(A[k]![i]!) > Math.abs(A[p]![i]!)) p = k; [A[i], A[p]] = [A[p]!, A[i]!]; [b[i], b[p]] = [b[p]!, b[i]!]; for (let k = i + 1; k < 3; k++) { const f = A[k]![i]! / A[i]![i]!; for (let j = i; j < 3; j++) A[k]![j]! -= f * A[i]![j]!; b[k]! -= f * b[i]!; } }
    const x = [0, 0, 0];
    for (let i = 2; i >= 0; i--) { let s = b[i]!; for (let j = i + 1; j < 3; j++) s -= A[i]![j]! * x[j]!; x[i] = s / A[i]![i]!; }
    return x;
  };
  const ax = solve(ring.map((p) => p.x));
  const ay = solve(ring.map((p) => p.y));
  let err = 0;
  for (let i = 0; i < n; i++) { const [u, v] = points[i]!; err = Math.max(err, Math.hypot(ax[0]! * u + ax[1]! * v + ax[2]! - ring[i]!.x, ay[0]! * u + ay[1]! * v + ay[2]! - ring[i]!.y)); }
  const det = ax[0]! * ay[1]! - ax[1]! * ay[0]!;
  const toLocal = (w: P[]): P[] => w.map(([x, y]) => { const dx = x - ax[2]!; const dy = y - ay[2]!; return [Math.round(((ay[1]! * dx - ax[1]! * dy) / det) * 1e5) / 1e5, Math.round(((-ay[0]! * dx + ax[0]! * dy) / det) * 1e5) / 1e5] as P; });
  return { toLocal, err };
}
