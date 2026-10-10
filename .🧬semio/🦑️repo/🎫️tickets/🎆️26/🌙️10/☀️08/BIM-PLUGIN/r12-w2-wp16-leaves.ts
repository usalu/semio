#!/usr/bin/env bun
/**
 * 🤝️ Wave W2 (`w2-wp16-coordination`): the twelve leaves of the coordination vocabulary of `s.bim.model@1` (binary tags 16000..16011): create, set and delete of a clash set, a rule, an issue and an issue
 * comment, plus the cascade extension of `delete-storey` for clash sets and rules scoped to a removed storey (one case). `bun r12-w2-wp16-leaves.ts` rewrites their boilerplate, their hand-written `diff`/`inverse`
 * and their fixtures (never a blessed `after`/`diff`) and writes the mount text to `🗑️generated/w2-wp16-coordination/mounts.txt`. Bless with `BIM_BLESS=1 cargo test … <kind>`.
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitCase, emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { em, JSONF, mutations, RS } from "./r3-f1-paths.ts";

type Label = { en: string; de: string };
const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
const ref = (def: string) => ({ $ref: `${ART}#/$defs/${def}` });
const assigned = (inner: unknown) => ({ type: "object", additionalProperties: false, required: ["value"], properties: { value: { oneOf: [inner, { type: "null" }] } } });
const strings = { type: "array", items: { type: "string" } };
const id = (role: "target" | "identity", label: Label, kind: string, order = 10): Prop => ({
  name: "id",
  rust: "String",
  schema: { type: "string" },
  ui: role === "target" ? { widget: "reference", role, label, ref: { kind }, group: "target", order } : { widget: "text", role: "identity", label, group: "identity", order },
});
const prop = (name: string, rust: string, schema: unknown, widget: string, label: Label, order: number, refKind?: string): Prop => ({
  name,
  rust,
  schema: schema as Record<string, unknown>,
  ui: { widget, role: "value", label, group: "value", order, ...(refKind ? { ref: { kind: refKind } } : {}) },
});

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const P = F.P;

//#region 🔖️Fixtures
const sel = (over: Record<string, unknown> = {}) => ({ classes: [], storeys: [], phases: [], ids: [], ...over });
const clashSetRow = (over: Record<string, unknown> = {}) => ({ name: "Beams against walls", a: sel({ classes: ["Beam"] }), b: sel({ classes: ["Wall"] }), tolerance: 0.001, clearance: 0, ...over });
const scopeRow = (over: Record<string, unknown> = {}) => ({ storeys: [], phases: [], ids: [], filter: "", ...over });
const ruleRow = (over: Record<string, unknown> = {}) => ({ name: "Maximum riser", kind: "MaxRiser", limit: 0.19, severity: "Error", scope: scopeRow(), ...over });
const camera = { target: P(4, 3), target_height: 1.5, azimuth: 0.8, pitch: 0.5, distance: 25 };
const pt3 = (x: number, y: number, z: number) => ({ x, y, z });
const viewpointRow = (over: Record<string, unknown> = {}) => ({ camera, isolate: [], ...over });
const issueRow = (over: Record<string, unknown> = {}) => ({
  title: "Beam hits the south wall",
  description: "The beam runs through the wall.",
  status: "Open",
  priority: "Normal",
  assignee: "",
  author: "UG",
  created: "2026-10-09T08:30:00Z",
  labels: ["Clash"],
  elements: ["w-south", "w-east"],
  viewpoint: viewpointRow(),
  ...over,
});
const commentRow = (over: Record<string, unknown> = {}) => ({ issue: "i-1", author: "AB", date: "2026-10-09T09:00:00Z", text: "I will check the structure.", ...over });

const scene = (extra: Record<string, unknown> = {}) => ({ ...F.scene(), ...extra });
const withSet = () => scene({ clash_sets: { "cs-1": clashSetRow() } });
const rich = () =>
  scene({
    clash_sets: { "cs-1": clashSetRow(), "cs-2": clashSetRow({ name: "Walls against walls", a: sel({ classes: ["Wall"] }), b: sel({ classes: ["Wall"] }), clearance: 0.05 }) },
    rules: { "r-1": ruleRow(), "r-2": ruleRow({ name: "Minimum door width", kind: "MinDoorWidth", limit: 0.9, severity: "Warning", scope: scopeRow({ storeys: ["st-ground"] }) }) },
    issues: {
      "i-1": issueRow(),
      "i-2": issueRow({ title: "Door too narrow", priority: "High", assignee: "AB", labels: [], elements: ["w-east"], viewpoint: undefined, clash: { set: "cs-1", first: "w-south", second: "w-east" } }),
    },
    issue_comments: { "c-1": commentRow(), "c-2": commentRow({ author: "UG", date: "2026-10-09T10:00:00Z", text: "Thanks." }), "c-3": commentRow({ issue: "i-2" }) },
  });
const withIssue = () => scene({ issues: { "i-1": issueRow() } });
const withIssueAndSet = () => scene({ clash_sets: { "cs-1": clashSetRow() }, issues: { "i-1": issueRow() } });
const withRule = () => scene({ rules: { "r-1": ruleRow() } });
const withComments = () => scene({ issues: { "i-1": issueRow() }, issue_comments: { "c-1": commentRow(), "c-2": commentRow({ date: "2026-10-09T10:00:00Z", text: "Thanks." }) } });
const clean = (value: Record<string, unknown>) => JSON.parse(JSON.stringify(value));
//#endregion 🔖️Fixtures

const MUTATION_USES: Record<string, string> = {
  "set-clash-set": "use crate::{ClashSetPatch, ElementSelector};",
  "set-rule": "use crate::{RuleKind, RulePatch, RuleScope, RuleSeverity};",
  "set-issue": "use crate::{Assigned, ClashRef, IssuePatch, IssuePriority, IssueStatus, IssueViewpoint};",
  "set-issue-comment": "use crate::IssueCommentPatch;",
};

const emoji = ["2705", "1f9f2", "1f9f5", "1f9f4", "1f9f3", "1f9f1", "1f9f0", "1f9ef", "1f9ee", "1f9ed", "1f9e9", "1f9e8", "1f9e7", "1f9e6", "1f9e5", "1f9e4", "1f9e3", "1f9e2", "1f9e1", "1f9e0", "1f9df", "1f9de", "1f9dd", "1f9dc", "1f9db", "1f9da", "1f9d9", "1f9d8", "1f9d7", "1f9d6"].map((hex) => parseInt(hex, 16));
const cases = <T extends { name: string; emoji?: number }>(rows: T[]) => rows.map((row, index) => ({ ...row, emoji: emoji[index] }));
const target = "vec![self.id.clone()]";

const selectorProp = (name: string, label: Label, order: number, optional: boolean) =>
  prop(name, optional ? "Option<ElementSelector>" : "ElementSelector", ref("ElementSelector"), "record", label, order);

export const leaves: Leaf[] = [
  {
    kind: "create-clash-set", emoji: 0x1f6a8, variant: "CreateClashSet", verb: "create", entity: "clash-set", binaryTag: 16000, displayName: "Create Clash Set",
    doc: "Brings a new clash set into the model: side A and side B as selectors (classes, storeys, phases, element ids), the tolerance below which an overlap is no hard clash and the clearance below which two elements are a soft clash. The clashes themselves are inferred.",
    props: [id("identity", { en: "Clash set id", de: "Kollisionssatz-Id" }, "clash-set"), prop("clash_set", "ClashSet", ref("ClashSet"), "record", { en: "Clash set", de: "Kollisionssatz" }, 20)],
    label: { en: 'format!("Create clash set \\"{}\\"", self.clash_set.name)', de: 'format!("Kollisionssatz \\"{}\\" anlegen", self.clash_set.name)' },
    target,
    cases: cases([
      { name: "adds-beams-against-walls", before: scene(), mutation: { id: "cs-1", clash_set: clashSetRow() }, outcome: ok },
      { name: "adds-a-storey-restricted-set", before: scene(), mutation: { id: "cs-1", clash_set: clashSetRow({ name: "Ground floor", a: sel({ storeys: ["st-ground"] }), b: sel({ storeys: ["st-ground"] }) }) }, outcome: ok },
      { name: "adds-a-clearance-set", before: scene(), mutation: { id: "cs-1", clash_set: clashSetRow({ name: "Service clearance", a: sel({ classes: ["Mep"] }), b: sel({ classes: ["Beam", "Column", "Slab"] }), tolerance: 0.005, clearance: 0.05 }) }, outcome: ok },
      { name: "adds-an-element-selected-set", before: scene(), mutation: { id: "cs-1", clash_set: clashSetRow({ name: "South against east", a: sel({ ids: ["w-south"] }), b: sel({ ids: ["w-east"] }) }) }, outcome: ok },
      { name: "adds-a-phase-restricted-set", before: scene(), mutation: { id: "cs-1", clash_set: clashSetRow({ name: "New against existing", a: sel({ phases: ["New"] }), b: sel({ phases: ["Existing"] }) }) }, outcome: ok },
      { name: "duplicate-id", before: withSet(), mutation: { id: "cs-1", clash_set: clashSetRow({ name: "Other" }) }, outcome: reject("mutation.duplicate-id", ["cs-1"]) },
      { name: "id-taken-by-another-kind", before: scene(), mutation: { id: "w-south", clash_set: clashSetRow() }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "blank-name", before: scene(), mutation: { id: "cs-1", clash_set: clashSetRow({ name: " " }) }, outcome: reject("mutation.invariant", ["clash_set", "name"]) },
      { name: "negative-tolerance", before: scene(), mutation: { id: "cs-1", clash_set: clashSetRow({ tolerance: -0.001 }) }, outcome: reject("mutation.invariant", ["clash_set", "tolerance"]) },
      { name: "clearance-too-large", before: scene(), mutation: { id: "cs-1", clash_set: clashSetRow({ clearance: 11 }) }, outcome: reject("mutation.invariant", ["clash_set", "clearance"]) },
      { name: "storey-missing", before: scene(), mutation: { id: "cs-1", clash_set: clashSetRow({ a: sel({ storeys: ["st-attic"] }) }) }, outcome: reject("mutation.target-missing", ["clash_set", "a"]) },
      { name: "element-missing", before: scene(), mutation: { id: "cs-1", clash_set: clashSetRow({ b: sel({ ids: ["w-9"] }) }) }, outcome: reject("mutation.target-missing", ["clash_set", "b"]) },
      { name: "class-twice", before: scene(), mutation: { id: "cs-1", clash_set: clashSetRow({ a: sel({ classes: ["Beam", "Beam"] }) }) }, outcome: reject("mutation.invariant", ["clash_set", "a"]) },
    ]),
  },
  {
    kind: "set-clash-set", emoji: 0x1f6a5, variant: "SetClashSet", verb: "set", entity: "clash-set", binaryTag: 16001, displayName: "Set Clash Set",
    doc: "Sparsely changes a clash set: its name, either selector (replaced as a whole), the tolerance and the clearance. The clashes are inferred and follow.",
    props: [
      id("target", { en: "Clash set", de: "Kollisionssatz" }, "clash-set"),
      prop("name", "Option<String>", { type: "string" }, "text", { en: "Name", de: "Name" }, 20),
      selectorProp("a", { en: "Side A", de: "Seite A" }, 30, true),
      selectorProp("b", { en: "Side B", de: "Seite B" }, 40, true),
      prop("tolerance", "Option<f64>", { type: "number", minimum: 0 }, "number", { en: "Tolerance (m)", de: "Toleranz (m)" }, 50),
      prop("clearance", "Option<f64>", { type: "number", minimum: 0 }, "number", { en: "Clearance (m)", de: "Abstand (m)" }, 60),
    ],
    uses: [MUTATION_USES["set-clash-set"]],
    label: { en: 'format!("Change clash set \\"{}\\"", self.id)', de: 'format!("Kollisionssatz \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "renames", before: rich(), mutation: { id: "cs-1", name: "Beams against masonry" }, outcome: ok },
      { name: "retargets-side-a", before: rich(), mutation: { id: "cs-1", a: sel({ classes: ["Beam", "Column"] }) }, outcome: ok },
      { name: "retargets-side-b", before: rich(), mutation: { id: "cs-1", b: sel({ classes: ["Wall", "Slab"], storeys: ["st-ground"] }) }, outcome: ok },
      { name: "loosens-the-tolerance", before: rich(), mutation: { id: "cs-1", tolerance: 0.01 }, outcome: ok },
      { name: "asks-for-a-clearance", before: rich(), mutation: { id: "cs-1", clearance: 0.1 }, outcome: ok },
      { name: "drops-the-clearance", before: rich(), mutation: { id: "cs-2", clearance: 0 }, outcome: ok },
      { name: "restates-an-unchanged-field", before: rich(), mutation: { id: "cs-1", name: "Beams against walls", tolerance: 0.002 }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "cs-9", name: "Gone" }, outcome: reject("mutation.target-missing", ["cs-9"]) },
      { name: "unchanged", before: rich(), mutation: { id: "cs-1", name: "Beams against walls", clearance: 0 }, outcome: reject("mutation.no-op", ["cs-1"]) },
      { name: "empty-patch", before: rich(), mutation: { id: "cs-1" }, outcome: reject("mutation.no-op", ["cs-1"]) },
      { name: "blank-name", before: rich(), mutation: { id: "cs-1", name: "" }, outcome: reject("mutation.invariant", ["name"]) },
      { name: "tolerance-too-large", before: rich(), mutation: { id: "cs-1", tolerance: 2 }, outcome: reject("mutation.invariant", ["tolerance"]) },
      { name: "storey-missing", before: rich(), mutation: { id: "cs-1", b: sel({ storeys: ["st-attic"] }) }, outcome: reject("mutation.target-missing", ["b"]) },
      { name: "element-missing", before: rich(), mutation: { id: "cs-1", a: sel({ ids: ["w-9"] }) }, outcome: reject("mutation.target-missing", ["a"]) },
    ]),
  },
  {
    kind: "delete-clash-set", emoji: 0x1f4a2, variant: "DeleteClashSet", verb: "delete", entity: "clash-set", binaryTag: 16002, displayName: "Delete Clash Set",
    doc: "Removes a clash set together with its properties and classifications. Issues raised from one of its clashes keep naming the set as history.",
    props: [id("target", { en: "Clash set", de: "Kollisionssatz" }, "clash-set")],
    inverseRows: { bounded: 4096 },
    label: { en: 'format!("Delete clash set \\"{}\\"", self.id)', de: 'format!("Kollisionssatz \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "removes", before: rich(), mutation: { id: "cs-2" }, outcome: ok },
      { name: "keeps-the-issues-raised-from-it", before: rich(), mutation: { id: "cs-1" }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "cs-9" }, outcome: reject("mutation.target-missing", ["cs-9"]) },
    ]),
  },
  {
    kind: "create-rule", emoji: 0x2696, variant: "CreateRule", verb: "create", entity: "rule", binaryTag: 16003, displayName: "Create Rule",
    doc: "Brings a new numeric code check into the model: the measure and direction (minimum clear height, maximum riser, minimum tread, minimum stair width, minimum door width, maximum ramp slope, minimum corridor width, maximum compartment area), the limit, the severity and the scope. Its findings are inferred.",
    props: [id("identity", { en: "Rule id", de: "Regel-Id" }, "rule"), prop("rule", "Rule", ref("Rule"), "record", { en: "Rule", de: "Regel" }, 20)],
    label: { en: 'format!("Create rule \\"{}\\"", self.rule.name)', de: 'format!("Regel \\"{}\\" anlegen", self.rule.name)' },
    target,
    cases: cases([
      { name: "adds-a-riser-limit", before: scene(), mutation: { id: "r-1", rule: ruleRow() }, outcome: ok },
      { name: "adds-a-door-width", before: scene(), mutation: { id: "r-1", rule: ruleRow({ name: "Minimum door width", kind: "MinDoorWidth", limit: 0.9, severity: "Warning", scope: scopeRow({ storeys: ["st-ground"] }) }) }, outcome: ok },
      { name: "adds-a-ramp-slope", before: scene(), mutation: { id: "r-1", rule: ruleRow({ name: "Ramp slope 1:12", kind: "MaxRampSlope", limit: 0.0833, severity: "Error" }) }, outcome: ok },
      { name: "adds-a-corridor-width", before: scene(), mutation: { id: "r-1", rule: ruleRow({ name: "Corridor width", kind: "MinCorridorWidth", limit: 1.2, severity: "Warning", scope: scopeRow({ filter: "Corridor" }) }) }, outcome: ok },
      { name: "adds-a-compartment-area", before: scene(), mutation: { id: "r-1", rule: ruleRow({ name: "Fire compartment", kind: "MaxCompartmentArea", limit: 400, severity: "Error", scope: scopeRow({ filter: "Fire compartment" }) }) }, outcome: ok },
      { name: "adds-a-note-for-one-element", before: scene(), mutation: { id: "r-1", rule: ruleRow({ name: "Clear height", kind: "MinClearHeight", limit: 2.4, severity: "Note", scope: scopeRow({ phases: ["New"] }) }) }, outcome: ok },
      { name: "duplicate-id", before: withRule(), mutation: { id: "r-1", rule: ruleRow({ name: "Other" }) }, outcome: reject("mutation.duplicate-id", ["r-1"]) },
      { name: "id-taken-by-another-kind", before: scene(), mutation: { id: "w-south", rule: ruleRow() }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "blank-name", before: scene(), mutation: { id: "r-1", rule: ruleRow({ name: "" }) }, outcome: reject("mutation.invariant", ["rule", "name"]) },
      { name: "zero-limit", before: scene(), mutation: { id: "r-1", rule: ruleRow({ limit: 0 }) }, outcome: reject("mutation.invariant", ["rule", "limit"]) },
      { name: "slope-too-steep", before: scene(), mutation: { id: "r-1", rule: ruleRow({ kind: "MaxRampSlope", limit: 1.5 }) }, outcome: reject("mutation.invariant", ["rule", "limit"]) },
      { name: "storey-missing", before: scene(), mutation: { id: "r-1", rule: ruleRow({ scope: scopeRow({ storeys: ["st-attic"] }) }) }, outcome: reject("mutation.target-missing", ["rule", "scope"]) },
      { name: "element-missing", before: scene(), mutation: { id: "r-1", rule: ruleRow({ scope: scopeRow({ ids: ["w-9"] }) }) }, outcome: reject("mutation.target-missing", ["rule", "scope"]) },
    ]),
  },
  {
    kind: "set-rule", emoji: 0x1f6a6, variant: "SetRule", verb: "set", entity: "rule", binaryTag: 16004, displayName: "Set Rule",
    doc: "Sparsely changes a rule: its name, kind, limit, severity and scope (the scope is replaced as a whole). The findings are inferred and follow.",
    props: [
      id("target", { en: "Rule", de: "Regel" }, "rule"),
      prop("name", "Option<String>", { type: "string" }, "text", { en: "Name", de: "Name" }, 20),
      prop("kind", "Option<RuleKind>", ref("RuleKind"), "select", { en: "Check", de: "Prüfung" }, 30),
      prop("limit", "Option<f64>", { type: "number", exclusiveMinimum: 0 }, "number", { en: "Limit", de: "Grenzwert" }, 40),
      prop("severity", "Option<RuleSeverity>", ref("RuleSeverity"), "select", { en: "Severity", de: "Schwere" }, 50),
      prop("scope", "Option<RuleScope>", ref("RuleScope"), "record", { en: "Scope", de: "Geltungsbereich" }, 60),
    ],
    uses: [MUTATION_USES["set-rule"]],
    label: { en: 'format!("Change rule \\"{}\\"", self.id)', de: 'format!("Regel \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "renames", before: rich(), mutation: { id: "r-1", name: "Maximum riser height" }, outcome: ok },
      { name: "tightens-the-limit", before: rich(), mutation: { id: "r-1", limit: 0.18 }, outcome: ok },
      { name: "changes-the-check", before: rich(), mutation: { id: "r-1", kind: "MinTread", limit: 0.26 }, outcome: ok },
      { name: "softens-the-severity", before: rich(), mutation: { id: "r-1", severity: "Warning" }, outcome: ok },
      { name: "narrows-the-scope", before: rich(), mutation: { id: "r-1", scope: scopeRow({ storeys: ["st-ground"], phases: ["New"] }) }, outcome: ok },
      { name: "restates-an-unchanged-field", before: rich(), mutation: { id: "r-1", name: "Maximum riser", limit: 0.2 }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "r-9", limit: 1 }, outcome: reject("mutation.target-missing", ["r-9"]) },
      { name: "unchanged", before: rich(), mutation: { id: "r-1", kind: "MaxRiser", severity: "Error" }, outcome: reject("mutation.no-op", ["r-1"]) },
      { name: "empty-patch", before: rich(), mutation: { id: "r-1" }, outcome: reject("mutation.no-op", ["r-1"]) },
      { name: "blank-name", before: rich(), mutation: { id: "r-1", name: " " }, outcome: reject("mutation.invariant", ["name"]) },
      { name: "zero-limit", before: rich(), mutation: { id: "r-1", limit: 0 }, outcome: reject("mutation.invariant", ["limit"]) },
      { name: "slope-too-steep", before: rich(), mutation: { id: "r-1", kind: "MaxRampSlope", limit: 1.5 }, outcome: reject("mutation.invariant", ["limit"]) },
      { name: "storey-missing", before: rich(), mutation: { id: "r-1", scope: scopeRow({ storeys: ["st-attic"] }) }, outcome: reject("mutation.target-missing", ["scope"]) },
    ]),
  },
  {
    kind: "delete-rule", emoji: 0x2757, variant: "DeleteRule", verb: "delete", entity: "rule", binaryTag: 16005, displayName: "Delete Rule",
    doc: "Removes a rule together with its properties and classifications.",
    props: [id("target", { en: "Rule", de: "Regel" }, "rule")],
    inverseRows: { bounded: 4096 },
    label: { en: 'format!("Delete rule \\"{}\\"", self.id)', de: 'format!("Regel \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "removes", before: rich(), mutation: { id: "r-2" }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "r-9" }, outcome: reject("mutation.target-missing", ["r-9"]) },
    ]),
  },
  {
    kind: "create-issue", emoji: 0x1f6a9, variant: "CreateIssue", verb: "create", entity: "issue", binaryTag: 16006, displayName: "Create Issue",
    doc: "Raises an issue (a BCF topic): title, description, status, priority, assignee, author, the moment it was raised, labels, the elements it concerns, the clash it was raised from and the viewpoint (orbit camera, section box, isolated elements) that shows it.",
    props: [id("identity", { en: "Issue id", de: "Hinweis-Id" }, "issue"), prop("issue", "Issue", ref("Issue"), "record", { en: "Issue", de: "Hinweis" }, 20)],
    label: { en: 'format!("Raise issue \\"{}\\"", self.issue.title)', de: 'format!("Hinweis \\"{}\\" erfassen", self.issue.title)' },
    target,
    cases: cases([
      { name: "raises-an-issue", before: scene(), mutation: { id: "i-1", issue: issueRow() }, outcome: ok },
      { name: "raises-one-from-a-clash", before: withSet(), mutation: { id: "i-1", issue: issueRow({ clash: { set: "cs-1", first: "w-south", second: "w-east" } }) }, outcome: ok },
      { name: "raises-one-with-a-section-box", before: scene(), mutation: { id: "i-1", issue: issueRow({ viewpoint: viewpointRow({ section: { min: pt3(0, 0, 0), max: pt3(8, 6, 3) }, isolate: ["w-south"] }) }) }, outcome: ok },
      { name: "raises-a-bare-issue", before: scene(), mutation: { id: "i-1", issue: issueRow({ description: "", labels: [], elements: [], viewpoint: undefined, created: "2026-10-09" }) }, outcome: ok },
      { name: "duplicate-id", before: withIssue(), mutation: { id: "i-1", issue: issueRow({ title: "Other" }) }, outcome: reject("mutation.duplicate-id", ["i-1"]) },
      { name: "id-taken-by-another-kind", before: scene(), mutation: { id: "w-south", issue: issueRow() }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "blank-title", before: scene(), mutation: { id: "i-1", issue: issueRow({ title: " " }) }, outcome: reject("mutation.invariant", ["issue", "title"]) },
      { name: "blank-author", before: scene(), mutation: { id: "i-1", issue: issueRow({ author: "" }) }, outcome: reject("mutation.invariant", ["issue", "author"]) },
      { name: "unreadable-date", before: scene(), mutation: { id: "i-1", issue: issueRow({ created: "9 October" }) }, outcome: reject("mutation.invariant", ["issue", "created"]) },
      { name: "label-twice", before: scene(), mutation: { id: "i-1", issue: issueRow({ labels: ["Clash", "Clash"] }) }, outcome: reject("mutation.invariant", ["issue", "labels"]) },
      { name: "element-missing", before: scene(), mutation: { id: "i-1", issue: issueRow({ elements: ["w-9"] }) }, outcome: reject("mutation.target-missing", ["issue", "elements"]) },
      { name: "clash-set-missing", before: scene(), mutation: { id: "i-1", issue: issueRow({ clash: { set: "cs-9", first: "w-south", second: "w-east" } }) }, outcome: reject("mutation.target-missing", ["issue", "clash"]) },
      { name: "clash-with-itself", before: withSet(), mutation: { id: "i-1", issue: issueRow({ clash: { set: "cs-1", first: "w-south", second: "w-south" } }) }, outcome: reject("mutation.invariant", ["issue", "clash"]) },
      { name: "camera-without-distance", before: scene(), mutation: { id: "i-1", issue: issueRow({ viewpoint: viewpointRow({ camera: { ...camera, distance: 0 } }) }) }, outcome: reject("mutation.invariant", ["issue", "viewpoint"]) },
      { name: "inverted-section-box", before: scene(), mutation: { id: "i-1", issue: issueRow({ viewpoint: viewpointRow({ section: { min: pt3(0, 0, 3), max: pt3(8, 6, 3) } }) }) }, outcome: reject("mutation.invariant", ["issue", "viewpoint"]) },
      { name: "isolated-element-missing", before: scene(), mutation: { id: "i-1", issue: issueRow({ viewpoint: viewpointRow({ isolate: ["w-9"] }) }) }, outcome: reject("mutation.target-missing", ["issue", "viewpoint"]) },
    ]),
  },
  {
    kind: "set-issue", emoji: 0x1f3c1, variant: "SetIssue", verb: "set", entity: "issue", binaryTag: 16007, displayName: "Set Issue",
    doc: "Sparsely changes an issue: title, description, status, priority, assignee, author, creation moment, labels, elements, the clash it was raised from (an assigned null clears it) and the viewpoint (an assigned null clears it).",
    props: [
      id("target", { en: "Issue", de: "Hinweis" }, "issue"),
      prop("title", "Option<String>", { type: "string" }, "text", { en: "Title", de: "Titel" }, 20),
      prop("description", "Option<String>", { type: "string" }, "text", { en: "Description", de: "Beschreibung" }, 30),
      prop("status", "Option<IssueStatus>", ref("IssueStatus"), "select", { en: "Status", de: "Status" }, 40),
      prop("priority", "Option<IssuePriority>", ref("IssuePriority"), "select", { en: "Priority", de: "Priorität" }, 50),
      prop("assignee", "Option<String>", { type: "string" }, "text", { en: "Assignee", de: "Zuständig" }, 60),
      prop("author", "Option<String>", { type: "string" }, "text", { en: "Author", de: "Verfasser" }, 70),
      prop("created", "Option<String>", { type: "string" }, "text", { en: "Raised (date and time)", de: "Erfasst (Datum und Zeit)" }, 80),
      prop("labels", "Option<Vec<String>>", strings, "list", { en: "Labels", de: "Stichwörter" }, 90),
      prop("elements", "Option<Vec<String>>", strings, "list", { en: "Elements", de: "Elemente" }, 100),
      prop("clash", "Option<Assigned<Option<ClashRef>>>", assigned(ref("ClashRef")), "record", { en: "Clash (empty = none)", de: "Kollision (leer = keine)" }, 110),
      prop("viewpoint", "Option<Assigned<Option<IssueViewpoint>>>", assigned(ref("IssueViewpoint")), "record", { en: "Viewpoint (empty = none)", de: "Ansicht (leer = keine)" }, 120),
    ],
    uses: [MUTATION_USES["set-issue"]],
    label: { en: 'format!("Change issue \\"{}\\"", self.id)', de: 'format!("Hinweis \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "retitles", before: rich(), mutation: { id: "i-1", title: "Beam runs through the south wall" }, outcome: ok },
      { name: "assigns", before: rich(), mutation: { id: "i-1", assignee: "AB" }, outcome: ok },
      { name: "resolves", before: rich(), mutation: { id: "i-1", status: "Resolved" }, outcome: ok },
      { name: "raises-the-priority", before: rich(), mutation: { id: "i-1", priority: "Critical" }, outcome: ok },
      { name: "relabels", before: rich(), mutation: { id: "i-2", labels: ["Door", "Accessibility"] }, outcome: ok },
      { name: "adds-an-element", before: rich(), mutation: { id: "i-2", elements: ["w-east", "w-south"] }, outcome: ok },
      { name: "sets-the-viewpoint", before: rich(), mutation: { id: "i-2", viewpoint: { value: viewpointRow({ section: { min: pt3(0, 0, 0), max: pt3(8, 6, 3) }, isolate: ["w-east"] }) } }, outcome: ok },
      { name: "clears-the-viewpoint", before: rich(), mutation: { id: "i-1", viewpoint: { value: null } }, outcome: ok },
      { name: "links-a-clash", before: rich(), mutation: { id: "i-1", clash: { value: { set: "cs-1", first: "w-south", second: "w-east" } } }, outcome: ok },
      { name: "clears-the-clash", before: rich(), mutation: { id: "i-2", clash: { value: null } }, outcome: ok },
      { name: "restates-an-unchanged-field", before: rich(), mutation: { id: "i-1", title: "Beam hits the south wall", status: "Closed" }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "i-9", title: "Gone" }, outcome: reject("mutation.target-missing", ["i-9"]) },
      { name: "unchanged", before: rich(), mutation: { id: "i-1", status: "Open", assignee: "" }, outcome: reject("mutation.no-op", ["i-1"]) },
      { name: "empty-patch", before: rich(), mutation: { id: "i-1" }, outcome: reject("mutation.no-op", ["i-1"]) },
      { name: "blank-title", before: rich(), mutation: { id: "i-1", title: "" }, outcome: reject("mutation.invariant", ["title"]) },
      { name: "unreadable-date", before: rich(), mutation: { id: "i-1", created: "yesterday" }, outcome: reject("mutation.invariant", ["created"]) },
      { name: "element-missing", before: rich(), mutation: { id: "i-1", elements: ["w-9"] }, outcome: reject("mutation.target-missing", ["elements"]) },
      { name: "clash-set-missing", before: rich(), mutation: { id: "i-1", clash: { value: { set: "cs-9", first: "w-south", second: "w-east" } } }, outcome: reject("mutation.target-missing", ["clash"]) },
    ]),
  },
  {
    kind: "delete-issue", emoji: 0x1f4e2, variant: "DeleteIssue", verb: "delete", entity: "issue", binaryTag: 16008, displayName: "Delete Issue",
    doc: "Removes an issue together with its comments, its properties and its classifications. The elements and the clash it named are not touched.",
    props: [id("target", { en: "Issue", de: "Hinweis" }, "issue")],
    inverseRows: { bounded: 4096 },
    label: { en: 'format!("Delete issue \\"{}\\"", self.id)', de: 'format!("Hinweis \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "cascades-its-comments", before: rich(), mutation: { id: "i-1" }, outcome: ok },
      { name: "removes-an-issue-without-comments", before: withIssue(), mutation: { id: "i-1" }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "i-9" }, outcome: reject("mutation.target-missing", ["i-9"]) },
    ]),
  },
  {
    kind: "create-issue-comment", emoji: 0x1f4ac, variant: "CreateIssueComment", verb: "create", entity: "issue-comment", binaryTag: 16009, displayName: "Create Issue Comment",
    doc: "Adds a comment to an issue: who wrote it, when and what.",
    props: [id("identity", { en: "Comment id", de: "Kommentar-Id" }, "issue-comment"), prop("issue_comment", "IssueComment", ref("IssueComment"), "record", { en: "Comment", de: "Kommentar" }, 20)],
    label: { en: 'format!("Comment on issue \\"{}\\"", self.issue_comment.issue)', de: 'format!("Hinweis \\"{}\\" kommentieren", self.issue_comment.issue)' },
    target,
    cases: cases([
      { name: "adds-a-comment", before: withIssue(), mutation: { id: "c-1", issue_comment: commentRow() }, outcome: ok },
      { name: "adds-a-reply", before: withComments(), mutation: { id: "c-3", issue_comment: commentRow({ author: "UG", date: "2026-10-09T11:30:00+02:00", text: "Fixed in the next revision." }) }, outcome: ok },
      { name: "adds-a-dated-comment", before: withIssue(), mutation: { id: "c-1", issue_comment: commentRow({ date: "2026-10-10" }) }, outcome: ok },
      { name: "duplicate-id", before: withComments(), mutation: { id: "c-1", issue_comment: commentRow() }, outcome: reject("mutation.duplicate-id", ["c-1"]) },
      { name: "id-taken-by-another-kind", before: withIssue(), mutation: { id: "w-south", issue_comment: commentRow() }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "issue-missing", before: withIssue(), mutation: { id: "c-1", issue_comment: commentRow({ issue: "i-9" }) }, outcome: reject("mutation.target-missing", ["issue_comment", "issue"]) },
      { name: "blank-author", before: withIssue(), mutation: { id: "c-1", issue_comment: commentRow({ author: " " }) }, outcome: reject("mutation.invariant", ["issue_comment", "author"]) },
      { name: "blank-text", before: withIssue(), mutation: { id: "c-1", issue_comment: commentRow({ text: "" }) }, outcome: reject("mutation.invariant", ["issue_comment", "text"]) },
      { name: "unreadable-date", before: withIssue(), mutation: { id: "c-1", issue_comment: commentRow({ date: "tomorrow" }) }, outcome: reject("mutation.invariant", ["issue_comment", "date"]) },
    ]),
  },
  {
    kind: "set-issue-comment", emoji: 0x1f5e8, variant: "SetIssueComment", verb: "set", entity: "issue-comment", binaryTag: 16010, displayName: "Set Issue Comment",
    doc: "Sparsely changes a comment: its author, its moment and its text. The issue of a comment never changes.",
    props: [
      id("target", { en: "Comment", de: "Kommentar" }, "issue-comment"),
      prop("author", "Option<String>", { type: "string" }, "text", { en: "Author", de: "Verfasser" }, 20),
      prop("date", "Option<String>", { type: "string" }, "text", { en: "Written (date and time)", de: "Geschrieben (Datum und Zeit)" }, 30),
      prop("text", "Option<String>", { type: "string" }, "text", { en: "Comment", de: "Kommentar" }, 40),
    ],
    uses: [MUTATION_USES["set-issue-comment"]],
    label: { en: 'format!("Change comment \\"{}\\"", self.id)', de: 'format!("Kommentar \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "edits-the-text", before: rich(), mutation: { id: "c-1", text: "I will check the structure tomorrow." }, outcome: ok },
      { name: "re-dates", before: rich(), mutation: { id: "c-1", date: "2026-10-09T09:15:00Z" }, outcome: ok },
      { name: "re-signs", before: rich(), mutation: { id: "c-1", author: "MK" }, outcome: ok },
      { name: "restates-an-unchanged-field", before: rich(), mutation: { id: "c-1", author: "AB", text: "Edited." }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "c-9", text: "Gone" }, outcome: reject("mutation.target-missing", ["c-9"]) },
      { name: "unchanged", before: rich(), mutation: { id: "c-1", author: "AB" }, outcome: reject("mutation.no-op", ["c-1"]) },
      { name: "empty-patch", before: rich(), mutation: { id: "c-1" }, outcome: reject("mutation.no-op", ["c-1"]) },
      { name: "blank-text", before: rich(), mutation: { id: "c-1", text: "  " }, outcome: reject("mutation.invariant", ["text"]) },
      { name: "blank-author", before: rich(), mutation: { id: "c-1", author: "" }, outcome: reject("mutation.invariant", ["author"]) },
      { name: "unreadable-date", before: rich(), mutation: { id: "c-1", date: "later" }, outcome: reject("mutation.invariant", ["date"]) },
    ]),
  },
  {
    kind: "delete-issue-comment", emoji: 0x1f4ad, variant: "DeleteIssueComment", verb: "delete", entity: "issue-comment", binaryTag: 16011, displayName: "Delete Issue Comment",
    doc: "Removes a comment of an issue, together with its properties and classifications.",
    props: [id("target", { en: "Comment", de: "Kommentar" }, "issue-comment")],
    inverseRows: { bounded: 4096 },
    label: { en: 'format!("Delete comment \\"{}\\"", self.id)', de: 'format!("Kommentar \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "removes", before: rich(), mutation: { id: "c-2" }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "c-9" }, outcome: reject("mutation.target-missing", ["c-9"]) },
    ]),
  },
];

for (const leaf of leaves) for (const row of leaf.cases) {
  row.before = clean(row.before as Record<string, unknown>);
  row.mutation = clean(row.mutation);
}

const OPTIONAL: Record<string, string[]> = {
  "set-clash-set": ["name", "a", "b", "tolerance", "clearance"],
  "set-rule": ["name", "kind", "limit", "severity", "scope"],
  "set-issue": ["title", "description", "status", "priority", "assignee", "author", "created", "labels", "elements", "clash", "viewpoint"],
  "set-issue-comment": ["author", "date", "text"],
};

const COPY = new Set(["tolerance", "clearance", "kind", "limit", "severity", "status", "priority"]);
const patchBody = (fields: string[], from: string) => fields.map((field) => (COPY.has(field) ? `${field}: ${from}.${field}` : `${field}: ${from}.${field}.clone()`)).join(", ");

const patchImpl = (variant: string, patch: string, fields: string[], rest = "") => `impl ${variant} {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ${patch} {
        ${patch} { ${patchBody(fields, "self")}${rest} }
    }

    /// 🧩 The payload that provides exactly the fields \`patch\` names.
    pub fn from_patch(id: String, patch: ${patch}) -> Self {
        Self { id, ${fields.map((field) => `${field}: patch.${field}`).join(", ")} }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for ${variant} {`;

const createDiff = (variant: string, noun: string, collection: string, field: string, problem: string) => `//! 🔺️ Diff constructor for \`${variant}\`: one created ${noun} entry. The id must be free across every collection, then the ${noun} must be writable (see \`${problem}\`).

use super::super::elements;
use super::${variant};
use crate::{${problem}, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &${variant}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = ${problem}(base, &payload.id, &payload.${field}) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, ["${field}", problem.field]);
    }
    MutationOutcome::new(ModelDiff::${collection}(payload.id.clone(), Entry::Created(payload.${field}.clone())))
}
`;

const createInverse = (variant: string, deleteVariant: string, deleteModule: string, collection: string) => `//! ↩️ Inverse of \`${variant}\`: the concrete \`${deleteVariant}\` of the id it created, none when the id was already taken.

use super::super::${deleteModule}::${deleteVariant};
use super::${variant};
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &${variant}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.${collection}.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::${deleteVariant}(${deleteVariant} { id: payload.id.clone() })]
}
`;

const setDiff = (variant: string, noun: string, nounCap: string, collection: string, problem: string) => `//! 🔺️ Diff constructor for \`${variant}\`: a sparse ${noun} patch of exactly the provided fields that differ. The ${noun} that results must be writable (see \`${problem}\`); a patch that restates the current values is
//! \`mutation.no-op\`.

use super::${variant};
use crate::{${problem}, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &${variant}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.${collection}.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("${nounCap} \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let patch = payload.patch().minimal(record);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("${nounCap} \\"{}\\" already holds these values.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = ${problem}(base, &payload.id, &patch.write(record)) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, [problem.field]);
    }
    MutationOutcome::new(ModelDiff::${collection}(payload.id.clone(), Entry::Patched(patch)))
}
`;

const setInverse = (variant: string, noun: string, collection: string) => `//! ↩️ Inverse of \`${variant}\`: an absolute \`${variant}\` restoring the base value of exactly the fields the forward really changes, none when the ${noun} is absent or nothing changes.

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

const deleteDiff = (variant: string, noun: string, nounCap: string, collection: string, tail: string) => `//! 🔺️ Diff constructor for \`${variant}\`: the ${noun} leaves in one sparse diff together with ${tail}its properties and classifications (see the shared cascade).

use super::super::cascade;
use super::${variant};
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &${variant}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.${collection}.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("${nounCap} \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "${nounCap}", Some(&payload.id))
}
`;

const deleteInverse = (variant: string) => `//! ↩️ Inverse of \`${variant}\`: one concrete create per removed record and one setter per removed property or classification, in storage order (dependants first, the target last), so the store, which replays the
//! vector reversed, recreates the target before anything on it.

use super::super::cascade;
use super::${variant};
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &${variant}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
`;

const create = (variant: string, noun: string, collection: string, field: string, problem: string, deleteVariant: string, deleteModule: string) => ({ diff: createDiff(variant, noun, collection, field, problem), inverse: createInverse(variant, deleteVariant, deleteModule, collection) });
const remove = (variant: string, noun: string, nounCap: string, collection: string, tail = "") => ({ diff: deleteDiff(variant, noun, nounCap, collection, tail), inverse: deleteInverse(variant) });

const MODULES: Record<string, { diff: string; inverse: string; mutation?: (source: string) => string }> = {
  "create-clash-set": create("CreateClashSet", "clash set", "clash_sets", "clash_set", "clash_set_problem", "DeleteClashSet", "delete_clash_set"),
  "set-clash-set": { diff: setDiff("SetClashSet", "clash set", "Clash set", "clash_sets", "clash_set_problem"), inverse: setInverse("SetClashSet", "clash set", "clash_sets"), mutation: (source) => source.replace("impl MutationKind<ModelSnapshot, ModelMutation> for SetClashSet {", patchImpl("SetClashSet", "ClashSetPatch", OPTIONAL["set-clash-set"])) },
  "delete-clash-set": remove("DeleteClashSet", "clash set", "Clash set", "clash_sets"),
  "create-rule": create("CreateRule", "rule", "rules", "rule", "rule_problem", "DeleteRule", "delete_rule"),
  "set-rule": { diff: setDiff("SetRule", "rule", "Rule", "rules", "rule_problem"), inverse: setInverse("SetRule", "rule", "rules"), mutation: (source) => source.replace("impl MutationKind<ModelSnapshot, ModelMutation> for SetRule {", patchImpl("SetRule", "RulePatch", OPTIONAL["set-rule"])) },
  "delete-rule": remove("DeleteRule", "rule", "Rule", "rules"),
  "create-issue": create("CreateIssue", "issue", "issues", "issue", "issue_problem", "DeleteIssue", "delete_issue"),
  "set-issue": { diff: setDiff("SetIssue", "issue", "Issue", "issues", "issue_problem"), inverse: setInverse("SetIssue", "issue", "issues"), mutation: (source) => source.replace("impl MutationKind<ModelSnapshot, ModelMutation> for SetIssue {", patchImpl("SetIssue", "IssuePatch", OPTIONAL["set-issue"])) },
  "delete-issue": remove("DeleteIssue", "issue", "Issue", "issues", "its comments, "),
  "create-issue-comment": create("CreateIssueComment", "comment", "issue_comments", "issue_comment", "comment_problem", "DeleteIssueComment", "delete_issue_comment"),
  "set-issue-comment": { diff: setDiff("SetIssueComment", "comment", "Comment", "issue_comments", "comment_problem"), inverse: setInverse("SetIssueComment", "comment", "issue_comments"), mutation: (source) => source.replace("impl MutationKind<ModelSnapshot, ModelMutation> for SetIssueComment {", patchImpl("SetIssueComment", "IssueCommentPatch", OPTIONAL["set-issue-comment"], ", ..Default::default()")) },
  "delete-issue-comment": remove("DeleteIssueComment", "comment", "Comment", "issue_comments"),
};

const dir = (spec: Leaf) => join(mutations, em(spec.emoji) + spec.kind);
const sub = (spec: Leaf, emojiPoint: number, name: string) => join(dir(spec), em(emojiPoint) + name);

const fixup = (spec: Leaf) => {
  const optional = OPTIONAL[spec.kind] ?? [];
  const component = join(sub(spec, 0x1f9a0, "mutation"), RS);
  let source = readFileSync(component, "utf8");
  for (const name of optional) source = source.replace(new RegExp(`^    pub ${name}: Option<`, "m"), `    #[value(default, skip_serializing_if = "Option::is_none")]\n    pub ${name}: Option<`);
  source = MODULES[spec.kind].mutation?.(source) ?? source;
  writeFileSync(component, source);
  const schemaFile = join(sub(spec, 0x1f9ec, "schema"), JSONF);
  const schema = JSON.parse(readFileSync(schemaFile, "utf8"));
  schema.required = schema.required.filter((name: string) => !optional.includes(name));
  writeFileSync(schemaFile, JSON.stringify(schema, null, 2) + "\n");
  writeFileSync(join(sub(spec, 0x1f53a, "diff"), RS), MODULES[spec.kind].diff);
  writeFileSync(join(sub(spec, 0x21a9, "inverse"), RS), MODULES[spec.kind].inverse);
};

const upstairs = () => scene({ clash_sets: { "cs-1": clashSetRow({ a: sel({ storeys: ["st-first"] }) }), "cs-2": clashSetRow({ name: "Ground" , a: sel({ storeys: ["st-ground"] }) }) }, rules: { "r-1": ruleRow({ scope: scopeRow({ storeys: ["st-first"] }) }), "r-2": ruleRow({ name: "Everywhere" }) } });

const storeyCascade = {
  name: "cascades-clash-sets-and-rules-scoped-to-it",
  emoji: 0x1f9d5,
  before: clean(upstairs() as Record<string, unknown>),
  mutation: { id: "st-first" },
  outcome: ok,
};

if (import.meta.main) {
  const out = join(import.meta.dir, "🗑️generated", "w2-wp16-coordination");
  mkdirSync(out, { recursive: true });
  let mounts = "";
  for (const spec of leaves) {
    mounts += emitLeaf(spec);
    fixup(spec);
  }
  writeFileSync(join(out, "mounts.txt"), mounts);
  writeFileSync(join(out, "delete-storey-case.txt"), emitCase("delete-storey", em(0x1f6ae) + "delete-storey", "DeleteStorey", storeyCascade));
  console.log(`emitted ${leaves.length} leaves and 1 delete-storey case`);
}
