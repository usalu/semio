import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const [file, scale, ...poses] = process.argv.slice(2) as [string, string, ...string[]];
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const bounds = (rings: { x: number; y: number }[][]) => {
  const xs = rings.flat().map((p) => p.x), ys = rings.flat().map((p) => p.y);
  return [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)].map((v) => Math.round(v * 1000) / 1000);
};
for (const pose of poses) {
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: Number(scale) } });
  console.log("[DEBUG] pose", pose, "bones", Object.entries(state.bones).map(([k, b]) => `${k}:${JSON.stringify(b.origin.x.toFixed(3))},${b.origin.y.toFixed(3)}`).join(" "));
  for (const v of ["elevation", "plan", "front"] as const) {
    const all = state.shapes[v].map((s) => s.rings);
    console.log("[DEBUG]", v, "all", JSON.stringify(bounds(all.flat())));
    for (const s of state.shapes[v]) if (/tyre|wheel|fork|handlebar|fender-f|stem/.test(s.shape.id)) console.log("[DEBUG]  ", v, s.shape.id, s.shape.bone, JSON.stringify(bounds(s.rings)));
  }
}
