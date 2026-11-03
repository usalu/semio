const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { readFileSync } = await import("node:fs");
const [slug, cat, scale, view, query, filt] = process.argv.slice(2);
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
const q = query.startsWith("clip:") ? { kind: "clip", clip: query.slice(5).split("@")[0], timeS: +query.split("@")[1] } : { kind: "pose", pose: query };
const st = evaluate(a, q, { kind: "scale", scale: { denominator: +scale } });
for (const s of st.shapes[view]) {
  if (filt && !(s.shape.id.match(filt) || s.shape.bone.match(filt))) continue;
  const p = s.rings.flat(); if (!p.length) continue;
  const f = (n: number) => n.toFixed(3);
  console.log(s.shape.id.padEnd(20), s.shape.bone.padEnd(12), "x", f(Math.min(...p.map((q: any) => q.x))), f(Math.max(...p.map((q: any) => q.x))), "y", f(Math.min(...p.map((q: any) => q.y))), f(Math.max(...p.map((q: any) => q.y))));
}
for (const [k, v] of Object.entries<any>(st.bones)) if (!filt) console.log("bone", k, v.origin.x.toFixed(3), v.origin.y.toFixed(3));
