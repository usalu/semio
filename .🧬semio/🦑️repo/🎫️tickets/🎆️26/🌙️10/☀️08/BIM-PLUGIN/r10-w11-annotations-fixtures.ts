#!/usr/bin/env bun
/**
 * 🪧️ Wave W1 `w11-annotations`: writes the committed inference fixture of `🪧️annotation-layout`: a one-storey room of four 0.3 m brick walls with a window, a grid line and a column,
 * annotated with dimensions (a wall length, a chain through a window centre, the clear width between two faces, a wall thickness between its two faces, a column to a grid line,
 * a kept lock, a broken lock and a dimension measured parallel to its faces), tags, a note and a leader. `🐍️.py write` of the case then writes the expected table.
 * `bun r10-w11-annotations-fixtures.ts` rewrites `🧫️fixtures/💡️inferences/🪧️annotation-layout/<case>/📸️snapshot/🔣️.json`.
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import * as F from "./r3-f1-fixtures.ts";
import { child, em, fixtures, JSONF } from "./r3-f1-paths.ts";

const P = F.P;
const wallEnd = (wall: string, end: "Start" | "End") => ({ WallEnd: { wall, end } });
const face = (wall: string, side: "Left" | "Right") => ({ WallFace: { wall, side } });
const free = (x: number, y: number) => ({ Point: { point: P(x, y) } });
const style = (name: string, textHeight = 0.25) => ({ name, text_height: textHeight, terminator: "Tick", unit: "Metre", precision: 2, mark_size: 0.15, gap: 0.1, overshoot: 0.2 });
const dimension = (anchors: unknown[], extra: Record<string, unknown> = {}) => ({ storey: "st-ground", anchors, angle: 0, offset: -1, style: "as-plan", name: "Dimension", ...extra });
const tag = (element: string, category: string, offset = P(0, -0.5)) => ({ storey: "st-ground", element, category, offset, style: "as-plan" });

const walls = {
  "w-north": F.wall("st-ground", "wt-300", F.line([8, 6], [0, 6]), F.storeyTop(0), "North"),
  "w-west": F.wall("st-ground", "wt-300", F.line([0, 6], [0, 0]), F.storeyTop(0), "West"),
};

const room = () => ({
  ...F.scene({ walls }),
  window_types: { "win-1": { name: "Window 1.2", width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.05, frame_depth: 0.1, panes: 1, material: "m-brick" } },
  openings: { "o-win": { ...F.opening("w-south", "South window"), offset: 2 } },
  grids: { "g-1": { building: "bldg-1", label: "1", start: P(0, 0), end: P(8, 0) } },
  column_types: { "ct-30": { name: "Column 30", profile: { Rectangle: { width: 0.3, depth: 0.3 } }, material: "m-brick" } },
  columns: { "col-1": { storey: "st-ground", column_type: "ct-30", position: P(4, 3), rotation: 0, base_offset: 0, top: F.storeyTop(0), phase: "New", name: "Column A" } },
  views: {
    "v-plan-st-ground": { building: "bldg-1", name: "Plan Ground", kind: "Plan", depth: 100, hidden: [], scale: 100, detail: "Medium", storey: "st-ground" },
    "v-plan-st-first": { building: "bldg-1", name: "Plan First", kind: "Plan", depth: 100, hidden: [], scale: 100, detail: "Medium", storey: "st-first" },
  },
  annotation_styles: { "as-plan": style("Plan 1:50") },
  dimensions: {
    "dim-south": dimension([wallEnd("w-south", "Start"), wallEnd("w-south", "End")], { name: "South length" }),
    "dim-chain": dimension([wallEnd("w-south", "Start"), { OpeningCentre: { opening: "o-win" } }, wallEnd("w-south", "End")], { offset: -2, name: "Window position" }),
    "dim-clear": dimension([face("w-south", "Left"), face("w-north", "Left")], { angle: Math.PI / 2, offset: 1, name: "Clear width" }),
    "dim-thickness": dimension([face("w-east", "Left"), face("w-east", "Right")], { offset: 1, name: "East thickness" }),
    "dim-column": dimension([{ ColumnCentre: { column: "col-1" } }, { Grid: { grid: "g-1" } }], { angle: Math.PI / 2, offset: -1, name: "Column to grid" }),
    "dim-locked": dimension([free(0, -2), free(3, -2)], { offset: 0, lock: 3, name: "Kept lock" }),
    "dim-lock-broken": dimension([free(0, -3), free(3, -3)], { offset: 0, lock: 2.5, name: "Broken lock" }),
    "dim-parallel": dimension([face("w-south", "Left"), face("w-north", "Left")], { name: "Parallel faces" }),
  },
  tags: {
    "tag-name": tag("w-south", "Name"),
    "tag-type": tag("w-south", "Type", P(0, -1)),
    "tag-size": tag("o-win", "Size", P(0, 0.8)),
    "tag-column": tag("col-1", "Size", P(0.5, 0.5)),
    "tag-empty": tag("w-south", "Number", P(0, 1)),
  },
  text_notes: { "note-1": { storey: "st-ground", position: P(2, 2), text: "Verify on site", rotation: 0, style: "as-plan" } },
  leaders: { "lead-1": { storey: "st-ground", anchor: face("w-south", "Right"), offset: P(1, -1), text: "Brick 300", style: "as-plan" } },
});

const inferences = child(fixtures, "inferences");
const dir = join(inferences, em(0x1faa7) + "annotation-layout", em(0x1f3e0) + "room", em(0x1f4f8) + "snapshot");
mkdirSync(dir, { recursive: true });
writeFileSync(join(dir, JSONF), JSON.stringify(room(), null, 2) + "\n");
console.log(`wrote ${dir}`);
