#!/usr/bin/env bun
/** 🏡️ Writes the `house` and `office` example snapshot sources (`🖼️assets/<example>/📸️snapshot.json`); the DSL texts are blessed from them by `BIM_BLESS=1 cargo test bless_the_`. Every value is authored; nothing derivable (elevations, heights, areas) is stored. */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, em, subset } from "./r3-f1-paths.ts";
import * as F from "./r3-f1-fixtures.ts";

type Pt = [number, number];
type Json = Record<string, unknown>;
const P = F.P;
const pt = (p: Pt) => P(p[0], p[1]);
const V = (x: number, y: number, bulge = 0) => ({ point: P(x, y), bulge });
const rect = (x0: number, y0: number, x1: number, y1: number) => [V(x0, y0), V(x1, y0), V(x1, y1), V(x0, y1)];
const arc = (a: Pt, b: Pt, bulge: number) => ({ Arc: { start: pt(a), end: pt(b), bulge } });
const line = (a: Pt, b: Pt) => F.line(a, b);

const MATERIALS: Record<string, [string, string, [number, number, number], number, number, number]> = {
  "m-concrete": ["Reinforced Concrete", "Concrete", [0.62, 0.62, 0.6], 2400, 2.3, 1000],
  "m-brick": ["Clay Block", "Masonry", [0.7, 0.35, 0.25], 800, 0.14, 1000],
  "m-insulation": ["Mineral Wool", "Insulation", [0.95, 0.85, 0.35], 40, 0.035, 1030],
  "m-plaster": ["Lime Gypsum Plaster", "Finish", [0.92, 0.92, 0.88], 1400, 0.7, 1000],
  "m-timber": ["Spruce Timber", "Wood", [0.8, 0.62, 0.4], 450, 0.13, 1600],
  "m-glass": ["Float Glass", "Glass", [0.65, 0.85, 0.9], 2500, 1.0, 750],
  "m-steel": ["Structural Steel", "Metal", [0.55, 0.58, 0.62], 7850, 50, 450],
  "m-screed": ["Cement Screed", "Finish", [0.75, 0.75, 0.72], 2000, 1.4, 1000],
  "m-membrane": ["Bitumen Membrane", "Membrane", [0.15, 0.15, 0.15], 1100, 0.17, 1000],
  "m-tile": ["Clay Roof Tile", "Masonry", [0.65, 0.25, 0.15], 1900, 1.0, 900],
  "m-gravel": ["Ballast Gravel", "Other", [0.6, 0.58, 0.55], 1800, 0.7, 840],
};
const materials = (ids: string[]) => Object.fromEntries(ids.map((id) => {
  const [name, category, [r, g, b], density, conductivity, specific_heat] = MATERIALS[id];
  return [id, { name, category, color: { r, g, b }, density, conductivity, specific_heat }];
}));

const layer = F.layer;
const stack = (name: string, layers: ReturnType<typeof layer>[]) => ({ name: `${name} ${Math.round(layers.reduce((sum, l) => sum + l.thickness, 0) * 1000) / 10}`, layers });
const windowType = (name: string, width: number, height: number, sill: number, panes: number, material: string) => ({ name, width, height, sill, frame_width: 0.07, frame_depth: 0.08, panes, material });
const doorType = (name: string, width: number, height: number, leaves: string, swing: string, material: string) => ({ name, width, height, frame_width: 0.07, frame_depth: 0.08, leaves, swing, material });
const rectProfile = (width: number, depth: number) => ({ Rectangle: { width, depth } });

const wall = (storey: string, type: string, axis: unknown, top: unknown, name: string) => F.wall(storey, type, axis, top, name);
const column = (storey: string, type: string, at: Pt, name: string, top: unknown = F.storeyTop(0)) => ({ storey, column_type: type, position: pt(at), rotation: 0, base_offset: 0, top, name });
const beam = (storey: string, type: string, start: Pt, end: Pt, top_offset: number, name: string) => ({ storey, beam_type: type, start: pt(start), end: pt(end), top_offset, name });
const slab = (storey: string, type: string, boundary: unknown[], holes: unknown[][], name: string) => ({ storey, slab_type: type, boundary, holes, offset: 0, name });
const roof = (storey: string, type: string, footprint: unknown[], shape: unknown, overhang: number, base_offset: number, name: string) => ({ storey, roof_type: type, footprint, shape, overhang, base_offset, name });
type StairConstruction = { stringer?: { kind: string; width: number; depth: number }; nosing?: number; tread_thickness?: number; riser?: string };
const stair = (storey: string, start: Pt, direction: number, width: number, flight: unknown, max_riser: number, min_tread: number, name: string, c: StairConstruction = {}) => ({
  storey, start: pt(start), direction, width, flight, top: F.storeyTop(0), max_riser, min_tread, stringer: c.stringer ?? { kind: "None", width: 0.05, depth: 0.25 }, nosing: c.nosing ?? 0, tread_thickness: c.tread_thickness ?? 0.04, riser: c.riser ?? "Closed", landing_depth: width, name,
});
type RailingConstruction = { profile?: unknown; post_profile?: unknown; baluster?: unknown; infill?: unknown };
const railing = (storey: string, path: Pt[], material: string, name: string, c: RailingConstruction = {}) => ({
  storey, path: path.map(pt), height: 1.0, post_spacing: 1.2, profile: c.profile ?? rectProfile(0.06, 0.04), post_profile: c.post_profile ?? rectProfile(0.05, 0.05), ...(c.baluster === undefined ? {} : { baluster: c.baluster }), infill: c.infill ?? "None", material, base_offset: 0, name,
});
const space = (storey: string, number: string, name: string, boundary: unknown, usage: string) => ({ storey, number, name, boundary, usage });
const seed = (x: number, y: number) => ({ Bounded: { seed: P(x, y) } });
const outline = (vertices: unknown[]) => ({ Explicit: { outline: vertices } });
const grid = (building: string, label: string, a: Pt, b: Pt) => ({ building, label, start: pt(a), end: pt(b) });
const curtainWall = (storey: string, axis: unknown, name: string) => ({ storey, axis, base_offset: 0, top: F.storeyTop(0), u_spacing: 1.5, v_spacing: 1.3, mullion: rectProfile(0.05, 0.15), panel_material: "m-glass", mullion_material: "m-steel", name });

type OpeningOptions = { width?: number; height?: number; sillOverride?: number; flipHand?: boolean; flipFacing?: boolean };
const opened = (host: string, kind: unknown, offset: number, name: string, o: OpeningOptions) => ({
  host, kind, offset, ...(o.sillOverride === undefined ? {} : { sill_override: o.sillOverride }), ...(o.width === undefined ? {} : { width: o.width }), ...(o.height === undefined ? {} : { height: o.height }), flip_hand: o.flipHand ?? false, flip_facing: o.flipFacing ?? false, name,
});
const win = (host: string, type: string, offset: number, name: string, o: OpeningOptions = {}) => opened(host, { Window: { window_type: type } }, offset, name, o);
const door = (host: string, type: string, offset: number, name: string, o: OpeningOptions = {}) => opened(host, { Door: { door_type: type } }, offset, name, o);
const hole = (host: string, offset: number, width: number, height: number, name: string) => opened(host, { Void: { width, height } }, offset, name, {});

const text = (value: string) => ({ Text: { value } });
const real = (value: number) => ({ Real: { value } });
const flag = (value: boolean) => ({ Boolean: { value } });
const classify = (system: string, code: string, title: string) => ({ system, code, title });

const reference = (path: string, id: unknown, ...collections: Json[]) => {
  if (typeof id !== "string" || !collections.some((c) => id in c)) throw new Error(`dangling reference ${path} -> ${String(id)}`);
};
const arcLength = (start: Pt, end: Pt, bulge: number) => {
  const chord = Math.hypot(end[0] - start[0], end[1] - start[1]);
  if (bulge === 0) return chord;
  const sweep = 4 * Math.atan(Math.abs(bulge));
  return (chord / (2 * Math.sin(sweep / 2))) * sweep;
};
const hostLength = (axis: any) => {
  const body = axis.Line ?? axis.Arc;
  return arcLength([body.start.x, body.start.y], [body.end.x, body.end.y], axis.Arc ? axis.Arc.bulge : 0);
};

function verify(model: any) {
  const c = (name: string): Json => model[name];
  for (const [id, w] of Object.entries<any>(c("walls"))) {
    reference(`walls/${id}/storey`, w.storey, c("storeys"));
    reference(`walls/${id}/wall_type`, w.wall_type, c("wall_types"));
    if (w.top.Storey) reference(`walls/${id}/top`, w.top.Storey.storey, c("storeys"));
  }
  for (const [name, storeyField] of [["curtain_walls", "storey"], ["columns", "storey"], ["beams", "storey"], ["slabs", "storey"], ["roofs", "storey"], ["stairs", "storey"], ["railings", "storey"], ["spaces", "storey"]] as const)
    for (const [id, e] of Object.entries<any>(c(name))) reference(`${name}/${id}/storey`, e[storeyField], c("storeys"));
  for (const [id, e] of Object.entries<any>(c("columns"))) reference(`columns/${id}/type`, e.column_type, c("column_types"));
  for (const [id, e] of Object.entries<any>(c("beams"))) reference(`beams/${id}/type`, e.beam_type, c("beam_types"));
  for (const [id, e] of Object.entries<any>(c("slabs"))) reference(`slabs/${id}/type`, e.slab_type, c("slab_types"));
  for (const [id, e] of Object.entries<any>(c("roofs"))) reference(`roofs/${id}/type`, e.roof_type, c("roof_types"));
  for (const [id, e] of Object.entries<any>(c("railings"))) reference(`railings/${id}/material`, e.material, c("materials"));
  for (const [id, e] of Object.entries<any>(c("curtain_walls"))) { reference(`curtain_walls/${id}/panel`, e.panel_material, c("materials")); reference(`curtain_walls/${id}/mullion`, e.mullion_material, c("materials")); }
  for (const kind of ["wall_types", "slab_types", "roof_types"]) for (const [id, t] of Object.entries<any>(c(kind))) for (const l of t.layers) reference(`${kind}/${id}/layer`, l.material, c("materials"));
  for (const kind of ["column_types", "beam_types", "window_types", "door_types"]) for (const [id, t] of Object.entries<any>(c(kind))) reference(`${kind}/${id}/material`, t.material, c("materials"));
  for (const [id, b] of Object.entries<any>(c("buildings"))) reference(`buildings/${id}/site`, b.site, c("sites"));
  for (const [id, s] of Object.entries<any>(c("storeys"))) reference(`storeys/${id}/building`, s.building, c("buildings"));
  for (const [id, g] of Object.entries<any>(c("grids"))) reference(`grids/${id}/building`, g.building, c("buildings"));
  const hosts = { ...c("walls"), ...c("curtain_walls") } as Record<string, any>;
  const byHost = new Map<string, { id: string; from: number; to: number }[]>();
  for (const [id, o] of Object.entries<any>(c("openings"))) {
    reference(`openings/${id}/host`, o.host, c("walls"), c("curtain_walls"));
    const k = o.kind.Window ?? o.kind.Door ?? o.kind.Void;
    const type = o.kind.Window ? c("window_types")[k.window_type] : o.kind.Door ? c("door_types")[k.door_type] : k;
    if (!type) throw new Error(`opening ${id} has no type`);
    const width = o.width ?? type.width;
    const height = o.height ?? type.height;
    const sill = o.sill_override ?? (o.kind.Window ? type.sill : 0);
    const length = hostLength(hosts[o.host].axis);
    const from = o.offset - width / 2;
    const to = o.offset + width / 2;
    if (from < 0.1 || to > length - 0.1) throw new Error(`opening ${id} leaves host ${o.host} (length ${length.toFixed(3)}, span ${from.toFixed(3)}..${to.toFixed(3)})`);
    const storey = c("storeys")[hosts[o.host].storey] as any;
    if (sill + height > storey.height - 0.1) throw new Error(`opening ${id} is taller than its storey`);
    (byHost.get(o.host) ?? byHost.set(o.host, []).get(o.host)!).push({ id, from, to });
  }
  for (const [host, rows] of byHost) for (const a of rows) for (const b of rows) if (a.id < b.id && a.from < b.to + 0.05 && b.from < a.to + 0.05) throw new Error(`openings ${a.id} and ${b.id} overlap on ${host}`);
  const owners = new Set(["walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "spaces"].flatMap((name) => Object.keys(c(name))));
  for (const name of ["properties", "classifications"]) for (const id of Object.keys(c(name))) if (!owners.has(id)) throw new Error(`${name} of unknown element ${id}`);
  const numbers = new Set<string>();
  for (const s of Object.values<any>(c("spaces"))) {
    const key = `${s.storey}/${s.number}`;
    if (numbers.has(key)) throw new Error(`duplicate space number ${key}`);
    numbers.add(key);
  }
  const levels = new Set<string>();
  for (const s of Object.values<any>(c("storeys"))) {
    const key = `${s.building}/${s.level}`;
    if (levels.has(key)) throw new Error(`duplicate level ${key}`);
    levels.add(key);
  }
}

//#region 🏡️ house
function house() {
  const bay: [Pt, Pt, number] = [[10, 1], [10, 4], 0.5];
  const exterior = (sid: string, storey: string, type: string, top: unknown, label: string) => ({
    [`w-${sid}-south`]: wall(storey, type, line([0, 0], [10, 0]), top, `${label} South`),
    [`w-${sid}-east-1`]: wall(storey, type, line([10, 0], [10, 1]), top, `${label} East South`),
    [`w-${sid}-bay`]: wall(storey, type, arc(...bay), top, `${label} Bay`),
    [`w-${sid}-east-2`]: wall(storey, type, line([10, 4], [10, 8]), top, `${label} East North`),
    [`w-${sid}-north`]: wall(storey, type, line([10, 8], [0, 8]), top, `${label} North`),
    [`w-${sid}-west`]: wall(storey, type, line([0, 8], [0, 0]), top, `${label} West`),
  });
  const top = F.storeyTop(0);
  const knee = F.unconnected(0.9);
  const walls: Record<string, unknown> = {
    ...exterior("b", "st-basement", "wt-base-37", top, "Basement"),
    "w-b-spine": wall("st-basement", "wt-base-25", line([3.6, 0], [3.6, 8]), top, "Basement Spine"),
    ...exterior("g", "st-ground", "wt-ext-37", top, "Ground"),
    "w-g-spine": wall("st-ground", "wt-int-205", line([3.6, 0], [3.6, 8]), top, "Ground Spine"),
    "w-g-cross": wall("st-ground", "wt-int-205", line([0, 5], [10, 5]), top, "Cross Wall"),
    "w-g-wc": wall("st-ground", "wt-int-145", line([1.8, 5], [1.8, 8]), top, "WC Partition"),
    ...exterior("u", "st-upper", "wt-ext-37", top, "Upper"),
    "w-u-spine": wall("st-upper", "wt-int-205", line([3.6, 0], [3.6, 8]), top, "Upper Spine"),
    "w-u-bedrooms": wall("st-upper", "wt-int-145", line([3.6, 4.5], [10, 4.5]), top, "Bedroom Partition"),
    "w-u-bath": wall("st-upper", "wt-int-145", line([0, 5], [1.8, 5]), top, "Bathroom South"),
    "w-u-bath-east": wall("st-upper", "wt-int-145", line([1.8, 5], [1.8, 8]), top, "Bathroom East"),
    "w-a-south": wall("st-attic", "wt-attic-34", line([0, 0], [10, 0]), knee, "Attic South"),
    "w-a-east": wall("st-attic", "wt-attic-34", line([10, 0], [10, 8]), knee, "Attic East"),
    "w-a-north": wall("st-attic", "wt-attic-34", line([10, 8], [0, 8]), knee, "Attic North"),
    "w-a-west": wall("st-attic", "wt-attic-34", line([0, 8], [0, 0]), knee, "Attic West"),
  };
  const bayWindows = (sid: string, storey: string) => [0.75, 1.74, 2.73].map((offset, i) => [`o-${sid}-bay-${i + 1}`, win(`w-${sid}-bay`, "wn-bay", offset, `${storey === "st-ground" ? "Living" : "Master"} Bay Window ${i + 1}`)] as const);
  const openings: Record<string, unknown> = Object.fromEntries([
    ["o-b-light-s1", win("w-b-south", "wn-cellar", 1.8, "Cellar Light South West")],
    ["o-b-light-s2", win("w-b-south", "wn-cellar", 7, "Cellar Light South East")],
    ["o-b-light-n1", win("w-b-north", "wn-cellar", 3, "Cellar Light North East")],
    ["o-b-light-n2", win("w-b-north", "wn-cellar", 8.2, "Cellar Light North West")],
    ["o-b-light-w", win("w-b-west", "wn-cellar", 3, "Cellar Light West")],
    ["o-b-door", door("w-b-spine", "dr-cellar", 2, "Cellar Door", { flipFacing: true })],
    ["o-g-entry", door("w-g-south", "dr-entry", 2.9, "Front Door")],
    ["o-g-living", win("w-g-south", "wn-picture", 5.9, "Living Room Window")],
    ["o-g-patio", door("w-g-south", "dr-patio", 8.4, "Patio Door", { flipHand: true })],
    ...bayWindows("g", "st-ground"),
    ["o-g-kitchen-east", win("w-g-east-2", "wn-casement", 2, "Kitchen East Window")],
    ["o-g-kitchen-north", win("w-g-north", "wn-casement", 3.2, "Kitchen North Window", { width: 1.8 })],
    ["o-g-cellar-north", win("w-g-north", "wn-wc", 7.3, "Cellar Stair Window")],
    ["o-g-wc-north", win("w-g-north", "wn-wc", 9.1, "WC Window")],
    ["o-g-hall-west", win("w-g-west", "wn-casement", 5, "Hall Window", { width: 1 })],
    ["o-g-living-door", door("w-g-spine", "dr-int-90", 1.6, "Living Room Door")],
    ["o-g-wc-door", door("w-g-cross", "dr-int-80", 0.9, "WC Door", { flipHand: true })],
    ["o-g-cellar-door", door("w-g-cross", "dr-int-90", 2.7, "Cellar Stair Door", { flipFacing: true })],
    ["o-g-kitchen-void", hole("w-g-cross", 6.8, 1.8, 2.1, "Kitchen Passage")],
    ["o-u-landing-south", win("w-u-south", "wn-casement", 1.5, "Landing South Window", { width: 1 })],
    ["o-u-master", win("w-u-south", "wn-picture", 6, "Master Bedroom Window")],
    ...bayWindows("u", "st-upper"),
    ["o-u-child-east", win("w-u-east-2", "wn-casement", 2, "Child Room East Window")],
    ["o-u-child-north-1", win("w-u-north", "wn-casement", 3.2, "Child Room North Window 1")],
    ["o-u-child-north-2", win("w-u-north", "wn-casement", 5.4, "Child Room North Window 2")],
    ["o-u-landing-north", win("w-u-north", "wn-casement", 7.4, "Landing North Window", { width: 0.9 })],
    ["o-u-bath-north", win("w-u-north", "wn-wc", 9.1, "Bathroom Window")],
    ["o-u-landing-west", win("w-u-west", "wn-casement", 5.4, "Landing West Window", { width: 1 })],
    ["o-u-master-door", door("w-u-spine", "dr-int-90", 1, "Master Bedroom Door")],
    ["o-u-child-door", door("w-u-spine", "dr-int-90", 6.5, "Child Room Door", { flipHand: true })],
    ["o-u-bath-door", door("w-u-bath", "dr-int-80", 0.75, "Bathroom Door")],
  ]);
  const outer = [V(0, 0), V(10, 0), V(10, 1, 0.5), V(10, 4), V(10, 8), V(0, 8)];
  const bayRoof = [V(10, 4), V(10, 2.5), V(10, 1, 0.5)];
  const model = F.snap({
    materials: materials(["m-concrete", "m-brick", "m-insulation", "m-plaster", "m-timber", "m-glass", "m-steel", "m-screed", "m-membrane", "m-tile"]),
    wall_types: {
      "wt-ext-37": stack("Exterior Wall", [layer("m-plaster", 0.015, "Finish"), layer("m-brick", 0.175), layer("m-insulation", 0.16, "Insulation"), layer("m-plaster", 0.02, "Finish")]),
      "wt-base-37": stack("Basement Wall", [layer("m-plaster", 0.02, "Finish"), layer("m-concrete", 0.25), layer("m-insulation", 0.1, "Insulation")]),
      "wt-base-25": stack("Basement Bearing Wall", [layer("m-concrete", 0.25)]),
      "wt-int-205": stack("Bearing Partition", [layer("m-plaster", 0.015, "Finish"), layer("m-brick", 0.175), layer("m-plaster", 0.015, "Finish")]),
      "wt-int-145": stack("Partition", [layer("m-plaster", 0.015, "Finish"), layer("m-brick", 0.115), layer("m-plaster", 0.015, "Finish")]),
      "wt-attic-34": stack("Attic Knee Wall", [layer("m-plaster", 0.016, "Finish"), layer("m-timber", 0.1), layer("m-insulation", 0.2, "Insulation"), layer("m-timber", 0.024, "Finish")]),
    },
    slab_types: {
      "st-found-49": stack("Foundation Slab", [layer("m-screed", 0.07, "Finish"), layer("m-insulation", 0.12, "Insulation"), layer("m-concrete", 0.3)]),
      "st-floor-30": stack("Floor Slab", [layer("m-screed", 0.07, "Finish"), layer("m-insulation", 0.03, "Insulation"), layer("m-concrete", 0.2)]),
      "st-timber-30": stack("Timber Floor", [layer("m-timber", 0.024, "Finish"), layer("m-insulation", 0.16, "Insulation"), layer("m-timber", 0.116)]),
    },
    roof_types: {
      "rt-pitched": stack("Pitched Roof", [layer("m-tile", 0.04, "Finish"), layer("m-timber", 0.05, "Substrate"), layer("m-membrane", 0.002, "Membrane"), layer("m-insulation", 0.24, "Insulation"), layer("m-plaster", 0.016, "Finish")]),
      "rt-flat": stack("Flat Roof", [layer("m-membrane", 0.01, "Membrane"), layer("m-insulation", 0.2, "Insulation"), layer("m-timber", 0.18)]),
    },
    column_types: { "ct-chs-168": { name: "Steel Tube 168", profile: { Circle: { diameter: 0.168 } }, material: "m-steel" } },
    beam_types: { "bt-ipe-240": { name: "IPE 240", profile: { IShape: { width: 0.12, depth: 0.24, web: 0.0062, flange: 0.0098 } }, material: "m-steel" } },
    window_types: {
      "wn-cellar": windowType("Cellar Window", 0.8, 0.5, 1.75, 2, "m-timber"),
      "wn-wc": windowType("Small Window", 0.6, 0.6, 1.3, 3, "m-timber"),
      "wn-casement": windowType("Casement Window", 1.2, 1.35, 0.9, 3, "m-timber"),
      "wn-picture": windowType("Picture Window", 1.8, 1.4, 0.75, 3, "m-timber"),
      "wn-bay": windowType("Bay Window", 0.9, 1.6, 0.6, 3, "m-timber"),
    },
    door_types: {
      "dr-entry": doorType("Front Door", 1, 2.15, "Single", "Right", "m-timber"),
      "dr-int-90": doorType("Interior Door 90", 0.9, 2.1, "Single", "Left", "m-timber"),
      "dr-int-80": doorType("Interior Door 80", 0.8, 2.1, "Single", "Left", "m-timber"),
      "dr-cellar": doorType("Cellar Door", 0.9, 2, "Single", "Left", "m-steel"),
      "dr-patio": doorType("Patio Door", 1.6, 2.25, "Double", "Right", "m-timber"),
    },
    sites: { "site-1": { name: "Plot Bern", latitude: 46.948, longitude: 7.4474, elevation: 542, true_north: 0.05, boundary: [P(0, 0), P(22.5, 0), P(24, 29), P(1.5, 30.5)] } },
    buildings: { "bldg-1": { site: "site-1", name: "Family House", origin: P(6, 9), rotation: 0, elevation: 0.3 } },
    storeys: {
      "st-basement": F.storey("bldg-1", "Basement", -1, 2.6),
      "st-ground": F.storey("bldg-1", "Ground Floor", 0, 2.8),
      "st-upper": F.storey("bldg-1", "Upper Floor", 1, 2.7),
      "st-attic": F.storey("bldg-1", "Attic", 2, 2.4, 0.5),
    },
    grids: {
      "g-a": grid("bldg-1", "A", [0, -1], [0, 9]), "g-b": grid("bldg-1", "B", [3.6, -1], [3.6, 9]), "g-c": grid("bldg-1", "C", [10, -1], [10, 9]),
      "g-1": grid("bldg-1", "1", [-1, 0], [11, 0]), "g-2": grid("bldg-1", "2", [-1, 5], [11, 5]), "g-3": grid("bldg-1", "3", [-1, 8], [11, 8]),
    },
    walls,
    columns: { "c-b-support": column("st-basement", "ct-chs-168", [6.8, 5], "Support Column", F.storeyTop(-0.54)) },
    beams: { "bm-b-support": beam("st-basement", "bt-ipe-240", [3.6, 5], [10, 5], -0.3, "Kitchen Wall Support") },
    slabs: {
      "sl-b": slab("st-basement", "st-found-49", outer, [], "Foundation Slab"),
      "sl-g": slab("st-ground", "st-floor-30", outer, [rect(2.25, 5.11, 3.15, 7.8)], "Ground Floor Slab"),
      "sl-u": slab("st-upper", "st-floor-30", outer, [rect(0.2, 1.7, 2.1, 4.5)], "Upper Floor Slab"),
      "sl-a": slab("st-attic", "st-timber-30", rect(0, 0, 10, 8), [], "Attic Floor"),
    },
    roofs: {
      "rf-main": roof("st-attic", "rt-pitched", rect(0, 0, 10, 8), { Gable: { pitch: 0.6109, ridge_direction: 0 } }, 0.5, 0.9, "Main Roof"),
      "rf-bay": roof("st-attic", "rt-flat", bayRoof, "Flat", 0.1, 0, "Bay Roof"),
    },
    openings,
    stairs: {
      "sr-main": stair("st-ground", [1.65, 1.7], Math.PI / 2, 0.9, { UTurn: { gap: 0.1 } }, 0.19, 0.26, "Main Stair", { stringer: { kind: "Closed", width: 0.04, depth: 0.26 }, nosing: 0.03 }),
      "sr-cellar": stair("st-basement", [2.7, 7.75], -Math.PI / 2, 0.9, "Straight", 0.2, 0.22, "Cellar Stair", { stringer: { kind: "Open", width: 0.05, depth: 0.2 }, nosing: 0.02, tread_thickness: 0.05, riser: "Open" }),
    },
    railings: {
      "rl-main": railing("st-upper", [[2.1, 1.7], [2.1, 4.5], [0.2, 4.5]], "m-steel", "Stair Opening Railing", { profile: rectProfile(0.05, 0.04), baluster: { profile: rectProfile(0.02, 0.02), spacing: 0.12 } }),
      "rl-cellar": railing("st-ground", [[2.25, 5.3], [2.25, 7.8], [3.15, 7.8], [3.15, 5.3]], "m-timber", "Cellar Stair Railing", { profile: rectProfile(0.06, 0.05), post_profile: rectProfile(0.07, 0.07), infill: { Panel: { thickness: 0.02 } } }),
    },
    spaces: {
      "sp-b1": space("st-basement", "B.01", "Cellar Stair", seed(1.8, 4), "Circulation"),
      "sp-b2": space("st-basement", "B.02", "Hobby and Storage", seed(7, 4), "Storage"),
      "sp-g1": space("st-ground", "0.01", "Hall", seed(3, 3), "Circulation"),
      "sp-g2": space("st-ground", "0.02", "WC", seed(0.9, 6.5), "Sanitary"),
      "sp-g3": space("st-ground", "0.03", "Cellar Stair Room", seed(2.7, 6.5), "Circulation"),
      "sp-g4": space("st-ground", "0.04", "Living Room", seed(7, 2.5), "Living"),
      "sp-g5": space("st-ground", "0.05", "Kitchen and Dining", seed(7, 6.5), "Kitchen"),
      "sp-u1": space("st-upper", "1.01", "Landing", seed(3, 3), "Circulation"),
      "sp-u2": space("st-upper", "1.02", "Bathroom", seed(0.9, 6.5), "Sanitary"),
      "sp-u3": space("st-upper", "1.03", "Master Bedroom", seed(7, 2), "Sleeping"),
      "sp-u4": space("st-upper", "1.04", "Child Room", seed(7, 6.5), "Sleeping"),
      "sp-a1": space("st-attic", "2.01", "Roof Space", seed(5, 4), "Storage"),
    },
    properties: {
      "w-g-south": { Pset_WallCommon: { IsExternal: flag(true), LoadBearing: flag(true), FireRating: text("REI 60"), ThermalTransmittance: real(0.18) } },
      "w-g-spine": { Pset_WallCommon: { IsExternal: flag(false), LoadBearing: flag(true), FireRating: text("REI 60") } },
      "o-g-entry": { Pset_DoorCommon: { SecurityRating: text("RC2"), IsExternal: flag(true) } },
      "o-g-living": { Pset_WindowCommon: { ThermalTransmittance: real(0.9), GlazingAreaFraction: real(0.78) } },
      "rf-main": { Pset_RoofCommon: { ThermalTransmittance: real(0.12), FireRating: text("RF1") } },
      "sp-g4": { Pset_SpaceCommon: { OccupancyType: text("Living"), Reference: text("0.04") }, Semio_Energy: { HeatedZone: flag(true), TargetTemperature: real(21) } },
    },
    classifications: {
      "w-g-south": classify("Uniclass 2015", "EF_25_10", "Walls"),
      "w-g-spine": classify("DIN 276", "341", "Tragende Innenwände"),
      "o-g-entry": classify("DIN 276", "334", "Außentüren und -fenster"),
      "o-g-living": classify("DIN 276", "334", "Außentüren und -fenster"),
      "rf-main": classify("DIN 276", "361", "Dachkonstruktionen"),
      "sl-g": classify("DIN 276", "351", "Deckenkonstruktionen"),
    },
  }, "Family House");
  Object.assign(model.project, { description: "Detached single-family house with basement, two storeys and a pitched attic roof.", author: "semio", organization: "semio", phase_names: ["Design", "Permit", "Construction"] });
  return model;
}
//#endregion 🏡️ house

//#region 🏢️ office
const COLUMN_LABELS = ["A", "B", "C", "D", "E", "F"];
const BAY = 6;

function office() {
  const levels = [0, 1, 2, 3];
  const storeyId = (level: number) => `st-${level}`;
  const top = F.storeyTop(0);
  const walls: Record<string, unknown> = {};
  const curtain: Record<string, unknown> = {};
  const columns: Record<string, unknown> = {};
  const beams: Record<string, unknown> = {};
  const openings: Record<string, unknown> = {};
  const stairs: Record<string, unknown> = {};
  const spaces: Record<string, unknown> = {};
  const slabs: Record<string, unknown> = {};
  const core = (level: number, side: "w" | "e", x0: number) => {
    const s = storeyId(level);
    const x1 = x0 + 3;
    const id = `w-${level}-core-${side}`;
    walls[`${id}-south`] = wall(s, "wt-core-25", line([x0, 6.4], [x1, 6.4]), top, `Core ${side.toUpperCase()} South`);
    walls[`${id}-east`] = wall(s, "wt-core-25", line([x1, 6.4], [x1, 11.6]), top, `Core ${side.toUpperCase()} East`);
    walls[`${id}-north`] = wall(s, "wt-core-25", line([x1, 11.6], [x0, 11.6]), top, `Core ${side.toUpperCase()} North`);
    walls[`${id}-west`] = wall(s, "wt-core-25", line([x0, 11.6], [x0, 6.4]), top, `Core ${side.toUpperCase()} West`);
    openings[`o-${level}-core-${side}-in`] = door(`${id}-south`, "dr-core", 1.5, `Core ${side.toUpperCase()} Entry`);
    openings[`o-${level}-core-${side}-out`] = door(`${id}-north`, "dr-core", 1.5, `Core ${side.toUpperCase()} Exit`, { flipHand: true });
    if (level < 3) stairs[`sr-${level}-${side}`] = stair(s, [x0 + 1.5, 6.65], Math.PI / 2, 1.2, "Straight", 0.2, 0.22, `Escape Stair ${side.toUpperCase()} ${level}`, { stringer: { kind: "Mono", width: 0.2, depth: 0.3 }, tread_thickness: 0.06 });
  };
  for (const level of levels) {
    const s = storeyId(level);
    for (const [i, y0] of [0, 6, 12].entries()) {
      walls[`w-${level}-west-${i + 1}`] = wall(s, "wt-infill-34", line([0, y0 + 5.75], [0, y0 + 0.25]), F.storeyTop(-0.86), `West Infill ${i + 1}`);
      walls[`w-${level}-east-${i + 1}`] = wall(s, "wt-infill-34", line([30, y0 + 0.25], [30, y0 + 5.75]), F.storeyTop(-0.86), `East Infill ${i + 1}`);
      openings[`o-${level}-west-${i + 1}`] = win(`w-${level}-west-${i + 1}`, "wn-ribbon", 2.75, `West Window ${level}.${i + 1}`);
      openings[`o-${level}-east-${i + 1}`] = win(`w-${level}-east-${i + 1}`, "wn-ribbon", 2.75, `East Window ${level}.${i + 1}`, { flipFacing: true });
    }
    curtain[`cu-${level}-south`] = curtainWall(s, line([0, -0.45], [30, -0.45]), `South Curtain Wall ${level}`);
    curtain[`cu-${level}-north`] = curtainWall(s, line([30, 18.45], [0, 18.45]), `North Curtain Wall ${level}`);
    core(level, "w", 7);
    core(level, "e", 20);
    const rows = [0, 1, 2, 3];
    for (const row of rows) for (const [col, label] of COLUMN_LABELS.entries()) {
      columns[`c-${level}-${label}${row + 1}`] = column(s, level < 2 ? "ct-rc-500" : "ct-rc-400", [col * BAY, row * BAY], `Column ${label}${row + 1}`);
      if (col < 5) beams[`bm-${level}-x-${label}${row + 1}`] = beam(s, "bt-rc-primary", [col * BAY, row * BAY], [(col + 1) * BAY, row * BAY], -0.36, `Beam ${label}${row + 1}-${COLUMN_LABELS[col + 1]}${row + 1}`);
      if (row < 3) beams[`bm-${level}-y-${label}${row + 1}`] = beam(s, "bt-rc-secondary", [col * BAY, row * BAY], [col * BAY, (row + 1) * BAY], -0.36, `Beam ${label}${row + 1}-${label}${row + 2}`);
    }
    const outer = rect(-0.3, -0.3, 30.3, 18.3);
    const shafts = level === 0 ? [] : [rect(7.15, 6.55, 9.85, 11.45), rect(20.15, 6.55, 22.85, 11.45)];
    slabs[`sl-${level}`] = slab(s, level === 0 ? "sl-ground" : "sl-floor", outer, shafts, level === 0 ? "Ground Slab" : `Floor Slab ${level}`);
    const n = (k: number) => `${level}.0${k}`;
    spaces[`sp-${level}-1`] = space(s, n(1), level === 0 ? "Lobby" : "Open Office South", outline(rect(0, 0, 30, 6)), level === 0 ? "Lobby" : "Office");
    spaces[`sp-${level}-2`] = space(s, n(2), "Open Office North", outline(rect(0, 12, 30, 18)), "Office");
    spaces[`sp-${level}-3`] = space(s, n(3), "Circulation West", outline(rect(0, 6, 7, 12)), "Circulation");
    spaces[`sp-${level}-4`] = space(s, n(4), "Circulation Centre", outline(rect(10, 6, 20, 12)), "Circulation");
    spaces[`sp-${level}-5`] = space(s, n(5), "Circulation East", outline(rect(23, 6, 30, 12)), "Circulation");
    spaces[`sp-${level}-6`] = space(s, n(6), "Stair Core West", seed(8.5, 9), "Escape Stair");
    spaces[`sp-${level}-7`] = space(s, n(7), "Stair Core East", seed(21.5, 9), "Escape Stair");
  }
  openings["o-0-entry"] = door("cu-0-south", "dr-glass", 15, "Main Entrance");
  const parapet = (id: string, a: Pt, b: Pt, name: string) => { walls[id] = wall("st-r", "wt-parapet-25", line(a, b), F.unconnected(1), name); };
  parapet("w-r-par-south", [-0.3, -0.3], [30.3, -0.3], "Parapet South");
  parapet("w-r-par-east", [30.3, -0.3], [30.3, 18.3], "Parapet East");
  parapet("w-r-par-north", [30.3, 18.3], [-0.3, 18.3], "Parapet North");
  parapet("w-r-par-west", [-0.3, 18.3], [-0.3, -0.3], "Parapet West");
  const model = F.snap({
    materials: materials(["m-concrete", "m-insulation", "m-plaster", "m-glass", "m-steel", "m-screed", "m-membrane", "m-gravel"]),
    wall_types: {
      "wt-infill-34": stack("Infill Wall", [layer("m-plaster", 0.015, "Finish"), layer("m-concrete", 0.15), layer("m-insulation", 0.16, "Insulation"), layer("m-plaster", 0.015, "Finish")]),
      "wt-core-25": stack("Core Wall", [layer("m-concrete", 0.25)]),
      "wt-parapet-25": stack("Parapet", [layer("m-concrete", 0.19), layer("m-insulation", 0.06, "Insulation")]),
    },
    slab_types: {
      "sl-ground": stack("Ground Slab", [layer("m-screed", 0.07, "Finish"), layer("m-insulation", 0.14, "Insulation"), layer("m-concrete", 0.35)]),
      "sl-floor": stack("Office Floor", [layer("m-screed", 0.08, "Finish"), layer("m-concrete", 0.28)]),
    },
    roof_types: { "rt-flat": stack("Flat Roof", [layer("m-gravel", 0.05, "Finish"), layer("m-membrane", 0.01, "Membrane"), layer("m-insulation", 0.24, "Insulation"), layer("m-concrete", 0.25)]) },
    column_types: {
      "ct-rc-500": { name: "RC Column 500", profile: rectProfile(0.5, 0.5), material: "m-concrete" },
      "ct-rc-400": { name: "RC Column 400", profile: rectProfile(0.4, 0.4), material: "m-concrete" },
    },
    beam_types: {
      "bt-rc-primary": { name: "RC Beam 400x600", profile: rectProfile(0.4, 0.6), material: "m-concrete" },
      "bt-rc-secondary": { name: "RC Beam 300x500", profile: rectProfile(0.3, 0.5), material: "m-concrete" },
    },
    window_types: { "wn-ribbon": windowType("Ribbon Window", 3, 1.6, 0.85, 3, "m-steel") },
    door_types: {
      "dr-glass": doorType("Glass Entrance Door", 1.8, 2.4, "Double", "Left", "m-steel"),
      "dr-core": doorType("Fire Door", 0.9, 2.1, "Single", "Left", "m-steel"),
    },
    sites: { "site-1": { name: "Plot Zurich", latitude: 47.391, longitude: 8.488, elevation: 408, true_north: 0.12, boundary: [P(0, 0), P(56, 0), P(56, 48), P(0, 48)] } },
    buildings: { "bldg-1": { site: "site-1", name: "Office Building", origin: P(12, 14), rotation: 0, elevation: 0 } },
    storeys: {
      "st-0": F.storey("bldg-1", "Ground Floor", 0, 4),
      "st-1": F.storey("bldg-1", "First Floor", 1, 3.8),
      "st-2": F.storey("bldg-1", "Second Floor", 2, 3.8),
      "st-3": F.storey("bldg-1", "Third Floor", 3, 3.8),
      "st-r": F.storey("bldg-1", "Roof", 4, 1.2),
    },
    grids: Object.fromEntries([
      ...COLUMN_LABELS.map((label, col) => [`g-${label}`, grid("bldg-1", label, [col * BAY, -2], [col * BAY, 20])]),
      ...[1, 2, 3, 4].map((row) => [`g-${row}`, grid("bldg-1", String(row), [-2, (row - 1) * BAY], [32, (row - 1) * BAY])]),
    ]),
    walls,
    curtain_walls: curtain,
    columns,
    beams,
    slabs,
    roofs: { "rf-main": roof("st-r", "rt-flat", rect(-0.3, -0.3, 30.3, 18.3), "Flat", 0, 0, "Flat Roof") },
    openings,
    stairs,
    spaces,
    properties: {
      "c-0-A1": { Pset_ColumnCommon: { LoadBearing: flag(true), FireRating: text("R 90") } },
      "cu-0-south": { Pset_CurtainWallCommon: { ThermalTransmittance: real(1.1), IsExternal: flag(true) } },
      "w-0-core-w-south": { Pset_WallCommon: { LoadBearing: flag(true), FireRating: text("REI 120"), IsExternal: flag(false) } },
      "sp-0-1": { Pset_SpaceCommon: { OccupancyType: text("Lobby"), Reference: text("0.01") } },
    },
    classifications: {
      "c-0-A1": classify("DIN 276", "333", "Außenstützen"),
      "c-1-B2": classify("DIN 276", "343", "Innenstützen"),
      "cu-0-south": classify("DIN 276", "337", "Elementierte Außenwände"),
      "w-0-core-w-south": classify("DIN 276", "341", "Tragende Innenwände"),
      "rf-main": classify("DIN 276", "360", "Dächer"),
    },
  }, "Office Building");
  Object.assign(model.project, { description: "Four-storey office building on a six metre structural grid with two escape stair cores and a curtain wall façade.", author: "semio", organization: "semio", phase_names: ["Design", "Permit", "Construction"] });
  return model;
}
//#endregion 🏢️ office

for (const [dir, name, build] of [[em(0x1f3e1) + "house", "house", house], [em(0x1f3e2) + "office", "office", office]] as const) {
  const model = build();
  verify(model);
  const assets = join(child(subset, "assets"), dir);
  mkdirSync(assets, { recursive: true });
  writeFileSync(join(assets, em(0x1f4f8) + "snapshot.json"), JSON.stringify(model, null, 2) + "\n");
  const counts = Object.fromEntries(Object.entries(model).filter(([, v]) => v && typeof v === "object" && !Array.isArray(v) && !("name" in (v as Json) && "description" in (v as Json))).map(([k, v]) => [k, Object.keys(v as Json).length]));
  console.log(`${name}: ${JSON.stringify(counts)}`);
}
