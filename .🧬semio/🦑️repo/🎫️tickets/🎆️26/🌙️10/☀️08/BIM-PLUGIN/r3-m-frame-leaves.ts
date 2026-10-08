#!/usr/bin/env bun
/**
 * 🏛️ Wave M slice `m-frame` of `s.bim.model@1`: columns and beams (create / delete / sparse set), binary tags 500 to 505.
 * `bun r3-m-frame-leaves.ts` is idempotent: it rewrites the generated boilerplate and fixtures through `emitLeaf`, patches the
 * optional (sparse) payload fields, copies the hand-written diff and inverse sources from `🗑️generated/m-frame/src` while that
 * scratch folder exists, writes the authored-top unit tests and prints the mount block to `🗑️generated/m-frame/mounts.txt`.
 */
import { copyFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { artifact, em, JSONF, mutations, RS } from "./r3-f1-paths.ts";

type Label = { en: string; de: string };
type Mine = Prop & { optional?: true };
type MineLeaf = Omit<Leaf, "props"> & { props: Mine[]; uses?: string[] };

const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
const record = (def: string) => ({ $ref: `${ART}#/$defs/${def}` });
const id = (entity: string, role: "target" | "identity", label: Label, order = 10): Mine => ({
  name: "id",
  rust: "String",
  schema: { type: "string" },
  ui: role === "target" ? { widget: "reference", role, label, ref: { kind: entity }, group: "target", order } : { widget: "text", role: "identity", label, group: "identity", order },
});
const recordProp = (name: string, def: string, label: Label, order = 20): Mine => ({ name, rust: def, schema: record(def), ui: { widget: "record", role: "value", label, group: "value", order } });
const sparse = (prop: Mine, description: Label): Mine => ({ ...prop, optional: true, rust: `Option<${prop.rust}>`, ui: { ...prop.ui, description } });
const keep: Label = { en: "Leave empty to keep the current value.", de: "Leer lassen, um den aktuellen Wert zu behalten." };
const reference = (name: string, kind: string, label: Label, order: number): Mine => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "reference", role: "value", label, ref: { kind }, group: "type", order } });
const length = (name: string, label: Label, order: number): Mine => ({ name, rust: "f64", schema: { type: "number" }, ui: { widget: "number", role: "value", label, group: "placement", order, unit: "m", step: 0.01, precision: 3 } as Prop["ui"] });
const angle = (name: string, label: Label, order: number): Mine => ({
  name,
  rust: "f64",
  schema: { type: "number" },
  ui: { widget: "dial", role: "value", label, group: "placement", order, unit: "rad", displayUnit: "deg", displayFactor: 57.29577951308232, step: 0.017453292519943295, precision: 4 } as Prop["ui"],
});
const point = (name: string, label: Label, order: number): Mine => ({ name, rust: "Point2", schema: record("Point2"), ui: { widget: "record", role: "value", label, group: "placement", order } });
const topProp = (order: number): Mine => ({ name: "top", rust: "TopConstraint", schema: record("TopConstraint"), ui: { widget: "record", role: "value", label: { en: "Top", de: "Oberkante" }, group: "placement", order } });
const name = (order: number): Mine => ({ name: "name", rust: "String", schema: { type: "string" }, ui: { widget: "text", role: "value", label: { en: "Name", de: "Name" }, group: "identity", order } });

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";

const rect = (width: number, depth: number) => ({ Rectangle: { width, depth } });
const columnType = (label: string, side: number) => ({ name: label, profile: rect(side, side), material: "m-brick" });
const beamType = (label: string, width: number, depth: number) => ({ name: label, profile: rect(width, depth), material: "m-brick" });
const column = (storey: string, type: string, at: [number, number], top: unknown, label: string, rotation = 0, baseOffset = 0) => ({ storey, column_type: type, position: F.P(...at), rotation, base_offset: baseOffset, top, name: label });
const beam = (storey: string, type: string, a: [number, number], b: [number, number], topOffset: number, label: string) => ({ storey, beam_type: type, start: F.P(...a), end: F.P(...b), top_offset: topOffset, name: label });

const frame = (extra: { columns?: Record<string, unknown>; beams?: Record<string, unknown> } = {}) => ({
  ...F.scene(),
  column_types: { "ct-400": columnType("Concrete 40x40", 0.4), "ct-500": columnType("Concrete 50x50", 0.5) },
  beam_types: { "bt-30x50": beamType("Concrete 30x50", 0.3, 0.5), "bt-40x60": beamType("Concrete 40x60", 0.4, 0.6) },
  columns: extra.columns ?? {},
  beams: extra.beams ?? {},
});
const withColumn = () => frame({ columns: { "c-a1": column("st-ground", "ct-400", [0, 0], F.storeyTop(0), "A1") } });
const withBeam = () => frame({ beams: { "b-a": beam("st-ground", "bt-30x50", [0, 0], [4, 0], -0.1, "Beam A") } });
const twoBuildings = () => {
  const base = withColumn();
  return { ...base, buildings: { ...base.buildings, "bldg-2": F.building("site-1", "Annex") }, storeys: { ...base.storeys, "st-annex": F.storey("bldg-2", "Annex", 0, 3) } };
};

const E = { ok: 0x2705, no: 0x1f6ab, miss: 0x26d4, magnet: 0x1f9f2, compass: 0x1f9ed, down: 0x1f53b, crane: 0x1f3d7, puzzle: 0x1f9e9, badge: 0x1f4db, stop: 0x1f6d1, warn: 0x26a0, spark: 0x2728, scales: 0x2696, ruler: 0x1f4cf, wrench: 0x1f527 };

const newColumn = (top: unknown, label = "B1", at: [number, number] = [6, 0]) => column("st-ground", "ct-400", at, top, label);

export const leaves: MineLeaf[] = [
  {
    kind: "create-column", emoji: 0x1f3db, variant: "CreateColumn", verb: "create", entity: "column",
    doc: "Brings a new column onto a storey; its height is never stored, it is inferred from the authored top constraint.",
    displayName: "Create Column", binaryTag: 500,
    props: [id("column", "identity", { en: "Column id", de: "Stützen-Id" }, 10), recordProp("column", "Column", { en: "Column", de: "Stütze" })],
    label: { en: 'format!("Create column \\"{}\\"", self.column.name)', de: 'format!("Stütze \\"{}\\" anlegen", self.column.name)' },
    target,
    cases: [
      { name: "adds-to-the-storey-top", emoji: E.ok, before: withColumn(), mutation: { id: "c-b1", column: newColumn(F.storeyTop(0)) }, outcome: ok },
      { name: "spans-into-the-storey-above", emoji: E.spark, before: withColumn(), mutation: { id: "c-b1", column: newColumn(F.toStorey("st-first", 0.2)) }, outcome: ok },
      { name: "duplicate", emoji: E.no, before: withColumn(), mutation: { id: "c-a1", column: newColumn(F.storeyTop(0)) }, outcome: reject("mutation.duplicate-id", ["c-a1"]) },
      { name: "storey-missing", emoji: E.miss, before: withColumn(), mutation: { id: "c-b1", column: column("st-attic", "ct-400", [6, 0], F.storeyTop(0), "B1") }, outcome: reject("mutation.target-missing", ["column", "storey"]) },
      { name: "type-missing", emoji: E.magnet, before: withColumn(), mutation: { id: "c-b1", column: column("st-ground", "ct-missing", [6, 0], F.storeyTop(0), "B1") }, outcome: reject("mutation.target-missing", ["column", "column_type"]) },
      { name: "top-storey-missing", emoji: E.compass, before: withColumn(), mutation: { id: "c-b1", column: newColumn(F.toStorey("st-attic", 0)) }, outcome: reject("mutation.target-missing", ["column", "top", "storey"]) },
      { name: "top-storey-other-building", emoji: E.puzzle, before: twoBuildings(), mutation: { id: "c-b1", column: newColumn(F.toStorey("st-annex", 0)) }, outcome: reject("mutation.invariant", ["column", "top", "storey"]) },
      { name: "top-below-base", emoji: E.down, before: withColumn(), mutation: { id: "c-b1", column: newColumn(F.storeyTop(-3.5)) }, outcome: reject("mutation.invariant", ["column", "top"]) },
      { name: "zero-height", emoji: E.stop, before: withColumn(), mutation: { id: "c-b1", column: newColumn(F.unconnected(0)) }, outcome: reject("mutation.invariant", ["column", "top"]) },
    ],
  },
  {
    kind: "delete-column", emoji: 0x1faa6, variant: "DeleteColumn", verb: "delete", entity: "column", doc: "Removes a column; nothing is hosted by it.", displayName: "Delete Column", binaryTag: 501,
    props: [id("column", "target", { en: "Column", de: "Stütze" })],
    label: { en: 'format!("Delete column \\"{}\\"", self.id)', de: 'format!("Stütze \\"{}\\" löschen", self.id)' },
    target,
    cases: [
      { name: "removes", emoji: E.ok, before: withColumn(), mutation: { id: "c-a1" }, outcome: ok },
      { name: "missing", emoji: E.miss, before: withColumn(), mutation: { id: "c-gone" }, outcome: reject("mutation.target-missing", ["c-gone"]) },
    ],
  },
  {
    kind: "set-column", emoji: 0x1f39b, variant: "SetColumn", verb: "set", entity: "column",
    doc: "Edits a column sparsely: type, position, rotation, base offset, authored top constraint and name; absent fields stay untouched.",
    displayName: "Set Column", binaryTag: 502,
    uses: ["Point2", "TopConstraint"],
    props: [
      id("column", "target", { en: "Column", de: "Stütze" }),
      sparse(reference("column_type", "column-type", { en: "Column type", de: "Stützentyp" }, 20), keep),
      sparse(point("position", { en: "Position", de: "Position" }, 30), keep),
      sparse(angle("rotation", { en: "Rotation", de: "Drehung" }, 40), keep),
      sparse(length("base_offset", { en: "Base offset (m)", de: "Fußversatz (m)" }, 50), keep),
      sparse(topProp(60), keep),
      sparse(name(70), keep),
    ],
    label: { en: 'format!("Edit column \\"{}\\"", self.id)', de: 'format!("Stütze \\"{}\\" ändern", self.id)' },
    target,
    cases: [
      { name: "retypes-moves-and-renames", emoji: E.ok, before: withColumn(), mutation: { id: "c-a1", column_type: "ct-500", position: F.P(1.5, 0.5), rotation: 0.5, name: "A1 heavy" }, outcome: ok },
      { name: "constrains-the-top", emoji: E.spark, before: withColumn(), mutation: { id: "c-a1", top: F.toStorey("st-first", 0.2) }, outcome: ok },
      { name: "keeps-equal-fields-out-of-the-diff", emoji: E.scales, before: withColumn(), mutation: { id: "c-a1", column_type: "ct-400", base_offset: 0.1 }, outcome: ok },
      { name: "missing", emoji: E.miss, before: withColumn(), mutation: { id: "c-gone", name: "Gone" }, outcome: reject("mutation.target-missing", ["c-gone"]) },
      { name: "type-missing", emoji: E.magnet, before: withColumn(), mutation: { id: "c-a1", column_type: "ct-missing" }, outcome: reject("mutation.target-missing", ["column_type"]) },
      { name: "top-storey-missing", emoji: E.compass, before: withColumn(), mutation: { id: "c-a1", top: F.toStorey("st-attic", 0) }, outcome: reject("mutation.target-missing", ["top", "storey"]) },
      { name: "top-storey-other-building", emoji: E.puzzle, before: twoBuildings(), mutation: { id: "c-a1", top: F.toStorey("st-annex", 0) }, outcome: reject("mutation.invariant", ["top", "storey"]) },
      { name: "top-below-base", emoji: E.down, before: withColumn(), mutation: { id: "c-a1", top: F.storeyTop(-3.5) }, outcome: reject("mutation.invariant", ["top"]) },
      { name: "base-above-top", emoji: E.stop, before: withColumn(), mutation: { id: "c-a1", base_offset: 3.5 }, outcome: reject("mutation.invariant", ["base_offset"]) },
      { name: "nothing-to-change", emoji: E.warn, before: withColumn(), mutation: { id: "c-a1", column_type: "ct-400", name: "A1" }, outcome: reject("mutation.no-op", ["c-a1"]) },
    ],
  },
  {
    kind: "create-beam", emoji: 0x2796, variant: "CreateBeam", verb: "create", entity: "beam",
    doc: "Brings a new beam onto a storey; it hangs below the storey top by its authored offset, no elevation is stored.",
    displayName: "Create Beam", binaryTag: 503,
    props: [id("beam", "identity", { en: "Beam id", de: "Träger-Id" }, 10), recordProp("beam", "Beam", { en: "Beam", de: "Träger" })],
    label: { en: 'format!("Create beam \\"{}\\"", self.beam.name)', de: 'format!("Träger \\"{}\\" anlegen", self.beam.name)' },
    target,
    cases: [
      { name: "adds-below-the-storey-top", emoji: E.ok, before: withBeam(), mutation: { id: "b-b", beam: beam("st-ground", "bt-40x60", [0, 3], [4, 3], -0.2, "Beam B") }, outcome: ok },
      { name: "duplicate", emoji: E.no, before: withBeam(), mutation: { id: "b-a", beam: beam("st-ground", "bt-30x50", [0, 3], [4, 3], -0.2, "Beam B") }, outcome: reject("mutation.duplicate-id", ["b-a"]) },
      { name: "storey-missing", emoji: E.miss, before: withBeam(), mutation: { id: "b-b", beam: beam("st-attic", "bt-30x50", [0, 3], [4, 3], -0.2, "Beam B") }, outcome: reject("mutation.target-missing", ["beam", "storey"]) },
      { name: "type-missing", emoji: E.magnet, before: withBeam(), mutation: { id: "b-b", beam: beam("st-ground", "bt-missing", [0, 3], [4, 3], -0.2, "Beam B") }, outcome: reject("mutation.target-missing", ["beam", "beam_type"]) },
      { name: "zero-length", emoji: E.stop, before: withBeam(), mutation: { id: "b-b", beam: beam("st-ground", "bt-30x50", [2, 3], [2, 3], -0.2, "Beam B") }, outcome: reject("mutation.invariant", ["beam", "end"]) },
    ],
  },
  {
    kind: "delete-beam", emoji: 0x2702, variant: "DeleteBeam", verb: "delete", entity: "beam", doc: "Removes a beam; nothing is hosted by it.", displayName: "Delete Beam", binaryTag: 504,
    props: [id("beam", "target", { en: "Beam", de: "Träger" })],
    label: { en: 'format!("Delete beam \\"{}\\"", self.id)', de: 'format!("Träger \\"{}\\" löschen", self.id)' },
    target,
    cases: [
      { name: "removes", emoji: E.ok, before: withBeam(), mutation: { id: "b-a" }, outcome: ok },
      { name: "missing", emoji: E.miss, before: withBeam(), mutation: { id: "b-gone" }, outcome: reject("mutation.target-missing", ["b-gone"]) },
    ],
  },
  {
    kind: "set-beam", emoji: 0x1f4d0, variant: "SetBeam", verb: "set", entity: "beam",
    doc: "Edits a beam sparsely: type, start, end, top offset and name; absent fields stay untouched.",
    displayName: "Set Beam", binaryTag: 505,
    uses: ["Point2"],
    props: [
      id("beam", "target", { en: "Beam", de: "Träger" }),
      sparse(reference("beam_type", "beam-type", { en: "Beam type", de: "Trägertyp" }, 20), keep),
      sparse(point("start", { en: "Start", de: "Anfang" }, 30), keep),
      sparse(point("end", { en: "End", de: "Ende" }, 40), keep),
      sparse(length("top_offset", { en: "Top offset (m)", de: "Oberkantenversatz (m)" }, 50), keep),
      sparse(name(60), keep),
    ],
    label: { en: 'format!("Edit beam \\"{}\\"", self.id)', de: 'format!("Träger \\"{}\\" ändern", self.id)' },
    target,
    cases: [
      { name: "retypes-and-stretches", emoji: E.ok, before: withBeam(), mutation: { id: "b-a", beam_type: "bt-40x60", end: F.P(6, 0), name: "Beam A long" }, outcome: ok },
      { name: "lowers-the-beam", emoji: E.spark, before: withBeam(), mutation: { id: "b-a", top_offset: -0.3 }, outcome: ok },
      { name: "keeps-equal-fields-out-of-the-diff", emoji: E.scales, before: withBeam(), mutation: { id: "b-a", beam_type: "bt-30x50", top_offset: -0.25 }, outcome: ok },
      { name: "missing", emoji: E.miss, before: withBeam(), mutation: { id: "b-gone", name: "Gone" }, outcome: reject("mutation.target-missing", ["b-gone"]) },
      { name: "type-missing", emoji: E.magnet, before: withBeam(), mutation: { id: "b-a", beam_type: "bt-missing" }, outcome: reject("mutation.target-missing", ["beam_type"]) },
      { name: "zero-length", emoji: E.stop, before: withBeam(), mutation: { id: "b-a", end: F.P(0, 0) }, outcome: reject("mutation.invariant", ["end"]) },
      { name: "nothing-to-change", emoji: E.warn, before: withBeam(), mutation: { id: "b-a", name: "Beam A" }, outcome: reject("mutation.no-op", ["b-a"]) },
    ],
  },
];

const scratch = join(import.meta.dir, "🗑️generated", "m-frame");

function patch(leaf: MineLeaf) {
  const leafDir = join(mutations, em(leaf.emoji) + leaf.kind);
  const optional = leaf.props.filter((p) => p.optional);
  if (optional.length) {
    const file = join(leafDir, em(0x1f9a0) + "mutation", RS);
    let text = readFileSync(file, "utf8").replace(/^use crate::\{.*\};$/m, `use crate::{ModelDiff, ModelMutation, ModelSnapshot, ${(leaf.uses ?? []).sort().join(", ")}};`);
    for (const prop of optional) text = text.replace(`    pub ${prop.name}: Option<`, `    #[value(default, skip_serializing_if = "Option::is_none")]\n    pub ${prop.name}: Option<`);
    writeFileSync(file, text);
    const schemaFile = join(leafDir, em(0x1f9ec) + "schema", JSONF);
    const schema = JSON.parse(readFileSync(schemaFile, "utf8"));
    schema.required = schema.required.filter((n: string) => !optional.some((p) => p.name === n));
    writeFileSync(schemaFile, JSON.stringify(schema, null, 2) + "\n");
  }
  for (const [folder, source] of [[em(0x1f53a) + "diff", "diff"], [em(0x21a9) + "inverse", "inverse"]] as const) {
    const from = join(scratch, "src", `${leaf.kind}.${source}.rs`);
    if (existsSync(from)) copyFileSync(from, join(leafDir, folder, RS));
  }
}

const authoredTops: Record<string, { dir: string; first: string; source: (before: string) => string }> = {
  "create-column": {
    dir: "authored-top-is-stored",
    first: "adds-to-the-storey-top",
    source: (before) => `//! ${em(0x1f4cc)} \`create-column\` / \`authored-top-is-stored\`: the authored top constraint is stored verbatim, no derived height is stored with it, and a taller storey changes nothing stored.

use crate::standards::v1::subsets::any::schema::mutations::apply_model_mutation;
use crate::standards::v1::subsets::any::schema::mutations::create_column::CreateColumn;
use crate::standards::v1::subsets::any::schema::mutations::set_storey_height::SetStoreyHeight;
use crate::{Column, ModelMutation, ModelSnapshot, Point2, TopConstraint};
use semio_framework_pack_json::{from_json_str, to_json_string, JsonMemberPolicy};

const BEFORE: &str = include_str!("${before}");

#[semio_framework_async_macros::async_test]
async fn the_authored_top_is_stored_and_a_taller_storey_changes_nothing_stored() {
    let before: ModelSnapshot = from_json_str(BEFORE, JsonMemberPolicy::Reject).expect("before snapshot decodes");
    let created = Column { storey: "st-ground".into(), column_type: "ct-400".into(), position: Point2 { x: 6.0, y: 0.0 }, rotation: 0.0, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.25 }, name: "New".into() };
    let stored = apply_model_mutation(&before, &ModelMutation::CreateColumn(CreateColumn { id: "c-new".into(), column: created.clone() })).expect("the column is created");
    assert_eq!(stored.columns["c-new"].top, TopConstraint::StoreyTop { offset: 0.25 });
    let wire: serde_json::Value = serde_json::from_str(&to_json_string(&stored.columns["c-new"])).expect("the column encodes as JSON");
    let mut keys: Vec<&str> = wire.as_object().expect("a column is an object").keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(keys, ["base_offset", "column_type", "name", "position", "rotation", "storey", "top"]);
    let taller = apply_model_mutation(&stored, &ModelMutation::SetStoreyHeight(SetStoreyHeight { id: "st-ground".into(), height: 3.5 })).expect("the storey grows");
    assert_eq!(taller.columns["c-new"], created);
}
`,
  },
  "create-beam": {
    dir: "authored-top-offset-is-stored",
    first: "adds-below-the-storey-top",
    source: (before) => `//! ${em(0x1f4cc)} \`create-beam\` / \`authored-top-offset-is-stored\`: the authored top offset is stored verbatim, no derived elevation is stored with it, and a taller storey changes nothing stored.

use crate::standards::v1::subsets::any::schema::mutations::apply_model_mutation;
use crate::standards::v1::subsets::any::schema::mutations::create_beam::CreateBeam;
use crate::standards::v1::subsets::any::schema::mutations::set_storey_height::SetStoreyHeight;
use crate::{Beam, ModelMutation, ModelSnapshot, Point2};
use semio_framework_pack_json::{from_json_str, to_json_string, JsonMemberPolicy};

const BEFORE: &str = include_str!("${before}");

#[semio_framework_async_macros::async_test]
async fn the_authored_top_offset_is_stored_and_a_taller_storey_changes_nothing_stored() {
    let before: ModelSnapshot = from_json_str(BEFORE, JsonMemberPolicy::Reject).expect("before snapshot decodes");
    let created = Beam { storey: "st-ground".into(), beam_type: "bt-30x50".into(), start: Point2 { x: 0.0, y: 3.0 }, end: Point2 { x: 4.0, y: 3.0 }, top_offset: -0.2, name: "New".into() };
    let stored = apply_model_mutation(&before, &ModelMutation::CreateBeam(CreateBeam { id: "b-new".into(), beam: created.clone() })).expect("the beam is created");
    assert_eq!(stored.beams["b-new"].top_offset, -0.2);
    let wire: serde_json::Value = serde_json::from_str(&to_json_string(&stored.beams["b-new"])).expect("the beam encodes as JSON");
    let mut keys: Vec<&str> = wire.as_object().expect("a beam is an object").keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(keys, ["beam_type", "end", "name", "start", "storey", "top_offset"]);
    let taller = apply_model_mutation(&stored, &ModelMutation::SetStoreyHeight(SetStoreyHeight { id: "st-ground".into(), height: 3.5 })).expect("the storey grows");
    assert_eq!(taller.beams["b-new"], created);
}
`,
  },
};

function withAuthoredTop(leaf: MineLeaf, mount: string): string {
  const spec = authoredTops[leaf.kind];
  if (!spec) return mount;
  const leafDirName = em(leaf.emoji) + leaf.kind;
  const dir = join(mutations, leafDirName, em(0x1f9ea) + "tests", em(0x1f4cc) + spec.dir);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, RS), spec.source(`../../../../../${em(0x1f9eb)}fixtures/${em(0x1f9ec)}mutations/${leafDirName}/${em(0x2705)}${spec.first}/${em(0x1f4f8)}snapshot/${em(0x2b05)}before/${JSONF}`));
  const lines = mount.split("\n");
  const closing = lines.length - 2;
  const pathLine = lines.find((line) => line.includes(em(0x1f9ea) + "tests"))!;
  const rooted = pathLine.slice(pathLine.indexOf('"') + 1, pathLine.lastIndexOf(em(0x1f9ea) + "tests"));
  const indent = pathLine.slice(0, pathLine.indexOf("#"));
  const added = [`${indent}#[cfg(test)]`, `${indent}#[path = "${rooted}${em(0x1f9ea)}tests/${em(0x1f4cc)}${spec.dir}/${RS}"]`, `${indent}mod tests_${spec.dir.replaceAll("-", "_")};`];
  return [...lines.slice(0, closing), ...added, ...lines.slice(closing)].join("\n");
}

const register = (mounts: Map<string, string>) => {
  const root = join(artifact, RS);
  const marker = "//#endregion 🔖️Leaves";
  let text = readFileSync(root, "utf8");
  const fresh = [...mounts].filter(([module]) => !text.includes(`pub mod ${module} {`));
  const at = text.indexOf(marker);
  const lineStart = text.lastIndexOf("\n", at) + 1;
  text = text.slice(0, lineStart) + fresh.map(([, block]) => block).join("") + text.slice(lineStart);
  if (fresh.length) writeFileSync(root, text);
  console.log(`registered ${fresh.length} of ${mounts.size} mount blocks`);
};

if (import.meta.main) {
  mkdirSync(scratch, { recursive: true });
  const mounts = new Map<string, string>();
  for (const leaf of leaves) {
    const mount = emitLeaf(leaf);
    patch(leaf);
    mounts.set(leaf.kind.replaceAll("-", "_"), withAuthoredTop(leaf, mount));
  }
  writeFileSync(join(scratch, "mounts.txt"), [...mounts.values()].join(""));
  if (process.argv.includes("register")) register(mounts);
  console.log(`emitted ${leaves.length} leaves`);
}

