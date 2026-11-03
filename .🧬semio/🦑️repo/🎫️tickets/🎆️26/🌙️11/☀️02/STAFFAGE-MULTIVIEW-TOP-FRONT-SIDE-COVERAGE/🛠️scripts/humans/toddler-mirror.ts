import { load, save } from "./lib.ts";
const d = load("parent-carrying-toddler"); const a = d.asset;
const r5 = (n: number) => +n.toFixed(5);
let n = 0;
for (const r of Object.values<any>(a.representations)) {
  if (r.status !== "authored") continue;
  for (const v of ["front", "plan"] as const) {
    const suit = r.geometry[v].find((s: any) => s.id === "toddler-suit");
    const lat = (s: any): number[] => s.kind === "poly" ? s.points.map((p: number[]) => p[v === "front" ? 0 : 1]) : s.kind === "ellipse" ? (v === "front" ? [s.cx - s.rx, s.cx + s.rx] : [s.cy - s.ry, s.cy + s.ry]) : s.kind === "capsule" ? (v === "front" ? [s.x0 - s.r0, s.x1 + s.r1, s.x0 + s.r0, s.x1 - s.r1] : [s.y0 - s.r0, s.y1 + s.r1, s.y0 + s.r0, s.y1 - s.r1]) : [0];
    const xs = lat(suit); const c = (Math.min(...xs) + Math.max(...xs)) / 2;
    for (const s of r.geometry[v]) {
      if (!s.bone.startsWith("toddler-") || s.id === "toddler-suit") continue;
      const f = (x: number) => r5(2 * c - x);
      if (s.kind === "poly") for (const p of s.points) p[v === "front" ? 0 : 1] = f(p[v === "front" ? 0 : 1]);
      else if (s.kind === "ellipse") { if (v === "front") s.cx = f(s.cx); else s.cy = f(s.cy); }
      else if (s.kind === "capsule") { if (v === "front") { s.x0 = f(s.x0); s.x1 = f(s.x1); } else { s.y0 = f(s.y0); s.y1 = f(s.y1); } }
      else if (s.kind === "rect") { if (v === "front") s.x = f(s.x + s.w); else s.y = f(s.y + s.h); }
      n++;
    }
  }
}
console.log("toddler limbs mirrored about the toddler centre line:", n); save("parent-carrying-toddler", d);
