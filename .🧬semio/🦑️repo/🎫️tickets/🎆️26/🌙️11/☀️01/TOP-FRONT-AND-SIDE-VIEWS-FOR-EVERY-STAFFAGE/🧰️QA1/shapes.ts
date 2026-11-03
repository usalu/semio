/** [DEBUG] QA1: per-shape bbox per view. usage: bun shapes.ts <asset> <scales,> <poses,> */
import { join, resolve } from "node:path";
const S = join(resolve(import.meta.dir, "../../../../../../../.."), "🎓️teaching/🏛️architecture/🧍️staffage");
const { catalog } = await import(join(S, "🧪️tests/🧊️three-views/🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const [id, scales, poses] = process.argv.slice(2);
const asset = (catalog() as any[]).find((a) => a.id === id);
const f = (n: number) => n.toFixed(3);
for (const sc of scales!.split(",")) for (const pose of poses!.split(",")) {
  const q = pose.startsWith("clip:") ? { kind: "clip", clip: pose.slice(5).split("@")[0], timeS: Number(pose.split("@")[1]) } : { kind: "pose", pose };
  const s = evaluate(asset, q, { kind: "scale", scale: { denominator: Number(sc) } });
  console.log(`## 1:${sc} ${pose}`);
  for (const v of ["elevation", "front", "plan"] as const) {
    for (const e of s.shapes[v]) {
      if (e.shape.role === "ground") continue;
      const pts = e.rings.flat();
      if (!pts.length) continue;
      const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
      console.log(`${v[0]} ${e.shape.id} (${e.shape.bone}) x[${f(Math.min(...xs))},${f(Math.max(...xs))}] y[${f(Math.min(...ys))},${f(Math.max(...ys))}]`);
    }
  }
}
