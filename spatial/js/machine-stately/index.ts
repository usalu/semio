// #region 🧲Header
/** @emoji 🎭 `@spatial/js-machine-stately` — XState `StateEngine` for `FactorySpec.machine`; transitions mirror spec while `applyTransition` owns effects. See `.repo/✍️/spatial.md`. */
// #endregion 🧲Header

// #region 📥Imports
import { createActor, setup } from "xstate";
import {
	applyTransition,
	buildBoxCommandSpec as buildBoxFactorySpec,
	cellRef,
	createCommandRuntime as createFactoryRuntime,
	expandMachineTransitions,
	pureTsStateEngineProvider,
	type CommandEvent as FactoryEvent,
	type CommandRuntime as FactoryRuntime,
	type CommandSpec as FactorySpec,
	type KernelAdapter,
	type StateEngine,
	type StateEngineProvider,
	type StateEngineSendResult,
	TopologyGraph,
	type Vec3,
} from "@spatial/js-core";
// #endregion 📥Imports

// #region 🎭AdvanceEvent
type StatelyAdvance = { type: "__advance"; factoryKind: string; branch: number };
// #endregion 🎭AdvanceEvent

// #region 🎭MachineBuild
/** @emoji 🎭 Builds a flat XState chart isomorphic to `spec.machine` (`__advance` encodes branch index). */
function buildStatelyMachine(spec: FactorySpec, initial: string) {
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
	for (const sId of Object.keys(spec.machine.states)) {
		const st = spec.machine.states[sId]!;
		const rows: { guard: (args: { event: StatelyAdvance }) => boolean; target: string }[] = [];
		if (st.on) {
			for (const [eventKind, raw] of Object.entries(st.on)) {
				const choices = expandMachineTransitions(raw);
				for (let i = 0; i < choices.length; i++) {
					const tr = choices[i]!;
					const tgt = (tr.target ?? sId) as string;
					rows.push({
						guard: ({ event }) => event.factoryKind === eventKind && event.branch === i,
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
		id: `spatial-factory-${spec.id}`,
		initial,
		states,
	});
}
// #endregion 🎭MachineBuild

// #region 🎭StatelyStateEngine
/** @emoji 🎭 XState-backed `StateEngine`; `send` runs `applyTransition` then syncs the actor via `__advance`. */
export class StatelyStateEngine implements StateEngine {
	private factoryState: string;
	private readonly factoryContext: Record<string, unknown> = {};
	private machine: ReturnType<typeof buildStatelyMachine>;
	private actor!: { stop: () => void; start: () => void; send: (e: StatelyAdvance) => void; getSnapshot: () => { value: unknown } };

	constructor(private readonly spec: FactorySpec) {
		this.factoryState = spec.machine.initial;
		this.machine = buildStatelyMachine(spec, this.factoryState);
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
		return this.factoryState;
	}

	getContext(): Record<string, unknown> {
		return this.factoryContext;
	}

	reset(): void {
		for (const k of Object.keys(this.factoryContext)) delete this.factoryContext[k];
		this.factoryState = this.spec.machine.initial;
		this.rebuildMachine(this.factoryState);
	}

	restore(state: string, context: Record<string, unknown>): void {
		for (const k of Object.keys(this.factoryContext)) delete this.factoryContext[k];
		Object.assign(this.factoryContext, context);
		this.factoryState = state;
		this.rebuildMachine(state);
	}

	async send(event: FactoryEvent, kernel?: KernelAdapter): Promise<StateEngineSendResult> {
		if (String(this.actor.getSnapshot().value) !== this.factoryState) {
			this.rebuildMachine(this.factoryState);
		}
		const r = await applyTransition(this.spec, this.factoryState, this.factoryContext, event, kernel);
		if (!r.ok) return { ok: false };
		this.factoryState = r.nextState;
		this.actor.send({ type: "__advance", factoryKind: event.kind, branch: r.branchIndex });
		return { ok: true, transient: r.transient };
	}
}
// #endregion 🎭StatelyStateEngine

// #region 🎭Provider
/** @emoji 🎭 `StateEngineProvider` wiring `StatelyStateEngine` (XState v5). */
export const statelyStateEngineProvider: StateEngineProvider = {
	id: "xstate-stately",
	create(spec: FactorySpec): StateEngine {
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

	async function assertSnapshotsEqual(a: FactoryRuntime, b: FactoryRuntime) {
		const sa = a.getSnapshot();
		const sb = b.getSnapshot();
		expect(sb.state).toBe(sa.state);
		expect(sb.context).toEqual(sa.context);
		expect(sb.capabilities).toEqual(sa.capabilities);
	}

	describe("@spatial/js-machine-stately", () => {
		it("matches pure-ts factory snapshots through box workflow + commit", async () => {
			const spec = buildBoxFactorySpec();
			const k1 = new StubKernel();
			const k2 = new StubKernel();
			const rtPure = createFactoryRuntime(spec, {
				kernel: k1,
				document: { topology: new TopologyGraph(), nodes: [] },
				stateEngine: pureTsStateEngineProvider,
			});
			const rtSt = createFactoryRuntime(spec, {
				kernel: k2,
				document: { topology: new TopologyGraph(), nodes: [] },
				stateEngine: statelyStateEngineProvider,
			});
			await assertSnapshotsEqual(rtPure, rtSt);
			await rtPure.send({ kind: "start" });
			await rtSt.send({ kind: "start" });
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
			await rtPure.commit();
			await rtSt.commit();
			await assertSnapshotsEqual(rtPure, rtSt);
			expect(k1.lastBox).toEqual(k2.lastBox);
		});

		it("matches pure-ts after factory-local undo", async () => {
			const spec = buildBoxFactorySpec();
			const k1 = new StubKernel();
			const k2 = new StubKernel();
			const rtPure = createFactoryRuntime(spec, {
				kernel: k1,
				document: { topology: new TopologyGraph(), nodes: [] },
				stateEngine: pureTsStateEngineProvider,
			});
			const rtSt = createFactoryRuntime(spec, {
				kernel: k2,
				document: { topology: new TopologyGraph(), nodes: [] },
				stateEngine: statelyStateEngineProvider,
			});
			await rtPure.send({ kind: "start" });
			await rtSt.send({ kind: "start" });
			await rtPure.send({ kind: "pointer.down", point: [1, 1, 0] as Vec3, modifiers: {} });
			await rtSt.send({ kind: "pointer.down", point: [1, 1, 0] as Vec3, modifiers: {} });
			await assertSnapshotsEqual(rtPure, rtSt);
			rtPure.undo();
			rtSt.undo();
			await assertSnapshotsEqual(rtPure, rtSt);
		});
	});
}
// #endregion 🧪Tests
