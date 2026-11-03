import { load, stateAt, three, transformShape } from "./lib.ts";
const { asset: a } = load("miscanthus-grass");
const m = () => { const s = stateAt(a, 100); const f = three.extentOf(s.shapes.front), p = three.extentOf(s.shapes.plan); return `fr ${f.minX.toFixed(2)}..${f.maxX.toFixed(2)} y ${f.maxY.toFixed(3)} pl ${p.minX.toFixed(2)}..${p.maxX.toFixed(2)}`; };
console.log("now", m());
for (const s of a.representations["100"].geometry.front) if (/^plume-\d$/.test(s.bone)) transformShape(s, { sx: 1 / 1.3, tx: 0, sy: 1, ty: 0 });
console.log("unscaled front plumes", m());
