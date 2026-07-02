// #region 🧲Header
/** @emoji 📽 Presentation tile play app — one-to-many morph tile editor. */
// #endregion 🧲Header

import {
	CommandBus,
	Controller,
	PLAYGROUND_NO_EXAMPLE_ID,
	type PlaygroundExampleCatalog,
	type PlaygroundExampleHost,
	isPlaygroundNoExampleId,
	Platform,
	AppRuntime,
	ModeRuntime,
	WindowKindRuntime,
	buildPanelWindowBody,
	createPlayAppRuntime,
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
	type WindowMeasure,
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
import { registerOsMediaExportHandler } from "@semio-tech/framework-os-core";
import { rasterizeSvgMarkupToPngDataUrl } from "@semio-tech/kernel-2d-js";
import { Store } from "@semio-tech/framework-core";
import { DocumentVcsStore, createDocumentVcsEnvelope, recordProjectionChange } from "@semio-tech/vcs-core/internal";
import {
	applyPresentationEditOp,
	backwardsPresentationEditOp,
	buildTileMorphPrompt,
	diffPresentationEditOp,
	parseGridEngagement,
	populateTileDraftsFromGrid,
	type FigureTileDraft,
	type FigureTileSource,
	type PresentationDeckV1,
	type PresentationEditOp,
} from "./internal.ts";

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
export class PresentationPlayController extends Controller implements PlaygroundExampleHost {
	readonly mainMode = new ModeRuntime("main", "Edit", undefined);
	private activeExampleId = PLAYGROUND_NO_EXAMPLE_ID;
	private readonly docStore = new DocumentVcsStore<PresentationDeckV1, PresentationEditOp>({
		envelope: createDocumentVcsEnvelope("presentation.deck/v1", "presentation-tile-play", {
			schema: "presentation.deck/v1",
			source: PRESENTATION_PLAY_DEFAULT_SOURCE,
			tiles: [],
		}),
		applyOp: applyPresentationEditOp,
		backwardsOp: backwardsPresentationEditOp,
		diffOp: diffPresentationEditOp,
	});
	private readonly snapshotStore: PresentationPlaySnapshotStore;
	private snapshotCache: PresentationPlaySnapshot | null = null;
	clipboardPrompt: string | null = null;
	clipboardEpoch = 0;
	private engagementInput = "";

	get selectedIds(): readonly string[] {
		return this.pointerFocus.getSnapshot().selection;
	}

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(PRESENTATION_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.snapshotStore = new PresentationPlaySnapshotStore(this);
		this.provideStore(PRESENTATION_PLAY_STORE_ID, this.snapshotStore);
		this.rebuildShellMode();
	}

	private projection(): PresentationDeckV1 {
		return this.docStore.projection();
	}

	private applyDeckEdit(op: PresentationEditOp): void {
		recordProjectionChange(this.docStore, [op]);
	}

	private commitDeck(next: PresentationDeckV1): void {
		this.applyDeckEdit({ op: "setDocument", document: next });
	}

	get source(): FigureTileSource {
		return this.projection().source;
	}

	get tiles(): readonly FigureTileDraft[] {
		return this.projection().tiles;
	}

	getDocumentVcsStore(): DocumentVcsStore<PresentationDeckV1, PresentationEditOp> {
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

	private windowMeasures(): readonly WindowMeasure[] {
		const deck = this.projection();
		return [
			{
				kind: "slider",
				id: "presentation-tile-count",
				label: "Tiles",
				value: deck.tiles.length,
				min: 0,
				max: Math.max(deck.tiles.length, 1),
				step: 1,
				onChange: presentationPlayCmd("addTile"),
			},
			{
				kind: "slider",
				id: "presentation-selected-count",
				label: "Selected",
				value: this.selectedIds.length,
				min: 0,
				max: Math.max(this.selectedIds.length, 1),
				step: 1,
				onChange: presentationPlayCmd("deleteSelection"),
			},
		];
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
				this.windowMeasures(),
				this.windowEngagement(),
			),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Presentation play window "${windowKind.id}"`);
		}
	}

	private seedGrid(rows: number, columns: number): void {
		const tiles = populateTileDraftsFromGrid({ source: this.source, rows, columns });
		this.applyDeckEdit({ op: "setTiles", tiles });
		this.pointerFocus.setSelection(tiles.length > 0 ? [tiles[0]!.id] : []);
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

	getExampleCatalog(): PlaygroundExampleCatalog {
		return { activeExampleId: this.activeExampleId, options: [] };
	}

	override run(command: string, args?: unknown): void {
		switch (command) {
			case "setActiveExample": {
				const fixtureId = (args as { fixtureId?: string }).fixtureId ?? "";
				const nextId = isPlaygroundNoExampleId(fixtureId) ? PLAYGROUND_NO_EXAMPLE_ID : fixtureId;
				if (nextId === this.activeExampleId) break;
				this.activeExampleId = nextId;
				if (isPlaygroundNoExampleId(nextId)) {
					this.applyDeckEdit({
						op: "setDocument",
						document: {
							schema: "presentation.deck/v1",
							source: { ...PRESENTATION_PLAY_DEFAULT_SOURCE },
							tiles: [],
						},
					});
					this.pointerFocus.setSelection([]);
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
					this.applyDeckEdit({
						op: "setDocument",
						document: {
							schema: "presentation.deck/v1",
							source: { ...PRESENTATION_PLAY_DEFAULT_SOURCE },
							tiles: [],
						},
					});
					this.pointerFocus.setSelection([]);
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
				this.applyDeckEdit({ op: "replaceSource", source, resetTiles: replaced });
				if (replaced) this.pointerFocus.setSelection([]);
				break;
			}
			case "setFrame": {
				const { frame } = args as { frame?: FigureTileSource["frame"] };
				if (frame) {
					this.applyDeckEdit({ op: "setSourceFrame", frame });
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
				this.applyDeckEdit({ op: "addTile", tile: { id, name: id, crop: nextCrop } });
				this.pointerFocus.setSelection([id]);
				break;
			}
			case "deleteTile": {
				const { id } = (args ?? {}) as { id?: string };
				const targetIds = typeof id === "string" ? [id] : this.selectedIds;
				if (targetIds.length === 0) {
					break;
				}
				const remove = new Set(targetIds);
				this.applyDeckEdit({ op: "removeTiles", tileIds: [...remove] });
				this.pointerFocus.setSelection(this.selectedIds.filter((selected) => !remove.has(selected)));
				break;
			}
			case "deleteSelection": {
				if (this.selectedIds.length === 0) {
					break;
				}
				this.applyDeckEdit({ op: "removeTiles", tileIds: [...this.selectedIds] });
				this.pointerFocus.setSelection([]);
				break;
			}
			case "renameTiles": {
				const { ids, name, value } = args as { ids?: readonly string[]; name?: string; value?: string };
				const nextName = (value ?? name)?.trim();
				const targetIds = (ids ?? []).filter((id) => this.tiles.some((tile) => tile.id === id));
				if (!nextName || targetIds.length === 0) {
					break;
				}
				this.applyDeckEdit({ op: "renameTiles", tileIds: targetIds, name: nextName });
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
				const targetIds = (ids ?? []).filter((id) => this.tiles.some((tile) => tile.id === id));
				if (targetIds.length === 0) {
					break;
				}
				this.applyDeckEdit({ op: "patchTileCrops", tileIds: targetIds, field, value });
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
					this.applyDeckEdit({ op: "patchTileCrop", tileId: id, crop });
				}
				break;
			}
			case "setSelectedIds": {
				const { ids } = args as { ids?: readonly string[] };
				const validIds = new Set(this.tiles.map((tile) => tile.id));
				this.pointerFocus.setSelection((ids ?? []).filter((id) => validIds.has(id)));
				break;
			}
			case "clearTiles": {
				this.applyDeckEdit({ op: "clearTiles" });
				this.pointerFocus.setSelection([]);
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

export function buildPresentationPlayAppRuntime(controller: PresentationPlayController): AppRuntime {
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

export * from "./internal.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for presentation deck tiles. */
export function buildPresentationDeckProgramDefinition(): PlatformDefinition {
	return {
		id: "presentation.deck",
		name: "Presentation Deck",
		apiVersion: "1",
		apps: [{ id: "presentation.deck", label: "Presentation", controllerId: PRESENTATION_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}

/** @emoji 🧩 S program definition for presentation. */
export function buildPresentationProgramDefinition(): PlatformDefinition {
	return {
		id: "presentation",
		name: "Presentation",
		apiVersion: "1",
		apps: [{ id: "presentation", label: "Presentation", controllerId: PRESENTATION_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🔖Play
import {
	createPlaygroundApp,
	createProductPlaygroundPlatform,
} from "@semio-tech/framework-playground-core";

export const presentationPlayAppDefinition = createPlaygroundApp({
	id: PRESENTATION_PLAY_APP_ID,
	label: "Presentation",
	controllerId: PRESENTATION_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	keybindings: [
		{ key: "Delete", controllerId: PRESENTATION_PLAY_CONTROLLER_ID, command: "deleteSelection" },
		{ key: "Backspace", controllerId: PRESENTATION_PLAY_CONTROLLER_ID, command: "deleteSelection" },
	],
	devHost: {
		playEntryKind: "presentation",
		resolveDedupe: ["react", "react-dom", "./internal.ts"],
		optimizeDeps: { include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "./internal.ts"] },
	},
	createRuntime: () => {
		const runtime = createProductPlaygroundPlatform(PRESENTATION_PLAY_APP_ID);
		const ctrl = new PresentationPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildPresentationPlayAppRuntime(ctrl));
		return runtime;
	},
	registerBodies: () => registerPresentationPlayDeclarativeBodies(),
	bootRenderer: async (pg) => {
		const { bootPresentationPlay } = await import("@semio-tech/framework-playground-renderer-react/presentation");
		bootPresentationPlay(pg);
	},
});
//#endregion 🔖Play

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	function thoughtScope(thought: Thought): ResolutionScope {
		return buildResolutionScope([thought]);
	}

	const sampleIntro = intro({
		title: {
			full: ["Line A", "Line B", "Line C"],
			short: "Short title",
		},
		description: { full: ["D1", "D2"], short: "D short" },
		goal: ["G1"],
		authors: {
			lines: [[{ name: "Alice" }, { name: "Bob" }], [{ name: "Carol", marks: ["1", "b"] }]],
		},
		affiliations: {
			steps: [
				[{ mark: "a", name: "Faculty" }],
				[
					{ mark: "a", name: "Faculty" },
					{ mark: "1", name: "University" },
					{ mark: "2", name: "Other University" },
				],
				[
					{ mark: "a", name: "Faculty" },
					{
						mark: "1",
						name: "University",
						shortName: "LUH",
						suffix: { mark: "x", name: "Chair X" },
					},
					{
						mark: "2",
						name: "Other University",
						shortName: "UdK",
						suffix: { mark: "y", name: "Chair Y" },
					},
				],
			],
		},
	});

	describe("loadPresentationFromSlideGlob", () => {
		it("assembles chapters, sequences, thoughts, and ordered slides from slide paths", () => {
			const deck = loadPresentationFromSlideGlob(
				{ id: "deck", name: "Deck", language: "de" },
				{
					"./slide/Hauptteil/Einführung/Einleitung/Titel.ts": {
						default: {
							order: 0,
							participants: [{ id: "title" }],
							embodiments: [{ kind: "text", id: "title--main", lines: ["A"], level: "title" }],
							arrangement: {
								id: "title",
								name: "Titel",
								dispositions: [{ participantId: "title", embodimentId: "title--main", emphasis: "active" }],
							},
						},
					},
					"./slide/Hauptteil/Einführung/Einleitung/Ziel.ts": {
						default: {
							order: 1,
							arrangement: {
								id: "goal",
								name: "Ziel",
								dispositions: [{ participantId: "title", embodimentId: "title--main", emphasis: "active" }],
							},
						},
					},
					"./slide/Hauptteil/Einführung/Medien/Bauteilkatalog.ts": {
						default: {
							order: 0,
							arrangement: {
								id: "catalogue",
								name: "Bauteilkatalog",
								dispositions: [{ participantId: "catalogue", embodimentId: "catalogue--figure", emphasis: "active" }],
							},
						},
					},
				},
			);
			expect(deck.chapters[0]).toMatchObject({
				name: "Hauptteil",
				sequences: [
					{
						name: "Einführung",
						thoughts: [
							{
								name: "Einleitung",
								slides: [{ arrangement: { name: "Titel" } }, { arrangement: { name: "Ziel" } }],
							},
							{
								name: "Medien",
								slides: [{ arrangement: { name: "Bauteilkatalog" } }],
							},
						],
					},
				],
			});
		});
	});

	describe("parsePresentationSlideFilePath", () => {
		it("round-trips canonical slide module paths", () => {
			const path = presentationSlideFilePath("Hauptteil", "Einführung", "Einleitung", "Titel");
			expect(path).toBe("slide/Hauptteil/Einführung/Einleitung/Titel.ts");
			expect(parsePresentationSlideFilePath(`./${path}`)).toEqual({
				chapter: "Hauptteil",
				sequence: "Einführung",
				thought: "Einleitung",
				slide: "Titel",
			});
		});
	});

	describe("parsePresentationThoughtFilePath", () => {
		it("round-trips canonical thought template paths", () => {
			const path = presentationThoughtFilePath("Hauptteil", "Einführung", "Einleitung");
			expect(path).toBe("slide/Hauptteil/Einführung/Einleitung.ts");
			expect(parsePresentationThoughtFilePath(`./${path}`)).toEqual({
				chapter: "Hauptteil",
				sequence: "Einführung",
				thought: "Einleitung",
			});
			expect(parsePresentationSlideFilePath(`./${path}`)).toBeNull();
		});
	});

	describe("loadPresentationFromSlideGlob thought templates", () => {
		it("expands intro thought files into ordered slides", () => {
			const deck = loadPresentationFromSlideGlob(
				{ id: "deck", name: "Deck", language: "de" },
				{
					"./slide/Hauptteil/Einführung/Einleitung.ts": {
						default: introThoughtFile({
							language: "de",
							title: { full: ["T"], short: "T" },
							description: { full: ["D"], short: "D" },
							goal: ["G"],
							authors: { lines: [[{ name: "A", marks: ["a"] }]] },
							affiliations: { steps: [[{ mark: "a", name: "Faculty" }], [{ mark: "a", name: "Faculty" }], [{ mark: "a", name: "Faculty" }]] },
						}),
					},
				},
			);
			const thought = deck.chapters[0]!.sequences[0]!.thoughts[0]!;
			expect(thought.name).toBe("Einleitung");
			expect(thought.slides.map((slide) => slide.arrangement.name)).toEqual([
				"Titel",
				"Beschreibung",
				"Ziel",
				"Autoren",
				"Fakultät",
				"Universitäten",
				"Lehrstühle",
			]);
		});
	});

	describe("intro", () => {
		it("recognizes intro arrangement ids regardless of bookmark language", () => {
			expect(isIntroArrangementId("affiliations-3")).toBe(true);
			expect(isIntroArrangementId("Lehrstühle")).toBe(false);
		});

		it("builds seven slides in one thought", () => {
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			expect(thought.slides.map((slide) => slide.arrangement.id)).toEqual([
				"title",
				"description",
				"goal",
				"authors",
				"affiliations-1",
				"affiliations-2",
				"affiliations-3",
			]);
			expect(expandThoughtSlides(thought).map((slide) => slide.id)).toEqual([
				"title",
				"description",
				"goal",
				"authors",
				"affiliations-1",
				"affiliations-2",
				"affiliations-3",
			]);
		});

		it("keeps a single universities bookmark at v=5", () => {
			const uniSlides = collectPresentationSlides(sampleIntro).filter((slide) => slide.slide === "Universities");
			expect(uniSlides).toHaveLength(1);
			expect(uniSlides[0]?.v).toBe(5);
		});

		it("uses fixed-size heading blocks without fit-text", () => {
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			const textEmbodiments = (thought.embodiments ?? []).filter((e): e is TextEmbodiment => e.kind === "text");
			expect(textEmbodiments.every((e) => e.fit !== true)).toBe(true);
			expect(textEmbodiments.every((e) => resolveTextMorphRoot(e) === "heading-block")).toBe(true);
		});

		it("uses affiliation short names when chairs are introduced", () => {
			expect(
				affiliationLineName({
					mark: "1",
					name: "Leibniz Universität Hannover",
					shortName: "LUH",
				}),
			).toBe("LUH");
		});

		it("includes muted authors on goal so reveal can morph into the authors slide", () => {
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			const goal = thought.slides.find((slide) => slide.arrangement.id === "goal")!.arrangement;
			const goalAuthors = resolveArrangement(thoughtScope(thought), goal).find(
				(resolved) => resolved.participant.id === INTRO_PARTICIPANT_AUTHORS,
			);
			expect(goalAuthors?.emphasis).toBe("muted");
			expect(goalAuthors?.embodiment.id).toBe(INTRO_EMBODIMENT_AUTHORS_PLAIN);
		});

		it("uses plain short description embodiment on goal and later intro slides", () => {
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			const goal = thought.slides.find((slide) => slide.arrangement.id === "goal")!.arrangement;
			const authors = thought.slides.find((slide) => slide.arrangement.id === "authors")!.arrangement;
			const goalDescription = resolveArrangement(thoughtScope(thought), goal).find(
				(resolved) => resolved.participant.id === INTRO_PARTICIPANT_DESCRIPTION,
			)!;
			const authorsDescription = resolveArrangement(thoughtScope(thought), authors).find(
				(resolved) => resolved.participant.id === INTRO_PARTICIPANT_DESCRIPTION,
			)!;
			if (goalDescription.embodiment.kind === "text" && authorsDescription.embodiment.kind === "text") {
				expect(goalDescription.embodiment.id).toBe(INTRO_EMBODIMENT_DESCRIPTION_SHORT);
				expect(goalDescription.embodiment.morphFromLines).toBeUndefined();
				expect(authorsDescription.embodiment.id).toBe(INTRO_EMBODIMENT_DESCRIPTION_SHORT);
			}
		});

		it("records prior affiliation labels for embodiment morph on chairs slide", () => {
			const previous = [
				{ mark: "a", name: "Faculty" },
				{ mark: "1", name: "Leibniz Universität Hannover" },
			] as const;
			const current = [
				{ mark: "a", name: "Faculty" },
				{
					mark: "1",
					name: "Leibniz Universität Hannover",
					shortName: "LUH",
					suffix: { mark: "x", name: "Chair X" },
				},
			] as const;
			expect(affiliationEmbodimentMorphLabels(previous, current)).toEqual({
				"1": "Leibniz Universität Hannover",
			});
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			const affiliations3 = thought.slides.find((slide) => slide.arrangement.id === "affiliations-3")!.arrangement;
			const step3 = resolveArrangement(thoughtScope(thought), affiliations3).find(
				(resolved) => resolved.participant.id === INTRO_PARTICIPANT_INSTITUTIONS,
			)!;
			if (step3.embodiment.kind === "affiliations") {
				expect(step3.embodiment.morphLineLabels).toEqual({
					"1": "University",
					"2": "Other University",
				});
			}
		});

		it("abbreviates author first names on affiliation slides", () => {
			expect(abbreviateAuthorFirstName("Ueli Saluz")).toBe("U. Saluz");
			expect(abbreviateAuthorFirstName("Christoph Gengnagel")).toBe("C. Gengnagel");
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			const affiliations1 = thought.slides.find((slide) => slide.arrangement.id === "affiliations-1")!.arrangement;
			const authors = resolveArrangement(thoughtScope(thought), affiliations1).find(
				(resolved) => resolved.participant.id === INTRO_PARTICIPANT_AUTHORS,
			)!;
			if (authors.embodiment.kind === "authors") {
				expect(authors.embodiment.abbreviateFirstName).toBe(true);
			}
		});

		it("introduces author marks with each affiliation step", () => {
			const lines = [[{ name: "Alice", marks: ["a", "1", "x"] }]] as const;
			const rawSteps = [
				[{ mark: "a", name: "Faculty" }],
				[
					{ mark: "a", name: "Faculty" },
					{ mark: "1", name: "University" },
				],
				[
					{ mark: "a", name: "Faculty" },
					{ mark: "1", name: "University", suffix: { mark: "x", name: "Chair X" } },
				],
			] as const;
			const aff1 = authorLinesForAffiliationStep(lines, rawSteps[0], [])[0]![0]!;
			expect(aff1.markEntries?.map((m) => m.mark)).toEqual(["a"]);
			const aff2 = authorLinesForAffiliationStep(lines, rawSteps[1], rawSteps[0])[0]![0]!;
			expect(aff2.markEntries?.map((m) => m.mark)).toEqual(["a", "1"]);
			expect(aff2.markEntries?.find((m) => m.mark === "a")?.emphasis).toBe("muted");
			expect(aff2.markEntries?.find((m) => m.mark === "1")?.emphasis).toBe("active");
			const aff3 = authorLinesForAffiliationStep(lines, rawSteps[2], rawSteps[1])[0]![0]!;
			expect(aff3.markEntries?.map((m) => m.mark)).toEqual(["a", "1", "x"]);
			expect(aff3.markEntries?.find((m) => m.mark === "x")?.emphasis).toBe("active");
		});

		it("highlights only new affiliation marks per slide", () => {
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			const affiliations2 = thought.slides.find((slide) => slide.arrangement.id === "affiliations-2")!.arrangement;
			const step2 = resolveArrangement(thoughtScope(thought), affiliations2).find(
				(resolved) => resolved.participant.id === INTRO_PARTICIPANT_INSTITUTIONS,
			)!;
			if (step2.embodiment.kind === "affiliations") {
				expect(step2.embodiment.entries.find((e) => e.mark === "1")?.lineEmphasis).toBe("active");
				expect(step2.embodiment.entries.find((e) => e.mark === "a")?.lineEmphasis).toBe("muted");
			}
			const affiliations3 = thought.slides.find((slide) => slide.arrangement.id === "affiliations-3")!.arrangement;
			const step3 = resolveArrangement(thoughtScope(thought), affiliations3).find(
				(resolved) => resolved.participant.id === INTRO_PARTICIPANT_INSTITUTIONS,
			)!;
			if (step3.embodiment.kind === "affiliations") {
				const uni = step3.embodiment.entries.find((e) => e.mark === "1");
				expect(uni?.lineEmphasis).toBe("muted");
				expect(uni?.suffixEmphasis).toBe("active");
				expect(step3.embodiment.entries.find((e) => e.mark === "a")?.lineEmphasis).toBe("muted");
			}
		});
		it("assigns German bookmark names when language is de", () => {
			const deck = intro({
				language: "de",
				title: { full: ["A"], short: "Short" },
				description: { full: ["D"], short: "D short" },
				goal: ["G"],
				authors: { lines: [[{ name: "Alice" }]] },
				affiliations: {
					steps: [
						[{ mark: "a", name: "Faculty" }],
						[{ mark: "a", name: "Faculty" }, { mark: "1", name: "Uni" }],
						[{ mark: "a", name: "Faculty" }, { mark: "1", name: "Uni" }],
					],
				},
			});
			const chapter = deck.chapters[0]!;
			const sequence = chapter.sequences[0]!;
			const thought = sequence.thoughts[0]!;
			expect(chapter.name).toBe("Hauptteil");
			expect(sequence.name).toBe("Einführung");
			expect(thought.name).toBe("Einleitung");
			expect(thought.slides.map((slide) => slide.arrangement.name)).toEqual([
				"Titel",
				"Beschreibung",
				"Ziel",
				"Autoren",
				"Fakultät",
				"Universitäten",
				"Lehrstühle",
			]);
			const goalSlide = collectPresentationSlides(deck).find((slide) => slide.slide === "Ziel");
			expect(goalSlide).toMatchObject({
				h: 0,
				chapter: "Hauptteil",
				sequence: "Einführung",
				thought: "Einleitung",
				slide: "Ziel",
			});
			expect(goalSlide?.v).toBeGreaterThan(0);
		});
	});

	describe("resolveEmbodiment", () => {
		it("throws when embodiment id is missing from scope", () => {
			const scope = buildResolutionScope([
				{
					participants: [{ id: "x" }],
					embodiments: [{ kind: "text", id: "x--body", lines: ["a"], level: "body" }],
				},
			]);
			expect(() => resolveEmbodiment(scope, "missing")).toThrow(/Unknown embodiment/);
		});
	});

	describe("buildResolutionScope", () => {
		it("lets inner scopes override embodiment ids", () => {
			const scope = buildResolutionScope([
				{ embodiments: [{ kind: "text", id: "a", lines: ["outer"], level: "body" }] },
				{ embodiments: [{ kind: "text", id: "a", lines: ["inner"], level: "body" }] },
			]);
			expect((resolveEmbodiment(scope, "a") as TextEmbodiment).lines[0]).toBe("inner");
		});
	});

	describe("morphId", () => {
		it("uses participant id as reveal data-id", () => {
			expect(morphId("title")).toBe("title");
		});
	});

	describe("split", () => {
		it("produces one participant, embodiment, and disposition per grid cell", () => {
			const artifacts = split({
				source: "/catalogue.png",
				rows: 2,
				columns: 2,
				frame: { x: 0, y: 0, width: 1, height: 1 },
			});
			expect(artifacts.participants).toHaveLength(4);
			expect(artifacts.embodiments).toHaveLength(4);
			expect(artifacts.dispositions).toHaveLength(4);
			expect(artifacts.dispositions[0]?.participantId).toBe("tile-r0-c0");
			expect(artifacts.dispositions[0]?.embodimentId).toBe("tile-r0-c0-figure");
		});
	});

	describe("expandThoughtSlides", () => {
		it("assigns one auto-animate id per morph run", () => {
			const thought: Thought = {
				id: "morph",
				participants: [{ id: "label" }],
				embodiments: [
					{ kind: "text", id: "source", lines: ["Reuse"], level: "heading" },
					{ kind: "text", id: "target", lines: ["Remanufacture"], level: "heading" },
				],
				slides: [
					{
						arrangement: {
							id: "source",
							dispositions: [{ participantId: "label", embodimentId: "source", emphasis: "active" }],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "mapping",
							dispositions: [{ participantId: "label", embodimentId: "target", emphasis: "active" }],
						},
					},
				],
			};
			const expanded = expandThoughtSlides(thought);
			expect(expanded.map((slide) => slide.id)).toEqual(["source", "mapping"]);
			expect(expanded.every((slide) => slide.autoAnimateId === "morph--m0")).toBe(true);
		});

		it("keeps morphFrom on label slides without expanding arrangements", () => {
			const thought: Thought = {
				id: "merge",
				participants: [{ id: "col1" }, { id: "labels" }],
				embodiments: [
					{ kind: "figure", id: "crop", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
					{ kind: "text", id: "label", lines: ["A"], level: "heading" },
					{ kind: "text", id: "stack", lines: ["A"], level: "heading", morphRoot: "heading-block" },
				],
				slides: [
					{
						arrangement: {
							id: "focus",
							dispositions: [
								{
									participantId: "col1",
									embodimentId: "crop",
									emphasis: "active",
									position: { x: 0.1, y: 0.2, width: 0.3, height: 0.6 },
								},
							],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "labels",
							dispositions: [
								{
									participantId: "labels",
									embodimentId: "stack",
									emphasis: "active",
									position: { x: 0.38, y: 0.12, width: 0.24, height: 0.24 },
									morphFrom: [
										{
											participantId: "col1",
											embodimentId: "crop",
											position: { x: 0.38, y: 0.12, width: 0.24, height: 0.24 },
											targetLineIndex: 0,
										},
									],
								},
							],
						},
					},
				],
			};
			const labels = thought.slides.find((slide) => slide.arrangement.id === "labels")!.arrangement;
			const morphFrom = labels.dispositions[0]?.morphFrom?.[0];
			expect(morphFrom?.targetLineIndex).toBe(0);
			expect(morphFrom?.position).toEqual({ x: 0.38, y: 0.12, width: 0.24, height: 0.24 });
			expect(arrangementRestDispositions(labels)).toHaveLength(1);
		});

		it("resolves morph-into targets with --label morph ids", () => {
			const scope = buildResolutionScope([
				{
					participants: [{ id: "catalogue-col1" }],
					embodiments: [
						{ kind: "text", id: "catalogue-col1--label", lines: ["Rippenplatte"], level: "heading" },
					],
				},
			]);
			const resolved = resolveArrangement(scope, {
				id: "labels",
				dispositions: [
					{
						participantId: "catalogue-col1",
						embodimentId: "catalogue-col1--label",
						emphasis: "active",
						position: { x: 0.1, y: 0.4, width: 0.2, height: 0.1 },
						morphFrom: [
							{
								participantId: "Rippenplatte 1",
								embodimentId: "Rippenplatte 1-figure",
								position: { x: 0.1, y: 0.4, width: 0.2, height: 0.1 },
							},
						],
					},
				],
			});
			expect(resolved[0]?.morphId).toBe("catalogue-col1--label");
		});

		it("preserves declarative settleBeforeMorphTo on arrangements", () => {
			const thought: Thought = {
				id: "media",
				participants: [{ id: "col1" }],
				embodiments: [
					{ kind: "figure", id: "crop", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
					{ kind: "text", id: "label", lines: ["A"], level: "heading" },
				],
				slides: [
					{
						arrangement: {
							id: "focus",
							settleBeforeMorphTo: ["labels"],
							dispositions: [
								{
									participantId: "col1",
									embodimentId: "crop",
									emphasis: "active",
									position: { x: 0.1, y: 0.2, width: 0.3, height: 0.6 },
									style: { opacity: 0 },
								},
							],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "labels",
							dispositions: [
								{
									participantId: "col1",
									embodimentId: "label",
									emphasis: "active",
									position: { x: 0.38, y: 0.12, width: 0.24, height: 0.24 },
								},
							],
						},
					},
				],
			};
			const focus = expandThoughtSlides(thought).find((slide) => slide.id === "focus");
			expect(focus?.arrangement.settleBeforeMorphTo).toEqual(["labels"]);
		});

		it("keeps consecutive morph slides without extra render slides", () => {
			const thought: Thought = {
				id: "move",
				participants: [{ id: "box" }],
				embodiments: [{ kind: "text", id: "box--main", lines: ["A"], level: "body" }],
				slides: [
					{
						arrangement: {
							id: "left",
							dispositions: [
								{
									participantId: "box",
									embodimentId: "box--main",
									emphasis: "active",
									position: { x: 0.1, y: 0.2, width: 0.3, height: 0.2 },
								},
							],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "right",
							dispositions: [
								{
									participantId: "box",
									embodimentId: "box--main",
									emphasis: "active",
									position: { x: 0.6, y: 0.2, width: 0.3, height: 0.2 },
								},
							],
						},
					},
				],
			};
			expect(expandThoughtSlides(thought).map((slide) => slide.id)).toEqual(["left", "right"]);
		});

		it("keeps morphTo on the source slide without expanding arrangements", () => {
			const thought: Thought = {
				id: "split",
				participants: [{ id: "whole" }, { id: "tile-a" }],
				embodiments: [
					{ kind: "figure", id: "whole--figure", src: "/a.png" },
					{ kind: "figure", id: "tile-a--figure", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
				],
				slides: [
					{
						arrangement: {
							id: "whole",
							dispositions: [
								{
									participantId: "whole",
									embodimentId: "whole--figure",
									emphasis: "active",
									morphTo: [
										{
											participantId: "tile-a",
											position: { x: 0.1, y: 0.1, width: 0.35, height: 0.8 },
										},
									],
								},
							],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "tiles",
							dispositions: [
								{
									participantId: "tile-a",
									embodimentId: "tile-a--figure",
									emphasis: "active",
								},
							],
						},
					},
				],
			};
			const whole = thought.slides[0]!.arrangement;
			expect(whole.dispositions[0]?.morphTo).toHaveLength(1);
			expect(arrangementRestDispositions(whole)).toHaveLength(1);
		});

		it("starts a new morph run after a fade transition", () => {
			const thought: Thought = {
				id: "fade",
				participants: [{ id: "box" }],
				embodiments: [{ kind: "text", id: "box--main", lines: ["A"], level: "body" }],
				slides: [
					{
						arrangement: {
							id: "a",
							dispositions: [{ participantId: "box", embodimentId: "box--main", emphasis: "active" }],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "b",
							dispositions: [{ participantId: "box", embodimentId: "box--main", emphasis: "active" }],
						},
						transition: { kind: "fade" },
					},
					{
						arrangement: {
							id: "c",
							dispositions: [{ participantId: "box", embodimentId: "box--main", emphasis: "active" }],
						},
					},
				],
			};
			const expanded = expandThoughtSlides(thought);
			expect(expanded[0]?.autoAnimateId).toBe("fade--m0");
			expect(expanded[1]?.autoAnimateId).toBe("fade--m0");
			expect(expanded[2]?.autoAnimateId).toBeUndefined();
		});

		it("starts a new morph run when consecutive slides share no participants", () => {
			const thought: Thought = {
				id: "media",
				participants: [{ id: "catalogue" }, { id: "col1" }],
				embodiments: [
					{ kind: "figure", id: "catalogue--figure", src: "/a.png" },
					{ kind: "figure", id: "crop", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
					{ kind: "text", id: "label", lines: ["A"], level: "heading" },
				],
				slides: [
					{
						arrangement: {
							id: "catalogue",
							dispositions: [
								{ participantId: "catalogue", embodimentId: "catalogue--figure", emphasis: "active" },
							],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "focus",
							dispositions: [
								{
									participantId: "col1",
									embodimentId: "crop",
									emphasis: "active",
									position: { x: 0.1, y: 0.2, width: 0.3, height: 0.6 },
								},
							],
						},
						transition: { kind: "morph" },
					},
					{
						arrangement: {
							id: "labels",
							dispositions: [
								{
									participantId: "col1",
									embodimentId: "label",
									emphasis: "active",
									position: { x: 0.38, y: 0.12, width: 0.24, height: 0.24 },
								},
							],
						},
					},
				],
			};
			const expanded = expandThoughtSlides(thought);
			expect(expanded.map((slide) => slide.id)).toEqual(["catalogue", "focus", "labels"]);
			expect(expanded[0]?.autoAnimateId).toBeUndefined();
			expect(expanded[1]?.autoAnimateId).toBe("media--m1");
			expect(expanded[2]?.autoAnimateId).toBe("media--m1");
		});
	});

	describe("centerResolvedArrangement", () => {
		it("offsets placements so their bounding box is centered in the unit slide", () => {
			const resolved: ResolvedDisposition[] = [
				{
					participant: { id: "a" },
					embodiment: { kind: "figure", id: "a--figure", src: "/a.png" },
					embodimentId: "a--figure",
					emphasis: "active",
					morphId: "a",
					position: { x: 0.2, y: 0.1, width: 0.3, height: 0.5 },
				},
			];
			const centered = centerResolvedArrangement(resolved);
			expect(centered[0]?.position?.x).toBeCloseTo(0.35);
			expect(centered[0]?.position?.y).toBeCloseTo(0.25);
		});
	});

	describe("figureFrameForSourceAspect", () => {
		const slideAspect = PRESENTATION_DEFAULT_SLIDE_ASPECT;

		it("matches the source physical aspect inside default padding", () => {
			const sourceAspect = 1536 / 1024;
			const frame = figureFrameForSourceAspect(sourceAspect, slideAspect);
			expect((frame.width / frame.height) * slideAspect).toBeCloseTo(sourceAspect, 10);
			expect(frame.x).toBeCloseTo((1 - frame.width) / 2, 10);
			expect(frame.y).toBeCloseTo((1 - frame.height) / 2, 10);
		});

		it("prefers full width for wider-than-slide sources", () => {
			const frame = figureFrameForSourceAspect(2, slideAspect);
			expect(frame.width).toBeCloseTo(0.92, 10);
		});
	});

	describe("resolveMediaScrollOrigin", () => {
		it("defaults to center when scroll origin is omitted", () => {
			expect(resolveMediaScrollOrigin(undefined)).toEqual({ x: 50, y: 50 });
		});

		it("resolves partial scroll origins and axis percents", () => {
			expect(resolveMediaScrollOrigin({ x: 0 })).toEqual({ x: 0, y: 50 });
			expect(mediaScrollPercentForAxis("x", MEDIA_SCROLL_ORIGIN_TOP_LEFT)).toBe(0);
			expect(mediaScrollPercentForAxis("y", MEDIA_SCROLL_ORIGIN_TOP_LEFT)).toBe(0);
			expect(mediaScrollPercentForAxis("y", MEDIA_SCROLL_ORIGIN_CENTER)).toBe(50);
		});
	});

	describe("splitFigureGrid", () => {
		const frame = { x: 0.1, y: 0.2, width: 0.8, height: 0.6 };

		it("builds rows×columns tiles with frame-relative source crops", () => {
			const tiles = splitFigureGrid({ rows: 3, columns: 5, frame });
			expect(tiles).toHaveLength(15);
			expect(tiles[0]?.key).toBe("tile-r0-c0");
			expect(tiles[0]?.crop.x).toBeCloseTo(0.1, 10);
			expect(tiles[0]?.crop.y).toBeCloseTo(0.2, 10);
			expect(tiles[0]?.crop.width).toBeCloseTo(0.16, 10);
			expect(tiles[0]?.crop.height).toBeCloseTo(0.2, 10);
			expect(tiles[14]?.key).toBe("tile-r2-c4");
			expect(tiles[14]?.crop.x).toBeCloseTo(0.74, 10);
			expect(tiles[14]?.crop.y).toBeCloseTo(0.6, 10);
		});

		it("reconstructs the frame at gap zero", () => {
			const tiles = splitFigureGrid({ rows: 2, columns: 2, frame });
			expect(tiles[0]?.position).toEqual({ x: 0.1, y: 0.2, width: 0.4, height: 0.3 });
			expect(tiles[3]?.position).toEqual({ x: 0.5, y: 0.5, width: 0.4, height: 0.3 });
		});

		it("inserts gap between tile cells", () => {
			const tiles = splitFigureGrid({ rows: 2, columns: 2, frame, gap: 0.05 });
			expect(tiles[1]?.position.x).toBeCloseTo(0.1 + 0.375 + 0.05, 5);
			expect(tiles[1]?.position.width).toBeCloseTo(0.375, 5);
		});

		it("applies default emphasis to every tile when set", () => {
			const tiles = splitFigureGrid({ rows: 1, columns: 2, frame, emphasis: "muted" });
			expect(tiles.every((tile) => tile.emphasis === "muted")).toBe(true);
		});
	});

	describe("unionSourceCrops", () => {
		it("unions normalized crops from grid cells", () => {
			const cells = splitFigureGrid({ rows: 2, columns: 2, frame: { x: 0, y: 0, width: 1, height: 1 } });
			const union = unionSourceCrops(cells.map((cell) => cell.crop));
			expect(union).toEqual({ x: 0, y: 0, width: 1, height: 1 });
		});
	});

	describe("resolveTextMorphRoot", () => {
		it("maps intro title embodiments like eg-ice-25", () => {
			const full: TextEmbodiment = {
				kind: "text",
				id: "full",
				lines: ["A", "B"],
				level: "heading",
				morphRoot: "heading-block",
			};
			const short: TextEmbodiment = {
				kind: "text",
				id: "short",
				lines: ["Short"],
				level: "subheading",
				morphRoot: "subheading-line",
			};
			expect(resolveTextMorphRoot(full)).toBe("heading-block");
			expect(resolveTextMorphRoot(short)).toBe("subheading-line");
		});
	});

	describe("resolveArrangement morphId", () => {
		it("resolves morphId per disposition", () => {
			const thought = sampleIntro.chapters[0]!.sequences[0]!.thoughts[0]!;
			const goal = thought.slides.find((slide) => slide.arrangement.id === "goal")!.arrangement;
			const resolved = resolveArrangement(thoughtScope(thought), goal);
			expect(resolved.map((resolvedDisposition) => resolvedDisposition.morphId)).toEqual([
				"title",
				"description",
				"goal",
				"authors",
			]);
		});
	});

	describe("presentationSequences", () => {
		it("flattens sequences in chapter order", () => {
			const deck: Presentation = {
				id: "flat",
				name: "Flat",
				chapters: [
					{ id: "c1", sequences: [{ id: "s1", thoughts: [] }, { id: "s2", thoughts: [] }] },
					{ id: "c2", sequences: [{ id: "s3", thoughts: [] }] },
				],
			};
			expect(presentationSequences(deck).map((sequence) => sequence.id)).toEqual(["s1", "s2", "s3"]);
		});
	});

	describe("collectPresentationSlides", () => {
		it("orders slides by sequence h then arrangement v", () => {
			const deck: Presentation = {
				id: "multi",
				name: "Multi",
				chapters: [
					{
						id: "chapter-a",
						sequences: [
							{
								id: "seq-a",
								thoughts: [
									{
										id: "thought-a",
										participants: [],
										slides: [
											{ arrangement: { id: "a1", dispositions: [] } },
											{ arrangement: { id: "a2", dispositions: [] } },
										],
									},
								],
							},
						],
					},
					{
						id: "chapter-b",
						sequences: [
							{
								id: "seq-b",
								thoughts: [
									{
										id: "thought-b",
										participants: [],
										slides: [{ arrangement: { id: "b1", dispositions: [] } }],
									},
								],
							},
						],
					},
				],
			};
			expect(collectPresentationSlides(deck)).toEqual([
				{ h: 0, v: 0, chapter: "chapter-a", sequence: "seq-a", thought: "thought-a", slide: "a1" },
				{ h: 0, v: 1, chapter: "chapter-a", sequence: "seq-a", thought: "thought-a", slide: "a2" },
				{ h: 1, v: 0, chapter: "chapter-b", sequence: "seq-b", thought: "thought-b", slide: "b1" },
			]);
			expect(presentationSlideAt(deck, { h: 1, v: 0 })?.slide).toBe("b1");
		});
	});

	describe("parsePresentationSlideHash", () => {
		it("round-trips reveal.js hash paths", () => {
			expect(parsePresentationSlideHash("#/")).toEqual({ h: 0, v: 0 });
			expect(parsePresentationSlideHash("#/2/3")).toEqual({ h: 2, v: 3 });
			expect(parsePresentationSlideHash("#/0/2?sequence=main&thought=intro&slide=goal")).toEqual({ h: 0, v: 2 });
			expect(formatPresentationSlideHash({ h: 2, v: 3 })).toBe("/2/3");
		});

		it("formats chapter, sequence, thought, and slide bookmark params after the hash path", () => {
			const bookmark = {
				chapter: "Main",
				sequence: "Introduction",
				thought: "Introduction",
				slide: "Title",
			};
			expect(formatPresentationUrlHash({ h: 0, v: 0 }, bookmark)).toBe(
				"#/?chapter=Main&sequence=Introduction&thought=Introduction&slide=Title",
			);
			expect(formatPresentationUrlHash({ h: 0, v: 2 }, { ...bookmark, slide: "Goal" })).toBe(
				"#/0/2?chapter=Main&sequence=Introduction&thought=Introduction&slide=Goal",
			);
		});

		it("uses German bookmark query keys and titleized bookmark names for de presentations", () => {
			const bookmark = {
				chapter: "Hauptteil",
				sequence: "Einführung",
				thought: "Einleitung",
				slide: "Universitäten",
			};
			const hash = formatPresentationUrlHash({ h: 0, v: 5 }, bookmark, "de");
			expect(hash.startsWith("#/0/5?")).toBe(true);
			const params = new URLSearchParams(hash.split("?")[1] ?? "");
			expect(params.get("kapitel")).toBe("Hauptteil");
			expect(params.get("sequenz")).toBe("Einführung");
			expect(params.get("gedanke")).toBe("Einleitung");
			expect(params.get("folie")).toBe("Universitäten");
			expect(presentationSlideBookmarkParamKeys("de")).toEqual({
				chapter: "kapitel",
				sequence: "sequenz",
				thought: "gedanke",
				slide: "folie",
			});
		});
	});

	describe("analogy", () => {
		const sampleAnalogy = analogy({
			source: { label: "Reuse", figure: "/reuse.png" },
			target: { label: "Remanufacture", figure: "/remanufacture.png" },
		});

		it("builds two morph slides", () => {
			const thought = sampleAnalogy.chapters[0]!.sequences[0]!.thoughts[0]!;
			expect(thought.slides.map((slide) => slide.arrangement.id)).toEqual(["source", "mapping"]);
			expect(expandThoughtSlides(thought).map((slide) => slide.id)).toEqual(["source", "mapping"]);
		});

		it("resolves positioned visual dispositions", () => {
			const thought = sampleAnalogy.chapters[0]!.sequences[0]!.thoughts[0]!;
			const mapping = thought.slides.find((slide) => slide.arrangement.id === "mapping")!.arrangement;
			const resolved = resolveArrangement(thoughtScope(thought), mapping);
			const visual = resolved.find((resolvedDisposition) => resolvedDisposition.participant.id === ANALOGY_PARTICIPANT_VISUAL);
			expect(visual?.position).toEqual({ x: 0.1, y: 0.35, width: 0.8, height: 0.5 });
			expect(visual?.embodiment.kind).toBe("figure");
		});
	});

	describe("video and pdf embodiments", () => {
		it("resolves video and pdf kinds", () => {
			const scope = buildResolutionScope([
				{
					participants: [{ id: "clip" }, { id: "doc" }],
					embodiments: [
						{ kind: "video", id: "clip--video", src: "/demo.mp4", muted: true },
						{ kind: "pdf", id: "doc--pdf", src: "/paper.pdf", page: 2 },
					],
				},
			]);
			const arrangement: Arrangement = {
				id: "slide",
				dispositions: [
					{ participantId: "clip", embodimentId: "clip--video", emphasis: "active" },
					{
						participantId: "doc",
						embodimentId: "doc--pdf",
						emphasis: "active",
						position: { x: 0.2, y: 0.2, width: 0.6, height: 0.6 },
					},
				],
			};
			const resolved = resolveArrangement(scope, arrangement);
			expect(resolved[0]?.embodiment.kind).toBe("video");
			expect(resolved[1]?.embodiment.kind).toBe("pdf");
			if (resolved[1]?.embodiment.kind === "pdf") {
				expect(resolved[1].embodiment.page).toBe(2);
				expect(resolved[1].embodiment.pages).toBeUndefined();
			}
			expect(resolved[1]?.position?.width).toBe(0.6);
		});

		it("resolves pdf embodiments with a page subset", () => {
			const scope = buildResolutionScope([
				{
					participants: [{ id: "doc" }],
					embodiments: [
						{
							kind: "pdf",
							id: "doc--pdf",
							src: "/thesis.pdf",
							page: 25,
							pages: [1, 12, 25, 35],
						},
					],
				},
			]);
			const resolved = resolveArrangement(scope, {
				id: "slide",
				dispositions: [{ participantId: "doc", embodimentId: "doc--pdf", emphasis: "active" }],
			});
			expect(resolved[0]?.embodiment.kind).toBe("pdf");
			if (resolved[0]?.embodiment.kind === "pdf") {
				expect(resolved[0].embodiment.pages).toEqual([1, 12, 25, 35]);
			}
		});
	});

	describe("tile play", () => {
		const source: FigureTileSource = {
			src: "/catalogue.png",
			sourceAspect: 1222 / 896,
			frame: { x: 0.127, y: 0.1, width: 0.746, height: 0.75 },
		};

		it("seeds grid drafts with frame-relative crops", () => {
			const drafts = populateTileDraftsFromGrid({ source, rows: 2, columns: 2 });
			expect(drafts).toHaveLength(4);
			expect(drafts[0]?.id).toBe("tile-r0-c0");
			expect(drafts[0]?.crop.x).toBeCloseTo(source.frame.x, 10);
		});

		it("parses grid engagement tokens", () => {
			expect(parseGridEngagement("3x5")).toEqual({ rows: 3, columns: 5 });
			expect(parseGridEngagement("2×4")).toEqual({ rows: 2, columns: 4 });
			expect(parseGridEngagement("add")).toBeNull();
		});

		it("clamps move and resize to the unit square", () => {
			const rect = { x: 0.8, y: 0.8, width: 0.15, height: 0.15 };
			const moved = moveNormalizedRect(rect, 0.2, 0.2);
			expect(moved.x + moved.width).toBeLessThanOrEqual(1.001);
			expect(moved.y + moved.height).toBeLessThanOrEqual(1.001);
			const resized = resizeNormalizedRect({ x: 0.1, y: 0.1, width: 0.4, height: 0.4 }, "se", 0.5, 0.5);
			expect(resized.width).toBeLessThanOrEqual(0.9);
			expect(resized.height).toBeLessThanOrEqual(0.9);
		});

		it("allows overlapping tile crops in the morph prompt", () => {
			const drafts: FigureTileDraft[] = [
				{ id: "a", name: "Tile A", crop: { x: 0.1, y: 0.1, width: 0.5, height: 0.5 } },
				{ id: "b", name: "Tile B", crop: { x: 0.3, y: 0.3, width: 0.5, height: 0.5 } },
			];
			const prompt = buildTileMorphPrompt(source, drafts);
			expect(prompt).toContain("Tile A");
			expect(prompt).toContain("Tile B");
			expect(prompt).toContain("mit-bestand/präsentation/33.projektetage/spec.ts");
			expect(prompt).toContain("morphTo");
		});

		it("detects supported tile media kinds from file metadata", () => {
			expect(figureTileMediaKindFromFile("image/png", "photo.png")).toBe("figure");
			expect(figureTileMediaKindFromFile("image/svg+xml", "icon.svg")).toBe("figure");
			expect(figureTileMediaKindFromFile("video/mp4", "clip.mp4")).toBe("video");
			expect(figureTileMediaKindFromFile("application/pdf", "doc.pdf")).toBe("pdf");
			expect(figureTileMediaKindFromFile("", "notes.pdf")).toBe("pdf");
			expect(isFigureTileMediaFile("text/plain", "readme.txt")).toBe(false);
		});

		it("embeds video and pdf kind in the morph prompt", () => {
			const prompt = buildTileMorphPrompt(
				{ src: "/clip.mp4", kind: "video", sourceAspect: 16 / 9, frame: { x: 0, y: 0, width: 1, height: 1 } },
				[{ id: "t1", name: "Intro", crop: { x: 0, y: 0, width: 0.5, height: 0.5 } }],
			);
			expect(prompt).toContain("kind: video");
			expect(prompt).toContain("video(...)");
			const pdfPrompt = buildTileMorphPrompt(
				{ src: "/paper.pdf", kind: "pdf", pdfPage: 2, sourceAspect: FIGURE_TILE_PDF_PAGE_ASPECT, frame: { x: 0, y: 0, width: 1, height: 1 } },
				[],
			);
			expect(pdfPrompt).toContain("kind: pdf");
			expect(pdfPrompt).toContain("pdfPage: 2");
		});
	});

	describe("createPresentationAppVcsHandler", () => {
		it("materializes deck projection from inline json", () => {
			const handler = createPresentationAppVcsHandler();
			const projection = handler.materializeProjection({
				inline: JSON.stringify({ schema: "presentation.deck", source: { src: "/a.png" }, tiles: [{ id: "t1", name: "A", crop: { x: 0, y: 0, width: 1, height: 1 } }] }),
			}) as { tiles: readonly unknown[] };
			expect(projection.tiles).toHaveLength(1);
		});
	});
}

//#region 🔖MediaExport
async function presentationDeckToPngDataUrl(deck: PresentationDeckV1): Promise<{ dataUrl: string; width: number; height: number }> {
	const frame = deck.source.frame;
	const width = Math.max(1, Math.round(frame.width * 1024));
	const height = Math.max(1, Math.round(frame.height * 1024));
	if (typeof document === "undefined") return { dataUrl: "", width, height };
	const canvas = document.createElement("canvas");
	canvas.width = width;
	canvas.height = height;
	const ctx = canvas.getContext("2d");
	if (!ctx) return { dataUrl: "", width, height };
	await new Promise<void>((resolve) => {
		const image = new Image();
		image.crossOrigin = "anonymous";
		image.onload = () => {
			const sx = frame.x * image.naturalWidth;
			const sy = frame.y * image.naturalHeight;
			const sw = frame.width * image.naturalWidth;
			const sh = frame.height * image.naturalHeight;
			ctx.drawImage(image, sx, sy, sw, sh, 0, 0, width, height);
			for (const tile of deck.tiles) {
				const crop = tile.crop;
				ctx.strokeStyle = "#4b7bec";
				ctx.lineWidth = 2;
				ctx.strokeRect(crop.x * width, crop.y * height, crop.width * width, crop.height * height);
			}
			resolve();
		};
		image.onerror = () => resolve();
		image.src = deck.source.src;
	});
	return { dataUrl: canvas.toDataURL("image/png"), width, height };
}

function presentationDeckToSvg(deck: PresentationDeckV1, width: number, height: number, pngDataUrl: string): string {
	const overlays = deck.tiles
		.map((tile) => {
			const crop = tile.crop;
			return `<rect x="${crop.x * width}" y="${crop.y * height}" width="${crop.width * width}" height="${crop.height * height}" fill="none" stroke="#4b7bec" stroke-width="2"/>`;
		})
		.join("");
	return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${width} ${height}" width="${width}" height="${height}"><image href="${pngDataUrl}" width="${width}" height="${height}"/>${overlays}</svg>`;
}

/** @emoji 💾 Registers presentation deck SVG/PNG export handlers for the OS media graph. */
export function registerPresentationMediaExportHandlers(): void {
	registerOsMediaExportHandler("presentation.deck", "png", async (doc) => {
		const deck = doc as PresentationDeckV1;
		const { dataUrl } = await presentationDeckToPngDataUrl(deck);
		const blob = await fetch(dataUrl).then((response) => response.blob());
		return { data: new Uint8Array(await blob.arrayBuffer()), mimeType: "image/png", fileName: "presentation.png" };
	});
	registerOsMediaExportHandler("presentation.deck", "svg", async (doc) => {
		const deck = doc as PresentationDeckV1;
		const { dataUrl, width, height } = await presentationDeckToPngDataUrl(deck);
		return {
			data: presentationDeckToSvg(deck, width, height, dataUrl),
			mimeType: "image/svg+xml",
			fileName: "presentation.svg",
		};
	});
}
//#endregion 🔖MediaExport

