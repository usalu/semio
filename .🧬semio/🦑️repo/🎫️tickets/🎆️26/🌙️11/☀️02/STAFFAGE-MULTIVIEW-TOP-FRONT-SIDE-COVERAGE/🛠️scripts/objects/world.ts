const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { readFileSync } = await import("node:fs");
const [cat, slug, band, pose] = process.argv.slice(2);
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
const st = evaluate(a, { kind: "pose", pose: pose ?? a.presentation.defaultPose }, { kind: "scale", scale: { denominator: +(band ?? 50) } });
for (const [k, w] of Object.entries<any>(st.anchors)) console.log(k, w.at.x.toFixed(3), w.at.y.toFixed(3), (w.z ?? 0).toFixed(3));
