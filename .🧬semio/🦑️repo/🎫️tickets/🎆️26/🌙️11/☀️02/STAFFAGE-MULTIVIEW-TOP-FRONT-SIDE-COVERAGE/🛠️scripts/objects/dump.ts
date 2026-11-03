const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { readFileSync, readdirSync } = await import("node:fs");
const [cat, slug, band, pose, view] = process.argv.slice(2);
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
if (!band) { console.log(JSON.stringify(a.anchors.map((x:any)=>[x.id,x.bone,x.x,x.y,x.z,x.kind])), a.poses.map((p:any)=>p.id).join(",")); process.exit(0); }
const st = evaluate(a, { kind: "pose", pose: pose ?? a.presentation.defaultPose }, { kind: "scale", scale: { denominator: +band } });
for (const s of st.shapes[view!]) { const xs=s.rings.flat().map((p:any)=>p.x), ys=s.rings.flat().map((p:any)=>p.y); console.log(s.shape.id??s.shape.bone, s.shape.role, s.closed?"c":"o", Math.min(...xs).toFixed(3), Math.max(...xs).toFixed(3), Math.min(...ys).toFixed(3), Math.max(...ys).toFixed(3)); }
