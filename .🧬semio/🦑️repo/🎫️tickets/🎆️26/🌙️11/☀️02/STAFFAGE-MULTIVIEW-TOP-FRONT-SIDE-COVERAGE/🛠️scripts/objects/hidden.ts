const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { readdirSync, readFileSync } = await import("node:fs");
const inside = (px: number, py: number, ring: { x: number; y: number }[]): boolean => {
  let c = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if (ring[i]!.y > py !== ring[j]!.y > py && px < ((ring[j]!.x - ring[i]!.x) * (py - ring[i]!.y)) / (ring[j]!.y - ring[i]!.y) + ring[i]!.x) c = !c;
  return c;
};
const near = (px: number, py: number, shapes: any[], tol: number): boolean =>
  shapes.some((s) => s.rings.some((ring: any[]) => (s.closed && inside(px, py, ring)) || ring.some((p) => Math.hypot(p.x - px, p.y - py) < tol)));
const feet = (shapes: any[], axis: "x" | "y", lowMax: number): [number, number][] => {
  const out: [number, number][] = [];
  for (const s of shapes) {
    if (s.shape.role === "ground") continue;
    const p = s.rings.flat();
    if (p.length === 0) continue;
    if (Math.min(...p.map((q: any) => q.y)) > lowMax) continue;
    const v = p.filter((q: any) => q.y <= lowMax).map((q: any) => q[axis]);
    out.push([Math.min(...v), Math.max(...v)]);
  }
  return out;
};
for (const cat of ["🪑️objects", "🛋️furniture"])
  for (const slug of readdirSync(`${L}/🗂️catalog/${cat}`)) {
    const a = JSON.parse(readFileSync(`${L}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
    const missing: string[] = [];
    for (const k of Object.keys(a.representations)) {
      const r = a.representations[k];
      if (r.status !== "authored") continue;
      const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: +k } });
      const fx = feet(st.shapes.elevation, "x", 0.06);
      const fz = feet(st.shapes.front, "x", 0.06);
      const hit: string[] = [];
      for (const [x0, x1] of fx) for (const [z0, z1] of fz) {
        const cx = (x0 + x1) / 2, cz = (z0 + z1) / 2;
        if (!near(cx, cz, st.shapes.plan.filter((s: any) => s.shape.role !== "ground"), 0.02)) hit.push(`(${cx.toFixed(2)},${cz.toFixed(2)})`);
      }
      if (hit.length > 0) missing.push(`1:${k} ${[...new Set(hit)].slice(0, 6).join(" ")}`);
    }
    console.log(slug.padEnd(24), missing.length === 0 ? "plan covers every foot contact" : "foot centre not drawn in plan: " + missing.join(" | "));
  }
