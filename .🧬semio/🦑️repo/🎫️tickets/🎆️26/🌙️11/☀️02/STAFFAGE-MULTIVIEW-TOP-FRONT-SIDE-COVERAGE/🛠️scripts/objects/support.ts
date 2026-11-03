const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { readdirSync, readFileSync } = await import("node:fs");
for (const cat of ["🪑️objects", "🛋️furniture"]) for (const slug of readdirSync(`${L}/🗂️catalog/${cat}`)) {
  const a = JSON.parse(readFileSync(`${L}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
  const sup = a.anchors.filter((x: any) => x.kind === "support" || x.kind === "contact");
  const out: string[] = [];
  for (const k of Object.keys(a.representations)) {
    const r = a.representations[k]; if (r.status !== "authored") continue;
    const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: +k } });
    const miss: string[] = [];
    for (const an of sup) {
      const w = st.anchors[an.id]; if (!w) continue;
      for (const [v, px, py] of [["front", 0, w.at.y], ["plan", w.at.x, 0]] as any) {
        let ok = false; const tol = 0.06;
        for (const s of st.shapes[v]) { if (s.shape.role === "ground") continue; for (const ring of s.rings) { for (const p of ring) { if (Math.abs(p.x - px) < tol * 3 && Math.abs(p.y - py) < tol * 3) ok = true; } } }
        const inB = st.shapes[v].some((s: any) => s.shape.role !== "ground" && s.rings.flat().length && (() => { const P = s.rings.flat(); return px >= Math.min(...P.map((q: any) => q.x)) - tol && px <= Math.max(...P.map((q: any) => q.x)) + tol && (v === "front" ? true : py >= Math.min(...P.map((q: any) => q.y)) - tol && py <= Math.max(...P.map((q: any) => q.y)) + tol); })());
        if (!inB) miss.push(`${an.id}@${v}`);
      }
    }
    out.push(`1:${k} ${miss.length ? "MISS " + miss.join(",") : "ok"}`);
  }
  console.log(slug.padEnd(22), sup.map((x: any) => `${x.id}(${x.bone},${x.x},${x.y})`).join(" "), "|", out.join(" ; "));
}
