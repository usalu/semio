#!/usr/bin/env bun
/**
 * 🧪️ Wave M slice 3 (`m-context`): the six leaves set-project-info, set-site, set-building, create-grid-line, delete-grid-line and
 * set-grid-line of `s.bim.model@1` (binary tags 300..305). `bun r3-m-context-leaves.ts` rewrites their boilerplate and fixtures
 * (never a blessed `after`/`diff`; the hand-written `🔺️diff`/`↩️inverse` files are not touched) and writes the mount text to
 * `🗑️generated/m-context/mounts.txt`. Bless with `BIM_BLESS=1 cargo test -p semio-s-artifact-bim-model --lib <kind>`.
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { em, fixtures, JSONF, mutations, RS } from "./r3-f1-paths.ts";

type Label = { en: string; de: string };
const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
const record = (def: string) => ({ $ref: `${ART}#/$defs/${def}` });
const id = (entity: string, role: "target" | "identity", label: Label, order = 10): Prop => ({
  name: "id",
  rust: "String",
  schema: { type: "string" },
  ui: role === "target" ? { widget: "reference", role, label, ref: { kind: entity }, group: "target", order } : { widget: "text", role: "identity", label, group: "identity", order },
});
const recordProp = (name: string, def: string, label: Label, order = 20): Prop => ({ name, rust: def, schema: record(def), ui: { widget: "record", role: "value", label, group: "value", order } });
const num = (name: string, label: Label, order: number, bounds: { minimum?: number; maximum?: number } = {}): Prop => ({ name, rust: "Option<f64>", schema: { type: "number", ...bounds }, ui: { widget: "number", role: "value", label, group: "value", order } });
const text = (name: string, label: Label, order: number): Prop => ({ name, rust: "Option<String>", schema: { type: "string" }, ui: { widget: "text", role: "value", label, group: "identity", order } });
const point = (name: string, label: Label, order: number): Prop => ({ name, rust: "Option<Point2>", schema: record("Point2"), ui: { widget: "record", role: "value", label, group: "value", order } });
const points = (name: string, label: Label, order: number): Prop => ({ name, rust: "Option<Vec<Point2>>", schema: { type: "array", items: record("Point2") }, ui: { widget: "list", role: "value", label, group: "value", order } });
const names = (name: string, label: Label, order: number): Prop => ({ name, rust: "Option<Vec<String>>", schema: { type: "array", items: { type: "string" } }, ui: { widget: "list", role: "value", label, group: "value", order } });

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";
const P = F.P;
const grid = (building: string, label: string, start: [number, number], end: [number, number]) => ({ building, label, start: P(...start), end: P(...end) });

const withProject = (project: Record<string, unknown>) => ({ ...F.scene(), project: { ...F.project("Model"), ...project } });
const withSite = (site: Record<string, unknown>) => {
  const snapshot: any = F.scene();
  snapshot.sites["site-1"] = { ...snapshot.sites["site-1"], ...site };
  return snapshot;
};
const withGrids = (grids: Record<string, unknown> = { "g-1": grid("bldg-1", "1", [0, 0], [10, 0]), "g-2": grid("bldg-1", "2", [0, 6], [10, 6]) }, extra: { buildings?: Record<string, unknown> } = {}) => {
  const snapshot: any = F.scene();
  snapshot.grids = grids;
  snapshot.buildings = { ...snapshot.buildings, ...extra.buildings };
  return snapshot;
};
const OUTLINE = [P(0, 0), P(30, 0), P(30, 20), P(0, 20)];

type Extra = { optional: string[]; imports: string[]; extraTests?: { name: string; emoji: number }[] };
const extras = new Map<string, Extra>();
const leaf = (spec: Leaf, extra: Extra): Leaf => (extras.set(spec.kind, extra), spec);

export const leaves: Leaf[] = [
  leaf(
    {
      kind: "set-project-info", emoji: 0x1f4c7, variant: "SetProjectInfo", verb: "set", entity: "project", binaryTag: 300, displayName: "Set Project Info",
      doc: "Patches the project metadata (name, description, author, organization, phase names): exactly the provided fields change, an empty patch is a no-op.",
      props: [
        text("name", { en: "Name", de: "Name" }, 10),
        text("description", { en: "Description", de: "Beschreibung" }, 20),
        text("author", { en: "Author", de: "Autor" }, 30),
        text("organization", { en: "Organization", de: "Organisation" }, 40),
        names("phase_names", { en: "Phase names", de: "Phasennamen" }, 50),
      ],
      label: { en: 'String::from("Update the project info")', de: 'String::from("Projektinformationen ändern")' },
      target: 'vec!["project".to_string()]',
      cases: [
        { name: "renames-the-project", emoji: 0x2705, before: F.scene(), mutation: { name: "Villa Aurora", author: "Ueli Saluz" }, outcome: ok },
        { name: "sets-the-phases", emoji: 0x1f9f2, before: withProject({ description: "Draft" }), mutation: { description: "", phase_names: ["Existing", "New"] }, outcome: ok },
        { name: "empty-patch", emoji: 0x1f6ab, before: F.scene(), mutation: {}, outcome: reject("mutation.no-op", ["project"]) },
        { name: "unchanged", emoji: 0x26d4, before: withProject({ author: "Ueli Saluz" }), mutation: { name: "Model", author: "Ueli Saluz" }, outcome: reject("mutation.no-op", ["project"]) },
        { name: "blank-phase", emoji: 0x1f4a5, before: F.scene(), mutation: { phase_names: ["New", " "] }, outcome: reject("mutation.invariant", ["phase_names"]) },
      ],
    },
    { optional: ["name", "description", "author", "organization", "phase_names"], imports: [] },
  ),
  leaf(
    {
      kind: "set-site", emoji: 0x1f5fa, variant: "SetSite", verb: "set", entity: "site", binaryTag: 301, displayName: "Set Site",
      doc: "Patches a site (name, latitude, longitude, elevation, true north, boundary loop): exactly the provided fields change. Changing the site elevation re-infers the absolute elevation of every storey of every building on it (parametric).",
      props: [
        id("site", "target", { en: "Site", de: "Standort" }),
        text("name", { en: "Name", de: "Name" }, 20),
        num("latitude", { en: "Latitude (deg)", de: "Breitengrad (°)" }, 30, { minimum: -90, maximum: 90 }),
        num("longitude", { en: "Longitude (deg)", de: "Längengrad (°)" }, 40, { minimum: -180, maximum: 180 }),
        num("elevation", { en: "Elevation (m)", de: "Höhe (m)" }, 50),
        num("true_north", { en: "True north (rad)", de: "Geographisch Nord (rad)" }, 60),
        points("boundary", { en: "Boundary", de: "Grundstücksgrenze" }, 70),
      ],
      label: { en: 'format!("Update site \\"{}\\"", self.id)', de: 'format!("Standort \\"{}\\" ändern", self.id)' },
      target,
      cases: [
        { name: "moves-the-site", emoji: 0x2705, before: F.scene(), mutation: { id: "site-1", latitude: 46.95, longitude: 7.45, elevation: 540 }, outcome: ok },
        { name: "outlines-the-boundary", emoji: 0x1f9f2, before: F.scene(), mutation: { id: "site-1", boundary: OUTLINE }, outcome: ok },
        { name: "clears-the-boundary", emoji: 0x1f9f5, before: withSite({ boundary: OUTLINE }), mutation: { id: "site-1", boundary: [] }, outcome: ok },
        { name: "latitude-out-of-range", emoji: 0x1f6ab, before: F.scene(), mutation: { id: "site-1", latitude: 91 }, outcome: reject("mutation.invariant", ["latitude"]) },
        { name: "longitude-out-of-range", emoji: 0x1f4a5, before: F.scene(), mutation: { id: "site-1", longitude: -181 }, outcome: reject("mutation.invariant", ["longitude"]) },
        { name: "open-boundary", emoji: 0x1f512, before: F.scene(), mutation: { id: "site-1", boundary: [P(0, 0), P(5, 0)] }, outcome: reject("mutation.invariant", ["boundary"]) },
        { name: "missing", emoji: 0x26d4, before: F.scene(), mutation: { id: "site-9", name: "Elsewhere" }, outcome: reject("mutation.target-missing", ["site-9"]) },
        { name: "unchanged", emoji: 0x1f501, before: F.scene(), mutation: { id: "site-1", name: "Plot", elevation: 100 }, outcome: reject("mutation.no-op", ["site-1"]) },
      ],
    },
    { optional: ["name", "latitude", "longitude", "elevation", "true_north", "boundary"], imports: ["Point2"], extraTests: [{ name: "raises-the-absolute-levels", emoji: 0x1fa9c }] },
  ),
  leaf(
    {
      kind: "set-building", emoji: 0x1f3d7, variant: "SetBuilding", verb: "set", entity: "building", binaryTag: 302, displayName: "Set Building",
      doc: "Patches a building (name, origin, rotation, elevation): exactly the provided fields change; the site it stands on never changes here. Changing the building elevation re-infers the absolute elevation of every storey of the building (parametric).",
      props: [
        id("building", "target", { en: "Building", de: "Gebäude" }),
        text("name", { en: "Name", de: "Name" }, 20),
        point("origin", { en: "Origin", de: "Ursprung" }, 30),
        num("rotation", { en: "Rotation (rad)", de: "Drehung (rad)" }, 40),
        num("elevation", { en: "Elevation (m)", de: "Höhe (m)" }, 50),
      ],
      label: { en: 'format!("Update building \\"{}\\"", self.id)', de: 'format!("Gebäude \\"{}\\" ändern", self.id)' },
      target,
      cases: [
        { name: "raises-the-datum", emoji: 0x2705, before: F.scene(), mutation: { id: "bldg-1", elevation: 1.5 }, outcome: ok },
        { name: "places-the-building", emoji: 0x1f9f2, before: F.scene(), mutation: { id: "bldg-1", name: "Villa", origin: P(12, -4), rotation: 0.5 }, outcome: ok },
        { name: "missing", emoji: 0x26d4, before: F.scene(), mutation: { id: "bldg-9", elevation: 1 }, outcome: reject("mutation.target-missing", ["bldg-9"]) },
        { name: "unchanged", emoji: 0x1f6ab, before: F.scene(), mutation: { id: "bldg-1", name: "House", rotation: 0 }, outcome: reject("mutation.no-op", ["bldg-1"]) },
        { name: "empty-patch", emoji: 0x1f501, before: F.scene(), mutation: { id: "bldg-1" }, outcome: reject("mutation.no-op", ["bldg-1"]) },
      ],
    },
    { optional: ["name", "origin", "rotation", "elevation"], imports: ["Point2"], extraTests: [{ name: "raises-the-absolute-levels", emoji: 0x1fa9c }] },
  ),
  leaf(
    {
      kind: "create-grid-line", emoji: 0x1f532, variant: "CreateGridLine", verb: "create", entity: "grid-line", binaryTag: 303, displayName: "Create Grid Line",
      doc: "Brings a new labelled grid line into a building; its label is unique within the building and it has length.",
      props: [id("grid-line", "identity", { en: "Grid line id", de: "Rasterlinien-Id" }, 10), recordProp("grid_line", "GridLine", { en: "Grid line", de: "Rasterlinie" })],
      label: { en: 'format!("Create grid line \\"{}\\"", self.grid_line.label)', de: 'format!("Rasterlinie \\"{}\\" anlegen", self.grid_line.label)' },
      target,
      cases: [
        { name: "adds-axis-a", emoji: 0x2705, before: withGrids(), mutation: { id: "g-a", grid_line: grid("bldg-1", "A", [0, 0], [0, 6]) }, outcome: ok },
        { name: "same-label-other-building", emoji: 0x1f9f2, before: withGrids({ "g-1": grid("bldg-1", "1", [0, 0], [10, 0]) }, { buildings: { "bldg-2": F.building("site-1", "Annex") } }), mutation: { id: "g-x", grid_line: grid("bldg-2", "1", [0, 0], [8, 0]) }, outcome: ok },
        { name: "duplicate-id", emoji: 0x1f6ab, before: withGrids(), mutation: { id: "g-1", grid_line: grid("bldg-1", "A", [0, 0], [0, 6]) }, outcome: reject("mutation.duplicate-id", ["g-1"]) },
        { name: "building-missing", emoji: 0x26d4, before: withGrids(), mutation: { id: "g-a", grid_line: grid("bldg-9", "A", [0, 0], [0, 6]) }, outcome: reject("mutation.target-missing", ["grid_line", "building"]) },
        { name: "zero-length", emoji: 0x1f4a5, before: withGrids(), mutation: { id: "g-a", grid_line: grid("bldg-1", "A", [2, 2], [2, 2]) }, outcome: reject("mutation.invariant", ["grid_line", "end"]) },
        { name: "label-taken", emoji: 0x1f512, before: withGrids(), mutation: { id: "g-a", grid_line: grid("bldg-1", "2", [0, 0], [0, 6]) }, outcome: reject("mutation.invariant", ["grid_line", "label"]) },
        { name: "blank-label", emoji: 0x1f501, before: withGrids(), mutation: { id: "g-a", grid_line: grid("bldg-1", "  ", [0, 0], [0, 6]) }, outcome: reject("mutation.invariant", ["grid_line", "label"]) },
      ],
    },
    { optional: [], imports: ["GridLine"] },
  ),
  leaf(
    {
      kind: "delete-grid-line", emoji: 0x1f533, variant: "DeleteGridLine", verb: "delete", entity: "grid-line", binaryTag: 304, displayName: "Delete Grid Line",
      doc: "Removes a grid line; nothing references a grid line, so nothing blocks or cascades.",
      props: [id("grid-line", "target", { en: "Grid line", de: "Rasterlinie" })],
      label: { en: 'format!("Delete grid line \\"{}\\"", self.id)', de: 'format!("Rasterlinie \\"{}\\" löschen", self.id)' },
      target,
      cases: [
        { name: "removes", emoji: 0x2705, before: withGrids(), mutation: { id: "g-1" }, outcome: ok },
        { name: "missing", emoji: 0x26d4, before: withGrids(), mutation: { id: "g-9" }, outcome: reject("mutation.target-missing", ["g-9"]) },
      ],
    },
    { optional: [], imports: [] },
  ),
  leaf(
    {
      kind: "set-grid-line", emoji: 0x1f9ed, variant: "SetGridLine", verb: "set", entity: "grid-line", binaryTag: 305, displayName: "Set Grid Line",
      doc: "Patches a grid line (label, start, end): exactly the provided fields change; the building it belongs to never changes here.",
      props: [
        id("grid-line", "target", { en: "Grid line", de: "Rasterlinie" }),
        text("label", { en: "Label", de: "Bezeichnung" }, 20),
        point("start", { en: "Start", de: "Start" }, 30),
        point("end", { en: "End", de: "Ende" }, 40),
      ],
      label: { en: 'format!("Update grid line \\"{}\\"", self.id)', de: 'format!("Rasterlinie \\"{}\\" ändern", self.id)' },
      target,
      cases: [
        { name: "relabels", emoji: 0x2705, before: withGrids(), mutation: { id: "g-1", label: "A" }, outcome: ok },
        { name: "moves-the-end", emoji: 0x1f9f2, before: withGrids(), mutation: { id: "g-1", end: P(12, 0.5) }, outcome: ok },
        { name: "missing", emoji: 0x26d4, before: withGrids(), mutation: { id: "g-9", label: "A" }, outcome: reject("mutation.target-missing", ["g-9"]) },
        { name: "label-taken", emoji: 0x1f6ab, before: withGrids(), mutation: { id: "g-1", label: "2" }, outcome: reject("mutation.invariant", ["label"]) },
        { name: "zero-length", emoji: 0x1f4a5, before: withGrids(), mutation: { id: "g-1", start: P(10, 0) }, outcome: reject("mutation.invariant", ["start"]) },
        { name: "unchanged", emoji: 0x1f501, before: withGrids(), mutation: { id: "g-1", label: "1", end: P(10, 0) }, outcome: reject("mutation.no-op", ["g-1"]) },
      ],
    },
    { optional: ["label", "start", "end"], imports: ["Point2"] },
  ),
];

const dir = (spec: Leaf) => join(mutations, em(spec.emoji) + spec.kind);
const sub = (spec: Leaf, emoji: number, name: string) => join(dir(spec), em(emoji) + name);

const fixup = (spec: Leaf) => {
  const extra = extras.get(spec.kind)!;
  const component = join(sub(spec, 0x1f9a0, "mutation"), RS);
  let source = readFileSync(component, "utf8");
  const imports = ["ModelDiff", "ModelMutation", "ModelSnapshot", ...spec.props.map((prop) => prop.rust).filter((name) => /^[A-Z][A-Za-z0-9]*$/.test(name) && name !== "String"), ...extra.imports];
  source = source.replace(/use crate::\{[^}]*\};/, `use crate::{${[...new Set(imports)].sort().join(", ")}};`);
  for (const name of extra.optional) source = source.replace(new RegExp(`^    pub ${name}: Option<`, "m"), `    #[value(default, skip_serializing_if = "Option::is_none")]\n    pub ${name}: Option<`);
  writeFileSync(component, source);
  const schemaFile = join(sub(spec, 0x1f9ec, "schema"), JSONF);
  const schema = JSON.parse(readFileSync(schemaFile, "utf8"));
  schema.required = schema.required.filter((name: string) => !extra.optional.includes(name));
  writeFileSync(schemaFile, JSON.stringify(schema, null, 2) + "\n");
};

const snake = (kind: string) => kind.replaceAll("-", "_");

const INFERENCE: Record<string, { case: string; collection: string; id: string; what: string }> = {
  "set-building": { case: "raises-the-datum", collection: "buildings", id: "bldg-1", what: "building" },
  "set-site": { case: "moves-the-site", collection: "sites", id: "site-1", what: "site" },
};

const inferenceTest = (spec: Leaf, test: { name: string; emoji: number }) => {
  const info = INFERENCE[spec.kind];
  const leafDirName = em(spec.emoji) + spec.kind;
  const caseDirName = em(0x2705) + info.case;
  const base = `../../../../../${fixtures.split(/[\\/]/).pop()}/${em(0x1f9ec)}mutations/${leafDirName}/${caseDirName}`;
  const include = (tail: string) => `include_str!("${base}/${tail}")`;
  return `//! ${em(test.emoji)} \`${spec.kind}\` / \`${test.name}\`: inference level. The ${info.what} elevation is a datum of the parametric chain: changing it moves the
//! absolute elevation of every storey it carries by the same amount (\`storey-levels\`) and leaves the building-relative elevations alone.

use crate::standards::v1::subsets::any::schema::inferences::storey_levels::compute_storey_levels;
use crate::standards::v1::subsets::any::schema::mutations::kit::{self, Case};
use protocol::Mutation;

const CASE: Case = Case {
    dir: "${leafDirName}/${caseDirName}",
    before: ${include(`${em(0x1f4f8)}snapshot/${em(0x2b05)}before/${JSONF}`)},
    after: ${include(`${em(0x1f4f8)}snapshot/${em(0x27a1)}after/${JSONF}`)},
    mutation: ${include(`${em(0x1f9a0)}mutation/${JSONF}`)},
    diff: ${include(`${em(0x1f53a)}diff/${JSONF}`)},
    outcome: ${include(`${em(0x1f3af)}outcome/${JSONF}`)},
};

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-9 * right.abs().max(1.0)
}

#[semio_framework_async_macros::async_test]
async fn the_${info.what}_elevation_moves_every_absolute_storey_elevation() {
    let before = kit::before(&CASE);
    let (diff, _) = kit::mutation(&CASE).diff(&before).into_parts();
    let after = protocol::apply_diff(&diff, &before).expect("the diff applies");
    let raised = after.${info.collection}["${info.id}"].elevation - before.${info.collection}["${info.id}"].elevation;
    assert!(raised.abs() > 0.0, "the case changes the ${info.what} elevation");
    let (low, high) = (compute_storey_levels(&before), compute_storey_levels(&after));
    assert!(!low.is_empty() && low.len() == high.len(), "the scene carries storeys");
    for (id, level) in &low {
        let next = high[id];
        assert!(close(next.absolute_elevation - level.absolute_elevation, raised), "{id}: absolute elevation follows the ${info.what} elevation");
        assert!(close(next.absolute_top_elevation - level.absolute_top_elevation, raised), "{id}: absolute top elevation follows the ${info.what} elevation");
        assert_eq!(next.elevation, level.elevation, "{id}: the building-relative elevation is untouched");
        assert_eq!(next.top_elevation, level.top_elevation, "{id}: the building-relative top elevation is untouched");
    }
}
`;
};

if (import.meta.main) {
  const out = join(import.meta.dir, "🗑️generated", "m-context");
  mkdirSync(out, { recursive: true });
  let mounts = "";
  for (const spec of leaves) {
    let text = emitLeaf(spec);
    fixup(spec);
    for (const test of extras.get(spec.kind)!.extraTests ?? []) {
      const testDir = join(sub(spec, 0x1f9ea, "tests"), em(test.emoji) + test.name);
      mkdirSync(testDir, { recursive: true });
      writeFileSync(join(testDir, RS), inferenceTest(spec, test));
      const marker = "                        }\n";
      const parts = text.match(/#\[path = "([^"]*)"\]\n\s*mod tests_/)![1].split("/");
      parts.splice(-2, 2, em(test.emoji) + test.name, RS);
      const relative = parts.join("/");
      text = text.slice(0, text.length - marker.length) + `                            #[cfg(test)]\n                            #[path = "${relative}"]\n                            mod tests_${snake(test.name)};\n` + marker;
    }
    mounts += text;
  }
  writeFileSync(join(out, "mounts.txt"), mounts);
  console.log(`emitted ${leaves.length} leaves`);
}
