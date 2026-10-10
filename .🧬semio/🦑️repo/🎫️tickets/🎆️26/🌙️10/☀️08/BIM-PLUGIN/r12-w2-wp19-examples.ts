/**
 * 🏗️ Wave W2 `w2-wp19-frame`: the beams, columns and curtain walls of the `office` example, merged into the snapshot source by `r4-x-examples-gen.ts`. The office gets the curtain wall type `cwt-facade` (1.5 x 1.3 m
 * cells, glass, 5 x 15 cm mullions) that every façade wall follows, and the ground-floor south wall gets its own height grid (a 2.4 m door row under a 1.6 m top row) with a service door in the door row, an open
 * vent and a concrete spandrel in the top row; the first-floor south wall gets a ribbon window panel and the second-floor north wall an explicit list of five-metre bays. The first floor gets an arc beam
 * between the columns E1 and F2 (its ends are cut back to the column faces), a leaning feature column in the open office; the third floor an inclined beam between E3 and F4. Every value is authored (type, grid
 * rules, cells, panels, axis, offsets, lean); the cut lengths, volumes, panel counts and joins are inferred. `frameFor(name, model)` edits `model` in place and refuses a reference that points at nothing.
 */
type Json = Record<string, any>;

const rect = (width: number, depth: number) => ({ Rectangle: { width, depth } });
const P = (x: number, y: number) => ({ x, y });

export const FACADE_TYPE = {
  name: "Curtain wall 1.5 x 1.3",
  u_grid: { Spacing: { spacing: 1.5 } },
  v_grid: { Spacing: { spacing: 1.3 } },
  interior_mullion: rect(0.05, 0.15),
  border_mullion: rect(0.05, 0.15),
  panel: "Glass",
  panel_material: "m-glass",
  mullion_material: "m-steel",
};

const override = (curtain: string, u: number, v: number, panel: unknown) => ({ curtain, u, v, panel });
const need = (model: Json, collection: string, id: string, by: string) => {
  if (model[collection]?.[id] === undefined) throw new Error(`frame: ${by} names ${collection}/${id}, which does not exist`);
};

export function frameFor(name: string, model: Json): void {
  model.curtain_wall_types = name === "office" ? { "cwt-facade": FACADE_TYPE } : {};
  model.curtain_panel_overrides = {};
  if (name !== "office") return;
  for (const [id, wall] of Object.entries<Json>(model.curtain_walls)) {
    wall.curtain_wall_type = "cwt-facade";
    need(model, "curtain_wall_types", wall.curtain_wall_type, `curtain_walls/${id}`);
  }
  Object.assign(model.curtain_walls["cu-0-south"], { v_grid: { Lines: { positions: [2.4] } } });
  Object.assign(model.curtain_walls["cu-2-north"], { u_grid: { Lines: { positions: [5, 10, 15, 20, 25] } } });
  model.curtain_panel_overrides = {
    "cpo-0-south-door": override("cu-0-south", 15, 0, { Door: { door_type: "dr-core" } }),
    "cpo-0-south-vent": override("cu-0-south", 3, 1, "Empty"),
    "cpo-0-south-spandrel": override("cu-0-south", 6, 1, { Solid: { material: "m-concrete" } }),
    "cpo-1-south-window": override("cu-1-south", 4, 0, { Window: { window_type: "wn-ribbon" } }),
  };
  model.columns["c-1-lean"] = { storey: "st-1", column_type: "ct-rc-400", position: P(15, 3), rotation: 0, tilt: { direction: 0, angle: 0.1 }, base_offset: 0, top: { StoreyTop: { offset: 0 } }, name: "Leaning Column" };
  model.beams["bm-1-arc-E1-F2"] = { storey: "st-1", beam_type: "bt-rc-secondary", axis: { Arc: { start: P(24, 0), end: P(30, 6), bulge: -0.25 } }, top_offset: -0.36, name: "Arc Beam E1-F2" };
  model.beams["bm-3-incline-E3-F4"] = { storey: "st-3", beam_type: "bt-rc-secondary", axis: { Line: { start: P(24, 12), end: P(30, 18) } }, top_offset: -0.36, end_top_offset: -1, name: "Inclined Beam E3-F4" };
  for (const [id, row] of Object.entries<Json>(model.curtain_panel_overrides)) {
    need(model, "curtain_walls", row.curtain, `curtain_panel_overrides/${id}`);
    const panel = row.panel;
    if (panel.Door) need(model, "door_types", panel.Door.door_type, `curtain_panel_overrides/${id}`);
    if (panel.Window) need(model, "window_types", panel.Window.window_type, `curtain_panel_overrides/${id}`);
    if (panel.Solid) need(model, "materials", panel.Solid.material, `curtain_panel_overrides/${id}`);
  }
  for (const id of ["c-1-E1", "c-1-F2", "c-3-E3", "c-3-F4"]) need(model, "columns", id, "the beam joins");
  need(model, "materials", FACADE_TYPE.panel_material, "cwt-facade");
  need(model, "materials", FACADE_TYPE.mullion_material, "cwt-facade");
}
