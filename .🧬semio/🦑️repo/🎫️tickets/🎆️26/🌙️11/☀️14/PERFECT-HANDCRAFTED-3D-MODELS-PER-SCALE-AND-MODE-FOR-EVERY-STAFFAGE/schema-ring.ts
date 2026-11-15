import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { exposedArea } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
const circle = (r: number, n = 64) => Array.from({ length: n }, (_, i) => [r * Math.cos((2 * Math.PI * i) / n), r * Math.sin((2 * Math.PI * i) / n)] as [number, number]);
const q = { chordM: 0.002, minSegments: 32, maxSegments: 64 };
const mk = (id: string, solid: any) => { const g = tessellateSolid({ id, bone: "b", material: "m", role: "body", tone: 0.5, node: "b", ...solid } as never, q); return { id, positions: Float64Array.from(g.positions), indices: Uint32Array.from(g.indices) }; };
const ring = mk("ring", { kind: "extrude", plane: "xy", profile: circle(0.31), holes: [circle(0.26).reverse()], from: -0.11, to: 0.11 });
const ring2 = mk("ring2", { kind: "extrude", plane: "xy", profile: circle(0.31), holes: [circle(0.26)], from: -0.11, to: 0.11 });
const disc = mk("disc", { kind: "extrude", plane: "xy", profile: circle(0.26), from: -0.05, to: 0.05 });
const signed = (w: any) => { let v = 0; const p = w.positions; for (let t = 0; t < w.indices.length; t += 3) { const [a, b, c] = [w.indices[t] * 3, w.indices[t + 1] * 3, w.indices[t + 2] * 3]; v += (p[a] * (p[b + 1] * p[c + 2] - p[b + 2] * p[c + 1]) - p[a + 1] * (p[b] * p[c + 2] - p[b + 2] * p[c]) + p[a + 2] * (p[b] * p[c + 1] - p[b + 1] * p[c])) / 6; } return v; };
console.log("volumes", signed(ring), signed(ring2), signed(disc));
for (const r of [ring, ring2]) { console.log(r.id, "alone", exposedArea([r], [r], [], (o) => o === r), "with disc", exposedArea([r], [r, disc], [], (o) => o === r), "eps 1e-4", exposedArea([r], [r, disc], [], (o) => o === r, 1e-4)); }
