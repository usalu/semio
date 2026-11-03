const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { readdirSync, readFileSync } = await import("node:fs");
type P = { x: number; y: number };
const inside = (px: number, py: number, ring: P[]): boolean => {
  let c = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if (ring[i]!.y > py !== ring[j]!.y > py && px < ((ring[j]!.x - ring[i]!.x) * (py - ring[i]!.y)) / (ring[j]!.y - ring[i]!.y) + ring[i]!.x) c = !c;
  return c;
};
const seg = (px: number, py: number, a: P, b: P): number => {
  const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy;
  const t = l === 0 ? 0 : Math.max(0, Math.min(1, ((px - a.x) * dx + (py - a.y) * dy) / l));
  return Math.hypot(px - (a.x + t * dx), py - (a.y + t * dy));
};
const distance = (shapes: any[], px: number, py: number): number => {
  let best = Infinity;
  for (const s of shapes) {
    if (s.shape.role === "ground") continue;
    for (const ring of s.rings as P[][]) {
      if (s.closed && inside(px, py, ring)) return 0;
      for (let i = 0; i + 1 < ring.length; i++) best = Math.min(best, seg(px, py, ring[i]!, ring[i + 1]!));
      if (s.closed && ring.length > 2) best = Math.min(best, seg(px, py, ring[ring.length - 1]!, ring[0]!));
      if (ring.length === 1) best = Math.min(best, Math.hypot(px - ring[0]!.x, py - ring[0]!.y));
    }
  }
  return best;
};
const tol = 0.03;
let total = 0, flagged = 0;
for (const cat of ["🪑️objects", "🛋️furniture"])
  for (const slug of readdirSync(`${L}/🗂️catalog/${cat}`)) {
    const a = JSON.parse(readFileSync(`${L}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
    const bad: string[] = [];
    for (const k of Object.keys(a.representations)) {
      if (a.representations[k].status !== "authored") continue;
      const poses = [a.presentation.defaultPose, ...a.poses.map((p: any) => p.id)];
      for (const pose of new Set<string>(poses)) {
        const st = evaluate(a, { kind: "pose", pose }, { kind: "scale", scale: { denominator: +k } });
        for (const an of a.anchors) {
          const w = st.anchors[an.id];
          if (w === undefined) continue;
          total++;
          const z = w.z ?? 0;
          const d = { e: distance(st.shapes.elevation, w.at.x, w.at.y), f: distance(st.shapes.front, z, w.at.y), p: distance(st.shapes.plan, w.at.x, z) };
          const lost = (["e", "f", "p"] as const).filter((v) => d[v] > tol);
          if (lost.length > 0) {
            flagged++;
            bad.push(`1:${k}/${pose} ${an.id}[${an.kind}] at(${w.at.x.toFixed(2)},${w.at.y.toFixed(2)},z${z.toFixed(2)}) ${lost.map((v) => `${v}=${d[v].toFixed(2)}`).join(",")}`);
          }
        }
      }
    }
    const kinds = [...new Set(a.anchors.map((x: any) => x.kind))].join("/");
    const withZ = a.anchors.filter((x: any) => x.z !== undefined && x.z !== 0).length;
    console.log(`${slug.padEnd(24)} anchors ${a.anchors.length} (${kinds}) with z ${withZ}; lost in a view: ${bad.length}`);
    for (const b of bad) console.log("    ", b);
  }
console.log("checked", total, "anchor placements, outside the drawn geometry (>3 cm) in at least one view:", flagged);
