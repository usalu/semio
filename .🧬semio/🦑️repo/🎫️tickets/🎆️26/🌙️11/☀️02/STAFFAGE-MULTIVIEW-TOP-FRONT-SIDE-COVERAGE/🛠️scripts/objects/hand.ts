const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { readdirSync, readFileSync } = await import("node:fs");
const side = (id: string) => (/(^|-)(l|left)(-|\d|$)/.test(id) ? 1 : /(^|-)(r|right)(-|\d|$)/.test(id) ? -1 : 0);
for (const cat of ["🪑️objects", "🛋️furniture"]) for (const slug of readdirSync(`${L}/🗂️catalog/${cat}`)) {
  const a = JSON.parse(readFileSync(`${L}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
  const bad: string[] = [];
  for (const k of Object.keys(a.representations)) {
    const r = a.representations[k]; if (r.status !== "authored") continue;
    const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: +k } });
    for (const [view, axis] of [["front", "x"], ["plan", "y"]] as const) for (const s of st.shapes[view]) {
      const sd = side(s.shape.id); if (!sd) continue;
      const p = s.rings.flat(); if (!p.length) continue;
      const c = p.reduce((t: number, q: any) => t + q[axis], 0) / p.length;
      if (Math.abs(c) > 0.01 && c * sd < 0) bad.push(`${k}/${view}/${s.shape.id}:${c.toFixed(2)}`);
    }
  }
  if (bad.length) console.log(slug, bad.join(" "));
}
