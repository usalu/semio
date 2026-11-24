/** 🖥️ Repair of `height-adjustable-desk` (front elevation 16.0 mm at 1:50 and 86.6 mm at 1:100, depth of the body 0.812 m against 0.80 m at 1:50).
 * - 1:50 and 1:100 volume: the cable tray was a 1.5 mm sheet, a fifth and a tenth of the smallest feature (7.5 and 15 mm), which the raster of the scale holds or loses with the position of its grid, while the front elevation draws it with 4 mm and 8 mm walls; the solid `top-tray` gets the walls of the drawing of its scale.
 * - 1:50 and 1:100 drawings: the VESA plate (and at 1:50 the two hinge ears) that fill the gap between the monitor back and the neck were not drawn in the front elevation and the plan; they are stated as extruded solids of the bone `screen-tilt` at the lateral span of their solids. At 1:100 the back bulge of the monitor is drawn as deep as its loft (0.130 to 0.161 m instead of the 0.168 m of 1:50).
 * - 1:50: the handset stood 12 mm proud of the front edge of the top; it now stands 7 mm proud (within one smallest feature of 1:50), in the solid and in the front elevation and plan, so the declared depth of 0.80 m holds.
 *  bun fix-height-adjustable-desk.ts [--file <asset.json>] [--dry] */
import { run, shapeOf, solidOf, stateShape } from "./lib.ts";

const square = (u0: number, u1: number, v: number): number[][] => [[u0, -v], [u1, -v], [u1, v], [u0, v]];
const part = (id: string, tone: number, penMm: number, points: number[][], z0: number, z1: number): Record<string, unknown> => ({ id, bone: "screen-tilt", role: "body", z: 3, filled: true, tone, kind: "poly", points, closed: true, region: "monitor-housing", material: "steel-desk", penMm, solid: { kind: "extruded", sweep: "w", z0, z1 } });

await run("height-adjustable-desk", (asset) => {
  solidOf(asset, 50, "top-tray").profile = [[0.0, 0.185], [0.0, 0.204], [-0.106, 0.204], [-0.106, 0.336], [0.0, 0.336], [0.0, 0.355], [-0.004, 0.355], [-0.004, 0.34], [-0.11, 0.34], [-0.11, 0.2], [-0.004, 0.2], [-0.004, 0.185]];
  solidOf(asset, 100, "top-tray").profile = [[0.0, 0.2], [0.0, 0.208], [-0.102, 0.208], [-0.102, 0.332], [0.0, 0.332], [0.0, 0.34], [-0.11, 0.34], [-0.11, 0.2]];
  for (const view of ["front", "plan"]) {
    stateShape(asset, 50, view, "monitor-hinge", part("monitor-vesa-plate", 0.31, 0.25, square(-0.058, 0.058, 0.058), 0.168, 0.1705));
    stateShape(asset, 50, view, "monitor-vesa-plate", part("monitor-hinge-ear-l", 0.31, 0.25, square(-0.0465, -0.0375, 0.0165), 0.1675, 0.1845));
    stateShape(asset, 50, view, "monitor-hinge-ear-l", part("monitor-hinge-ear-r", 0.31, 0.25, square(0.0375, 0.0465, 0.0165), 0.1675, 0.1845));
    stateShape(asset, 100, view, "monitor-hinge", part("monitor-vesa-plate", 0.31, 0.2, square(-0.058, 0.058, 0.058), 0.159, 0.175));
    shapeOf(asset, 100, view, "monitor-bulge").solid.z1 = 0.161;
  }
  solidOf(asset, 50, "top-handset").transform.translation = [1.445, -0.012, -0.367];
  shapeOf(asset, 50, "front", "top-handset").x = -0.407;
  shapeOf(asset, 50, "plan", "top-handset").y = -0.407;
});
