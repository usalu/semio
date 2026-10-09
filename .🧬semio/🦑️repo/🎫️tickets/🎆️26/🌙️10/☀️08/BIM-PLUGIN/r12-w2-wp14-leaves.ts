#!/usr/bin/env bun
/**
 * 📄️ Wave W2 (`w2-wp14-sheets`): the nine leaves of the sheet vocabulary of `s.bim.model@1` (binary tags 14000..14008): create, set and delete of a sheet, a viewport and a sheet revision, plus the
 * cascade case `cascades-its-viewports` of the existing `delete-view` leaf. `bun r12-w2-wp14-leaves.ts` rewrites their boilerplate, their hand-written `diff`/`inverse` and their fixtures (never a blessed
 * `after`/`diff`) and writes the mount text to `🗑️generated/w2-wp14-sheets/mounts.txt`. Bless with `BIM_BLESS=1 cargo test … <kind>`.
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitCase, emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { em, JSONF, mutations, RS } from "./r3-f1-paths.ts";

type Label = { en: string; de: string };
const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
const ref = (def: string) => ({ $ref: `${ART}#/$defs/${def}` });
const assigned = (inner: unknown) => ({ type: "object", additionalProperties: false, required: ["value"], properties: { value: { oneOf: [inner, { type: "null" }] } } });
const id = (role: "target" | "identity", label: Label, kind: string, order = 10): Prop => ({
  name: "id",
  rust: "String",
  schema: { type: "string" },
  ui: role === "target" ? { widget: "reference", role, label, ref: { kind }, group: "target", order } : { widget: "text", role: "identity", label, group: "identity", order },
});
const prop = (name: string, rust: string, schema: unknown, widget: string, label: Label, order: number, refKind?: string): Prop => ({
  name,
  rust,
  schema: schema as Record<string, unknown>,
  ui: { widget, role: "value", label, group: "value", order, ...(refKind ? { ref: { kind: refKind } } : {}) },
});

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const P = F.P;

//#region 🔖️Fixtures
const view = (over: Record<string, unknown>) => ({ building: "bldg-1", name: "View", kind: "Plan", depth: 100, hidden: [], scale: 100, detail: "Medium", ...over });
const plan = (name: string, storey: string) => view({ name, kind: "Plan", storey });
const section = (name: string) => view({ name, kind: "Section", plane: { start: P(0, 3), end: P(8, 3) } });
const camera = { target: P(4, 3), target_height: 1.5, azimuth: 0.8, pitch: 0.5, distance: 25 };
const orbit = (name: string) => view({ name, kind: "Orthographic", camera });
const crop = (x0: number, y0: number, x1: number, y1: number) => ({ min: P(x0, y0), max: P(x1, y1) });
const VIEWS = { "v-ground": plan("Ground plan", "st-ground"), "v-section": section("Section A"), "v-orbit": orbit("Iso") };

const sheetRow = (over: Record<string, unknown> = {}) => ({ number: "A-101", name: "Ground floor", paper: { Iso: { size: "A3" } }, orientation: "Landscape", project: "", drawn_by: "", checked_by: "", date: "", revision: "", scale_label: "", ...over });
const viewportRow = (over: Record<string, unknown> = {}) => ({ sheet: "sh-1", view: "v-ground", position: P(20, 20), scale: 100, ...over });
const revisionRow = (over: Record<string, unknown> = {}) => ({ sheet: "sh-1", number: "A", date: "2026-10-01", description: "Issued for permit", author: "UG", ...over });

const scene = (extra: Record<string, unknown> = {}) => ({ ...F.scene(), views: VIEWS, ...extra });
const base = () => scene({ sheets: { "sh-1": sheetRow() } });
const rich = () =>
  scene({
    sheets: { "sh-1": sheetRow(), "sh-2": sheetRow({ number: "A-102", name: "Sections", orientation: "Portrait" }) },
    viewports: { "vp-1": viewportRow(), "vp-2": viewportRow({ view: "v-section", position: P(220, 20), scale: 50, crop: crop(0, 0, 6, 4), label: "Section through the hall" }), "vp-3": viewportRow({ sheet: "sh-2", view: "v-section", position: P(10, 10) }) },
    sheet_revisions: { "rev-1": revisionRow(), "rev-2": revisionRow({ number: "B", date: "2026-10-05", description: "Window sizes", author: "AB" }), "rev-3": revisionRow({ sheet: "sh-2", number: "A" }) },
  });
const withData = () => {
  const snapshot: any = rich();
  snapshot.properties = { "sh-1": { Pset_Sheet: { Reviewer: { Text: { value: "UG" } } } } };
  snapshot.classifications = { "sh-1": { system: "DIN 276", code: "710", title: "Planung" } };
  return snapshot;
};
const viewlessSheet = () => scene({ sheets: { "sh-1": sheetRow() }, viewports: { "vp-1": viewportRow() } });
//#endregion 🔖️Fixtures

const MUTATION_USES: Record<string, string> = {
  "set-sheet": "use crate::{Orientation, Paper, SheetPatch};",
  "set-viewport": "use crate::{Assigned, Point2, ViewCrop, ViewportPatch};",
  "set-sheet-revision": "use crate::SheetRevisionPatch;",
};

const emoji = ["2705", "1f9f2", "1f9f5", "1f9f4", "1f9f3", "1f9f1", "1f9f0", "1f9ef", "1f9ee", "1f9ed", "1f9e9", "1f9e8", "1f9e7", "1f9e6", "1f9e5", "1f9e4", "1f9e3", "1f9e2", "1f9e1", "1f9e0", "1f9df", "1f9de", "1f9dd", "1f9dc", "1f9db", "1f9da", "1f9d9", "1f9d8", "1f9d7", "1f9d6"].map((hex) => parseInt(hex, 16));
const cases = <T extends { name: string; emoji?: number }>(rows: T[]) => rows.map((row, index) => ({ ...row, emoji: emoji[index] }));
const target = "vec![self.id.clone()]";

const sheetFields = (verb: "create" | "set") => [
  prop("number", "Option<String>", { type: "string" }, "text", { en: "Sheet number", de: "Plannummer" }, 20),
  prop("name", "Option<String>", { type: "string" }, "text", { en: "Name", de: "Name" }, 30),
  prop("paper", "Option<Paper>", ref("Paper"), "record", { en: "Paper", de: "Papier" }, 40),
  prop("orientation", "Option<Orientation>", ref("Orientation"), "select", { en: "Orientation", de: "Ausrichtung" }, 50),
  prop("project", "Option<String>", { type: "string" }, "text", { en: "Project line", de: "Projektzeile" }, 60),
  prop("drawn_by", "Option<String>", { type: "string" }, "text", { en: "Drawn by", de: "Gezeichnet" }, 70),
  prop("checked_by", "Option<String>", { type: "string" }, "text", { en: "Checked by", de: "Geprüft" }, 80),
  prop("date", "Option<String>", { type: "string" }, "text", { en: "Date (year-month-day)", de: "Datum (Jahr-Monat-Tag)" }, 90),
  prop("revision", "Option<String>", { type: "string" }, "text", { en: "Revision mark", de: "Änderungsstand" }, 100),
  prop("scale_label", "Option<String>", { type: "string" }, "text", { en: "Scale line", de: "Massstabszeile" }, 110),
];

export const leaves: Leaf[] = [
  {
    kind: "create-sheet", emoji: 0x1f4c4, variant: "CreateSheet", verb: "create", entity: "sheet", binaryTag: 14000, displayName: "Create Sheet",
    doc: "Brings a new sheet into the drawing set: number, name, paper size, orientation and the authored fields of the title block. The frame, the title block and the revision table it prints are inferred.",
    props: [id("identity", { en: "Sheet id", de: "Blatt-Id" }, "sheet"), prop("sheet", "Sheet", ref("Sheet"), "record", { en: "Sheet", de: "Blatt" }, 20)],
    label: { en: 'format!("Create sheet \\"{}\\"", self.sheet.number)', de: 'format!("Blatt \\"{}\\" anlegen", self.sheet.number)' },
    target,
    cases: cases([
      { name: "adds-a-sheet", before: scene(), mutation: { id: "sh-1", sheet: sheetRow() }, outcome: ok },
      { name: "adds-a-portrait-a1", before: base(), mutation: { id: "sh-2", sheet: sheetRow({ number: "A-102", name: "Elevations", paper: { Iso: { size: "A1" } }, orientation: "Portrait" }) }, outcome: ok },
      { name: "adds-a-custom-paper", before: base(), mutation: { id: "sh-2", sheet: sheetRow({ number: "A-103", name: "Long section", paper: { Custom: { width: 1500, height: 420 } } }) }, outcome: ok },
      { name: "adds-a-filled-title-block", before: base(), mutation: { id: "sh-2", sheet: sheetRow({ number: "A-104", name: "Details", project: "House on the hill", drawn_by: "UG", checked_by: "AB", date: "2026-10-09", revision: "B", scale_label: "As indicated" }) }, outcome: ok },
      { name: "duplicate-id", before: base(), mutation: { id: "sh-1", sheet: sheetRow({ number: "A-102" }) }, outcome: reject("mutation.duplicate-id", ["sh-1"]) },
      { name: "id-taken-by-another-kind", before: base(), mutation: { id: "w-south", sheet: sheetRow({ number: "A-102" }) }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "number-taken", before: base(), mutation: { id: "sh-2", sheet: sheetRow() }, outcome: reject("mutation.invariant", ["sheet", "number"]) },
      { name: "blank-number", before: base(), mutation: { id: "sh-2", sheet: sheetRow({ number: " " }) }, outcome: reject("mutation.invariant", ["sheet", "number"]) },
      { name: "blank-name", before: base(), mutation: { id: "sh-2", sheet: sheetRow({ number: "A-102", name: "" }) }, outcome: reject("mutation.invariant", ["sheet", "name"]) },
      { name: "custom-paper-too-small", before: base(), mutation: { id: "sh-2", sheet: sheetRow({ number: "A-102", paper: { Custom: { width: 20, height: 400 } } }) }, outcome: reject("mutation.invariant", ["sheet", "paper"]) },
      { name: "custom-paper-too-large", before: base(), mutation: { id: "sh-2", sheet: sheetRow({ number: "A-102", paper: { Custom: { width: 400, height: 12000 } } }) }, outcome: reject("mutation.invariant", ["sheet", "paper"]) },
      { name: "date-unreadable", before: base(), mutation: { id: "sh-2", sheet: sheetRow({ number: "A-102", date: "9 October" }) }, outcome: reject("mutation.invariant", ["sheet", "date"]) },
    ]),
  },
  {
    kind: "set-sheet", emoji: 0x1f4d1, variant: "SetSheet", verb: "set", entity: "sheet", binaryTag: 14001, displayName: "Set Sheet",
    doc: "Sparsely changes a sheet: number, name, paper, orientation and the authored fields of the title block. The title block, the frame and the revision table are inferred and follow.",
    props: [id("target", { en: "Sheet", de: "Blatt" }, "sheet"), ...sheetFields("set")],
    uses: [MUTATION_USES["set-sheet"]],
    label: { en: 'format!("Change sheet \\"{}\\"", self.id)', de: 'format!("Blatt \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "renames", before: rich(), mutation: { id: "sh-1", name: "Ground floor plan" }, outcome: ok },
      { name: "renumbers", before: rich(), mutation: { id: "sh-1", number: "A-110" }, outcome: ok },
      { name: "resizes-the-paper", before: rich(), mutation: { id: "sh-1", paper: { Iso: { size: "A1" } } }, outcome: ok },
      { name: "turns-the-sheet", before: rich(), mutation: { id: "sh-1", orientation: "Portrait" }, outcome: ok },
      { name: "cuts-a-custom-paper", before: rich(), mutation: { id: "sh-1", paper: { Custom: { width: 600, height: 400 } } }, outcome: ok },
      { name: "fills-the-title-block", before: rich(), mutation: { id: "sh-1", project: "House on the hill", drawn_by: "UG", checked_by: "AB", date: "2026-10-09" }, outcome: ok },
      { name: "marks-the-revision-and-scale", before: rich(), mutation: { id: "sh-1", revision: "B", scale_label: "1:100 / 1:50" }, outcome: ok },
      { name: "clears-the-title-block", before: scene({ sheets: { "sh-1": sheetRow({ drawn_by: "UG", date: "2026-10-09" }) } }), mutation: { id: "sh-1", drawn_by: "", date: "" }, outcome: ok },
      { name: "restates-an-unchanged-field", before: rich(), mutation: { id: "sh-1", name: "Ground floor", drawn_by: "UG" }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "sh-9", name: "Gone" }, outcome: reject("mutation.target-missing", ["sh-9"]) },
      { name: "unchanged", before: rich(), mutation: { id: "sh-1", name: "Ground floor", orientation: "Landscape" }, outcome: reject("mutation.no-op", ["sh-1"]) },
      { name: "empty-patch", before: rich(), mutation: { id: "sh-1" }, outcome: reject("mutation.no-op", ["sh-1"]) },
      { name: "number-taken", before: rich(), mutation: { id: "sh-1", number: "A-102" }, outcome: reject("mutation.invariant", ["number"]) },
      { name: "blank-number", before: rich(), mutation: { id: "sh-1", number: "" }, outcome: reject("mutation.invariant", ["number"]) },
      { name: "blank-name", before: rich(), mutation: { id: "sh-1", name: "  " }, outcome: reject("mutation.invariant", ["name"]) },
      { name: "custom-paper-too-small", before: rich(), mutation: { id: "sh-1", paper: { Custom: { width: 400, height: 10 } } }, outcome: reject("mutation.invariant", ["paper"]) },
      { name: "date-unreadable", before: rich(), mutation: { id: "sh-1", date: "2026-14-01" }, outcome: reject("mutation.invariant", ["date"]) },
    ]),
  },
  {
    kind: "delete-sheet", emoji: 0x1f4d2, variant: "DeleteSheet", verb: "delete", entity: "sheet", binaryTag: 14002, displayName: "Delete Sheet",
    doc: "Removes a sheet together with its viewports, its revision rows, its properties and its classifications. The views it showed are not touched.",
    props: [id("target", { en: "Sheet", de: "Blatt" }, "sheet")],
    inverseRows: { bounded: 4096 },
    label: { en: 'format!("Delete sheet \\"{}\\"", self.id)', de: 'format!("Blatt \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "cascades-viewports-and-revisions", before: rich(), mutation: { id: "sh-1" }, outcome: ok },
      { name: "removes-its-data", before: withData(), mutation: { id: "sh-1" }, outcome: ok },
      { name: "removes-an-empty-sheet", before: base(), mutation: { id: "sh-1" }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "sh-9" }, outcome: reject("mutation.target-missing", ["sh-9"]) },
    ]),
  },
  {
    kind: "create-viewport", emoji: 0x1f4d3, variant: "CreateViewport", verb: "create", entity: "viewport", binaryTag: 14003, displayName: "Create Viewport",
    doc: "Places a view on a sheet: a plan, ceiling plan, section or elevation at a scale from 1:1 to 1:1000, with the position of its window on the paper, an optional crop of the drawing and an optional label. The drawing is the inferred linework of the view.",
    props: [id("identity", { en: "Viewport id", de: "Ansichtsfenster-Id" }, "viewport"), prop("viewport", "Viewport", ref("Viewport"), "record", { en: "Viewport", de: "Ansichtsfenster" }, 20)],
    label: { en: 'format!("Place view \\"{}\\" on sheet \\"{}\\"", self.viewport.view, self.viewport.sheet)', de: 'format!("Ansicht \\"{}\\" auf Blatt \\"{}\\" platzieren", self.viewport.view, self.viewport.sheet)' },
    target,
    cases: cases([
      { name: "places-a-plan", before: base(), mutation: { id: "vp-1", viewport: viewportRow() }, outcome: ok },
      { name: "places-a-cropped-section", before: base(), mutation: { id: "vp-1", viewport: viewportRow({ view: "v-section", position: P(220, 20), scale: 50, crop: crop(0, 0, 6, 4), label: "Section through the hall" }) }, outcome: ok },
      { name: "places-the-same-view-twice", before: viewlessSheet(), mutation: { id: "vp-2", viewport: viewportRow({ position: P(220, 120), scale: 20, crop: crop(0, 0, 3, 3) }) }, outcome: ok },
      { name: "places-at-the-largest-scale", before: base(), mutation: { id: "vp-1", viewport: viewportRow({ scale: 1000 }) }, outcome: ok },
      { name: "duplicate-id", before: viewlessSheet(), mutation: { id: "vp-1", viewport: viewportRow({ position: P(5, 5) }) }, outcome: reject("mutation.duplicate-id", ["vp-1"]) },
      { name: "id-taken-by-another-kind", before: base(), mutation: { id: "w-south", viewport: viewportRow() }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "sheet-missing", before: base(), mutation: { id: "vp-1", viewport: viewportRow({ sheet: "sh-9" }) }, outcome: reject("mutation.target-missing", ["viewport", "sheet"]) },
      { name: "view-missing", before: base(), mutation: { id: "vp-1", viewport: viewportRow({ view: "v-9" }) }, outcome: reject("mutation.target-missing", ["viewport", "view"]) },
      { name: "camera-view", before: base(), mutation: { id: "vp-1", viewport: viewportRow({ view: "v-orbit" }) }, outcome: reject("mutation.invariant", ["viewport", "view"]) },
      { name: "scale-zero", before: base(), mutation: { id: "vp-1", viewport: viewportRow({ scale: 0 }) }, outcome: reject("mutation.invariant", ["viewport", "scale"]) },
      { name: "scale-too-large", before: base(), mutation: { id: "vp-1", viewport: viewportRow({ scale: 1001 }) }, outcome: reject("mutation.invariant", ["viewport", "scale"]) },
      { name: "empty-crop", before: base(), mutation: { id: "vp-1", viewport: viewportRow({ crop: crop(2, 0, 1, 4) }) }, outcome: reject("mutation.invariant", ["viewport", "crop"]) },
      { name: "blank-label", before: base(), mutation: { id: "vp-1", viewport: viewportRow({ label: " " }) }, outcome: reject("mutation.invariant", ["viewport", "label"]) },
    ]),
  },
  {
    kind: "set-viewport", emoji: 0x1f4d4, variant: "SetViewport", verb: "set", entity: "viewport", binaryTag: 14004, displayName: "Set Viewport",
    doc: "Sparsely changes a viewport: its sheet, its view, the position of its window on the paper, its scale (an assigned null never clears it), its crop (an assigned null clears it) and its label (an assigned null returns to the name of the view).",
    props: [
      id("target", { en: "Viewport", de: "Ansichtsfenster" }, "viewport"),
      prop("sheet", "Option<String>", { type: "string" }, "reference", { en: "Sheet", de: "Blatt" }, 20, "sheet"),
      prop("view", "Option<String>", { type: "string" }, "reference", { en: "View", de: "Ansicht" }, 30, "view"),
      prop("position", "Option<Point2>", ref("Point2"), "record", { en: "Position (mm)", de: "Position (mm)" }, 40),
      prop("scale", "Option<u32>", { type: "integer", minimum: 1, maximum: 1000 }, "integer", { en: "Scale 1:n", de: "Massstab 1:n" }, 50),
      prop("crop", "Option<Assigned<Option<ViewCrop>>>", assigned(ref("ViewCrop")), "record", { en: "Crop (empty = none)", de: "Zuschnitt (leer = keiner)" }, 60),
      prop("label", "Option<Assigned<Option<String>>>", assigned({ type: "string" }), "text", { en: "Label (empty = view name)", de: "Beschriftung (leer = Ansichtsname)" }, 70),
    ],
    uses: [MUTATION_USES["set-viewport"]],
    label: { en: 'format!("Change viewport \\"{}\\"", self.id)', de: 'format!("Ansichtsfenster \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "moves", before: rich(), mutation: { id: "vp-1", position: P(30, 40) }, outcome: ok },
      { name: "rescales", before: rich(), mutation: { id: "vp-1", scale: 200 }, outcome: ok },
      { name: "shows-another-view", before: rich(), mutation: { id: "vp-1", view: "v-section" }, outcome: ok },
      { name: "moves-to-another-sheet", before: rich(), mutation: { id: "vp-1", sheet: "sh-2", position: P(15, 150) }, outcome: ok },
      { name: "crops", before: rich(), mutation: { id: "vp-1", crop: { value: crop(1, 1, 5, 4) } }, outcome: ok },
      { name: "clears-the-crop", before: rich(), mutation: { id: "vp-2", crop: { value: null } }, outcome: ok },
      { name: "labels", before: rich(), mutation: { id: "vp-1", label: { value: "Entrance floor" } }, outcome: ok },
      { name: "clears-the-label", before: rich(), mutation: { id: "vp-2", label: { value: null } }, outcome: ok },
      { name: "restates-an-unchanged-field", before: rich(), mutation: { id: "vp-1", scale: 100, position: P(60, 20) }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "vp-9", scale: 50 }, outcome: reject("mutation.target-missing", ["vp-9"]) },
      { name: "unchanged", before: rich(), mutation: { id: "vp-1", scale: 100, view: "v-ground" }, outcome: reject("mutation.no-op", ["vp-1"]) },
      { name: "empty-patch", before: rich(), mutation: { id: "vp-1" }, outcome: reject("mutation.no-op", ["vp-1"]) },
      { name: "sheet-missing", before: rich(), mutation: { id: "vp-1", sheet: "sh-9" }, outcome: reject("mutation.target-missing", ["sheet"]) },
      { name: "view-missing", before: rich(), mutation: { id: "vp-1", view: "v-9" }, outcome: reject("mutation.target-missing", ["view"]) },
      { name: "camera-view", before: rich(), mutation: { id: "vp-1", view: "v-orbit" }, outcome: reject("mutation.invariant", ["view"]) },
      { name: "scale-zero", before: rich(), mutation: { id: "vp-1", scale: 0 }, outcome: reject("mutation.invariant", ["scale"]) },
      { name: "scale-too-large", before: rich(), mutation: { id: "vp-1", scale: 1001 }, outcome: reject("mutation.invariant", ["scale"]) },
      { name: "empty-crop", before: rich(), mutation: { id: "vp-1", crop: { value: crop(1, 1, 1, 4) } }, outcome: reject("mutation.invariant", ["crop"]) },
      { name: "blank-label", before: rich(), mutation: { id: "vp-1", label: { value: "  " } }, outcome: reject("mutation.invariant", ["label"]) },
    ]),
  },
  {
    kind: "delete-viewport", emoji: 0x1f4d5, variant: "DeleteViewport", verb: "delete", entity: "viewport", binaryTag: 14005, displayName: "Delete Viewport",
    doc: "Removes a viewport together with its properties and classifications. The sheet and the view stay.",
    props: [id("target", { en: "Viewport", de: "Ansichtsfenster" }, "viewport")],
    inverseRows: { bounded: 4096 },
    label: { en: 'format!("Delete viewport \\"{}\\"", self.id)', de: 'format!("Ansichtsfenster \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "removes", before: rich(), mutation: { id: "vp-2" }, outcome: ok },
      { name: "removes-its-data", before: (() => { const snapshot: any = rich(); snapshot.properties = { "vp-2": { Pset_Viewport: { Purpose: { Text: { value: "Permit" } } } } }; return snapshot; })(), mutation: { id: "vp-2" }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "vp-9" }, outcome: reject("mutation.target-missing", ["vp-9"]) },
    ]),
  },
  {
    kind: "create-sheet-revision", emoji: 0x1f4d6, variant: "CreateSheetRevision", verb: "create", entity: "sheet-revision", binaryTag: 14006, displayName: "Create Sheet Revision",
    doc: "Adds a row to the revision table of a sheet: the mark, its date, what changed and who changed it. The table is printed in the corner of the sheet by the inferred sheet layout.",
    props: [id("identity", { en: "Revision id", de: "Änderungs-Id" }, "sheet-revision"), prop("sheet_revision", "SheetRevision", ref("SheetRevision"), "record", { en: "Revision", de: "Änderung" }, 20)],
    label: { en: 'format!("Add revision \\"{}\\" to sheet \\"{}\\"", self.sheet_revision.number, self.sheet_revision.sheet)', de: 'format!("Änderung \\"{}\\" zu Blatt \\"{}\\" hinzufügen", self.sheet_revision.number, self.sheet_revision.sheet)' },
    target,
    cases: cases([
      { name: "adds-a-revision", before: base(), mutation: { id: "rev-1", sheet_revision: revisionRow() }, outcome: ok },
      { name: "adds-the-next-mark", before: rich(), mutation: { id: "rev-9", sheet_revision: revisionRow({ number: "C", date: "2026-10-09", description: "Stair rotated" }) }, outcome: ok },
      { name: "adds-an-undated-revision", before: base(), mutation: { id: "rev-1", sheet_revision: revisionRow({ date: "", author: "" }) }, outcome: ok },
      { name: "duplicate-id", before: rich(), mutation: { id: "rev-1", sheet_revision: revisionRow({ number: "C" }) }, outcome: reject("mutation.duplicate-id", ["rev-1"]) },
      { name: "id-taken-by-another-kind", before: base(), mutation: { id: "w-south", sheet_revision: revisionRow() }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "sheet-missing", before: base(), mutation: { id: "rev-1", sheet_revision: revisionRow({ sheet: "sh-9" }) }, outcome: reject("mutation.target-missing", ["sheet_revision", "sheet"]) },
      { name: "mark-taken", before: rich(), mutation: { id: "rev-9", sheet_revision: revisionRow() }, outcome: reject("mutation.invariant", ["sheet_revision", "number"]) },
      { name: "blank-mark", before: base(), mutation: { id: "rev-1", sheet_revision: revisionRow({ number: "" }) }, outcome: reject("mutation.invariant", ["sheet_revision", "number"]) },
      { name: "date-unreadable", before: base(), mutation: { id: "rev-1", sheet_revision: revisionRow({ date: "last week" }) }, outcome: reject("mutation.invariant", ["sheet_revision", "date"]) },
      { name: "blank-description", before: base(), mutation: { id: "rev-1", sheet_revision: revisionRow({ description: " " }) }, outcome: reject("mutation.invariant", ["sheet_revision", "description"]) },
    ]),
  },
  {
    kind: "set-sheet-revision", emoji: 0x1f4d7, variant: "SetSheetRevision", verb: "set", entity: "sheet-revision", binaryTag: 14007, displayName: "Set Sheet Revision",
    doc: "Sparsely changes a revision row: the mark, the date, the description and the author. The sheet of a row never changes.",
    props: [
      id("target", { en: "Revision", de: "Änderung" }, "sheet-revision"),
      prop("number", "Option<String>", { type: "string" }, "text", { en: "Mark", de: "Index" }, 20),
      prop("date", "Option<String>", { type: "string" }, "text", { en: "Date (year-month-day)", de: "Datum (Jahr-Monat-Tag)" }, 30),
      prop("description", "Option<String>", { type: "string" }, "text", { en: "Description", de: "Beschreibung" }, 40),
      prop("author", "Option<String>", { type: "string" }, "text", { en: "Author", de: "Bearbeiter" }, 50),
    ],
    uses: [MUTATION_USES["set-sheet-revision"]],
    label: { en: 'format!("Change revision \\"{}\\"", self.id)', de: 'format!("Änderung \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "redates", before: rich(), mutation: { id: "rev-1", date: "2026-10-02" }, outcome: ok },
      { name: "describes", before: rich(), mutation: { id: "rev-2", description: "Window sizes and sills" }, outcome: ok },
      { name: "re-marks", before: rich(), mutation: { id: "rev-2", number: "C" }, outcome: ok },
      { name: "signs", before: rich(), mutation: { id: "rev-1", author: "MK" }, outcome: ok },
      { name: "restates-an-unchanged-field", before: rich(), mutation: { id: "rev-1", number: "A", author: "MK" }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "rev-9", author: "MK" }, outcome: reject("mutation.target-missing", ["rev-9"]) },
      { name: "unchanged", before: rich(), mutation: { id: "rev-1", number: "A", date: "2026-10-01" }, outcome: reject("mutation.no-op", ["rev-1"]) },
      { name: "empty-patch", before: rich(), mutation: { id: "rev-1" }, outcome: reject("mutation.no-op", ["rev-1"]) },
      { name: "mark-taken", before: rich(), mutation: { id: "rev-1", number: "B" }, outcome: reject("mutation.invariant", ["number"]) },
      { name: "blank-mark", before: rich(), mutation: { id: "rev-1", number: " " }, outcome: reject("mutation.invariant", ["number"]) },
      { name: "date-unreadable", before: rich(), mutation: { id: "rev-1", date: "01.10.2026" }, outcome: reject("mutation.invariant", ["date"]) },
      { name: "blank-description", before: rich(), mutation: { id: "rev-1", description: "" }, outcome: reject("mutation.invariant", ["description"]) },
    ]),
  },
  {
    kind: "delete-sheet-revision", emoji: 0x1f4d8, variant: "DeleteSheetRevision", verb: "delete", entity: "sheet-revision", binaryTag: 14008, displayName: "Delete Sheet Revision",
    doc: "Removes a row from the revision table of a sheet, together with its properties and classifications.",
    props: [id("target", { en: "Revision", de: "Änderung" }, "sheet-revision")],
    inverseRows: { bounded: 4096 },
    label: { en: 'format!("Delete revision \\"{}\\"", self.id)', de: 'format!("Änderung \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "removes", before: rich(), mutation: { id: "rev-2" }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "rev-9" }, outcome: reject("mutation.target-missing", ["rev-9"]) },
    ]),
  },
];

const OPTIONAL: Record<string, string[]> = {
  "set-sheet": ["number", "name", "paper", "orientation", "project", "drawn_by", "checked_by", "date", "revision", "scale_label"],
  "set-viewport": ["sheet", "view", "position", "scale", "crop", "label"],
  "set-sheet-revision": ["number", "date", "description", "author"],
};

const sheetMapping = ["number", "name", "paper", "orientation", "project", "drawn_by", "checked_by", "date", "revision", "scale_label"];
const COPY = new Set(["orientation"]);
const patchBody = (fields: string[], from: string) => fields.map((field) => (COPY.has(field) ? `${field}: ${from}.${field}` : `${field}: ${from}.${field}.clone()`)).join(", ");
const revisionMapping = ["number", "date", "description", "author"];

const patchImpl = (variant: string, patch: string, fields: string[], rest = ", ..Default::default()") => `impl ${variant} {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ${patch} {
        ${patch} { ${patchBody(fields, "self")}${rest} }
    }

    /// 🧩 The payload that provides exactly the fields \`patch\` names.
    pub fn from_patch(id: String, patch: ${patch}) -> Self {
        Self { id, ${fields.map((field) => `${field}: patch.${field}`).join(", ")} }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for ${variant} {`;

const viewportPatchImpl = `impl SetViewport {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ViewportPatch {
        ViewportPatch { sheet: self.sheet.clone(), view: self.view.clone(), position: self.position, scale: self.scale, crop: self.crop.clone(), label: self.label.clone() }
    }

    /// 🧩 The payload that provides exactly the fields \`patch\` names.
    pub fn from_patch(id: String, patch: ViewportPatch) -> Self {
        Self { id, sheet: patch.sheet, view: patch.view, position: patch.position, scale: patch.scale, crop: patch.crop, label: patch.label }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetViewport {`;

const createDiff = (variant: string, noun: string, collection: string, field: string, problem: string, nounCap: string) => `//! 🔺️ Diff constructor for \`${variant}\`: one created ${noun} entry. The id must be free across every collection, then the ${noun} must be writable (see \`${problem}\`).

use super::super::elements;
use super::${variant};
use crate::{${problem}, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &${variant}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = ${problem}(base, &payload.id, &payload.${field}) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, ["${field}", problem.field]);
    }
    MutationOutcome::new(ModelDiff::${collection}(payload.id.clone(), Entry::Created(payload.${field}.clone())))
}
`;

const createInverse = (variant: string, deleteVariant: string, deleteModule: string, collection: string) => `//! ↩️ Inverse of \`${variant}\`: the concrete \`${deleteVariant}\` of the id it created, none when the id was already taken.

use super::super::${deleteModule}::${deleteVariant};
use super::${variant};
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &${variant}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.${collection}.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::${deleteVariant}(${deleteVariant} { id: payload.id.clone() })]
}
`;

const setDiff = (variant: string, noun: string, nounCap: string, collection: string, problem: string) => `//! 🔺️ Diff constructor for \`${variant}\`: a sparse ${noun} patch of exactly the provided fields that differ. The ${noun} that results must be writable (see \`${problem}\`); a patch that restates the current values is a
//! \`mutation.no-op\`.

use super::${variant};
use crate::{${problem}, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &${variant}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.${collection}.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("${nounCap} \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let patch = payload.patch().minimal(record);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("${nounCap} \\"{}\\" already holds these values.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = ${problem}(base, &payload.id, &patch.write(record)) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, [problem.field]);
    }
    MutationOutcome::new(ModelDiff::${collection}(payload.id.clone(), Entry::Patched(patch)))
}
`;

const setInverse = (variant: string, noun: string, collection: string) => `//! ↩️ Inverse of \`${variant}\`: an absolute \`${variant}\` restoring the base value of exactly the fields the forward really changes, none when the ${noun} is absent or nothing changes.

use super::${variant};
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &${variant}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.${collection}.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).negate(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::${variant}(${variant}::from_patch(payload.id.clone(), restore))]
}
`;

const deleteDiff = (variant: string, noun: string, nounCap: string, collection: string) => `//! 🔺️ Diff constructor for \`${variant}\`: the ${noun} leaves in one sparse diff together with ${variant === "DeleteSheet" ? "its viewports, its revision rows, " : ""}its properties and classifications (see the shared cascade). The outcome carries a cascade note when more than the
//! ${noun} leaves.

use super::super::cascade;
use super::${variant};
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &${variant}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.${collection}.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("${nounCap} \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "${nounCap}", Some(&payload.id))
}
`;

const deleteInverse = (variant: string) => `//! ↩️ Inverse of \`${variant}\`: one concrete create per removed record and one setter per removed property or classification, in storage order (dependants first, the target last), so the store, which replays the
//! vector reversed, recreates the target before anything on it.

use super::super::cascade;
use super::${variant};
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &${variant}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
`;

const MODULES: Record<string, { diff: string; inverse: string; mutation?: (source: string) => string }> = {
  "create-sheet": { diff: createDiff("CreateSheet", "sheet", "sheets", "sheet", "sheet_problem", "Sheet"), inverse: createInverse("CreateSheet", "DeleteSheet", "delete_sheet", "sheets") },
  "set-sheet": { diff: setDiff("SetSheet", "sheet", "Sheet", "sheets", "sheet_problem"), inverse: setInverse("SetSheet", "sheet", "sheets"), mutation: (source) => source.replace("impl MutationKind<ModelSnapshot, ModelMutation> for SetSheet {", patchImpl("SetSheet", "SheetPatch", sheetMapping, "")) },
  "delete-sheet": { diff: deleteDiff("DeleteSheet", "sheet", "Sheet", "sheets"), inverse: deleteInverse("DeleteSheet") },
  "create-viewport": { diff: createDiff("CreateViewport", "viewport", "viewports", "viewport", "viewport_problem", "Viewport"), inverse: createInverse("CreateViewport", "DeleteViewport", "delete_viewport", "viewports") },
  "set-viewport": { diff: setDiff("SetViewport", "viewport", "Viewport", "viewports", "viewport_problem"), inverse: setInverse("SetViewport", "viewport", "viewports"), mutation: (source) => source.replace("impl MutationKind<ModelSnapshot, ModelMutation> for SetViewport {", viewportPatchImpl) },
  "delete-viewport": { diff: deleteDiff("DeleteViewport", "viewport", "Viewport", "viewports"), inverse: deleteInverse("DeleteViewport") },
  "create-sheet-revision": { diff: createDiff("CreateSheetRevision", "revision", "sheet_revisions", "sheet_revision", "revision_problem", "Revision"), inverse: createInverse("CreateSheetRevision", "DeleteSheetRevision", "delete_sheet_revision", "sheet_revisions") },
  "set-sheet-revision": { diff: setDiff("SetSheetRevision", "revision", "Revision", "sheet_revisions", "revision_problem"), inverse: setInverse("SetSheetRevision", "revision", "sheet_revisions"), mutation: (source) => source.replace("impl MutationKind<ModelSnapshot, ModelMutation> for SetSheetRevision {", patchImpl("SetSheetRevision", "SheetRevisionPatch", revisionMapping)) },
  "delete-sheet-revision": { diff: deleteDiff("DeleteSheetRevision", "revision", "Revision", "sheet_revisions"), inverse: deleteInverse("DeleteSheetRevision") },
};

const dir = (spec: Leaf) => join(mutations, em(spec.emoji) + spec.kind);
const sub = (spec: Leaf, emojiPoint: number, name: string) => join(dir(spec), em(emojiPoint) + name);

const fixup = (spec: Leaf) => {
  const optional = OPTIONAL[spec.kind] ?? [];
  const component = join(sub(spec, 0x1f9a0, "mutation"), RS);
  let source = readFileSync(component, "utf8");
  for (const name of optional) source = source.replace(new RegExp(`^    pub ${name}: Option<`, "m"), `    #[value(default, skip_serializing_if = "Option::is_none")]\n    pub ${name}: Option<`);
  source = MODULES[spec.kind].mutation?.(source) ?? source;
  writeFileSync(component, source);
  const schemaFile = join(sub(spec, 0x1f9ec, "schema"), JSONF);
  const schema = JSON.parse(readFileSync(schemaFile, "utf8"));
  schema.required = schema.required.filter((name: string) => !optional.includes(name));
  writeFileSync(schemaFile, JSON.stringify(schema, null, 2) + "\n");
  writeFileSync(join(sub(spec, 0x1f53a, "diff"), RS), MODULES[spec.kind].diff);
  writeFileSync(join(sub(spec, 0x21a9, "inverse"), RS), MODULES[spec.kind].inverse);
};

const viewCascade = {
  name: "cascades-its-viewports",
  emoji: 0x1f9d5,
  before: rich(),
  mutation: { id: "v-section" },
  outcome: ok,
};

if (import.meta.main) {
  const out = join(import.meta.dir, "🗑️generated", "w2-wp14-sheets");
  mkdirSync(out, { recursive: true });
  let mounts = "";
  for (const spec of leaves) {
    mounts += emitLeaf(spec);
    fixup(spec);
  }
  writeFileSync(join(out, "mounts.txt"), mounts);
  writeFileSync(join(out, "delete-view-case.txt"), emitCase("delete-view", em(0x1f4fa) + "delete-view", "DeleteView", viewCascade));
  console.log(`emitted ${leaves.length} leaves and 1 delete-view case`);
}
