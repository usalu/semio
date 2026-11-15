const S = "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog";
const dir = process.env.VOLUMES_DIR ?? "🧊️volumes";
const { exposedArea } = await import(`${S}/${dir}/🧮️measures/🟦️.ts`);
const { tessellateSolid } = await import("../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts");
const circle = (r: number, n = 64): [number, number][] => Array.from({ length: n }, (_, i) => [r * Math.cos((2 * Math.PI * i) / n), r * Math.sin((2 * Math.PI * i) / n)]);
const kernel = (id: string, solid: Record<string, unknown>, q: any) => { const g = tessellateSolid({ id, bone: "b", material: "m", role: "body", tone: 0.5, node: "b", ...solid } as never, q); return { id, positions: Float64Array.from(g.positions), indices: Uint32Array.from(g.indices) }; };
const expected = 1.2283;
for (const [qr, qd, rd] of [[64, 64, 0.26], [64, 48, 0.26], [64, 32, 0.26], [48, 64, 0.26], [64, 64, 0.27], [64, 64, 0.255], [32, 64, 0.26], [64, 40, 0.26]] as const) {
  const ring = kernel("ring", { kind: "extrude", plane: "xy", profile: circle(0.31, qr), holes: [circle(0.26, qr).reverse()], from: -0.11, to: 0.11 }, { chordM: 0.002, minSegments: 32, maxSegments: 64 });
  const disc = kernel("disc", { kind: "extrude", plane: "xy", profile: circle(rd, qd), from: -0.05, to: 0.05 }, { chordM: 0.002, minSegments: 32, maxSegments: 64 });
  const a = exposedArea([ring, disc], [ring, disc], [], () => false, 0.00015, 0.0015);
  console.log(`ring ${qr} disc ${qd} r ${rd}: ${a.toFixed(4)} expected ${rd === 0.26 ? expected : "-"}`);
}
