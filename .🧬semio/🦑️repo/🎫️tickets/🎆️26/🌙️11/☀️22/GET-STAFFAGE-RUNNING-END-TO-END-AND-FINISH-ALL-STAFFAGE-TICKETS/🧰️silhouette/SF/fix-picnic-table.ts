/** 🧺️ Repair of `picnic-table` at 1:50 (side elevation 25.2 mm, plan 22.7 mm).
 * - Table top: the five planks were drawn 125, 118, 118, 118 and 125 mm wide with 24 mm joints, the solid `top-plank` was five planks of 132 mm at a pitch of 142 mm (10 mm joints, 0.2 mm of paper, of which the 0.13 mm pen leaves 0.07 mm: below the 0.15 mm a printed gap needs, `suitability-gap`). Body and drawing become the same top: five planks of 128 mm at a pitch of 143 mm, 15 mm joints (0.17 mm of clear paper); the solid (body and thermal list), the elevation polygons (the eased profile of the solid) and the plan rectangles state it.
 * - Plan: the cleat under the top (`top-cleat`, 45 mm, at mid length) that closes the joints there was not drawn. It shows only in the 15 mm joints, of which the pens of the planks leave 0.17 mm of paper, less than the 0.2 mm a filled shape needs (`suitability-feature`), so it is the unfilled outline of the cleat under the planks: its edges cross the joints as lines.
 * - Plan: the battens under the benches close the joint between the two seat planks at mid length in the same way and are stated as the same kind of outline.
 * - Side elevation: the battens under the two benches (`batten-l`, `batten-r`, 45 x 45 mm, 0.28 m long) that reach 10 mm beyond the gussets were not drawn; they are stated with the section of their solids.
 *  bun fix-picnic-table.ts [--file <asset.json>] [--dry] */
import { run, shapeOf, stateShape } from "./lib.ts";

const PROFILE = [[0.0, 0.005], [0.00067, 0.0025], [0.0025, 0.00067], [0.005, 0.0], [0.123, 0.0], [0.1255, 0.00067], [0.12733, 0.0025], [0.128, 0.005], [0.128, 0.035], [0.12733, 0.0375], [0.1255, 0.03933], [0.123, 0.04], [0.005, 0.04], [0.0025, 0.03933], [0.00067, 0.0375], [0.0, 0.035]];
const round = (n: number): number => Number(n.toFixed(5));

await run("picnic-table", (asset) => {
  for (const list of ["body", "thermal"]) for (const solid of asset.volumes["50"][list]) if (solid.kind === "extrude" && /(^|-)top-plank$/u.test(solid.id)) Object.assign(solid, { profile: PROFILE.map(([u, v]) => [u, v]), repeat: { count: 5, translation: [0.143, 0.0, 0.0] } });
  for (let k = 0; k < 5; k++) {
    shapeOf(asset, 50, "elevation", `top-plank-${k + 1}`).points = PROFILE.map(([u, v]) => [round(u! + k * 0.143), v]);
    Object.assign(shapeOf(asset, 50, "plan", `top-plank-${k + 1}`), { x: round(-0.35 + k * 0.143), w: 0.128 });
  }
  const plank = shapeOf(asset, 50, "plan", "top-plank-5");
  stateShape(asset, 50, "plan", "top-plank-5", { id: "top-cleat", bone: "top", role: "detail", z: plank.z - 0.1, filled: false, tone: 0.5, region: plank.region, material: "pine-timber", kind: "rect", x: -0.31, y: -0.0225, w: 0.62, h: 0.045 });
  for (const [side, v0, v1] of [["l", -0.025, 0.02], ["r", -0.02, 0.025]] as const) {
    const gusset = shapeOf(asset, 50, "elevation", `gusset-${side}`);
    const timber = asset.representations["50"].geometry.elevation.find((s: any) => s.bone === gusset.bone && s.material === "pine-timber");
    const seat = asset.representations["50"].geometry.plan.find((s: any) => s.bone === gusset.bone && s.material === "pine-timber");
    stateShape(asset, 50, "plan", seat.id, { id: `batten-${side}`, bone: gusset.bone, role: "detail", z: seat.z - 0.1, filled: false, tone: 0.5, region: seat.region, material: "pine-timber", kind: "rect", x: side === "l" ? -0.75 : 0.47, y: -0.0225, w: 0.28, h: 0.045 });
    stateShape(asset, 50, "elevation", gusset.id, { id: `batten-${side}`, bone: gusset.bone, role: "body", z: gusset.z - 0.1, filled: true, tone: 0.5, region: timber.region, material: "pine-timber", kind: "poly", points: [[0.02, v0], [0.3, v0], [0.3, v1], [0.02, v1]], closed: true });
  }
});
