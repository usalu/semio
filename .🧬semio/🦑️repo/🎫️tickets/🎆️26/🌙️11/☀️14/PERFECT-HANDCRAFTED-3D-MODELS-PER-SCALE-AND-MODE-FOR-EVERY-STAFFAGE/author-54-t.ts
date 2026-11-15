import { localGeometry } from "./volumes-lib.ts";
const mk = (secs: any[]) => ({ id: "t", kind: "loft", bone: "b", material: "m", role: "body", tone: 0.5, sections: secs, capStart: true, capEnd: true });
const q = { chordM: 0.001, minSegments: 12, maxSegments: 64 };
for (const [name, secs] of [
  ["two equal", [{ at: [0, 0, 0], rx: 0.118, ry: 0.017, exponent: 2.8 }, { at: [0.4, 0, 0], rx: 0.118, ry: 0.017, exponent: 2.8 }]],
  ["three, mid offset c2 tiny", [{ at: [0, 0, 0], rx: 0.118, ry: 0.017, exponent: 2.8 }, { at: [0.2, 0, 0], rx: 0.118, ry: 0.017, exponent: 2.8 }, { at: [0.4, 0, 0.003], rx: 0.118, ry: 0.017, exponent: 2.8 }]],
] as const) {
  const g = localGeometry(mk(secs as any), q);
  const ws: number[] = [];
  for (let i = 0; i < g.positions.length; i += 3) if (Math.abs(g.positions[i]! - 0.2) < 0.01) ws.push(g.positions[i + 2]!);
  console.log(name, "n", g.positions.length / 3, "w@0.2", Math.min(...ws).toFixed(3), Math.max(...ws).toFixed(3));
}
