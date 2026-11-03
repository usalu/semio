/** [DEBUG] QA1: bbox of one shape id in every pose and view. usage: bun neck.ts <asset> <scale> <shapeId> */
import { join, resolve } from "node:path";
const S = join(resolve(import.meta.dir, "../../../../../../../.."), "🎓️teaching/🏛️architecture/🧍️staffage");
const { catalog } = await import(join(S, "🧪️tests/🧊️three-views/🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const [id, sc, shape] = process.argv.slice(2);
const asset = (catalog() as any[]).find((a) => a.id === id);
for (const pose of asset.poses.filter((p: any) => !p.key)) {
  const st = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: Number(sc) } });
  for (const v of ["elevation", "front", "plan"] as const) for (const e of st.shapes[v]) if (e.shape.id === shape) {
    const pts = e.rings.flat(); const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
    console.log(`${pose.id} ${v} w=${(Math.max(...xs) - Math.min(...xs)).toFixed(4)} h=${(Math.max(...ys) - Math.min(...ys)).toFixed(4)}`);
  }
}
