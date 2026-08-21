#!/usr/bin/env bun
/** 🌀️ Handcrafted mutation fixtures for `procedural`'s `procedural2d` artifact (14 leaves).
 *
 * `Procedural2dDiff` is a WHOLE-FIELD delta, not an id-keyed one: every fixture-lane leaf routes
 * through `diff_fixture_from_helpers`, which folds its sparse helper into a cloned fixture and
 * publishes the RESULT as `fixture: Some(..)`; every generation-lane leaf routes through
 * `diff_generation_from_ops` and publishes `generation: Some(..)`. So each committed diff below
 * carries exactly one of those two fields — which one, and with what content, is transcribed from
 * that leaf's own `🔺️diff/🦀️component.rs`.
 */
import { emitAll, f, type Case, type Tree } from "./🛠️emit-fixture.ts";

const tree: Tree = {
  mutationsRoot: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
  glue: "✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs",
  gluePrefix: "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
  diffPath: "crate::artifacts::procedural2d::diff",
  diffName: "Procedural2dDiff",
  snapshotPath: "crate::artifacts::procedural2d",
  snapshotName: "Procedural2dSnapshot",
  mutationsPath: "crate::artifacts::procedural2d::mutations",
  mutationName: "Procedural2dMutation",
  entry: "named",
  applyFn: "apply_procedural2d_mutation",
  inverseFn: "inverse_procedural2d_mutation",
};

//#region 🔖️Vocabulary
const noteA = { kind: "inputNote", id: "note-a", text: "Alpha" };
const noteB = { kind: "inputNote", id: "note-b", text: "Beta" };
const noteC = { kind: "inputNote", id: "note-c", text: "Gamma" };
const linkAb = { id: "link-ab", from: "note-a", to: "note-b", fromPort: "out", toPort: "in" };
const linkBc = { id: "link-bc", from: "note-b", to: "note-c", fromPort: "out", toPort: "in" };
const homeCamera = { x: f(0), y: f(0), zoom: f(1) };
const homeLayout = { "note-a": { x: f(0), y: f(0) } };

type FixtureOver = { schema?: string; camera?: unknown; widgets?: unknown[]; synapses?: unknown[]; layout?: unknown };
const fixture = (over: FixtureOver = {}) => ({
  schema: over.schema ?? "flow.fixture",
  camera: over.camera ?? homeCamera,
  widgets: over.widgets ?? [noteA, noteB],
  synapses: over.synapses ?? [linkAb],
  layout: over.layout ?? homeLayout,
});

const genOne = { id: "gen-1", name: "Baseline", values: { height: f(3) } };
const genTwo = { id: "gen-2", name: "Taller", values: { height: f(5) } };
const play = (generations: unknown[], selected: string) => ({ generations, selectedGenerationId: selected });
const homePlay = play([genOne], "gen-1");

const snapshot = (fx: unknown = fixture(), generation: unknown = homePlay) => ({ fixture: fx, generation });

type Delta = { fixture?: unknown; generation?: unknown };
const delta = (over: Delta) => ({
  artifact: null,
  fixture: over.fixture ?? null,
  generation: over.generation ?? null,
  selectedIds: null,
  graphCamera: null,
  showMode: null,
  selectedGenerationId: null,
  generationPreviewText: null,
  locale: null,
});
const applied = { status: "applied" };
//#endregion 🔖️Vocabulary

//#region 🔖️FixtureLane
const threeNotes = fixture({ widgets: [noteA, noteB, noteC] });

const fixtureCases: readonly Case[] = [
  {
    leafDir: "🌱create-widget",
    leafSlug: "create-widget",
    caseName: "inserts-note-c-at-index-2",
    headline: "`create-widget`'s diff builder rejects a duplicate widget id, then publishes the fixture with the payload's widget spliced in at its final-state index; synapses and layout are carried through untouched.",
    before: snapshot(),
    after: snapshot(fixture({ widgets: [noteA, noteB, noteC] })),
    mutation: { CreateWidget: { index: 2, widget: noteC } },
    diff: delta({ fixture: fixture({ widgets: [noteA, noteB, noteC] }) }),
    outcome: applied,
  },
  {
    leafDir: "🔁replace-widget",
    leafSlug: "replace-widget",
    caseName: "rewrites-the-note-b-body-in-place",
    headline: "`replace-widget`'s diff builder resolves the widget's BASE index first and writes the replacement at that same position, so a replace never reorders the widget list.",
    before: snapshot(),
    after: snapshot(fixture({ widgets: [noteA, { kind: "inputNote", id: "note-b", text: "Beta, revised" }] })),
    mutation: { ReplaceWidget: { widget: { kind: "inputNote", id: "note-b", text: "Beta, revised" } } },
    diff: delta({ fixture: fixture({ widgets: [noteA, { kind: "inputNote", id: "note-b", text: "Beta, revised" }] }) }),
    outcome: applied,
  },
  {
    leafDir: "🗑️delete-widget",
    leafSlug: "delete-widget",
    caseName: "removes-note-a-and-flags-the-dangling-synapse",
    headline: "`delete-widget`'s diff builder removes ONLY the widget — it deliberately does not cascade, leaving `link-ab` and the `note-a` layout entry in place and raising an `mutation.cascade` info message that names the synapse now left dangling.",
    before: snapshot(),
    after: snapshot(fixture({ widgets: [noteB] })),
    mutation: { DeleteWidget: { id: "note-a" } },
    diff: delta({ fixture: fixture({ widgets: [noteB] }) }),
    outcome: { status: "applied", messages: [{ level: "info", code: "mutation.cascade" }] },
  },
  {
    leafDir: "🔗connect-synapse",
    leafSlug: "connect-synapse",
    caseName: "joins-note-b-to-note-c-at-index-1",
    headline: "`connect-synapse`'s diff builder clears four guards — duplicate synapse id, source widget present, target widget present, and no existing edge with the same from/from-port/to/to-port quadruple — before publishing the fixture with the new edge inserted.",
    before: snapshot(threeNotes),
    after: snapshot(fixture({ widgets: [noteA, noteB, noteC], synapses: [linkAb, linkBc] })),
    mutation: { ConnectSynapse: { index: 1, synapse: linkBc } },
    diff: delta({ fixture: fixture({ widgets: [noteA, noteB, noteC], synapses: [linkAb, linkBc] }) }),
    outcome: applied,
  },
  {
    leafDir: "🔄replace-synapse",
    leafSlug: "replace-synapse",
    caseName: "repoints-link-ab-onto-the-alt-port",
    headline: "`replace-synapse`'s diff builder resolves the edge's BASE index and overwrites that slot, so re-porting an edge keeps its position in the synapse list.",
    before: snapshot(),
    after: snapshot(fixture({ synapses: [{ id: "link-ab", from: "note-a", to: "note-b", fromPort: "out", toPort: "alt" }] })),
    mutation: { ReplaceSynapse: { synapse: { id: "link-ab", from: "note-a", to: "note-b", fromPort: "out", toPort: "alt" } } },
    diff: delta({ fixture: fixture({ synapses: [{ id: "link-ab", from: "note-a", to: "note-b", fromPort: "out", toPort: "alt" }] }) }),
    outcome: applied,
  },
  {
    leafDir: "✂️disconnect-synapse",
    leafSlug: "disconnect-synapse",
    caseName: "severs-link-ab-leaving-both-notes",
    headline: "`disconnect-synapse`'s diff builder drops the edge id from the synapse list only — both endpoint widgets and the whole layout map survive verbatim.",
    before: snapshot(),
    after: snapshot(fixture({ synapses: [] })),
    mutation: { DisconnectSynapse: { id: "link-ab" } },
    diff: delta({ fixture: fixture({ synapses: [] }) }),
    outcome: applied,
  },
  {
    leafDir: "📍move-widget",
    leafSlug: "move-widget",
    caseName: "repositions-note-a-on-the-canvas",
    headline: "`move-widget`'s diff builder checks the widget exists and the coordinates are finite, then writes ONE layout-map entry — the widget list itself is never rewritten by a move.",
    before: snapshot(),
    after: snapshot(fixture({ layout: { "note-a": { x: f(12.5), y: f(-4.25) } } })),
    mutation: { MoveWidget: { id: "note-a", layout: { x: f(12.5), y: f(-4.25) } } },
    diff: delta({ fixture: fixture({ layout: { "note-a": { x: f(12.5), y: f(-4.25) } } }) }),
    outcome: applied,
  },
  {
    leafDir: "🧹clear-widget-layout",
    leafSlug: "clear-widget-layout",
    caseName: "drops-the-note-a-layout-entry",
    headline: "`clear-widget-layout`'s diff builder removes the widget's layout-map key while the widget itself stays in the fixture — an unpositioned widget, not a deleted one.",
    before: snapshot(),
    after: snapshot(fixture({ layout: {} })),
    mutation: { ClearWidgetLayout: { id: "note-a" } },
    diff: delta({ fixture: fixture({ layout: {} }) }),
    outcome: applied,
  },
  {
    leafDir: "🎛set-camera",
    leafSlug: "set-camera",
    caseName: "pans-and-zooms-the-graph-camera",
    headline: "`set-camera`'s `UpdateCamera` diff builder rejects non-finite x/y/zoom and no-ops on an unchanged camera; here it publishes the fixture with only its `camera` field moved — widgets, synapses and layout are byte-identical.",
    before: snapshot(),
    after: snapshot(fixture({ camera: { x: f(8), y: f(-2), zoom: f(2) } })),
    mutation: { UpdateCamera: { camera: { x: f(8), y: f(-2), zoom: f(2) } } },
    diff: delta({ fixture: fixture({ camera: { x: f(8), y: f(-2), zoom: f(2) } }) }),
    outcome: applied,
  },
  {
    leafDir: "🔤change-schema",
    leafSlug: "change-schema",
    caseName: "restamps-the-fixture-schema",
    headline: "`change-schema`'s diff builder no-ops when the schema already matches; here it publishes the fixture with only its `schema` string restamped, leaving every widget, synapse and layout entry alone.",
    before: snapshot(),
    after: snapshot(fixture({ schema: "flow.fixture.revised" })),
    mutation: { ChangeSchema: { schema: "flow.fixture.revised" } },
    diff: delta({ fixture: fixture({ schema: "flow.fixture.revised" }) }),
    outcome: applied,
  },
];
//#endregion 🔖️FixtureLane

//#region 🔖️GenerationLane
const generationCases: readonly Case[] = [
  {
    leafDir: "➕create-generation",
    leafSlug: "create-generation",
    caseName: "appends-generation-2-and-selects-it",
    headline: "`create-generation`'s diff builder rejects a duplicate generation id, then folds a `GenerationMutation::Add` into the play state — which both appends the generation AND moves the selection onto it, so the committed diff carries the new `selectedGenerationId` too.",
    before: snapshot(fixture(), homePlay),
    after: snapshot(fixture(), play([genOne, genTwo], "gen-2")),
    mutation: { CreateGeneration: { generation: genTwo } },
    diff: delta({ generation: play([genOne, genTwo], "gen-2") }),
    outcome: applied,
  },
  {
    leafDir: "➖delete-generation",
    leafSlug: "delete-generation",
    caseName: "removes-the-selected-generation-2-and-falls-back-to-generation-1",
    headline: "`delete-generation`'s diff builder folds a `GenerationMutation::Remove` in — and because the removed generation was the SELECTED one, the play state re-points the selection at the first surviving generation, which the committed diff must therefore carry.",
    before: snapshot(fixture(), play([genOne, genTwo], "gen-2")),
    after: snapshot(fixture(), play([genOne], "gen-1")),
    mutation: { DeleteGeneration: { id: "gen-2" } },
    diff: delta({ generation: play([genOne], "gen-1") }),
    outcome: applied,
  },
  {
    leafDir: "🏷️rename-generation",
    leafSlug: "rename-generation",
    caseName: "retitles-generation-1",
    headline: "`rename-generation`'s diff builder rejects a missing id and no-ops on an identical name; here it rewrites only that generation's `name`, leaving its answer `values` and the selection untouched.",
    before: snapshot(fixture(), homePlay),
    after: snapshot(fixture(), play([{ id: "gen-1", name: "Baseline, revised", values: { height: f(3) } }], "gen-1")),
    mutation: { RenameGeneration: { id: "gen-1", name: "Baseline, revised" } },
    diff: delta({ generation: play([{ id: "gen-1", name: "Baseline, revised", values: { height: f(3) } }], "gen-1") }),
    outcome: applied,
  },
  {
    leafDir: "🔢change-generation-value",
    leafSlug: "change-generation-value",
    caseName: "raises-the-height-answer-in-generation-1",
    headline: "`change-generation-value`'s diff builder no-ops when the answer already equals the payload value; here it upserts ONE key of that generation's `values` map, leaving its name and the selection as they were.",
    before: snapshot(fixture(), homePlay),
    after: snapshot(fixture(), play([{ id: "gen-1", name: "Baseline", values: { height: f(4.5) } }], "gen-1")),
    mutation: { ChangeGenerationValue: { id: "gen-1", question_id: "height", value: f(4.5) } },
    diff: delta({ generation: play([{ id: "gen-1", name: "Baseline", values: { height: f(4.5) } }], "gen-1") }),
    outcome: applied,
  },
];
//#endregion 🔖️GenerationLane

emitAll(tree, [...fixtureCases, ...generationCases]);
