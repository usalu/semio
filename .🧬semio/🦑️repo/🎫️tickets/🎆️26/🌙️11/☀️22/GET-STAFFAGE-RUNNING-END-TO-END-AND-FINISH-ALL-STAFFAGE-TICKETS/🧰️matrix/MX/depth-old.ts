/** 🕰️ The depth the removed line of `shapeDepthIn` gave the three vectors of `🧫️fixtures/🧊️solid-sweep` (`depth.cases`), beside the vector and the depth of the engine as it is: `bun depth-old.ts`. The removed line carried every profile point of an extruded solid as (first coordinate along u, second along v, middle of the span along w), whatever its sweep. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { S } from "../MR/lib.ts";

const { evaluate, shapeDepth } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const vectors = JSON.parse(readFileSync(join(S, "🧫️fixtures", "🧊️solid-sweep", "🔣️.json"), "utf8"));
const sample = JSON.parse(readFileSync(join(S, "🧫️fixtures", "👤️sample-figure", "🔣️.json"), "utf8")).asset;
const key = String(vectors.scale);
const { profile, span } = vectors.depth;
const { u, v, w } = vectors.basis;
const middle = (span.z0 + span.z1) / 2;
const corners = [[profile.x, profile.y], [profile.x + profile.w, profile.y], [profile.x + profile.w, profile.y + profile.h], [profile.x, profile.y + profile.h]];
const old = corners.map(([a, b]) => [a! * u[0] + b! * v[0] + middle * w[0], a! * u[1] + b! * v[1] + middle * w[1]]);
for (const c of vectors.depth.cases) {
  const shape = { id: "slat", kind: "rect", bone: vectors.bone, role: "body", z: 1, filled: true, tone: 0.5, ...profile, solid: { kind: "extruded", sweep: c.sweep, ...span } };
  const asset = { ...sample, representations: { ...sample.representations, [key]: { ...sample.representations[key], geometry: { ...sample.representations[key].geometry, front: [shape], plan: [shape] } } } };
  const state = evaluate(asset, { kind: "pose", pose: vectors.pose }, { kind: "scale", scale: { denominator: vectors.scale } });
  const origin = state.bones.pelvis.origin;
  console.log(`[DEBUG] sweep ${c.sweep}: vector front ${c.front} plan ${c.plan}; removed line front ${Math.max(...old.map((p) => p[0]!))} plan ${Math.max(...old.map((p) => p[1]!))}; engine now front ${shapeDepth(state, state.shapes.front[0].shape, "front") - origin.x} plan ${shapeDepth(state, state.shapes.plan[0].shape, "plan") - origin.y}`);
}
