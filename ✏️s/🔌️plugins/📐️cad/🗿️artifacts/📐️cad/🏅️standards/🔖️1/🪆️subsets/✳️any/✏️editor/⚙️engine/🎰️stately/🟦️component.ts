// #region 🧲️Header
/** @emoji 🎭️ `@semio-tech/cad-js/stately` — `@semio-tech/machine`-backed `StateEngine` for `InteractionSpec.machine`; transitions mirror spec while `applyTransition` owns effects. See `.🦑️repo/✍️/spatial.md`. Was XState-backed; ported to the in-house statechart kernel (Wave 8, runtime-dependency-elimination) — see the kernel's own flat/guarded fixture tests in `🧰️framework/🔨️modules/🔄️machine/🟦️.ts`. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { EventId, GuardId, NodeId, ROOT, init, macrostep, NullInspector, type Command, type GuardFn, type Machine, type MachineDefinition, type MachineSpec, type NodeDef, type Snapshot, type StatechartEvent, type TransitionDef } from "@semio-tech/machine";
import type { Vec3 } from "@semio-tech/s-3d-js";
import { emptyMeshTransfer, solidRef } from "@semio-tech/s-3d-js";
import {
  Model,
  defaultModelDefinitionId,
  initialContextForSpec,
  listSpatialInteractionsForModelDefinition,
  type InteractionEvent,
  type InteractionSpec,
  type EdgeRef,
  type FaceRef,
  type VertexRef,
  type WireRef,
} from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts";
import { isEmptyModelDiff, type SpatialKernel, type ModelDiff } from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️component.ts";
import { applyTransition, pureTsStateEngineProvider, type ActionRegistry, type StateEngine, type StateEngineProvider, type StateEngineSendResult } from "../🎬️actions/🟦️component.ts";
import { createInteractionRuntime, loadSpatialInteraction, type InteractionRuntime } from "../📄️artifact/🟦️component.ts";
// #endregion 🔌️Adapters

// #region 🎭️AdvanceEvent
/** @emoji 🎭️ The one wire-level event kind every compiled `StatelyMachineSpec` reacts to; rows for
 * different `spec.machine` `(state, event)` pairs are disambiguated purely by guard, not by event
 * identity — same encoding the former XState chart used (`__advance` + `interactionKind` + `branch`). */
interface StatelyAdvanceEvent extends StatechartEvent {
  readonly type: "__advance";
  readonly interactionKind: string;
  readonly branch: number;
}
const ADVANCE_EVENT_ID = EventId(0);
function makeAdvanceEvent(interactionKind: string, branch: number): StatelyAdvanceEvent {
  return { type: "__advance", interactionKind, branch, eventCount: 1, eventId: () => ADVANCE_EVENT_ID, eventName: () => "__advance" };
}

/** @emoji 🎭️ Associated types bound to `@semio-tech/machine`'s generic kernel for the stately adapter. */
interface StatelyMachineSpec extends MachineSpec {
  Context: undefined;
  Event: StatelyAdvanceEvent;
  Input: undefined;
  Output: never;
  Effect: never;
}
// #endregion 🎭️AdvanceEvent

// #region 🎭️MachineBuild
/** @emoji 🎭️ Builds a flat, one-level `MachineDefinition` isomorphic to `spec.machine` — every state is
 * an atomic child of the synthetic root, `initial` selects the root's entry child, and every transition
 * row becomes one `TransitionDef` on the shared `__advance` event id, guarded by `(interactionKind,
 * branch)`. Rebuilt on every state change (cheap: this is a flat table, not a running actor). */
function buildStatelyMachine(spec: InteractionSpec, initial: string): Machine<StatelyMachineSpec> {
  const stateIds = spec.machine.states.map((st) => st.name);
  const nodeIdByStableId = new Map<string, NodeId>(stateIds.map((id, i) => [id, NodeId(i + 1)]));
  const nodes: NodeDef[] = [
    { stableId: "__root", kind: "compound", initial: nodeIdByStableId.get(initial)!, children: [...nodeIdByStableId.values()], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
    ...spec.machine.states.map(
      (st, i): NodeDef => ({ stableId: st.name, kind: "atomic", parent: ROOT, children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: i + 1 }),
    ),
  ];
  const guards: GuardFn<StatelyMachineSpec>[] = [];
  const transitions: TransitionDef[] = [];
  for (const st of spec.machine.states) {
    if (!st.on) continue;
    const source = nodeIdByStableId.get(st.name)!;
    for (const h of st.on) {
      for (let i = 0; i < h.transitions.length; i++) {
        const tr = h.transitions[i]!;
        const target = nodeIdByStableId.get((tr.target ?? st.name) as string)!;
        const eventKind = h.event;
        const branchIndex = i;
        const guardId = GuardId(guards.length);
        guards.push((_context, event) => event !== undefined && event.interactionKind === eventKind && event.branch === branchIndex);
        transitions.push({ source, trigger: { kind: "event", event: ADVANCE_EVENT_ID }, guard: guardId, targets: [target], kind: "external", actions: [], docIndex: transitions.length });
      }
    }
  }
  const definition: MachineDefinition<StatelyMachineSpec> = {
    id: `spatial-interaction-${spec.id}`,
    nodes,
    transitions,
    contextFromInput: () => undefined,
    guards,
    actions: [],
    fingerprint: 0n,
    manifestJson: "{}",
  };
  return { definition };
}

/** @emoji 📊️ One transition row for `🔣️machine.json` / Mermaid (matches `__advance` branch order). */
export interface SpatialStatelyMachineTransitionView {
  readonly from: string;
  readonly to: string;
  readonly on: string;
  readonly branch: number;
  readonly guard: string | null;
  readonly transient: boolean;
  readonly key?: string;
  readonly label?: string;
}

/** @emoji 📊️ Serializable state node summary. */
export interface SpatialStatelyMachineStateView {
  readonly id: string;
  readonly final: boolean;
  readonly selectionAccept?: readonly string[];
}

/** @emoji 📊️ Single spatial interaction as a viewable state machine (edges + Mermaid). */
export interface SpatialStatelyMachineView {
  readonly interactionId: string;
  readonly interactionVersion: string;
  readonly label: string;
  readonly hostKey: string;
  readonly initial: string;
  readonly states: readonly SpatialStatelyMachineStateView[];
  readonly edges: readonly SpatialStatelyMachineTransitionView[];
  readonly mermaid: string;
  readonly statelyRoutingNote: string;
}

/** @emoji 📊️ Catalog of model-definition interactions for Stately/Mermaid viewers (`🔣️machine.json`). */
export interface SpatialStatelyMachineCatalogView {
  readonly kind: "spatial.stately-machine-view/v1";
  readonly schemaVersion: "1.0";
  readonly generatedAt: string;
  readonly machines: readonly SpatialStatelyMachineView[];
  readonly mermaidCombined: string;
}

/** @emoji 📊️ Collects flat transition rows from `InteractionSpec.machine` (same order as `StatelyStateEngine`). */
export function collectSpatialStatelyMachineTransitions(spec: InteractionSpec): readonly SpatialStatelyMachineTransitionView[] {
  const out: SpatialStatelyMachineTransitionView[] = [];
  for (const st of spec.machine.states) {
    const from = st.name;
    if (!st.on) continue;
    for (const h of st.on) {
      for (let i = 0; i < h.transitions.length; i++) {
        const tr = h.transitions[i]!;
        const to = (tr.target ?? from) as string;
        out.push({
          from,
          to,
          on: h.event,
          branch: i,
          guard: tr.guard ?? null,
          transient: Boolean(tr.transient),
          ...(typeof tr.key === "string" && tr.key.length > 0 ? { key: tr.key } : {}),
          ...(typeof tr.label === "string" && tr.label.length > 0 ? { label: tr.label } : {}),
        });
      }
    }
  }
  return out;
}

function buildSpatialStatelyStateViews(spec: InteractionSpec): SpatialStatelyMachineStateView[] {
  return spec.machine.states.map((st) => {
    const acc = st.selection?.accept;
    return {
      id: st.name,
      final: Boolean(st.final),
      ...(acc && acc.length ? { selectionAccept: [...acc] } : {}),
    };
  });
}

function mermaidForSpatialInteraction(spec: InteractionSpec, title: string): string {
  const slug = spec.id.replace(/[^\w]+/g, "_");
  const sid = (s: string) => `${slug}__${s.replace(/[^\w]+/g, "_")}`;
  const esc = (s: string) => s.replace(/"/g, "'");
  const lines = ["flowchart TB", `  subgraph sub_${slug} ["${esc(title)}"]`];
  for (const st of buildSpatialStatelyStateViews(spec)) {
    const tag = st.final ? " (final)" : "";
    lines.push(`    ${sid(st.id)}["${esc(st.id)}${esc(tag)}"]`);
  }
  for (const e of collectSpatialStatelyMachineTransitions(spec)) {
    let el = e.on;
    if (e.guard) el += ` [${e.guard}]`;
    if (e.key) el += ` key:${e.key}`;
    if (e.transient) el += " ·transient";
    lines.push(`    ${sid(e.from)} -->|"${esc(el)}"| ${sid(e.to)}`);
  }
  lines.push("  end");
  return lines.join("\n");
}

/** @emoji 📊️ Builds one view document for a loaded `InteractionSpec` (interaction metadata for labels/keys). */
export function buildSpatialStatelyMachineViewForSpec(spec: InteractionSpec, meta: { readonly hostKey: string; readonly interactionLabel: string }): SpatialStatelyMachineView {
  const edges = collectSpatialStatelyMachineTransitions(spec);
  return {
    interactionId: spec.id,
    interactionVersion: spec.version,
    label: spec.label ?? meta.interactionLabel,
    hostKey: meta.hostKey,
    initial: spec.machine.initial,
    states: buildSpatialStatelyStateViews(spec),
    edges,
    mermaid: mermaidForSpatialInteraction(spec, `${meta.interactionLabel} (${spec.id})`),
    statelyRoutingNote:
      "Runtime applies `applyTransition` (guards/effects) in core; `StatelyStateEngine` then sends `{ type: '__advance', interactionKind, branch }` where `branch` is the transition index for that `from` state and `on` event (same order as `edges`).",
  };
}

/** @emoji 📊️ Model-definition-scoped interaction machines from shipped interaction JSON. */
export function buildSpatialStatelyMachineCatalogView(opts: { readonly modelDefinitionId: string; readonly interactionIds?: readonly string[]; readonly generatedAt?: string }): SpatialStatelyMachineCatalogView {
  const want = opts.interactionIds?.length ? new Set(opts.interactionIds) : null;
  const machines: SpatialStatelyMachineView[] = [];
  for (const p of listSpatialInteractionsForModelDefinition(opts.modelDefinitionId)) {
    if (want && !want.has(p.id)) continue;
    const spec = loadSpatialInteraction(p.id);
    if (!spec) continue;
    machines.push(buildSpatialStatelyMachineViewForSpec(spec, { hostKey: p.key, interactionLabel: p.label }));
  }
  const generatedAt = opts.generatedAt ?? new Date().toISOString();
  return {
    kind: "spatial.stately-machine-view/v1",
    schemaVersion: "1.0",
    generatedAt,
    machines,
    mermaidCombined: machines.map((m) => m.mermaid).join("\n\n"),
  };
}
// #endregion 🎭️MachineBuild

// #region 🎭️StatelyStateEngine
/** @emoji 🎭️ `@semio-tech/machine`-backed `StateEngine`; `send` runs `applyTransition` then syncs the
 * kernel snapshot via a synchronous `macrostep` of `__advance`. */
export class StatelyStateEngine implements StateEngine {
  private interactionState: string;
  private interactionContext: Record<string, unknown>;
  private machine: Machine<StatelyMachineSpec>;
  private snapshot!: Snapshot<StatelyMachineSpec>;

  constructor(private readonly spec: InteractionSpec) {
    this.interactionState = spec.machine.initial;
    this.interactionContext = initialContextForSpec(spec);
    this.machine = buildStatelyMachine(spec, this.interactionState);
    this.bootMachine();
  }

  private bootMachine(): void {
    const sink: Command<StatelyMachineSpec>[] = [];
    this.snapshot = init(this.machine, undefined, sink);
  }

  private rebuildMachine(initial: string): void {
    this.machine = buildStatelyMachine(this.spec, initial);
    this.bootMachine();
  }

  getState(): string {
    return this.interactionState;
  }

  getContext(): Record<string, unknown> {
    return this.interactionContext;
  }

  reset(): void {
    this.interactionState = this.spec.machine.initial;
    this.interactionContext = initialContextForSpec(this.spec);
    this.rebuildMachine(this.interactionState);
  }

  restore(state: string, context: Record<string, unknown>): void {
    this.interactionContext = context;
    this.interactionState = state;
    this.rebuildMachine(state);
  }

  async send(event: InteractionEvent, kernel?: SpatialKernel, model?: Model, actions?: ActionRegistry, preview?: import("../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️component.ts").SpatialPreviewKernel, activeModelDefinitionId?: string | null): Promise<StateEngineSendResult> {
    if (!this.snapshot.matches(this.interactionState)) {
      this.rebuildMachine(this.interactionState);
    }
    const r = await applyTransition(this.spec, this.interactionState, this.interactionContext, event, kernel, actions, model, preview, activeModelDefinitionId ?? null);
    if (!r.ok) return { ok: false };
    if (r.childCall) return { ok: true, transient: r.transient, childCall: r.childCall };
    this.interactionState = r.nextState;
    const sink: Command<StatelyMachineSpec>[] = [];
    macrostep(this.machine, this.snapshot, makeAdvanceEvent(event.kind, r.branchIndex), sink, new NullInspector());
    return { ok: true, transient: r.transient };
  }
}
// #endregion 🎭️StatelyStateEngine

// #region 🎭️Provider
/** @emoji 🎭️ `StateEngineProvider` wiring `StatelyStateEngine` (`@semio-tech/machine`-backed). */
export const statelyStateEngineProvider: StateEngineProvider = {
  id: "machine-stately",
  create(spec: InteractionSpec): StateEngine {
    return new StatelyStateEngine(spec);
  },
};
// #endregion 🎭️Provider

// #region 🧪️Tests
const __spatialStatelyTestRuntime = import.meta.vitest ? await import("../🏃️runtime/🟦️component.ts") : null;
const __spatialStatelyTestKernel = import.meta.vitest ? await import("../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts") : null;

if (import.meta.vitest) {
  __spatialStatelyTestRuntime!.bootstrapCadModules();
  const { BrepjsKernel } = __spatialStatelyTestKernel!;
  const { describe, expect, it } = import.meta.vitest;

  class StubKernel extends BrepjsKernel {
    readonly id = "stub-parity";
    readonly operations = ["solid.createBox", "entity.tessellate"] as const;
    lastBox: { cornerA: Vec3; cornerB: Vec3; height: number } | null = null;
    async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }) {
      this.lastBox = input;
      return solidRef("stub-solid");
    }
    async volume() {
      return 0;
    }
    async tessellate() {
      return {
        ...emptyMeshTransfer(),
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        index: new Uint32Array([0, 1, 2]),
      };
    }
  }

  function normalizeModelDiffIds(diff: ModelDiff): ModelDiff {
    const clone = JSON.parse(JSON.stringify(diff)) as ModelDiff;
    const stamp = (added: readonly { id: string }[] | undefined, tag: string) => {
      for (const r of added ?? []) r.id = tag;
    };
    stamp(clone.anchors?.added, "__anchor__");
    stamp(clone.vertices?.added, "__vertex__");
    stamp(clone.edges?.added, "__edge__");
    stamp(clone.wires?.added, "__wire__");
    stamp(clone.faces?.added, "__face__");
    stamp(clone.shells?.added, "__shell__");
    stamp(clone.solids?.added, "__solid__");
    return {
      ...clone,
      wires: clone.wires && {
        ...clone.wires,
        added: clone.wires.added?.map((w) => ({ ...w, edgeIds: w.edgeIds.map(() => "__edge__" as EdgeRef) })),
      },
    };
  }

  async function assertSnapshotsEqual(a: InteractionRuntime, b: InteractionRuntime) {
    const sa = a.getSnapshot();
    const sb = b.getSnapshot();
    expect(sb.state).toBe(sa.state);
    expect(sb.context).toEqual(sa.context);
    expect(sb.capabilities).toEqual(sa.capabilities);
    expect(sb.lastResponse?.ok).toBe(sa.lastResponse?.ok);
    expect(sb.lastResponse?.data).toEqual(sa.lastResponse?.data);
    expect(normalizeModelDiffIds(sb.lastResponse?.diff ?? {})).toEqual(normalizeModelDiffIds(sa.lastResponse?.diff ?? {}));
  }

  class MeasureParityKernel extends BrepjsKernel {
    readonly id = "stub-measure-parity";
    readonly operations = ["surface.resolveFaces", "measure.distance", "measure.area"] as const;
    async createBoxFromCorners() {
      return solidRef("unused");
    }
    async volume() {
      return 0;
    }
    async tessellate() {
      return emptyMeshTransfer();
    }
    async query(name: string, params: Record<string, unknown>) {
      if (name === "surface.resolveFaces") return [String(params.surfaceId ?? "")];
      return undefined;
    }
    async vertexDistance(a: VertexRef, b: VertexRef, model: Model) {
      const pa = model.vertices[String(a)]?.position;
      const pb = model.vertices[String(b)]?.position;
      if (!pa || !pb) return 0;
      return Math.hypot(pa[0] - pb[0], pa[1] - pb[1], pa[2] - pb[2]);
    }
    async faceArea(_f: FaceRef, _model: Model) {
      return 42;
    }
  }

  describe("@semio-tech/cad-js/stately", () => {
    it("buildSpatialStatelyMachineCatalogView lists scoped interactions with edges and mermaid", () => {
      const doc = buildSpatialStatelyMachineCatalogView({ modelDefinitionId: defaultModelDefinitionId() });
      expect(doc.kind).toBe("spatial.stately-machine-view/v1");
      expect(doc.machines.length).toBe(listSpatialInteractionsForModelDefinition(defaultModelDefinitionId()).length);
      const box = doc.machines.find((m) => m.interactionId === "primitive.box");
      expect(box?.edges.length).toBeGreaterThan(0);
      expect(box?.mermaid).toContain("primitive_box");
      expect(doc.mermaidCombined.length).toBeGreaterThan(100);
    });

    // 🎓️ Differential test: `xstate` is a devDependency test oracle ONLY (never a production
    // dependency) — asserts `buildStatelyMachine`'s `@semio-tech/machine` definition reaches the
    // same active state as a literal XState v5 chart built from the same `InteractionSpec.machine`,
    // for every `__advance` row of every shipped spatial interaction. Kept so a future edit to
    // `buildStatelyMachine` can't silently drift from the XState semantics it replaced.
    it("buildStatelyMachine matches an XState v5 chart built from the same spec (oracle)", async () => {
      const { createActor, setup } = await import("xstate");
      for (const p of listSpatialInteractionsForModelDefinition(defaultModelDefinitionId())) {
        const spec = loadSpatialInteraction(p.id);
        if (!spec) continue;
        for (const st of spec.machine.states) {
          if (!st.on) continue;
          for (const h of st.on) {
            for (let branch = 0; branch < h.transitions.length; branch++) {
              const tr = h.transitions[branch]!;
              const expected = (tr.target ?? st.name) as string;

              const xstateMachineDef = setup({ types: { events: {} as { type: "__advance"; interactionKind: string; branch: number } } }).createMachine({
                id: `oracle-${spec.id}`,
                initial: st.name,
                states: Object.fromEntries(
                  spec.machine.states.map((s) => [
                    s.name,
                    s.on
                      ? {
                          on: {
                            __advance: s.on.flatMap((row, i) =>
                              row.transitions.map((t, j) => ({
                                guard: ({ event }: { event: { interactionKind: string; branch: number } }) => event.interactionKind === row.event && event.branch === j,
                                target: (t.target ?? s.name) as string,
                                __rowIndex: i,
                              })),
                            ),
                          },
                        }
                      : {},
                  ]),
                ),
              });
              const actor = createActor(xstateMachineDef);
              actor.start();
              actor.send({ type: "__advance", interactionKind: h.event, branch });
              const xstateResult = String(actor.getSnapshot().value);
              actor.stop();

              const ownMachine = buildStatelyMachine(spec, st.name);
              const sink: Command<StatelyMachineSpec>[] = [];
              const snapshot = init(ownMachine, undefined, sink);
              macrostep(ownMachine, snapshot, makeAdvanceEvent(h.event, branch), sink, new NullInspector());
              expect(snapshot.matches(expected)).toBe(true);
              expect(xstateResult).toBe(expected);
            }
          }
        }
      }
    });

    it("matches pure-ts interaction snapshots through box workflow + commit", async () => {
      const spec = loadSpatialInteraction("primitive.box")!;
      const k1 = new StubKernel();
      const k2 = new StubKernel();
      const rtPure = createInteractionRuntime(spec, {
        kernel: k1,
        document: { model: new Model(), nodes: [] },
        stateEngine: pureTsStateEngineProvider,
      });
      const rtSt = createInteractionRuntime(spec, {
        kernel: k2,
        document: { model: new Model(), nodes: [] },
        stateEngine: statelyStateEngineProvider,
      });
      await assertSnapshotsEqual(rtPure, rtSt);
      await rtPure.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await rtSt.send({ kind: "pointer.down", point: [0, 0, 0] as Vec3, modifiers: {} });
      await assertSnapshotsEqual(rtPure, rtSt);
      await rtPure.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} });
      await rtSt.send({ kind: "pointer.down", point: [2, 3, 0] as Vec3, modifiers: {} });
      await assertSnapshotsEqual(rtPure, rtSt);
      await rtPure.send({ kind: "set.height", value: 4, modifiers: {} });
      await rtSt.send({ kind: "set.height", value: 4, modifiers: {} });
      await assertSnapshotsEqual(rtPure, rtSt);
      expect(k1.lastBox).toEqual(k2.lastBox);
    });

    it("matches pure-ts after interaction-local undo", async () => {
      const spec = loadSpatialInteraction("primitive.box")!;
      const k1 = new StubKernel();
      const k2 = new StubKernel();
      const rtPure = createInteractionRuntime(spec, {
        kernel: k1,
        document: { model: new Model(), nodes: [] },
        stateEngine: pureTsStateEngineProvider,
      });
      const rtSt = createInteractionRuntime(spec, {
        kernel: k2,
        document: { model: new Model(), nodes: [] },
        stateEngine: statelyStateEngineProvider,
      });
      await rtPure.send({ kind: "pointer.down", point: [1, 1, 0] as Vec3, modifiers: {} });
      await rtSt.send({ kind: "pointer.down", point: [1, 1, 0] as Vec3, modifiers: {} });
      await assertSnapshotsEqual(rtPure, rtSt);
      rtPure.undo();
      rtSt.undo();
      await assertSnapshotsEqual(rtPure, rtSt);
    });

    it("matches pure-ts distance + area measure commits (response parity)", async () => {
      const distSpec = loadSpatialInteraction("measure.distance")!;
      const areaSpec = loadSpatialInteraction("measure.area")!;
      const mkModel = () => {
        const t = new Model();
        const v0 = "v0" as VertexRef;
        const v1 = "v1" as VertexRef;
        t.vertices[v0] = { id: v0, position: [0, 0, 0] };
        t.vertices[v1] = { id: v1, position: [3, 4, 0] };
        const wf = "w0" as WireRef;
        const e0 = "e0" as EdgeRef;
        const f0 = "f0" as FaceRef;
        t.edges[e0] = { id: e0, vertexIds: [v0, v1] };
        t.wires[wf] = { id: wf, edgeIds: [e0] };
        t.faces[f0] = { id: f0, wireIds: [wf] };
        return t;
      };
      const k1d = new MeasureParityKernel();
      const k2d = new MeasureParityKernel();
      const rtPd = createInteractionRuntime(distSpec, {
        kernel: k1d,
        document: { model: mkModel(), nodes: [] },
        stateEngine: pureTsStateEngineProvider,
      });
      const rtSd = createInteractionRuntime(distSpec, {
        kernel: k2d,
        document: { model: mkModel(), nodes: [] },
        stateEngine: statelyStateEngineProvider,
      });
      await rtPd.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: "v0", editable: true }], modifiers: {} });
      await rtSd.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: "v0", editable: true }], modifiers: {} });
      await rtPd.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: "v1", editable: true }], modifiers: {} });
      await rtSd.send({ kind: "selection.changed", targets: [{ kind: "vertex", id: "v1", editable: true }], modifiers: {} });
      const rd = rtPd.getSnapshot().lastResponse!;
      const sd = rtSd.getSnapshot().lastResponse!;
      expect(rd.data).toBe(5);
      expect(sd.data).toBe(5);
      expect(isEmptyModelDiff(rd.diff)).toBe(false);
      expect(isEmptyModelDiff(sd.diff)).toBe(false);
      expect(rd.diff.edges?.added?.length).toBe(1);
      expect(sd.diff.edges?.added?.length).toBe(1);
      await assertSnapshotsEqual(rtPd, rtSd);

      const k1a = new MeasureParityKernel();
      const k2a = new MeasureParityKernel();
      const rtPa = createInteractionRuntime(areaSpec, {
        kernel: k1a,
        document: { model: mkModel(), nodes: [] },
        stateEngine: pureTsStateEngineProvider,
      });
      const rtSa = createInteractionRuntime(areaSpec, {
        kernel: k2a,
        document: { model: mkModel(), nodes: [] },
        stateEngine: statelyStateEngineProvider,
      });
      await rtPa.send({ kind: "selection.changed", targets: [{ kind: "face", id: "f0", editable: true }], modifiers: {} });
      await rtSa.send({ kind: "selection.changed", targets: [{ kind: "face", id: "f0", editable: true }], modifiers: {} });
      const ra = rtPa.getSnapshot().lastResponse!;
      const sa = rtSa.getSnapshot().lastResponse!;
      expect(ra.data).toBe(42);
      expect(sa.data).toBe(42);
      expect(isEmptyModelDiff(ra.diff)).toBe(false);
      expect(isEmptyModelDiff(sa.diff)).toBe(false);
      expect(ra.diff.anchors?.added?.length).toBe(1);
      expect(sa.diff.anchors?.added?.length).toBe(1);
      await assertSnapshotsEqual(rtPa, rtSa);
    });
  });
}
// #endregion 🧪️Tests
