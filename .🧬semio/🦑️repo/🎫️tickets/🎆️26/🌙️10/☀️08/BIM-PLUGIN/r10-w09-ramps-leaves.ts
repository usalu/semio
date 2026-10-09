#!/usr/bin/env bun
/**
 * 🛝️ Wave W09 (`w09-ramps`): the three ramp leaves `create-ramp`, `set-ramp`, `delete-ramp` of `s.bim.model@1` (binary tags 9000..9002).
 * `bun r10-w09-ramps-leaves.ts` writes their boilerplate and fixtures (never the two logic files of a leaf, never a blessed `after`/`diff`)
 * and prints the mount blocks into `🗑️generated/w09-ramps/mounts.txt`. The hand-written parts (`set-ramp` payload patch methods, diffs, inverses)
 * live in the leaf directories, which are the source of truth after the first run.
 */
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { em, mutations } from "./r3-f1-paths.ts";

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
const optionalRecord = (name: string, def: string, rust: string, label: Label, order: number): Prop => ({ name, rust: `Option<${rust}>`, schema: record(def), ui: { widget: "record", role: "value", label, group: "value", order } });
const num = (name: string, label: Label, order: number): Prop => ({ name, rust: "Option<f64>", schema: { type: "number" }, ui: { widget: "number", role: "value", label, group: "value", order } });
const toggle = (name: string, label: Label, order: number): Prop => ({ name, rust: "Option<bool>", schema: { type: "boolean" }, ui: { widget: "toggle", role: "value", label, group: "value", order } });
const text = (name: string, label: Label, order: number): Prop => ({ name, rust: "Option<String>", schema: { type: "string" }, ui: { widget: "text", role: "value", label, group: "identity", order } });
const materialRef = (order: number): Prop => ({ name: "material", rust: "Option<String>", schema: { type: "string" }, ui: { widget: "reference", role: "value", label: { en: "Material", de: "Material" }, ref: { kind: "material" }, group: "value", order } });
const pathProp = (order: number): Prop => ({ name: "path", rust: "Option<Vec<Vertex>>", schema: { type: "array", items: record("Vertex") }, ui: { widget: "record", role: "value", label: { en: "Path", de: "Verlauf" }, group: "value", order } });

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";

const vertex = (x: number, y: number, bulge = 0) => ({ point: F.P(x, y), bulge });
const ramp = (over: Record<string, unknown> = {}) => ({
  storey: "st-ground",
  path: [vertex(0, 0), vertex(10, 0)],
  width: 1.2,
  landing_start: 1.5,
  landing_end: 1.5,
  landing_turn: 1.5,
  max_slope: 0.0833333333333333,
  thickness: 0.2,
  material: "m-concrete",
  base_offset: 0,
  top: F.unconnected(0.5),
  railing_left: false,
  railing_right: false,
  name: "Entrance Ramp",
  ...over,
});

const base = (extra: { ramps?: Record<string, unknown>; railings?: Record<string, unknown>; properties?: Record<string, unknown>; classifications?: Record<string, unknown> } = {}) => {
  const snapshot: any = F.scene();
  snapshot.materials["m-concrete"] = F.material("Concrete");
  snapshot.ramps = extra.ramps ?? {};
  if (extra.railings) snapshot.railings = extra.railings;
  if (extra.properties) snapshot.properties = extra.properties;
  if (extra.classifications) snapshot.classifications = extra.classifications;
  return snapshot;
};
const WITH_RAMP = () => base({ ramps: { "rp-1": ramp() } });
const hosted = (element: string) => ({ storey: "st-ground", path: [], height: 1, post_spacing: 1.2, profile: { Rectangle: { width: 0.06, depth: 0.04 } }, post_profile: { Rectangle: { width: 0.05, depth: 0.05 } }, infill: "None", material: "m-concrete", base_offset: 0, host: { element, side: "Left", edge: 0, inset: 0.05 }, phase: "New", name: "Ramp Guard" });

export const leaves: Leaf[] = [
  {
    kind: "create-ramp", emoji: 0x1f6dd, variant: "CreateRamp", verb: "create", entity: "ramp", doc: "Brings a new ramp onto a storey; its length, rise, slope, landings, solid and compliance are inferred from the authored path, landings and top constraint.", displayName: "Create Ramp", binaryTag: 9000,
    props: [id("ramp", "identity", { en: "Ramp id", de: "Rampen-Id" }, 10), recordProp("ramp", "Ramp", { en: "Ramp", de: "Rampe" })],
    label: { en: 'format!("Create ramp \\"{}\\"", self.ramp.name)', de: 'format!("Rampe \\"{}\\" anlegen", self.ramp.name)' },
    target,
    cases: [
      { name: "adds-a-straight-ramp", emoji: 0x2705, before: base(), mutation: { id: "rp-1", ramp: ramp() }, outcome: ok },
      { name: "adds-a-bent-ramp-with-railings", emoji: 0x1f4d0, before: base(), mutation: { id: "rp-2", ramp: ramp({ path: [vertex(0, 0), vertex(6, 0), vertex(6, 5)], top: F.unconnected(0.6), railing_left: true, railing_right: true, name: "Bent Ramp" }) }, outcome: ok },
      { name: "adds-a-curved-ramp-to-the-first-storey", emoji: 0x1f300, before: base(), mutation: { id: "rp-3", ramp: ramp({ storey: "st-first", path: [vertex(0, 0, 0.4142135623730951), vertex(5, 5)], top: F.toStorey("st-first", 0.5), base_offset: 0.1, name: "Curved Ramp" }) }, outcome: ok },
      { name: "duplicate", emoji: 0x1f6ab, before: WITH_RAMP(), mutation: { id: "rp-1", ramp: ramp({ name: "Other" }) }, outcome: reject("mutation.duplicate-id", ["rp-1"]) },
      { name: "id-taken-by-another-kind", emoji: 0x1f9ed, before: base(), mutation: { id: "w-south", ramp: ramp() }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "storey-missing", emoji: 0x26d4, before: base(), mutation: { id: "rp-1", ramp: ramp({ storey: "st-attic" }) }, outcome: reject("mutation.target-missing", ["ramp", "storey"]) },
      { name: "material-missing", emoji: 0x1f9f1, before: base(), mutation: { id: "rp-1", ramp: ramp({ material: "m-glass" }) }, outcome: reject("mutation.target-missing", ["ramp", "material"]) },
      { name: "top-storey-missing", emoji: 0x1f534, before: base(), mutation: { id: "rp-1", ramp: ramp({ top: F.toStorey("st-attic", 0) }) }, outcome: reject("mutation.target-missing", ["ramp", "top", "storey"]) },
      { name: "path-too-short", emoji: 0x1f4cf, before: base(), mutation: { id: "rp-1", ramp: ramp({ path: [vertex(1, 1)] }) }, outcome: reject("mutation.invariant", ["ramp", "path"]) },
      { name: "non-positive-width", emoji: 0x1f6d1, before: base(), mutation: { id: "rp-1", ramp: ramp({ width: 0 }) }, outcome: reject("mutation.invariant", ["ramp", "width"]) },
      { name: "non-positive-thickness", emoji: 0x1f4a5, before: base(), mutation: { id: "rp-1", ramp: ramp({ thickness: -0.1 }) }, outcome: reject("mutation.invariant", ["ramp", "thickness"]) },
      { name: "non-positive-slope-limit", emoji: 0x1f4c9, before: base(), mutation: { id: "rp-1", ramp: ramp({ max_slope: 0 }) }, outcome: reject("mutation.invariant", ["ramp", "max_slope"]) },
      { name: "negative-landing", emoji: 0x1f6a7, before: base(), mutation: { id: "rp-1", ramp: ramp({ landing_end: -1 }) }, outcome: reject("mutation.invariant", ["ramp", "landing_start"]) },
    ],
  },
  {
    kind: "set-ramp", emoji: 0x1f6f9, variant: "SetRamp", verb: "set", entity: "ramp", doc: "Sets any of a ramp's authored path, width, landing lengths, slope limit, thickness, material, base offset, top constraint, side railings and name; absent fields stay untouched.", displayName: "Set Ramp", binaryTag: 9001,
    props: [
      id("ramp", "target", { en: "Ramp", de: "Rampe" }),
      pathProp(20),
      num("width", { en: "Width (m)", de: "Breite (m)" }, 30),
      num("landing_start", { en: "Foot landing (m)", de: "Antrittspodest (m)" }, 31),
      num("landing_end", { en: "Head landing (m)", de: "Austrittspodest (m)" }, 32),
      num("landing_turn", { en: "Turn landing (m)", de: "Wendepodest (m)" }, 33),
      num("max_slope", { en: "Slope limit (rise per run)", de: "Neigungsgrenze (Höhe je Länge)" }, 34),
      num("thickness", { en: "Thickness (m)", de: "Dicke (m)" }, 35),
      materialRef(36),
      num("base_offset", { en: "Base offset (m)", de: "Fußversatz (m)" }, 37),
      optionalRecord("top", "TopConstraint", "TopConstraint", { en: "Top constraint", de: "Oberkante" }, 38),
      toggle("railing_left", { en: "Left railing", de: "Geländer links" }, 39),
      toggle("railing_right", { en: "Right railing", de: "Geländer rechts" }, 40),
      text("name", { en: "Name", de: "Name" }, 50),
    ],
    label: { en: 'format!("Edit ramp \\"{}\\"", self.id)', de: 'format!("Rampe \\"{}\\" ändern", self.id)' },
    target,
    cases: [
      { name: "reshapes", emoji: 0x2705, before: WITH_RAMP(), mutation: { id: "rp-1", path: [vertex(0, 0), vertex(12, 0)], width: 1.5, top: F.unconnected(0.7) }, outcome: ok },
      { name: "renames-only", emoji: 0x1f3f7, before: WITH_RAMP(), mutation: { id: "rp-1", name: "Service Ramp" }, outcome: ok },
      { name: "carries-railings-on-both-sides", emoji: 0x1f6e4, before: WITH_RAMP(), mutation: { id: "rp-1", railing_left: true, railing_right: true }, outcome: ok },
      { name: "steepens-beyond-the-limit", emoji: 0x26f0, before: WITH_RAMP(), mutation: { id: "rp-1", top: F.unconnected(1.2) }, outcome: ok },
      { name: "restates-an-unchanged-field", emoji: 0x1f9ed, before: WITH_RAMP(), mutation: { id: "rp-1", width: 1.2, landing_start: 2 }, outcome: ok },
      { name: "nothing-to-change", emoji: 0x1f4a4, before: WITH_RAMP(), mutation: { id: "rp-1", width: 1.2, name: "Entrance Ramp" }, outcome: reject("mutation.no-op", ["rp-1"]) },
      { name: "missing", emoji: 0x1f6ab, before: base(), mutation: { id: "rp-1", width: 1.5 }, outcome: reject("mutation.target-missing", ["rp-1"]) },
      { name: "path-too-short", emoji: 0x1f4cf, before: WITH_RAMP(), mutation: { id: "rp-1", path: [vertex(2, 2)] }, outcome: reject("mutation.invariant", ["path"]) },
      { name: "non-positive-width", emoji: 0x1f6d1, before: WITH_RAMP(), mutation: { id: "rp-1", width: -1 }, outcome: reject("mutation.invariant", ["width"]) },
      { name: "non-positive-slope-limit", emoji: 0x1f4c9, before: WITH_RAMP(), mutation: { id: "rp-1", max_slope: 0 }, outcome: reject("mutation.invariant", ["max_slope"]) },
      { name: "material-missing", emoji: 0x1f9f1, before: WITH_RAMP(), mutation: { id: "rp-1", material: "m-glass" }, outcome: reject("mutation.target-missing", ["material"]) },
      { name: "top-storey-missing", emoji: 0x1f534, before: WITH_RAMP(), mutation: { id: "rp-1", top: F.toStorey("st-attic", 0) }, outcome: reject("mutation.target-missing", ["top", "storey"]) },
    ],
  },
  {
    kind: "delete-ramp", emoji: 0x1f6fc, variant: "DeleteRamp", verb: "delete", entity: "ramp", doc: "Removes a ramp together with the railings hosted by it and its properties and classifications.", displayName: "Delete Ramp", binaryTag: 9002,
    props: [id("ramp", "target", { en: "Ramp", de: "Rampe" })],
    label: { en: 'format!("Delete ramp \\"{}\\"", self.id)', de: 'format!("Rampe \\"{}\\" löschen", self.id)' },
    target,
    inverseRows: { bounded: 4096 },
    cases: [
      { name: "removes", emoji: 0x2705, before: WITH_RAMP(), mutation: { id: "rp-1" }, outcome: ok },
      { name: "missing", emoji: 0x1f6ab, before: base(), mutation: { id: "rp-1" }, outcome: reject("mutation.target-missing", ["rp-1"]) },
      {
        name: "removes-its-data", emoji: 0x1f9ed,
        before: base({ ramps: { "rp-1": ramp() }, properties: { "rp-1": { Pset_RampCommon: { Reference: { Text: { value: "R-01" } } } } }, classifications: { "rp-1": { system: "Uniclass", code: "Ss_32_30", title: "Ramps" } } }),
        mutation: { id: "rp-1" }, outcome: ok,
      },
      { name: "takes-its-hosted-railing-with-it", emoji: 0x1f6e4, before: base({ ramps: { "rp-1": ramp() }, railings: { "rl-1": hosted("rp-1") } }), mutation: { id: "rp-1" }, outcome: ok },
    ],
  },
];

if (import.meta.main) {
  const out = join(import.meta.dir, "🗑️generated", "w09-ramps");
  mkdirSync(out, { recursive: true });
  const fresh = leaves.filter((leaf) => !existsSync(join(mutations, em(leaf.emoji) + leaf.kind)));
  const mounts = fresh.map(emitLeaf).join("");
  writeFileSync(join(out, "mounts.txt"), mounts);
  console.log(`wrote ${fresh.length} of ${leaves.length} leaves (existing leaves are never rewritten), mounts in ${out}`);
}
