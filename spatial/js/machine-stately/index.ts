// #region 🧲Header
/** @emoji 🎭 `@spatial/js-machine-stately` — XState `StateEngine` for `InteractionSpec.machine`; transitions mirror spec while `applyTransition` owns effects. See `.repo/✍️/spatial.md`. */
// #endregion 🧲Header

// #region 📥Imports
import { createActor, setup } from "xstate";
import {
	applyTransition,
	buildAreaInteractionSpec,
	buildBoxInteractionSpec,
	buildDistanceInteractionSpec,
	cellRef,
	createInteractionRuntime,
	initialContextForSpec,
	isEmptyTopologyDiff,
	listSpatialInteractions,
	loadSpatialInteraction,
	pureTsStateEngineProvider,
	type InteractionEvent,
	type InteractionRuntime,
	type InteractionSpec,
	type EdgeRef,
	type FaceRef,
	type KernelAdapter,
	type ActionRegistry,
	type StateEngine,
	type StateEngineProvider,
	type StateEngineSendResult,
	TopologyGraph,
	type Vec3,
	type VertexRef,
	type WireRef,
} from "@spatial/js-core";
// #endregion 📥Imports

// #region 🎭AdvanceEvent
type StatelyAdvance = { type: "__advance"; interactionKind: string; branch: number };
// #endregion 🎭AdvanceEvent

// #region 🎭MachineBuild
/** @emoji 🎭 Builds a flat XState chart isomorphic to `spec.machine` (`__advance` encodes branch index). */
function buildStatelyMachine(spec: InteractionSpec, initial: string) {
	const states: Record<
		string,
		{
			on?: {
				__advance: readonly {
					readonly guard: (args: { event: StatelyAdvance }) => boolean;
					readonly target: string;
				}[];
			};
		}
	> = {};
	for (const st of spec.machine.states) {
		const sId = st.name;
		const rows: { guard: (args: { event: StatelyAdvance }) => boolean; target: string }[] = [];
		if (st.on) {
			for (const h of st.on) {
				const choices = h.transitions;
				for (let i = 0; i < choices.length; i++) {
					const tr = choices[i]!;
					const tgt = (tr.target ?? sId) as string;
					rows.push({
						guard: ({ event }) => event.interactionKind === h.event && event.branch === i,
						target: tgt,
					});
				}
			}
		}
		states[sId] = rows.length ? { on: { __advance: rows } } : {};
	}
	return setup({
		types: { events: {} as StatelyAdvance },
	}).createMachine({
		id: `spatial-interaction-${spec.id}`,
		initial,
		states,
	});
}

/** @emoji 📊 One transition row for `machine.json` / Mermaid (matches `__advance` branch order). */
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

/** @emoji 📊 Serializable state node summary. */
export interface SpatialStatelyMachineStateView {
	readonly id: string;
	readonly final: boolean;
	readonly selectionAccept?: readonly string[];
}

/** @emoji 📊 Single spatial interaction as a viewable state machine (edges + Mermaid). */
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

/** @emoji 📊 Catalog of built-in interactions for Stately/Mermaid viewers (`machine.json`). */
export interface SpatialStatelyMachineCatalogView {
	readonly kind: "spatial.stately-machine-view/v1";
	readonly schemaVersion: "1.0";
	readonly generatedAt: string;
	readonly machines: readonly SpatialStatelyMachineView[];
	readonly mermaidCombined: string;
}

/** @emoji 📊 Collects flat transition rows from `InteractionSpec.machine` (same order as `StatelyStateEngine`). */
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

/** @emoji 📊 Builds one view document for a loaded `InteractionSpec` (interaction metadata for labels/keys). */
export function buildSpatialStatelyMachineViewForSpec(
	spec: InteractionSpec,
	meta: { readonly hostKey: string; readonly interactionLabel: string },
): SpatialStatelyMachineView {
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

/** @emoji 📊 Full catalog from `listSpatialInteractions` / optional `interactionIds` filter. */
export function buildSpatialStatelyMachineCatalogView(opts?: {
	readonly interactionIds?: readonly string[];
	readonly generatedAt?: string;
}): SpatialStatelyMachineCatalogView {
	const want = opts?.interactionIds?.length ? new Set(opts.interactionIds) : null;
	const machines: SpatialStatelyMachineView[] = [];
	for (const p of listSpatialInteractions()) {
		if (want && !want.has(p.id)) continue;
		const spec = loadSpatialInteraction(p.id);
		if (!spec) continue;
		machines.push(buildSpatialStatelyMachineViewForSpec(spec, { hostKey: p.key, interactionLabel: p.label }));
	}
	const generatedAt = opts?.generatedAt ?? new Date().toISOString();
	return {
		kind: "spatial.stately-machine-view/v1",
		schemaVersion: "1.0",
		generatedAt,
		machines,
		mermaidCombined: machines.map((m) => m.mermaid).join("\n\n"),
	};
}
// #endregion 🎭MachineBuild

// #region 🎭StatelyStateEngine
/** @emoji 🎭 XState-backed `StateEngine`; `send` runs `applyTransition` then syncs the actor via `__advance`. */
export class StatelyStateEngine implements StateEngine {
	private interactionState: string;
	private interactionContext: Record<string, unknown>;
	private machine: ReturnType<typeof buildStatelyMachine>;
	private actor!: { stop: () => void; start: () => void; send: (e: StatelyAdvance) => void; getSnapshot: () => { value: unknown } };

	constructor(private readonly spec: InteractionSpec) {
		this.interactionState = spec.machine.initial;
		this.interactionContext = initialContextForSpec(spec);
		this.machine = buildStatelyMachine(spec, this.interactionState);
		this.bootActor();
	}

	private bootActor(): void {
		this.actor = createActor(this.machine);
		this.actor.start();
	}

	private rebuildMachine(initial: string): void {
		this.machine = buildStatelyMachine(this.spec, initial);
		this.actor.stop();
		this.bootActor();
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

	async send(
		event: InteractionEvent,
		kernel?: KernelAdapter,
		topology?: TopologyGraph,
		actions?: ActionRegistry,
		derived?: import("@spatial/js-core").DerivedViewService,
	): Promise<StateEngineSendResult> {
		if (String(this.actor.getSnapshot().value) !== this.interactionState) {
			this.rebuildMachine(this.interactionState);
		}
		const r = await applyTransition(this.spec, this.interactionState, this.interactionContext, event, kernel, actions, topology, derived);
		if (!r.ok) return { ok: false };
		this.interactionState = r.nextState;
		this.actor.send({ type: "__advance", interactionKind: event.kind, branch: r.branchIndex });
		return { ok: true, transient: r.transient };
	}
}
// #endregion 🎭StatelyStateEngine

// #region 🎭Provider
/** @emoji 🎭 `StateEngineProvider` wiring `StatelyStateEngine` (XState v5). */
export const statelyStateEngineProvider: StateEngineProvider = {
	id: "xstate-stately",
	create(spec: InteractionSpec): StateEngine {
		return new StatelyStateEngine(spec);
	},
};
// #endregion 🎭Provider

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	class StubKernel implements KernelAdapter {
		readonly id = "stub-parity";
		readonly operations = ["cell.createBox", "entity.tessellate"] as const;
		lastBox: { cornerA: Vec3; cornerB: Vec3; height: number } | null = null;
		async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }) {
			this.lastBox = input;
			return cellRef("stub-cell");
		}
		async volume() {
			return 0;
		}
		async tessellate() {
			return {
				positions: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
				indices: new Uint32Array([0, 1, 2]),
			};
		}
	}

	async function assertSnapshotsEqual(a: InteractionRuntime, b: InteractionRuntime) {
		const sa = a.getSnapshot();
		const sb = b.getSnapshot();
		expect(sb.state).toBe(sa.state);
		expect(sb.context).toEqual(sa.context);
		expect(sb.capabilities).toEqual(sa.capabilities);
		expect(sb.lastResponse?.ok).toBe(sa.lastResponse?.ok);
		expect(sb.lastResponse?.data).toEqual(sa.lastResponse?.data);
		expect(sb.lastResponse?.diff).toEqual(sa.lastResponse?.diff);
	}

	class MeasureParityKernel implements KernelAdapter {
		readonly id = "stub-measure-parity";
		readonly operations = ["surface.resolveFaces", "measure.distance", "measure.area"] as const;
		async createBoxFromCorners() {
			return cellRef("unused");
		}
		async volume() {
			return 0;
		}
		async tessellate() {
			return { positions: new Float32Array(), indices: new Uint32Array() };
		}
		async query(name: string, params: Record<string, unknown>) {
			if (name === "surface.resolveFaces") return [String(params.surfaceId ?? "")];
			return undefined;
		}
		async vertexDistance(a: VertexRef, b: VertexRef, topo: TopologyGraph) {
			const pa = topo.vertices[String(a)]?.position;
			const pb = topo.vertices[String(b)]?.position;
			if (!pa || !pb) return 0;
			return Math.hypot(pa[0] - pb[0], pa[1] - pb[1], pa[2] - pb[2]);
		}
		async faceArea(_f: FaceRef, _topo: TopologyGraph) {
			return 42;
		}
	}

	describe("@spatial/js-machine-stately", () => {
		it("buildSpatialStatelyMachineCatalogView lists all interactions with edges and mermaid", () => {
			const doc = buildSpatialStatelyMachineCatalogView();
			expect(doc.kind).toBe("spatial.stately-machine-view/v1");
			expect(doc.machines.length).toBeGreaterThanOrEqual(5);
			const box = doc.machines.find((m) => m.interactionId === "primitive.box");
			expect(box?.edges.length).toBeGreaterThan(0);
			expect(box?.mermaid).toContain("primitive_box");
			expect(doc.mermaidCombined.length).toBeGreaterThan(100);
		});

		it("matches pure-ts interaction snapshots through box workflow + commit", async () => {
			const spec = buildBoxInteractionSpec();
			const k1 = new StubKernel();
			const k2 = new StubKernel();
			const rtPure = createInteractionRuntime(spec, {
				kernel: k1,
				document: { topology: new TopologyGraph(), nodes: [] },
				stateEngine: pureTsStateEngineProvider,
			});
			const rtSt = createInteractionRuntime(spec, {
				kernel: k2,
				document: { topology: new TopologyGraph(), nodes: [] },
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
			const spec = buildBoxInteractionSpec();
			const k1 = new StubKernel();
			const k2 = new StubKernel();
			const rtPure = createInteractionRuntime(spec, {
				kernel: k1,
				document: { topology: new TopologyGraph(), nodes: [] },
				stateEngine: pureTsStateEngineProvider,
			});
			const rtSt = createInteractionRuntime(spec, {
				kernel: k2,
				document: { topology: new TopologyGraph(), nodes: [] },
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
			const distSpec = buildDistanceInteractionSpec();
			const areaSpec = buildAreaInteractionSpec();
			const mkTopo = () => {
				const t = new TopologyGraph();
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
				document: { topology: mkTopo(), nodes: [] },
				stateEngine: pureTsStateEngineProvider,
			});
			const rtSd = createInteractionRuntime(distSpec, {
				kernel: k2d,
				document: { topology: mkTopo(), nodes: [] },
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
			expect(isEmptyTopologyDiff(rd.diff)).toBe(true);
			expect(isEmptyTopologyDiff(sd.diff)).toBe(true);
			await assertSnapshotsEqual(rtPd, rtSd);

			const k1a = new MeasureParityKernel();
			const k2a = new MeasureParityKernel();
			const rtPa = createInteractionRuntime(areaSpec, {
				kernel: k1a,
				document: { topology: mkTopo(), nodes: [] },
				stateEngine: pureTsStateEngineProvider,
			});
			const rtSa = createInteractionRuntime(areaSpec, {
				kernel: k2a,
				document: { topology: mkTopo(), nodes: [] },
				stateEngine: statelyStateEngineProvider,
			});
			await rtPa.send({ kind: "selection.changed", targets: [{ kind: "face", id: "f0", editable: true }], modifiers: {} });
			await rtSa.send({ kind: "selection.changed", targets: [{ kind: "face", id: "f0", editable: true }], modifiers: {} });
			const ra = rtPa.getSnapshot().lastResponse!;
			const sa = rtSa.getSnapshot().lastResponse!;
			expect(ra.data).toBe(42);
			expect(sa.data).toBe(42);
			expect(isEmptyTopologyDiff(ra.diff)).toBe(true);
			await assertSnapshotsEqual(rtPa, rtSa);
		});
	});
}
// #endregion 🧪Tests
