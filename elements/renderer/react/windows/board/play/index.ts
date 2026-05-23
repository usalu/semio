// #region 🧲Header
// 💻 elements/client/lib/board/play/index.ts — Framework-free board play: declarative window bodies, LOD measures, workbench wiring (no React).
// #endregion 🧲Header

import {
	CommandBus,
	Controller,
	Workbench,
	WorkbenchApp,
	WorkbenchMode,
	WorkbenchWindowKind,
	type ShellWindowBodyViewContext,
	type ShellWindowMeasure,
	type UiNode,
} from "@elements/ui-shell";

import {
	BOARD_LOD_MODE_AUTOMATIC,
	boardLodAutomaticSelectLabel,
	isBoardDrawLodKind,
	type BoardDrawLodKind,
	type BoardLodModeKind,
} from "../index";

//#region 🔖Ids
export type BoardPlayPaneId = "board-overview" | "board-detail" | "board-selection";

export const BOARD_PLAY_APP_ID = "elements-board-play";
export const BOARD_PLAY_CONTROLLER_ID = "board-play";
export const BOARD_PLAY_BOARD_SURFACE_ID = "elements.board.play.board/v1";

export const BOARD_PLAY_BODY_KEY_OVERVIEW = "elements.board.play.overview";
export const BOARD_PLAY_BODY_KEY_DETAIL = "elements.board.play.detail";
export const BOARD_PLAY_BODY_KEY_SELECTION = "elements.board.play.selection";

export const BOARD_PLAY_LOD_TIERS: BoardDrawLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

export function boardPlayLodTierMenuLabel(tier: BoardDrawLodKind): string {
	return tier.charAt(0).toUpperCase() + tier.slice(1);
}

export const BOARD_PLAY_PACKAGE_ROOT = import.meta.url;
//#endregion 🔖Ids

//#region 🔖Controller
/** @emoji 🎛 Board play shell controller: per-pane LOD modes for declarative window measures. */
export class BoardPlayShellController extends Controller {
	readonly mainMode = new WorkbenchMode("main", "Board", undefined);
	private lodModeByPane: Record<BoardPlayPaneId, BoardLodModeKind>;
	private effectiveLodByPane: Record<BoardPlayPaneId, BoardDrawLodKind>;

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(BOARD_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.lodModeByPane = {
			"board-detail": BOARD_LOD_MODE_AUTOMATIC,
			"board-overview": BOARD_LOD_MODE_AUTOMATIC,
			"board-selection": BOARD_LOD_MODE_AUTOMATIC,
		};
		this.effectiveLodByPane = {
			"board-detail": "normal",
			"board-overview": "normal",
			"board-selection": "normal",
		};
		this.rebuildShellMode();
	}

	private lodMeasureForPane(paneId: BoardPlayPaneId): ShellWindowMeasure {
		return {
			kind: "select",
			id: `${paneId}-lod`,
			label: "LOD",
			value: this.lodModeByPane[paneId],
			items: [
				{ id: "automatic", value: BOARD_LOD_MODE_AUTOMATIC, label: boardLodAutomaticSelectLabel(this.effectiveLodByPane[paneId]) },
				...BOARD_PLAY_LOD_TIERS.map((tier) => ({ id: tier, value: tier, label: boardPlayLodTierMenuLabel(tier) })),
			],
			onChange: { controllerId: BOARD_PLAY_CONTROLLER_ID, command: "setLodModeForPane", args: { pane: paneId } },
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.windowKinds = [
			new WorkbenchWindowKind("board-overview", "Overview", BOARD_PLAY_BODY_KEY_OVERVIEW, undefined, [this.lodMeasureForPane("board-overview")]),
			new WorkbenchWindowKind("board-detail", "Zoom", BOARD_PLAY_BODY_KEY_DETAIL, undefined, [this.lodMeasureForPane("board-detail")]),
			new WorkbenchWindowKind("board-selection", "Selection", BOARD_PLAY_BODY_KEY_SELECTION, undefined, [this.lodMeasureForPane("board-selection")]),
		];
	}

	override run(command: string, args?: unknown): void {
		switch (command) {
			case "setLodModeForPane": {
				const { pane, value } = args as { pane: BoardPlayPaneId; value?: string };
				if (pane !== "board-overview" && pane !== "board-detail" && pane !== "board-selection") break;
				if (value === BOARD_LOD_MODE_AUTOMATIC || (typeof value === "string" && isBoardDrawLodKind(value))) {
					this.lodModeByPane = { ...this.lodModeByPane, [pane]: value as BoardLodModeKind };
				}
				break;
			}
			case "setEffectiveLodForPane": {
				const { pane, lod } = args as { pane: BoardPlayPaneId; lod: BoardDrawLodKind };
				if (!isBoardDrawLodKind(lod)) break;
				if (this.effectiveLodByPane[pane] === lod) break;
				this.effectiveLodByPane = { ...this.effectiveLodByPane, [pane]: lod };
				break;
			}
			default:
				break;
		}
		this.rebuildShellMode();
		this.emit();
	}

	getLodModeByPane(): Readonly<Record<BoardPlayPaneId, BoardLodModeKind>> {
		return this.lodModeByPane;
	}

	getEffectiveLodByPane(): Readonly<Record<BoardPlayPaneId, BoardDrawLodKind>> {
		return this.effectiveLodByPane;
	}
}
//#endregion 🔖Controller

//#region 🔖DeclarativeBodies
function buildBoardPlayDeclarativeBody(paneId: BoardPlayPaneId): (ctx: ShellWindowBodyViewContext) => UiNode {
	return () => ({
		type: "stack",
		direction: "vertical",
		padding: "none",
		children: [{ type: "board", surfaceId: BOARD_PLAY_BOARD_SURFACE_ID, controllerId: BOARD_PLAY_CONTROLLER_ID, paneId }],
	});
}

export const buildBoardPlayOverviewDeclarativeBody = buildBoardPlayDeclarativeBody("board-overview");
export const buildBoardPlayDetailDeclarativeBody = buildBoardPlayDeclarativeBody("board-detail");
export const buildBoardPlaySelectionDeclarativeBody = buildBoardPlayDeclarativeBody("board-selection");
//#endregion 🔖DeclarativeBodies

/** @emoji 🧩 Registers board play window kinds on the supplied controller mode (layout supplied by host). */
export function attachBoardPlayWindowKinds(controller: BoardPlayShellController, layout: unknown): WorkbenchApp {
	const app = new WorkbenchApp(BOARD_PLAY_APP_ID, "Board", undefined, controller, layout as never, []);
	app.defaultModeId = controller.mainMode.id;
	app.addMode(controller.mainMode);
	return app;
}

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("board play declarative shell", () => {
		it("declarative overview body references board host surface", () => {
			const bus = new CommandBus();
			const wb = new Workbench();
			const ctrl = new BoardPlayShellController(bus, () => wb.notify());
			attachBoardPlayWindowKinds(ctrl, { root: { kind: "row", children: [] } });
			const tree = buildBoardPlayOverviewDeclarativeBody({
				workbench: wb,
				windowKindId: "board-overview",
				bodyKey: BOARD_PLAY_BODY_KEY_OVERVIEW,
				activeModeId: "main",
				generation: 0,
			});
			expect(tree.type).toBe("stack");
			if (tree.type !== "stack") return;
			expect(tree.children[0]).toEqual({
				type: "board",
				surfaceId: BOARD_PLAY_BOARD_SURFACE_ID,
				controllerId: BOARD_PLAY_CONTROLLER_ID,
				paneId: "board-overview",
			});
		});
	});
}
//#endregion 🧪Tests
