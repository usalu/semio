/** ✅️ Runs every primitive of `kit-lib.ts` with its documented example and checks that the kernel tessellates it. Usage: `bun kit-selftest.ts`. */
import { meshesAt } from "./volumes-lib.ts";
import { boxOf, capsuleOf, ellipsoidOf, envelopePrism, extrudeOf, latticeBars, loftFromOutline, loftOf, lobeStack, openCtx, revolveOf, scaledSolid, shellOffset, stationLoft, superellipsePoints, superellipseSection } from "./kit-lib.ts";

const c = openCtx("vehicles/delivery-van", 100);
const bone = "chassis";
const common = (id: string) => ({ id, bone, material: c.m.materials[0].id });
const thigh = loftOf(common("loft"), [superellipseSection([0, 0, 0], 0.07, 0.08), superellipseSection([0.44, 0, 0], 0.05, 0.05)]);
const items: Record<string, any[]> = {
  loftOf: [thigh],
  stationLoft: [stationLoft(common("st"), "u", [{ s: 0, c1: 0.9, c2: 0, rx: 0.9, ry: 0.7 }, { s: 5, c1: 0.9, c2: 0, rx: 0.9, ry: 0.7 }])],
  loftFromOutline: [loftFromOutline(common("lo"), [{ x: 0, y: 0 }, { x: 4, y: 0 }, { x: 4, y: 1 }, { x: 0, y: 1 }], { axis: "u", across: "v", depth: () => 0.9, stations: 8 })],
  shellOffset: [shellOffset(thigh, 0.004, "shell")!],
  scaledSolid: [scaledSolid(thigh, 0.85, "sc")],
  lobeStack: lobeStack(common("crown"), [{ at: [6, 0, 0], radii: [2, 1.5, 1.5] }, { at: [8, 0.3, 0], radii: [1.4, 1.1, 1.1] }]),
  latticeBars: latticeBars(common("truss"), { a: [0, 0, 0], b: [1, 0, 0], c: [0.5, 0.8, 0] }, [["a", "b"], ["a", "c"], ["b", "c"]], 0.02),
  envelopePrism: [envelopePrism(common("env"), [0, -0.4, -0.4], [12, 0.4, 0.4])],
  revolveOf: [revolveOf(common("rev"), [[-0.1, 0.25], [0.1, 0.25], [0.1, 0.4], [-0.1, 0.4]], "w")],
  extrudeOf: [extrudeOf(common("ex"), "xy", superellipsePoints(0.5, 0.2, 4, 24), -0.9, 0.9)],
  capsuleOf: [capsuleOf(common("cap"), [0, 0, 0], [0.43, 0, 0], 0.05, 0.035)],
  ellipsoidOf: [ellipsoidOf(common("el"), [0.13, 0, 0], [0.12, 0.1, 0.09])],
  boxOf: [boxOf(common("box"), [0.2, 0.05, 0.9], [0.4, 0.1, 0.2], 0.01)],
};
let bad = 0;
for (const [name, list] of Object.entries(items)) {
  try {
    const m = meshesAt(c.m, 100, list);
    const tris = m.reduce((n, x) => n + x.indices.length / 3, 0);
    process.stdout.write(`ok   ${name.padEnd(16)} ${list.length} solid(s), ${tris} triangles\n`);
    if (tris === 0) throw new Error("empty mesh");
  } catch (e) {
    bad++;
    process.stdout.write(`FAIL ${name}: ${(e as Error).message}\n`);
  }
}
process.exit(bad === 0 ? 0 : 1);
