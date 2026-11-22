/** 🌊️ Prints the outer-area shares (air, cover, cavity) of the cases of `🧫️fixtures/🧊️volumes-flood` whose name contains the argument, built as `🧪️tests/🧊️volumes-flood` builds them. Usage: `bun validator-flood-shares.ts <part of a case name>` */
import { readFileSync } from "node:fs";
import { exposedShares, outsideOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts";
import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
const v = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🧊️volumes-flood/🔣️.json", "utf8"));
const quality = { chordM: 0.002, minSegments: 32, maxSegments: 64 };
const circle = (r: number, n: number): [number, number][] => Array.from({ length: n }, (_, i) => [r * Math.cos((2 * Math.PI * i) / n), r * Math.sin((2 * Math.PI * i) / n)]);
const kernel = (id: string, solid: Record<string, unknown>) => {
  const g = tessellateSolid({ id, bone: "b", material: "m", role: "body", tone: 0.5, node: "b", ...solid } as never, quality);
  return { id, positions: Float64Array.from(g.positions), indices: Uint32Array.from(g.indices) };
};
const worldOf = (s: any, k: number) => s.kind === "tube"
  ? kernel(`${s.region}-${k}`, { kind: "extrude", plane: "xy", profile: circle(s.outerRadiusM, s.segments), ...(s.innerRadiusM > 0 ? { holes: [circle(s.innerRadiusM, s.segments).reverse()] } : {}), from: s.from, to: s.to })
  : kernel(`${s.region}-${k}`, { kind: "extrude", plane: "xy", profile: [[s.min[0], s.min[1]], [s.max[0], s.min[1]], [s.max[0], s.max[1]], [s.min[0], s.max[1]]], from: s.min[2], to: s.max[2] });
for (const c of v.cases.filter((x: any) => x.name.includes(process.argv[2] ?? ""))) {
  const worlds = c.solids.map(worldOf);
  const own = worlds.filter((_: unknown, k: number) => c.solids[k].region === c.target);
  const outside = outsideOf(worlds, v.featureM, v.probeM);
  const shares = exposedShares(own, own, [], () => false, v.probeM, v.seamM, (base, normal) => outside.reach(base, normal, (solid) => own.includes(solid as never)));
  console.log(c.name, "hand", c.areaM2, "air", shares.air.toFixed(5), "cover", shares.cover.toFixed(5), "cavity", shares.cavity.toFixed(5));
}
