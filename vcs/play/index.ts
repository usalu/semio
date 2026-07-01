// #region 🧲Header
/** @emoji 🗄️ VCS play harness on `@semio-tech/framework-playground-core`. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	Platform,
	Playground,
	WindowKindRuntime,
	buildVcsWindowBody,
	createDefaultLayout,
	createPlayAppRuntime,
	createProductPlaygroundPlatform,
	enforcePlaygroundWindowEngagementInput,
	registerWindowBody,
	type DocumentVcsEnvelope,
	type HistoryColumn,
	type WindowEngagement,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import type { DocumentVcsStore } from "@semio-tech/vcs-core";
import {
	createVcsDemoStore,
	seedVcsDemoHistory,
	VCS_DEMO_AUTHORS,
	type VcsDemoOp,
	type VcsDemoProjection,
} from "./demo.ts";

export const VCS_PLAY_APP_ID = "vcs-play";
export const VCS_PLAY_CONTROLLER_ID = "vcs-play";
export const VCS_PLAY_SURFACE_ID_EDITOR = "vcs.play.editor/v1";
export const VCS_PLAY_SURFACE_ID_HISTORY = "vcs.play.history/v1";
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

export function buildVcsPlayAppRuntime(ctrl: VcsPlayController): AppRuntime {
	return createPlayAppRuntime(VCS_PLAY_APP_ID, "VCS", ctrl, VCS_PLAY_LAYOUT, ctrl.mainMode);
}

export function registerVcsPlayDeclarativeBodies(): void {
	registerWindowBody(VCS_PLAY_BODY_KEY_EDITOR, () =>
		buildVcsWindowBody(VCS_PLAY_SURFACE_ID_EDITOR, VCS_PLAY_CONTROLLER_ID, "editor", "editor"));
	registerWindowBody(VCS_PLAY_BODY_KEY_HISTORY, () =>
		buildVcsWindowBody(VCS_PLAY_SURFACE_ID_HISTORY, VCS_PLAY_CONTROLLER_ID, "history", "history"));
}

export class PlaygroundVcs extends Playground {
	readonly id = VCS_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id, "VCS");
		const ctrl = new VcsPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildVcsPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerVcsPlayDeclarativeBodies();
	}
}

export { vcsPlayCmd };

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("seedVcsDemoHistory", () => {
		it("creates checkpoints and alternatives", () => {
			const local = createVcsDemoStore();
			seedVcsDemoHistory(local);
			expect(local.getEnvelope().vcs.checkpoints.length).toBeGreaterThanOrEqual(3);
			expect(local.getEnvelope().vcs.alternatives.length).toBeGreaterThanOrEqual(2);
			expect(local.historyColumns().length).toBeGreaterThanOrEqual(3);
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

// #region 🔖Boot
if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "vcs") {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootVcsPlay } = await import("@semio-tech/framework-playground-renderer-react/vcs");
		bootVcsPlay(new PlaygroundVcs());
	})();
}
// #endregion 🔖Boot
