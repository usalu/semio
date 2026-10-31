/** [DEBUG] Nudge the japanese-maple 1:500 drawing back inside the extent tolerance. */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
let path = "";
for (const cat of readdirSync(root)) {
  try {
    const doc = readdirSync(`${root}/${cat}/japanese-maple`).find((f) => f.endsWith(".json"));
    if (doc) path = `${root}/${cat}/japanese-maple/${doc}`;
  } catch { /* next */ }
}
const doc = JSON.parse(readFileSync(path, "utf8"));
const asset = doc.asset;
const rep = asset.representations["500"];
function bounds(): { w: number; h: number } {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: 500 } });
  return { w: state.bounds.maxX - state.bounds.minX, h: state.bounds.maxY - state.bounds.minY };
}
function scale(s: number): void {
  for (const shape of [...rep.geometry.elevation, ...(rep.geometry.plan ?? [])]) {
    for (const key of ["x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "x", "y", "w", "h", "r"]) if (typeof shape[key] === "number") shape[key] *= s;
    if (shape.kind === "poly") shape.points = shape.points.map(([x, y]: number[]) => [x * s, y * s]);
    if (shape.kind === "path") shape.d = shape.d.replace(/-?\d*\.?\d+(?:e-?\d+)?/gu, (n: string) => String(Math.round(Number(n) * s * 1e5) / 1e5));
  }
}
const before = bounds();
const widthCap = asset.dimensions.widthM + Math.max(0.02 * asset.dimensions.widthM, 0.015) - 0.008;
const s = widthCap / before.w;
scale(s);
const after = bounds();
const tol = (nominal: number, measured: number): boolean => Math.abs(measured - nominal) <= Math.max(0.02 * nominal, 0.015);
console.log(`[DEBUG] maple ${before.w.toFixed(3)}x${before.h.toFixed(3)} -> ${after.w.toFixed(3)}x${after.h.toFixed(3)} s ${s.toFixed(4)} okW ${tol(asset.dimensions.widthM, after.w)} okH ${tol(asset.dimensions.heightM, after.h)}`);
writeFileSync(path, JSON.stringify(doc, null, 1) + "\n");
