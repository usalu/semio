// #region 🧲Header
/** @emoji 📽 Presentation tile play — dev sandbox for one-to-many morph tile parameters on `@semio-tech/framework-playground-core`. */
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
	type ToolLeaf,
	toolCollection,
	type UiNode,
	type UiTreeItemNode,
	type UiTreeNode,
	type WindowBodyViewContext,
	type WindowEngagement,
	uiDeclarativeSectionsToTree,
	FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
	FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
	FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID,
	FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
	FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
	FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
	UI_INSPECTOR_MIXED_PLACEHOLDER,
	uiInspectorGroupsToTree,
	uiInspectorMixedNumber,
	uiInspectorMixedText,
	uiInspectorReadonlyField,
	type UiInspectorFieldGroup,
} from "@semio-tech/framework-playground-core";
import { Store, DocumentVcsStore, applyJsonReplaceOp, createDocumentVcsEnvelope, recordJsonProjectionChange, type JsonReplaceOp } from "@semio-tech/framework-core";
import {
	buildTileMorphPrompt,
	clampNormalizedFraction,
	NORMALIZED_RECT_MIN_FRACTION,
	parseGridEngagement,
	populateTileDraftsFromGrid,
	type FigureTileDraft,
	type FigureTileSource,
	type PresentationDeckV1,
} from "@semio-tech/framework-presentation-core";

import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";

//#region 🔖Ids
export const PRESENTATION_PLAY_APP_ID = "presentation-tile-play";
export const PRESENTATION_PLAY_CONTROLLER_ID = "presentation-tile-play";
export const PRESENTATION_PLAY_SURFACE_ID = "presentation.tile.play/v1";
export const PRESENTATION_PLAY_BODY_KEY_MAIN = "presentation.tile.play.main";
export const PRESENTATION_PLAY_BODY_KEY_HIERARCHY = "presentation.tile.play.hierarchy";
export const PRESENTATION_PLAY_BODY_KEY_CATALOGUE = "presentation.tile.play.catalogue";
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
	readonly mainMode = new ModeRuntime("main", "Edit", undefined);
	private activeFixtureId = PLAYGROUND_NO_FIXTURE_ID;
	private readonly docStore = new DocumentVcsStore<PresentationDeckV1, JsonReplaceOp<PresentationDeckV1>>({
		envelope: createDocumentVcsEnvelope("presentation.deck/v1", "presentation-tile-play", {
			schema: "presentation.deck/v1",
			source: PRESENTATION_PLAY_DEFAULT_SOURCE,
			tiles: [],
		}),
		applyOp: applyJsonReplaceOp,
	});
	private readonly snapshotStore: PresentationPlaySnapshotStore;
	private snapshotCache: PresentationPlaySnapshot | null = null;
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

	private projection(): PresentationDeckV1 {
		return this.docStore.projection();
	}

	private commitDeck(next: PresentationDeckV1): void {
		recordJsonProjectionChange(this.docStore, next);
	}

	get source(): FigureTileSource {
		return this.projection().source;
	}

	get tiles(): readonly FigureTileDraft[] {
		return this.projection().tiles;
	}

	getDocumentVcsStore(): DocumentVcsStore<PresentationDeckV1, JsonReplaceOp<PresentationDeckV1>> {
		return this.docStore;
	}

	private rebuildSnapshotCache(): void {
		const deck = this.projection();
		this.snapshotCache = {
			source: deck.source,
			tiles: [...deck.tiles],
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
		this.mainMode.tools = [
			toolCollection("create", "plus", [
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
			]),
			toolCollection("actions", "more-horizontal", [
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
			]),
		];
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
		const tiles = populateTileDraftsFromGrid({ source: this.source, rows, columns });
		this.commitDeck({ ...this.projection(), tiles });
		this.selectedIds = tiles.length > 0 ? [tiles[0]!.id] : [];
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
					this.commitDeck({
						schema: "presentation.deck/v1",
						source: { ...PRESENTATION_PLAY_DEFAULT_SOURCE },
						tiles: [],
					});
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
				const deck = this.projection();
				if (!trimmed) {
					this.commitDeck({
						schema: "presentation.deck/v1",
						source: { ...PRESENTATION_PLAY_DEFAULT_SOURCE },
						tiles: [],
					});
					this.selectedIds = [];
					break;
				}
				const replaced = trimmed !== deck.source.src;
				const mediaKind = kind ?? deck.source.kind ?? "figure";
				const source: FigureTileSource = {
					src: trimmed,
					kind: mediaKind,
					frame: { x: 0, y: 0, width: 1, height: 1 },
					...(sourceAspect !== undefined ? { sourceAspect } : deck.source.sourceAspect !== undefined ? { sourceAspect: deck.source.sourceAspect } : {}),
					...(mediaKind === "pdf" ? { pdfPage: pdfPage ?? deck.source.pdfPage ?? 1 } : {}),
				};
				this.commitDeck({ ...deck, source, tiles: replaced ? [] : deck.tiles });
				if (replaced) this.selectedIds = [];
				break;
			}
			case "setFrame": {
				const { frame } = args as { frame?: FigureTileSource["frame"] };
				if (frame) {
					const deck = this.projection();
					this.commitDeck({ ...deck, source: { ...deck.source, frame } });
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
				const deck = this.projection();
				this.commitDeck({ ...deck, tiles: [...deck.tiles, { id, name: id, crop: nextCrop }] });
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
				const deck = this.projection();
				this.commitDeck({ ...deck, tiles: deck.tiles.filter((tile) => !remove.has(tile.id)) });
				this.selectedIds = this.selectedIds.filter((selected) => !remove.has(selected));
				break;
			}
			case "deleteSelection": {
				if (this.selectedIds.length === 0) {
					break;
				}
				const remove = new Set(this.selectedIds);
				const deck = this.projection();
				this.commitDeck({ ...deck, tiles: deck.tiles.filter((tile) => !remove.has(tile.id)) });
				this.selectedIds = [];
				break;
			}
			case "renameTiles": {
				const { ids, name, value } = args as { ids?: readonly string[]; name?: string; value?: string };
				const nextName = (value ?? name)?.trim();
				const targetIds = (ids ?? []).filter((id) => this.tiles.some((tile) => tile.id === id));
				if (!nextName || targetIds.length === 0) {
					break;
				}
				const targets = new Set(targetIds);
				const deck = this.projection();
				this.commitDeck({
					...deck,
					tiles: deck.tiles.map((tile) => (targets.has(tile.id) ? { ...tile, name: nextName } : tile)),
				});
				break;
			}
			case "renameTile": {
				const { id, name, value } = args as { id?: string; name?: string; value?: string };
				const nextName = name ?? value;
				if (typeof id === "string" && typeof nextName === "string") {
					this.run("renameTiles", { ids: [id], value: nextName });
				}
				break;
			}
			case "patchTileCrops": {
				const { ids, field, value } = args as { ids?: readonly string[]; field?: keyof FigureTileDraft["crop"]; value?: number };
				if (!field || typeof value !== "number" || !Number.isFinite(value)) {
					break;
				}
				const targetIds = new Set((ids ?? []).filter((id) => this.tiles.some((tile) => tile.id === id)));
				if (targetIds.size === 0) {
					break;
				}
				const deck = this.projection();
				this.commitDeck({
					...deck,
					tiles: deck.tiles.map((row) => {
						if (!targetIds.has(row.id)) return row;
						return { ...row, crop: clampTileCrop({ ...row.crop, [field]: value }) };
					}),
				});
				break;
			}
			case "patchTileCrop": {
				const { id, field, value } = args as { id?: string; field?: keyof FigureTileDraft["crop"]; value?: number };
				if (typeof id === "string" && field && typeof value === "number" && Number.isFinite(value)) {
					this.run("patchTileCrops", { ids: [id], field, value });
				}
				break;
			}
			case "setTileCrop": {
				const { id, crop } = args as { id?: string; crop?: FigureTileDraft["crop"] };
				if (typeof id === "string" && crop) {
					const deck = this.projection();
					this.commitDeck({ ...deck, tiles: deck.tiles.map((tile) => (tile.id === id ? { ...tile, crop } : tile)) });
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
				this.commitDeck({ ...this.projection(), tiles: [] });
				this.selectedIds = [];
				break;
			}
			case "copyPrompt": {
				this.clipboardPrompt = buildTileMorphPrompt(this.source, [...this.tiles]);
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

function presentationPlayInspectorPatch(tileIds: readonly string[], field: keyof FigureTileDraft["crop"]) {
	return presentationPlayCmd("patchTileCrops", { ids: tileIds, field });
}

function presentationPlayInspectorCropField(
	tileIds: readonly string[],
	field: keyof FigureTileDraft["crop"],
	label: string,
	values: readonly number[],
): UiNode {
	const mixed = uiInspectorMixedNumber(values);
	return {
		type: "field",
		id: `presentation.play.tile.crop.${field}`,
		label,
		child: {
			type: "input",
			id: `presentation.play.tile.crop.${field}.input`,
			inputKind: "number",
			value: mixed.uniform ? values[0]!.toFixed(6) : "",
			placeholder: mixed.uniform ? undefined : UI_INSPECTOR_MIXED_PLACEHOLDER,
			commit: "blur",
			onChange: presentationPlayInspectorPatch(tileIds, field),
		},
	};
}

export function buildPresentationPlayDetailsBody(ctx: SidePanelBodyViewContext): UiTreeNode {
	const controller = presentationPlayControllerFromContext(ctx);
	if (!controller) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "presentation.play.details.missing", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Missing presentation play controller" }] },
		]);
	}
	const snapshot = controller.getSnapshot();
	if (snapshot.selectedIds.length === 0) {
		return uiDeclarativeSectionsToTree([
			{
				type: "section",
				id: "presentation.play.details.empty",
				label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
				children: [{ type: "text", value: "Select a tile in the canvas or workbench hierarchy." }],
			},
		]);
	}
	const tiles = snapshot.selectedIds
		.map((id) => snapshot.tiles.find((row) => row.id === id))
		.filter((tile): tile is FigureTileDraft => Boolean(tile));
	if (!tiles.length) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "presentation.play.details.not-found", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Selected tile not found." }] },
		]);
	}
	const tileIds = tiles.map((tile) => tile.id);
	const nameMixed = uiInspectorMixedText(tiles.map((tile) => tile.name));
	const groups: UiInspectorFieldGroup[] = [
		{
			id: "presentation.play.details.crop",
			label: "Crop",
			fields: [
				presentationPlayInspectorCropField(tileIds, "x", "X", tiles.map((tile) => tile.crop.x)),
				presentationPlayInspectorCropField(tileIds, "y", "Y", tiles.map((tile) => tile.crop.y)),
				presentationPlayInspectorCropField(tileIds, "width", "Width", tiles.map((tile) => tile.crop.width)),
				presentationPlayInspectorCropField(tileIds, "height", "Height", tiles.map((tile) => tile.crop.height)),
			],
		},
		{
			id: "presentation.play.details.identity",
			label: "Identity",
			fields: [
				{
					type: "field",
					id: "presentation.play.tile.name",
					label: "Name",
					child: {
						type: "input",
						id: "presentation.play.tile.name.input",
						inputKind: "text",
						value: nameMixed.value,
						placeholder: nameMixed.placeholder,
						commit: "blur",
						onChange: presentationPlayCmd("renameTiles", { ids: tileIds }),
					},
				},
				uiInspectorReadonlyField(
					"presentation.play.tile.id",
					"Id",
					tileIds.length === 1 ? (tileIds[0] ?? "") : `${tileIds.length} selected`,
				),
				...(tileIds.length === 1
					? [
							{
								type: "button" as const,
								id: `presentation.play.tile.${tileIds[0]}.delete`,
								label: "Delete tile",
								command: presentationPlayCmd("deleteTile", { id: tileIds[0] }),
							},
						]
					: []),
				{
					type: "button",
					id: "presentation.play.details.delete-selection",
					label: "Delete selection",
					command: presentationPlayCmd("deleteSelection"),
				},
			],
		},
	];
	return uiInspectorGroupsToTree(groups);
}

function buildPresentationPlayCatalogueBody(ctx: SidePanelBodyViewContext): UiTreeNode {
	const controller = presentationPlayControllerFromContext(ctx);
	if (!controller) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "presentation.play.catalogue.missing", label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, children: [{ type: "text", value: "Missing presentation play controller" }] },
		]);
	}
	const snapshot = controller.getSnapshot();
	return uiDeclarativeSectionsToTree([
		{
			type: "section",
			id: "presentation.play.catalogue.templates",
			label: "Tile templates",
			children: [
				{ type: "text", value: "Seed morph tiles from figure templates." },
				{
					type: "button",
					id: "presentation.play.catalogue.seed-2x2",
					label: "Split 2×2 grid",
					command: presentationPlayCmd("seedGrid", { rows: 2, columns: 2 }),
				},
				{
					type: "button",
					id: "presentation.play.catalogue.seed-3x5",
					label: "Split 3×5 catalogue grid",
					command: presentationPlayCmd("seedGrid", { rows: 3, columns: 5 }),
				},
				{
					type: "button",
					id: "presentation.play.catalogue.add-tile",
					label: "Add single tile",
					command: presentationPlayCmd("addTile"),
				},
				{
					type: "button",
					id: "presentation.play.catalogue.clear",
					label: "Clear tiles",
					command: presentationPlayCmd("clearTiles"),
				},
			],
		},
		{
			type: "section",
			id: "presentation.play.catalogue.figure",
			label: "Figure templates",
			children: [
				{
					type: "button",
					id: "presentation.play.catalogue.figure.catalogue",
					label: "Use catalogue figure",
					command: presentationPlayCmd("setSource", { ...PRESENTATION_PLAY_DEFAULT_SOURCE }),
				},
				{
					type: "field",
					id: "presentation.play.catalogue.figure.src",
					label: "Active source",
					child: { type: "text", value: snapshot.source.src },
				},
				{
					type: "field",
					id: "presentation.play.catalogue.figure.kind",
					label: "Media kind",
					child: { type: "text", value: snapshot.source.kind ?? "figure" },
				},
			],
		},
	]);
}

export function registerPresentationPlayDeclarativeBodies(): void {
	registerWindowBody(PRESENTATION_PLAY_BODY_KEY_MAIN, buildPresentationPlayMainBody);
	registerSidePanelBody(PRESENTATION_PLAY_BODY_KEY_HIERARCHY, buildPresentationPlayHierarchyBody);
	registerSidePanelBody(PRESENTATION_PLAY_BODY_KEY_CATALOGUE, buildPresentationPlayCatalogueBody);
	registerSidePanelBody(PRESENTATION_PLAY_BODY_KEY_DETAILS, buildPresentationPlayDetailsBody);
}

function buildPresentationPlayAppRuntime(controller: PresentationPlayController): AppRuntime {
	const layout = createStackLayout(["tile-editor"], ["Tile editor"]);
	const app = createPlayAppRuntime(PRESENTATION_PLAY_APP_ID, "Presentation", controller, layout, controller.mainMode);
	app.panelTabs = [
		{
			id: `${PRESENTATION_PLAY_APP_ID}.hierarchy`,
			iconId: FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID,
			panel: "workbench",
			order: 0,
			bodyKey: PRESENTATION_PLAY_BODY_KEY_HIERARCHY,
			label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
		},
		{
			id: `${PRESENTATION_PLAY_APP_ID}.catalogue`,
			iconId: FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
			panel: "workbench",
			order: 1,
			bodyKey: PRESENTATION_PLAY_BODY_KEY_CATALOGUE,
			label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
		},
		{
			id: `${PRESENTATION_PLAY_APP_ID}.details`,
			iconId: FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
			panel: "details",
			order: 0,
			bodyKey: PRESENTATION_PLAY_BODY_KEY_DETAILS,
			label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
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

		it("builds catalogue workbench tab with tile templates", () => {
			const bus = new CommandBus();
			const runtime = new Platform({ id: PRESENTATION_PLAY_APP_ID });
			const ctrl = new PresentationPlayController(bus, () => runtime.notify());
			runtime.addApp(buildPresentationPlayAppRuntime(ctrl));
			const body = buildPresentationPlayCatalogueBody({
				runtime,
				windowKindId: `${PRESENTATION_PLAY_APP_ID}.catalogue`,
				bodyKey: PRESENTATION_PLAY_BODY_KEY_CATALOGUE,
				activeModeId: null,
				generation: runtime.generation,
			});
			expect(body.type).toBe("tree");
			const templateSection = body.sections.find((section) => section.id === "presentation.play.catalogue.templates");
			expect(templateSection).toBeDefined();
			expect(templateSection!.items.some((item) => item.id === "presentation.play.catalogue.seed-3x5")).toBe(true);
		});

		it("builds details inspector for the selected tile", () => {
			const bus = new CommandBus();
			const runtime = new Platform({ id: PRESENTATION_PLAY_APP_ID });
			const ctrl = new PresentationPlayController(bus, () => runtime.notify());
			runtime.addApp(buildPresentationPlayAppRuntime(ctrl));
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "addTile");
			const tileId = ctrl.tiles[0]!.id;
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "setSelectedIds", { ids: [tileId] });
			const body = buildPresentationPlayDetailsBody({
				runtime,
				windowKindId: `${PRESENTATION_PLAY_APP_ID}.details`,
				bodyKey: PRESENTATION_PLAY_BODY_KEY_DETAILS,
				activeModeId: null,
				generation: runtime.generation,
			});
			expect(body.type).toBe("tree");
			const items = body.sections.flatMap((section) => section.items);
			expect(items.some((item) => item.control?.type === "input" && item.id === "presentation.play.tile.name")).toBe(true);
		});

		it("builds mixed crop inspector for multi-select tiles", () => {
			const bus = new CommandBus();
			const runtime = new Platform({ id: PRESENTATION_PLAY_APP_ID });
			const ctrl = new PresentationPlayController(bus, () => runtime.notify());
			runtime.addApp(buildPresentationPlayAppRuntime(ctrl));
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "seedGrid", { rows: 2, columns: 2 });
			const tileIds = ctrl.tiles.slice(0, 2).map((tile) => tile.id);
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "setSelectedIds", { ids: tileIds });
			const body = buildPresentationPlayDetailsBody({
				runtime,
				windowKindId: `${PRESENTATION_PLAY_APP_ID}.details`,
				bodyKey: PRESENTATION_PLAY_BODY_KEY_DETAILS,
				activeModeId: null,
				generation: runtime.generation,
			});
			const cropSection = body.sections.find((section) => section.id === "presentation.play.details.crop");
			expect(cropSection).toBeDefined();
			const widthField = cropSection!.items.find((item) => item.id === "presentation.play.tile.crop.width");
			expect(widthField?.control?.onChange?.command).toBe("patchTileCrops");
			expect(widthField?.control?.onChange?.args).toMatchObject({ ids: tileIds, field: "width" });
		});

		it("patchTileCrops updates every selected tile", () => {
			const bus = new CommandBus();
			const ctrl = new PresentationPlayController(bus, () => undefined);
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "seedGrid", { rows: 1, columns: 2 });
			const tileIds = ctrl.tiles.map((tile) => tile.id);
			bus.dispatch(PRESENTATION_PLAY_CONTROLLER_ID, "patchTileCrops", { ids: tileIds, field: "x", value: 0.25 });
			for (const tile of ctrl.tiles) {
				expect(tile.crop.x).toBe(0.25);
			}
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
		const { bootPresentationPlay } = await import("@semio-tech/framework-playground-renderer-react/presentation");
		bootPresentationPlay(new PresentationPlay());
	})();
}
//#endregion 🔖Boot
