const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { readFileSync } = await import("node:fs");
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/🪑️objects/charcoal-grill/🔣️.json`, "utf8")).asset;
for (const variant of [{}, { "lid-vent": { angleDeg: 70 } }, { "ash-damper": { angleDeg: 70 } }, { "lid-vent": { angleDeg: 70 }, "ash-damper": { angleDeg: 70 } }]) {
  const b = JSON.parse(JSON.stringify(a)); b.poses.find((p: any) => p.id === "grilling-closed").bones = variant;
  const st = evaluate(b, { kind: "pose", pose: "grilling-closed" }, { kind: "scale", scale: { denominator: 50 } });
  const s = st.shapes.front.find((x: any) => x.shape.id === "lid"); const ys = s.rings.flat().map((p: any) => p.y);
  console.log(JSON.stringify(variant), Math.min(...ys).toFixed(3), Math.max(...ys).toFixed(3), "lid bone origin", st.bones.lid.origin.y.toFixed(3), JSON.stringify(st.bones.lid.frame.u));
}
