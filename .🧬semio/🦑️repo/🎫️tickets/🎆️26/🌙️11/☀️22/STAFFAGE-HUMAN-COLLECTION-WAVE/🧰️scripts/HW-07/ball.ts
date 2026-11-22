/** 🏀️ The basketball of `basketball-player`: a size 7 ball (FIBA: circumference 749 to 780 mm, 567 to 650 g; here 750 mm and 620 g) on the `ball`
 * bone, drawn per scale with the panel seams it shows from the side, from ahead and from above (the ball translates, it never turns, so flat
 * drawings stay true in every pose).
 * @see https://about.fiba.basketball/en/rules — FIBA Official Basketball Rules, Basketball Equipment (ball size 7) */
import { r, tx, type Json } from "./lib.ts";

export const BALL = { radius: 0.1194, massKg: 0.62 };
const R = BALL.radius;
const circle = (rad: number, n = 40, cx = 0, cy = 0): number[][] => Array.from({ length: n }, (_, i) => [r(cx + rad * Math.cos((2 * Math.PI * i) / n), 5), r(cy + rad * Math.sin((2 * Math.PI * i) / n), 5)]);
/** 〰️ A seam seen on the ball: the ellipse arc x = a cos t, y = R sin t (a great circle tilted against the view), as an open polyline. */
const arc = (a: number, n = 16): number[][] => Array.from({ length: n + 1 }, (_, i) => { const t = -Math.PI / 2 + (i / n) * Math.PI; return [r(a * Math.cos(t), 5), r(R * 0.985 * Math.sin(t), 5)]; });
const line = (x0: number, y0: number, x1: number, y1: number): number[][] => [[r(x0, 5), r(y0, 5)], [r(x1, 5), r(y1, 5)]];

/** ✏️ The ball shapes of one scale and view; `centre` is the paper position of the ball centre at rest (front and plan shapes are rest-world outlines, elevation shapes bone-local). */
export function ballShapes(scale: string, view: string, centre: [number, number]): Json[] {
  const [cx, cy] = centre;
  const shift = (pts: number[][]) => pts.map((p) => [r(p[0]! + cx, 5), r(p[1]! + cy, 5)]);
  const base = { bone: "ball", region: "ball", material: "ball-composite" };
  const disc = { ...base, id: "ball", role: "body", z: 9, filled: true, tone: scale === "500" ? 0 : 0.42, kind: "poly", points: shift(circle(R, scale === "500" ? 24 : 40)), closed: true };
  if (scale === "500") return [disc];
  const seam = (id: string, points: number[][]) => ({ ...base, id, role: scale === "200" ? "detail" : "detail", z: 9.1, filled: false, tone: 0.1, kind: "poly", points: shift(points), closed: false });
  if (scale === "200") return [disc, seam("ball-seam", line(-R * 0.98, 0, R * 0.98, 0))];
  const curved = view === "plan" ? [seam("ball-seam-a", arc(R * 0.55)), seam("ball-seam-b", arc(-R * 0.55))] : [seam("ball-seam-a", arc(R * 0.62)), seam("ball-seam-b", arc(-R * 0.62))];
  const straight = view === "front" ? seam("ball-seam-c", line(0, -R * 0.985, 0, R * 0.985)) : seam("ball-seam-c", line(-R * 0.985, 0, R * 0.985, 0));
  return scale === "50" ? [disc, ...curved, straight] : [disc, straight, curved[0]!];
}

/** 🧊️ The ball solids of one volume scale. */
export function ballVolume(scale: string): { body: Json[]; thermal: Json[]; structure: Json[] } {
  const body = [{ id: "ball", kind: "ellipsoid", bone: "ball", material: "ball-composite", role: "body", tone: scale === "500" ? 0 : 0.42, radii: [R, R, R] }];
  return { body, thermal: [{ id: "th-ball", of: "ball", region: "ball" }], structure: [{ id: "st-ball", of: "ball", member: "ball", part: "member" }] };
}

export const BALL_MATERIALS = [{ id: "ball-composite", title: tx("Composite leather ball cover over a butyl bladder", "Ballhülle aus Kompositleder über einer Butylblase"), emissivity: 0.95, solarAbsorptivity: 0.75, source: "authored-estimate" }];
