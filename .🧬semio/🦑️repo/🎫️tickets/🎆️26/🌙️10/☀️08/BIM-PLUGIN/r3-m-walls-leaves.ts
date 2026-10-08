#!/usr/bin/env bun
/**
 * 🧱️ Wave M slice 4 (`m-walls`, binary tags 400 to 499): the wall leaves of `s.bim.model@1`. `bun r3-m-walls-leaves.ts` rewrites their
 * boilerplate and fixtures (never the hand-written `🔺️diff`/`↩️inverse`, never a blessed `after`/`diff`), post-processes the sparse payload of
 * `set-curtain-wall`, and writes the mount text to `🗑️generated/m-walls/mounts.txt`. With `--register` it also inserts the variants, kinds and
 * mount blocks into the aggregate and the artifact root surgically and idempotently (run it only once the hand-written Rust files exist).
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { artifact, em, JSONF, mutations, RS, rel } from "./r3-f1-paths.ts";

type Label = { en: string; de: string };
const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
const record = (def: string) => ({ $ref: `${ART}#/$defs/${def}` });
const target = "vec![self.id.clone()]";

const ref = (name: string, kind: string, role: "target" | "value", label: Label, order = 10): Prop => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "reference", role, label, ref: { kind }, group: role === "target" ? "target" : "value", order } });
const identity = (name: string, label: Label, order = 10): Prop => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "text", role: "identity", label, group: "identity", order } });
const recordProp = (name: string, def: string, label: Label, order = 20, rust = def): Prop => ({ name, rust, schema: record(def), ui: { widget: "record", role: "value", label, group: "value", order } });
const num = (name: string, label: Label, order = 20, rust = "f64"): Prop => ({ name, rust, schema: { type: "number" }, ui: { widget: "number", role: "value", label, group: "value", order } });
const text = (name: string, label: Label, order = 20, rust = "String"): Prop => ({ name, rust, schema: { type: "string" }, ui: { widget: "text", role: "value", label, group: "identity", order } });
const choice = (name: string, def: string, label: Label, order = 20): Prop => ({ name, rust: def, schema: record(def), ui: { widget: "select", role: "value", label, group: "value", order } });
const wallId = ref("id", "wall", "target", { en: "Wall", de: "Wand" });
const curtainId = ref("id", "curtain-wall", "target", { en: "Curtain wall", de: "Vorhangfassade" });

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const missing = (id: string) => reject("mutation.target-missing", [id]);

const P = F.P;
const arc = (a: [number, number], b: [number, number], bulge: number) => ({ Arc: { start: P(...a), end: P(...b), bulge } });
const line = F.line;
const rect = { Rectangle: { width: 0.05, depth: 0.15 } };
const hosted = (host: string, name: string, offset: number) => ({ ...F.opening(host, name), offset });
const curtain = (axis: unknown, name: string, over: Record<string, unknown> = {}) => ({ storey: "st-ground", axis, base_offset: 0, top: F.storeyTop(0), u_spacing: 1.5, v_spacing: 1.2, mullion: rect, panel_material: "m-glass", mullion_material: "m-alu", name, ...over });

const scene = F.scene;
const withArc = () => scene({ walls: { "w-arc": F.wall("st-ground", "wt-300", arc([0, 6], [8, 6], 1), F.storeyTop(0), "Arc") } });
const withThinType = () => ({ ...scene(), wall_types: { ...scene().wall_types, "wt-150": F.wallType("Brick 150", [F.layer("m-brick", 0.15)]) } });
const withFacade = (openings: Record<string, unknown> = {}) => {
  const base = scene();
  return { ...base, materials: { ...base.materials, "m-glass": { ...F.material("Glass"), category: "Glass" }, "m-alu": { ...F.material("Aluminium"), category: "Metal" } }, curtain_walls: { "cw-1": curtain(line([0, 8], [8, 8]), "Facade") }, openings };
};
const withOpenings = () => scene({ openings: { "o-1": hosted("w-south", "Door", 1), "o-2": hosted("w-south", "Window", 5), "o-3": hosted("w-south", "Window", 7), "o-4": hosted("w-east", "Window", 2) } });

const leaves: Leaf[] = [
  {
    kind: "set-wall-axis", emoji: 0x3030, variant: "SetWallAxis", verb: "set", entity: "wall", displayName: "Set Wall Axis", binaryTag: 400,
    doc: "Replaces a wall's axis, a line or an arc by bulge; this is how a wall is moved, stretched, curved or straightened. Hosted openings are untouched, their placement is inferred.",
    props: [wallId, recordProp("axis", "Axis", { en: "Axis", de: "Achse" })],
    label: { en: 'format!("Reshape the axis of wall \\"{}\\"", self.id)', de: 'format!("Achse von Wand \\"{}\\" ändern", self.id)' },
    target,
    cases: [
      { name: "stretches-under-an-opening", emoji: 0x2705, before: scene({ openings: { "o-1": hosted("w-south", "Door", 1) } }), mutation: { id: "w-south", axis: line([0, 0], [10, 0]) }, outcome: ok },
      { name: "curves-the-wall", emoji: 0x1f300, before: scene(), mutation: { id: "w-east", axis: arc([8, 0], [8, 6], 0.4) }, outcome: ok },
      { name: "straightens-the-arc", emoji: 0x1f9f5, before: withArc(), mutation: { id: "w-arc", axis: line([0, 6], [8, 6]) }, outcome: ok },
      { name: "unchanged", emoji: 0x1f9f2, before: scene(), mutation: { id: "w-south", axis: line([0, 0], [8, 0]) }, outcome: reject("mutation.no-op", ["w-south"]) },
      { name: "zero-length", emoji: 0x1f6ab, before: scene(), mutation: { id: "w-south", axis: line([2, 2], [2, 2]) }, outcome: reject("mutation.invariant", ["axis"]) },
      { name: "flat-arc", emoji: 0x26d4, before: scene(), mutation: { id: "w-south", axis: arc([0, 0], [8, 0], 0) }, outcome: reject("mutation.invariant", ["axis", "bulge"]) },
      { name: "missing", emoji: 0x1f573, before: scene(), mutation: { id: "w-attic", axis: line([0, 0], [8, 0]) }, outcome: missing("w-attic") },
    ],
  },
  {
    kind: "set-wall-base-offset", emoji: 0x1f53d, variant: "SetWallBaseOffset", verb: "set", entity: "wall", displayName: "Set Wall Base Offset", binaryTag: 401,
    doc: "Sets the distance of a wall's base above its storey's floor in metres; the resolved base, top and height follow by inference.",
    props: [wallId, num("base_offset", { en: "Base offset (m)", de: "Fußversatz (m)" })],
    label: { en: 'format!("Set wall base offset to {} m", self.base_offset)', de: 'format!("Wandfußversatz auf {} m setzen", self.base_offset)' },
    target,
    cases: [
      { name: "raises-the-base", emoji: 0x2705, before: scene(), mutation: { id: "w-south", base_offset: 0.15 }, outcome: ok },
      { name: "sinks-below-the-floor", emoji: 0x1f31f, before: scene(), mutation: { id: "w-east", base_offset: -0.3 }, outcome: ok },
      { name: "unchanged", emoji: 0x1f9f2, before: scene(), mutation: { id: "w-south", base_offset: 0 }, outcome: reject("mutation.no-op", ["w-south"]) },
      { name: "missing", emoji: 0x1f573, before: scene(), mutation: { id: "w-attic", base_offset: 0.1 }, outcome: missing("w-attic") },
    ],
  },
  {
    kind: "set-wall-type-of", emoji: 0x1f95e, variant: "SetWallTypeOf", verb: "set", entity: "wall", displayName: "Set Wall Type", binaryTag: 402,
    doc: "Builds a wall as another wall type; its thickness and layers follow by inference.",
    props: [wallId, ref("wall_type", "wall-type", "value", { en: "Wall type", de: "Wandtyp" })],
    label: { en: 'format!("Set type of wall \\"{}\\" to \\"{}\\"", self.id, self.wall_type)', de: 'format!("Typ von Wand \\"{}\\" auf \\"{}\\" setzen", self.id, self.wall_type)' },
    target,
    cases: [
      { name: "retypes-the-wall", emoji: 0x2705, before: withThinType(), mutation: { id: "w-east", wall_type: "wt-150" }, outcome: ok },
      { name: "unchanged", emoji: 0x1f9f2, before: withThinType(), mutation: { id: "w-south", wall_type: "wt-300" }, outcome: reject("mutation.no-op", ["w-south"]) },
      { name: "type-missing", emoji: 0x1f6ab, before: withThinType(), mutation: { id: "w-east", wall_type: "wt-missing" }, outcome: reject("mutation.target-missing", ["wall_type"]) },
      { name: "missing", emoji: 0x1f573, before: withThinType(), mutation: { id: "w-attic", wall_type: "wt-150" }, outcome: missing("w-attic") },
    ],
  },
  {
    kind: "set-wall-location", emoji: 0x1f99a, variant: "SetWallLocation", verb: "set", entity: "wall", displayName: "Set Wall Location Line", binaryTag: 403,
    doc: "Chooses which face or line of the wall its axis denotes: centre, interior, exterior or core centre.",
    props: [wallId, choice("location", "LocationLine", { en: "Location line", de: "Bezugslinie" })],
    label: { en: 'format!("Locate wall \\"{}\\" on its {:?} line", self.id, self.location)', de: 'format!("Wand \\"{}\\" auf Bezugslinie {:?} legen", self.id, self.location)' },
    target,
    cases: [
      { name: "moves-to-the-exterior-face", emoji: 0x2705, before: scene(), mutation: { id: "w-south", location: "Exterior" }, outcome: ok },
      { name: "moves-to-the-core-centre", emoji: 0x1f31f, before: scene(), mutation: { id: "w-east", location: "CoreCenter" }, outcome: ok },
      { name: "unchanged", emoji: 0x1f9f2, before: scene(), mutation: { id: "w-south", location: "Center" }, outcome: reject("mutation.no-op", ["w-south"]) },
      { name: "missing", emoji: 0x1f573, before: scene(), mutation: { id: "w-attic", location: "Interior" }, outcome: missing("w-attic") },
    ],
  },
  {
    kind: "flip-wall", emoji: 0x1fa9e, variant: "FlipWall", verb: "toggle", entity: "wall", displayName: "Flip Wall", binaryTag: 404,
    doc: "Reverses the orientation of a wall: its axis swaps start and end and the bulge changes sign, so the same curve runs the other way.",
    props: [wallId],
    label: { en: 'format!("Flip wall \\"{}\\"", self.id)', de: 'format!("Wand \\"{}\\" wenden", self.id)' },
    target,
    cases: [
      { name: "reverses-a-line", emoji: 0x2705, before: scene(), mutation: { id: "w-south" }, outcome: ok },
      { name: "reverses-an-arc", emoji: 0x1f300, before: withArc(), mutation: { id: "w-arc" }, outcome: ok },
      { name: "pointlike-wall", emoji: 0x1f9f2, before: scene({ walls: { "w-point": F.wall("st-ground", "wt-300", line([1, 1], [1, 1]), F.storeyTop(0), "Point") } }), mutation: { id: "w-point" }, outcome: reject("mutation.no-op", ["w-point"]) },
      { name: "missing", emoji: 0x1f573, before: scene(), mutation: { id: "w-attic" }, outcome: missing("w-attic") },
    ],
  },
  {
    kind: "split-wall", emoji: 0x1f988, variant: "SplitWall", verb: "split", entity: "wall", displayName: "Split Wall", binaryTag: 405,
    doc: "Splits a wall at the fraction `t` of its axis into the original wall and a new wall that continues it with the same type, location, base, top and phase; openings beyond the split point move onto the new wall.",
    props: [wallId, num("t", { en: "Split position (0 to 1)", de: "Teilungsstelle (0 bis 1)" }), identity("new_id", { en: "New wall id", de: "Id der neuen Wand" }, 30)],
    inverseRows: { bounded: 4096 },
    label: { en: 'format!("Split wall \\"{}\\" at {}", self.id, self.t)', de: 'format!("Wand \\"{}\\" bei {} teilen", self.id, self.t)' },
    target,
    cases: [
      { name: "splits-a-straight-wall", emoji: 0x2705, before: scene(), mutation: { id: "w-south", t: 0.25, new_id: "w-south-2" }, outcome: ok },
      { name: "splits-an-arc", emoji: 0x1f300, before: withArc(), mutation: { id: "w-arc", t: 0.5, new_id: "w-arc-2" }, outcome: ok },
      { name: "hands-openings-over", emoji: 0x1fa9d, before: withOpenings(), mutation: { id: "w-south", t: 0.5, new_id: "w-south-2" }, outcome: ok },
      { name: "at-the-start", emoji: 0x1f6ab, before: scene(), mutation: { id: "w-south", t: 0, new_id: "w-south-2" }, outcome: reject("mutation.invariant", ["t"]) },
      { name: "beyond-the-end", emoji: 0x26d4, before: scene(), mutation: { id: "w-south", t: 1.5, new_id: "w-south-2" }, outcome: reject("mutation.invariant", ["t"]) },
      { name: "new-id-taken", emoji: 0x1f9e9, before: scene(), mutation: { id: "w-south", t: 0.5, new_id: "w-east" }, outcome: reject("mutation.duplicate-id", ["w-east"]) },
      { name: "missing", emoji: 0x1f573, before: scene(), mutation: { id: "w-attic", t: 0.5, new_id: "w-attic-2" }, outcome: missing("w-attic") },
    ],
  },
  {
    kind: "create-curtain-wall", emoji: 0x1f3ec, variant: "CreateCurtainWall", verb: "create", entity: "curtain-wall", displayName: "Create Curtain Wall", binaryTag: 406,
    doc: "Brings a new curtain wall onto a storey; its height is never stored, it is inferred from the top constraint.",
    props: [identity("id", { en: "Curtain wall id", de: "Vorhangfassaden-Id" }), recordProp("curtain_wall", "CurtainWall", { en: "Curtain wall", de: "Vorhangfassade" })],
    label: { en: 'format!("Create curtain wall \\"{}\\"", self.curtain_wall.name)', de: 'format!("Vorhangfassade \\"{}\\" anlegen", self.curtain_wall.name)' },
    target,
    cases: [
      { name: "adds-a-facade", emoji: 0x2705, before: { ...withFacade(), curtain_walls: {} }, mutation: { id: "cw-1", curtain_wall: curtain(line([0, 8], [8, 8]), "Facade") }, outcome: ok },
      { name: "adds-a-curved-facade", emoji: 0x1f300, before: { ...withFacade(), curtain_walls: {} }, mutation: { id: "cw-2", curtain_wall: curtain(arc([0, 8], [8, 8], 0.3), "Bow", { top: F.toStorey("st-first", 0) }) }, outcome: ok },
      { name: "duplicate", emoji: 0x1f6ab, before: withFacade(), mutation: { id: "cw-1", curtain_wall: curtain(line([0, 9], [8, 9]), "Other") }, outcome: reject("mutation.duplicate-id", ["cw-1"]) },
      { name: "storey-missing", emoji: 0x26d4, before: { ...withFacade(), curtain_walls: {} }, mutation: { id: "cw-1", curtain_wall: curtain(line([0, 8], [8, 8]), "Facade", { storey: "st-attic" }) }, outcome: reject("mutation.target-missing", ["curtain_wall", "storey"]) },
      { name: "zero-length", emoji: 0x1f4a5, before: { ...withFacade(), curtain_walls: {} }, mutation: { id: "cw-1", curtain_wall: curtain(line([3, 8], [3, 8]), "Facade") }, outcome: reject("mutation.invariant", ["curtain_wall", "axis"]) },
      { name: "spacing-non-positive", emoji: 0x1f4cf, before: { ...withFacade(), curtain_walls: {} }, mutation: { id: "cw-1", curtain_wall: curtain(line([0, 8], [8, 8]), "Facade", { u_spacing: 0 }) }, outcome: reject("mutation.invariant", ["curtain_wall", "u_spacing"]) },
      { name: "material-missing", emoji: 0x1f9ea, before: { ...withFacade(), curtain_walls: {} }, mutation: { id: "cw-1", curtain_wall: curtain(line([0, 8], [8, 8]), "Facade", { panel_material: "m-missing" }) }, outcome: reject("mutation.target-missing", ["curtain_wall", "panel_material"]) },
    ],
  },
  {
    kind: "delete-curtain-wall", emoji: 0x1f996, variant: "DeleteCurtainWall", verb: "delete", entity: "curtain-wall", displayName: "Delete Curtain Wall", binaryTag: 407,
    doc: "Removes a curtain wall that hosts no opening.",
    props: [curtainId],
    label: { en: 'format!("Delete curtain wall \\"{}\\"", self.id)', de: 'format!("Vorhangfassade \\"{}\\" löschen", self.id)' },
    target,
    cases: [
      { name: "removes", emoji: 0x2705, before: withFacade(), mutation: { id: "cw-1" }, outcome: ok },
      { name: "hosts-an-opening", emoji: 0x1f6ab, before: withFacade({ "o-1": F.opening("cw-1", "Window") }), mutation: { id: "cw-1" }, outcome: reject("mutation.target-referenced", ["cw-1"]) },
      { name: "missing", emoji: 0x1f573, before: withFacade(), mutation: { id: "cw-9" }, outcome: missing("cw-9") },
    ],
  },
  {
    kind: "set-curtain-wall", emoji: 0x1f506, variant: "SetCurtainWall", verb: "set", entity: "curtain-wall", displayName: "Set Curtain Wall", binaryTag: 408,
    doc: "Adjusts the named parameters of a curtain wall (axis, base offset, top, grid spacings, mullion profile, materials, name); every field left out stays as it is.",
    props: [
      curtainId,
      recordProp("axis", "Axis", { en: "Axis", de: "Achse" }, 20, "Option<Axis>"),
      num("base_offset", { en: "Base offset (m)", de: "Fußversatz (m)" }, 30, "Option<f64>"),
      recordProp("top", "TopConstraint", { en: "Top", de: "Oberkante" }, 40, "Option<TopConstraint>"),
      num("u_spacing", { en: "Horizontal spacing (m)", de: "Horizontaler Raster (m)" }, 50, "Option<f64>"),
      num("v_spacing", { en: "Vertical spacing (m)", de: "Vertikaler Raster (m)" }, 60, "Option<f64>"),
      recordProp("mullion", "Profile", { en: "Mullion profile", de: "Pfostenprofil" }, 70, "Option<Profile>"),
      { ...ref("panel_material", "material", "value", { en: "Panel material", de: "Füllungsmaterial" }, 80), rust: "Option<String>" },
      { ...ref("mullion_material", "material", "value", { en: "Mullion material", de: "Pfostenmaterial" }, 90), rust: "Option<String>" },
      text("name", { en: "Name", de: "Name" }, 100, "Option<String>"),
    ],
    label: { en: 'format!("Adjust curtain wall \\"{}\\"", self.id)', de: 'format!("Vorhangfassade \\"{}\\" anpassen", self.id)' },
    target,
    cases: [
      { name: "re-grids-the-facade", emoji: 0x2705, before: withFacade(), mutation: { id: "cw-1", u_spacing: 2, v_spacing: 1 }, outcome: ok },
      { name: "curves-the-facade", emoji: 0x1f300, before: withFacade(), mutation: { id: "cw-1", axis: arc([0, 8], [8, 8], 0.3), top: F.toStorey("st-first", 0.2) }, outcome: ok },
      { name: "swaps-materials-and-renames", emoji: 0x1f3a8, before: withFacade(), mutation: { id: "cw-1", panel_material: "m-alu", mullion: { Circle: { diameter: 0.08 } }, name: "South Facade" }, outcome: ok },
      { name: "unchanged", emoji: 0x1f9f2, before: withFacade(), mutation: { id: "cw-1", u_spacing: 1.5, name: "Facade" }, outcome: reject("mutation.no-op", ["cw-1"]) },
      { name: "spacing-non-positive", emoji: 0x1f6ab, before: withFacade(), mutation: { id: "cw-1", v_spacing: -1 }, outcome: reject("mutation.invariant", ["v_spacing"]) },
      { name: "material-missing", emoji: 0x26d4, before: withFacade(), mutation: { id: "cw-1", mullion_material: "m-missing" }, outcome: reject("mutation.target-missing", ["mullion_material"]) },
      { name: "missing", emoji: 0x1f573, before: withFacade(), mutation: { id: "cw-9", u_spacing: 2 }, outcome: missing("cw-9") },
    ],
  },
];

const snake = (kind: string) => kind.replaceAll("-", "_");
const relRoot = (path: string) => rel(path).slice(rel(artifact).length + 1);
const leafDir = (leaf: Leaf) => join(mutations, em(leaf.emoji) + leaf.kind);

const sparse = (leaf: Leaf) => {
  const optional = leaf.props.filter((prop) => prop.rust.startsWith("Option<"));
  if (optional.length === 0) return;
  const file = join(leafDir(leaf), em(0x1f9a0) + "mutation", RS);
  let source = readFileSync(file, "utf8");
  const imports = [...new Set(optional.map((prop) => prop.rust.slice("Option<".length, -1)).filter((name) => /^[A-Z]/.test(name) && name !== "String"))];
  source = source.replace(/^use crate::\{.*\};$/m, `use crate::{${["ModelDiff", "ModelMutation", "ModelSnapshot", ...imports].sort().join(", ")}};`);
  for (const prop of optional) source = source.replace(new RegExp(`^    pub ${prop.name}: `, "m"), (head) => `    #[value(default, skip_serializing_if = "Option::is_none")]\n${head}`);
  writeFileSync(file, source);
  const schemaFile = join(leafDir(leaf), em(0x1f9ec) + "schema", JSONF);
  const schema = JSON.parse(readFileSync(schemaFile, "utf8"));
  schema.required = schema.required.filter((name: string) => !optional.some((prop) => prop.name === name));
  writeFileSync(schemaFile, JSON.stringify(schema, null, 2) + "\n");
};

const helperDir = join(mutations, em(0x1f989) + "wall-geometry");
const helperMount = `                        #[path = "."]
                        pub mod wall_geometry {
                            #[path = "${relRoot(join(helperDir, RS))}"]
                            mod component;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "${relRoot(join(helperDir, em(0x1f9ea) + "tests", em(0x1f52c) + "unit", RS))}"]
                            mod tests;
                        }
`;

const withParametricTest = (leaf: Leaf, mount: string) => {
  const tail = "                        }\n";
  return mount.slice(0, -tail.length) + `                            #[cfg(test)]
                            #[path = "${relRoot(join(leafDir(leaf), em(0x1f9ea) + "tests", em(0x1f517) + "follows-by-inference", RS))}"]
                            mod tests_follows_by_inference;
` + tail;
};

if (import.meta.main) {
  const out = join(import.meta.dir, "🗑️generated", "m-walls");
  mkdirSync(out, { recursive: true });
  mkdirSync(helperDir, { recursive: true });
  const blocks: (readonly [string, string])[] = [["wall_geometry", helperMount]];
  for (const leaf of leaves) {
    const mount = emitLeaf(leaf);
    sparse(leaf);
    blocks.push([snake(leaf.kind), withParametricTest(leaf, mount)]);
  }
  writeFileSync(join(out, "mounts.txt"), blocks.map(([, block]) => block).join(""));
  console.log(`emitted ${leaves.length} leaves`);

  if (process.argv.includes("--register")) {
    const aggregate = join(mutations, RS);
    let source = readFileSync(aggregate, "utf8");
    for (const leaf of leaves) {
      const variant = `    ${leaf.variant}(super::${snake(leaf.kind)}::${leaf.variant}),\n`;
      if (!source.includes(variant.trim())) source = source.replace("pub enum ModelMutation {\n", (head) => head + variant);
      const kind = `    "${leaf.kind}",\n`;
      if (!source.includes(kind)) source = source.replace("pub const KINDS: &[&str] = &[\n", (head) => head + kind);
    }
    writeFileSync(aggregate, source);

    const rootFile = join(artifact, RS);
    let root = readFileSync(rootFile, "utf8");
    const at = root.lastIndexOf("\n", root.indexOf("//#endregion 🔖️Leaves")) + 1;
    const fresh = blocks.filter(([name]) => !root.includes(`pub mod ${name} {`)).map(([, block]) => block).join("");
    if (fresh) root = root.slice(0, at) + fresh + root.slice(at);
    writeFileSync(rootFile, root);
    console.log(fresh ? "registered" : "already registered");
  }
}
