import { fileOf, patchView } from "./patch.ts";

const cl = (v: number, lim: number): number => Math.max(-lim, Math.min(lim, v));
const interval = (lo: number, size: number, lim: number): [number, number] => {
  const a = cl(lo, lim);
  const b = cl(lo + size, lim);
  return [a, b - a];
};
const r5 = (n: number): number => +n.toFixed(5);

export function clampLateral(shapes: any[], view: "front" | "plan", lim: number, only?: (s: any) => boolean): any[] {
  return shapes.map((s) => {
    if (s.role === "ground" || (only !== undefined && !only(s))) return s;
    const out = { ...s };
    if (s.solid?.kind === "extruded") {
      out.solid = { ...s.solid, z0: r5(cl(s.solid.z0, lim)), z1: r5(cl(s.solid.z1, lim)) };
      return out;
    }
    if (s.solid !== undefined) return s;
    if (s.kind === "poly") out.points = s.points.map(([x, y]: number[]) => (view === "front" ? [r5(cl(x, lim)), y] : [x, r5(cl(y, lim))]));
    else if (s.kind === "rect") {
      if (view === "front") {
        const [x, w] = interval(s.x, s.w, lim);
        out.x = r5(x);
        out.w = r5(w);
      } else {
        const [y, h] = interval(s.y, s.h, lim);
        out.y = r5(y);
        out.h = r5(h);
      }
    } else if (s.kind === "ellipse") {
      if (view === "front") out.cx = r5(cl(s.cx, lim - s.rx));
      else out.cy = r5(cl(s.cy, lim - s.ry));
    } else if (s.kind === "capsule" && view === "plan") {
      out.y0 = r5(cl(s.y0, lim - s.r0));
      out.y1 = r5(cl(s.y1, lim - s.r1));
    }
    return out;
  });
}

const jobs: [string, string, number[], number][] = [
  ["🪑️objects", "retractable-bollard", [200, 500], 0.2],
  ["🪑️objects", "waste-bin", [50, 100, 200, 500], 0.25],
  ["🪑️objects", "playground-swing", [50, 100, 200, 500], 0.6],
  ["🪑️objects", "charcoal-grill", [200, 500], 0.32],
  ["🪑️objects", "bicycle-stand", [500], 0.025],
  ["🛋️furniture", "office-swivel-chair", [50, 100, 200, 500], 0.3],
  ["🛋️furniture", "cafe-chair", [50, 100, 200, 500], 0.252],
];
for (const [cat, slug, bands, lim] of jobs)
  for (const band of bands) for (const view of ["front", "plan"] as const) patchView(fileOf(cat, slug), band, view, (shapes) => clampLateral(shapes, view, lim));
