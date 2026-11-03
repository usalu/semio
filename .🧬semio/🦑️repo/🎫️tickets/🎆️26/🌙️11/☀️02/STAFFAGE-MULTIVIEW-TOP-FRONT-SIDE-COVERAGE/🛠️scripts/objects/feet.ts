const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { readFileSync, existsSync } = await import("node:fs");
for (const slug of process.argv.slice(2)) {
  const f = ["🪑️objects", "🛋️furniture"].map((c) => `${L}/🗂️catalog/${c}/${slug}/🔣️.json`).find(existsSync)!;
  const a = JSON.parse(readFileSync(f, "utf8")).asset;
  const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } });
  console.log("##", slug, "pose", a.presentation.defaultPose);
  for (const view of ["elevation", "front"] as const) {
    const rows = st.shapes[view].filter((s: any) => s.shape.role !== "ground").map((s: any) => { const p = s.rings.flat(); const low = p.filter((q: any) => q.y < 0.03); return low.length ? `${s.shape.id}[${s.shape.bone}] x ${Math.min(...low.map((q: any) => q.x)).toFixed(2)}..${Math.max(...low.map((q: any) => q.x)).toFixed(2)}` : null; }).filter(Boolean);
    console.log(" ", view, rows.join(" | "));
  }
  const pl = st.shapes.plan.filter((s: any) => /leg|foot|wheel|castor|plinth|base|rail/.test(s.shape.id)).map((s: any) => { const p = s.rings.flat(); return `${s.shape.id} x ${Math.min(...p.map((q: any) => q.x)).toFixed(2)}..${Math.max(...p.map((q: any) => q.x)).toFixed(2)} z ${Math.min(...p.map((q: any) => q.y)).toFixed(2)}..${Math.max(...p.map((q: any) => q.y)).toFixed(2)}`; });
  console.log("  plan", pl.slice(0, 14).join(" | "));
  for (const an of a.anchors.filter((x: any) => x.kind === "contact" || x.kind === "support")) { const w = st.anchors[an.id]; console.log("  anchor", an.id, w.at.x.toFixed(2), w.at.y.toFixed(2), "fp", an.footprintLengthM, an.footprintWidthM); }
}
