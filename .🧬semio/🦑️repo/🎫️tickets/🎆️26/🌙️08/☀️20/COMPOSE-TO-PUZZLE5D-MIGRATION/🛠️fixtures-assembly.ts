#!/usr/bin/env bun
/** 🧩️ Handcrafted mutation fixtures for `procedural`'s `assembly` artifact (9 leaves).
 *
 * Every `after` and every `diff` below was transcribed from that leaf's own
 * `🔺️diff/🦀️component.rs` — the guard order, the exact `AssemblyDiff` fields it sets, and the
 * cascade/no-op messages it attaches — never from the leaf's name.
 */
import { emitAll, f, type Case, type Tree } from "./🛠️emit-fixture.ts";

const tree: Tree = {
  mutationsRoot: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
  glue: "✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs",
  gluePrefix: "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
  diffPath: "crate::artifacts::assembly::diff",
  diffName: "AssemblyDiff",
  snapshotPath: "crate::artifacts::assembly::schema::snapshot",
  snapshotName: "AssemblySnapshot",
  mutationsPath: "crate::artifacts::assembly::mutations",
  mutationName: "AssemblyMutation",
  entry: "named",
  applyFn: "apply_assembly_mutation",
  inverseFn: "inverse_assembly_mutation",
};

//#region 🔖️Vocabulary
const slotA = { id: "slot-a", x: f(0), y: f(0), z: f(0) };
const slotB = { id: "slot-b", x: f(2), y: f(0), z: f(0) };
const slotC = { id: "slot-c", x: f(4), y: f(0), z: f(0) };
const edgeAb = { id: "edge-ab", fromSlotId: "slot-a", toSlotId: "slot-b" };
const edgeBc = { id: "edge-bc", fromSlotId: "slot-b", toSlotId: "slot-c" };
const moduleChild = (childId: string, artifactId: string) => ({
  childId,
  target: { artifactId, dialect: { artifactKind: "s.stdio.semio", standard: "1", subset: "kit" } },
});
const modules = [moduleChild("module-wall", "kit-wall"), moduleChild("module-roof", "kit-roof")];
const wallWeight = { moduleId: "module-wall", weight: f(1) };
const ruleWallRoof = { id: "rule-wall-roof", moduleAId: "module-wall", moduleBId: "module-roof", allowed: true, params: { kind: "null" } };
const ruleRoofWall = { id: "rule-roof-wall", moduleAId: "module-roof", moduleBId: "module-wall", allowed: false, params: { kind: "null" } };

type Assembly = { slots?: unknown[]; edges?: unknown[]; weights?: unknown[]; rules?: unknown[]; seed?: number };
const snapshot = (over: Assembly = {}) => ({
  schema: "s.assembly",
  seed: over.seed ?? 7,
  slots: over.slots ?? [slotA, slotB],
  edges: over.edges ?? [edgeAb],
  modules,
  weights: over.weights ?? [wallWeight],
  rules: over.rules ?? [ruleWallRoof],
});

type Delta = {
  schema?: string | null;
  seed?: number | null;
  slotsRemoved?: string[];
  slotsUpserted?: unknown[];
  edgesRemoved?: string[];
  edgesUpserted?: unknown[];
  weightsRemoved?: string[];
  weightsUpserted?: unknown[];
  rulesRemoved?: string[];
  rulesUpserted?: unknown[];
};
const delta = (over: Delta) => ({
  schema: over.schema ?? null,
  seed: over.seed ?? null,
  slotsRemoved: over.slotsRemoved ?? [],
  slotsUpserted: over.slotsUpserted ?? [],
  edgesRemoved: over.edgesRemoved ?? [],
  edgesUpserted: over.edgesUpserted ?? [],
  weightsRemoved: over.weightsRemoved ?? [],
  weightsUpserted: over.weightsUpserted ?? [],
  rulesRemoved: over.rulesRemoved ?? [],
  rulesUpserted: over.rulesUpserted ?? [],
});
const applied = { status: "applied" };
//#endregion 🔖️Vocabulary

const cases: readonly Case[] = [
  {
    leafDir: "🌱create-slot",
    leafSlug: "create-slot",
    caseName: "appends-slot-c-at-index-2",
    headline: "`create-slot`'s diff builder writes ONE `slots_upserted` entry carrying the payload's own final-state index; `edges`, `weights`, `rules` and `seed` are never touched.",
    before: snapshot(),
    after: snapshot({ slots: [slotA, slotB, slotC] }),
    mutation: { CreateSlot: { index: 2, slot: slotC } },
    diff: delta({ slotsUpserted: [[2, slotC]] }),
    outcome: applied,
  },
  {
    leafDir: "🗑️delete-slot",
    leafSlug: "delete-slot",
    caseName: "removes-slot-a-and-cascades-edge-ab",
    headline: "`delete-slot`'s diff builder removes the slot id AND every edge incident to it, and attaches an `mutation.cascade` info message naming the severed edges.",
    before: snapshot(),
    after: snapshot({ slots: [slotB], edges: [] }),
    mutation: { DeleteSlot: { id: "slot-a" } },
    diff: delta({ slotsRemoved: ["slot-a"], edgesRemoved: ["edge-ab"] }),
    outcome: { status: "applied", messages: [{ level: "info", code: "mutation.cascade" }] },
  },
  {
    leafDir: "🌱create-rule",
    leafSlug: "create-rule",
    caseName: "appends-a-rule-forbidding-roof-over-wall",
    headline: "`create-rule`'s diff builder writes ONE `rules_upserted` entry after checking that BOTH referenced module ids exist among the snapshot's owned `modules` children.",
    before: snapshot(),
    after: snapshot({ rules: [ruleWallRoof, ruleRoofWall] }),
    mutation: { CreateRule: { index: 1, rule: ruleRoofWall } },
    diff: delta({ rulesUpserted: [[1, ruleRoofWall]] }),
    outcome: applied,
  },
  {
    leafDir: "🗑️delete-rule",
    leafSlug: "delete-rule",
    caseName: "removes-the-wall-roof-rule",
    headline: "`delete-rule`'s diff builder removes exactly one id from `rules` — no cascade, no other collection.",
    before: snapshot(),
    after: snapshot({ rules: [] }),
    mutation: { DeleteRule: { id: "rule-wall-roof" } },
    diff: delta({ rulesRemoved: ["rule-wall-roof"] }),
    outcome: applied,
  },
  {
    leafDir: "🔢change-weight",
    leafSlug: "change-weight",
    caseName: "raises-the-wall-module-selection-bias",
    headline: "`change-weight`'s diff builder upserts an UNINDEXED `weights` row after checking the module exists and the value is finite and non-negative; the prior `1.0` row is replaced in place.",
    before: snapshot(),
    after: snapshot({ weights: [{ moduleId: "module-wall", weight: f(2.5) }] }),
    mutation: { ChangeWeight: { module_id: "module-wall", weight: f(2.5) } },
    diff: delta({ weightsUpserted: [{ moduleId: "module-wall", weight: f(2.5) }] }),
    outcome: applied,
  },
  {
    leafDir: "🗑️remove-weight",
    leafSlug: "remove-weight",
    caseName: "drops-the-wall-module-weight-override",
    headline: "`remove-weight`'s diff builder writes one `weights_removed` id, returning that module to the solver's neutral default bias; no other collection is written.",
    before: snapshot(),
    after: snapshot({ weights: [] }),
    mutation: { RemoveWeight: { module_id: "module-wall" } },
    diff: delta({ weightsRemoved: ["module-wall"] }),
    outcome: applied,
  },
  {
    leafDir: "🔗connect-slots",
    leafSlug: "connect-slots",
    caseName: "joins-slot-b-to-slot-c-at-index-1",
    headline: "`connect-slots`'s diff builder writes ONE `edges_upserted` entry after clearing four guards: duplicate edge id, both endpoints present, no self-loop, and no parallel edge in either direction.",
    before: snapshot({ slots: [slotA, slotB, slotC] }),
    after: snapshot({ slots: [slotA, slotB, slotC], edges: [edgeAb, edgeBc] }),
    mutation: { ConnectSlots: { index: 1, edge: edgeBc } },
    diff: delta({ edgesUpserted: [[1, edgeBc]] }),
    outcome: applied,
  },
  {
    leafDir: "✂️disconnect-slots",
    leafSlug: "disconnect-slots",
    caseName: "severs-edge-ab-leaving-both-slots",
    headline: "`disconnect-slots`'s diff builder removes only the edge id — the two slots it joined stay in `slots` untouched.",
    before: snapshot(),
    after: snapshot({ edges: [] }),
    mutation: { DisconnectSlots: { id: "edge-ab" } },
    diff: delta({ edgesRemoved: ["edge-ab"] }),
    outcome: applied,
  },
  {
    leafDir: "🎲change-seed",
    leafSlug: "change-seed",
    caseName: "reseeds-the-solve-from-7-to-99",
    headline: "`change-seed`'s diff builder sets the scalar `seed` field only — every id-keyed collection delta stays empty, so the WFC solve inference re-runs without the spec itself moving.",
    before: snapshot(),
    after: snapshot({ seed: 99 }),
    mutation: { ChangeSeed: { seed: 99 } },
    diff: delta({ seed: 99 }),
    outcome: applied,
  },
];

emitAll(tree, cases);
