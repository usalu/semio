const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { extentOf } = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const { readFileSync, existsSync } = await import("node:fs");
for (const slug of process.argv.slice(2)) {
  const f = ["🪑️objects", "🛋️furniture"].map((c) => `${L}/🗂️catalog/${c}/${slug}/🔣️.json`).find(existsSync)!;
  const a = JSON.parse(readFileSync(f, "utf8")).asset;
  const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } });
  const r = (e: any) => e && [e.minX, e.maxX, e.minY, e.maxY].map((n: number) => +n.toFixed(3)).join(" ");
  console.log(slug.padEnd(20), "E", r(extentOf(st.shapes.elevation)), "| F", r(extentOf(st.shapes.front)), "| P", r(extentOf(st.shapes.plan)));
}
