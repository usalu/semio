const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { readFileSync } = await import("node:fs");
const [cat, slug, band, view, region] = process.argv.slice(2);
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: +band! } });
for (const s of st.shapes[view!]) { if (region && !JSON.stringify(s.shape).includes(region)) continue; const xs=s.rings.flat().map((p:any)=>p.x), ys=s.rings.flat().map((p:any)=>p.y); console.log(s.shape.id, s.shape.role, s.closed?"c":"o", "x", Math.min(...xs).toFixed(3), Math.max(...xs).toFixed(3), "y", Math.min(...ys).toFixed(3), Math.max(...ys).toFixed(3)); }
