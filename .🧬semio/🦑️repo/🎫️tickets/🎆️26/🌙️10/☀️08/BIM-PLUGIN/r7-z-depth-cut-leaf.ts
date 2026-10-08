#!/usr/bin/env bun
/**
 * 🎬️ Wave Z depth: the leaf `set-storey-cut-height` (authored plan cut height per storey). `bun r7-z-depth-cut-leaf.ts`
 * writes the boilerplate and fixtures of the leaf and prints the mount block for the artifact root. The hand-written
 * `diff`/`inverse` live in the leaf directory; do not re-run once they are blessed.
 */
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";

const assigned: Prop = {
  name: "cut_height",
  rust: "Assigned<Option<f64>>",
  schema: { type: "object", additionalProperties: false, required: ["value"], properties: { value: { type: ["number", "null"] } } },
  ui: { widget: "number", role: "value", label: { en: "Plan cut height (m, empty = default 1.2 m)", de: "Schnitthöhe im Grundriss (m, leer = Standard 1,2 m)" }, group: "value", order: 20 },
};
const id: Prop = { name: "id", rust: "String", schema: { type: "string" }, ui: { widget: "reference", role: "target", label: { en: "Storey", de: "Geschoss" }, ref: { kind: "storey" }, group: "target", order: 10 } };
const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });

const withCut = (cut: number) => {
  const scene: any = F.scene();
  scene.storeys["st-ground"].cut_height = cut;
  return scene;
};

export const leaf: Leaf = {
  kind: "set-storey-cut-height", emoji: 0x1f3ac, variant: "SetStoreyCutHeight", verb: "set", entity: "storey",
  doc: "Sets the height above its elevation at which the plan view cuts a storey; an assigned null returns to the 1.2 m default. Pure view convention: nothing else in the model depends on it.",
  displayName: "Set Storey Cut Height", binaryTag: 12,
  props: [id, assigned],
  uses: ["use crate::Assigned;"],
  label: { en: 'format!("Set plan cut height of storey \\"{}\\"", self.id)', de: 'format!("Schnitthöhe von Geschoss \\"{}\\" setzen", self.id)' },
  target: "vec![self.id.clone()]",
  cases: [
    { name: "sets-the-cut", emoji: 0x2705, before: F.scene(), mutation: { id: "st-ground", cut_height: { value: 1.5 } }, outcome: ok },
    { name: "clears-the-cut", emoji: 0x2795, before: withCut(1.5), mutation: { id: "st-ground", cut_height: { value: null } }, outcome: ok },
    { name: "non-positive", emoji: 0x1f6ab, before: F.scene(), mutation: { id: "st-ground", cut_height: { value: 0 } }, outcome: reject("mutation.invariant", ["cut_height"]) },
    { name: "missing", emoji: 0x26d4, before: F.scene(), mutation: { id: "st-attic", cut_height: { value: 1.5 } }, outcome: reject("mutation.target-missing", ["st-attic"]) },
    { name: "already-set", emoji: 0x1f534, before: withCut(1.5), mutation: { id: "st-ground", cut_height: { value: 1.5 } }, outcome: reject("mutation.no-op", ["st-ground"]) },
    { name: "already-default", emoji: 0x1f7e2, before: F.scene(), mutation: { id: "st-ground", cut_height: { value: null } }, outcome: reject("mutation.no-op", ["st-ground"]) },
  ],
};

console.log(emitLeaf(leaf));
