#!/usr/bin/env bun
/**
 * 🎢️ Wave W04 (binary tags 4000 and 4001): the leaves `set-element-storey` and `set-element-phase` of `s.bim.model@1`. `bun r10-w04-storey-phase-leaves.ts`
 * writes the boilerplate and the fixtures of both leaves (never the hand-written `🔺️diff`/`↩️inverse`, never a blessed `after`/`diff`) and prints the
 * mount blocks for the artifact root. Re-run it only to add a case; the leaf directories are the source of truth afterwards.
 */
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";

type Label = { en: string; de: string };
const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
const ref = (name: string, kind: string, role: "target" | "value", label: Label, order: number): Prop => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "reference", role, label, ref: { kind }, group: role === "target" ? "target" : "value", order } });
const phaseProp: Prop = { name: "phase", rust: "Phase", schema: { $ref: `${ART}#/$defs/Phase` }, ui: { widget: "select", role: "value", label: { en: "Phase", de: "Phase" }, group: "value", order: 20 } };
const target = "vec![self.id.clone()]";
const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });

const profile = { Rectangle: { width: 0.3, depth: 0.3 } };
const point = (x: number, y: number) => ({ point: F.P(x, y), bulge: 0 });
const window = (host: string, name: string, offset: number) => ({ host, kind: { Window: { window_type: "win-1" } }, offset, flip_hand: false, flip_facing: false, name });

function house(firstHeight = 2.8): any {
  const model: any = F.scene({ storeys: { "st-first": F.storey("bldg-1", "First", 1, firstHeight) }, openings: { "op-1": window("w-south", "Window 1", 2), "op-2": window("w-south", "Window 2", 5) } });
  model.window_types = { "win-1": { name: "Window", width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.05, frame_depth: 0.1, panes: 2, material: "m-brick" } };
  model.column_types = { "ct-1": { name: "Column 300", profile, material: "m-brick" } };
  model.slab_types = { "stt-22": { name: "Slab 220", layers: [F.layer("m-brick", 0.22)] } };
  model.columns = { "c-1": { storey: "st-ground", column_type: "ct-1", position: F.P(1, 1), rotation: 0, base_offset: 0, top: F.storeyTop(0), phase: "New", name: "Column" } };
  model.slabs = { "sl-1": { storey: "st-ground", slab_type: "stt-22", boundary: [point(0, 0), point(8, 0), point(8, 6), point(0, 6)], holes: [], offset: 0, phase: "New", name: "Slab" } };
  model.spaces = { "sp-1": { storey: "st-ground", number: "1", name: "Hall", boundary: { Bounded: { seed: F.P(2, 2) } }, usage: "", phase: "New" } };
  return model;
}

function withColumnToStorey(): any {
  const model = house();
  model.columns["c-1"].top = F.toStorey("st-first", 0);
  return model;
}

function twoBuildings(): any {
  const model = house();
  model.buildings["bldg-2"] = F.building("site-1", "Annex");
  model.storeys["st-annex"] = F.storey("bldg-2", "Annex", 0, 3);
  return model;
}

function demolished(): any {
  const model = house();
  model.walls["w-south"].phase = "Demolished";
  return model;
}

export const storey: Leaf = {
  kind: "set-element-storey", emoji: 0x1f3a2, variant: "SetElementStorey", verb: "set", entity: "element", displayName: "Set Element Storey", binaryTag: 4000,
  doc: "Stands a storey-placed element (wall, curtain wall, column, beam, slab, ceiling, roof, stair, railing, ramp, space) on another storey of its building. Hosted openings follow their host by reference; refused while the top constraint would no longer lie above the base or a hosted opening would rise above the new height.",
  props: [ref("id", "element", "target", { en: "Element", de: "Element" }, 10), ref("storey", "storey", "value", { en: "Storey", de: "Geschoss" }, 20)],
  label: { en: 'format!("Move element \\"{}\\" to storey \\"{}\\"", self.id, self.storey)', de: 'format!("Element \\"{}\\" in Geschoss \\"{}\\" verschieben", self.id, self.storey)' },
  target,
  cases: [
    { name: "moves-a-wall-and-its-openings-follow", emoji: 0x2705, before: house(), mutation: { id: "w-south", storey: "st-first" }, outcome: ok },
    { name: "moves-a-column", emoji: 0x1f3db, before: house(), mutation: { id: "c-1", storey: "st-first" }, outcome: ok },
    { name: "moves-a-room", emoji: 0x1f6cb, before: house(), mutation: { id: "sp-1", storey: "st-first" }, outcome: ok },
    { name: "moves-a-slab", emoji: 0x2b1c, before: house(), mutation: { id: "sl-1", storey: "st-first" }, outcome: ok },
    { name: "opening-would-break", emoji: 0x1f6aa, before: house(1.0), mutation: { id: "w-south", storey: "st-first" }, outcome: reject("mutation.invariant", ["storey"]) },
    { name: "top-no-longer-above-the-base", emoji: 0x1f51d, before: withColumnToStorey(), mutation: { id: "c-1", storey: "st-first" }, outcome: reject("mutation.invariant", ["storey"]) },
    { name: "unchanged", emoji: 0x1f9f2, before: house(), mutation: { id: "w-south", storey: "st-ground" }, outcome: reject("mutation.no-op", ["w-south"]) },
    { name: "storey-missing", emoji: 0x1f6ab, before: house(), mutation: { id: "w-south", storey: "st-attic" }, outcome: reject("mutation.target-missing", ["storey"]) },
    { name: "other-building", emoji: 0x1f3d8, before: twoBuildings(), mutation: { id: "w-south", storey: "st-annex" }, outcome: reject("mutation.invariant", ["storey"]) },
    { name: "opening-follows-its-host", emoji: 0x1fa9f, before: house(), mutation: { id: "op-1", storey: "st-first" }, outcome: reject("mutation.invariant", ["op-1"]) },
    { name: "not-storey-placed", emoji: 0x1f30d, before: house(), mutation: { id: "bldg-1", storey: "st-first" }, outcome: reject("mutation.invariant", ["bldg-1"]) },
    { name: "missing", emoji: 0x26d4, before: house(), mutation: { id: "w-attic", storey: "st-first" }, outcome: reject("mutation.target-missing", ["w-attic"]) },
  ],
};

export const phase: Leaf = {
  kind: "set-element-phase", emoji: 0x1f570, variant: "SetElementPhase", verb: "set", entity: "element", displayName: "Set Element Phase", binaryTag: 4001,
  doc: "Puts a phasable element (wall, curtain wall, column, beam, slab, roof, stair, railing, space) into a construction phase: existing, new, demolished or temporary. An opening takes the phase of its host, so only the host is set.",
  props: [ref("id", "element", "target", { en: "Element", de: "Element" }, 10), phaseProp],
  label: { en: 'format!("Set element \\"{}\\" to phase {:?}", self.id, self.phase)', de: 'format!("Element \\"{}\\" in Phase {:?} setzen", self.id, self.phase)' },
  target,
  cases: [
    { name: "demolishes-a-wall", emoji: 0x2705, before: house(), mutation: { id: "w-south", phase: "Demolished" }, outcome: ok },
    { name: "keeps-a-demolished-wall-existing", emoji: 0x1f3da, before: demolished(), mutation: { id: "w-south", phase: "Existing" }, outcome: ok },
    { name: "phases-a-room", emoji: 0x1f6cb, before: house(), mutation: { id: "sp-1", phase: "Existing" }, outcome: ok },
    { name: "phases-a-slab", emoji: 0x2b1c, before: house(), mutation: { id: "sl-1", phase: "Temporary" }, outcome: ok },
    { name: "unchanged", emoji: 0x1f9f2, before: house(), mutation: { id: "w-south", phase: "New" }, outcome: reject("mutation.no-op", ["w-south"]) },
    { name: "opening-takes-the-phase-of-its-host", emoji: 0x1fa9f, before: house(), mutation: { id: "op-1", phase: "Demolished" }, outcome: reject("mutation.invariant", ["op-1"]) },
    { name: "carries-no-phase", emoji: 0x1f30d, before: house(), mutation: { id: "st-ground", phase: "Existing" }, outcome: reject("mutation.invariant", ["st-ground"]) },
    { name: "missing", emoji: 0x26d4, before: house(), mutation: { id: "w-attic", phase: "Existing" }, outcome: reject("mutation.target-missing", ["w-attic"]) },
  ],
};

console.log(emitLeaf(storey));
console.log(emitLeaf(phase));
