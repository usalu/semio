// #region 🧲Header
/** @emoji 🗄️ VCS play app — version control editor and history. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	WindowKindRuntime,
	buildVcsWindowBody,
	createDefaultLayout,
	createPlayAppRuntime,
	enforcePlaygroundWindowEngagementInput,
	registerWindowBody,
	type DocumentVcsEnvelope,
	type HistoryColumn,
	type WindowEngagement,
  createPlaygroundApp,
  createProductPlaygroundPlatform,
} from "@semio-tech/framework-playground-core";
import {
	DocumentVcsStore,
	createDocumentVcsEnvelope,
	type Author,
	type DocumentVcsEnvelope as VcsEnvelope,
} from "./internal.ts";

export * from "./internal.ts";

export const VCS_PLAY_APP_ID = "vcs-play";
export const VCS_PLAY_CONTROLLER_ID = "vcs-play";
export const VCS_PLAY_SURFACE_ID_EDITOR = "vcs.play.editor";
export const VCS_PLAY_SURFACE_ID_HISTORY = "vcs.play.history";
export const VCS_PLAY_BODY_KEY_EDITOR = "vcs.play.editor";
export const VCS_PLAY_BODY_KEY_HISTORY = "vcs.play.history";
export const VCS_PLAY_WINDOW_KIND_EDITOR = "vcs-editor";
export const VCS_PLAY_WINDOW_KIND_HISTORY = "vcs-history";

export const VCS_PLAY_LAYOUT = createDefaultLayout(
	[VCS_PLAY_WINDOW_KIND_EDITOR, VCS_PLAY_WINDOW_KIND_HISTORY],
	"row",
	[30, 70],
	["Editor", "History"],
);

function vcsPlayCmd(command: string, args: Record<string, unknown> = {}): { controllerId: string; command: string; args: Record<string, unknown> } {
	return { controllerId: VCS_PLAY_CONTROLLER_ID, command, args };
}

function vcsEditorEngagement(projection: VcsDemoProjection, alternativeCount: number): WindowEngagement {
	return {
		sessionActive: false,
		input: {
			id: "vcs-editor-input",
			value: "",
			placeholder: "VCS editor",
			onChange: vcsPlayCmd("noop"),
		},
		status: [
			{ id: "vcs-counter", text: `counter ${projection.counter}` },
			{ id: "vcs-alternatives", text: `${alternativeCount} alternatives` },
		],
	};
}

function vcsHistoryEngagement(checkpointCount: number): WindowEngagement {
	return {
		sessionActive: false,
		input: {
			id: "vcs-history-input",
			value: "",
			placeholder: "History",
			onChange: vcsPlayCmd("noop"),
		},
		status: [{ id: "vcs-checkpoints", text: `${checkpointCount} checkpoints` }],
	};
}

/** @emoji 🎮 VCS play controller. */
export class VcsPlayController extends Controller {
	readonly mainMode = new ModeRuntime("main", "VCS", undefined);
	private readonly store = createVcsDemoStore();
	private interactionRevision = 0;
	private listeners = new Set<() => void>();

	constructor(bus: CommandBus, notifyPlatform: () => void) {
		super(VCS_PLAY_CONTROLLER_ID, bus, notifyPlatform);
		seedVcsDemoHistory(this.store);
		this.store.subscribe(() => this.bump());
		this.rebuildShellMode();
	}

	private rebuildShellMode(): void {
		const envelope = this.store.getEnvelope();
		this.mainMode.windowKinds = [
			new WindowKindRuntime(
				VCS_PLAY_WINDOW_KIND_EDITOR,
				"Editor",
				VCS_PLAY_BODY_KEY_EDITOR,
				undefined,
				[],
				vcsEditorEngagement(this.projection(), envelope.vcs.alternatives.length),
			),
			new WindowKindRuntime(
				VCS_PLAY_WINDOW_KIND_HISTORY,
				"History",
				VCS_PLAY_BODY_KEY_HISTORY,
				undefined,
				[],
				vcsHistoryEngagement(envelope.vcs.checkpoints.length),
			),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `VCS play window "${windowKind.id}"`);
		}
	}

	private bump(): void {
		this.interactionRevision += 1;
		this.rebuildShellMode();
		for (const listener of this.listeners) listener();
		this.emit();
	}

	subscribeSnapshot(listener: () => void): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	getInteractionRevision(): number {
		return this.interactionRevision;
	}

	getStore(): DocumentVcsStore<VcsDemoProjection, VcsDemoOp> {
		return this.store;
	}

	projection(): VcsDemoProjection {
		return this.store.projection();
	}

	historyColumns(): HistoryColumn[] {
		return this.store.historyColumns();
	}

	getEnvelope(): DocumentVcsEnvelope<VcsDemoProjection, VcsDemoOp> {
		return this.store.getEnvelope();
	}

	run(command: string, args: Record<string, unknown> = {}): void {
		switch (command) {
			case "incrementCounter": {
				const projection = this.projection();
				this.store.dispatch({ kind: "apply", operations: [{ op: "setCounter", counter: projection.counter + 1 }] });
				return;
			}
			case "commitCheckpoint": {
				const projection = this.projection();
				this.store.dispatch({
					kind: "commitCheckpoint",
					message: String(args.message ?? `Checkpoint @ ${projection.counter}`),
					authors: VCS_DEMO_AUTHORS.slice(0, 1),
				});
				return;
			}
			case "undo":
				this.store.dispatch({ kind: "undo" });
				return;
			case "redo":
				this.store.dispatch({ kind: "redo" });
				return;
			case "createAlternative": {
				const count = this.store.getEnvelope().vcs.alternatives.length;
				this.store.dispatch({ kind: "createAlternative", name: String(args.name ?? `alt-${count + 1}`) });
				return;
			}
			case "noop":
				return;
			default:
				return;
		}
	}
}

/** @emoji 🛝 Builds VCS play {@link AppRuntime}. */
export function buildVcsPlayAppRuntime(ctrl: VcsPlayController): AppRuntime {
	return createPlayAppRuntime(VCS_PLAY_APP_ID, "VCS", ctrl, VCS_PLAY_LAYOUT, ctrl.mainMode);
}

/** @emoji 🧩 Registers VCS play window bodies. */
export const vcsPlayWindowBodies: Readonly<Record<string, (ctx: import("@semio-tech/framework-platform-core").WindowBodyViewContext) => UiNode>> = {
	[VCS_PLAY_BODY_KEY_EDITOR]: () =>
		buildVcsWindowBody(VCS_PLAY_SURFACE_ID_EDITOR, VCS_PLAY_CONTROLLER_ID, "editor", "editor"),
	[VCS_PLAY_BODY_KEY_HISTORY]: () =>
		buildVcsWindowBody(VCS_PLAY_SURFACE_ID_HISTORY, VCS_PLAY_CONTROLLER_ID, "history", "history"),
};

export function registerVcsPlayDeclarativeBodies(): void {
	for (const [key, build] of Object.entries(vcsPlayWindowBodies)) registerWindowBody(key, build);
}

export { vcsPlayCmd };

//#region 🔖demo
/** @emoji 🗄️ VCS play demo projection and semantic edit operations. */

export const VCS_DEMO_SCHEMA = "vcs.demo";

export interface VcsDemoProjection {
	readonly schema: string;
	readonly title: string;
	readonly counter: number;
	readonly notes: string;
	readonly status: string;
	readonly tags: readonly string[];
}

export type VcsDemoOp =
	| { readonly op: "setCounter"; readonly counter: number }
	| { readonly op: "setTitle"; readonly title: string }
	| { readonly op: "setNotes"; readonly notes: string }
	| { readonly op: "setStatus"; readonly status: string }
	| { readonly op: "addTag"; readonly tag: string }
	| { readonly op: "removeTag"; readonly tag: string };

export const VCS_DEMO_AUTHORS: readonly Author[] = [
	{ id: "author-alice", name: "Alice", avatar: undefined },
	{ id: "author-bob", name: "Bob", avatar: undefined },
	{ id: "author-carol", name: "Carol", avatar: undefined },
];

export function emptyVcsDemoProjection(): VcsDemoProjection {
	return { schema: VCS_DEMO_SCHEMA, title: "VCS Demo", counter: 0, notes: "", status: "new", tags: [] };
}

export function applyVcsDemoOp(projection: VcsDemoProjection, operation: VcsDemoOp): VcsDemoProjection {
	switch (operation.op) {
		case "setCounter":
			return { ...projection, counter: operation.counter };
		case "setTitle":
			return { ...projection, title: operation.title };
		case "setNotes":
			return { ...projection, notes: operation.notes };
		case "setStatus":
			return { ...projection, status: operation.status };
		case "addTag":
			return projection.tags.includes(operation.tag)
				? projection
				: { ...projection, tags: [...projection.tags, operation.tag] };
		case "removeTag":
			return { ...projection, tags: projection.tags.filter((tag) => tag !== operation.tag) };
	}
}

export function backwardsVcsDemoOp(projection: VcsDemoProjection, operation: VcsDemoOp): readonly VcsDemoOp[] {
	switch (operation.op) {
		case "setCounter":
			return [{ op: "setCounter", counter: projection.counter }];
		case "setTitle":
			return [{ op: "setTitle", title: projection.title }];
		case "setNotes":
			return [{ op: "setNotes", notes: projection.notes }];
		case "setStatus":
			return [{ op: "setStatus", status: projection.status }];
		case "addTag":
			return [{ op: "removeTag", tag: operation.tag }];
		case "removeTag":
			return [{ op: "addTag", tag: operation.tag }];
	}
}

export function diffVcsDemoOp(_projection: VcsDemoProjection, operation: VcsDemoOp): unknown {
	return operation;
}

export function createVcsDemoStore(envelope?: VcsEnvelope<VcsDemoProjection, VcsDemoOp>): DocumentVcsStore<VcsDemoProjection, VcsDemoOp> {
	return new DocumentVcsStore({
		envelope: envelope ?? createDocumentVcsEnvelope(VCS_DEMO_SCHEMA, "vcs-demo", emptyVcsDemoProjection()),
		applyOp: applyVcsDemoOp,
		backwardsOp: backwardsVcsDemoOp,
		diffOp: diffVcsDemoOp,
	});
}

export function seedVcsDemoHistory(store: DocumentVcsStore<VcsDemoProjection, VcsDemoOp>): void {
	const alice = VCS_DEMO_AUTHORS[0]!;
	const bob = VCS_DEMO_AUTHORS[1]!;
	const carol = VCS_DEMO_AUTHORS[2]!;
	const lastCheckpointId = () => store.getEnvelope().vcs.checkpoints.at(-1)!.id;

	store.dispatch({
		kind: "apply",
		operations: [{ op: "setCounter", counter: 1 }, { op: "setTitle", title: "VCS Demo" }],
		description: "bootstrap",
	});
	store.dispatch({ kind: "commitCheckpoint", message: "Bootstrap", authors: [alice] });
	const c1 = lastCheckpointId();

	store.dispatch({
		kind: "apply",
		operations: [{ op: "setNotes", notes: "main line" }, { op: "setStatus", status: "draft" }],
	});
	store.dispatch({ kind: "commitCheckpoint", message: "Annotate main draft", authors: [alice, bob] });
	const c2 = lastCheckpointId();

	store.dispatch({ kind: "apply", operations: [{ op: "setCounter", counter: 2 }] });
	store.dispatch({ kind: "commitCheckpoint", message: "Main milestone", authors: [alice, bob, carol] });
	const c3 = lastCheckpointId();

	store.dispatch({ kind: "checkoutCheckpoint", checkpointId: c3 });
	store.dispatch({ kind: "createAlternative", name: "feature-a" });
	store.dispatch({
		kind: "apply",
		operations: [{ op: "setTitle", title: "Feature A" }, { op: "addTag", tag: "feature-a" }],
	});
	store.dispatch({ kind: "commitCheckpoint", message: "Start feature A", authors: [alice] });
	const c4 = lastCheckpointId();
	const featureAId = store.getEnvelope().activeAlternativeId!;

	store.dispatch({ kind: "apply", operations: [{ op: "setCounter", counter: 10 }] });
	store.dispatch({ kind: "commitCheckpoint", message: "Feature A progress", authors: [alice, bob] });

	store.dispatch({ kind: "checkoutCheckpoint", checkpointId: c3 });
	store.dispatch({ kind: "createAlternative", name: "feature-b" });
	store.dispatch({
		kind: "apply",
		operations: [{ op: "setTitle", title: "Feature B" }, { op: "setNotes", notes: "branch b" }],
	});
	store.dispatch({ kind: "commitCheckpoint", message: "Start feature B", authors: [bob] });
	const featureBId = store.getEnvelope().activeAlternativeId!;

	store.dispatch({ kind: "apply", operations: [{ op: "setCounter", counter: 20 }] });
	store.dispatch({ kind: "commitCheckpoint", message: "Feature B try", authors: [bob, carol] });

	store.dispatch({ kind: "checkoutCheckpoint", checkpointId: c3 });
	store.dispatch({ kind: "apply", operations: [{ op: "setStatus", status: "active" }] });
	store.dispatch({ kind: "commitCheckpoint", message: "Resume main", authors: [carol] });
	const c8 = lastCheckpointId();

	store.dispatch({ kind: "switchAlternative", alternativeId: featureAId });
	store.dispatch({
		kind: "apply",
		operations: [{ op: "setCounter", counter: 11 }, { op: "addTag", tag: "wip" }],
	});
	store.dispatch({ kind: "commitCheckpoint", message: "Feature A sprint", authors: [alice, carol] });

	store.dispatch({ kind: "checkoutCheckpoint", checkpointId: c4 });
	store.dispatch({ kind: "createAlternative", name: "feature-a-hotfix" });
	store.dispatch({ kind: "apply", operations: [{ op: "setStatus", status: "hotfix" }] });
	store.dispatch({ kind: "commitCheckpoint", message: "Hotfix off feature A", authors: [bob] });

	store.dispatch({ kind: "switchAlternative", alternativeId: featureBId });
	store.dispatch({ kind: "apply", operations: [{ op: "addTag", tag: "review" }] });
	store.dispatch({ kind: "commitCheckpoint", message: "Feature B review", authors: [bob] });

	store.dispatch({ kind: "checkoutCheckpoint", checkpointId: c8 });
	store.dispatch({
		kind: "apply",
		operations: [
			{ op: "setCounter", counter: 3 },
			{ op: "setNotes", notes: "main polish" },
			{ op: "addTag", tag: "release" },
		],
	});
	store.dispatch({ kind: "commitCheckpoint", message: "Main batch polish", authors: [alice, bob, carol] });

	store.dispatch({ kind: "apply", operations: [{ op: "setStatus", status: "done" }] });
	store.dispatch({ kind: "commitCheckpoint", message: "Main release", authors: [alice] });

	store.dispatch({ kind: "checkoutCheckpoint", checkpointId: c2 });
	store.dispatch({ kind: "createAlternative", name: "docs" });
	store.dispatch({ kind: "apply", operations: [{ op: "setNotes", notes: "documentation pass" }] });
	store.dispatch({ kind: "commitCheckpoint", message: "Docs branch", authors: [carol] });

	store.dispatch({ kind: "checkoutCheckpoint", checkpointId: c1 });
	store.dispatch({ kind: "createAlternative", name: "spike" });
	store.dispatch({ kind: "apply", operations: [{ op: "setTitle", title: "Spike prototype" }] });
	store.dispatch({ kind: "commitCheckpoint", message: "Spike experiment", authors: [bob, carol] });
}
//#endregion 🔖demo

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for vcs. */
export function buildVcsProgramDefinition(): PlatformDefinition {
	return {
		id: "vcs",
		name: "VCS",
		apiVersion: "1",
		apps: [{ id: "vcs", label: "VCS", controllerId: VCS_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🔖OsProgram
import { createTypedAppVcsHandler, mergeOsProgramDefinition, osBaselineResource, registerAppVcsHandler } from "@semio-tech/framework-os-core";
import type { OsProgramContribution } from "@semio-tech/framework-platform-core";

/** @emoji 🗄️ S app VCS handler for vcs demo documents. */
function createVcsDemoAppVcsHandler() {
	type Doc = { readonly schema: string; readonly title: string; readonly counter: number };
	type Op = { readonly op: "setDocument"; readonly document: Doc } | { readonly op: "setCounter"; readonly counter: number };
	return createTypedAppVcsHandler<Doc, Op>(
		"vcs.demo",
		"vcs.demo",
		() => ({ schema: "vcs.demo", title: "VCS Demo", counter: 0 }),
		(doc, op) => {
			if (op.op === "setDocument") return op.document;
			return { ...doc, counter: op.counter };
		},
	);
}

const vcsProgramContributionResources = {
		"vcs": { ...osBaselineResource("vcs.document", "vcs.demo", "vcs", [{ id: "explore", label: "Explore" }]), parameterFields: [{ fieldPath: "/counter", label: "Counter", type: "numeric" }] },
	};

/** @emoji 🧩 OS program contribution for vcs. */
export const vcsProgramContribution: OsProgramContribution = {
	programId: "vcs",
	register() {
		mergeOsProgramDefinition("vcs", buildVcsProgramDefinition(), vcsProgramContributionResources);
		registerAppVcsHandler(createVcsDemoAppVcsHandler());
	},
};
//#endregion 🔖OsProgram


//#region 🔖Play

/** @emoji 🛝 VCS playground app. */


export const vcsPlayAppDefinition = createPlaygroundApp({
	id: VCS_PLAY_APP_ID,
	label: "VCS",
	controllerId: VCS_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "vcs",
		resolveDedupe: ["react", "react-dom", "@semio-tech/ui-react", "@semio-tech/vcs-react"],
		optimizeDeps: { include: ["react", "react-dom"] },
	},
	createRuntime: () => {
		const runtime = createProductPlaygroundPlatform(VCS_PLAY_APP_ID, "VCS");
			const ctrl = new VcsPlayController(runtime.commandBus, () => runtime.notify());
			runtime.addApp(buildVcsPlayAppRuntime(ctrl));
			return runtime;
	},
	loadRenderer: async () => (await import("@semio-tech/vcs-react/play")).vcsAppRenderer,
});
//#endregion 🔖Play

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("seedVcsDemoHistory", () => {
		it("creates checkpoints and alternatives", () => {
			const local = createVcsDemoStore();
			seedVcsDemoHistory(local);
			const envelope = local.getEnvelope();
			expect(envelope.vcs.checkpoints.length).toBeGreaterThanOrEqual(15);
			expect(envelope.vcs.alternatives.length).toBeGreaterThanOrEqual(5);
			expect(local.historyColumns().length).toBeGreaterThanOrEqual(15);
			const parentCounts = new Map<string | undefined, number>();
			for (const checkpoint of envelope.vcs.checkpoints) {
				parentCounts.set(checkpoint.parentId, (parentCounts.get(checkpoint.parentId) ?? 0) + 1);
			}
			expect([...parentCounts.values()].some((count) => count >= 2)).toBe(true);
		});
	});

	describe("VcsPlayController", () => {
		it("increments counter through run", () => {
			const bus = new CommandBus();
			const ctrl = new VcsPlayController(bus, () => {});
			const before = ctrl.projection().counter;
			ctrl.run("incrementCounter");
			expect(ctrl.projection().counter).toBe(before + 1);
		});
	});
}
// #endregion 🧪Tests
