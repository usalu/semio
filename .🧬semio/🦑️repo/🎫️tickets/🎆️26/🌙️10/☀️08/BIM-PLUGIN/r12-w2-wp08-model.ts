/**
 * 🧷️ The wall depth vocabulary of `s.bim.model@1` (WP-08: wall sweeps, authored opening reveals, wall top and base attachment). `r3-f1-gen-model.ts` spreads these rows into
 * its single model, so Rust, JSON Schema, TypeScript, GraphQL and proto are generated from here and never drift.
 */
type Field = { name: string; type: string; doc?: string };

const f = (spec: string): Field[] =>
  spec.split(",").map((part) => part.trim()).filter(Boolean).map((part) => {
    const at = part.indexOf(":");
    return { name: part.slice(0, at).trim(), type: part.slice(at + 1).trim() };
  });

export const wallDepthTopVariants = [
  { name: "Roof", fields: f("roof:string, offset:f64") },
  { name: "Slab", fields: f("slab:string, offset:f64") },
  { name: "Ceiling", fields: f("ceiling:string, offset:f64") },
];

export const wallDepthWallFields = f("base_slab:opt:string");

export const wallDepthOpeningFields = f("reveal_depth:opt:f64, reveal_material:opt:string");

export const wallDepthStructs = [
  {
    name: "WallSweep",
    doc: "🧷️ A wall sweep: a profile run along one face of a wall (a baseboard, a cornice, a drip rail). It follows its host by inference: along the join-trimmed face, interrupted by the openings that reach its height, and clipped where the top of the wall is attached. Its solid, length and areas are inferred, never stored.",
    entity: { collection: "wall_sweeps", plural: "WallSweeps" },
    fields: f("host:string, side:WallSide, profile:Profile, height:f64, inset:f64, material:string, name:string"),
  },
];

export const wallDepthFieldDocs: Record<string, string> = {
  "Wall.base_slab": "Optional slab the base of the wall stands on: the base follows the top surface of that slab (sloped slabs included) plus the base offset; absent means the base lies on the elevation of the storey plus the base offset.",
  "Opening.reveal_depth": "Optional depth in metres of the reveal on the front side of the opening: the distance from the front face of the host to the front plane of the window or door frame; absent centres the frame in the thickness of the host.",
  "Opening.reveal_material": "Optional material of the reveal surfaces (jambs, head and sill) between the front face of the host and the frame; absent leaves them in the material of the wall layers.",
  "WallSweep.host": "The wall the sweep runs along.",
  "WallSweep.side": "The face of the wall the sweep stands on, looking along the axis from start to end: the left (interior) or the right (exterior) face.",
  "WallSweep.profile": "Section of the sweep: the first coordinate runs out of the wall perpendicular to its face, the second runs up; the section is centred on its own origin, so a rectangle of width 0.02 and depth 0.12 is a baseboard 2 cm deep and 12 cm high.",
  "WallSweep.height": "Height in metres above the wall base of the lowest point of the profile.",
  "WallSweep.inset": "Distance in metres the profile is moved into the wall: zero stands the inner edge of the profile on the face, a positive inset embeds it, so only the rest shows.",
  "WallSweep.material": "Material of the sweep.",
};
