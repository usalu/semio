const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { extentOf } = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const { readFileSync, existsSync } = await import("node:fs");
for (const slug of process.argv.slice(2)) {
  const f = ["🪑️objects", "🛋️furniture"].map((c) => `${L}/🗂️catalog/${c}/${slug}/🔣️.json`).find(existsSync)!;
  const a = JSON.parse(readFileSync(f, "utf8")).asset;
  const d = a.dimensions;
  for (const sy of a.symbols) {
    const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "symbol", symbol: sy.id });
    const e = extentOf(st.shapes.elevation), fr = extentOf(st.shapes.front), p = extentOf(st.shapes.plan);
    const w = (x: any, ax: "X" | "Y") => (x ? +(x["max" + ax] - x["min" + ax]).toFixed(3) : "none");
    console.log(slug, sy.id, sy.semantics, "dims w/d/h", d.widthM, d.depthM, d.heightM, "| E w,h", w(e, "X"), w(e, "Y"), "| F d,h", w(fr, "X"), w(fr, "Y"), "| P w,d", w(p, "X"), w(p, "Y"));
  }
}
