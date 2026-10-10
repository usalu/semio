/**
 * 🏗️ The frame vocabulary of `s.bim.model@1` (WP-19: arc and inclined beams, tilted columns, curtain-wall types, grids and keyed panel overrides). `r3-f1-gen-model.ts` spreads these
 * rows into its single model (and edits the `Beam`, `Column` and `CurtainWall` rows in place), so Rust, JSON Schema, TypeScript, GraphQL and proto are generated and never drift.
 */
type Field = { name: string; type: string; doc?: string };

const f = (spec: string): Field[] =>
  spec.split(",").map((part) => part.trim()).filter(Boolean).map((part) => {
    const at = part.indexOf(":");
    return { name: part.slice(0, at).trim(), type: part.slice(at + 1).trim() };
  });

export const frameDataEnums = [
  {
    name: "CurtainGrid",
    doc: "🕸️ The grid rule of one direction of a curtain wall: equal cells of about `spacing` metres, or explicit interior grid lines. Along the wall the positions are metres from the start of the axis, up the wall they are metres above its base; the cells are what lies between the outer edges and the lines.",
    variants: [{ name: "Spacing", fields: f("spacing:f64") }, { name: "Lines", fields: f("positions:vec:f64") }],
  },
  {
    name: "CurtainPanel",
    doc: "🪟️ What fills one cell of a curtain wall: a glass pane of the glass material of the type, a solid panel of a material, a door or a window of a type that fills the cell, or nothing.",
    variants: [{ name: "Glass", fields: [] }, { name: "Solid", fields: f("material:string") }, { name: "Door", fields: f("door_type:string") }, { name: "Window", fields: f("window_type:string") }, { name: "Empty", fields: [] }],
  },
];

export const frameStructs = [
  {
    name: "CurtainWallType",
    doc: "🏬️ A curtain wall type: the grid rules of both directions, the mullion sections of the interior grid lines and of the border, the default panel of every cell, the glass material and the mullion material.",
    entity: { collection: "curtain_wall_types", plural: "CurtainWallTypes" },
    fields: f("name:string, u_grid:CurtainGrid, v_grid:CurtainGrid, interior_mullion:Profile, border_mullion:Profile, panel:CurtainPanel, panel_material:string, mullion_material:string, u_value:opt:f64, g_value:opt:f64, frame_fraction:opt:f64"),
  },
  {
    name: "CurtainPanelOverride",
    doc: "🎯️ The panel of one cell of a curtain wall that differs from the default panel of its type; the cell is named by its indices along the wall and up it, at most one override per cell.",
    entity: { collection: "curtain_panel_overrides", plural: "CurtainPanelOverrides" },
    fields: f("curtain:string, u:u32, v:u32, panel:CurtainPanel"),
  },
];

export const frameFieldDocs: Record<string, string> = {
  "Beam.axis": "The line or arc of the beam in plan, from its start to its end, exactly like the axis of a wall; a curved beam is an axis whose bulge is set.",
  "Beam.top_offset": "Signed vertical offset in metres of the top of the beam at the start of its axis from the top of its storey: positive lifts it above the storey top, negative hangs it below, zero is flush.",
  "Beam.end_top_offset": "Optional signed offset in metres of the top of the beam at the end of its axis from the top of its storey; absent means the same as at the start (a level beam), a different value inclines the beam, linearly along the axis.",
  "Column.tilt": "Optional lean of the column: the top leans towards `direction` (radians, counter-clockwise from +x) by `angle` radians from the vertical, about the base point; absent is plumb. The cross-section perpendicular to the axis is the profile of the type, the ends are cut horizontally.",
  "CurtainWall.curtain_wall_type": "The curtain wall type that carries the grid rules, mullion sections, default panel and materials.",
  "CurtainWall.u_grid": "Optional grid rule along the wall that replaces the rule of the type; absent follows the type.",
  "CurtainWall.v_grid": "Optional grid rule up the wall that replaces the rule of the type; absent follows the type.",
  "CurtainWallType.u_grid": "Grid rule along the wall: equal cells of about the spacing, or explicit lines in metres from the start of the axis.",
  "CurtainWallType.v_grid": "Grid rule up the wall: equal cells of about the spacing, or explicit lines in metres above the base.",
  "CurtainWallType.interior_mullion": "Section of the mullions on the interior grid lines: the first coordinate runs along the wall, the second through it.",
  "CurtainWallType.border_mullion": "Section of the mullions on the outer edges of the wall.",
  "CurtainWallType.panel": "The panel of every cell that has no override.",
  "CurtainWallType.panel_material": "Material of glass panels.",
  "CurtainWallType.mullion_material": "Material of every mullion.",
  "CurtainPanelOverride.curtain": "The curtain wall whose cell is overridden.",
  "CurtainPanelOverride.u": "Zero-based index of the cell along the wall, from the start of the axis.",
  "CurtainPanelOverride.v": "Zero-based index of the cell up the wall, from the base; a door panel belongs in row zero.",
  "CurtainPanelOverride.panel": "What fills the cell instead of the default panel.",
};
