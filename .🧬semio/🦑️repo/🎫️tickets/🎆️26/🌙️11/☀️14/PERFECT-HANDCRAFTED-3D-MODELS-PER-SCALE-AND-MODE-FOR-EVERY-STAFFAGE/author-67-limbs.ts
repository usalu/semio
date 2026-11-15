import { Kit, bboxOfShapes } from "./author-02-lib.ts";
const k = new Kit(process.argv[2]!, Number(process.argv[3]));
const f = (b: any) => `a[${b.minX.toFixed(2)},${b.maxX.toFixed(2)}] y[${b.minY.toFixed(2)},${b.maxY.toFixed(2)}]`;
for (const id of (process.argv[4] ?? "limb-w1,limb-e1,limb-w3").split(",")) {
  console.log(id, "E", f(bboxOfShapes(k.E((s) => s.id === id))), "F", f(bboxOfShapes(k.F((s) => s.id === id))), "Pcap", f(bboxOfShapes(k.P((s) => s.id === `plan-${id}`))));
}
