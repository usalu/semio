/** 🔬️ Round 3, vegetation/agave: places one 2D shape of the candidate document in every view of the default pose and prints which view fails. Usage: bun r3-agave-probe1.ts <scale> <shape id> */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, STAFFAGE } from "./volumes-lib.ts";
const { evaluate } = await import(join(STAFFAGE, "🔨️modules/🧮️evaluation/🟦️.ts"));
const asset = JSON.parse(readFileSync(join(GENERATED, "r3-agave", "🔣️.json"), "utf8")).asset;
const [scale, id] = [process.argv[2]!, process.argv[3]!];
const full = asset.representations[scale].geometry;
for (const view of ["elevation", "front", "plan"]) {
  const a = { ...asset, representations: { ...asset.representations, [scale]: { ...asset.representations[scale], geometry: Object.fromEntries(Object.entries(full).map(([v, list]) => [v, v === view ? (list as any[]).filter((s) => s.id === id) : []])) } } };
  try { evaluate(a, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } }); console.log(`[DEBUG] ${view} ok`); } catch (e) { console.log(`[DEBUG] ${view} FAIL ${String(e).slice(0, 160)}`); }
}
console.log("[DEBUG]", JSON.stringify(full.elevation.find((s: any) => s.id === id)).slice(0, 900));
