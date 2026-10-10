/**
 * 🪑️ Wave W2 `w2-f3-assets`: the furnished rooms of the `house` example and the equipped first floor of the `office` example, merged into the snapshot by `r4-x-examples-gen.ts`.
 *
 * HOUSE: a bed, a dining table with chairs, a kitchen unit, a WC, a basin and two lamps, each an instance of a parametric family (bed, table, chair, kitchen unit, WC, basin, lamp) with a few per-instance
 * overrides (table width and depth, chair seat height, bed width); the kitchen unit, the WC, the basin and the beds cling to their host walls; the lamps are terminals of the lighting system joined by a cable tray.
 * OFFICE (first floor): desks (instances of the table family of the families kit with overridden widths) with chairs, supply and return diffusers fed by ducts, pendant lamps fed by a lighting tray, a power tray,
 * and a hosted basin with a tap and the domestic water and waste pipes that meet their connectors. Every value is authored: components, their overrides and the routed MEP elements; the placement on a wall,
 * the solids, volumes and clashes are inferred. `componentsFor(name, model)` returns the collections to merge and refuses every dangling reference.
 */
import { cuboid, parameter, solid, tableFamily } from "./r12-w2-f2-families-examples.ts";

type Json = Record<string, any>;
export const P = (x: number, y: number) => ({ x, y });
export const Q = (x: number, y: number, z: number) => ({ x, y, z });
const centred = (size: string) => `0 m - ${size} / 2`;
const material = (id: string) => `"${id}"`;

export const family = (id: string, name: string, category: string, parameters: [string, string, string][], solids: [string, ReturnType<typeof cuboid>, string][]) => ({
  families: { [id]: { name, category } },
  family_parameters: Object.fromEntries(parameters.map(([n, kind, value]) => parameter(id, n, kind, value))),
  family_solids: Object.fromEntries(solids.map(([n, shape, m]) => [`fs-${id.replace("fam-", "")}-${n.toLowerCase().replace(/[^a-z0-9]+/g, "-")}`, solid(id, n, shape, material(m))])),
});

export const chair = () =>
  family("fam-chair", "Chair", "Furniture", [
    ["seat_width", "Length", "0.45 m"],
    ["seat_depth", "Length", "0.45 m"],
    ["seat_height", "Length", "0.45 m"],
    ["back_height", "Length", "0.45 m"],
    ["plank", "Length", "40 mm"],
    ["leg", "Length", "40 mm"],
    ["leg_x", "Length", "seat_width - leg"],
    ["leg_y", "Length", "seat_depth - leg"],
    ["leg_height", "Length", "seat_height - plank"],
  ], [
    ["Seat", cuboid("0 m", "0 m", "leg_height", "seat_width", "seat_depth", "plank"), "m-oak"],
    ["Back", cuboid("0 m", "0 m", "seat_height", "seat_width", "plank", "back_height"), "m-oak"],
    ["Leg 1", cuboid("0 m", "0 m", "0 m", "leg", "leg", "leg_height"), "m-steel"],
    ["Leg 2", cuboid("leg_x", "0 m", "0 m", "leg", "leg", "leg_height"), "m-steel"],
    ["Leg 3", cuboid("leg_x", "leg_y", "0 m", "leg", "leg", "leg_height"), "m-steel"],
    ["Leg 4", cuboid("0 m", "leg_y", "0 m", "leg", "leg", "leg_height"), "m-steel"],
  ]);

export const bed = () =>
  family("fam-bed", "Bed", "Furniture", [
    ["width", "Length", "1.8 m"],
    ["length", "Length", "2 m"],
    ["frame_height", "Length", "0.35 m"],
    ["leg_height", "Length", "0.1 m"],
    ["mattress_height", "Length", "0.22 m"],
    ["head_height", "Length", "1 m"],
    ["head_thickness", "Length", "60 mm"],
  ], [
    ["Base", cuboid(centred("width"), "0 m", "leg_height", "width", "length", "frame_height - leg_height"), "m-timber"],
    ["Headboard", cuboid(centred("width"), "0 m", "leg_height", "width", "head_thickness", "head_height - leg_height"), "m-timber"],
    ["Mattress", cuboid("0 m - width / 2 + 0.04 m", "head_thickness", "frame_height", "width - 0.08 m", "length - head_thickness - 0.04 m", "mattress_height"), "m-plaster"],
  ]);

export const kitchen = () =>
  family("fam-kitchen", "Kitchen unit", "Casework", [
    ["width", "Length", "2.4 m"],
    ["depth", "Length", "0.6 m"],
    ["height", "Length", "0.9 m"],
    ["plinth", "Length", "0.1 m"],
    ["worktop", "Length", "40 mm"],
    ["overhang", "Length", "20 mm"],
  ], [
    ["Carcass", cuboid(centred("width"), "0 m", "plinth", "width", "depth", "height - plinth - worktop"), "m-timber"],
    ["Worktop", cuboid(centred("width"), "0 m", "height - worktop", "width", "depth + overhang", "worktop"), "m-ceramic"],
  ]);

export const wc = () =>
  family("fam-wc", "WC", "Plumbing", [
    ["width", "Length", "0.38 m"],
    ["depth", "Length", "0.68 m"],
    ["bowl_height", "Length", "0.4 m"],
    ["cistern_depth", "Length", "0.2 m"],
    ["cistern_height", "Length", "0.8 m"],
  ], [
    ["Cistern", cuboid(centred("width"), "0 m", "0 m", "width", "cistern_depth", "cistern_height"), "m-ceramic"],
    ["Bowl", cuboid(centred("width"), "cistern_depth", "0 m", "width", "depth - cistern_depth", "bowl_height"), "m-ceramic"],
  ]);

export const basin = () =>
  family("fam-basin", "Basin", "Plumbing", [
    ["width", "Length", "0.55 m"],
    ["depth", "Length", "0.45 m"],
    ["bowl_height", "Length", "0.18 m"],
  ], [["Bowl", cuboid(centred("width"), "0 m", "0 m", "width", "depth", "bowl_height"), "m-ceramic"]]);

export const tap = () =>
  family("fam-tap", "Tap", "Plumbing", [
    ["body", "Length", "30 mm"],
    ["reach", "Length", "0.18 m"],
    ["tap_height", "Length", "0.12 m"],
  ], [["Body", cuboid(centred("body"), "0 m", "0 m", "body", "reach", "tap_height"), "m-steel"]]);

export const lamp = () =>
  family("fam-lamp", "Pendant lamp", "Lighting", [
    ["drop", "Length", "0.5 m"],
    ["shade", "Length", "0.4 m"],
    ["shade_height", "Length", "0.18 m"],
    ["cord", "Length", "12 mm"],
  ], [
    ["Cord", cuboid(centred("cord"), centred("cord"), "0 m - drop", "cord", "cord", "drop"), "m-steel"],
    ["Shade", cuboid(centred("shade"), centred("shade"), "0 m - drop - shade_height", "shade", "shade", "shade_height"), "m-paint"],
  ]);

export const diffuser = () =>
  family("fam-diffuser", "Air terminal", "Mechanical", [
    ["size", "Length", "0.6 m"],
    ["thickness", "Length", "80 mm"],
  ], [["Plate", cuboid(centred("size"), centred("size"), "0 m - thickness", "size", "size", "thickness"), "m-steel"]]);

export const merge = (...parts: Json[]) => ({
  families: Object.assign({}, ...parts.map((p) => p.families)),
  family_parameters: Object.assign({}, ...parts.map((p) => p.family_parameters)),
  family_solids: Object.assign({}, ...parts.map((p) => p.family_solids)),
});

export const component = (storey: string, family: string, position: { x: number; y: number }, elevation: number, rotation: number, name: string, extra: Json = {}) => ({ storey, family, position, elevation, rotation, mirrored: false, ...extra, name });
export const override = (component: string, name: string, value: string) => [`${component}.${name}`, { component, name, value }] as const;
export const duct = (storey: string, system: string, width: number, height: number, path: [number, number, number][], name: string) => ({ storey, system, shape: { Duct: { width, height } }, path: path.map(([x, y, z]) => Q(x, y, z)), name });
export const pipe = (storey: string, system: string, diameter: number, path: [number, number, number][], name: string) => ({ storey, system, shape: { Pipe: { diameter } }, path: path.map(([x, y, z]) => Q(x, y, z)), name });
export const tray = (storey: string, system: string, width: number, height: number, path: [number, number, number][], name: string) => ({ storey, system, shape: { Tray: { width, height } }, path: path.map(([x, y, z]) => Q(x, y, z)), name });

const houseParts = () => {
  const families = merge(bed(), tableFamily("m-oak"), chair(), kitchen(), wc(), basin(), lamp());
  const components: Json = {
    "cmp-bed-master": component("st-upper", "fam-bed", P(3.8, 2.8), 0, 0, "Master Bed", { host: "w-u-spine" }),
    "cmp-bed-child": component("st-upper", "fam-bed", P(8.5, 5.0), 0, 0, "Child Bed", { host: "w-u-bedrooms" }),
    "cmp-table-dining": component("st-ground", "fam-table", P(5, 5.6), 0, 0, "Dining Table"),
    "cmp-chair-1": component("st-ground", "fam-chair", P(5.3, 5.2), 0, 0, "Dining Chair 1"),
    "cmp-chair-2": component("st-ground", "fam-chair", P(6.0, 5.2), 0, 0, "Dining Chair 2"),
    "cmp-chair-3": component("st-ground", "fam-chair", P(5.75, 6.85), 0, Math.PI, "Dining Chair 3"),
    "cmp-chair-4": component("st-ground", "fam-chair", P(6.45, 6.85), 0, Math.PI, "Dining Chair 4"),
    "cmp-kitchen-unit": component("st-ground", "fam-kitchen", P(7.6, 7.6), 0, 0, "Kitchen Unit", { host: "w-g-north" }),
    "cmp-wc": component("st-ground", "fam-wc", P(0.5, 7.0), 0, 0, "WC", { host: "w-g-west" }),
    "cmp-basin": component("st-ground", "fam-basin", P(1.6, 6.4), 0.8, 0, "Basin", { host: "w-g-wc" }),
    "cmp-lamp-1": component("st-ground", "fam-lamp", P(5.5, 2.5), 2.6, 0, "Living Lamp 1", { system: "Lighting" }),
    "cmp-lamp-2": component("st-ground", "fam-lamp", P(8.0, 2.5), 2.6, 0, "Living Lamp 2", { system: "Lighting" }),
  };
  const overrides = Object.fromEntries([
    override("cmp-bed-child", "width", "0.9 m"),
    override("cmp-bed-child", "length", "1.9 m"),
    override("cmp-table-dining", "width", "1.8 m"),
    override("cmp-table-dining", "depth", "0.9 m"),
    override("cmp-chair-1", "seat_height", "0.47 m"),
    override("cmp-chair-3", "seat_height", "0.47 m"),
    override("cmp-kitchen-unit", "width", "2.1 m"),
    override("cmp-lamp-2", "drop", "0.7 m"),
  ]);
  const mep = { "mep-tray-living": tray("st-ground", "Lighting", 0.1, 0.05, [[5.5, 2.5, 2.6], [8.0, 2.5, 2.6]], "Living Lighting Tray") };
  return { ...families, components, component_overrides: overrides, mep_elements: mep };
};

const officeParts = () => {
  const families = merge(tableFamily("m-oak"), chair(), diffuser(), basin(), tap(), lamp());
  const components: Json = {};
  const overrides: Json = {};
  const desks: [number, number][] = [[2, 1.6], [6.2, 1.8], [10.4, 1.4], [14.6, 1.8], [18.8, 1.6], [23, 1.4]];
  desks.forEach(([x, width], index) => {
    const n = index + 1;
    components[`cmp-desk-${n}`] = component("st-1", "fam-table", P(x, 1.0), 0, 0, `Desk ${n}`);
    Object.assign(overrides, Object.fromEntries([override(`cmp-desk-${n}`, "width", `${width} m`), override(`cmp-desk-${n}`, "depth", "0.7 m"), ...(n % 2 === 0 ? [override(`cmp-desk-${n}`, "height", "0.72 m")] : [])]));
    components[`cmp-chair-${n}`] = component("st-1", "fam-chair", P(x + width / 2 + 0.225, 2.15), 0, Math.PI, `Office Chair ${n}`);
    if (n % 3 === 0) Object.assign(overrides, Object.fromEntries([override(`cmp-chair-${n}`, "seat_height", "0.5 m")]));
  });
  [5, 12, 19, 26].forEach((x, i) => (components[`cmp-supply-${i + 1}`] = component("st-1", "fam-diffuser", P(x, 3), 2.8, 0, `Supply Diffuser ${i + 1}`, { system: "Supply" })));
  [8, 15, 22].forEach((x, i) => (components[`cmp-return-${i + 1}`] = component("st-1", "fam-diffuser", P(x, 15), 2.8, 0, `Return Grille ${i + 1}`, { system: "Return" })));
  [3, 9, 15, 21].forEach((x, i) => (components[`cmp-lamp-${i + 1}`] = component("st-1", "fam-lamp", P(x, 1), 3.1, 0, `Pendant Lamp ${i + 1}`, { system: "Lighting" })));
  components["cmp-basin"] = component("st-1", "fam-basin", P(6.7, 9), 0.8, 0, "Pantry Basin", { host: "w-1-core-w-west", system: "Waste" });
  components["cmp-tap"] = component("st-1", "fam-tap", P(6.7, 9), 0.95, 0, "Pantry Tap", { host: "w-1-core-w-west", system: "DomesticWater" });
  Object.assign(overrides, Object.fromEntries([override("cmp-lamp-3", "drop", "0.8 m"), override("cmp-supply-4", "size", "0.45 m"), override("cmp-basin", "width", "0.6 m")]));
  const mep: Json = {
    "mep-supply-main": duct("st-1", "Supply", 0.4, 0.25, [[1, 3, 3.0], [27, 3, 3.0]], "Supply Main"),
    "mep-return-main": duct("st-1", "Return", 0.4, 0.25, [[1, 15, 3.0], [24, 15, 3.0]], "Return Main"),
    "mep-lighting-main": tray("st-1", "Lighting", 0.3, 0.06, [[1, 1, 3.35], [23, 1, 3.35]], "Lighting Tray"),
    "mep-power-main": tray("st-1", "Power", 0.3, 0.06, [[1, 16.5, 3.1], [29, 16.5, 3.1]], "Power Tray"),
    "mep-water": pipe("st-1", "DomesticWater", 0.02, [[6.875, 10.5, 0.95], [6.875, 9, 0.95]], "Pantry Cold Water"),
    "mep-waste": pipe("st-1", "Waste", 0.05, [[6.875, 9, 0.8], [6.875, 9, 0.2], [6.875, 10.5, 0.2]], "Pantry Waste"),
  };
  [5, 12, 19, 26].forEach((x, i) => (mep[`mep-supply-drop-${i + 1}`] = duct("st-1", "Supply", 0.3, 0.3, [[x, 3, 3.0], [x, 3, 2.8]], `Supply Drop ${i + 1}`)));
  [8, 15, 22].forEach((x, i) => (mep[`mep-return-drop-${i + 1}`] = duct("st-1", "Return", 0.3, 0.3, [[x, 15, 3.0], [x, 15, 2.8]], `Return Drop ${i + 1}`)));
  [3, 9, 15, 21].forEach((x, i) => (mep[`mep-lighting-stub-${i + 1}`] = tray("st-1", "Lighting", 0.1, 0.05, [[x, 1, 3.35], [x, 1, 3.1]], `Lighting Stub ${i + 1}`)));
  return { ...families, components, component_overrides: overrides, mep_elements: mep };
};

const REQUIRED = ["components", "component_overrides", "mep_elements"] as const;

function verify(model: Json, made: Json) {
  const walls = model.walls as Json;
  const parameters = new Set(Object.values<Json>(model.family_parameters).map((row) => `${row.family}.${row.name}`));
  for (const [id, row] of Object.entries<Json>(made.components)) {
    if (model.storeys[row.storey] === undefined) throw new Error(`components/${id}: storey ${row.storey}`);
    const kind = model.families[row.family];
    if (kind === undefined || kind.category === "Profile") throw new Error(`components/${id}: family ${row.family} is missing or a profile`);
    if (row.host !== undefined) {
      if (walls[row.host] === undefined || walls[row.host].storey !== row.storey) throw new Error(`components/${id}: host ${row.host} is no wall of ${row.storey}`);
    }
    if (![row.position.x, row.position.y, row.elevation, row.rotation].every(Number.isFinite)) throw new Error(`components/${id}: not finite`);
  }
  for (const [key, row] of Object.entries<Json>(made.component_overrides)) {
    const owner = made.components[row.component];
    if (owner === undefined) throw new Error(`component_overrides/${key}: component ${row.component}`);
    if (key !== `${row.component}.${row.name}`) throw new Error(`component_overrides/${key}: the key is not component.name`);
    if (!parameters.has(`${owner.family}.${row.name}`)) throw new Error(`component_overrides/${key}: ${owner.family} has no parameter ${row.name}`);
  }
  for (const [id, row] of Object.entries<Json>(made.mep_elements)) {
    if (model.storeys[row.storey] === undefined) throw new Error(`mep_elements/${id}: storey ${row.storey}`);
    const sizes = Object.values<Json>(row.shape)[0];
    if (!Object.values<number>(sizes).every((size) => size > 0)) throw new Error(`mep_elements/${id}: a dimension is not positive`);
    if (row.path.length < 2) throw new Error(`mep_elements/${id}: the path has fewer than two points`);
    row.path.forEach((point: Json, i: number) => {
      if (i > 0 && Math.hypot(point.x - row.path[i - 1].x, point.y - row.path[i - 1].y, point.z - row.path[i - 1].z) < 1e-9) throw new Error(`mep_elements/${id}: repeated vertex ${i}`);
    });
  }
  const taken = new Set<string>();
  for (const name of REQUIRED) for (const id of Object.keys(made[name])) {
    if (name !== "component_overrides" && taken.has(id)) throw new Error(`${name}/${id}: the id is used twice`);
    taken.add(id);
  }
}

export function componentsFor(name: string, model: Json): Json {
  const parts = name === "house" ? houseParts() : officeParts();
  const merged = { ...parts, families: { ...model.families, ...parts.families }, family_parameters: { ...model.family_parameters, ...parts.family_parameters }, family_solids: { ...model.family_solids, ...parts.family_solids } };
  verify({ ...model, ...merged }, merged);
  return merged;
}
