import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const R = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture";
const ids = "bar-stool,bookshelf,cafe-chair,dining-table,folding-deck-chair,height-adjustable-desk,office-swivel-chair,operating-table,park-bench,picnic-table,single-bed,three-seat-sofa".split(",");
type P = { x: number; y: number };
const inside = (p: P, ring: P[]) => { let c = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) { const a = ring[i]!, b = ring[j]!; if ((a.y > p.y) !== (b.y > p.y) && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) c = !c; } return c; };
for (const id of ids) {
  const asset = JSON.parse(readFileSync(`${R}/${id}/🔣️.json`, "utf8")).asset;
  for (const [sc, rep] of Object.entries<any>(asset.representations)) {
    if (rep.status !== "authored") continue;
    const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose } as any, { kind: "scale", scale: { denominator: Number(sc) } } as any);
    for (const v of ["elevation", "front", "plan"] as const) {
      const list = st.shapes[v] as any[];
      for (const u of list) {
        if (u.shape.filled || u.shape.role === "ground") continue;
        const pts = u.rings.flat() as P[];
        if (pts.length === 0) continue;
        for (const f of list) {
          if (!f.shape.filled || f.shape.z <= u.shape.z || f === u) continue;
          const reg = (s: any) => (s.region ? [s.region] : s.regions ?? []);
          if (!reg(u.shape).some((r: string) => reg(f.shape).includes(r))) continue;
          const n = pts.filter((p) => f.rings.some((r: P[]) => inside(p, r))).length;
          if (n / pts.length > 0.5) { console.log(`${id} 1:${sc} ${v} detail ${u.shape.id} (z ${u.shape.z}) under ${f.shape.id} (z ${f.shape.z}) ${(100 * n / pts.length).toFixed(0)}%`); break; }
        }
      }
    }
  }
}
