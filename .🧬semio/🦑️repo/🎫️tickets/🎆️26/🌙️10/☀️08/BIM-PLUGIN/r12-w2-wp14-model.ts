/**
 * 📄️ The sheet vocabulary of `s.bim.model@1` (WP-14: sheets, viewports and the revision table). `r3-f1-gen-model.ts` spreads these rows into its single model, so Rust, JSON Schema,
 * TypeScript, GraphQL and proto are generated from here and never drift.
 *
 * A sheet is a piece of paper of the project: its number, name, paper size, orientation and the authored fields of its title block. A viewport places one authored view on one
 * sheet at a scale (position, crop and label are authored too). Everything drawn on the sheet (the frame, the scaled linework, the title block, the revision table) is the inferred
 * `sheet-layout`; nothing of it is stored.
 */
type Field = { name: string; type: string; doc?: string };

const f = (spec: string): Field[] =>
  spec.split(",").map((part) => part.trim()).filter(Boolean).map((part) => {
    const at = part.indexOf(":");
    return { name: part.slice(0, at).trim(), type: part.slice(at + 1).trim() };
  });

export const sheetUnitEnums = [
  { name: "IsoSize", doc: "📄️ An ISO 216 A series paper size: A0 (841 by 1189 mm) to A4 (210 by 297 mm).", variants: ["A0", "A1", "A2", "A3", "A4"] },
  { name: "Orientation", doc: "🔄️ How a sheet lies: landscape (the long side is the width) or portrait (the long side is the height).", variants: ["Landscape", "Portrait"] },
];

export const sheetDataEnums = [
  {
    name: "Paper",
    doc: "📄️ The paper of a sheet: an ISO A size, or a custom size given by its two sides in millimetres (which side is the width follows from the orientation of the sheet).",
    variants: [
      { name: "Iso", fields: f("size:IsoSize") },
      { name: "Custom", fields: f("width:f64, height:f64") },
    ],
  },
];

export const sheetStructs = [
  {
    name: "Sheet",
    doc: "📄️ A sheet of the drawing set: number, name, paper and the authored fields of its title block. Its frame, title block and revision table are inferred, and so are the drawings its viewports show.",
    entity: { collection: "sheets", plural: "Sheets" },
    fields: f("number:string, name:string, paper:Paper, orientation:Orientation, project:string, drawn_by:string, checked_by:string, date:string, revision:string, scale_label:string"),
  },
  {
    name: "Viewport",
    doc: "🖼️ A viewport: one authored view placed on one sheet at a drawing scale. The drawing is the inferred linework of the view, scaled to the viewport and clipped to its window.",
    entity: { collection: "viewports", plural: "Viewports" },
    fields: f("sheet:string, view:string, position:Point2, scale:u32, crop:opt:ViewCrop, label:opt:string"),
  },
  {
    name: "SheetRevision",
    doc: "🧾️ One row of the revision table of a sheet: the revision mark, its date, what changed and who changed it.",
    entity: { collection: "sheet_revisions", plural: "SheetRevisions" },
    fields: f("sheet:string, number:string, date:string, description:string, author:string"),
  },
];

export const sheetFieldDocs: Record<string, string> = {
  "Sheet.number": "The sheet number as printed on the drawing, unique within the project (for example A-101).",
  "Sheet.name": "The sheet title as printed in the title block.",
  "Sheet.paper": "The paper of the sheet: an ISO A size or a custom size in millimetres.",
  "Sheet.orientation": "Landscape puts the long side of the paper along the width, portrait along the height.",
  "Sheet.project": "The project line of the title block; empty prints none.",
  "Sheet.drawn_by": "Who drew the sheet, as printed in the title block.",
  "Sheet.checked_by": "Who checked the sheet, as printed in the title block.",
  "Sheet.date": "The date of the sheet as year-month-day, as printed in the title block; empty prints none.",
  "Sheet.revision": "The current revision mark, as printed in the title block; empty prints the mark of the last row of the revision table.",
  "Sheet.scale_label": "The scale line of the title block (for example 1:100 or As indicated); empty prints the distinct scales of the viewports, comma-joined.",
  "Viewport.sheet": "The sheet the viewport stands on.",
  "Viewport.view": "The view the viewport shows: a plan, ceiling plan, section or elevation (a camera view draws nothing on paper).",
  "Viewport.position": "Where the top left corner of the viewport window lies on the paper, in millimetres from the top left corner of the sheet, x to the right and y downward.",
  "Viewport.scale": "Drawing scale denominator of the viewport: 100 draws the view at 1:100, from 1 to 1000.",
  "Viewport.crop": "Optional crop rectangle in the drawing coordinates of the view (metres): the window shows only that part of the drawing; absent shows the whole drawing.",
  "Viewport.label": "Optional title printed under the viewport; absent prints the name of the view.",
  "SheetRevision.sheet": "The sheet whose revision table holds the row.",
  "SheetRevision.number": "The revision mark (for example A, B or 1), unique within the sheet.",
  "SheetRevision.date": "The date of the revision as year-month-day; empty prints none.",
  "SheetRevision.description": "What the revision changed.",
  "SheetRevision.author": "Who made the revision.",
};

export const sheetCollections = ["sheets", "viewports", "sheet_revisions"];
