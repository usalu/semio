#!/usr/bin/env bun
/**
 * 🏗️ Wave W2 `w2-wp19-frame` (binary tags 19000..19009): the ten new frame leaves of `s.bim.model@1` (set-beam-axis, set-column-tilt, curtain wall type create/set/delete/set-of,
 * set-curtain-wall-grid, curtain panel override create/set/delete) and the regeneration of the three leaves whose payload changed with the new record shapes (set-beam,
 * set-curtain-wall, create-curtain-wall). `bun r12-w2-wp19-leaves.ts` rewrites the boilerplate and fixtures through `emitLeaf`, fixes the optional `set-*` fields of the payload structs
 * and schemas, writes the hand logic files (`🔺️diff`, `↩️inverse`) of the leaves whose logic is declared here, adds extra cases to existing leaves and mounts everything once in the
 * artifact root. `after`/`diff` fixtures of applied cases are never overwritten: bless them with `BIM_BLESS=1 cargo test`.
 */
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitCase, emitLeaf, type Case, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { artifact, em, JSONF, mutations, RS } from "./r3-f1-paths.ts";

type Label = { en: string; de: string };
type Mine = Prop & { optional?: true };
type MineLeaf = Omit<Leaf, "props"> & { props: Mine[]; force?: true; patch?: { type: string; expr: string; from: string } };

const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
const record = (def: string) => ({ $ref: `${ART}#/$defs/${def}` });
const id = (entity: string, role: "target" | "identity", label: Label, order = 10): Mine => ({
  name: "id",
  rust: "String",
  schema: { type: "string" },
  ui: role === "target" ? { widget: "reference", role, label, ref: { kind: entity }, group: "target", order } : { widget: "text", role: "identity", label, group: "identity", order },
});
const recordProp = (name: string, def: string, label: Label, order = 20, group = "value"): Mine => ({ name, rust: def, schema: record(def), ui: { widget: "record", role: "value", label, group, order } });
const sparse = (prop: Mine, description: Label): Mine => ({ ...prop, optional: true, rust: `Option<${prop.rust}>`, ui: { ...prop.ui, description } });
const keep: Label = { en: "Leave empty to keep the current value.", de: "Leer lassen, um den aktuellen Wert zu behalten." };
const reference = (name: string, kind: string, label: Label, order: number, group = "type"): Mine => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "reference", role: "value", label, ref: { kind }, group, order } });
const length = (name: string, label: Label, order: number): Mine => ({ name, rust: "f64", schema: { type: "number" }, ui: { widget: "number", role: "value", label, group: "placement", order, unit: "m", step: 0.01, precision: 3 } as Prop["ui"] });
const text = (name: string, label: Label, order: number): Mine => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "text", role: "value", label, group: "identity", order } });
const topProp = (order: number): Mine => recordProp("top", "TopConstraint", { en: "Top", de: "Oberkante" }, order, "placement");
const assignedProp = (name: string, def: string, label: Label, order: number): Mine => ({
  name,
  rust: `Assigned<Option<${def === "f64" ? "f64" : def}>>`,
  schema: { type: "object", additionalProperties: false, required: ["value"], properties: { value: { oneOf: [{ type: "null" }, def === "f64" ? { type: "number" } : record(def)] } } },
  ui: { widget: def === "f64" ? "number" : "record", role: "value", label, group: "value", order, ...(def === "f64" ? { unit: "m", step: 0.01, precision: 3 } : {}) } as Prop["ui"],
});
const optionalTilt = (order: number): Mine => ({ name: "tilt", rust: "Option<Slope>", optional: true, schema: record("Slope"), ui: { widget: "record", role: "value", label: { en: "Tilt", de: "Neigung" }, group: "placement", order, description: { en: "Leave empty for a plumb column.", de: "Leer lassen für eine lotrechte Stütze." } } });

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";
const APPLIED = [0x2705, 0x2795, 0x2728, 0x1f44d, 0x1f9f2, 0x1f31f, 0x1f4aa, 0x1f389, 0x1f680, 0x1f48e];
const REJECTED = [0x1f6ab, 0x26d4, 0x274c, 0x1f6d1, 0x1f6b7, 0x1f645, 0x1f4db, 0x1f6a7, 0x1f9ef, 0x2757, 0x1f4a5, 0x1f9f1, 0x1f573, 0x1f9ed, 0x1f9e9];
type Row = { name: string; before: unknown; mutation: Record<string, unknown>; outcome: Case["outcome"] };
const cases = (rows: Row[]): Case[] => {
  let applied = 0;
  let rejected = 0;
  return rows.map((row) => ({ ...row, emoji: row.outcome.status === "applied" ? APPLIED[applied++] : REJECTED[rejected++] }));
};

//#region 🔖️Scenes
const P = F.P;
const rect = (width: number, depth: number) => ({ Rectangle: { width, depth } });
const line = (a: [number, number], b: [number, number]) => F.line(a, b);
const arc = (a: [number, number], b: [number, number], bulge: number) => ({ Arc: { start: P(...a), end: P(...b), bulge } });
const spacing = (spacing: number) => ({ Spacing: { spacing } });
const lines = (...positions: number[]) => ({ Lines: { positions } });
const glassType = (label: string, u = 1.5, v = 1.5) => ({ name: label, u_grid: spacing(u), v_grid: spacing(v), interior_mullion: rect(0.05, 0.15), border_mullion: rect(0.08, 0.15), panel: "Glass", panel_material: "m-glass", mullion_material: "m-steel" });
const beam = (storey: string, type: string, axis: unknown, topOffset: number, label: string, end?: number) => ({ storey, beam_type: type, axis, top_offset: topOffset, ...(end === undefined ? {} : { end_top_offset: end }), phase: "New", name: label });
const column = (storey: string, type: string, at: [number, number], top: unknown, label: string, tilt?: { direction: number; angle: number }) => ({ storey, column_type: type, position: P(...at), rotation: 0, ...(tilt ? { tilt } : {}), base_offset: 0, top, phase: "New", name: label });
const facade = (type: string, axis: unknown, label: string, grids: { u_grid?: unknown; v_grid?: unknown } = {}) => ({ storey: "st-ground", curtain_wall_type: type, axis, base_offset: 0, top: F.storeyTop(0), ...grids, phase: "New", name: label });
const cell = (curtain: string, u: number, v: number, panel: unknown) => ({ curtain, u, v, panel });
const door = (type = "door-1") => ({ Door: { door_type: type } });
const window_ = (type = "win-1") => ({ Window: { window_type: type } });
const solid = (material = "m-wood") => ({ Solid: { material } });

const frame = (extra: Record<string, unknown> = {}) => ({
  ...F.scene(),
  materials: { "m-brick": F.material("Brick"), "m-steel": F.material("Steel"), "m-glass": F.material("Glass"), "m-wood": F.material("Wood"), "m-spare": F.material("Spare") },
  column_types: { "ct-400": { name: "Concrete 40x40", profile: rect(0.4, 0.4), material: "m-brick" }, "ct-500": { name: "Concrete 50x50", profile: rect(0.5, 0.5), material: "m-brick" } },
  beam_types: { "bt-30x50": { name: "Concrete 30x50", profile: rect(0.3, 0.5), material: "m-brick" }, "bt-40x60": { name: "Concrete 40x60", profile: rect(0.4, 0.6), material: "m-brick" } },
  window_types: { "win-1": { name: "Window", width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.07, frame_depth: 0.12, panes: 2, material: "m-wood" }, "win-2": { name: "Spare window", width: 1, height: 1, sill: 0.9, frame_width: 0.07, frame_depth: 0.12, panes: 1, material: "m-wood" } },
  door_types: { "door-1": { name: "Door", width: 0.9, height: 2.1, frame_width: 0.06, frame_depth: 0.12, leaves: "Single", swing: "Left", material: "m-wood" }, "door-2": { name: "Spare door", width: 1, height: 2.1, frame_width: 0.06, frame_depth: 0.12, leaves: "Single", swing: "Left", material: "m-wood" } },
  curtain_wall_types: { "cwt-1": glassType("Facade 1.5 x 1.5"), "cwt-2": glassType("Facade 2 x 3", 2, 3) },
  ...extra,
});
const withBeam = () => frame({ beams: { "b-a": beam("st-ground", "bt-30x50", line([0, 0], [4, 0]), -0.1, "Beam A") } });
const withArcBeam = () => frame({ beams: { "b-a": beam("st-ground", "bt-30x50", arc([0, 0], [4, 0], 0.4), -0.1, "Beam A") } });
const withInclined = () => frame({ beams: { "b-a": beam("st-ground", "bt-30x50", line([0, 0], [4, 0]), -0.1, "Beam A", -0.5) } });
const withColumn = () => frame({ columns: { "c-a1": column("st-ground", "ct-400", [0, 0], F.storeyTop(0), "A1") } });
const withLeaning = () => frame({ columns: { "c-a1": column("st-ground", "ct-400", [0, 0], F.storeyTop(0), "A1", { direction: 0.5, angle: 0.2 }) } });
const FACADE = () => facade("cwt-1", line([0, 0], [6, 0]), "South facade");
const withFacade = () => frame({ curtain_walls: { "cw-1": FACADE() } });
const withGrid = () => frame({ curtain_walls: { "cw-1": facade("cwt-1", line([0, 0], [6, 0]), "South facade", { u_grid: lines(2, 4) }) } });
const withOverride = () => frame({ curtain_walls: { "cw-1": FACADE() }, curtain_panel_overrides: { "ov-1": cell("cw-1", 2, 0, door()) } });
const withData = () => ({ ...withFacade(), properties: { "cw-1": { Pset_CurtainWallCommon: { FireRating: { Text: { value: "REI 30" } } } } }, classifications: { "cw-1": { system: "Uniclass", code: "EF_25_80", title: "Curtain walling" } } });
const withHosted = () => frame({ curtain_walls: { "cw-1": FACADE() }, openings: { "o-1": { host: "cw-1", kind: { Window: { window_type: "win-1" } }, offset: 2, flip_hand: false, flip_facing: false, name: "Vent" } }, curtain_panel_overrides: { "ov-1": cell("cw-1", 2, 0, door()), "ov-2": cell("cw-1", 1, 1, window_()) } });
const withSpareType = () => frame({ curtain_wall_types: { "cwt-1": glassType("Facade 1.5 x 1.5"), "cwt-2": glassType("Facade 2 x 3", 2, 3) } });
const withUsedType = () => frame({ curtain_walls: { "cw-2": facade("cwt-2", line([0, 0], [6, 0]), "East facade") } });
//#endregion 🔖️Scenes

//#region 🔖️Leaves
const typeRef: Label = { en: "Curtain wall type", de: "Fassadentyp" };
const wallRef: Label = { en: "Curtain wall", de: "Vorhangfassade" };
const overrideRef: Label = { en: "Panel override", de: "Paneel-Abweichung" };
const beamRef: Label = { en: "Beam", de: "Träger" };

const leaves: MineLeaf[] = [
  {
    kind: "set-beam-axis", emoji: 0x1fa9d, variant: "SetBeamAxis", verb: "set", entity: "beam", binaryTag: 19000, displayName: "Set Beam Axis",
    doc: "Replaces the axis of a beam, a line or an arc by bulge, exactly like the axis of a wall; this is how a beam is moved, stretched, curved or straightened. The inferred sweep, joins and quantities follow.",
    props: [id("beam", "target", beamRef), recordProp("axis", "Axis", { en: "Axis", de: "Achse" })],
    label: { en: 'format!("Reshape the axis of beam \\"{}\\"", self.id)', de: 'format!("Achse von Träger \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "curves-the-beam", before: withBeam(), mutation: { id: "b-a", axis: arc([0, 0], [4, 0], 0.4) }, outcome: ok },
      { name: "straightens-the-arc", before: withArcBeam(), mutation: { id: "b-a", axis: line([0, 0], [4, 0]) }, outcome: ok },
      { name: "moves-the-beam", before: withBeam(), mutation: { id: "b-a", axis: line([1, 1], [5, 1]) }, outcome: ok },
      { name: "keeps-the-inclination", before: withInclined(), mutation: { id: "b-a", axis: arc([0, 0], [4, 0], -0.3) }, outcome: ok },
      { name: "zero-length", before: withBeam(), mutation: { id: "b-a", axis: line([2, 2], [2, 2]) }, outcome: reject("mutation.invariant", ["axis"]) },
      { name: "flat-arc", before: withBeam(), mutation: { id: "b-a", axis: arc([0, 0], [4, 0], 0) }, outcome: reject("mutation.invariant", ["axis", "bulge"]) },
      { name: "unchanged", before: withBeam(), mutation: { id: "b-a", axis: line([0, 0], [4, 0]) }, outcome: reject("mutation.no-op", ["b-a"]) },
      { name: "missing", before: withBeam(), mutation: { id: "b-gone", axis: line([0, 0], [4, 0]) }, outcome: reject("mutation.target-missing", ["b-gone"]) },
    ]),
  },
  {
    kind: "set-column-tilt", emoji: 0x1f5fd, variant: "SetColumnTilt", verb: "set", entity: "column", binaryTag: 19001, displayName: "Set Column Tilt",
    doc: "Sets the lean of a column about its base point: the top leans towards the direction by the angle from the vertical, at most 60 degrees; leaving the tilt out makes the column plumb. The tilted solid, its volume and joins follow by inference.",
    props: [id("column", "target", { en: "Column", de: "Stütze" }), optionalTilt(20)],
    label: { en: 'format!("Set the tilt of column \\"{}\\"", self.id)', de: 'format!("Neigung von Stütze \\"{}\\" setzen", self.id)' },
    target,
    cases: cases([
      { name: "leans-the-column", before: withColumn(), mutation: { id: "c-a1", tilt: { direction: 1.5707963267948966, angle: 0.1 } }, outcome: ok },
      { name: "re-aims-the-lean", before: withLeaning(), mutation: { id: "c-a1", tilt: { direction: 3.141592653589793, angle: 0.3 } }, outcome: ok },
      { name: "straightens-the-column", before: withLeaning(), mutation: { id: "c-a1" }, outcome: ok },
      { name: "angle-zero", before: withColumn(), mutation: { id: "c-a1", tilt: { direction: 0, angle: 0 } }, outcome: reject("mutation.invariant", ["tilt", "angle"]) },
      { name: "too-steep", before: withColumn(), mutation: { id: "c-a1", tilt: { direction: 0, angle: 1.2 } }, outcome: reject("mutation.invariant", ["tilt", "angle"]) },
      { name: "negative-angle", before: withColumn(), mutation: { id: "c-a1", tilt: { direction: 0, angle: -0.1 } }, outcome: reject("mutation.invariant", ["tilt", "angle"]) },
      { name: "unchanged", before: withLeaning(), mutation: { id: "c-a1", tilt: { direction: 0.5, angle: 0.2 } }, outcome: reject("mutation.no-op", ["c-a1"]) },
      { name: "already-plumb", before: withColumn(), mutation: { id: "c-a1" }, outcome: reject("mutation.no-op", ["c-a1"]) },
      { name: "missing", before: withColumn(), mutation: { id: "c-gone", tilt: { direction: 0, angle: 0.1 } }, outcome: reject("mutation.target-missing", ["c-gone"]) },
    ]),
  },
  {
    kind: "create-curtain-wall-type", emoji: 0x1f3df, variant: "CreateCurtainWallType", verb: "create", entity: "curtain-wall-type", binaryTag: 19002, displayName: "Create Curtain Wall Type",
    doc: "Brings a new curtain wall type into the library: the grid rules of both directions, the mullion sections of the interior grid lines and of the border, the default panel and the materials.",
    props: [id("curtain-wall-type", "identity", { en: "Curtain wall type id", de: "Fassadentyp-Id" }, 10), recordProp("curtain_wall_type", "CurtainWallType", typeRef)],
    label: { en: 'format!("Create curtain wall type \\"{}\\"", self.curtain_wall_type.name)', de: 'format!("Fassadentyp \\"{}\\" anlegen", self.curtain_wall_type.name)' },
    target,
    cases: cases([
      { name: "adds-a-uniform-grid", before: frame(), mutation: { id: "cwt-3", curtain_wall_type: glassType("Facade 1 x 1", 1, 1) }, outcome: ok },
      { name: "adds-explicit-lines-and-a-solid-panel", before: frame(), mutation: { id: "cwt-3", curtain_wall_type: { ...glassType("Lined"), u_grid: lines(1, 2.5), v_grid: lines(1), panel: solid() } }, outcome: ok },
      { name: "adds-a-door-default", before: frame(), mutation: { id: "cwt-3", curtain_wall_type: { ...glassType("Doors"), panel: door() } }, outcome: ok },
      { name: "duplicate", before: frame(), mutation: { id: "cwt-1", curtain_wall_type: glassType("Again") }, outcome: reject("mutation.duplicate-id", ["cwt-1"]) },
      { name: "id-taken-by-another-kind", before: frame(), mutation: { id: "st-ground", curtain_wall_type: glassType("Again") }, outcome: reject("mutation.duplicate-id", ["st-ground"]) },
      { name: "spacing-non-positive", before: frame(), mutation: { id: "cwt-3", curtain_wall_type: { ...glassType("Flat"), u_grid: spacing(0) } }, outcome: reject("mutation.invariant", ["curtain_wall_type", "u_grid", "spacing"]) },
      { name: "lines-not-ascending", before: frame(), mutation: { id: "cwt-3", curtain_wall_type: { ...glassType("Unsorted"), v_grid: lines(2, 1) } }, outcome: reject("mutation.invariant", ["curtain_wall_type", "v_grid", "positions"]) },
      { name: "mullion-flat", before: frame(), mutation: { id: "cwt-3", curtain_wall_type: { ...glassType("Thin"), border_mullion: rect(0, 0.1) } }, outcome: reject("mutation.invariant", ["curtain_wall_type", "border_mullion"]) },
      { name: "door-type-missing", before: frame(), mutation: { id: "cwt-3", curtain_wall_type: { ...glassType("Ghost door"), panel: door("door-9") } }, outcome: reject("mutation.target-missing", ["curtain_wall_type", "panel", "door_type"]) },
      { name: "material-missing", before: frame(), mutation: { id: "cwt-3", curtain_wall_type: { ...glassType("Ghost glass"), panel_material: "m-ghost" } }, outcome: reject("mutation.target-missing", ["curtain_wall_type", "panel_material"]) },
    ]),
  },
  {
    kind: "set-curtain-wall-type", emoji: 0x1f3e4, variant: "SetCurtainWallType", verb: "set", entity: "curtain-wall-type", binaryTag: 19003, displayName: "Set Curtain Wall Type",
    doc: "Patches exactly the provided fields of a curtain wall type: name, grid rules, mullion sections, default panel and materials; every curtain wall of the type follows by inference.",
    props: [
      id("curtain-wall-type", "target", typeRef),
      sparse(text("name", { en: "Name", de: "Name" }, 20), keep),
      sparse(recordProp("u_grid", "CurtainGrid", { en: "Grid along the wall", de: "Raster entlang der Wand" }, 30), keep),
      sparse(recordProp("v_grid", "CurtainGrid", { en: "Grid up the wall", de: "Raster nach oben" }, 40), keep),
      sparse(recordProp("interior_mullion", "Profile", { en: "Interior mullion", de: "Innenpfosten" }, 50), keep),
      sparse(recordProp("border_mullion", "Profile", { en: "Border mullion", de: "Randpfosten" }, 60), keep),
      sparse(recordProp("panel", "CurtainPanel", { en: "Default panel", de: "Standardfüllung" }, 70), keep),
      sparse(reference("panel_material", "material", { en: "Glass material", de: "Glasmaterial" }, 80, "material"), keep),
      sparse(reference("mullion_material", "material", { en: "Mullion material", de: "Pfostenmaterial" }, 90, "material"), keep),
    ],
    patch: {
      type: "CurtainWallTypePatch",
      expr: "CurtainWallTypePatch { name: self.name.clone(), u_grid: self.u_grid.clone(), v_grid: self.v_grid.clone(), interior_mullion: self.interior_mullion.clone(), border_mullion: self.border_mullion.clone(), panel: self.panel.clone(), panel_material: self.panel_material.clone(), mullion_material: self.mullion_material.clone() }",
      from: "Self { id, name: patch.name, u_grid: patch.u_grid, v_grid: patch.v_grid, interior_mullion: patch.interior_mullion, border_mullion: patch.border_mullion, panel: patch.panel, panel_material: patch.panel_material, mullion_material: patch.mullion_material }",
    },
    label: { en: 'format!("Edit curtain wall type \\"{}\\"", self.id)', de: 'format!("Fassadentyp \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "renames", before: frame(), mutation: { id: "cwt-1", name: "Facade (renamed)" }, outcome: ok },
      { name: "regrids", before: frame(), mutation: { id: "cwt-1", u_grid: lines(1, 2, 3), v_grid: spacing(1) }, outcome: ok },
      { name: "swaps-the-mullions", before: frame(), mutation: { id: "cwt-1", interior_mullion: rect(0.04, 0.12), border_mullion: rect(0.1, 0.2) }, outcome: ok },
      { name: "changes-the-default-panel", before: frame(), mutation: { id: "cwt-1", panel: window_() }, outcome: ok },
      { name: "swaps-the-materials", before: frame(), mutation: { id: "cwt-1", panel_material: "m-spare", mullion_material: "m-brick" }, outcome: ok },
      { name: "restates-an-unchanged-field", before: frame(), mutation: { id: "cwt-1", name: "Facade 1.5 x 1.5", panel: solid() }, outcome: ok },
      { name: "empty-patch", before: frame(), mutation: { id: "cwt-1" }, outcome: reject("mutation.no-op", ["cwt-1"]) },
      { name: "unchanged", before: frame(), mutation: { id: "cwt-1", name: "Facade 1.5 x 1.5", panel: "Glass" }, outcome: reject("mutation.no-op", ["cwt-1"]) },
      { name: "spacing-non-positive", before: frame(), mutation: { id: "cwt-1", v_grid: spacing(-1) }, outcome: reject("mutation.invariant", ["v_grid", "spacing"]) },
      { name: "lines-not-ascending", before: frame(), mutation: { id: "cwt-1", u_grid: lines(3, 3) }, outcome: reject("mutation.invariant", ["u_grid", "positions"]) },
      { name: "window-type-missing", before: frame(), mutation: { id: "cwt-1", panel: window_("win-9") }, outcome: reject("mutation.target-missing", ["panel", "window_type"]) },
      { name: "material-missing", before: frame(), mutation: { id: "cwt-1", mullion_material: "m-ghost" }, outcome: reject("mutation.target-missing", ["mullion_material"]) },
      { name: "missing", before: frame(), mutation: { id: "cwt-9", name: "Ghost" }, outcome: reject("mutation.target-missing", ["cwt-9"]) },
    ]),
  },
  {
    kind: "delete-curtain-wall-type", emoji: 0x1f3e5, variant: "DeleteCurtainWallType", verb: "delete", entity: "curtain-wall-type", binaryTag: 19004, displayName: "Delete Curtain Wall Type",
    doc: "Removes a curtain wall type that no curtain wall uses.",
    props: [id("curtain-wall-type", "target", typeRef)],
    label: { en: 'format!("Delete curtain wall type \\"{}\\"", self.id)', de: 'format!("Fassadentyp \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "removes", before: withSpareType(), mutation: { id: "cwt-2" }, outcome: ok },
      { name: "used-by-curtain-walls", before: withUsedType(), mutation: { id: "cwt-2" }, outcome: reject("mutation.target-referenced", ["cwt-2"]) },
      { name: "missing", before: frame(), mutation: { id: "cwt-9" }, outcome: reject("mutation.target-missing", ["cwt-9"]) },
    ]),
  },
  {
    kind: "set-curtain-wall-type-of", emoji: 0x1f3e6, variant: "SetCurtainWallTypeOf", verb: "set", entity: "curtain-wall", binaryTag: 19005, displayName: "Set Curtain Wall Type Of",
    doc: "Builds a curtain wall as another curtain wall type; its grid, mullions and panels follow by inference.",
    props: [id("curtain-wall", "target", wallRef), reference("curtain_wall_type", "curtain-wall-type", typeRef, 20)],
    label: { en: 'format!("Set type of curtain wall \\"{}\\" to \\"{}\\"", self.id, self.curtain_wall_type)', de: 'format!("Typ von Vorhangfassade \\"{}\\" auf \\"{}\\" setzen", self.id, self.curtain_wall_type)' },
    target,
    cases: cases([
      { name: "retypes-the-facade", before: withFacade(), mutation: { id: "cw-1", curtain_wall_type: "cwt-2" }, outcome: ok },
      { name: "keeps-the-overrides", before: withOverride(), mutation: { id: "cw-1", curtain_wall_type: "cwt-2" }, outcome: ok },
      { name: "unchanged", before: withFacade(), mutation: { id: "cw-1", curtain_wall_type: "cwt-1" }, outcome: reject("mutation.no-op", ["cw-1"]) },
      { name: "type-missing", before: withFacade(), mutation: { id: "cw-1", curtain_wall_type: "cwt-9" }, outcome: reject("mutation.target-missing", ["curtain_wall_type"]) },
      { name: "missing", before: withFacade(), mutation: { id: "cw-9", curtain_wall_type: "cwt-2" }, outcome: reject("mutation.target-missing", ["cw-9"]) },
    ]),
  },
  {
    kind: "set-curtain-wall-grid", emoji: 0x1f3e7, variant: "SetCurtainWallGrid", verb: "set", entity: "curtain-wall", binaryTag: 19006, displayName: "Set Curtain Wall Grid",
    doc: "Sets, per direction, the grid rule of one curtain wall that replaces the rule of its type: explicit grid lines (a whole list owned by the curtain wall) or a uniform spacing; assigning nothing clears the override and the wall follows its type again.",
    props: [
      id("curtain-wall", "target", wallRef),
      sparse(assignedProp("u_grid", "CurtainGrid", { en: "Grid along the wall", de: "Raster entlang der Wand" }, 20), keep),
      sparse(assignedProp("v_grid", "CurtainGrid", { en: "Grid up the wall", de: "Raster nach oben" }, 30), keep),
    ],
    patch: { type: "CurtainWallPatch", expr: "CurtainWallPatch { u_grid: self.u_grid.clone(), v_grid: self.v_grid.clone(), ..Default::default() }", from: "Self { id, u_grid: patch.u_grid, v_grid: patch.v_grid }" },
    label: { en: 'format!("Set the grid of curtain wall \\"{}\\"", self.id)', de: 'format!("Raster von Vorhangfassade \\"{}\\" setzen", self.id)' },
    target,
    cases: cases([
      { name: "sets-explicit-lines", before: withFacade(), mutation: { id: "cw-1", u_grid: { value: lines(2, 4) } }, outcome: ok },
      { name: "sets-a-uniform-spacing", before: withFacade(), mutation: { id: "cw-1", v_grid: { value: spacing(1) } }, outcome: ok },
      { name: "sets-both-directions", before: withFacade(), mutation: { id: "cw-1", u_grid: { value: lines(3) }, v_grid: { value: lines(1, 2) } }, outcome: ok },
      { name: "adds-a-line", before: withGrid(), mutation: { id: "cw-1", u_grid: { value: lines(2, 3, 4) } }, outcome: ok },
      { name: "removes-a-line", before: withGrid(), mutation: { id: "cw-1", u_grid: { value: lines(2) } }, outcome: ok },
      { name: "clears-the-override", before: withGrid(), mutation: { id: "cw-1", u_grid: { value: null } }, outcome: ok },
      { name: "restates-an-unchanged-field", before: withGrid(), mutation: { id: "cw-1", u_grid: { value: lines(2, 4) }, v_grid: { value: spacing(0.5) } }, outcome: ok },
      { name: "unchanged", before: withGrid(), mutation: { id: "cw-1", u_grid: { value: lines(2, 4) } }, outcome: reject("mutation.no-op", ["cw-1"]) },
      { name: "names-no-field", before: withGrid(), mutation: { id: "cw-1" }, outcome: reject("mutation.no-op", ["cw-1"]) },
      { name: "already-following-the-type", before: withFacade(), mutation: { id: "cw-1", u_grid: { value: null } }, outcome: reject("mutation.no-op", ["cw-1"]) },
      { name: "lines-not-ascending", before: withFacade(), mutation: { id: "cw-1", u_grid: { value: lines(4, 2) } }, outcome: reject("mutation.invariant", ["u_grid", "positions"]) },
      { name: "line-at-the-start", before: withFacade(), mutation: { id: "cw-1", v_grid: { value: lines(0, 1) } }, outcome: reject("mutation.invariant", ["v_grid", "positions"]) },
      { name: "spacing-non-positive", before: withFacade(), mutation: { id: "cw-1", v_grid: { value: spacing(0) } }, outcome: reject("mutation.invariant", ["v_grid", "spacing"]) },
      { name: "missing", before: withFacade(), mutation: { id: "cw-9", u_grid: { value: lines(2) } }, outcome: reject("mutation.target-missing", ["cw-9"]) },
    ]),
  },
  {
    kind: "create-curtain-panel-override", emoji: 0x1f3e8, variant: "CreateCurtainPanelOverride", verb: "create", entity: "curtain-panel-override", binaryTag: 19007, displayName: "Create Curtain Panel Override",
    doc: "Overrides the panel of one cell of a curtain wall, named by its indices along the wall and up it; at most one override per cell. A door panel belongs in the base row.",
    props: [id("curtain-panel-override", "identity", { en: "Panel override id", de: "Paneel-Abweichung-Id" }, 10), recordProp("curtain_panel_override", "CurtainPanelOverride", overrideRef)],
    label: { en: 'format!("Override cell ({}, {}) of curtain wall \\"{}\\"", self.curtain_panel_override.u, self.curtain_panel_override.v, self.curtain_panel_override.curtain)', de: 'format!("Feld ({}, {}) von Vorhangfassade \\"{}\\" abweichend füllen", self.curtain_panel_override.u, self.curtain_panel_override.v, self.curtain_panel_override.curtain)' },
    target,
    cases: cases([
      { name: "adds-a-door", before: withFacade(), mutation: { id: "ov-1", curtain_panel_override: cell("cw-1", 2, 0, door()) }, outcome: ok },
      { name: "adds-a-window", before: withFacade(), mutation: { id: "ov-1", curtain_panel_override: cell("cw-1", 1, 1, window_()) }, outcome: ok },
      { name: "adds-a-solid-panel", before: withFacade(), mutation: { id: "ov-1", curtain_panel_override: cell("cw-1", 0, 0, solid()) }, outcome: ok },
      { name: "adds-an-empty-cell", before: withOverride(), mutation: { id: "ov-2", curtain_panel_override: cell("cw-1", 3, 1, "Empty") }, outcome: ok },
      { name: "duplicate", before: withOverride(), mutation: { id: "ov-1", curtain_panel_override: cell("cw-1", 0, 0, solid()) }, outcome: reject("mutation.duplicate-id", ["ov-1"]) },
      { name: "id-taken-by-another-kind", before: withFacade(), mutation: { id: "cw-1", curtain_panel_override: cell("cw-1", 0, 0, solid()) }, outcome: reject("mutation.duplicate-id", ["cw-1"]) },
      { name: "cell-taken", before: withOverride(), mutation: { id: "ov-2", curtain_panel_override: cell("cw-1", 2, 0, window_()) }, outcome: reject("mutation.duplicate-id", ["ov-1"]) },
      { name: "curtain-wall-missing", before: withFacade(), mutation: { id: "ov-1", curtain_panel_override: cell("cw-9", 0, 0, solid()) }, outcome: reject("mutation.target-missing", ["curtain_panel_override", "curtain"]) },
      { name: "door-type-missing", before: withFacade(), mutation: { id: "ov-1", curtain_panel_override: cell("cw-1", 0, 0, door("door-9")) }, outcome: reject("mutation.target-missing", ["curtain_panel_override", "panel", "door_type"]) },
      { name: "window-type-missing", before: withFacade(), mutation: { id: "ov-1", curtain_panel_override: cell("cw-1", 0, 0, window_("win-9")) }, outcome: reject("mutation.target-missing", ["curtain_panel_override", "panel", "window_type"]) },
      { name: "material-missing", before: withFacade(), mutation: { id: "ov-1", curtain_panel_override: cell("cw-1", 0, 0, solid("m-ghost")) }, outcome: reject("mutation.target-missing", ["curtain_panel_override", "panel", "material"]) },
    ]),
  },
  {
    kind: "set-curtain-panel-override", emoji: 0x1f3e9, variant: "SetCurtainPanelOverride", verb: "set", entity: "curtain-panel-override", binaryTag: 19008, displayName: "Set Curtain Panel Override",
    doc: "Changes the panel of an existing override; the cell it addresses stays the same (to address another cell, delete the override and create it again).",
    props: [id("curtain-panel-override", "target", overrideRef), recordProp("panel", "CurtainPanel", { en: "Panel", de: "Füllung" })],
    label: { en: 'format!("Change the panel of override \\"{}\\"", self.id)', de: 'format!("Füllung der Abweichung \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "swaps-the-panel", before: withOverride(), mutation: { id: "ov-1", panel: window_() }, outcome: ok },
      { name: "empties-the-cell", before: withOverride(), mutation: { id: "ov-1", panel: "Empty" }, outcome: ok },
      { name: "another-door-type", before: withOverride(), mutation: { id: "ov-1", panel: door("door-2") }, outcome: ok },
      { name: "unchanged", before: withOverride(), mutation: { id: "ov-1", panel: door() }, outcome: reject("mutation.no-op", ["ov-1"]) },
      { name: "door-type-missing", before: withOverride(), mutation: { id: "ov-1", panel: door("door-9") }, outcome: reject("mutation.target-missing", ["panel", "door_type"]) },
      { name: "material-missing", before: withOverride(), mutation: { id: "ov-1", panel: solid("m-ghost") }, outcome: reject("mutation.target-missing", ["panel", "material"]) },
      { name: "missing", before: withOverride(), mutation: { id: "ov-9", panel: "Glass" }, outcome: reject("mutation.target-missing", ["ov-9"]) },
    ]),
  },
  {
    kind: "delete-curtain-panel-override", emoji: 0x1f3ea, variant: "DeleteCurtainPanelOverride", verb: "delete", entity: "curtain-panel-override", binaryTag: 19009, displayName: "Delete Curtain Panel Override",
    doc: "Removes the override of one cell; the cell is filled by the default panel of the curtain wall type again.",
    props: [id("curtain-panel-override", "target", overrideRef)],
    label: { en: 'format!("Delete panel override \\"{}\\"", self.id)', de: 'format!("Paneel-Abweichung \\"{}\\" löschen", self.id)' },
    target,
    inverseRows: { bounded: 4096 },
    cases: cases([
      { name: "removes", before: withOverride(), mutation: { id: "ov-1" }, outcome: ok },
      { name: "missing", before: withOverride(), mutation: { id: "ov-9" }, outcome: reject("mutation.target-missing", ["ov-9"]) },
    ]),
  },
  {
    kind: "set-beam", emoji: 0x1f4d0, variant: "SetBeam", verb: "set", entity: "beam", binaryTag: -1, displayName: "Set Beam", force: true,
    doc: "Edits a beam sparsely: type, top offset at the start (signed: positive above, negative below the storey top), top offset at the end (assigned: an inclined beam, or none for a level one) and name; absent fields stay untouched. Its axis is set by `set-beam-axis`.",
    props: [
      id("beam", "target", beamRef),
      sparse(reference("beam_type", "beam-type", { en: "Beam type", de: "Trägertyp" }, 20), keep),
      sparse(length("top_offset", { en: "Top offset at the start (m)", de: "Oberkante am Anfang (m)" }, 30), keep),
      sparse(assignedProp("end_top_offset", "f64", { en: "Top offset at the end (m)", de: "Oberkante am Ende (m)" }, 40), keep),
      sparse(text("name", { en: "Name", de: "Name" }, 50), keep),
    ],
    patch: { type: "BeamPatch", expr: "BeamPatch { beam_type: self.beam_type.clone(), top_offset: self.top_offset, end_top_offset: self.end_top_offset.clone(), name: self.name.clone(), ..Default::default() }", from: "Self { id, beam_type: patch.beam_type, top_offset: patch.top_offset, end_top_offset: patch.end_top_offset, name: patch.name }" },
    label: { en: 'format!("Edit beam \\"{}\\"", self.id)', de: 'format!("Träger \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "retypes-and-lowers", before: withBeam(), mutation: { id: "b-a", beam_type: "bt-40x60", top_offset: -0.3 }, outcome: ok },
      { name: "inclines-the-beam", before: withBeam(), mutation: { id: "b-a", end_top_offset: { value: -0.6 } }, outcome: ok },
      { name: "levels-the-beam", before: withInclined(), mutation: { id: "b-a", end_top_offset: { value: null } }, outcome: ok },
      { name: "renames", before: withBeam(), mutation: { id: "b-a", name: "Beam A (renamed)" }, outcome: ok },
      { name: "keeps-equal-fields-out-of-the-diff", before: withBeam(), mutation: { id: "b-a", beam_type: "bt-30x50", top_offset: -0.4 }, outcome: ok },
      { name: "nothing-to-change", before: withBeam(), mutation: { id: "b-a", beam_type: "bt-30x50", name: "Beam A" }, outcome: reject("mutation.no-op", ["b-a"]) },
      { name: "names-no-field", before: withBeam(), mutation: { id: "b-a" }, outcome: reject("mutation.no-op", ["b-a"]) },
      { name: "already-level", before: withBeam(), mutation: { id: "b-a", end_top_offset: { value: null } }, outcome: reject("mutation.no-op", ["b-a"]) },
      { name: "type-missing", before: withBeam(), mutation: { id: "b-a", beam_type: "bt-missing" }, outcome: reject("mutation.target-missing", ["beam_type"]) },
      { name: "missing", before: withBeam(), mutation: { id: "b-gone", name: "Gone" }, outcome: reject("mutation.target-missing", ["b-gone"]) },
    ]),
  },
  {
    kind: "set-curtain-wall", emoji: 0x1f506, variant: "SetCurtainWall", verb: "set", entity: "curtain-wall", binaryTag: -1, displayName: "Set Curtain Wall", force: true,
    doc: "Adjusts the named parameters of a curtain wall (axis, base offset, top, name); every field left out stays as it is. Its type is set by `set-curtain-wall-type-of`, its grid by `set-curtain-wall-grid`.",
    props: [
      id("curtain-wall", "target", wallRef),
      sparse(recordProp("axis", "Axis", { en: "Axis", de: "Achse" }, 20), keep),
      sparse(length("base_offset", { en: "Base offset (m)", de: "Fußversatz (m)" }, 30), keep),
      sparse(topProp(40), keep),
      sparse(text("name", { en: "Name", de: "Name" }, 50), keep),
    ],
    patch: { type: "CurtainWallPatch", expr: "CurtainWallPatch { axis: self.axis.clone(), base_offset: self.base_offset, top: self.top.clone(), name: self.name.clone(), ..Default::default() }", from: "Self { id, axis: patch.axis, base_offset: patch.base_offset, top: patch.top, name: patch.name }" },
    label: { en: 'format!("Adjust curtain wall \\"{}\\"", self.id)', de: 'format!("Vorhangfassade \\"{}\\" anpassen", self.id)' },
    target,
    cases: cases([
      { name: "moves-the-facade", before: withFacade(), mutation: { id: "cw-1", axis: line([0, 1], [6, 1]), base_offset: 0.2 }, outcome: ok },
      { name: "curves-the-facade", before: withFacade(), mutation: { id: "cw-1", axis: arc([0, 0], [6, 0], 0.3) }, outcome: ok },
      { name: "constrains-the-top", before: withFacade(), mutation: { id: "cw-1", top: F.toStorey("st-first", 0.5) }, outcome: ok },
      { name: "renames", before: withFacade(), mutation: { id: "cw-1", name: "Street facade" }, outcome: ok },
      { name: "restates-an-unchanged-field", before: withFacade(), mutation: { id: "cw-1", base_offset: 0, name: "North facade" }, outcome: ok },
      { name: "unchanged", before: withFacade(), mutation: { id: "cw-1", name: "South facade", base_offset: 0 }, outcome: reject("mutation.no-op", ["cw-1"]) },
      { name: "names-no-field", before: withFacade(), mutation: { id: "cw-1" }, outcome: reject("mutation.no-op", ["cw-1"]) },
      { name: "zero-length", before: withFacade(), mutation: { id: "cw-1", axis: line([1, 1], [1, 1]) }, outcome: reject("mutation.invariant", ["axis"]) },
      { name: "top-storey-missing", before: withFacade(), mutation: { id: "cw-1", top: F.toStorey("st-attic", 0) }, outcome: reject("mutation.target-missing", ["top", "storey"]) },
      { name: "missing", before: withFacade(), mutation: { id: "cw-9", name: "Ghost" }, outcome: reject("mutation.target-missing", ["cw-9"]) },
    ]),
  },
  {
    kind: "create-curtain-wall", emoji: 0x1f3ec, variant: "CreateCurtainWall", verb: "create", entity: "curtain-wall", binaryTag: -1, displayName: "Create Curtain Wall", force: true,
    doc: "Brings a new curtain wall onto a storey; its height is never stored, it is inferred from the top constraint, its grid, mullions and panels from its type.",
    props: [id("curtain-wall", "identity", { en: "Curtain wall id", de: "Vorhangfassaden-Id" }, 10), recordProp("curtain_wall", "CurtainWall", wallRef)],
    label: { en: 'format!("Create curtain wall \\"{}\\"", self.curtain_wall.name)', de: 'format!("Vorhangfassade \\"{}\\" anlegen", self.curtain_wall.name)' },
    target,
    cases: cases([
      { name: "adds-a-facade", before: withFacade(), mutation: { id: "cw-2", curtain_wall: facade("cwt-2", line([0, 6], [6, 6]), "North facade") }, outcome: ok },
      { name: "adds-a-curved-facade", before: withFacade(), mutation: { id: "cw-2", curtain_wall: facade("cwt-1", arc([0, 6], [6, 6], 0.3), "Bay facade") }, outcome: ok },
      { name: "adds-a-facade-with-its-own-grid", before: withFacade(), mutation: { id: "cw-2", curtain_wall: facade("cwt-1", line([0, 6], [6, 6]), "Lined facade", { u_grid: lines(1, 3), v_grid: spacing(1) }) }, outcome: ok },
      { name: "duplicate", before: withFacade(), mutation: { id: "cw-1", curtain_wall: facade("cwt-1", line([0, 6], [6, 6]), "Again") }, outcome: reject("mutation.duplicate-id", ["cw-1"]) },
      { name: "id-taken-by-another-kind", before: withFacade(), mutation: { id: "st-ground", curtain_wall: facade("cwt-1", line([0, 6], [6, 6]), "Again") }, outcome: reject("mutation.duplicate-id", ["st-ground"]) },
      { name: "storey-missing", before: withFacade(), mutation: { id: "cw-2", curtain_wall: { ...facade("cwt-1", line([0, 6], [6, 6]), "Attic"), storey: "st-attic" } }, outcome: reject("mutation.target-missing", ["curtain_wall", "storey"]) },
      { name: "type-missing", before: withFacade(), mutation: { id: "cw-2", curtain_wall: facade("cwt-9", line([0, 6], [6, 6]), "Ghost") }, outcome: reject("mutation.target-missing", ["curtain_wall", "curtain_wall_type"]) },
      { name: "zero-length", before: withFacade(), mutation: { id: "cw-2", curtain_wall: facade("cwt-1", line([1, 1], [1, 1]), "Flat") }, outcome: reject("mutation.invariant", ["curtain_wall", "axis"]) },
      { name: "lines-not-ascending", before: withFacade(), mutation: { id: "cw-2", curtain_wall: facade("cwt-1", line([0, 6], [6, 6]), "Unsorted", { u_grid: lines(3, 1) }) }, outcome: reject("mutation.invariant", ["curtain_wall", "u_grid", "positions"]) },
      { name: "spacing-non-positive", before: withFacade(), mutation: { id: "cw-2", curtain_wall: facade("cwt-1", line([0, 6], [6, 6]), "Flat", { v_grid: spacing(0) }) }, outcome: reject("mutation.invariant", ["curtain_wall", "v_grid", "spacing"]) },
    ]),
  },
];
//#endregion 🔖️Leaves

//#region 🔖️Extra
type Extra = { leaf: string; emoji: number; variant: string; cases: Case[] };
const extras: Extra[] = [
  {
    leaf: "create-beam", emoji: 0x2796, variant: "CreateBeam",
    cases: cases([
      { name: "adds-an-arc-beam", before: withBeam(), mutation: { id: "b-b", beam: beam("st-ground", "bt-40x60", arc([0, 3], [4, 3], 0.4), -0.2, "Arc beam") }, outcome: ok },
      { name: "adds-an-inclined-beam", before: withBeam(), mutation: { id: "b-b", beam: beam("st-ground", "bt-40x60", line([0, 3], [4, 3]), -0.2, "Inclined beam", -0.7) }, outcome: ok },
      { name: "flat-arc", before: withBeam(), mutation: { id: "b-b", beam: beam("st-ground", "bt-30x50", arc([0, 3], [4, 3], 0), -0.2, "Flat") }, outcome: reject("mutation.invariant", ["beam", "axis", "bulge"]) },
    ]),
  },
  {
    leaf: "split-beam", emoji: 0x1f956, variant: "SplitBeam",
    cases: cases([
      { name: "splits-an-arc-beam", before: withArcBeam(), mutation: { id: "b-a", t: 0.25, new_id: "b-b" }, outcome: ok },
      { name: "splits-an-inclined-beam", before: withInclined(), mutation: { id: "b-a", t: 0.5, new_id: "b-b" }, outcome: ok },
    ]),
  },
  {
    leaf: "create-column", emoji: 0x1f3db, variant: "CreateColumn",
    cases: cases([
      { name: "adds-a-leaning-column", before: withColumn(), mutation: { id: "c-b1", column: column("st-ground", "ct-400", [6, 0], F.storeyTop(0), "B1", { direction: 0.5, angle: 0.2 }) }, outcome: ok },
      { name: "tilt-too-steep", before: withColumn(), mutation: { id: "c-b1", column: column("st-ground", "ct-400", [6, 0], F.storeyTop(0), "B1", { direction: 0.5, angle: 1.3 }) }, outcome: reject("mutation.invariant", ["column", "tilt", "angle"]) },
    ]),
  },
  {
    leaf: "delete-curtain-wall", emoji: 0x1f996, variant: "DeleteCurtainWall",
    cases: cases([
      { name: "cascades-its-overrides", before: withHosted(), mutation: { id: "cw-1" }, outcome: ok },
    ]),
  },
];
//#endregion 🔖️Extra

//#region 🔖️Hand
const DIFF = em(0x1f53a);
const INVERSE = em(0x21a9);
const MUTATION = em(0x1f9a0);
const snake = (kind: string) => kind.replaceAll("-", "_");
const rs = (strings: TemplateStringsArray, ...values: string[]) =>
  strings.raw
    .reduce((out, chunk, index) => out + chunk + (values[index] ?? ""), "")
    .replace(/\\u\{([0-9a-fA-F]+)\}/g, (_match, hex: string) => String.fromCodePoint(parseInt(hex, 16)))
    .replace(/\\u([0-9a-fA-F]{4})/g, (_match, hex: string) => String.fromCodePoint(parseInt(hex, 16)))
    .replaceAll("¶", "`")
    .replaceAll('\\\\"', '\\"');

const restoreInverse = (variant: string, module: string, collection: string, noun: string) => rs`//! ${INVERSE} Inverse of ¶${variant}¶: an absolute ¶${variant}¶ restoring the base value of exactly the fields the forward really changes, none when the ${noun} is absent or nothing changes.

use super::${variant};
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &${variant}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.${collection}.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::${variant}(${variant}::from_patch(payload.id.clone(), restore))]
}
`;

const createInverse = (variant: string, deleteVariant: string, collection: string) => rs`//! ${INVERSE} Inverse of ¶${variant}¶: the concrete ¶${deleteVariant}¶ of the id it created, none when the id was already taken.

use super::super::${snake(deleteVariant.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase())}::${deleteVariant};
use super::${variant};
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &${variant}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.${collection}.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::${deleteVariant}(${deleteVariant} { id: payload.id.clone() })]
}
`;

const deleteInverse = (variant: string, createVariant: string, collection: string, field: string) => rs`//! ${INVERSE} Inverse of ¶${variant}¶: the concrete ¶${createVariant}¶ carrying the full removed record, none when the record was absent.

use super::super::${snake(createVariant.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase())}::${createVariant};
use super::${variant};
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &${variant}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.${collection}.get(&payload.id) {
        Some(record) => vec![ModelMutation::${createVariant}(${createVariant} { id: payload.id.clone(), ${field}: record.clone() })],
        None => Vec::new(),
    }
}
`;

const HAND: Record<string, { diff: string; inverse: string }> = {
  "set-beam-axis": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetBeamAxis¶: a one-field beam patch. The axis must have finite end points, length and a real arc, exactly like the axis of a wall;
//! the inferred sweep, the joins with columns and the quantities follow by inference.

use super::super::wall_geometry::flaw;
use super::SetBeamAxis;
use crate::{BeamPatch, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetBeamAxis, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(beam) = base.beams.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(flaw) = flaw(&payload.axis) {
        return flaw.under(&["axis"]).refuse();
    }
    if beam.axis == payload.axis {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Beam \\"{}\\" already runs along this axis.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::beams(payload.id.clone(), Entry::Patched(BeamPatch { axis: Some(payload.axis.clone()), ..Default::default() })))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶SetBeamAxis¶: an absolute ¶SetBeamAxis¶ back to the base axis, none when the beam is absent.

use super::SetBeamAxis;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetBeamAxis, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.beams.get(&payload.id) {
        Some(beam) => vec![ModelMutation::SetBeamAxis(SetBeamAxis { id: payload.id.clone(), axis: beam.axis.clone() })],
        None => Vec::new(),
    }
}
`,
  },
  "set-column-tilt": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetColumnTilt¶: a one-field column patch that assigns the tilt (absent in the payload clears it, the column is plumb again). A tilt leans by a
//! positive angle of at most 60 degrees towards a finite direction. The tilted solid, its volume and the joins follow by inference; the position stays the base point.

use super::super::wall_geometry::tilt_flaw;
use super::SetColumnTilt;
use crate::{Assigned, ColumnPatch, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetColumnTilt, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(column) = base.columns.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Column \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(flaw) = payload.tilt.as_ref().and_then(tilt_flaw) {
        return flaw.under(&["tilt"]).refuse();
    }
    if column.tilt == payload.tilt {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Column \\"{}\\" already has this tilt.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::columns(payload.id.clone(), Entry::Patched(ColumnPatch { tilt: Some(Assigned::new(payload.tilt)), ..Default::default() })))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶SetColumnTilt¶: an absolute ¶SetColumnTilt¶ back to the base tilt (plumb when none), none when the column is absent.

use super::SetColumnTilt;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetColumnTilt, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.columns.get(&payload.id) {
        Some(column) => vec![ModelMutation::SetColumnTilt(SetColumnTilt { id: payload.id.clone(), tilt: column.tilt })],
        None => Vec::new(),
    }
}
`,
  },
  "create-curtain-wall-type": {
    diff: rs`//! ${DIFF} Diff constructor for ¶CreateCurtainWallType¶: one created curtain wall type entry. Both grid rules are sound (a positive spacing, or explicit lines at
//! positive distances in strictly ascending order), both mullion sections have positive dimensions, the default panel names existing types or materials and both
//! materials exist.

use super::super::elements;
use super::super::wall_geometry::curtain_type_flaw;
use super::CreateCurtainWallType;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateCurtainWallType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(flaw) = curtain_type_flaw(base, &payload.curtain_wall_type) {
        return flaw.under(&["curtain_wall_type"]).refuse();
    }
    MutationOutcome::new(ModelDiff::curtain_wall_types(payload.id.clone(), Entry::Created(payload.curtain_wall_type.clone())))
}
`,
    inverse: createInverse("CreateCurtainWallType", "DeleteCurtainWallType", "curtain_wall_types"),
  },
  "set-curtain-wall-type": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetCurtainWallType¶: a sparse curtain wall type patch of exactly the provided fields that differ. Provided grid rules, mullion
//! sections, default panel and materials follow the create rules; a patch that changes nothing is a no-op. Every curtain wall of the type follows by inference.

use super::super::wall_geometry::{grid_flaw, material_flaw, panel_flaw, profile_flaw, Flaw};
use super::SetCurtainWallType;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

fn flaw_of(payload: &SetCurtainWallType, base: &ModelSnapshot) -> Option<Flaw> {
    payload.u_grid.as_ref().and_then(grid_flaw).map(|flaw| flaw.under(&["u_grid"]))
        .or_else(|| payload.v_grid.as_ref().and_then(grid_flaw).map(|flaw| flaw.under(&["v_grid"])))
        .or_else(|| payload.interior_mullion.as_ref().and_then(profile_flaw).map(|flaw| flaw.under(&["interior_mullion"])))
        .or_else(|| payload.border_mullion.as_ref().and_then(profile_flaw).map(|flaw| flaw.under(&["border_mullion"])))
        .or_else(|| payload.panel.as_ref().and_then(|panel| panel_flaw(base, panel)).map(|flaw| flaw.under(&["panel"])))
        .or_else(|| payload.panel_material.as_deref().and_then(|id| material_flaw(base, "panel_material", id)))
        .or_else(|| payload.mullion_material.as_deref().and_then(|id| material_flaw(base, "mullion_material", id)))
}

pub fn diff(payload: &SetCurtainWallType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(current) = base.curtain_wall_types.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall type \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(flaw) = flaw_of(payload, base) {
        return flaw.refuse();
    }
    let change = payload.patch().minimal(current);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Curtain wall type \\"{}\\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_wall_types(payload.id.clone(), Entry::Patched(change)))
}
`,
    inverse: restoreInverse("SetCurtainWallType", "set_curtain_wall_type", "curtain_wall_types", "curtain wall type"),
  },
  "delete-curtain-wall-type": {
    diff: rs`//! ${DIFF} Diff constructor for ¶DeleteCurtainWallType¶: one deleted curtain wall type entry; refused while a curtain wall still uses the type.

use super::DeleteCurtainWallType;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteCurtainWallType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.curtain_wall_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall type \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.curtain_walls.values().any(|row| row.curtain_wall_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Curtain wall type \\"{}\\" is still used by curtain walls.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_wall_types(payload.id.clone(), Entry::Deleted))
}
`,
    inverse: deleteInverse("DeleteCurtainWallType", "CreateCurtainWallType", "curtain_wall_types", "curtain_wall_type"),
  },
  "set-curtain-wall-type-of": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetCurtainWallTypeOf¶: a one-field curtain wall patch. The curtain wall type must exist; grid, mullions and panels are inferred from it.

use super::SetCurtainWallTypeOf;
use crate::{CurtainWallPatch, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetCurtainWallTypeOf, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.curtain_walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !base.curtain_wall_types.contains_key(&payload.curtain_wall_type) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall type \\"{}\\" does not exist.", payload.curtain_wall_type), ["curtain_wall_type"]);
    }
    if wall.curtain_wall_type == payload.curtain_wall_type {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Curtain wall \\"{}\\" already is of type \\"{}\\".", payload.id, payload.curtain_wall_type), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_walls(payload.id.clone(), Entry::Patched(CurtainWallPatch { curtain_wall_type: Some(payload.curtain_wall_type.clone()), ..Default::default() })))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶SetCurtainWallTypeOf¶: an absolute ¶SetCurtainWallTypeOf¶ back to the base curtain wall type, none when the curtain wall is absent.

use super::SetCurtainWallTypeOf;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetCurtainWallTypeOf, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.curtain_walls.get(&payload.id) {
        Some(wall) => vec![ModelMutation::SetCurtainWallTypeOf(SetCurtainWallTypeOf { id: payload.id.clone(), curtain_wall_type: wall.curtain_wall_type.clone() })],
        None => Vec::new(),
    }
}
`,
  },
  "set-curtain-wall-grid": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetCurtainWallGrid¶: a sparse curtain wall patch of the provided grid overrides that differ. A provided rule is a uniform spacing (positive) or a
//! list of grid lines at positive distances in strictly ascending order; an assigned none clears the override so the wall follows its type again. The grid lines are ONE list owned
//! by the curtain wall: a provided list replaces the list as a whole, its lines have no identity of their own. Cells, panels and overrides follow by inference.

use super::super::wall_geometry::grid_flaw;
use super::SetCurtainWallGrid;
use crate::{CurtainWallPatch, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetCurtainWallGrid, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.curtain_walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    for (field, assigned) in [("u_grid", &payload.u_grid), ("v_grid", &payload.v_grid)] {
        if let Some(flaw) = assigned.as_ref().and_then(|assigned| assigned.value.as_ref()).and_then(grid_flaw) {
            return flaw.under(&[field]).refuse();
        }
    }
    let change = payload.patch().minimal(wall);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Curtain wall \\"{}\\" already has this grid.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_walls(payload.id.clone(), Entry::Patched(change)))
}
`,
    inverse: restoreInverse("SetCurtainWallGrid", "set_curtain_wall_grid", "curtain_walls", "curtain wall"),
  },
  "create-curtain-panel-override": {
    diff: rs`//! ${DIFF} Diff constructor for ¶CreateCurtainPanelOverride¶: one created override entry. The curtain wall must exist, a solid panel names an existing material, a door or
//! window panel an existing type, and the cell (curtain wall, u, v) is overridden at most once. Whether the cell lies inside the inferred grid is a diagnostic, never a
//! refusal: the grid follows the type and the wall and may change under the override.

use super::super::elements;
use super::super::wall_geometry::panel_flaw;
use super::CreateCurtainPanelOverride;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateCurtainPanelOverride, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let record = &payload.curtain_panel_override;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.curtain_walls.contains_key(&record.curtain) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall \\"{}\\" does not exist.", record.curtain), ["curtain_panel_override", "curtain"]);
    }
    if let Some(flaw) = panel_flaw(base, &record.panel) {
        return flaw.under(&["curtain_panel_override", "panel"]).refuse();
    }
    if let Some((held, _)) = base.curtain_panel_overrides.iter().find(|(_, row)| row.curtain == record.curtain && row.u == record.u && row.v == record.v) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("Cell ({}, {}) of curtain wall \\"{}\\" is already overridden by \\"{held}\\".", record.u, record.v, record.curtain), [held.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_panel_overrides(payload.id.clone(), Entry::Created(record.clone())))
}
`,
    inverse: createInverse("CreateCurtainPanelOverride", "DeleteCurtainPanelOverride", "curtain_panel_overrides"),
  },
  "set-curtain-panel-override": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetCurtainPanelOverride¶: a one-field override patch. A solid panel names an existing material, a door or window panel an existing type; the cell the
//! override addresses never changes (delete and create to address another cell).

use super::super::wall_geometry::panel_flaw;
use super::SetCurtainPanelOverride;
use crate::{CurtainPanelOverridePatch, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetCurtainPanelOverride, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.curtain_panel_overrides.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Panel override \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(flaw) = panel_flaw(base, &payload.panel) {
        return flaw.under(&["panel"]).refuse();
    }
    if record.panel == payload.panel {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Panel override \\"{}\\" already holds this panel.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_panel_overrides(payload.id.clone(), Entry::Patched(CurtainPanelOverridePatch { panel: Some(payload.panel.clone()), ..Default::default() })))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶SetCurtainPanelOverride¶: an absolute ¶SetCurtainPanelOverride¶ back to the base panel, none when the override is absent.

use super::SetCurtainPanelOverride;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetCurtainPanelOverride, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.curtain_panel_overrides.get(&payload.id) {
        Some(record) => vec![ModelMutation::SetCurtainPanelOverride(SetCurtainPanelOverride { id: payload.id.clone(), panel: record.panel.clone() })],
        None => Vec::new(),
    }
}
`,
  },
  "delete-curtain-panel-override": {
    diff: rs`//! ${DIFF} Diff constructor for ¶DeleteCurtainPanelOverride¶: the override leaves in one sparse diff (see the shared cascade); the cell is filled by the default panel of the type again.

use super::super::cascade;
use super::DeleteCurtainPanelOverride;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteCurtainPanelOverride, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.curtain_panel_overrides.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Panel override \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Panel override", Some(&payload.id))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶DeleteCurtainPanelOverride¶: the concrete create of the removed override, in storage order (see the shared cascade).

use super::super::cascade;
use super::DeleteCurtainPanelOverride;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteCurtainPanelOverride, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
`,
  },
  "set-beam": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetBeam¶: a sparse beam patch naming only the fields that really change. The beam type must exist and both top offsets must be finite
//! (the end offset is assigned: none makes the beam level again). The axis is the business of ¶set-beam-axis¶; no elevation is written, it is inferred.

use super::super::wall_geometry::offsets_flaw;
use super::SetBeam;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetBeam, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(beam) = base.beams.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(kind) = &payload.beam_type {
        if !base.beam_types.contains_key(kind) {
            return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam type \\"{kind}\\" does not exist."), ["beam_type"]);
        }
    }
    let end = payload.end_top_offset.as_ref().and_then(|assigned| assigned.value);
    if let Some(flaw) = offsets_flaw(payload.top_offset.unwrap_or(beam.top_offset), end) {
        return flaw.refuse();
    }
    let patch = payload.patch().minimal(beam);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Beam \\"{}\\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::beams(payload.id.clone(), Entry::Patched(patch)))
}
`,
    inverse: restoreInverse("SetBeam", "set_beam", "beams", "beam"),
  },
  "set-curtain-wall": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetCurtainWall¶: a sparse curtain wall patch of exactly the named fields that differ from the base. Only the named fields are validated (axis, top,
//! base offset); a payload that changes nothing is a no-op. The type is set by ¶set-curtain-wall-type-of¶ and the grid by ¶set-curtain-wall-grid¶.

use super::super::wall_geometry::{flaw, top_flaw, Flaw};
use super::SetCurtainWall;
use crate::{CurtainWall, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

fn flaw_of(payload: &SetCurtainWall, base: &ModelSnapshot, wall: &CurtainWall) -> Option<Flaw> {
    payload.axis.as_ref().and_then(|axis| flaw(axis).map(|flaw| flaw.under(&["axis"])))
        .or_else(|| payload.base_offset.and_then(|offset| (!offset.is_finite()).then(|| Flaw::new(OutcomeCode::Invariant, &["base_offset"], "A base offset must be finite."))))
        .or_else(|| payload.top.as_ref().and_then(|top| top_flaw(base, &wall.storey, top)))
}

pub fn diff(payload: &SetCurtainWall, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.curtain_walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(flaw) = flaw_of(payload, base, wall) {
        return flaw.refuse();
    }
    let patch = payload.patch().minimal(wall);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Curtain wall \\"{}\\" already has these parameters.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_walls(payload.id.clone(), Entry::Patched(patch)))
}
`,
    inverse: restoreInverse("SetCurtainWall", "set_curtain_wall", "curtain_walls", "curtain wall"),
  },
};

function fixLeaf(spec: MineLeaf) {
  const optionals = spec.props.filter((prop) => prop.optional).map((prop) => prop.name);
  const dir = join(mutations, em(spec.emoji) + spec.kind);
  const file = join(dir, MUTATION + "mutation", RS);
  const used = new Set(spec.props.flatMap((prop) => prop.rust.match(/[A-Z][A-Za-z0-9]*/g) ?? []).filter((name) => !["Option", "String", "Vec"].includes(name)));
  if (spec.patch) used.add(spec.patch.type);
  let source = readFileSync(file, "utf8");
  source = source.replace(/^use crate::\{.*\};$/m, `use crate::{${["ModelDiff", "ModelMutation", "ModelSnapshot", ...used].sort().join(", ")}};`);
  for (const name of optionals) source = source.replace(new RegExp(`^    pub ${name}: `, "m"), `    #[value(default, skip_serializing_if = "Option::is_none")]\n    pub ${name}: `);
  if (spec.patch) {
    source = source.replace(
      /^impl MutationKind/m,
      `impl ${spec.variant} {\n    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.\n    pub fn patch(&self) -> ${spec.patch.type} {\n        ${spec.patch.expr}\n    }\n\n    /// 🧩 The payload that provides exactly the fields \`patch\` names.\n    pub fn from_patch(id: String, patch: ${spec.patch.type}) -> Self {\n        ${spec.patch.from}\n    }\n}\n\nimpl MutationKind`,
    );
  }
  writeFileSync(file, source);
  if (optionals.length > 0) {
    const schemaFile = join(dir, readdirSync(dir).find((name) => name.endsWith("schema"))!, JSONF);
    const schema = JSON.parse(readFileSync(schemaFile, "utf8"));
    schema.required = schema.required.filter((name: string) => !optionals.includes(name));
    writeFileSync(schemaFile, JSON.stringify(schema, null, 2) + "\n");
  }
}

function writeHand(spec: MineLeaf) {
  const hand = HAND[spec.kind];
  if (!hand) return;
  const dir = join(mutations, em(spec.emoji) + spec.kind);
  for (const [folder, body] of [[DIFF + "diff", hand.diff], [INVERSE + "inverse", hand.inverse]] as const) {
    mkdirSync(join(dir, folder), { recursive: true });
    const file = join(dir, folder, RS);
    if (spec.force || !existsSync(file)) writeFileSync(file, body);
  }
}
//#endregion 🔖️Hand

//#region 🔖️Run
const rootPath = join(artifact, RS);
let root = readFileSync(rootPath, "utf8");
const crlf = root.includes("\r\n");
if (crlf) root = root.replaceAll("\r\n", "\n");
const anchor = "                        //#endregion 🔖️Leaves";

const dropBlock = (module: string) => {
  const open = `                        #[path = "."]\n                        pub mod ${module} {\n`;
  const at = root.indexOf(open);
  if (at < 0) return false;
  const close = root.indexOf("\n                        }\n", at) + "\n                        }\n".length;
  root = root.slice(0, at) + root.slice(close);
  return true;
};

const existingTags = new Map<string, number>();
for (const spec of leaves) {
  const descriptor = join(mutations, em(spec.emoji) + spec.kind, JSONF);
  if (spec.binaryTag < 0 && existsSync(descriptor)) existingTags.set(spec.kind, JSON.parse(readFileSync(descriptor, "utf8")).binaryTag);
}

let mounted = 0;
for (const spec of leaves) {
  const leafDir = join(mutations, em(spec.emoji) + spec.kind);
  if (spec.force) {
    rmSync(join(leafDir, em(0x1f9ea) + "tests"), { recursive: true, force: true });
    dropBlock(snake(spec.kind));
  }
  if (spec.binaryTag < 0) spec.binaryTag = existingTags.get(spec.kind)!;
  const block = emitLeaf(spec as unknown as Leaf);
  fixLeaf(spec);
  writeHand(spec);
  if (!root.includes(`pub mod ${snake(spec.kind)} {`)) {
    root = root.replace(anchor, block + anchor);
    mounted += 1;
  }
}

const addCases = (extra: Extra) => {
  const leafDirName = em(extra.emoji) + extra.leaf;
  const module = snake(extra.leaf);
  const open = `                        pub mod ${module} {\n`;
  const at = root.indexOf(open);
  if (at < 0) throw new Error(`no mount block for ${module}`);
  const close = root.indexOf("\n                        }\n", at);
  let lines = "";
  for (const c of extra.cases) {
    if (root.slice(at, close).includes(`mod tests_${snake(c.name)};`)) continue;
    lines += "\n" + emitCase(extra.leaf, leafDirName, extra.variant, c).replace(/\n$/, "");
  }
  root = root.slice(0, close) + lines + root.slice(close);
};
for (const extra of extras) addCases(extra);

writeFileSync(rootPath, crlf ? root.replaceAll("\n", "\r\n") : root);
console.log(`w2-wp19-frame: ${leaves.length} leaves emitted, ${mounted} newly mounted, ${extras.reduce((n, e) => n + e.cases.length, 0)} extra cases`);
//#endregion 🔖️Run
