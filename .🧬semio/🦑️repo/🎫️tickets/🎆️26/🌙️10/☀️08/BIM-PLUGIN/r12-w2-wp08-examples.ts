/**
 * 🧗️ Wave W2 `w2-wp08-walldepth`: the wall depth of the examples, merged into the snapshot by `r4-x-examples-gen.ts`. The `house` attic is walled under its pitched roof (all four walls attach their top to the
 * underside of `rf-main`, so the gable ends rise into the roof and the eave walls follow its overhang), the ground floor gets two baseboards (the sweep follows the trimmed face and stops at the doors) and the
 * living room window is set back from the front face with a plastered reveal. Every value is authored (host, side, profile, height, inset, material; attach target and offset; reveal depth and material); the
 * elevation outlines, runs, lengths and areas are inferred. `wallDepthFor(name, model)` edits `model` in place and refuses a reference that points at nothing.
 */
type Json = Record<string, any>;

const ATTIC_WALLS = ["w-a-south", "w-a-east", "w-a-north-1-0", "w-a-west-1-0"];
const baseboard = (host: string, name: string) => ({ host, side: "Left", profile: { Rectangle: { width: 0.02, depth: 0.1 } }, height: 0, inset: 0, material: "m-timber", name });

export function wallDepthFor(name: string, model: Json): void {
  if (name !== "house") return;
  for (const id of ATTIC_WALLS) {
    const wall = model.walls[id];
    if (wall === undefined) throw new Error(`wall depth: attic wall ${id} is missing`);
    wall.top = { Roof: { roof: "rf-main", offset: 0 } };
  }
  model.wall_sweeps = {
    "ws-living-south": baseboard("w-g-south", "Baseboard Living Room South"),
    "ws-cross-wall": baseboard("w-g-cross", "Baseboard Cross Wall"),
  };
  const living = model.openings["o-g-living"];
  if (living === undefined) throw new Error("wall depth: the living room window is missing");
  Object.assign(living, { reveal_depth: 0.1, reveal_material: "m-plaster" });
  for (const [id, sweep] of Object.entries<Json>(model.wall_sweeps)) {
    if (model.walls[sweep.host] === undefined) throw new Error(`wall_sweeps/${id}: host ${sweep.host}`);
    if (model.materials[sweep.material] === undefined) throw new Error(`wall_sweeps/${id}: material ${sweep.material}`);
  }
  if (model.roofs["rf-main"] === undefined || model.materials["m-plaster"] === undefined) throw new Error("wall depth: the roof or the reveal material is missing");
}
