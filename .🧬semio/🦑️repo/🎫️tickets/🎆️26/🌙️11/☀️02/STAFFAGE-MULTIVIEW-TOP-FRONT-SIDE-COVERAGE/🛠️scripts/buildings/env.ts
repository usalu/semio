import { readFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const T = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/👤️humans/${process.argv[2]}/🔣️.json`, "utf8")).asset;
console.log(JSON.stringify(a.dimensions), "default", a.presentation.defaultPose);
for (const sc of [50, 100]) {
  let all = [1e9, -1e9, 1e9, -1e9, 1e9, -1e9];
  const per: string[] = [];
  for (const { name, query } of T.queries(a)) {
    const st = evaluate(a, query, { kind: "scale", scale: { denominator: sc } });
    const e = T.extentOf(st.shapes.elevation), f = T.extentOf(st.shapes.front), p = T.extentOf(st.shapes.plan);
    if (!e || !f || !p) continue;
    if (name === `pose ${a.presentation.defaultPose}`) per.push(`default x[${e.minX.toFixed(3)},${e.maxX.toFixed(3)}]=${(e.maxX - e.minX).toFixed(3)} y=${(e.maxY - e.minY).toFixed(3)} z=${(f.maxX - f.minX).toFixed(3)} planx=${(p.maxX - p.minX).toFixed(3)}`);
    all = [Math.min(all[0], e.minX), Math.max(all[1], e.maxX), Math.min(all[2], f.minX), Math.max(all[3], f.maxX), Math.min(all[4], e.minY), Math.max(all[5], e.maxY)];
  }
  console.log(sc, per.join(" | "), `ENVELOPE x ${(all[1] - all[0]).toFixed(3)} z ${(all[3] - all[2]).toFixed(3)} y ${(all[5] - all[4]).toFixed(3)}`);
}
