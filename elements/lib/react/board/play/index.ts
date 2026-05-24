// #region 🧲Header
// 💻 elements/client/lib/board/play/index.ts — Framework-free board play: declarative window bodies, LOD measures, workbench wiring (no React).
// #endregion 🧲Header

import {
	CommandBus,
	Controller,
	registerDeclarativeSidePanelBody,
	registerDeclarativeWindowBody,
	ShellExtension,
	type ShellExtensionContext,
	type ShellExtensionManifest,
	Workbench,
	WorkbenchApp,
	WorkbenchMode,
	WorkbenchWindowKind,
	createWindowLayout,
	type ShellSidePanelBodyViewContext,
	type ShellWindowBodyViewContext,
	type ShellWindowMeasure,
	type UiNode,
	type WindowLayout,
} from "@elements/framework";

import nakaginFixtureJson from "./fixtures/nakagin-capsule-tower.board.json";
import {
	BOARD_LOD_MODE_AUTOMATIC,
	boardLodAutomaticSelectLabel,
	isBoardDrawLodKind,
	parseBoardFixtureV1,
	type BoardDrawLodKind,
	type BoardFixtureV1,
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

export const BOARD_PLAY_TABLE_LIBRARY_SURFACE_ID = "elements.board.play.table.library/v1";
export const BOARD_PLAY_TABLE_INSPECTOR_SURFACE_ID = "elements.board.play.table.inspector/v1";
export const BOARD_PLAY_TABLE_SETTINGS_SURFACE_ID = "elements.board.play.table.settings/v1";

export const BOARD_PLAY_LIBRARY_TAB_BODY_KEY = "elements.board.play.tab.library";
export const BOARD_PLAY_INSPECTOR_TAB_BODY_KEY = "elements.board.play.tab.inspector";
export const BOARD_PLAY_SETTINGS_TAB_BODY_KEY = "elements.board.play.tab.settings";

export const BOARD_PLAY_ICON_LIBRARY = "elements.board-play.icon.library";
export const BOARD_PLAY_ICON_INSPECTOR = "elements.board-play.icon.inspector";
export const BOARD_PLAY_ICON_SETTINGS = "elements.board-play.icon.settings";

export const BOARD_PLAY_LOD_TIERS: BoardDrawLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

export function boardPlayLodTierMenuLabel(tier: BoardDrawLodKind): string {
	return tier.charAt(0).toUpperCase() + tier.slice(1);
}

export const BOARD_PLAY_PACKAGE_ROOT = import.meta.url;

export const BOARD_PLAY_DEFAULT_FIXTURE: BoardFixtureV1 =
	parseBoardFixtureV1(nakaginFixtureJson as unknown) ?? (nakaginFixtureJson as BoardFixtureV1);

export const BOARD_PLAY_LAYOUT: WindowLayout = {
	root: {
		kind: "row",
		children: [
			{
				kind: "stack",
				size: 50,
				children: [createWindowLayout("board-overview", "Overview")],
			},
			{
				kind: "column",
				size: 50,
				children: [
					{ kind: "stack", size: 50, children: [createWindowLayout("board-detail", "Zoom")] },
					{ kind: "stack", size: 50, children: [createWindowLayout("board-selection", "Selection")] },
				],
			},
		],
	},
};
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
function boardPlayControllerFromContext(ctx: ShellWindowBodyViewContext | ShellSidePanelBodyViewContext): BoardPlayShellController | undefined {
	return ctx.workbench.getActiveApp()?.controller as BoardPlayShellController | undefined;
}

function buildBoardPlayDeclarativeBody(paneId: BoardPlayPaneId): (ctx: ShellWindowBodyViewContext) => UiNode {
	return (ctx) => {
		if (!boardPlayControllerFromContext(ctx)) {
			return { type: "text", value: "Missing board play controller" };
		}
		return {
			type: "stack",
			direction: "vertical",
			padding: "none",
			children: [{ type: "board", surfaceId: BOARD_PLAY_BOARD_SURFACE_ID, controllerId: BOARD_PLAY_CONTROLLER_ID, paneId }],
		};
	};
}

export const buildBoardPlayOverviewDeclarativeBody = buildBoardPlayDeclarativeBody("board-overview");
export const buildBoardPlayDetailDeclarativeBody = buildBoardPlayDeclarativeBody("board-detail");
export const buildBoardPlaySelectionDeclarativeBody = buildBoardPlayDeclarativeBody("board-selection");

/** @emoji 📑 Declarative library side tab: table host surface only. */
export function buildBoardPlayLibraryDeclarativePanel(ctx: ShellSidePanelBodyViewContext): UiNode {
	if (!boardPlayControllerFromContext(ctx)) {
		return { type: "text", value: "Missing board play controller" };
	}
	return { type: "table", surfaceId: BOARD_PLAY_TABLE_LIBRARY_SURFACE_ID, controllerId: BOARD_PLAY_CONTROLLER_ID };
}

/** @emoji 📑 Declarative selection inspector side tab: table host surface only. */
export function buildBoardPlayInspectorDeclarativePanel(ctx: ShellSidePanelBodyViewContext): UiNode {
	if (!boardPlayControllerFromContext(ctx)) {
		return { type: "text", value: "Missing board play controller" };
	}
	return { type: "table", surfaceId: BOARD_PLAY_TABLE_INSPECTOR_SURFACE_ID, controllerId: BOARD_PLAY_CONTROLLER_ID };
}

/** @emoji 📑 Declarative settings side tab: table host surface only. */
export function buildBoardPlaySettingsDeclarativePanel(ctx: ShellSidePanelBodyViewContext): UiNode {
	if (!boardPlayControllerFromContext(ctx)) {
		return { type: "text", value: "Missing board play controller" };
	}
	return { type: "table", surfaceId: BOARD_PLAY_TABLE_SETTINGS_SURFACE_ID, controllerId: BOARD_PLAY_CONTROLLER_ID };
}
//#endregion 🔖DeclarativeBodies

/** @emoji 🧩 Registers board play window kinds on the supplied controller (layout supplied by host). */
export function attachBoardPlayWindowKinds(controller: BoardPlayShellController, layout: unknown): WorkbenchApp {
	const app = new WorkbenchApp(BOARD_PLAY_APP_ID, "Board", undefined, controller, layout as never, []);
	app.defaultModeId = controller.mainMode.id;
	app.addMode(controller.mainMode);
	return app;
}

/** @emoji 🧩 Builds the board play {@link WorkbenchApp} with declarative side tabs. */
export function buildBoardPlayWorkbenchApp(controller: BoardPlayShellController): WorkbenchApp {
	const app = attachBoardPlayWindowKinds(controller, BOARD_PLAY_LAYOUT);
	app.leftTabs = [{ id: "board-play-library", iconId: BOARD_PLAY_ICON_LIBRARY, order: 0, bodyKey: BOARD_PLAY_LIBRARY_TAB_BODY_KEY }];
	app.rightTabs = [
		{ id: "board-play-inspector", iconId: BOARD_PLAY_ICON_INSPECTOR, order: 0, bodyKey: BOARD_PLAY_INSPECTOR_TAB_BODY_KEY },
		{ id: "board-play-settings", iconId: BOARD_PLAY_ICON_SETTINGS, order: 1, bodyKey: BOARD_PLAY_SETTINGS_TAB_BODY_KEY },
	];
	return app;
}

/** @emoji 📝 Registers board play declarative window + side-panel bodies on the framework host. */
export function registerBoardPlayDeclarativeBodies(): void {
	registerDeclarativeWindowBody(BOARD_PLAY_BODY_KEY_OVERVIEW, buildBoardPlayOverviewDeclarativeBody);
	registerDeclarativeWindowBody(BOARD_PLAY_BODY_KEY_DETAIL, buildBoardPlayDetailDeclarativeBody);
	registerDeclarativeWindowBody(BOARD_PLAY_BODY_KEY_SELECTION, buildBoardPlaySelectionDeclarativeBody);
	registerDeclarativeSidePanelBody(BOARD_PLAY_LIBRARY_TAB_BODY_KEY, buildBoardPlayLibraryDeclarativePanel);
	registerDeclarativeSidePanelBody(BOARD_PLAY_INSPECTOR_TAB_BODY_KEY, buildBoardPlayInspectorDeclarativePanel);
	registerDeclarativeSidePanelBody(BOARD_PLAY_SETTINGS_TAB_BODY_KEY, buildBoardPlaySettingsDeclarativePanel);
}

//#region 🔖Extension
export const BOARD_PLAY_EXTENSION_MANIFEST: ShellExtensionManifest = {
	id: "elements.board-play",
	label: "Board Play",
	version: "0.1.0",
	contributes: {
		apps: [
			{
				id: BOARD_PLAY_APP_ID,
				label: "Board",
				controllerId: BOARD_PLAY_CONTROLLER_ID,
				defaultLayout: BOARD_PLAY_LAYOUT,
				defaultModeId: "main",
				windowKinds: [
					{ id: "board-overview", label: "Overview", bodyKey: BOARD_PLAY_BODY_KEY_OVERVIEW },
					{ id: "board-detail", label: "Zoom", bodyKey: BOARD_PLAY_BODY_KEY_DETAIL },
					{ id: "board-selection", label: "Selection", bodyKey: BOARD_PLAY_BODY_KEY_SELECTION },
				],
				modes: [{ id: "main", label: "Board" }],
				leftTabs: [{ id: "board-play-library", iconId: BOARD_PLAY_ICON_LIBRARY, order: 0, bodyKey: BOARD_PLAY_LIBRARY_TAB_BODY_KEY }],
				rightTabs: [
					{ id: "board-play-inspector", iconId: BOARD_PLAY_ICON_INSPECTOR, order: 0, bodyKey: BOARD_PLAY_INSPECTOR_TAB_BODY_KEY },
					{ id: "board-play-settings", iconId: BOARD_PLAY_ICON_SETTINGS, order: 1, bodyKey: BOARD_PLAY_SETTINGS_TAB_BODY_KEY },
				],
			},
		],
	},
};

/** @emoji 🔌 VS Code–style board play extension: registers declarative bodies on activate. */
export const boardPlayExtension: ShellExtension = {
	id: BOARD_PLAY_EXTENSION_MANIFEST.id,
	activate(context: ShellExtensionContext): void {
		registerBoardPlayDeclarativeBodies();
	},
};

/** @emoji 🚀 Creates a {@link Workbench} with board play app + declarative bodies registered. */
export function bootstrapBoardPlayWorkbench(): Workbench {
	registerBoardPlayDeclarativeBodies();
	const wb = new Workbench();
	const ctrl = new BoardPlayShellController(wb.commandBus, () => wb.notify());
	wb.addApp(buildBoardPlayWorkbenchApp(ctrl));
	return wb;
}
//#endregion 🔖Extension

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("board play declarative shell", () => {
		it("declarative overview body references board host surface", () => {
			const wb = bootstrapBoardPlayWorkbench();
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

		it("declarative library panel references table host surface", () => {
			const wb = bootstrapBoardPlayWorkbench();
			const tree = buildBoardPlayLibraryDeclarativePanel({
				workbench: wb,
				windowKindId: "board-play-library",
				bodyKey: BOARD_PLAY_LIBRARY_TAB_BODY_KEY,
				activeModeId: "main",
				generation: 0,
			});
			expect(tree).toEqual({
				type: "table",
				surfaceId: BOARD_PLAY_TABLE_LIBRARY_SURFACE_ID,
				controllerId: BOARD_PLAY_CONTROLLER_ID,
			});
		});
	});
}
//#endregion 🧪Tests
