const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { readdirSync, readFileSync } = await import("node:fs");
for (const cat of ["🪑️objects", "🛋️furniture"]) for (const slug of readdirSync(`${L}/🗂️catalog/${cat}`)) {
  const a = JSON.parse(readFileSync(`${L}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
  for (const e of a.thermal?.emitters ?? []) {
    const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } });
    for (const id of [e.anchor, ...(e.anchors ?? [])]) {
      const an = st.anchors[id]; const fr = st.bones[an.anchor.bone].frame;
      const t = (e.directionDeg * Math.PI) / 180, c = Math.cos(t), s = Math.sin(t);
      const d = { x: c * fr.u.x + s * fr.v.x, y: c * fr.u.y + s * fr.v.y, z: c * fr.u.z + s * fr.v.z };
      console.log(slug.padEnd(20), e.id.padEnd(18), e.kind.padEnd(8), id, an.anchor.bone, "dir", e.directionDeg, "=>", d.x.toFixed(2), d.y.toFixed(2), d.z.toFixed(2), d.y > 0.7 ? "UP" : "CHECK");
    }
  }
}
