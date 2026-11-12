const K = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗿️model";
const { tessellateSolid } = await import(`${K}/🔺️tessellation/🟦️.ts`);
const solid: any = { id: "x", node: "n", material: "m", kind: "extrude", profile: [[-2, 0.06], [2, 0.06], [2, 1.26], [-2, 1.26]], plane: "xy", from: -0.005, to: 0.04, holes: [[[-1.965, 0.095], [-0.015, 0.095], [-0.015, 1.225], [-1.965, 1.225]], [[0.015, 0.095], [1.965, 0.095], [1.965, 1.225], [0.015, 1.225]]] };
const q = { chordM: 0.002, minSegments: 12, maxSegments: 64 };
const g = tessellateSolid(solid, q);
console.log("tris", g.indices.length / 3, "verts", g.positions.length / 3);
const key = (i: number) => [0, 1, 2].map((k) => Math.round(g.positions[i * 3 + k]! / 1e-7)).join(",");
const edges = new Map<string, number>();
for (let t = 0; t < g.indices.length; t += 3) for (const [a, b] of [[0, 1], [1, 2], [2, 0]] as const) { const e = `${key(g.indices[t + a]!)}>${key(g.indices[t + b]!)}`; edges.set(e, (edges.get(e) ?? 0) + 1); }
const open: string[] = []; for (const [e] of edges) { const [a, b] = e.split(">"); if (!edges.has(`${b}>${a}`)) open.push(e); }
console.log(open.length, open.slice(0, 6));
// variants: single hole
for (const holes of [[solid.holes[0]], [solid.holes[1]], solid.holes]) { const gg = tessellateSolid({ ...solid, holes }, q); const ed = new Map<string, number>(); const k2 = (i: number) => [0, 1, 2].map((k) => Math.round(gg.positions[i * 3 + k]! / 1e-7)).join(","); for (let t = 0; t < gg.indices.length; t += 3) for (const [a, b] of [[0, 1], [1, 2], [2, 0]] as const) { const e = `${k2(gg.indices[t + a]!)}>${k2(gg.indices[t + b]!)}`; ed.set(e, 1); } let n = 0; for (const [e] of ed) { const [a, b] = e.split(">"); if (!ed.has(`${b}>${a}`)) n++; } console.log("holes", holes.length, "open", n, "tris", gg.indices.length / 3); }
