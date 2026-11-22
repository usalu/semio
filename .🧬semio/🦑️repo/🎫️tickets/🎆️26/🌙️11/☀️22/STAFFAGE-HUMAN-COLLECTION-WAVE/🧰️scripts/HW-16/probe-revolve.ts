/** 🔬️ [DEBUG] which local axis the lateralScale of a revolve about u stretches. */
import { tessellateSolid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
const g = tessellateSolid({ id: "t", kind: "revolve", bone: "b", node: "b", material: "m", role: "body", tone: 0.5, profile: [[0, 0.1], [0.2, 0.1], [0.2, 0.09], [0, 0.09], [0, 0.1]], axis: [1, 0, 0], origin: [0, 0, 0], lateralScale: 2 } as never, { chordM: 0.005, minSegments: 32, maxSegments: 64 });
const p = g.positions;
for (let k = 0; k < 3; k++) { let [lo, hi] = [Infinity, -Infinity]; for (let i = k; i < p.length; i += 3) (lo = Math.min(lo, p[i]!)), (hi = Math.max(hi, p[i]!)); console.log("axis", k, lo.toFixed(3), hi.toFixed(3)); }
