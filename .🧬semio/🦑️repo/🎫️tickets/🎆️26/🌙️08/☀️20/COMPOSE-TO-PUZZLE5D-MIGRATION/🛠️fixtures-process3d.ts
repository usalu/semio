#!/usr/bin/env bun
/** 🏭️ Handcrafted mutation fixtures for `process`'s `process3d` artifact (16 leaves).
 *
 * Two genuinely different families live in this tree, and the fixtures below say so:
 *
 * • The seven STEP verbs (`create-step`, `delete-step`, `rename-step`, `change-step-enabled`,
 *   `change-step-origin`, `replace-step-measure`, `reorder-steps`) are all DOCUMENTED NO-OPS today —
 *   each `🔺️diff/🦀️component.rs` ignores its payload entirely and returns
 *   `MutationOutcome::empty().warn("mutation.no-op", …)` pending a link resolver for the composed
 *   `s.stdio.semio.flow` steps child. Their fixtures therefore pin an EMPTY diff and an unchanged
 *   snapshot — the contract's "a warn no-op is applied with an empty diff, not rejected" case — and
 *   they are the tripwire that fires the day someone implements the real edit.
 *
 * • The nine WORKSHOP/STOCK/CURSOR verbs are real value diffs: the machine verbs republish a whole
 *   `Workshop`, the stock verbs set one scalar or one child handle.
 *
 * `Process3dMutation` is INTERNALLY tagged (`#[serde(tag = "mutation", rename_all = "camelCase")]`),
 * unlike lowpoly's externally-tagged enum, so every payload below is a flat map carrying `mutation`.
 */
import { emitAll, f, type Case, type Tree } from "./🛠️emit-fixture.ts";

const tree: Tree = {
  mutationsRoot: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
  glue: "✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📦️glue.rs",
  gluePrefix: "../../🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
  diffPath: "crate::artifacts::process3d::diff",
  diffName: "Process3dDiff",
  snapshotPath: "crate::artifacts::process3d",
  snapshotName: "Process3dSnapshot",
  mutationsPath: "crate::artifacts::process3d::mutations",
  mutationName: "Process3dMutation",
  entry: "kernel",
};

//#region 🔖️Vocabulary
const child = (kind: string, subset: string, childId: string, artifactId: string) => ({
  childId,
  target: { artifactId, dialect: { artifactKind: kind, standard: "v1", subset } },
});
const stockSolid = child("s.stdio.semio", "brep", "brep-stock-01", "stock-1-solid");
const plannedSolid = child("s.stdio.semio", "brep", "brep-stock-02", "stock-1-solid-planed");
const stepsChild = child("s.stdio.semio", "flow", "flow-steps-01", "process-1-steps");

const pose = (position: unknown[], axis: unknown[], angle: number) => ({ position, axis, angle: f(angle) });
const restPose = pose([f(0), f(0), f(0)], [f(0), f(0), f(1)], 0);

const cutCapability = {
  id: "cut",
  label: "Cut",
  iconId: "scissors",
  recipe: { recipe: "bladeCut", kerf: "kerf", length: "length", depth: "depth" },
  parameters: [
    { id: "kerf", label: "Kerf", value: f(0.0625) },
    { id: "length", label: "Length", value: f(0.5) },
    { id: "depth", label: "Depth", value: f(0.5) },
  ],
  rules: [],
};
const pocketCapability = {
  id: "pocket",
  label: "Pocket",
  iconId: "square",
  recipe: { recipe: "pocketCut", diameter: "diameter", depth: "depth" },
  parameters: [
    { id: "diameter", label: "Diameter", value: f(0.125) },
    { id: "depth", label: "Depth", value: f(0.25) },
  ],
  rules: [{ kind: "min", quantity: "Width", parameter: "diameter", margin: f(0.0625) }],
};
const saw = (over: { label?: string; iconId?: string; capabilities?: unknown[] } = {}) => ({
  id: "saw",
  label: over.label ?? "Bench Saw",
  iconId: over.iconId ?? "scissors",
  capabilities: over.capabilities ?? [cutCapability],
});
const drillPress = {
  id: "drill-press",
  label: "Drill Press",
  iconId: "circle-dot",
  capabilities: [
    {
      id: "bore",
      label: "Bore",
      iconId: "circle-dot",
      recipe: { recipe: "boreDrill", radius: "radius", depth: "depth" },
      parameters: [
        { id: "radius", label: "Radius", value: f(0.0625) },
        { id: "depth", label: "Depth", value: f(0.25) },
      ],
      rules: [],
    },
  ],
};

type SnapOver = { machines?: unknown[]; stockLabel?: string; stockPose?: unknown; stockSolid?: unknown; resolvedUpTo?: number | null };
const snapshot = (over: SnapOver = {}) => ({
  workshop: { machines: over.machines ?? [saw()] },
  stockId: "stock-1",
  stockLabel: over.stockLabel ?? "Oak Beam",
  stockPose: over.stockPose ?? restPose,
  stockSolid: over.stockSolid ?? stockSolid,
  steps: stepsChild,
  toolSolids: [],
  resolvedUpTo: over.resolvedUpTo ?? null,
});
const base = snapshot();

/** 🔺️ `Process3dDiff` carries `default` on the container and no `skip_serializing_if`, so all 29
 * fields are emitted — `null` for every one a leaf never touches. */
type Delta = { workshop?: unknown; stockLabel?: string; stockPose?: unknown; stockSolid?: unknown; resolvedUpTo?: number };
const delta = (over: Delta = {}) => ({
  artifact: null,
  workshop: over.workshop ?? null,
  stockId: null,
  stockLabel: over.stockLabel ?? null,
  stockPose: over.stockPose ?? null,
  stockSolid: over.stockSolid ?? null,
  steps: null,
  toolSolids: null,
  resolvedUpTo: over.resolvedUpTo ?? null,
  selectedId: null,
  selectedFaceId: null,
  activeUtilityId: null,
  selectionMethod: null,
  engagementInput: null,
  cameraPositionX: null,
  cameraPositionY: null,
  cameraPositionZ: null,
  cameraTargetX: null,
  cameraTargetY: null,
  cameraTargetZ: null,
  cameraFov: null,
  sunEnabled: null,
  sunAzimuth: null,
  sunElevation: null,
  sunIntensity: null,
  sunColor: null,
  locale: null,
  contributionsJson: null,
  hoveredId: null,
});
const applied = { status: "applied" };
const noOp = { status: "applied", messages: [{ level: "warning", code: "mutation.no-op" }] };
//#endregion 🔖️Vocabulary

//#region 🔖️StepLane
const ripCut = {
  id: "step-1",
  label: "Rip cut",
  enabled: true,
  origin: { machineId: "saw", capabilityId: "cut" },
  measure: { measure: "drill", radius: f(0.0625), depth: f(0.5), pose: restPose },
};

/** 🚧️ Every step verb shares the identical shape — an untouched snapshot and an empty diff — but
 * each one names its OWN payload and its own diff builder's wording. */
const stepCase = (leafDir: string, leafSlug: string, caseName: string, headline: string, mutation: unknown): Case => ({
  leafDir,
  leafSlug,
  caseName,
  headline,
  before: base,
  after: base,
  mutation,
  diff: delta(),
  outcome: noOp,
});

const stepCases: readonly Case[] = [
  stepCase(
    "🌱create-step",
    "create-step",
    "accepts-a-rip-cut-step-and-changes-nothing",
    "`create-step`'s diff builder takes `_payload` and `_base` by underscore and returns `MutationOutcome::empty().warn(\"mutation.no-op\", …)`: the timeline is a composed `s.stdio.semio.flow` child and no link resolver exists yet, so a fully-formed step payload still yields an empty diff and an untouched snapshot.",
    { mutation: "createStep", index: 0, step: ripCut },
  ),
  stepCase(
    "🗑️delete-step",
    "delete-step",
    "accepts-a-step-id-and-changes-nothing",
    "`delete-step`'s diff builder never reads `base.steps` — it cannot, the timeline lives in a composed child — so even a plausible step id produces the `mutation.no-op` warning and an empty diff rather than a `target-missing` error.",
    { mutation: "deleteStep", id: "step-1" },
  ),
  stepCase(
    "🏷️rename-step",
    "rename-step",
    "accepts-a-new-label-and-changes-nothing",
    "`rename-step` carries a `newLabel` payload field that its diff builder discards wholesale, returning the documented `mutation.no-op` warning; the stock label — a real, adjacent scalar — is deliberately untouched by it.",
    { mutation: "renameStep", id: "step-1", newLabel: "Crosscut" },
  ),
  stepCase(
    "🔘change-step-enabled",
    "change-step-enabled",
    "accepts-a-disable-flag-and-changes-nothing",
    "`change-step-enabled` would gate one step out of kernel replay; today its diff builder returns the empty no-op outcome, so `resolvedUpTo` and every other artifact-lane field come back byte-identical.",
    { mutation: "changeStepEnabled", id: "step-1", newEnabled: false },
  ),
  stepCase(
    "🧷change-step-origin",
    "change-step-origin",
    "accepts-a-machine-provenance-and-changes-nothing",
    "`change-step-origin` carries an `Option<StepOrigin>` payload (present here, naming the saw's cut capability) that the diff builder ignores; provenance is informational and never resolved, and today not even recorded.",
    { mutation: "changeStepOrigin", id: "step-1", newOrigin: { machineId: "saw", capabilityId: "cut" } },
  ),
  stepCase(
    "📐replace-step-measure",
    "replace-step-measure",
    "accepts-a-bore-measure-and-changes-nothing",
    "`replace-step-measure` is the only step verb carrying a whole `ProcessMeasure` — the internally-tagged geometry enum the kernel replays — and its diff builder still discards it for the documented `mutation.no-op`.",
    { mutation: "replaceStepMeasure", id: "step-1", newMeasure: { measure: "drill", radius: f(0.125), depth: f(0.5), pose: restPose } },
  ),
  stepCase(
    "🔀reorder-steps",
    "reorder-steps",
    "accepts-a-target-index-and-changes-nothing",
    "`reorder-steps` would permute the timeline the kernel replays in order; its diff builder returns the empty no-op outcome, so unlike lowpoly's `reorder-objects` no `reordered` permutation is published anywhere.",
    { mutation: "reorderSteps", id: "step-1", toIndex: 0 },
  ),
];
//#endregion 🔖️StepLane

//#region 🔖️WorkshopLane
const workshopCases: readonly Case[] = [
  {
    leafDir: "🏭create-machine",
    leafSlug: "create-machine",
    caseName: "adds-a-drill-press-to-the-workshop",
    headline: "`create-machine` rejects a duplicate machine id, then republishes the WHOLE `Workshop` with the payload machine PUSHED ONTO THE END — its `index` payload field is accepted but never honoured by the diff builder.",
    before: base,
    after: snapshot({ machines: [saw(), drillPress] }),
    mutation: { mutation: "createMachine", index: 1, machine: drillPress },
    diff: delta({ workshop: { machines: [saw(), drillPress] } }),
    outcome: applied,
  },
  {
    leafDir: "❌delete-machine",
    leafSlug: "delete-machine",
    caseName: "empties-the-workshop-of-the-saw",
    headline: "`delete-machine` errors with `target-missing` on an unknown id; otherwise it republishes the whole `Workshop` with that machine retained out — steps already authored from it keep working, because a step never resolves back to its machine.",
    before: base,
    after: snapshot({ machines: [] }),
    mutation: { mutation: "deleteMachine", id: "saw" },
    diff: delta({ workshop: { machines: [] } }),
    outcome: applied,
  },
  {
    leafDir: "🔖rename-machine",
    leafSlug: "rename-machine",
    caseName: "retitles-the-saw",
    headline: "`rename-machine` no-ops on an identical label; otherwise it clones the machine list, rewrites that one machine's `label`, and republishes the whole `Workshop` — its icon and capabilities ride through unchanged.",
    before: base,
    after: snapshot({ machines: [saw({ label: "Panel Saw" })] }),
    mutation: { mutation: "renameMachine", id: "saw", newLabel: "Panel Saw" },
    diff: delta({ workshop: { machines: [saw({ label: "Panel Saw" })] } }),
    outcome: applied,
  },
  {
    leafDir: "🎨change-machine-icon",
    leafSlug: "change-machine-icon",
    caseName: "swaps-the-saw-icon",
    headline: "`change-machine-icon` treats the icon id as an opaque string with no registry check, rewriting only that machine's `iconId` inside a republished `Workshop`; the machine's label and capabilities are untouched.",
    before: base,
    after: snapshot({ machines: [saw({ iconId: "saw-blade" })] }),
    mutation: { mutation: "changeMachineIcon", id: "saw", newIconId: "saw-blade" },
    diff: delta({ workshop: { machines: [saw({ iconId: "saw-blade" })] } }),
    outcome: applied,
  },
  {
    leafDir: "🔁replace-machine-capabilities",
    leafSlug: "replace-machine-capabilities",
    caseName: "trades-the-blade-cut-for-a-gated-pocket-cut",
    headline: "`replace-machine-capabilities` swaps the machine's capability list WHOLESALE (no per-capability merge), so the outgoing blade-cut recipe, its three parameters and its empty rule set are all replaced at once by the pocket-cut capability and its `min` stock rule.",
    before: base,
    after: snapshot({ machines: [saw({ capabilities: [pocketCapability] })] }),
    mutation: { mutation: "replaceMachineCapabilities", id: "saw", newCapabilities: [pocketCapability] },
    diff: delta({ workshop: { machines: [saw({ capabilities: [pocketCapability] })] } }),
    outcome: applied,
  },
];
//#endregion 🔖️WorkshopLane

//#region 🔖️StockLane
const liftedPose = pose([f(0), f(0), f(1.5)], [f(1), f(0), f(0)], 0.5);

const stockCases: readonly Case[] = [
  {
    leafDir: "📍move-stock",
    leafSlug: "move-stock",
    caseName: "lifts-and-tilts-the-stock",
    headline: "`move-stock` rejects a non-finite position/axis/angle and no-ops on an identical pose; otherwise it sets the single scalar `stockPose` field — the stock's brep child handle is NOT reissued, because a pose is applied by the kernel, not baked into geometry.",
    before: base,
    after: snapshot({ stockPose: liftedPose }),
    mutation: { mutation: "moveStock", newPose: liftedPose },
    diff: delta({ stockPose: liftedPose }),
    outcome: applied,
  },
  {
    leafDir: "🔤change-stock-label",
    leafSlug: "change-stock-label",
    caseName: "relabels-the-oak-beam-as-planed",
    headline: "`change-stock-label` no-ops on an identical string; otherwise it sets `stockLabel` and nothing else — notably NOT `stockId`, which is the stock's stable identity and has no mutation at all in this vocabulary.",
    before: base,
    after: snapshot({ stockLabel: "Oak Beam, planed" }),
    mutation: { mutation: "changeStockLabel", newLabel: "Oak Beam, planed" },
    diff: delta({ stockLabel: "Oak Beam, planed" }),
    outcome: applied,
  },
  {
    leafDir: "🧊replace-stock-solid",
    leafSlug: "replace-stock-solid",
    caseName: "reissues-the-stock-brep-child-handle",
    headline: "`replace-stock-solid` compares the whole `ArtifactChild` handle for equality and no-ops when unchanged; the diff it writes carries ONLY the two-string handle — the parent never stores brep topology, so swapping geometry is a handle swap.",
    before: base,
    after: snapshot({ stockSolid: plannedSolid }),
    mutation: { mutation: "replaceStockSolid", newSolid: plannedSolid },
    diff: delta({ stockSolid: plannedSolid }),
    outcome: applied,
  },
  {
    leafDir: "⏱️change-cursor",
    leafSlug: "change-cursor",
    caseName: "pins-the-replay-cursor-to-two-steps",
    headline: "`change-cursor` moves the timeline scrub point that bounds kernel replay; its diff field is a double-`Option` whose outer level means \"this diff touches the cursor\" — here it is set to a concrete `2`, the inner-`Some` case that survives a JSON round trip.",
    before: base,
    after: snapshot({ resolvedUpTo: 2 }),
    mutation: { mutation: "changeCursor", newResolvedUpTo: 2 },
    diff: delta({ resolvedUpTo: 2 }),
    outcome: applied,
  },
];
//#endregion 🔖️StockLane

emitAll(tree, [...stepCases, ...workshopCases, ...stockCases]);
