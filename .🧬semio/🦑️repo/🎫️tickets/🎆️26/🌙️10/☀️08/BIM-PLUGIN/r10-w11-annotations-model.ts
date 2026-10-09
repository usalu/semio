/**
 * 📏️ The annotation vocabulary of `s.bim.model@1` (WP-11: dimensions, tags, text notes, leaders and their styles). `r3-f1-gen-model.ts`
 * spreads these rows into its single model, so Rust, JSON Schema, TypeScript, GraphQL and proto are generated from here and never drift.
 */
type Field = { name: string; type: string; doc?: string };

const f = (spec: string): Field[] =>
  spec.split(",").map((part) => part.trim()).filter(Boolean).map((part) => {
    const at = part.indexOf(":");
    return { name: part.slice(0, at).trim(), type: part.slice(at + 1).trim() };
  });

export const annotationUnitEnums = [
  { name: "AnchorEnd", doc: "📍️ Which end of a wall axis an anchor names: the start or the end of the axis.", variants: ["Start", "End"] },
  { name: "WallSide", doc: "↔️ Which face of a wall, looking along its axis from start to end: the left or the right face.", variants: ["Left", "Right"] },
  { name: "TagCategory", doc: "🏷️ What a tag prints of its element: its name, the name of its type, its number (spaces), or its size (width by height of an opening, thickness of a wall).", variants: ["Name", "Type", "Number", "Size"] },
  { name: "Terminator", doc: "🔚️ The mark at the end of a dimension line or leader: a slanted tick, a closed arrow head or a filled dot.", variants: ["Tick", "Arrow", "Dot"] },
  { name: "DimensionUnit", doc: "📏️ The unit a dimension value is printed in.", variants: ["Metre", "Centimetre", "Millimetre"] },
];

export const annotationDataEnums = [
  {
    name: "AnnotationAnchor",
    doc: "⚓️ What an annotation is attached to: a free point, or an element of the model (a wall face, axis or end, the centre of an opening or column, a grid line). An element anchor follows its element by inference, a point stays where it was put.",
    variants: [
      { name: "Point", fields: f("point:Point2") },
      { name: "WallFace", fields: f("wall:string, side:WallSide") },
      { name: "WallAxis", fields: f("wall:string") },
      { name: "WallEnd", fields: f("wall:string, end:AnchorEnd") },
      { name: "OpeningCentre", fields: f("opening:string") },
      { name: "Grid", fields: f("grid:string") },
      { name: "ColumnCentre", fields: f("column:string") },
    ],
  },
];

export const annotationStructs = [
  {
    name: "Dimension",
    doc: "📏️ A dimension: the distances between consecutive anchors measured along `angle`, drawn `offset` metres beside the first anchor. Its values and text are inferred from the current geometry of the anchors, never stored; an optional lock names the value it must keep.",
    entity: { collection: "dimensions", plural: "Dimensions" },
    fields: f("storey:string, anchors:vec:AnnotationAnchor, angle:f64, offset:f64, style:string, lock:opt:f64, name:string"),
  },
  {
    name: "Tag",
    doc: "🏷️ A tag: text read from an element of the model, placed `offset` metres from the reference point of the element so it follows the element.",
    entity: { collection: "tags", plural: "Tags" },
    fields: f("storey:string, element:string, category:TagCategory, offset:Point2, style:string"),
  },
  {
    name: "TextNote",
    doc: "🗒️ A free text on a storey plan.",
    entity: { collection: "text_notes", plural: "TextNotes" },
    fields: f("storey:string, position:Point2, text:string, rotation:f64, style:string"),
  },
  {
    name: "Leader",
    doc: "↗️ A leader: a text joined by a line to an anchor; the text sits `offset` metres from the anchor point so it follows the anchored element.",
    entity: { collection: "leaders", plural: "Leaders" },
    fields: f("storey:string, anchor:AnnotationAnchor, offset:Point2, text:string, style:string"),
  },
  {
    name: "AnnotationStyle",
    doc: "🎨️ A style shared by dimensions, tags, notes and leaders: text height, line end mark, printed unit and precision.",
    entity: { collection: "annotation_styles", plural: "AnnotationStyles" },
    fields: f("name:string, text_height:f64, terminator:Terminator, unit:DimensionUnit, precision:u32, mark_size:f64, gap:f64, overshoot:f64"),
  },
];

export const annotationFieldDocs: Record<string, string> = {
  "Dimension.storey": "The storey plan the dimension is drawn on.",
  "Dimension.anchors": "At least two anchors, in measuring order; each consecutive pair gives one segment.",
  "Dimension.angle": "Direction in radians, counter-clockwise from the x axis, along which distances are measured.",
  "Dimension.offset": "Signed distance in metres from the first anchor to the dimension line, positive to the left of the measuring direction.",
  "Dimension.lock": "Optional check-only lock: the total length in metres the dimension must keep. It never moves anything; a diagnostic reports a violated lock.",
  "Tag.offset": "Offset in metres of the tag text from the reference point of the element.",
  "TextNote.rotation": "Rotation of the text in radians, counter-clockwise.",
  "Leader.offset": "Offset in metres of the text from the anchor point.",
  "AnnotationStyle.text_height": "Height of the text in metres of model space.",
  "AnnotationStyle.precision": "Digits after the decimal point of a printed dimension value.",
  "AnnotationStyle.mark_size": "Size in metres of the end marks of dimension lines and leaders.",
  "AnnotationStyle.gap": "Gap in metres between an anchor and the start of its extension line.",
  "AnnotationStyle.overshoot": "Length in metres by which an extension line passes the dimension line.",
};

export const annotationCollections = ["dimensions", "tags", "text_notes", "leaders", "annotation_styles"];
