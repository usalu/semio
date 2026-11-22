/** 🔎️ Compares per id the world box of every 3D body solid with the box of its evaluated 2D shape in one view of one scale and pose. */
import { readFileSync } from "node:fs";
import { evaluateDrawing } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids, project } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const [file, k, view, pose] = process.argv.slice(2);
const a = JSON.parse(readFileSync(file!, "utf8")).asset;
const d = Number(k);
const state = evaluateDrawing(a, { kind: "pose", pose: pose ?? a.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } });
const body = worldSolids(a, state.bones, d);
const box3 = new Map<string, number[]>();
for (const w of body) {
  let b = [Infinity, Infinity, -Infinity, -Infinity];
  for (let i = 0; i < w.positions.length; i += 3) { const p = project(view as "elevation", w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); b = [Math.min(b[0]!, p.x), Math.min(b[1]!, p.y), Math.max(b[2]!, p.x), Math.max(b[3]!, p.y)]; }
  box3.set(w.id, b);
}
const shapes = (state.shapes as Record<string, { shape: { id: string }; rings: { x: number; y: number }[][] }[]>)[view!]!;
const f = (b: number[]) => b.map((v) => v.toFixed(3)).join(" ");
for (const s of shapes) {
  let b = [Infinity, Infinity, -Infinity, -Infinity];
  for (const r of s.rings) for (const p of r) b = [Math.min(b[0]!, p.x), Math.min(b[1]!, p.y), Math.max(b[2]!, p.x), Math.max(b[3]!, p.y)];
  const t = box3.get(s.shape.id);
  const dev = t ? Math.max(...b.map((v, i) => Math.abs(v - t[i]!))) : -1;
  console.log(`[DEBUG] ${s.shape.id.padEnd(22)} 2d ${f(b)} | 3d ${t ? f(t) : "-"} | dev ${dev.toFixed(3)}`);
}
