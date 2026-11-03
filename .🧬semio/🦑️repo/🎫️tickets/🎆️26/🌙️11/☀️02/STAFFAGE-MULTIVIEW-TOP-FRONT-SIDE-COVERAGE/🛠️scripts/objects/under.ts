const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { queries } = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const { readFileSync } = await import("node:fs");
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/🪑️objects/market-parasol/🔣️.json`, "utf8")).asset;
const hemSlope = Math.atan2(-0.1035, 0.3465) * 180 / Math.PI;
let worst = -999, where = "";
for (const { name, query } of queries(a)) {
  const st = evaluate(a, query, { kind: "scale", scale: { denominator: 50 } });
  for (const [id, sign] of [["rib-right", 1], ["rib-left", -1]] as const) {
    const b = st.bones[id];
    const dir = Math.atan2(b.tip.y - b.origin.y, (b.tip.x - b.origin.x) * sign) * 180 / Math.PI;
    const tangent = dir + hemSlope;
    if (tangent > worst) { worst = tangent; where = `${name} ${id} chord ${dir.toFixed(1)}`; }
  }
}
console.log("hem slope relative to the chord", hemSlope.toFixed(1), "deg; steepest outward-rising hem tangent over all poses and clips:", worst.toFixed(1), "deg at", where);
