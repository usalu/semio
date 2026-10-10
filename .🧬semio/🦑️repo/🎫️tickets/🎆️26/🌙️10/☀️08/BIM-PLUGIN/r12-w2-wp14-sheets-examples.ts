/**
 * 📄️ WP-14 `w2-wp14-sheets`: the sheet set of the `house` and `office` examples, merged into the snapshot by `r4-x-examples-gen.ts` after the views. `sheetsFor(name, model)` makes what a person gets by placing the
 * views of a building on sheets: `A-101` carries the plans of the storeys, `A-201` the two sections and `A-301` the four elevations (schedule-free), every viewport cropped to the extent of its drawing at 1:100 so its
 * window is known from authored numbers alone. The paper is the smallest ISO A size, landscape, on which a shelf packing of the windows fits inside the frame clear of the title block and the revision table
 * (the constants mirror `sheet-layout`: 20 mm binding margin, 10 mm elsewhere, a title block 180 by 48 mm in the bottom right corner, a revision table of 6 mm rows above it). A viewport that would not fit refuses
 * the generator instead of writing a sheet with findings, so an example carries none.
 *
 * ISO 216 sizes: <https://en.wikipedia.org/wiki/ISO_216>; frame and title block: ISO 5457.
 */
import { extents } from "./r10-w12-views-examples.ts";

type Json = Record<string, any>;
type Rect = { x: number; y: number; width: number; height: number };

const PAPERS: [string, number, number][] = [["A3", 420, 297], ["A2", 594, 420], ["A1", 841, 594], ["A0", 1189, 841]];
const BINDING_MARGIN = 20;
const MARGIN = 10;
const TITLE_WIDTH = 180;
const TITLE_HEIGHT = 48;
const REVISION_ROW = 6;
const GAP = 10;
const DATE = "2026-10-09";

const meets = (a: Rect, b: Rect) => a.x < b.x + b.width && b.x < a.x + a.width && a.y < b.y + b.height && b.y < a.y + a.height;

/** 🪟️ The size of the window of a viewport cropped to `[x0, y0, x1, y1]` metres at `1:scale`. */
const windowOf = (crop: Json, scale: number): [number, number] => [Math.max(((crop.max.x - crop.min.x) * 1000) / scale, 10), Math.max(((crop.max.y - crop.min.y) * 1000) / scale, 10)];

/** 📦️ Shelf-packs `sizes` into a frame; `undefined` when a window would leave the frame or touch the reserved corner. */
function pack(sizes: [number, number][], frame: Rect, reserved: Rect): { x: number; y: number }[] | undefined {
  const places: { x: number; y: number }[] = [];
  let x = frame.x + GAP;
  let y = frame.y + GAP;
  let shelf = 0;
  for (const [width, height] of sizes) {
    if (x + width > frame.x + frame.width - GAP && shelf > 0) {
      x = frame.x + GAP;
      y += shelf + GAP;
      shelf = 0;
    }
    const window = { x, y, width, height };
    if (x + width > frame.x + frame.width - GAP || y + height > frame.y + frame.height - GAP || meets(window, { x: reserved.x - GAP, y: reserved.y - GAP, width: reserved.width + GAP, height: reserved.height + GAP })) return undefined;
    places.push({ x, y });
    x += width + GAP;
    shelf = Math.max(shelf, height);
  }
  return places;
}

/** 📄️ The sheet, its viewports and its revision rows for `views` (`[id, crop]` pairs in drawing order). */
function setOf(prefix: string, number: string, name: string, project: string, views: [string, Json][], revisions: Json[]) {
  const scale = 100;
  const sizes = views.map(([, crop]) => windowOf(crop, scale));
  for (const [paper, long, short] of PAPERS) {
    const frame = { x: BINDING_MARGIN, y: MARGIN, width: long - BINDING_MARGIN - MARGIN, height: short - 2 * MARGIN };
    const table = revisions.length === 0 ? 0 : (revisions.length + 1) * REVISION_ROW;
    const reserved = { x: frame.x + frame.width - TITLE_WIDTH, y: frame.y + frame.height - TITLE_HEIGHT - table, width: TITLE_WIDTH, height: TITLE_HEIGHT + table };
    const places = pack(sizes, frame, reserved);
    if (!places) continue;
    const sheet = { number, name, paper: { Iso: { size: paper } }, orientation: "Landscape", project, drawn_by: "semio", checked_by: "", date: DATE, revision: "", scale_label: "" };
    const viewports = Object.fromEntries(views.map(([view, crop], index) => [`vp-${prefix}-${view.replace(/^v-/u, "")}`, { sheet: `sh-${prefix}`, view, position: places[index], scale, crop }]));
    const rows = Object.fromEntries(revisions.map((row, index) => [`rev-${prefix}-${String.fromCharCode(97 + index)}`, { ...row, sheet: `sh-${prefix}` }]));
    return { id: `sh-${prefix}`, sheet, viewports, rows };
  }
  throw new Error(`${name}: the viewports do not fit on an A0 sheet`);
}

/** 📚️ The sheets, viewports and revision rows of a snapshot source whose views exist. */
export function sheetsFor(_name: string, model: Json): { sheets: Json; viewports: Json; sheet_revisions: Json } {
  const sheets: Json = {};
  const viewports: Json = {};
  const sheet_revisions: Json = {};
  const project = model.project?.name ?? "";
  for (const building of Object.keys(model.buildings ?? {})) {
    const single = Object.keys(model.buildings).length === 1;
    const prefix = single ? "" : `${building}-`;
    const rect = extents(model, building);
    if (!rect) continue;
    const storeys = Object.entries<Json>(model.storeys ?? {}).filter(([, row]) => row.building === building).sort((a, b) => a[1].level - b[1].level);
    const base: Record<string, number> = {};
    for (const [id, storey] of storeys) {
      base[id] = storey.level >= 0 ? storeys.filter(([, row]) => row.level >= 0 && row.level < storey.level).reduce((sum, [, row]) => sum + row.height, 0) : -storeys.filter(([, row]) => row.level < 0 && row.level >= storey.level).reduce((sum, [, row]) => sum + row.height, 0);
    }
    const zmin = Math.min(...storeys.map(([id]) => base[id])) - 0.5;
    const zmax = Math.max(...storeys.map(([id, row]) => base[id] + row.height)) + 4;
    const own = (kind: string) => Object.entries<Json>(model.views ?? {}).filter(([, view]) => view.building === building && view.kind === kind);
    const plans: [string, Json][] = own("Plan").map(([id]) => [id, { min: { x: rect[0], y: rect[1] }, max: { x: rect[2], y: rect[3] } }]);
    const vertical = (kind: string): [string, Json][] => own(kind).map(([id, view]) => [id, { min: { x: 0, y: zmin }, max: { x: Math.hypot(view.plane.end.x - view.plane.start.x, view.plane.end.y - view.plane.start.y), y: zmax } }]);
    const revisions = [
      { number: "A", date: "2026-10-01", description: "Issued for review", author: "semio" },
      { number: "B", date: "2026-10-05", description: "Sheets added", author: "semio" },
    ];
    const set = [
      setOf(`${prefix}plans`, `A-101${single ? "" : `-${building}`}`, "Floor plans", project, plans, revisions),
      setOf(`${prefix}sections`, `A-201${single ? "" : `-${building}`}`, "Sections", project, vertical("Section"), []),
      setOf(`${prefix}elevations`, `A-301${single ? "" : `-${building}`}`, "Elevations", project, vertical("Elevation"), []),
    ];
    for (const row of set) {
      sheets[row.id] = row.sheet;
      Object.assign(viewports, row.viewports);
      Object.assign(sheet_revisions, row.rows);
    }
  }
  return { sheets, viewports, sheet_revisions };
}
