import { Kit, bboxOfShapes } from "./author-02-lib.ts";
const k = new Kit(process.argv[2]!, Number(process.argv[3]));
const f = (b: any) => `x[${b.minX.toFixed(2)},${b.maxX.toFixed(2)}] y[${b.minY.toFixed(2)},${b.maxY.toFixed(2)}]`;
for (const v of ["elevation", "front", "plan"] as const) {
  for (const e of k.shapes(v, (s) => s.role === "body" && /fol|^f-|^p-|crown|pl-|lobe|crn/.test(s.id))) {
    const rings = e.rings; const xs = rings.flat().map((p: any) => p.x), ys = rings.flat().map((p: any) => p.y);
    console.log(v, e.shape.id, e.shape.bone, e.shape.region, f({ minX: Math.min(...xs), maxX: Math.max(...xs), minY: Math.min(...ys), maxY: Math.max(...ys) }));
  }
}
