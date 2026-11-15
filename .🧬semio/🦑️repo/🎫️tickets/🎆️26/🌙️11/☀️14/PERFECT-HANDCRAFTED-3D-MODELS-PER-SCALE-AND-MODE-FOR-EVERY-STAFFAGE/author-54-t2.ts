import { localGeometry } from "./volumes-lib.ts";
const s = { id: "t", kind: "revolve", bone: "b", material: "m", role: "body", tone: 0.5, axis: [1, 0, 0], origin: [0, 0, 0], profile: [[0, 0.0145], [0.5, 0.0145], [0.5, 0.0115], [0, 0.0115]] };
const g = localGeometry(s, { chordM: 0.002, minSegments: 16, maxSegments: 16 });
let A = 0; for (let t = 0; t < g.indices.length; t += 3) { const p = (k: number) => [g.positions[g.indices[t + k]! * 3]!, g.positions[g.indices[t + k]! * 3 + 1]!, g.positions[g.indices[t + k]! * 3 + 2]!]; const [a, b, c] = [p(0), p(1), p(2)]; const u = [b[0]! - a[0]!, b[1]! - a[1]!, b[2]! - a[2]!], v = [c[0]! - a[0]!, c[1]! - a[1]!, c[2]! - a[2]!]; A += 0.5 * Math.hypot(u[1]! * v[2]! - u[2]! * v[1]!, u[2]! * v[0]! - u[0]! * v[2]!, u[0]! * v[1]! - u[1]! * v[0]!); }
console.log("tris", g.indices.length / 3, "area", A.toFixed(4), "outer lateral", (2 * Math.PI * 0.0145 * 0.5).toFixed(4), "inner", (2 * Math.PI * 0.0115 * 0.5).toFixed(4));
