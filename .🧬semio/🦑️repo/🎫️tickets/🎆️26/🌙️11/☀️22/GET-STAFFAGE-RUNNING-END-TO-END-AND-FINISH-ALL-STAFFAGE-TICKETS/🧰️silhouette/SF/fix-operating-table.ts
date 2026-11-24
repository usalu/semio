/** 🏥️ Repair of `operating-table` (plan 101.2 mm at 1:50 and 1:100, front elevation 67.5 mm at 1:100): the plan and the front elevation drew only a part of what the volume has.
 * - Plan, 1:50 and 1:100: the four hinge barrels between the back, seat and leg sections (they close the outer 50 mm of the 15 mm joints, so the open joint the drawing showed was twice as long as the real one), the left castors of the table base and of the lamp stand, and at 1:50 the second wheel of every twin castor are stated; each is the plan of its solid (hinges as discs of the bone `top` extruded over their barrel, castors as rectangles like their drawn twins). At 1:50 a hinge shows only in the 10 mm joint between the pads, 0.2 mm of paper, which no filled shape may be (`suitability-feature`); it is therefore the unfilled outline of the barrel, whose edge crosses the joint as a line, and the guard reads an outline that encloses body as the body it outlines.
 * - Plan, 1:50: the two base beams were drawn as rectangles with 17.5 mm corners, the solids are superelliptic lofts (exponent 6); the shapes become the outline of the loft (numbers of `outline.ts operating-table 50 plan base-beam-head`).
 * - Front elevation and plan, 1:100: the tilt pin (30 mm, 0.20 m long) that closes the 5 mm between column head and saddle cheeks was not drawn; it is stated as at 1:50. The role `detail` has no pen at 1:100, so the hinges and the pin are `body` shapes there.
 *  bun fix-operating-table.ts [--file <asset.json>] [--dry] */
import { asPoly, run, shapeOf, stateShape } from "./lib.ts";

const BEAM: readonly (readonly [number, number])[] = [[-0.4692, 0.17207], [-0.47, 0], [-0.4692, -0.17207], [-0.46672, -0.2143], [-0.46236, -0.24054], [-0.45556, -0.25736], [-0.44461, -0.2669], [-0.4, -0.27], [-0.35539, -0.2669], [-0.34444, -0.25736], [-0.33764, -0.24054], [-0.33328, -0.2143], [-0.3308, -0.17207], [-0.33, 0], [-0.3308, 0.17207], [-0.33328, 0.2143], [-0.33764, 0.24054], [-0.34444, 0.25736], [-0.35539, 0.2669], [-0.4, 0.27], [-0.44461, 0.2669], [-0.45556, 0.25736], [-0.46236, 0.24054], [-0.46672, 0.2143]];
const HINGES: readonly (readonly [string, number, number, number])[] = [["hinge-back-r", -0.22, -0.25, -0.2], ["hinge-back-l", -0.22, 0.2, 0.25], ["hinge-leg-r", 0.26, -0.25, -0.2], ["hinge-leg-l", 0.26, 0.2, 0.25]];
const TWINS: Record<number, readonly (readonly [string, string, number])[]> = {
  50: [["castor-head-r-a", "castor-head-r-b", -0.205], ["castor-head-r-a", "castor-head-l-a", 0.181], ["castor-head-r-a", "castor-head-l-b", 0.215], ["castor-foot-r-a", "castor-foot-r-b", -0.205], ["castor-foot-r-a", "castor-foot-l-a", 0.181], ["castor-foot-r-a", "castor-foot-l-b", 0.215], ["stand-castor-near-r-a", "stand-castor-near-r-b", -0.191], ["stand-castor-near-r-a", "stand-castor-near-l-a", 0.173], ["stand-castor-near-r-a", "stand-castor-near-l-b", 0.199], ["stand-castor-far-r-a", "stand-castor-far-r-b", -0.191], ["stand-castor-far-r-a", "stand-castor-far-l-a", 0.173], ["stand-castor-far-r-a", "stand-castor-far-l-b", 0.199]],
  100: [["castor-head-r", "castor-head-l", 0.185], ["castor-foot-r", "castor-foot-l", 0.185], ["stand-castor-near-r", "stand-castor-near-l", 0.175], ["stand-castor-far-r", "stand-castor-far-l", 0.175]],
};

await run("operating-table", (asset) => {
  for (const scale of [50, 100]) {
    for (const [twin, id, y] of TWINS[scale]!) stateShape(asset, scale, "plan", twin, { ...shapeOf(asset, scale, "plan", twin), id, y });
    const frame = shapeOf(asset, scale, "plan", "frame-seat");
    for (const [id, cx, z0, z1] of HINGES) stateShape(asset, scale, "plan", "frame-seat", { id, bone: "top", role: scale === 50 ? "detail" : "body", z: frame.z + 0.01, filled: scale !== 50, tone: 0.4, region: "table-frame", material: "stainless-steel", kind: "ellipse", cx, cy: 0.06, rx: 0.022, ry: 0.022, solid: { kind: "extruded", sweep: "w", z0, z1 } });
  }
  asPoly(shapeOf(asset, 50, "plan", "base-beam-head"), BEAM);
  asPoly(shapeOf(asset, 50, "plan", "base-beam-foot"), [...BEAM].reverse().map(([x, y]) => [-x, y] as [number, number]));
  for (const view of ["front", "plan"]) {
    const head = shapeOf(asset, 100, view, "column-head");
    stateShape(asset, 100, view, "column-head", { id: "tilt-pin", bone: "top", role: "body", z: head.z + 0.01, filled: true, tone: 0.4, region: "table-frame", material: "stainless-steel", kind: "ellipse", cx: 0.0, cy: 0.0, rx: 0.015, ry: 0.015, solid: { kind: "extruded", sweep: "w", z0: -0.1, z1: 0.1 } });
  }
});
