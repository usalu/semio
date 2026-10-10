#!/usr/bin/env bun
/**
 * 📄️ Writes the committed room with a sheet set: the view-linework room plus three sheets whose viewports are all cropped (or draw nothing), so the shapely oracle derives every window from
 * authored numbers alone. Run `bun r12-w2-wp14-fixtures.ts <room snapshot in> <sheet room snapshot out>`.
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";

const [input, output] = process.argv.slice(2);
const room = JSON.parse(readFileSync(input, "utf8"));
const iso = (size: string) => ({ Iso: { size } });
const sheet = (number: string, name: string, paper: unknown, orientation: string, extra: Record<string, string> = {}) => ({
  number, name, paper, orientation, project: "Room", drawn_by: "UG", checked_by: "AB", date: "2026-10-09", revision: "", scale_label: "", ...extra,
});
const crop = (x0: number, y0: number, x1: number, y1: number) => ({ min: { x: x0, y: y0 }, max: { x: x1, y: y1 } });
const viewport = (sheetId: string, view: string, x: number, y: number, scale: number, region?: ReturnType<typeof crop>, label?: string) => ({
  sheet: sheetId, view, position: { x, y }, scale, ...(region ? { crop: region } : {}), ...(label ? { label } : {}),
});

room.sheets = {
  "sh-plans": sheet("A-101", "Plan and elevation", iso("A3"), "Landscape"),
  "sh-notes": sheet("A-901", "Notes", iso("A4"), "Portrait", { scale_label: "As indicated" }),
  "sh-custom": sheet("A-902", "Custom", { Custom: { width: 350, height: 500 } }, "Landscape"),
};
room.viewports = {
  "vp-plan": viewport("sh-plans", "v-plan-ground", 30, 30, 100, crop(-1, -1, 7, 5), "Ground floor"),
  "vp-south": viewport("sh-plans", "v-south", 30, 110, 50, crop(-2, 0, 8, 3)),
  "vp-detail": viewport("sh-plans", "v-section-a", 240, 30, 25, crop(0, 0, 4, 3)),
  "vp-camera": viewport("sh-notes", "v-3d", 30, 30, 100),
  "vp-outside": viewport("sh-notes", "v-plan-ground", 150, 200, 50, crop(-1, -1, 7, 5)),
  "vp-custom": viewport("sh-custom", "v-section-a", 30, 30, 100, crop(0, 0, 10, 3)),
  "vp-custom-b": viewport("sh-custom", "v-section-a", 100, 40, 100, crop(0, 0, 2, 2)),
};
room.sheet_revisions = {
  "rev-a": { sheet: "sh-plans", number: "A", date: "2026-10-01", description: "Issued for permit", author: "UG" },
  "rev-b": { sheet: "sh-plans", number: "B", date: "2026-10-05", description: "Section added", author: "AB" },
};
mkdirSync(dirname(output), { recursive: true });
writeFileSync(output, `${JSON.stringify(room, null, 2)}\n`);
console.log(`wrote ${output}`);
