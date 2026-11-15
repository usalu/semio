import { Kit, bboxOfShapes, areaOfShapes } from "./author-02-lib.ts";
const [spec, S] = process.argv.slice(2);
const k = new Kit(spec!, Number(S));
const f = (n: number) => n.toFixed(2);
for (const view of ["elevation", "front", "plan"] as const) {
  console.log("==", view);
  for (const e of k.shapes(view, () => true, ["body", "detail", "glazing", "texture"])) {
    const sh = [e.rings];
    const b = bboxOfShapes(sh as any);
    console.log(e.shape.id, e.shape.bone, e.shape.region ?? "-", e.shape.role, "pts", e.rings.reduce((n: number, r: any) => n + r.length, 0), `x[${f(b.minX)},${f(b.maxX)}] y[${f(b.minY)},${f(b.maxY)}]`, "A", f(areaOfShapes(sh as any)));
  }
}
const src = JSON.parse(require("fs").readFileSync(`volumes/${spec}.json`, "utf8")).volumes[S!];
console.log("source counts", src.body?.length, src.thermal?.length, src.structure?.length);
