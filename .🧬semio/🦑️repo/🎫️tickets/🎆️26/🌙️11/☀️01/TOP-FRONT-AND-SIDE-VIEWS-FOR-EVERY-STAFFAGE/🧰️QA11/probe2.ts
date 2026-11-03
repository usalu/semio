import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const [file, scale, base, bone, ...twists] = process.argv.slice(2) as [string, string, string, string, ...string[]];
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const round = (v: number) => Math.round(v * 1000) / 1000;
for (const t of twists) {
  const src = asset.poses.find((p: { id: string }) => p.id === base);
  const patched = { ...asset, poses: [...asset.poses, { ...src, id: "probe", bones: { ...src.bones, [bone]: { twistDeg: Number(t) } } }] };
  const state = evaluate(patched, { kind: "pose", pose: "probe" }, { kind: "scale", scale: { denominator: Number(scale) } });
  console.log("[DEBUG] twist", t);
  for (const v of ["elevation", "plan", "front"] as const)
    for (const s of state.shapes[v]) if (/^(tyre-f|wheel-f|handlebar|fork)/.test(s.shape.id)) console.log("[DEBUG]  ", v, s.shape.id, s.shape.bone, JSON.stringify(s.rings[0]!.filter((_, i, a) => i % Math.max(1, Math.floor(a.length / 8)) === 0).map((p) => [round(p.x), round(p.y)])));
}
