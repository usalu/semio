// #region 🧲️Header
/** 🎭️ `@semio-tech/cad-js/stately` — `@semio-tech/machine`-backed `StateEngine` for `InteractionSpec.machine`; transitions mirror spec while `applyTransition` owns effects. See `.🧬semio/🦑️repo/✍️/spatial.md`. Was XState-backed; ported to the in-house statechart kernel (Wave 8, runtime-dependency-elimination) — see the kernel's own flat/guarded fixture tests in `🧰️framework/🔨️modules/🔄️machine/🟦️.ts`. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { EventId, GuardId, NodeId, ROOT, init, macrostep, NullInspector, type Command, type GuardFn, type Machine, type MachineDefinition, type MachineSpec, type NodeDef, type Snapshot, type StatechartEvent, type TransitionDef } from "@semio-tech/machine";
import type { Vec3 } from "@semio-tech/framework-3d-js";
import { emptyMeshTransfer, solidRef } from "@semio-tech/framework-3d-js";
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
} from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts";
import { isEmptyModelDiff, type SpatialKernel, type ModelDiff } from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️.ts";
import { applyTransition, pureTsStateEngineProvider, type ActionRegistry, type StateEngine, type StateEngineProvider, type StateEngineSendResult } from "../🎬️actions/🟦️.ts";
import { createInteractionRuntime, loadSpatialInteraction, type InteractionRuntime } from "../🗿️artifact/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🎭️AdvanceEvent
/** 🎭️ The one wire-level event kind every compiled `StatelyMachineSpec` reacts to; rows for
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

/** 🎭️ Associated types bound to `@semio-tech/machine`'s generic kernel for the stately adapter. */
export interface StatelyMachineSpec extends MachineSpec {
  Context: undefined;
  Event: StatelyAdvanceEvent;
  Input: undefined;
  Output: never;
  Effect: never;
}
// #endregion 🎭️AdvanceEvent

// #region 🎭️MachineBuild
/** 🎭️ Builds a flat, one-level `MachineDefinition` isomorphic to `spec.machine` — every state is
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

/** 📊️ One transition row for `🔣️machine.json` / Mermaid (matches `__advance` branch order). */
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

/** 📊️ Serializable state node summary. */
export interface SpatialStatelyMachineStateView {
  readonly id: string;
  readonly final: boolean;
  readonly selectionAccept?: readonly string[];
}

/** 📊️ Single spatial interaction as a viewable state machine (edges + Mermaid). */
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

/** 📊️ Catalog of model-definition interactions for Stately/Mermaid viewers (`🔣️machine.json`). */
export interface SpatialStatelyMachineCatalogView {
  readonly kind: "spatial.stately-machine-view/v1";
  readonly schemaVersion: "1.0";
  readonly generatedAt: string;
  readonly machines: readonly SpatialStatelyMachineView[];
  readonly mermaidCombined: string;
}

/** 📊️ Collects flat transition rows from `InteractionSpec.machine` (same order as `StatelyStateEngine`). */
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

/** 📊️ Builds one view document for a loaded `InteractionSpec` (interaction metadata for labels/keys). */
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

/** 📊️ Model-definition-scoped interaction machines from shipped interaction JSON. */
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
/** 🎭️ `@semio-tech/machine`-backed `StateEngine`; `send` runs `applyTransition` then syncs the
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

  async send(event: InteractionEvent, kernel?: SpatialKernel, model?: Model, actions?: ActionRegistry, preview?: import("../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️.ts").SpatialPreviewKernel, activeModelDefinitionId?: string | null): Promise<StateEngineSendResult> {
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
/** 🎭️ `StateEngineProvider` wiring `StatelyStateEngine` (`@semio-tech/machine`-backed). */
export const statelyStateEngineProvider: StateEngineProvider = {
  id: "machine-stately",
  create(spec: InteractionSpec): StateEngine {
    return new StatelyStateEngine(spec);
  },
};
// #endregion 🎭️Provider

// #region 🧪️Tests
const __spatialStatelyTestRuntime = import.meta.vitest ? await import("../🏃️runtime/🟦️.ts") : null;
const __spatialStatelyTestKernel = import.meta.vitest ? await import("../../../../../../../../../../../🧑‍💻dev/📐️cad/🧪️tests/🔮️spatial-kernel/🧱️brepjs/🟦️.ts") : null;

/** 🎒️ The values this module hands its extracted suite `./🧪️tests/🧪️semio-tech-cad-js-stately/🟦️.ts`. */
export type StatelyTestDependencies = {
  readonly Model: typeof Model;
  readonly NullInspector: typeof NullInspector;
  readonly __spatialStatelyTestKernel: typeof __spatialStatelyTestKernel;
  readonly __spatialStatelyTestRuntime: typeof __spatialStatelyTestRuntime;
  readonly buildSpatialStatelyMachineCatalogView: typeof buildSpatialStatelyMachineCatalogView;
  readonly buildStatelyMachine: typeof buildStatelyMachine;
  readonly createInteractionRuntime: typeof createInteractionRuntime;
  readonly defaultModelDefinitionId: typeof defaultModelDefinitionId;
  readonly emptyMeshTransfer: typeof emptyMeshTransfer;
  readonly init: typeof init;
  readonly isEmptyModelDiff: typeof isEmptyModelDiff;
  readonly listSpatialInteractionsForModelDefinition: typeof listSpatialInteractionsForModelDefinition;
  readonly loadSpatialInteraction: typeof loadSpatialInteraction;
  readonly macrostep: typeof macrostep;
  readonly makeAdvanceEvent: typeof makeAdvanceEvent;
  readonly pureTsStateEngineProvider: typeof pureTsStateEngineProvider;
  readonly solidRef: typeof solidRef;
  readonly statelyStateEngineProvider: typeof statelyStateEngineProvider;
};

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️semio-tech-cad-js-stately/🟦️.ts");
  await registerTests1(import.meta.vitest, { Model, NullInspector, __spatialStatelyTestKernel, __spatialStatelyTestRuntime, buildSpatialStatelyMachineCatalogView, buildStatelyMachine, createInteractionRuntime, defaultModelDefinitionId, emptyMeshTransfer, init, isEmptyModelDiff, listSpatialInteractionsForModelDefinition, loadSpatialInteraction, macrostep, makeAdvanceEvent, pureTsStateEngineProvider, solidRef, statelyStateEngineProvider }, { url: import.meta.url });
}
// #endregion 🧪️Tests
