#!/usr/bin/env bun
/**
 * 📋️ R11 `w13-schedules`: the committed models of the case `infer-bim-1-schedules`. `bun r11-w13-schedules-fixtures.ts` derives them from the committed IFC house (the closed-form elements the sibling oracles measure with shapely: walls, slabs,
 * columns, beams, rooms, doors and windows; the roofs, stairs, railings and curtain walls with their openings are dropped), gives the rooms their finishes, renames two walls so the natural order has digits to compare, and authors nine schedules that
 * cover the vocabulary: presets, a filter, a property column, nested grouping, a collapsed table, a storey and phase scope and a material take-off. The expected tables are WRITTEN by `🐍️.py write`, never here.
 * Cases: `🏡️house` (the model) and `✂️wall-edit` (the same model plus the committed `setWallTop` that frees the height of a wall: the quantity edit the schedules must follow).
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { em, fixtures } from "./r3-f1-paths.ts";

type Json = Record<string, any>;
const source = JSON.parse(readFileSync(join(fixtures, em(0x1f3d7) + "ifc", em(0x1f3e0) + "house", em(0x1f4f8) + "snapshot", em(0x1f523) + ".json"), "utf8"));

const pascal = (token: string) => token.split("_").map((part) => part[0].toUpperCase() + part.slice(1)).join("");
const field = (token: string) => ({ Field: { field: pascal(token) } });
const property = (set: string, name: string) => ({ Property: { set, name } });
const col = (key: unknown, total = false, heading?: string) => ({ key, ...(heading === undefined ? {} : { heading }), total });
const sortBy = (key: unknown, descending = false) => ({ key, descending });
const filter = (key: unknown, op: string, value: string) => ({ key, op, value });
const groupBy = (key: unknown) => ({ key });
const schedule = (name: string, category: string, columns: unknown[], over: Json = {}) => ({ name, category, columns, sort: [], filter: [], group: [], itemize: true, storeys: [] as string[], phases: [] as string[], ...over });

const model = structuredClone(source) as Json;
for (const dropped of ["curtain_walls", "roofs", "stairs", "railings"]) model[dropped] = {};
model.openings = Object.fromEntries(Object.entries(model.openings as Json).filter(([, opening]) => model.walls[opening.host] !== undefined));
const alive = (id: string) => ["walls", "columns", "beams", "slabs", "spaces", "openings"].some((collection) => model[collection][id] !== undefined);
model.properties = Object.fromEntries(Object.entries(model.properties as Json).filter(([id]) => alive(id)));
model.classifications = Object.fromEntries(Object.entries(model.classifications as Json).filter(([id]) => alive(id)));
model.walls["w-east"].name = "Wall 10";
model.walls["w-north"].name = "Wall 2";
model.walls["w-south"].name = "Wall 1";
model.walls["w-west"].name = "Wall 3";
Object.assign(model.spaces["sp-1"], { floor_finish: "m-wood", wall_finish: "m-brick" });
Object.assign(model.spaces["sp-2"], { floor_finish: "m-wood", wall_finish: "m-conc", ceiling_finish: "m-steel" });
const rating = property("Pset_WallCommon", "FireRating");

model.schedules = {
  "sch-doors": schedule("Door schedule", "Door", [col(field("name")), col(field("type")), col(field("storey")), col(field("width")), col(field("height")), col(field("leaves")), col(field("swing")), col(field("count"), true)], { sort: [sortBy(field("storey")), sortBy(field("name"))] }),
  "sch-windows": schedule("Window schedule", "Window", [col(field("name")), col(field("type")), col(field("storey")), col(field("width")), col(field("height")), col(field("panes")), col(field("host")), col(field("count"), true)], { sort: [sortBy(field("type")), sortBy(field("width"), true)] }),
  "sch-rooms": schedule("Room schedule", "Space", [col(field("number")), col(field("name")), col(field("storey")), col(field("usage")), col(field("gross_area"), true), col(field("net_area"), true), col(field("height")), col(field("net_volume"), true)], { sort: [sortBy(field("number"))], group: [groupBy(field("storey"))] }),
  "sch-finishes": schedule("Room finish schedule", "Finish", [col(field("number")), col(field("name")), col(field("surface")), col(field("material")), col(field("finish_area"), true)], { sort: [sortBy(field("number")), sortBy(field("surface"))], group: [groupBy(field("storey"))] }),
  "sch-walls": schedule("Wall schedule", "Wall", [col(field("name")), col(field("type")), col(field("storey")), col(field("length"), true), col(field("height")), col(field("net_side_area"), true), col(field("net_volume"), true), col(rating, false, "Fire rating"), col(field("material"))], { sort: [sortBy(field("name"))], group: [groupBy(field("type"))], filter: [filter(field("length"), "GreaterOrEqual", "1")] }),
  "sch-nested": schedule("Walls by storey and type", "Wall", [col(field("id")), col(field("phase")), col(field("length"), true), col(field("gross_side_area"), true)], { group: [groupBy(field("storey")), groupBy(field("type"))], sort: [sortBy(field("length"), true)] }),
  "sch-types": schedule("Wall types", "Wall", [col(field("type")), col(field("count"), true), col(field("length"), true), col(field("net_volume"), true)], { itemize: false }),
  "sch-ground": schedule("New ground floor walls", "Wall", [col(field("name")), col(field("storey")), col(field("length")), col(field("phase"))], { sort: [sortBy(field("length"), true), sortBy(field("name"))], storeys: ["st-ground"], phases: ["New"], filter: [filter(rating, "Empty", "")] }),
  "sch-takeoff": schedule("Material take-off", "Material", [col(field("material")), col(field("layer_area"), true), col(field("layer_volume"), true), col(field("layer_mass"), true), col(field("count"), true)], { group: [groupBy(field("material"))], itemize: false, filter: [filter(field("kind"), "NotEquals", "window"), filter(field("kind"), "NotEquals", "door")] }),
};

const edit = { mutation: "setWallTop", id: "w-south", top: { Unconnected: { height: 2.4 } } };
const write = (path: string[], content: unknown) => {
  const dir = join(fixtures, em(0x1f4a1) + "inferences", em(0x1f4cb) + "schedules", ...path.slice(0, -1));
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, path[path.length - 1]), JSON.stringify(content, null, 2) + "\n");
};
const snapshot = [em(0x1f4f8) + "snapshot", em(0x1f523) + ".json"];
write([em(0x1f3e1) + "house", ...snapshot], model);
write([em(0x2702) + "wall-edit", ...snapshot], model);
write([em(0x2702) + "wall-edit", em(0x1f9a0) + "mutation", em(0x1f523) + ".json"], edit);
console.log(`schedules: ${Object.keys(model.schedules).length}, walls ${Object.keys(model.walls).length}, openings ${Object.keys(model.openings).length}, spaces ${Object.keys(model.spaces).length}`);
