/** 🔬️ Dumps the product's 3D projection groups and 2D drawn rings of one view and scale of the candidate, for comparison by author-compare.py. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const S = join(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules");
const { evaluate } = await import(join(S, "🧮️evaluation/🟦️.ts"));
const sil = await import(join(S, "🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const [scale, view, pose] = [Number(process.argv[2]), process.argv[3] as "plan", process.argv[4]];
const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const state = evaluate(asset, { kind: "pose", pose: pose ?? asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const body = sil.worldSolids(asset, state.bones, scale, "body");
const ref = sil.referenceOf(view, body, undefined, sil.drawnFor(asset, state, view, scale));
const shapes = state.shapes[view].map((s: any) => ({ id: s.shape.id, role: s.shape.role, filled: s.shape.filled, rings: s.rings ?? null, closed: s.closed }));
writeFileSync(join(import.meta.dir, "🗑️generated", `probe-${scale}-${view}.json`), JSON.stringify({ volume: ref.cut, shapes, sample: Object.keys(state.shapes[view][0] ?? {}) }));
console.log("[DEBUG] groups", ref.cut.length, "shapes", shapes.length, Object.keys(state.shapes[view][0] ?? {}));
