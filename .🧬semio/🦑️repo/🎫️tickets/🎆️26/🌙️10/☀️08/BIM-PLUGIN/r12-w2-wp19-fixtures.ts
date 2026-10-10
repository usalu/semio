#!/usr/bin/env bun
/**
 * 🏗️ Wave W2 `w2-wp19-frame`: adds the frame cases to the `🧊️element-solids` fixtures (keeps `expected` and `meshes` of every existing file: the third-party oracles rewrite `expected`, `BIM_BLESS=1 cargo test` the meshes).
 * `🏛️columns-profiles` gets two leaning columns (a rotated rectangle and an I shape on the upper storey); `➖️beams-profiles` gets the beams the WP adds: a beam joined to two columns, a beam whose end column leans out of
 * its end, inclined beams (alone and joined), arc beams (alone, joined and inclined); `🪟️curtain-overrides` is a new case: a curtain wall of a type with an explicit grid over its type, glass, open and opaque
 * panels and an override outside the grid. Run once: `bun r12-w2-wp19-fixtures.ts`.
 */
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const cases = join(root, "✏️s", "🔌️plugins", "🏙️bim", "🗿️artifacts", "🏢️model", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧫️fixtures", "💡️inferences", "🧊️element-solids");

type Json = Record<string, any>;
const P = (x: number, y: number) => ({ x, y });
const line = (a: [number, number], b: [number, number]) => ({ Line: { start: P(...a), end: P(...b) } });
const arc = (a: [number, number], b: [number, number], bulge: number) => ({ Arc: { start: P(...a), end: P(...b), bulge } });
const column = (type: string, at: [number, number], name: string, extra: Json = {}) => ({ storey: "st-ground", column_type: type, position: P(...at), rotation: 0, base_offset: 0, top: { StoreyTop: { offset: 0 } }, phase: "New", name, ...extra });
const beam = (type: string, axis: unknown, top_offset: number, name: string, extra: Json = {}) => ({ storey: "st-ground", beam_type: type, axis, top_offset, phase: "New", name, ...extra });

const read = (dir: string) => {
  const file = join(cases, dir, "🔣️.json");
  return { file, document: JSON.parse(readFileSync(file, "utf8")) as Json };
};
const write = (file: string, document: Json) => {
  const meshes = JSON.stringify(document.meshes ?? {});
  const head = JSON.stringify({ snapshot: document.snapshot, expected: document.expected ?? {} }, null, 2).slice(0, -2);
  rmSync(file, { force: true });
  writeFileSync(file, `${head},\n  "meshes": ${meshes}\n}\n`);
};

{
  const { file, document } = read("🏛️columns-profiles");
  Object.assign(document.snapshot.columns, {
    "c-lean": column("ct-rect", [9, 2], "Leaning rectangle", { rotation: 0.3, tilt: { direction: 0.7, angle: 0.25 } }),
    "c-lean-i": column("ct-i", [9, 5], "Leaning I", { storey: "st-first", base_offset: 0.2, top: { Unconnected: { height: 2 } }, tilt: { direction: -2, angle: 0.5 } }),
  });
  write(file, document);
}

{
  const { file, document } = read("➖️beams-profiles");
  const snapshot = document.snapshot;
  snapshot.column_types = { "ct-square": { name: "Square 400", profile: { Rectangle: { width: 0.4, depth: 0.4 } }, material: "m-concrete" } };
  snapshot.columns = {
    "c-j1": column("ct-square", [20, 0], "Joined west"),
    "c-j2": column("ct-square", [26, 0], "Joined east"),
    "c-j3": column("ct-square", [30, 0], "Rotated start", { rotation: 0.6 }),
    "c-j4": column("ct-square", [30, 6], "Leaning end", { tilt: { direction: Math.PI / 2, angle: 0.2 } }),
    "c-i1": column("ct-square", [20, 10], "Incline west"),
    "c-i2": column("ct-square", [26, 10], "Incline east"),
    "c-a1": column("ct-square", [20, 14], "Arc west"),
    "c-a2": column("ct-square", [26, 14], "Arc east"),
  };
  Object.assign(snapshot.beams, {
    "b-joined": beam("bt-rect", line([20, 0], [26, 0]), -0.1, "Joined at both ends"),
    "b-joined-lean": beam("bt-rect", line([30, 0], [30, 6]), -0.1, "Joined at the start, the end column leans away"),
    "b-incline": beam("bt-rect", line([0, 10], [6, 10]), 0, "Inclined", { end_top_offset: -0.8 }),
    "b-incline-joined": beam("bt-rect", line([20, 10], [26, 10]), -0.1, "Inclined and joined", { end_top_offset: -0.7 }),
    "b-arc": beam("bt-rect", arc([0, 14], [6, 14], 0.4), -0.1, "Arc"),
    "b-arc-joined": beam("bt-rect", arc([20, 14], [26, 14], 0.4), -0.1, "Arc joined at both ends"),
    "b-arc-incline": beam("bt-i", arc([0, 18], [6, 18], -0.3), 0, "Arc and inclined", { end_top_offset: -0.5 }),
  });
  write(file, document);
}

{
  const dir = join(cases, "🪟️curtain-overrides");
  mkdirSync(dir, { recursive: true });
  const base = JSON.parse(readFileSync(join(cases, "🏬️curtain-grid", "🔣️.json"), "utf8")).snapshot as Json;
  const snapshot: Json = { ...base, curtain_walls: {}, curtain_wall_types: {}, curtain_panel_overrides: {} };
  snapshot.curtain_wall_types = {
    "cwt-1": base.curtain_wall_types["cwt-1"],
    "cwt-wide": { name: "Curtain wall 2 x 1.5", u_grid: { Spacing: { spacing: 2 } }, v_grid: { Spacing: { spacing: 1.5 } }, interior_mullion: { Rectangle: { width: 0.05, depth: 0.1 } }, border_mullion: { Rectangle: { width: 0.08, depth: 0.14 } }, panel: "Glass", panel_material: "m-glass", mullion_material: "m-steel" },
  };
  const wall = (type: string, axis: unknown, name: string, extra: Json = {}) => ({ storey: "st-1", curtain_wall_type: type, axis, base_offset: 0, top: { Unconnected: { height: 3 } }, phase: "New", name, ...extra });
  snapshot.curtain_walls = {
    "cw-type": wall("cwt-1", line([0, 0], [6, 0]), "Follows its type"),
    "cw-lines": wall("cwt-wide", line([0, 5], [6, 5]), "Explicit lines over the type", { u_grid: { Lines: { positions: [1, 2.5, 4.75, 9] } }, v_grid: { Lines: { positions: [1] } } }),
    "cw-panels": wall("cwt-wide", line([0, 10], [6, 10]), "Panel overrides"),
    "cw-border": wall("cwt-wide", line([10, 10], [10, 12.5]), "Short wall"),
  };
  snapshot.curtain_panel_overrides = {
    "cpo-open": { curtain: "cw-panels", u: 1, v: 0, panel: "Empty" },
    "cpo-spandrel": { curtain: "cw-panels", u: 2, v: 1, panel: { Solid: { material: "m-wood" } } },
    "cpo-glass": { curtain: "cw-panels", u: 0, v: 1, panel: "Glass" },
    "cpo-stray": { curtain: "cw-panels", u: 7, v: 0, panel: "Empty" },
  };
  const file = join(dir, "🔣️.json");
  const previous = existsSync(file) ? (JSON.parse(readFileSync(file, "utf8")) as Json) : {};
  write(file, { snapshot, expected: previous.expected ?? {}, meshes: previous.meshes ?? {} });
}
{
  const dir = join(cases, "📐️frame-tilt-joins");
  mkdirSync(dir, { recursive: true });
  const columns = read("🏛️columns-profiles").document.snapshot;
  const beams = read("➖️beams-profiles").document.snapshot;
  const snapshot: Json = {
    schema: columns.schema,
    project: { ...columns.project, name: "Frame" },
    materials: columns.materials,
    sites: columns.sites,
    buildings: columns.buildings,
    storeys: columns.storeys,
    column_types: { ...columns.column_types, "ct-square": beams.column_types["ct-square"] },
    beam_types: beams.beam_types,
    columns: {
      "c-plumb": column("ct-rect", [0, 0], "Plumb", { rotation: 0.5 }),
      "c-lean": column("ct-rect", [4, 0], "Leaning rectangle", { rotation: 0.3, tilt: { direction: 0.7, angle: 0.25 } }),
      "c-lean-i": column("ct-i", [8, 0], "Leaning I", { storey: "st-first", base_offset: 0.2, top: { Unconnected: { height: 2 } }, tilt: { direction: -2, angle: 0.5 } }),
      "c-w": column("ct-square", [0, 6], "West"),
      "c-e": column("ct-square", [6, 6], "East"),
      "c-lean-end": column("ct-square", [12, 6], "Leaning away", { tilt: { direction: 0, angle: 0.2 } }),
    },
    beams: {
      "b-joined": beam("bt-rect", line([0, 6], [6, 6]), -0.1, "Joined at both ends"),
      "b-joined-lean": beam("bt-rect", line([6, 6], [12, 6]), -0.1, "Joined at the start, the end column leans away"),
      "b-incline": beam("bt-i", line([0, 10], [6, 10]), 0, "Inclined", { end_top_offset: -0.8 }),
      "b-incline-joined": beam("bt-rect", line([0, 14], [6, 14]), -0.1, "Inclined and joined", { end_top_offset: -0.7 }),
    },
  };
  snapshot.columns["c-ia"] = column("ct-square", [0, 14], "Incline west");
  snapshot.columns["c-ib"] = column("ct-square", [6, 14], "Incline east");
  const file = join(dir, "🔣️.json");
  const previous = existsSync(file) ? (JSON.parse(readFileSync(file, "utf8")) as Json) : {};
  write(file, { snapshot, expected: previous.expected ?? {}, meshes: previous.meshes ?? {} });
}
console.log("frame fixtures written");
