#!/usr/bin/env bun
/**
 * 🏘️ Wave W07 fixtures: the committed snapshots of the finish and zone inference cases. `bun r10-w07-zones-fixtures.ts` writes
 * `🧫️fixtures/💡️inferences/🎨️finishes/🏡️rooms/📸️snapshot/🔣️.json` (two rooms around a partition with windows, a door, a column, an island wall and two slabs of
 * different slope above, an open space and an upstairs explicit room) and `…/🏘️zones/🏡️zoning/📸️snapshot/🔣️.json` (the same model with zones and area schemes).
 * The expected tables beside them are written by the third-party oracles (`🐍️.py write`), never by hand.
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import * as F from "./r3-f1-fixtures.ts";
import { em, fixtures, JSONF } from "./r3-f1-paths.ts";

const loop = (points: [number, number][]) => points.map(([x, y]) => ({ point: F.P(x, y), bulge: 0 }));
const slab = (storey: string, points: [number, number][], slope?: { direction: number; angle: number }) => ({ storey, slab_type: "slt-250", boundary: loop(points), holes: [], offset: 0, ...(slope ? { slope } : {}), phase: "New", name: "Slab" });
const space = (storey: string, number: string, name: string, boundary: unknown, usage: string, over: Record<string, unknown> = {}) => ({ storey, number, name, boundary, usage, phase: "New", ...over });
const bounded = (x: number, y: number) => ({ Bounded: { seed: F.P(x, y) } });
const explicit = (points: [number, number][]) => ({ Explicit: { outline: loop(points) } });
const opening = (host: string, kind: unknown, offset: number, name: string) => ({ host, kind, offset, flip_hand: false, flip_facing: false, name });

export function house(withZones: boolean) {
  const wall = (id: string, a: [number, number], b: [number, number]) => [id, F.wall("st-ground", "wt-300", F.line(a, b), F.storeyTop(0), id)] as const;
  const snapshot: any = F.snap(
    {
      materials: Object.fromEntries(["m-board", "m-brick", "m-concrete", "m-paint", "m-tile", "m-wood"].map((id) => [id, F.material(id.slice(2))])),
      wall_types: { "wt-300": F.wallType("Brick 300", [F.layer("m-brick", 0.3)]) },
      slab_types: { "slt-250": F.wallType("Concrete 250", [F.layer("m-concrete", 0.25)]) },
      column_types: { "ct-400": { name: "Column 400", profile: { Rectangle: { width: 0.4, depth: 0.4 } }, material: "m-concrete" } },
      window_types: { "win-120": { name: "Window 120", width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.05, frame_depth: 0.1, panes: 2, material: "m-wood" } },
      door_types: { "door-90": { name: "Door 90", width: 0.9, height: 2.1, frame_width: 0.05, frame_depth: 0.1, leaves: "Single", swing: "Left", material: "m-wood" } },
      sites: { "site-1": F.site("Plot") },
      buildings: { "bldg-1": F.building("site-1", "House") },
      storeys: { "st-ground": F.storey("bldg-1", "Ground", 0, 3), "st-first": F.storey("bldg-1", "First", 1, 2.8) },
      walls: Object.fromEntries([wall("w-south", [0, 0], [8, 0]), wall("w-east", [8, 0], [8, 6]), wall("w-north", [8, 6], [0, 6]), wall("w-west", [0, 6], [0, 0]), wall("w-part", [4, 0], [4, 6]), wall("w-island", [5.5, 1], [5.5, 2])]),
      columns: { "c-east": { storey: "st-ground", column_type: "ct-400", position: F.P(6.5, 4.5), rotation: 0, base_offset: 0, top: F.storeyTop(0), phase: "New", name: "Column" } },
      openings: {
        "o-win-south": opening("w-south", { Window: { window_type: "win-120" } }, 2, "South window"),
        "o-door-part": opening("w-part", { Door: { door_type: "door-90" } }, 3, "Partition door"),
        "o-win-east": opening("w-east", { Window: { window_type: "win-120" } }, 3, "East window"),
      },
      slabs: {
        "sl-west": slab("st-first", [[0, 0], [4, 0], [4, 6], [0, 6]]),
        "sl-east": slab("st-first", [[4, 0], [8, 0], [8, 6], [4, 6]], { direction: 0, angle: 0.2 }),
      },
      spaces: {
        "sp-west": space("st-ground", "0.01", "Living", bounded(2, 3), "Living", { ...(withZones ? { zone: "z-day" } : {}), floor_finish: "m-tile", wall_finish: "m-paint", ceiling_finish: "m-paint" }),
        "sp-east": space("st-ground", "0.02", "Kitchen", bounded(6, 3), "Kitchen", { ...(withZones ? { zone: "z-night" } : {}), floor_finish: "m-wood", wall_finish: "m-paint" }),
        "sp-out": space("st-ground", "0.03", "Outside", bounded(12, 3), "Living", withZones ? { zone: "z-day" } : {}),
        "sp-up": space("st-first", "1.01", "Bedroom", explicit([[0, 0], [4, 0], [4, 3], [0, 3]]), "Sleeping", { ...(withZones ? { zone: "z-night" } : {}), floor_finish: "m-wood" }),
      },
    },
    withZones ? "Zoning" : "Rooms",
  );
  snapshot.ceiling_types = { "cet-board": { name: "Board 12.5", layers: [F.layer("m-board", 0.0125, "Finish")] }, "cet-deep": { name: "Board 50", layers: [F.layer("m-board", 0.05, "Finish")] } };
  snapshot.ceilings = {
    "ce-living": { storey: "st-ground", ceiling_type: "cet-board", boundary: loop([[0, 0], [3, 0], [3, 6], [0, 6]]), holes: [], offset: 0.4, slope: { direction: 1.5707963267948966, angle: 0.05 }, name: "Living ceiling" },
    "ce-kitchen": { storey: "st-ground", ceiling_type: "cet-deep", boundary: loop([[4, 0], [8, 0], [8, 6], [4, 6]]), holes: [loop([[7, 4.5], [7.5, 4.5], [7.5, 5.5], [7, 5.5]])], offset: 0.2, slope: { direction: 0, angle: 0.1 }, name: "Kitchen sloped ceiling" },
  };
  if (withZones) {
    snapshot.zones = { "z-day": { name: "Day zone", category: "Ventilation", occupancy_density: 0.1 }, "z-night": { name: "Night zone", category: "Ventilation", occupancy_density: 0.05 } };
    snapshot.area_schemes = {
      "as-gfa": { name: "Gross floor area", measure: "Gross", usages: [], zones: [] },
      "as-nsa": { name: "Net use area", measure: "Net", usages: ["Living", "Kitchen"], zones: [] },
      "as-day": { name: "Day zone area", measure: "Net", usages: [], zones: ["z-day"] },
      "as-sleep": { name: "Sleeping area", measure: "Net", usages: ["Sleeping"], zones: ["z-night"] },
    };
  }
  return snapshot;
}

export function oneRoom() {
  const wall = (id: string, a: [number, number], b: [number, number]) => [id, F.wall("st-ground", "wt-300", F.line(a, b), F.storeyTop(0), id)] as const;
  return F.snap(
    {
      materials: Object.fromEntries(["m-brick", "m-paint", "m-tile", "m-wood"].map((id) => [id, F.material(id.slice(2))])),
      wall_types: { "wt-300": F.wallType("Brick 300", [F.layer("m-brick", 0.3)]) },
      window_types: { "win-120": { name: "Window 120", width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.05, frame_depth: 0.1, panes: 2, material: "m-wood" } },
      door_types: { "door-90": { name: "Door 90", width: 0.9, height: 2.1, frame_width: 0.05, frame_depth: 0.1, leaves: "Single", swing: "Left", material: "m-wood" } },
      sites: { "site-1": F.site("Plot", 0) },
      buildings: { "bldg-1": F.building("site-1", "House") },
      storeys: { "st-ground": F.storey("bldg-1", "Ground", 0, 3) },
      walls: Object.fromEntries([wall("w-south", [0, 0], [6, 0]), wall("w-east", [6, 0], [6, 4]), wall("w-north", [6, 4], [0, 4]), wall("w-west", [0, 4], [0, 0])]),
      openings: {
        "o-win-south": opening("w-south", { Window: { window_type: "win-120" } }, 1.5, "South window"),
        "o-win-north": opening("w-north", { Window: { window_type: "win-120" } }, 4, "North window"),
        "o-door-east": opening("w-east", { Door: { door_type: "door-90" } }, 2, "East door"),
      },
      spaces: { "sp-room": space("st-ground", "0.01", "Room", bounded(3, 2), "Living", { floor_finish: "m-tile", wall_finish: "m-paint", ceiling_finish: "m-paint" }) },
    },
    "One room",
  );
}

const inferences = join(fixtures, em(0x1f4a1) + "inferences");
const write = (field: string, name: string, snapshot: unknown) => {
  const file = join(inferences, field, name, em(0x1f4f8) + "snapshot", JSONF);
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(file, JSON.stringify(snapshot, null, 2) + "\n");
};
write(em(0x1f3a8) + "finishes", em(0x1f3e1) + "rooms", house(false));
write(em(0x1f3a8) + "finishes", em(0x1f3e0) + "one-room", oneRoom());
write(em(0x1f3d8) + "zones", em(0x1f3e1) + "zoning", house(true));
console.log("fixtures written");
