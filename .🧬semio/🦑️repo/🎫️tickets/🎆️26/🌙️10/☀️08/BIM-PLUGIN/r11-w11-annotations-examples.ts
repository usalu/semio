/**
 * 🪧️ Wave R11 `w11-annotations`: the dimensions, tags, text notes and leaders of the `house` and `office` examples, merged into the snapshot by `r4-x-examples-gen.ts`. Every value is authored (anchors, direction,
 * offset, style, element, category, text); the printed distances and texts are inferred. `annotationsFor(name, model)` returns the five collections and refuses a reference that points at nothing.
 */
import * as F from "./r3-f1-fixtures.ts";

type Json = Record<string, any>;
const P = F.P;
const wallEnd = (wall: string, end: "Start" | "End") => ({ WallEnd: { wall, end } });
const face = (wall: string, side: "Left" | "Right") => ({ WallFace: { wall, side } });
const centre = (opening: string) => ({ OpeningCentre: { opening } });
const grid = (id: string) => ({ Grid: { grid: id } });
const column = (id: string) => ({ ColumnCentre: { column: id } });
const free = (x: number, y: number) => ({ Point: { point: P(x, y) } });
const style = (name: string, terminator: string, unit: string, precision: number, textHeight = 0.25) => ({ name, text_height: textHeight, terminator, unit, precision, mark_size: 0.15, gap: 0.1, overshoot: 0.2 });
const dimension = (storey: string, name: string, anchors: unknown[], angle: number, offset: number, styleId = "as-plan") => ({ storey, anchors, angle, offset, style: styleId, name });
const tag = (storey: string, element: string, category: "Name" | "Type" | "Number" | "Size", at: [number, number], styleId = "as-plan") => ({ storey, element, category, offset: P(at[0], at[1]), style: styleId });
const note = (storey: string, at: [number, number], text: string, styleId = "as-plan") => ({ storey, position: P(at[0], at[1]), text, rotation: 0, style: styleId });
const leader = (storey: string, anchor: unknown, at: [number, number], text: string, styleId = "as-plan") => ({ storey, anchor, offset: P(at[0], at[1]), text, style: styleId });
const QUARTER = Math.PI / 2;

const styles = () => ({ "as-plan": style("Plan 1:50", "Tick", "Metre", 2), "as-detail": style("Detail 1:20", "Arrow", "Millimetre", 0, 0.2) });

const house = () => ({
  dimensions: {
    "dim-g-south": dimension("st-ground", "South length", [wallEnd("w-g-south", "Start"), wallEnd("w-g-south", "End")], 0, -1.6),
    "dim-g-south-chain": dimension("st-ground", "Entry and living window", [wallEnd("w-g-south", "Start"), centre("o-g-entry"), centre("o-g-living"), wallEnd("w-g-south", "End")], 0, -0.9),
    "dim-g-west": dimension("st-ground", "West length", [wallEnd("w-g-west", "End"), wallEnd("w-g-west", "Start")], QUARTER, 1.6),
    "dim-g-grids": dimension("st-ground", "Axis spacing", [grid("g-a"), grid("g-b"), grid("g-c")], 0, 6),
    "dim-g-thickness": dimension("st-ground", "South wall thickness", [face("w-g-south", "Left"), face("w-g-south", "Right")], QUARTER, -6.5, "as-detail"),
  },
  tags: {
    "tag-g-south-name": tag("st-ground", "w-g-south", "Name", [0, 0.5]),
    "tag-g-south-type": tag("st-ground", "w-g-south", "Type", [0, 1]),
    "tag-g-entry-size": tag("st-ground", "o-g-entry", "Size", [0, -0.7]),
    "tag-g-hall": tag("st-ground", "sp-g1", "Number", [0, 0]),
  },
  text_notes: { "note-g-site": note("st-ground", [5, -3.2], "Check the plinth level on site") },
  leaders: { "lead-g-north": leader("st-ground", face("w-g-north", "Right"), [1.5, 1.2], "Clay block cavity wall") },
});

const office = () => ({
  dimensions: {
    "dim-0-axes-x": dimension("st-0", "Column axes along A to F", [grid("g-A"), grid("g-B"), grid("g-C"), grid("g-D"), grid("g-E"), grid("g-F")], 0, -13),
    "dim-0-axes-y": dimension("st-0", "Column axes along 1 to 4", [grid("g-1"), grid("g-2"), grid("g-3"), grid("g-4")], QUARTER, 19),
    "dim-0-core-west": dimension("st-0", "West core length", [wallEnd("w-0-core-w-south", "Start"), wallEnd("w-0-core-w-south", "End")], 0, -1.2),
    "dim-0-core-clear": dimension("st-0", "West core clear depth", [face("w-0-core-w-south", "Left"), face("w-0-core-w-north", "Left")], QUARTER, 2, "as-detail"),
    "dim-1-axes-x": dimension("st-1", "First floor axes A to C", [grid("g-A"), grid("g-B"), grid("g-C")], 0, -13),
  },
  tags: {
    "tag-0-column": tag("st-0", "c-0-A1", "Size", [0.7, 0.7]),
    "tag-0-lobby": tag("st-0", "sp-0-1", "Number", [0, 0]),
    "tag-0-core": tag("st-0", "w-0-core-w-south", "Type", [0, -0.6]),
  },
  text_notes: { "note-0-entry": note("st-0", [15, -6], "Main entrance, barrier-free") },
  leaders: { "lead-0-south": leader("st-0", free(15, 0), [3, -2.2], "Curtain wall, see detail") },
});

const collections = ["dimensions", "tags", "text_notes", "leaders"] as const;
const elementCollections = ["walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "spaces", "grids"] as const;

export function annotationsFor(name: string, model: Json): Json {
  const made: Json = { annotation_styles: styles(), ...(name === "house" ? house() : office()) };
  const exists = (id: string) => elementCollections.some((collection) => model[collection]?.[id] !== undefined);
  const anchorElements = (anchor: Json) => Object.values<any>(anchor).flatMap((body) => [body.wall, body.opening, body.grid, body.column].filter(Boolean));
  for (const collection of collections) {
    for (const [id, row] of Object.entries<any>(made[collection])) {
      if (model.storeys[row.storey] === undefined) throw new Error(`${collection}/${id}: storey ${row.storey}`);
      if (made.annotation_styles[row.style] === undefined) throw new Error(`${collection}/${id}: style ${row.style}`);
      const missing = [...(row.anchors ?? []), ...(row.anchor ? [row.anchor] : [])].flatMap(anchorElements).concat(row.element ? [row.element] : []).filter((element: string) => !exists(element));
      if (missing.length > 0) throw new Error(`${collection}/${id}: no element ${missing.join(", ")}`);
    }
  }
  return made;
}
