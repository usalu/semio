#!/usr/bin/env bun
/** 🧊️ Handcrafted mutation fixtures for `procedural`'s `procedural3d` artifact (14 leaves).
 *
 * Sibling of `procedural2d` but NOT a copy of it: every payload here carries
 * `#[serde(rename_all = "camelCase")]` (so `new_schema` is `newSchema`, `question_id` is
 * `questionId`, `new_value` is `newValue`), `delete-widget` raises NO cascade message, and
 * `connect-synapse` has three guards instead of four. Each `after`/`diff` below was transcribed
 * from this artifact's own leaf `🔺️diff/🦀️component.rs`, not from its 2d twin.
 */
import { emitAll, f, type Case, type Tree } from "./🛠️emit-fixture.ts";

const tree: Tree = {
  mutationsRoot: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
  glue: "✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs",
  gluePrefix: "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
  diffPath: "crate::artifacts::procedural3d::diff",
  diffName: "Procedural3dDiff",
  snapshotPath: "crate::artifacts::procedural3d",
  snapshotName: "Procedural3dSnapshot",
  mutationsPath: "crate::artifacts::procedural3d::mutations",
  mutationName: "Procedural3dMutation",
  entry: "named",
  applyFn: "apply_procedural3d_mutation",
  inverseFn: "inverse_procedural3d_mutation",
};

//#region 🔖️Vocabulary
const nodeA = { kind: "inputNote", id: "node-a", text: "Base plate" };
const nodeB = { kind: "inputNote", id: "node-b", text: "Column" };
const nodeC = { kind: "inputNote", id: "node-c", text: "Capital" };
const knob = { kind: "inputSlider", id: "knob", value: f(3), min: f(0), max: f(10), step: f(0.5) };
const knobRaised = { kind: "inputSlider", id: "knob", value: f(6.5), min: f(0), max: f(10), step: f(0.5) };
const wireAb = { id: "wire-ab", from: "node-a", to: "node-b", fromPort: "solid", toPort: "base" };
const wireBc = { id: "wire-bc", from: "node-b", to: "node-c", fromPort: "solid", toPort: "base" };
const homeCamera = { x: f(0), y: f(0), zoom: f(1) };
const homeLayout = { "node-a": { x: f(0), y: f(0) } };

type FixtureOver = { schema?: string; camera?: unknown; widgets?: unknown[]; synapses?: unknown[]; layout?: unknown };
const fixture = (over: FixtureOver = {}) => ({
  schema: over.schema ?? "flow.fixture",
  camera: over.camera ?? homeCamera,
  widgets: over.widgets ?? [nodeA, nodeB],
  synapses: over.synapses ?? [wireAb],
  layout: over.layout ?? homeLayout,
});

const genOne = { id: "gen-1", name: "Baseline", values: { storeys: f(3) } };
const genTwo = { id: "gen-2", name: "Taller", values: { storeys: f(5) } };
const play = (generations: unknown[], selected: string) => ({ generations, selectedGenerationId: selected });
const homePlay = play([genOne], "gen-1");

const snapshot = (fx: unknown = fixture(), generation: unknown = homePlay) => ({ fixture: fx, generation });

type Delta = { fixture?: unknown; generation?: unknown };
const delta = (over: Delta) => ({
  artifact: null,
  fixture: over.fixture ?? null,
  generation: over.generation ?? null,
  selectedNodeIds: null,
  lodMode: null,
  showMode: null,
  selectionMethod: null,
  hoveredNodeId: null,
  graphCamera: null,
  previewCamera: null,
  sunJson: null,
  selectedGenerationId: null,
  generationPreviewText: null,
  activeUtilityId: null,
  locale: null,
});
const applied = { status: "applied" };
//#endregion 🔖️Vocabulary

//#region 🔖️FixtureLane
const sliderFixture = (widget: unknown) => fixture({ widgets: [nodeA, widget], synapses: [] });
const threeNodes = (over: FixtureOver = {}) => fixture({ widgets: [nodeA, nodeB, nodeC], ...over });

const fixtureCases: readonly Case[] = [
  {
    leafDir: "🌱create-widget",
    leafSlug: "create-widget",
    caseName: "inserts-node-c-at-index-2",
    headline: "`create-widget`'s diff builder rejects a duplicate widget id via `widget_index`, then publishes the fixture with the payload's widget spliced in at its final-state index; synapses, layout and camera ride through unchanged.",
    before: snapshot(),
    after: snapshot(fixture({ widgets: [nodeA, nodeB, nodeC] })),
    mutation: { CreateWidget: { index: 2, widget: nodeC } },
    diff: delta({ fixture: fixture({ widgets: [nodeA, nodeB, nodeC] }) }),
    outcome: applied,
  },
  {
    leafDir: "🩹update-widget",
    leafSlug: "update-widget",
    caseName: "retunes-the-knob-slider-value",
    headline: "`update-widget`'s diff builder has a slider-specific invariant guard — non-finite or inverted value/min/max/step is fatal — and a no-op guard for an already-identical widget; here a valid slider retune is written back at the widget's own BASE index, never appended.",
    before: snapshot(sliderFixture(knob)),
    after: snapshot(sliderFixture(knobRaised)),
    mutation: { UpdateWidget: { widget: knobRaised } },
    diff: delta({ fixture: sliderFixture(knobRaised) }),
    outcome: applied,
  },
  {
    leafDir: "❌delete-widget",
    leafSlug: "delete-widget",
    caseName: "removes-node-a-and-leaves-wire-ab-dangling",
    headline: "`delete-widget`'s diff builder removes ONLY the widget and — unlike procedural2d's twin — raises no cascade message at all: `wire-ab` and the `node-a` layout entry survive in the published fixture, silently.",
    before: snapshot(),
    after: snapshot(fixture({ widgets: [nodeB] })),
    mutation: { DeleteWidget: { id: "node-a" } },
    diff: delta({ fixture: fixture({ widgets: [nodeB] }) }),
    outcome: applied,
  },
  {
    leafDir: "🔗connect-synapse",
    leafSlug: "connect-synapse",
    caseName: "wires-node-b-to-node-c-at-index-1",
    headline: "`connect-synapse`'s diff builder clears exactly three guards — duplicate synapse id, `from` widget present, `to` widget present — and has no parallel-edge check, then publishes the fixture with the new wire inserted at the payload index.",
    before: snapshot(threeNodes()),
    after: snapshot(threeNodes({ synapses: [wireAb, wireBc] })),
    mutation: { ConnectSynapse: { index: 1, synapse: wireBc } },
    diff: delta({ fixture: threeNodes({ synapses: [wireAb, wireBc] }) }),
    outcome: applied,
  },
  {
    leafDir: "🔄update-synapse",
    leafSlug: "update-synapse",
    caseName: "repoints-wire-ab-onto-the-cap-port",
    headline: "`update-synapse`'s diff builder no-ops on an identical spec; otherwise it hands the helper index `0` and relies on the id-keyed in-place replacement, so `wire-ab` keeps its own slot in the synapse list.",
    before: snapshot(),
    after: snapshot(fixture({ synapses: [{ id: "wire-ab", from: "node-a", to: "node-b", fromPort: "solid", toPort: "cap" }] })),
    mutation: { UpdateSynapse: { synapse: { id: "wire-ab", from: "node-a", to: "node-b", fromPort: "solid", toPort: "cap" } } },
    diff: delta({ fixture: fixture({ synapses: [{ id: "wire-ab", from: "node-a", to: "node-b", fromPort: "solid", toPort: "cap" }] }) }),
    outcome: applied,
  },
  {
    leafDir: "✂️disconnect-synapse",
    leafSlug: "disconnect-synapse",
    caseName: "cuts-wire-ab-leaving-both-nodes",
    headline: "`disconnect-synapse`'s diff builder drops the wire id from the synapse list only; both endpoint widgets and the layout map come through verbatim.",
    before: snapshot(),
    after: snapshot(fixture({ synapses: [] })),
    mutation: { DisconnectSynapse: { id: "wire-ab" } },
    diff: delta({ fixture: fixture({ synapses: [] }) }),
    outcome: applied,
  },
  {
    leafDir: "📍move-widget",
    leafSlug: "move-widget",
    caseName: "repositions-node-a-in-the-graph",
    headline: "`move-widget`'s diff builder checks the widget exists and the coordinates are finite, then upserts ONE layout-map entry — the widget list and the synapse list are republished byte-identical.",
    before: snapshot(),
    after: snapshot(fixture({ layout: { "node-a": { x: f(6.25), y: f(-3.5) } } })),
    mutation: { MoveWidget: { id: "node-a", layout: { x: f(6.25), y: f(-3.5) } } },
    diff: delta({ fixture: fixture({ layout: { "node-a": { x: f(6.25), y: f(-3.5) } } }) }),
    outcome: applied,
  },
  {
    leafDir: "🧹delete-widget-position",
    leafSlug: "delete-widget-position",
    caseName: "unpins-the-node-a-position",
    headline: "`delete-widget-position`'s diff builder guards on BOTH the widget existing and it actually having a layout entry, then removes that key — the widget itself stays in the fixture, merely unpositioned.",
    before: snapshot(),
    after: snapshot(fixture({ layout: {} })),
    mutation: { DeleteWidgetPosition: { id: "node-a" } },
    diff: delta({ fixture: fixture({ layout: {} }) }),
    outcome: applied,
  },
  {
    leafDir: "📷update-camera",
    leafSlug: "update-camera",
    caseName: "frames-the-graph-at-double-zoom",
    headline: "`update-camera`'s diff builder rejects non-finite x/y/zoom and no-ops on an unchanged camera; here only the fixture's `camera` field moves, and widgets/synapses/layout are republished unchanged.",
    before: snapshot(),
    after: snapshot(fixture({ camera: { x: f(-4), y: f(6), zoom: f(2) } })),
    mutation: { UpdateCamera: { camera: { x: f(-4), y: f(6), zoom: f(2) } } },
    diff: delta({ fixture: fixture({ camera: { x: f(-4), y: f(6), zoom: f(2) } }) }),
    outcome: applied,
  },
  {
    leafDir: "🔤change-schema",
    leafSlug: "change-schema",
    caseName: "restamps-the-fixture-schema-id",
    headline: "`change-schema`'s diff builder rejects a blank/whitespace schema id outright and no-ops on an unchanged one; its payload field is `newSchema`, and only the fixture's `schema` string is rewritten.",
    before: snapshot(),
    after: snapshot(fixture({ schema: "flow.fixture.solid" })),
    mutation: { ChangeSchema: { newSchema: "flow.fixture.solid" } },
    diff: delta({ fixture: fixture({ schema: "flow.fixture.solid" }) }),
    outcome: applied,
  },
];
//#endregion 🔖️FixtureLane

//#region 🔖️GenerationLane
const generationCases: readonly Case[] = [
  {
    leafDir: "➕create-generation",
    leafSlug: "create-generation",
    caseName: "appends-generation-2-and-moves-the-selection",
    headline: "`create-generation`'s diff builder rejects a duplicate id, then folds a `GenerationMutation::Add` into the play state — which appends the generation AND re-points `selectedGenerationId` at it, both of which the committed diff must carry.",
    before: snapshot(fixture(), homePlay),
    after: snapshot(fixture(), play([genOne, genTwo], "gen-2")),
    mutation: { CreateGeneration: { generation: genTwo } },
    diff: delta({ generation: play([genOne, genTwo], "gen-2") }),
    outcome: applied,
  },
  {
    leafDir: "🗑delete-generation",
    leafSlug: "delete-generation",
    caseName: "removes-the-selected-generation-2-and-falls-back",
    headline: "`delete-generation`'s diff builder folds a `GenerationMutation::Remove` in; because the removed generation was the selected one, the play state falls back to the first survivor, so the committed diff carries the moved selection as well as the shortened list.",
    before: snapshot(fixture(), play([genOne, genTwo], "gen-2")),
    after: snapshot(fixture(), play([genOne], "gen-1")),
    mutation: { DeleteGeneration: { id: "gen-2" } },
    diff: delta({ generation: play([genOne], "gen-1") }),
    outcome: applied,
  },
  {
    leafDir: "🏷rename-generation",
    leafSlug: "rename-generation",
    caseName: "retitles-generation-1-via-new-name",
    headline: "`rename-generation`'s diff builder rejects a missing id and no-ops on an identical name; its payload field is `newName`, and only that generation's `name` is rewritten — answers and selection are untouched.",
    before: snapshot(fixture(), homePlay),
    after: snapshot(fixture(), play([{ id: "gen-1", name: "Baseline, reviewed", values: { storeys: f(3) } }], "gen-1")),
    mutation: { RenameGeneration: { id: "gen-1", newName: "Baseline, reviewed" } },
    diff: delta({ generation: play([{ id: "gen-1", name: "Baseline, reviewed", values: { storeys: f(3) } }], "gen-1") }),
    outcome: applied,
  },
  {
    leafDir: "🔧change-generation-value",
    leafSlug: "change-generation-value",
    caseName: "raises-the-storeys-answer-in-generation-1",
    headline: "`change-generation-value`'s diff builder no-ops when the stored answer already equals `newValue`; here it upserts exactly one key of that generation's `values` map, leaving its name and the selection where they were.",
    before: snapshot(fixture(), homePlay),
    after: snapshot(fixture(), play([{ id: "gen-1", name: "Baseline", values: { storeys: f(4.5) } }], "gen-1")),
    mutation: { ChangeGenerationValue: { id: "gen-1", questionId: "storeys", newValue: f(4.5) } },
    diff: delta({ generation: play([{ id: "gen-1", name: "Baseline", values: { storeys: f(4.5) } }], "gen-1") }),
    outcome: applied,
  },
];
//#endregion 🔖️GenerationLane

emitAll(tree, [...fixtureCases, ...generationCases]);
