/** ⏱️ Round 3, vegetation/hosta-clump: CPU time of evaluating the default pose and depicting the three viewpoints of one scale, optionally without the shapes whose id matches a pattern. Usage: bun r3-hosta-clump-time.ts <scale> [regex of shape ids to leave out]
 * @see ./r3-vegetation-hosta-clump.md */
import { join } from "node:path";
import { STAFFAGE, entryOf } from "./volumes-lib.ts";
const { evaluate } = (await import(join(STAFFAGE, "🔨️modules/🧮️evaluation/🟦️.ts"))) as { evaluate: (...a: unknown[]) => { shapes: Record<string, unknown[]> } };
const { asset } = entryOf("vegetation/hosta-clump");
const [scale, drop] = [process.argv[2]!, process.argv[3]];
const rep = asset.representations[scale];
if (drop) for (const v of ["elevation", "front", "plan"]) rep.geometry[v] = rep.geometry[v].filter((s: { id: string }) => !new RegExp(drop).test(s.id));
const t0 = performance.now();
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } });
console.log(`[DEBUG] 1:${scale} without ${drop ?? "nothing"}: ${rep.geometry.elevation.length} shapes, evaluate ${((performance.now() - t0) / 1000).toFixed(1)} s, ${state.shapes.elevation!.length} evaluated`);
