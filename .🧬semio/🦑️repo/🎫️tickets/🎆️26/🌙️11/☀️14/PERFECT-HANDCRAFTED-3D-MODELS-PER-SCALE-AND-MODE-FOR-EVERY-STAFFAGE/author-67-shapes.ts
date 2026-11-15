import { Kit } from "./author-02-lib.ts";
const k = new Kit(process.argv[2]!, Number(process.argv[3]));
for (const v of ["elevation", "front", "plan"] as const) for (const e of k.shapes(v, () => true, ["body", "detail", "glazing", "texture"])) {
  const pts = e.rings.flat(); if (!pts.length) continue;
  const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
  console.log(v, e.shape.id, e.shape.role, e.shape.bone, e.shape.region ?? "-", e.shape.kind, `x[${Math.min(...xs).toFixed(2)},${Math.max(...xs).toFixed(2)}] y[${Math.min(...ys).toFixed(2)},${Math.max(...ys).toFixed(2)}]`, e.closed ? "closed" : "open");
}
