// #region 🧲Header
/** @emoji 📽 Presentation tile play — dev sandbox for one-to-many morph tile parameters on `@framework/playground/core`. */
// #endregion 🧲Header

import {
	CommandBus,
	Controller,
	Playground,
	PLAYGROUND_NO_FIXTURE_ID,
	type PlaygroundFixtureCatalog,
	type PlaygroundFixtureHost,
	isPlaygroundNoFixtureId,
	Platform,
	AppRuntime,
	ModeRuntime,
	WindowKindRuntime,
	buildPanelWindowBody,
	createPlayAppRuntime,
	createProductPlaygroundPlatform,
	createStackLayout,
	enforcePlaygroundWindowEngagementInput,
	platformFromViewContext,
	registerSidePanelBody,
	registerWindowBody,
	playgroundTreePanelRootItems,
	windowEngagementsEqual,
	type CommandDescriptor,
	type SidePanelBodyViewContext,
	type ToolItem,
	type UiNode,
	type UiTreeItemNode,
	type UiTreeNode,
	type WindowBodyViewContext,
	type WindowEngagement,
	uiDeclarativeSectionsToTree,
} from "@framework/playground/core";
import { Store } from "@framework/core";
import {
	buildTileMorphPrompt,
	clampNormalizedFraction,
	NORMALIZED_RECT_MIN_FRACTION,
	parseGridEngagement,
	populateTileDraftsFromGrid,
	type FigureTileDraft,
	type FigureTileSource,
} from "@framework/presentation/core";

import { bootstrapElementsSurfaceChromeDocument } from "@ui/react";

//#region 🔖Ids
export const PRESENTATION_PLAY_APP_ID = "presentation-tile-play";
export const PRESENTATION_PLAY_CONTROLLER_ID = "presentation-tile-play";
export const PRESENTATION_PLAY_SURFACE_ID = "presentation.tile.play/v1";
export const PRESENTATION_PLAY_BODY_KEY_MAIN = "presentation.tile.play.main";
export const PRESENTATION_PLAY_BODY_KEY_HIERARCHY = "presentation.tile.play.hierarchy";
export const PRESENTATION_PLAY_BODY_KEY_DETAILS = "presentation.tile.play.details";
export const PRESENTATION_PLAY_STORE_ID = "presentation-tile-play.snapshot";
export const PRESENTATION_PLAY_ICON_HIERARCHY = "presentation.play.icon.hierarchy";
export const PRESENTATION_PLAY_ICON_DETAILS = "presentation.play.icon.details";

/** @emoji 🖼 Default catalogue crop — aligned with `mit-bestand/präsentation/33.projektetage/spec.ts` (`CATALOGUE_FRAME`, `CATALOGUE_SOURCE_ASPECT`). */
export const PRESENTATION_PLAY_DEFAULT_SOURCE: FigureTileSource = {
	src: "/bauteilbörse.png",
	kind: "figure",
	frame: { x: 0.127, y: 0.1, width: 0.746, height: 0.75 },
	sourceAspect: 1222 / 896,
};

function presentationPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: PRESENTATION_PLAY_CONTROLLER_ID, command, args: args as never };
}

function clampTileCrop(crop: FigureTileDraft["crop"]): FigureTileDraft["crop"] {
	const width = Math.max(NORMALIZED_RECT_MIN_FRACTION, Math.min(crop.width, 1));
	const height = Math.max(NORMALIZED_RECT_MIN_FRACTION, Math.min(crop.height, 1));
	const x = clampNormalizedFraction(Math.min(crop.x, 1 - width));
	const y = clampNormalizedFraction(Math.min(crop.y, 1 - height));
	return { x, y, width, height };
}
//#endregion 🔖Ids

//#region 🔖Snapshot
export interface PresentationPlaySnapshot {
	readonly source: FigureTileSource;
	readonly tiles: readonly FigureTileDraft[];
	readonly selectedIds: readonly string[];
	readonly clipboardPrompt: string | null;
	readonly clipboardEpoch: number;
}

export const PRESENTATION_PLAY_IDLE_SNAPSHOT: PresentationPlaySnapshot = {
	source: PRESENTATION_PLAY_DEFAULT_SOURCE,
	tiles: [],
	selectedIds: [],
	clipboardPrompt: null,
	clipboardEpoch: 0,
};

class PresentationPlaySnapshotStore extends Store<PresentationPlaySnapshot> {
	constructor(private readonly controller: PresentationPlayController) {
		super();
	}

	getSnapshot(): PresentationPlaySnapshot {
		return this.controller.getSnapshot();
	}

	bump(): void {
		this.notify();
	}
}
//#endregion 🔖Snapshot

//#region 🔖Controller
let nextTileSerial = 0;

function newTileId(prefix = "tile"): string {
	nextTileSerial += 1;
	return `${prefix}-${nextTileSerial}`;
}

/** @emoji 🎛 Presentation tile play controller: grid seed, tile edits, LLM prompt export. */
export class PresentationPlayController extends Controller implements PlaygroundFixtureHost {
	readonly mainMode = new ModeRuntime("main", "Tile Morph", undefined);
	private activeFixtureId = PLAYGROUND_NO_FIXTURE_ID;
	private readonly snapshotStore: PresentationPlaySnapshotStore;
	private snapshotCache: PresentationPlaySnapshot | null = null;
	source: FigureTileSource = { ...PRESENTATION_PLAY_DEFAULT_SOURCE };
	tiles: FigureTileDraft[] = [];
	selectedIds: string[] = [];
	clipboardPrompt: string | null = null;
	clipboardEpoch = 0;
	private engagementInput = "";

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(PRESENTATION_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.snapshotStore = new PresentationPlaySnapshotStore(this);
		this.provideStore(PRESENTATION_PLAY_STORE_ID, this.snapshotStore);
		this.rebuildShellMode();
	}

	private rebuildSnapshotCache(): void {
		this.snapshotCache = {
			source: this.source,
			tiles: [...this.tiles],
			selectedIds: [...this.selectedIds],
			clipboardPrompt: this.clipboardPrompt,
			clipboardEpoch: this.clipboardEpoch,
		};
	}

	getSnapshot(): PresentationPlaySnapshot {
		if (!this.snapshotCache) {
			this.rebuildSnapshotCache();
		}
		return this.snapshotCache!;
	}

	private bumpSnapshot(): void {
		this.rebuildSnapshotCache();
		this.snapshotStore.bump();
	}

	private syncPresentationState(): void {
		this.bumpSnapshot();
		this.emit();
	}

	private syncShell(): void {
		this.rebuildShellMode();
		this.syncPresentationState();
	}

	private windowEngagement(): WindowEngagement {
		return {
			input: {
				id: "engagement-input",
				value: this.engagementInput,
				placeholder: "Grid (3x5), add, clear, copy prompt",
				onChange: presentationPlayCmd("engagementInput"),
				onSubmit: presentationPlayCmd("engagementSubmit"),
			},
			possibleEngagements: [
				{ id: "grid-3x5", label: "3x5", command: presentationPlayCmd("engagementSubmit", { value: "3x5" }) },
				{ id: "grid-2x2", label: "2x2", command: presentationPlayCmd("engagementSubmit", { value: "2x2" }) },
				{ id: "add-tile", label: "Add", command: presentationPlayCmd("engagementSubmit", { value: "add" }) },
				{ id: "clear-tiles", label: "Clear", command: presentationPlayCmd("engagementSubmit", { value: "clear" }) },
				{ id: "copy-prompt", label: "Copy prompt", command: presentationPlayCmd("copyPrompt") },
			],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = {
			create: [
				{
					id: "presentation.play.seed-3x5",
					kind: "button",
					iconId: "layout-grid",
					label: "3×5 grid",
					order: 0,
					controllerId: PRESENTATION_PLAY_CONTROLLER_ID,
					command: "seedGrid",
					args: { rows: 3, columns: 5 },
				},
				{
					id: "presentation.play.add",
					kind: "button",
					iconId: "plus",
					label: "Add tile",
					order: 1,
					controllerId: PRESENTATION_PLAY_CONTROLLER_ID,
					command: "addTile",
				},
				{
					id: "presentation.play.clear",
					kind: "button",
					iconId: "x",
					label: "Clear",
					order: 2,
					controllerId: PRESENTATION_PLAY_CONTROLLER_ID,
					command: "clearTiles",
				},
			],
			actions: [
				{
					id: "presentation.play.copy",
					kind: "button",
					iconId: "copy",
					label: "Copy prompt",
					order: 0,
					controllerId: PRESENTATION_PLAY_CONTROLLER_ID,
					command: "copyPrompt",
				},
				{
					id: "presentation.play.delete",
					kind: "button",
					iconId: "trash-2",
					label: "Delete",
					order: 1,
					controllerId: PRESENTATION_PLAY_CONTROLLER_ID,
					command: "deleteSelection",
				},
			],
		};
		this.mainMode.windowKinds = [
			new WindowKindRuntime(
				"tile-editor",
				"Tile editor",
				PRESENTATION_PLAY_BODY_KEY_MAIN,
				undefined,
				[],
				this.windowEngagement(),
			),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Presentation play window "${windowKind.id}"`);
		}
	}

	private seedGrid(rows: number, columns: number): void {
		this.tiles = populateTileDraftsFromGrid({ source: this.source, rows, columns });
		this.selectedIds = this.tiles.length > 0 ? [this.tiles[0]!.id] : [];
		this.syncShell();
	}

	private applyEngagement(value: string): boolean {
		const trimmed = value.trim();
		if (!trimmed) {
			return false;
		}
		const grid = parseGridEngagement(trimmed);
		if (grid) {
			this.seedGrid(grid.rows, grid.columns);
			return true;
		}
		if (trimmed.toLowerCase() === "add") {
			this.run("addTile");
			return true;
		}
		if (trimmed.toLowerCase() === "clear") {
			this.run("clearTiles");
			return true;
		}
		if (trimmed.toLowerCase() === "copy prompt" || trimmed.toLowerCase() === "copy") {
			this.run("copyPrompt");
			return true;
		}
		return false;
	}

	private syncWindowEngagement(): void {
		const existing = this.mainMode.windowKinds[0];
		if (!existing) {
			return;
		}
		const next = this.windowEngagement();
		if (windowEngagementsEqual(existing.engagement, next)) {
			return;
		}
		existing.engagement = next;
		this.mainMode.windowKinds = [...this.mainMode.windowKinds];
		this.emit();
	}

	getFixtureCatalog(): PlaygroundFixtureCatalog {
		return { activeFixtureId: this.activeFixtureId, options: [] };
	}

	override run(command: string, args?: unknown): void {
		switch (command) {
			case "setActiveFixture": {
				const fixtureId = (args as { fixtureId?: string }).fixtureId ?? "";
				const nextId = isPlaygroundNoFixtureId(fixtureId) ? PLAYGROUND_NO_FIXTURE_ID : fixtureId;
				if (nextId === this.activeFixtureId) break;
				this.activeFixtureId = nextId;
				if (isPlaygroundNoFixtureId(nextId)) {
					this.source = { ...PRESENTATION_PLAY_DEFAULT_SOURCE };
					this.tiles = [];
					this.selectedIds = [];
					this.syncShell();
				}
				break;
			}
			case "setSource": {
				const { src, sourceAspect, kind, pdfPage } = args as {
					src?: string;
					sourceAspect?: number;
					kind?: FigureTileSource["kind"];
					pdfPage?: number;
				};
				if (typeof src !== "string") {
					break;
				}
				const trimmed = src.trim();
				if (!trimmed) {
					this.source = { ...PRESENTATION_PLAY_DEFAULT_SOURCE };
					this.tiles = [];
					this.selectedIds = [];
					break;
				}
				const replaced = trimmed !== this.source.src;
				const mediaKind = kind ?? this.source.kind ?? "figure";
				this.source = {
					src: trimmed,
					kind: mediaKind,
					frame: { x: 0, y: 0, width: 1, height: 1 },
					...(sourceAspect !== undefined ? { sourceAspect } : this.source.sourceAspect !== undefined ? { sourceAspect: this.source.sourceAspect } : {}),
					...(mediaKind === "pdf" ? { pdfPage: pdfPage ?? this.source.pdfPage ?? 1 } : {}),
				};
				if (replaced) {
					this.tiles = [];
					this.selectedIds = [];
				}
				break;
			}
			case "setFrame": {
				const { frame } = args as { frame?: FigureTileSource["frame"] };
				if (frame) {
					this.source = { ...this.source, frame };
				}
				break;
			}
			case "seedGrid": {
				const { rows, columns } = args as { rows?: number; columns?: number };
				if (typeof rows === "number" && typeof columns === "number") {
					this.seedGrid(rows, columns);
					return;
				}
				break;
			}
			case "addTile": {
				const { crop } = (args ?? {}) as { crop?: FigureTileDraft["crop"] };
				const id = newTileId();
				const nextCrop = crop ?? { x: 0.1, y: 0.1, width: 0.2, height: 0.2 };
				this.tiles = [...this.tiles, { id, name: id, crop: nextCrop }];
				this.selectedIds = [id];
				break;
			}
			case "deleteTile": {
				const { id } = (args ?? {}) as { id?: string };
				const targetIds = typeof id === "string" ? [id] : this.selectedIds;
				if (targetIds.length === 0) {
					break;
				}
				const remove = new Set(targetIds);
				this.tiles = this.tiles.filter((tile) => !remove.has(tile.id));
				this.selectedIds = this.selectedIds.filter((selected) => !remove.has(selected));
				break;
			}
			case "deleteSelection": {
				if (this.selectedIds.length === 0) {
					break;
				}
				const remove = new Set(this.selectedIds);
				this.tiles = this.tiles.filter((tile) => !remove.has(tile.id));
				this.selectedIds = [];
				break;
			}
			case "renameTile": {
				const { id, name, value } = args as { id?: string; name?: string; value?: string };
				const nextName = name ?? value;
				if (typeof id === "string" && typeof nextName === "string") {
					this.tiles = this.tiles.map((tile) => (tile.id === id ? { ...tile, name: nextName.trim() || tile.name } : tile));
				}
				break;
			}
			case "patchTileCrop": {
				const { id, field, value } = args as { id?: string; field?: keyof FigureTileDraft["crop"]; value?: number };
				if (typeof id !== "string" || !field || typeof value !== "number" || !Number.isFinite(value)) {
					break;
				}
				const tile = this.tiles.find((row) => row.id === id);
				if (!tile) {
					break;
				}
				const crop = clampTileCrop({ ...tile.crop, [field]: value });
				this.tiles = this.tiles.map((row) => (row.id === id ? { ...row, crop } : row));
				break;
			}
			case "setTileCrop": {
				const { id, crop } = args as { id?: string; crop?: FigureTileDraft["crop"] };
				if (typeof id === "string" && crop) {
					this.tiles = this.tiles.map((tile) => (tile.id === id ? { ...tile, crop } : tile));
				}
				break;
			}
			case "setSelectedIds": {
				const { ids } = args as { ids?: readonly string[] };
				const validIds = new Set(this.tiles.map((tile) => tile.id));
				this.selectedIds = (ids ?? []).filter((id) => validIds.has(id));
				break;
			}
			case "clearTiles": {
				this.tiles = [];
				this.selectedIds = [];
				break;
			}
			case "copyPrompt": {
				this.clipboardPrompt = buildTileMorphPrompt(this.source, this.tiles);
				this.clipboardEpoch += 1;
				break;
			}
			case "engagementInput": {
				this.engagementInput = String((args as { value?: string }).value ?? "");
				this.syncWindowEngagement();
				return;
			}
			case "engagementSubmit": {
				const value = String((args as { value?: string }).value ?? this.engagementInput);
				if (this.applyEngagement(value)) {
					this.engagementInput = "";
				}
				this.syncWindowEngagement();
				this.bumpSnapshot();
				this.emit();
				return;
			}
			default:
				break;
		}
		this.syncPresentationState();
	}
}
//#endregion 🔖Controller

//#region 🔖DeclarativeBodies
function buildPresentationPlayMainBody(_ctx: WindowBodyViewContext): UiNode {
	return buildPanelWindowBody(PRESENTATION_PLAY_SURFACE_ID, PRESENTATION_PLAY_CONTROLLER_ID);
}

function presentationPlayControllerFromContext(
	ctx: SidePanelBodyViewContext | WindowBodyViewContext,
): PresentationPlayController | undefined {
	return platformFromViewContext(ctx)?.getActiveApp()?.controller as PresentationPlayController | undefined;
}

function buildPresentationPlayHierarchyBody(ctx: SidePanelBodyViewContext): UiNode {
	const controller = presentationPlayControllerFromContext(ctx);
	if (!controller) {
		return { type: "text", value: "Missing presentation play controller" };
	}
	const snapshot = controller.getSnapshot();
	const items: UiTreeItemNode[] = snapshot.tiles.map((tile) => ({
		id: tile.id,
		label: tile.name,
		description: `x=${tile.crop.x.toFixed(3)} y=${tile.crop.y.toFixed(3)} w=${tile.crop.width.toFixed(3)} h=${tile.crop.height.toFixed(3)}`,
		selected: snapshot.selectedIds.includes(tile.id),
		command: presentationPlayCmd("setSelectedIds", { ids: [tile.id] }),
	}));
	return {
		...playgroundTreePanelRootItems("presentation-tile-play.tiles", items.length ? items : [{ id: "empty", label: "(no tiles — seed a grid)" }], {
			selectedIds: snapshot.selectedIds,
		}),
		selectionChange: presentationPlayCmd("setSelectedIds"),
	};
}

function cropField(
	tileId: string,
	field: keyof FigureTileDraft["crop"],
	label: string,
	value: number,
): UiNode {
	return {
		type: "field",
		id: `presentation.play.tile.${tileId}.${field}`,
		label,
		child: {
			type: "input",
			id: `presentation.play.tile.${tileId}.${field}.input`,
			inputKind: "number",
			value: value.toFixed(6),
			commit: "blur",
			onChange: presentationPlayCmd("patchTileCrop", { id: tileId, field }),
		},
	};
}

export function buildPresentationPlayDetailsBody(ctx: SidePanelBodyViewContext): UiTreeNode {
	const controller = presentationPlayControllerFromContext(ctx);
	if (!controller) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "presentation.play.details.missing", label: "Tile", children: [{ type: "text", value: "Missing presentation play controller" }] },
		]);
	}
	const snapshot = controller.getSnapshot();
	if (snapshot.selectedIds.length !== 1) {
		return uiDeclarativeSectionsToTree([
			{
				type: "section",
				id: "presentation.play.details.empty",
				label: "Tile",
				children: [
					{
						type: "text",
						value:
							snapshot.selectedIds.length === 0
								? "Select a tile in the canvas or workbench hierarchy."
								: `${snapshot.selectedIds.length} tiles selected — pick one in the hierarchy to edit.`,
					},
				],
			},
		]);
	}
	const tile = snapshot.tiles.find((row) => row.id === snapshot.selectedIds[0]);
	if (!tile) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "presentation.play.details.not-found", label: "Tile", children: [{ type: "text", value: "Selected tile not found." }] },
		]);
	}
	return uiDeclarativeSectionsToTree([
		{
			type: "section",
			id: "presentation.play.details.tile",
			label: tile.name,
			children: [
				{
					type: "field",
					id: `presentation.play.tile.${tile.id}.name`,
					label: "Name",
					child: {
						type: "input",
						id: `presentation.play.tile.${tile.id}.name.input`,
						inputKind: "text",
						value: tile.name,
						commit: "blur",
						onChange: presentationPlayCmd("renameTile", { id: tile.id }),
					},
				},
				{ type: "field", id: `presentation.play.tile.${tile.id}.id`, label: "Id", child: { type: "text", value: tile.id } },
				cropField(tile.id, "x", "X", tile.crop.x),
				cropField(tile.id, "y", "Y", tile.crop.y),
				cropField(tile.id, "width", "Width", tile.crop.width),
				cropField(tile.id, "height", "Height", tile.crop.height),
				{
					type: "button",
					id: `presentation.play.tile.${tile.id}.delete`,
					label: "Delete tile",
					command: presentationPlayCmd("deleteTile", { id: tile.id }),
				},
				{
					type: "button",
					id: "presentation.play.details.delete-selection",
					label: "Delete selection",
					command: presentationPlayCmd("deleteSelection"),
				},
			],
		},
	]);
}

export function registerPresentationPlayDeclarativeBodies(): void {
	registerWindowBody(PRESENTATION_PLAY_BODY_KEY_MAIN, buildPresentationPlayMainBody);
	registerSidePanelBody(PRESENTATION_PLAY_BODY_KEY_HIERARCHY, buildPresentationPlayHierarchyBody);
	registerSidePanelBody(PRESENTATION_PLAY_BODY_KEY_DETAILS, buildPresentationPlayDetailsBody);
}

function buildPresentationPlayAppRuntime(controller: PresentationPlayController): AppRuntime {
	const layout = createStackLayout(["tile-editor"], ["Tile editor"]);
	const app = createPlayAppRuntime(PRESENTATION_PLAY_APP_ID, "semio · framework · product · presentation", controller, layout, controller.mainMode);
	app.panelTabs = [
		{
			id: `${PRESENTATION_PLAY_APP_ID}.hierarchy`,
			iconId: PRESENTATION_PLAY_ICON_HIERARCHY,
			panel: "workbench",
			order: 0,
			bodyKey: PRESENTATION_PLAY_BODY_KEY_HIERARCHY,
			label: "Hierarchy",
		},
		{
			id: `${PRESENTATION_PLAY_APP_ID}.details`,
			iconId: PRESENTATION_PLAY_ICON_DETAILS,
			panel: "details",
			order: 0,
			bodyKey: PRESENTATION_PLAY_BODY_KEY_DETAILS,
			label: "Tile",
		},
	];
	return app;
}

/** @emoji 🚀 Creates a {@link Platform} with the presentation tile play app. */
export function buildPresentationPlayRuntime(): Platform {
	registerPresentationPlayDeclarativeBodies();
	const runtime = new Platform({ id: PRESENTATION_PLAY_APP_ID });
	const ctrl = new PresentationPlayController(runtime.commandBus, () => runtime.notify());
	runtime.addApp(buildPresentationPlayAppRuntime(ctrl));
	return runtime;
}

/** @emoji 🛝 Presentation tile play harness as a single {@link Playground}. */
export class PresentationPlay extends Playground {
	readonly id = PRESENTATION_PLAY_APP_ID;
	readonly keybindings = [
		{ key: "Delete", controllerId: PRESENTATION_PLAY_CONTROLLER_ID, command: "deleteSelection" },
		{ key: "Backspace", controllerId: PRESENTATION_PLAY_CONTROLLER_ID, command: "deleteSelection" },
	];

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new PresentationPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildPresentationPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerPresentationPlayDeclarativeBodies();
	}
}
//#endregion 🔖DeclarativeBodies

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("PresentationPlayController", () => {
		it("seeds tiles from grid engagement", () => {
			const bus = new CommandBus();
			const ctrl = new PresentationPlayController(bus, () => undefined);
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "seedGrid", { rows: 2, columns: 2 });
			expect(ctrl.tiles).toHaveLength(4);
		});

		it("builds clipboard prompt on copyPrompt", () => {
			const bus = new CommandBus();
			const ctrl = new PresentationPlayController(bus, () => undefined);
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "seedGrid", { rows: 1, columns: 1 });
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "copyPrompt");
			expect(ctrl.clipboardPrompt).toContain("morphTo");
			expect(ctrl.clipboardEpoch).toBe(1);
		});

		it("returns a stable snapshot reference until state changes", () => {
			const bus = new CommandBus();
			const ctrl = new PresentationPlayController(bus, () => undefined);
			const before = ctrl.getSnapshot();
			expect(ctrl.getSnapshot()).toBe(before);
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "addTile");
			expect(ctrl.getSnapshot()).not.toBe(before);
		});

		it("builds hierarchy tree from controller snapshot", () => {
			const bus = new CommandBus();
			const runtime = new Platform({ id: PRESENTATION_PLAY_APP_ID });
			const ctrl = new PresentationPlayController(bus, () => runtime.notify());
			runtime.addApp(buildPresentationPlayAppRuntime(ctrl));
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "seedGrid", { rows: 1, columns: 2 });
			const body = buildPresentationPlayHierarchyBody({
				runtime,
				windowKindId: `${PRESENTATION_PLAY_APP_ID}.hierarchy`,
				bodyKey: PRESENTATION_PLAY_BODY_KEY_HIERARCHY,
				activeModeId: null,
				generation: runtime.generation,
			}) as UiTreeNode;
			expect(body.type).toBe("tree");
			expect(body.sections[0]?.items).toHaveLength(2);
		});

		it("builds hierarchy tree from platform-shaped side-panel context", () => {
			const bus = new CommandBus();
			const platform = new Platform({ id: PRESENTATION_PLAY_APP_ID });
			const ctrl = new PresentationPlayController(bus, () => platform.notify());
			platform.addApp(buildPresentationPlayAppRuntime(ctrl));
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "seedGrid", { rows: 1, columns: 1 });
			const body = buildPresentationPlayHierarchyBody({
				platform,
				windowKindId: `${PRESENTATION_PLAY_APP_ID}.hierarchy`,
				bodyKey: PRESENTATION_PLAY_BODY_KEY_HIERARCHY,
				activeModeId: null,
				generation: platform.generation,
			}) as UiTreeNode;
			expect(body.type).toBe("tree");
			expect(body.sections[0]?.items).toHaveLength(1);
		});

		it("builds details inspector for the selected tile", () => {
			const bus = new CommandBus();
			const runtime = new Platform({ id: PRESENTATION_PLAY_APP_ID });
			const ctrl = new PresentationPlayController(bus, () => runtime.notify());
			runtime.addApp(buildPresentationPlayAppRuntime(ctrl));
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "addTile");
			const tileId = ctrl.tiles[0]!.id;
			const body = buildPresentationPlayDetailsBody({
				runtime,
				windowKindId: `${PRESENTATION_PLAY_APP_ID}.details`,
				bodyKey: PRESENTATION_PLAY_BODY_KEY_DETAILS,
				activeModeId: null,
				generation: runtime.generation,
			});
			expect(body.type).toBe("tree");
			const items = body.sections[0]?.items ?? [];
			expect(items.some((item) => item.control?.type === "field" && item.id === `presentation.play.tile.${tileId}.name`)).toBe(true);
		});

		it("selects and deletes tiles", () => {
			const bus = new CommandBus();
			const ctrl = new PresentationPlayController(bus, () => undefined);
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "seedGrid", { rows: 2, columns: 2 });
			const targetId = ctrl.tiles[0]!.id;
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "setSelectedIds", { ids: [targetId] });
			expect(ctrl.selectedIds).toEqual([targetId]);
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "deleteSelection");
			expect(ctrl.tiles).toHaveLength(3);
			expect(ctrl.selectedIds).toEqual([]);
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "setSelectedIds", { ids: [ctrl.tiles[0]!.id] });
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "deleteTile", { id: ctrl.tiles[0]!.id });
			expect(ctrl.tiles).toHaveLength(2);
		});

		it("loads catalogue figure by default and replaces via setSource", () => {
			const bus = new CommandBus();
			const ctrl = new PresentationPlayController(bus, () => undefined);
			expect(ctrl.source.src).toBe("/bauteilbörse.png");
			expect(ctrl.source.sourceAspect).toBeCloseTo(1222 / 896);
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "setSource", { src: "/figure.png", sourceAspect: 2, kind: "figure" });
			expect(ctrl.source.src).toBe("/figure.png");
			expect(ctrl.source.kind).toBe("figure");
			expect(ctrl.source.sourceAspect).toBe(2);
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "addTile");
			expect(ctrl.tiles).toHaveLength(1);
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "setSource", { src: "/clip.mp4", sourceAspect: 1.777, kind: "video" });
			expect(ctrl.source.kind).toBe("video");
			expect(ctrl.tiles).toHaveLength(0);
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "setSource", { src: "/paper.pdf", kind: "pdf", pdfPage: 3, sourceAspect: 595 / 842 });
			expect(ctrl.source.kind).toBe("pdf");
			expect(ctrl.source.pdfPage).toBe(3);
		});
	});
}
//#endregion 🧪Tests

//#region 🔖Boot
if (
	typeof document !== "undefined" &&
	document.getElementById("root") != null &&
	!import.meta.vitest &&
	import.meta.env.PUZZLE_PLAY_ENTRY === "presentation"
) {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootPresentationPlay } = await import("@framework/playground/renderer/react/presentation");
		bootPresentationPlay(new PresentationPlay());
	})();
}
//#endregion 🔖Boot
