// #region 🧲Header
/** @emoji 🔧 Procedural 2D play app — flow editor with 2d preview. */
// #endregion 🧲Header

import {
    buildFlowPlayCatalogueTree,
    buildFlowPlayHierarchyTree,
    buildFlowPlayInspectorTree,
    buildFlowGeneratePlayToolbarTools,
    buildGeneratePlayWindowEngagement,
    buildGeneratePlayWindowMeasures,
    createDefaultGenerations,
    parseFlowPlayFixtureJson,
    runGenerationCommand,
} from "@semio-tech/flow-core";
import { flowFixtureToFormSpec, type FlowGeneration } from "@semio-tech/forms-react";
import { registerOsMediaExportHandler } from "@semio-tech/framework-os-core";
import {
	drawingSceneFromPreviewPayload,
	drawingSceneToSvgMarkup,
	rasterizeSvgMarkupToPngDataUrl,
	type DrawingScene,
} from "@semio-tech/kernel-2d-js";
import { FlowOrchestratorClient } from "../../../../flow/worker-client.ts";
import {
    buildCatalogueKindsTreeSections,
    buildFlowContextMenuItems,
    applyFlowFixtureEditOp,
    backwardsFlowFixtureEditOp,
    diffFlowFixtureEditOp,
    flowPlayCatalogueItemDragData,
    type CatalogueSection,
    type FlowCanvasCommandRequest,
    type FlowCanvasContextMenuContext,
    type FlowContextMenuDispatch,
    type FlowExtensionEntry,
    type FlowFixtureEditOp,
    type FlowGraphEditOp,
    type FlowReorganizeRequest,
} from "@semio-tech/flow-react";
import { DocumentVcsStore, createDocumentVcsEnvelope, recordProjectionChange } from "@semio-tech/vcs-core/internal";
import {
    AppRuntime,
    buildFlowWindowBody,
    buildFormsWindowBody,
    buildPuzzle2dWindowBody,
    CommandBus,
    Controller,
    createDefaultLayout,
    createPlayAppRuntime,
    enforcePlaygroundWindowEngagementInput,
    isPlaygroundExampleLocked,
    isPlaygroundNoExampleId,
    ModeRuntime,
    Platform,
    PLAYGROUND_NO_EXAMPLE_ID,
    playgroundResolvedExampleId,
    registerWindowBody,
    registerSidePanelBody,
    buildControllerTreeSidePanelBody,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID,
    FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    type SideTabSpec,
    eagerPlayExampleGlob,
    type AppTools,
    type CommandDescriptor,
    type PlaygroundExampleCatalog,
    type PlaygroundExampleHost,
    type ToolLeaf,
    toolCollection,
    type UiNode,
    type UiTreeNode,
    type UiTreeSectionNode,
    WindowKindRuntime,
    type WindowBodyViewContext,
    type WindowEngagement,
    createPlaygroundApp,
} from "@semio-tech/framework-playground-core";
import {
    extractChannelPreviewItems,
    filterVisiblePreviewItems,
    PROCEDURAL_DEFAULT_FIXTURE,
    procedural2dExtensionHost,
    proceduralFixtureToJson,
    resolveGeometryTargets,
    type FlowFixture,
    type ProceduralChannelRef,
    type ProceduralFixtureEdge,
    type ProceduralPreviewItem,
    type ProceduralPreviewShowMode,
} from "@semio-tech/procedural-2d-react";
import { PROCEDURAL_2D_PLAY_EXAMPLE_DEFAULT_ID, resolveProcedural2dPlayExampleSlug } from "./example-slugs.ts";

export { PROCEDURAL_2D_PLAY_EXAMPLE_DEFAULT_ID, resolveProcedural2dPlayExampleSlug };
import type { ContextMenuItem } from "@semio-tech/ui-react";
import { bootstrapElementsSurfaceChromeDocument, selectionMergeIds, type SelectionMergeMode } from "@semio-tech/ui-react";

function previewItemKey(item: ProceduralPreviewItem): string {
	return `${item.widgetId}:${item.port}:${item.direction}`;
}

function previewItemsWithScenes(
	items: ProceduralPreviewItem[],
	previewMeshes?: Readonly<Record<string, unknown>>,
	previous: readonly ProceduralPreviewItem[] = [],
): ProceduralPreviewItem[] {
	const previousByKey = new Map(previous.map((item) => [previewItemKey(item), item]));
	return items.map((item) => {
		if (item.kind !== "drawing") return item;
		const previousItem = previousByKey.get(previewItemKey(item));
		const scene =
			drawingSceneFromPreviewPayload(previewMeshes?.[item.handle]) ??
			(previousItem?.handle === item.handle ? previousItem.scene : undefined);
		return scene ? { ...item, scene } : item;
	});
}

export const PROCEDURAL_2D_PLAY_APP_ID = "procedural-2d-play";
export const PROCEDURAL_2D_PLAY_CONTROLLER_ID = "procedural-2d-play";
export const PROCEDURAL_2D_PLAY_SURFACE_ID = "procedural2d.play";
export const PROCEDURAL_2D_PLAY_BODY_KEY_MAIN = "procedural2d.play.main";
export const PROCEDURAL_2D_PLAY_WINDOW_KIND_ID = "procedural2d-main";
export const PROCEDURAL_2D_PLAY_WINDOW_KIND_PREVIEW = "procedural2d-preview";
export const PROCEDURAL_2D_PLAY_BODY_KEY_PREVIEW = "procedural2d.play.preview";
export const PROCEDURAL_2D_PLAY_BODY_KEY_GENERATE = "procedural2d.play.generate";
export const PROCEDURAL_2D_PLAY_SURFACE_ID_PREVIEW = "procedural2d.play.preview";
export const PROCEDURAL_2D_PLAY_SURFACE_ID_GENERATE = "procedural2d.play.generate";

export const PROCEDURAL_2D_PLAY_DEFAULT_FIXTURE: FlowFixture = PROCEDURAL_DEFAULT_FIXTURE;
export const PROCEDURAL_2D_PLAY_DEFAULT_FIXTURE_JSON = proceduralFixtureToJson(PROCEDURAL_DEFAULT_FIXTURE);

export const PROCEDURAL_2D_PLAY_LAYOUT = createDefaultLayout(
	[PROCEDURAL_2D_PLAY_WINDOW_KIND_ID, PROCEDURAL_2D_PLAY_WINDOW_KIND_PREVIEW],
	"row",
	[55, 45],
	["Flow", "Preview"],
);
export const PROCEDURAL_2D_PLAY_KINDS_TAB_ID = "procedural2d-play-kinds";
export const PROCEDURAL_2D_PLAY_EXTENSIONS_TAB_ID = "procedural2d-play-extensions";
export const PROCEDURAL_2D_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const PROCEDURAL_2D_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const PROCEDURAL_2D_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";
export const PROCEDURAL_2D_PLAY_HIERARCHY_BODY_KEY = "procedural2d.play.hierarchy";
export const PROCEDURAL_2D_PLAY_CATALOGUE_BODY_KEY = "procedural2d.play.catalogue";
export const PROCEDURAL_2D_PLAY_INSPECTION_BODY_KEY = "procedural2d.play.inspection";

const PROCEDURAL_2D_PLAY_STORE_KEY = "procedural2d.fixture";

/** @emoji 💾 Local persistence for procedural flow fixtures. */
export interface Procedural2dPlayFixtureStore {
	load(): string | null;
	save(fixtureJson: string): void;
	clear(): void;
}

export function createProcedural2dPlayFixtureStore(storage?: Pick<Storage, "getItem" | "setItem" | "removeItem">): Procedural2dPlayFixtureStore {
	const resolved =
		storage ??
		(typeof globalThis.localStorage !== "undefined"
			? globalThis.localStorage
			: (() => {
					const backing = new Map<string, string>();
					return {
						getItem: (key: string) => backing.get(key) ?? null,
						setItem: (key: string, value: string) => {
							backing.set(key, value);
						},
						removeItem: (key: string) => {
							backing.delete(key);
						},
					};
				})());
	return {
		load(): string | null {
			return resolved.getItem(PROCEDURAL_2D_PLAY_STORE_KEY);
		},
		save(fixtureJson: string): void {
			resolved.setItem(PROCEDURAL_2D_PLAY_STORE_KEY, fixtureJson);
		},
		clear(): void {
			resolved.removeItem(PROCEDURAL_2D_PLAY_STORE_KEY);
		},
	};
}

export type ProceduralLayoutOrientation = "leftRight" | "topBottom";
export type ProceduralPlaySelectionMode = SelectionMergeMode;
export type ProceduralPlaySelectionMethod = "rectangle" | "lasso";

const DEFAULT_LAYER_SPACING = 120;
const DEFAULT_SIBLING_GAP = 40;

function procedural2dPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: PROCEDURAL_2D_PLAY_CONTROLLER_ID, command, args };
}

function buildProceduralLayoutOptionsJson(layerSpacing: number, siblingGap: number, orientation: ProceduralLayoutOrientation): string {
	return JSON.stringify({ layerSpacing, siblingGap, orientation });
}

/** @emoji 🖱️ Procedural play canvas right-click menu with preview actions. */
export function buildProcedural2dPlayCanvasContextMenu(ctx: FlowCanvasContextMenuContext, dispatch: FlowContextMenuDispatch): ContextMenuItem[] {
	const items = [...buildFlowContextMenuItems(ctx, dispatch)];
	if (ctx.hoveredNodeId) {
		items.splice(items.length - 1, 0, {
			id: "procedural2d.ctx.isolatePreview",
			label: "Isolate in preview",
			icon: "eye",
			onSelect: () => {
				dispatch("setSelection", { ids: [ctx.hoveredNodeId], mode: "default" });
				dispatch("setShowMode", { id: "selected" });
			},
		});
	}
	return items;
}

/** @emoji 🧩 Workbench extensions tab: installed modules with enable/disable toggles. */
export function buildProcedural2dPlayExtensionsTree(entries: readonly FlowExtensionEntry[]): UiNode {
	if (!entries.length) {
		return {
			type: "tree",
			sections: [
				{
					id: "procedural2d-play-extensions.empty",
					label: "Extensions",
					defaultOpen: false,
					items: [{ id: "procedural2d-play-extensions.empty.msg", label: "Loading extensions…" }],
				},
			],
		};
	}
	const commandItems = procedural2dExtensionHost.activeCommands().map((command) => ({
		id: `procedural2d-play-extensions.command.${command.id}`,
		label: command.title,
		description: command.id,
		command: procedural2dPlayCmd("runExtensionCommand", { commandId: command.id }),
	}));
	const sections: UiTreeSectionNode[] = [
		{
			id: "procedural2d-play-extensions.installed",
			label: "Installed",
			defaultOpen: false,
			items: entries.map((entry) => {
				const operators = entry.manifest.contributes.operators ?? [];
				const schemas = entry.manifest.contributes.schemas ?? [];
				const commands = entry.manifest.contributes.commands ?? [];
				return {
					id: `procedural2d-play-extensions.${entry.id}`,
					label: entry.manifest.name,
					description: `${entry.manifest.version} · ${entry.active ? "enabled" : "disabled"} · ${operators.length} operators · ${schemas.length} schemas · ${commands.length} commands`,
					command: procedural2dPlayCmd("toggleExtension", { id: entry.id, enabled: !entry.active }),
				};
			}),
		},
	];
	if (commandItems.length) {
		sections.push({
			id: "procedural2d-play-extensions.commands",
			label: "Commands",
			defaultOpen: false,
			items: commandItems,
		});
	}
	return { type: "tree", sections };
}

/** @emoji 🏷️ Workbench catalogue tab: module sections plus Inputs and Outputs. */
export function buildProcedural2dPlayKindsTree(sections: readonly CatalogueSection[]): UiNode {
	if (!sections.length) {
		return {
			type: "tree",
			sections: [
				{
					id: "procedural2d-play-kinds.empty",
					label: "Catalogue",
					defaultOpen: false,
					items: [{ id: "procedural2d-play-kinds.empty.msg", label: "Loading catalogue…" }],
				},
			],
		};
	}
	const treeSections: UiTreeSectionNode[] = buildCatalogueKindsTreeSections(sections, "procedural2d-play-kinds", flowPlayCatalogueItemDragData);
	return { type: "tree", sections: treeSections };
}

export function buildProcedural2dPlayHierarchyTree(fixtureJson: string, selectedNodeIds: readonly string[]): UiNode {
	return buildFlowPlayHierarchyTree(fixtureJson, selectedNodeIds, PROCEDURAL_2D_PLAY_CONTROLLER_ID);
}

export function buildProcedural2dPlayCatalogueTree(sections: readonly CatalogueSection[], extensionEntries: readonly FlowExtensionEntry[]): UiNode {
	return buildFlowPlayCatalogueTree(sections, extensionEntries);
}

export function buildProcedural2dPlayInspectorTree(fixtureJson: string, selectedNodeIds: readonly string[]): UiNode {
	return buildFlowPlayInspectorTree(fixtureJson, selectedNodeIds, PROCEDURAL_2D_PLAY_CONTROLLER_ID);
}

/** @emoji 🧰 Snapshot read by {@link buildProcedural2dPlayToolbarTools}. */
export interface Procedural2dPlayToolbarState {
	readonly selectionMethod: ProceduralPlaySelectionMethod;
	readonly selectionMode: ProceduralPlaySelectionMode;
	readonly showMode: ProceduralPreviewShowMode;
	readonly selectionCount: number;
	readonly hasStoredFixture: boolean;
}

/** @emoji 🔗 Host bridge for toolbar commands that need React (file picker, download). */
export interface Procedural2dPlayHostBridge {
	getToolbarState(): Procedural2dPlayToolbarState;
	runHostCommand(command: string, args?: unknown): void;
}

/** @emoji 🧰 Playground {@link AppTools} for procedural play (selection, save, view, actions). */
export function buildProcedural2dPlayToolbarTools(state: Procedural2dPlayToolbarState, controllerId: string): AppTools {
	const selectionTools: ToolLeaf[] = [
		{
			id: "procedural2d.select.rectangle",
			kind: "toggle",
			iconId: "square",
			text: "Rectangle",
			order: 0,
			pressed: state.selectionMethod === "rectangle",
			controllerId,
			command: "setSelectionMethod",
			args: { method: "rectangle" },
		},
		{
			id: "procedural2d.select.lasso",
			kind: "toggle",
			iconId: "lasso",
			text: "Lasso",
			order: 1,
			pressed: state.selectionMethod === "lasso",
			controllerId,
			command: "setSelectionMethod",
			args: { method: "lasso" },
		},
		{
			id: "procedural2d.select.mode.default",
			kind: "toggle",
			iconId: "mouse-pointer-2",
			text: "Default",
			order: 2,
			pressed: state.selectionMode === "default",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "default" },
		},
		{
			id: "procedural2d.select.mode.additive",
			kind: "toggle",
			iconId: "plus",
			text: "Add",
			order: 3,
			pressed: state.selectionMode === "additive",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "additive" },
		},
		{
			id: "procedural2d.select.mode.subtractive",
			kind: "toggle",
			iconId: "minus",
			text: "Subtract",
			order: 4,
			pressed: state.selectionMode === "subtractive",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "subtractive" },
		},
		{
			id: "procedural2d.select.mode.invertive",
			kind: "toggle",
			iconId: "arrow-right-left",
			text: "Invert",
			order: 5,
			pressed: state.selectionMode === "invertive",
			controllerId,
			command: "setSelectionMode",
			args: { mode: "invertive" },
		},
		{
			id: "procedural2d.selection.clear",
			kind: "button",
			iconId: "x",
			label: "Clear",
			order: 6,
			disabled: state.selectionCount === 0,
			controllerId,
			command: "clearSelection",
		},
	];
	const saveTools: ToolLeaf[] = [
		{
			id: "procedural2d.save.stored",
			kind: "button",
			iconId: "hard-drive",
			label: "Store",
			order: 0,
			controllerId,
			command: "saveStored",
		},
		{
			id: "procedural2d.save.download",
			kind: "button",
			iconId: "save",
			label: "Download",
			order: 1,
			controllerId,
			command: "saveDownload",
		},
		{
			id: "procedural2d.save.load",
			kind: "button",
			iconId: "folder-open",
			label: "Load",
			order: 2,
			controllerId,
			command: "loadRequest",
		},
		{
			id: "procedural2d.save.loadStored",
			kind: "button",
			iconId: "rotate-ccw",
			label: "Restore",
			order: 3,
			disabled: !state.hasStoredFixture,
			controllerId,
			command: "loadStored",
		},
		{
			id: "procedural2d.save.reset",
			kind: "button",
			iconId: "refresh-cw",
			label: "Reset",
			order: 4,
			controllerId,
			command: "resetFixture",
		},
	];

	const exportTools: ToolLeaf[] = [
		{
			id: "procedural2d.export.svg",
			kind: "button",
			iconId: "file-code",
			label: "SVG",
			order: 0,
			controllerId,
			command: "exportSvg",
		},
		{
			id: "procedural2d.export.pdf",
			kind: "button",
			iconId: "file-text",
			label: "PDF",
			order: 1,
			controllerId,
			command: "exportPdf",
		},
		{
			id: "procedural2d.export.png",
			kind: "button",
			iconId: "image",
			label: "PNG",
			order: 2,
			controllerId,
			command: "exportPng",
		},
	];
	return [
		toolCollection("selection", "mouse-pointer-2", selectionTools),
		toolCollection("save", "save", saveTools),
		toolCollection("view", "layout-grid", [
			{
				id: "procedural2d.view.everything",
				kind: "toggle",
				iconId: "layers",
				text: "Everything",
				order: 0,
				pressed: state.showMode === "everything",
				controllerId,
				command: "setShowMode",
				args: { id: "everything" },
			},
			{
				id: "procedural2d.view.selected",
				kind: "toggle",
				iconId: "eye",
				text: "Selected",
				order: 1,
				pressed: state.showMode === "selected",
				controllerId,
				command: "setShowMode",
				args: { id: "selected" },
			},
		]),
		toolCollection("export", "download", exportTools),
		toolCollection("actions", "more-horizontal", [
			{
				id: "procedural2d.action.reorganize",
				kind: "button",
				iconId: "layout-grid",
				label: "Reorganize",
				order: 0,
				controllerId,
				command: "reorganize",
			},
			{
				id: "procedural2d.action.delete",
				kind: "button",
				iconId: "trash-2",
				label: "Delete",
				order: 1,
				disabled: state.selectionCount === 0,
				controllerId,
				command: "deleteSelection",
			},
		]),
	];
}


/** @emoji 🎛 Procedural play shell controller. */
export class Procedural2dPlayController extends Controller implements PlaygroundExampleHost {
	readonly mainMode = new ModeRuntime("main", "Edit", undefined);
	readonly generateMode = new ModeRuntime("generate", "Generate", undefined);
	private activeExampleId = playgroundResolvedExampleId(PROCEDURAL_2D_PLAY_EXAMPLE_DEFAULT_ID);
	private readonly docStore = new DocumentVcsStore<FlowFixture, FlowFixtureEditOp>({
		envelope: createDocumentVcsEnvelope(
			"flow.fixture",
			"procedural-2d-play",
			parseFlowPlayFixtureJson(procedural2dPlayFixtureJson(PROCEDURAL_2D_PLAY_EXAMPLE_DEFAULT_ID)) ?? PROCEDURAL_2D_PLAY_DEFAULT_FIXTURE,
		),
		applyOp: applyFlowFixtureEditOp,
		backwardsOp: backwardsFlowFixtureEditOp,
		diffOp: diffFlowFixtureEditOp,
	});
	private generations: FlowGeneration[] = createDefaultGenerations();
	private selectedGenerationId: string | null = null;
	private generatePreviewText = "—";
	private evalClient: FlowOrchestratorClient | null = null;
	private readonly fixtureStore: Procedural2dPlayFixtureStore;
	private hostBridge: Procedural2dPlayHostBridge | null = null;
	private previewText = "—";
	private catalogueSections: CatalogueSection[] = [];
	private catalogueRevision = 0;
	private readonly snapshotListeners = new Set<() => void>();
	private engagementInput = "";
	private layerSpacing = DEFAULT_LAYER_SPACING;
	private siblingGap = DEFAULT_SIBLING_GAP;
	private orientation: ProceduralLayoutOrientation = "leftRight";
	private reorganizeEpoch = 0;
	private reorganizeOptionsJson = buildProceduralLayoutOptionsJson(DEFAULT_LAYER_SPACING, DEFAULT_SIBLING_GAP, "leftRight");
	private commandRequestEpoch = 0;
	private commandRequestPayload: Omit<FlowCanvasCommandRequest, "epoch"> = { command: "" };
	private extensionRevision = 0;
	private previewItems: ProceduralPreviewItem[] = [];
	private selectedChannels: ProceduralChannelRef[] = [];
	private preselectNodeIds: string[] = [];
	private preselectRemovedNodeIds: string[] = [];
	private hoveredChannel: ProceduralChannelRef | null = null;
	private fixtureEdges: ProceduralFixtureEdge[] = [];
	private previewOffNodeIds: string[] = [];
	private showMode: ProceduralPreviewShowMode = "everything";
	private generateEngagementInput = "";
	private selectionMode: ProceduralPlaySelectionMode = "default";
	private selectionMethod: ProceduralPlaySelectionMethod = "rectangle";
	private interactionRevision = 0;

	constructor(commandBus: CommandBus, hostNotify: () => void, fixtureStore: Procedural2dPlayFixtureStore = createProcedural2dPlayFixtureStore()) {
		super(PROCEDURAL_2D_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.fixtureStore = fixtureStore;
		this.fixtureEdges = this.parseFixtureEdges(this.getFixtureJson());
		this.selectedGenerationId = this.generations[0]?.id ?? null;
		this.rebuildShellMode();
		this.rebuildGenerateMode();
	}

	hasStoredFixture(): boolean {
		return this.fixtureStore.load() != null;
	}

	getExampleCatalog(): PlaygroundExampleCatalog | null {
		if (isPlaygroundExampleLocked()) return null;
		return { activeExampleId: this.activeExampleId, options: [...PROCEDURAL_2D_PLAY_EXAMPLE_OPTIONS] };
	}

	/** @emoji 🔗 Attaches the React host bridge for toolbar file IO. */
	setHostBridge(bridge: Procedural2dPlayHostBridge | null): void {
		this.hostBridge = bridge;
		this.rebuildToolbarTools();
	}

	private toolbarState(): Procedural2dPlayToolbarState {
		return (
			this.hostBridge?.getToolbarState() ?? {
				selectionMethod: this.selectionMethod,
				selectionMode: this.selectionMode,
				showMode: this.showMode,
				selectionCount: this.getSelectedNodeIds().length,
				hasStoredFixture: this.hasStoredFixture(),
			}
		);
	}

	/** @emoji 🔄 Rebuilds {@link ModeRuntime.tools} from the latest toolbar snapshot. */
	rebuildToolbarTools(): void {
		this.mainMode.tools = buildProcedural2dPlayToolbarTools(this.toolbarState(), this.id);
	}

	private resetInteractionState(): void {
		this.pointerFocus.setSelection([]);
		this.pointerFocus.clearHover();
		this.preselectNodeIds = [];
		this.preselectRemovedNodeIds = [];
		this.hoveredChannel = null;
		this.selectedChannels = [];
		this.previewOffNodeIds = [];
		this.previewItems = [];
	}

	private parseFixtureEdges(json: string): ProceduralFixtureEdge[] {
		try {
			const parsed = JSON.parse(json) as {
				synapses?: Array<{
					from?: string;
					to?: string;
					from_port?: string;
					to_port?: string;
					fromPort?: string;
					toPort?: string;
				}>;
			};
			if (!Array.isArray(parsed.synapses)) return [];
			return parsed.synapses.flatMap((synapse) => {
				if (typeof synapse.from !== "string" || typeof synapse.to !== "string") return [];
				const fromPort =
					typeof synapse.from_port === "string"
						? synapse.from_port
						: typeof synapse.fromPort === "string"
							? synapse.fromPort
							: "";
				const toPort =
					typeof synapse.to_port === "string" ? synapse.to_port : typeof synapse.toPort === "string" ? synapse.toPort : "";
				return [{ source: `${synapse.from}:${fromPort}`, target: `${synapse.to}:${toPort}` }];
			});
		} catch {
			return [];
		}
	}

	private projection(): FlowFixture {
		return this.docStore.projection();
	}

	private commitFixture(next: FlowFixture): void {
		this.applyFixtureEdit({ op: "setDocument", document: next });
	}

	private applyFixtureEdit(op: FlowFixtureEditOp): void {
		recordProjectionChange(this.docStore, [op]);
	}

	getDocumentVcsStore(): DocumentVcsStore<FlowFixture, FlowFixtureEditOp> {
		return this.docStore;
	}

	private applyFixtureJson(json: string, resetInteraction = false): void {
		const parsed = parseFlowPlayFixtureJson(json);
		if (!parsed) return;
		const nextJson = proceduralFixtureToJson(parsed);
		const unchanged = nextJson === this.getFixtureJson();
		if (unchanged && !resetInteraction) return;
		if (!unchanged) {
			this.commitFixture(parsed);
			this.fixtureEdges = this.parseFixtureEdges(nextJson);
		}
		if (resetInteraction) this.resetInteractionState();
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.rebuildShellMode();
		this.emit();
	}

	private renameFlowWidget(oldId: string, newId: string): void {
		const trimmed = newId.trim();
		if (!trimmed || trimmed === oldId) return;
		const fixture = this.projection();
		if (fixture.widgets.some((widget) => widget.id === trimmed)) return;
		this.setSelectedNodeIds(this.getSelectedNodeIds().map((id) => (id === oldId ? trimmed : id)));
		this.applyFixtureEdit({ op: "renameWidget", oldId, newId: trimmed });
		this.fixtureEdges = this.parseFixtureEdges(proceduralFixtureToJson(this.projection()));
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.rebuildShellMode();
		this.emit();
	}

	private patchFlowWidget(widgetId: string, field: string, value: unknown): void {
		this.applyFixtureEdit({ op: "patchWidget", widgetId, field, value });
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.rebuildShellMode();
		this.emit();
	}

	private loadFixtureById(fixtureId: string): void {
		const nextId = isPlaygroundNoExampleId(fixtureId) ? PLAYGROUND_NO_EXAMPLE_ID : fixtureId;
		const nextJson = procedural2dPlayFixtureJson(nextId);
		if (nextId === this.activeExampleId && nextJson === this.getFixtureJson()) return;
		this.activeExampleId = nextId;
		this.applyFixtureJson(nextJson, true);
	}

	getFixtureJson(): string {
		return proceduralFixtureToJson(this.projection());
	}

	getPreviewText(): string {
		return this.previewText;
	}

	getGenerations(): readonly FlowGeneration[] {
		return this.generations;
	}

	getSelectedGenerationId(): string | null {
		return this.selectedGenerationId;
	}

	getGeneratePreviewText(): string {
		return this.generatePreviewText;
	}

	getGenerateFormSpecJson(): string {
		return JSON.stringify(flowFixtureToFormSpec(this.getFixtureJson()));
	}

	private getEvalClient(): FlowOrchestratorClient {
		if (!this.evalClient) this.evalClient = new FlowOrchestratorClient();
		return this.evalClient;
	}

	getCatalogueSections(): readonly CatalogueSection[] {
		return this.catalogueSections;
	}

	getCatalogueRevision(): number {
		return this.catalogueRevision;
	}

	getExtensionRevision(): number {
		return this.extensionRevision;
	}

	getExtensionEntries(): readonly FlowExtensionEntry[] {
		return procedural2dExtensionHost.listEntries();
	}

	getPreviewItems(): readonly ProceduralPreviewItem[] {
		return this.previewItems;
	}

	getSelectedNodeIds(): readonly string[] {
		return this.pointerFocus.getSnapshot().selection;
	}

	private setSelectedNodeIds(ids: readonly string[]): void {
		const next = [...new Set(ids.filter((id) => typeof id === "string"))];
		if (JSON.stringify(next) === JSON.stringify(this.getSelectedNodeIds())) return;
		this.pointerFocus.setSelection(next);
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.rebuildToolbarTools();
		this.emit();
	}

	getPreselectNodeIds(): readonly string[] {
		return this.preselectNodeIds;
	}

	getPreselectRemovedNodeIds(): readonly string[] {
		return this.preselectRemovedNodeIds;
	}

	getSelectionMode(): ProceduralPlaySelectionMode {
		return this.selectionMode;
	}

	getSelectionMethod(): ProceduralPlaySelectionMethod {
		return this.selectionMethod;
	}

	getHoveredNodeId(): string | null {
		return this.pointerFocus.getSnapshot().hover;
	}

	getHoveredChannel(): ProceduralChannelRef | null {
		return this.hoveredChannel;
	}

	getSelectedChannels(): readonly ProceduralChannelRef[] {
		return this.selectedChannels;
	}

	getHoveredGeometryTargets(): readonly ProceduralChannelRef[] {
		if (this.hoveredChannel) {
			return resolveGeometryTargets([this.hoveredChannel], null, this.previewItems, this.fixtureEdges);
		}
		if (this.getHoveredNodeId()) {
			return resolveGeometryTargets([], this.getHoveredNodeId(), this.previewItems, this.fixtureEdges);
		}
		return [];
	}

	getSelectedGeometryTargets(): readonly ProceduralChannelRef[] {
		if (this.selectedChannels.length > 0) {
			return resolveGeometryTargets(this.selectedChannels, null, this.previewItems, this.fixtureEdges);
		}
		if (this.getSelectedNodeIds().length > 0) {
			const targets: ProceduralChannelRef[] = [];
			for (const widgetId of this.getSelectedNodeIds()) {
				targets.push(...resolveGeometryTargets([], widgetId, this.previewItems, this.fixtureEdges));
			}
			return targets;
		}
		return [];
	}

	getPreviewOffNodeIds(): readonly string[] {
		return this.previewOffNodeIds;
	}

	getShowMode(): ProceduralPreviewShowMode {
		return this.showMode;
	}

	getInteractionRevision(): number {
		return this.interactionRevision;
	}

	getPrimaryDrawingHandle(): string | null {
		for (const item of this.previewItems) {
			if (item.kind === "drawing" && item.direction === "out" && typeof item.handle === "string") {
				return item.handle;
			}
		}
		return null;
	}

	private flowWindowMeasures(): readonly WindowMeasure[] {
		return [
			{
				kind: "select",
				id: `${PROCEDURAL_2D_PLAY_WINDOW_KIND_ID}-show`,
				label: "Show",
				value: this.showMode,
				items: [
					{ id: "everything", value: "everything", label: "Everything" },
					{ id: "selected", value: "selected", label: "Selected" },
				],
				onChange: { controllerId: PROCEDURAL_2D_PLAY_CONTROLLER_ID, command: "setShowMode" },
			},
			{
				kind: "slider",
				id: "procedural-layer-spacing",
				label: "Layer spacing",
				value: this.layerSpacing,
				min: 40,
				max: 320,
				step: 10,
				onChange: procedural2dPlayCmd("setSpacing", { field: "layerSpacing" }),
			},
			{
				kind: "slider",
				id: "procedural-sibling-gap",
				label: "Sibling gap",
				value: this.siblingGap,
				min: 10,
				max: 160,
				step: 5,
				onChange: procedural2dPlayCmd("setSpacing", { field: "siblingGap" }),
			},
		];
	}

	private previewWindowMeasures(): readonly WindowMeasure[] {
		return [
			{
				kind: "select",
				id: `${PROCEDURAL_2D_PLAY_WINDOW_KIND_PREVIEW}-show`,
				label: "Show",
				value: this.showMode,
				items: [
					{ id: "everything", value: "everything", label: "Everything" },
					{ id: "selected", value: "selected", label: "Selected" },
				],
				onChange: { controllerId: PROCEDURAL_2D_PLAY_CONTROLLER_ID, command: "setShowMode" },
			},
		];
	}

	/** @emoji 🔔 Subscribes to catalogue updates for workbench kinds panel refresh. */
	subscribeSnapshot(listener: () => void): () => void {
		this.snapshotListeners.add(listener);
		return () => this.snapshotListeners.delete(listener);
	}

	private notifySnapshot(): void {
		for (const listener of this.snapshotListeners) {
			listener();
		}
	}

	getReorganize(): FlowReorganizeRequest {
		return { epoch: this.reorganizeEpoch, optionsJson: this.reorganizeOptionsJson };
	}

	getCommandRequest(): FlowCanvasCommandRequest {
		return { epoch: this.commandRequestEpoch, ...this.commandRequestPayload };
	}

	private syncReorganizeOptionsJson(): void {
		this.reorganizeOptionsJson = buildProceduralLayoutOptionsJson(this.layerSpacing, this.siblingGap, this.orientation);
	}

	private triggerReorganize(): void {
		this.syncReorganizeOptionsJson();
		this.reorganizeEpoch += 1;
		this.rebuildShellMode();
		this.emit();
	}

	private flowWindowEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: "engagement-input",
				value: this.engagementInput,
				placeholder: "Reorganize, lr, tb",
				onChange: procedural2dPlayCmd("engagementInput"),
				onSubmit: procedural2dPlayCmd("engagementSubmit"),
			},
			possibleEngagements: [
				{ id: "procedural2d.tool.reorganize", label: "Reorganize", command: procedural2dPlayCmd("reorganize") },
				{ id: "procedural2d.layout.leftRight", label: "Left to Right", command: procedural2dPlayCmd("setOrientation", { orientation: "leftRight" }) },
				{ id: "procedural2d.layout.topBottom", label: "Top to Bottom", command: procedural2dPlayCmd("setOrientation", { orientation: "topBottom" }) },
			],
			status: [{ id: "procedural-layout-orientation", text: this.orientation === "leftRight" ? "Left to right" : "Top to bottom" }],
		};
	}

	private previewWindowEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: "preview-engagement-input",
				value: "",
				placeholder: "Preview",
				onChange: procedural2dPlayCmd("previewEngagementInput"),
				onSubmit: procedural2dPlayCmd("previewEngagementSubmit"),
			},
			status: [{ id: "procedural-preview-item-count", text: `${this.previewItems.length} preview items` }],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.windowKinds = [
			new WindowKindRuntime(PROCEDURAL_2D_PLAY_WINDOW_KIND_ID, "Flow", PROCEDURAL_2D_PLAY_BODY_KEY_MAIN, undefined, this.flowWindowMeasures(), this.flowWindowEngagement()),
			new WindowKindRuntime(
				PROCEDURAL_2D_PLAY_WINDOW_KIND_PREVIEW,
				"Preview",
				PROCEDURAL_2D_PLAY_BODY_KEY_PREVIEW,
				undefined,
				this.previewWindowMeasures(),
				this.previewWindowEngagement(),
			),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Procedural play window "${windowKind.id}"`);
		}
		this.rebuildToolbarTools();
	}

	private rebuildGenerateMode(): void {
		this.generateMode.tools = buildFlowGeneratePlayToolbarTools(PROCEDURAL_2D_PLAY_CONTROLLER_ID);
		this.generateMode.windowKinds = [
			new WindowKindRuntime(
				PROCEDURAL_2D_PLAY_WINDOW_KIND_ID,
				"Generate",
				PROCEDURAL_2D_PLAY_BODY_KEY_GENERATE,
				undefined,
				buildGeneratePlayWindowMeasures(PROCEDURAL_2D_PLAY_WINDOW_KIND_ID, PROCEDURAL_2D_PLAY_CONTROLLER_ID, this.generations, this.selectedGenerationId),
				buildGeneratePlayWindowEngagement(
					PROCEDURAL_2D_PLAY_CONTROLLER_ID,
					this.generateEngagementInput,
					this.generatePreviewText,
					procedural2dPlayCmd("generateEngagementInput"),
					procedural2dPlayCmd("generateEngagementSubmit"),
				),
			),
		];
		for (const windowKind of this.generateMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Procedural 2D play generate window "${windowKind.id}"`);
		}
	}

	override run(command: string, args?: unknown): void {
		if (command === "generateEngagementInput") {
			const value = (args as { value?: string }).value;
			if (typeof value === "string" && value !== this.generateEngagementInput) {
				this.generateEngagementInput = value;
				this.rebuildGenerateMode();
				this.emit();
			}
			return;
		}
		if (command === "generateEngagementSubmit") {
			const name = (args as { value?: string }).value ?? this.generateEngagementInput;
			const id = this.selectedGenerationId;
			if (typeof name === "string" && name.trim() && id) {
				this.run("renameGeneration", { id, name: name.trim() });
			}
			return;
		}
		if (command === "engagementInput") {
			const value = (args as { value?: string }).value;
			if (typeof value === "string" && value !== this.engagementInput) {
				this.engagementInput = value;
				this.rebuildShellMode();
				this.emit();
			}
			return;
		}
		if (command === "engagementSubmit") {
			const value = (args as { value?: string }).value ?? this.engagementInput;
			this.applyEngagement(value);
			return;
		}
		if (command === "setSpacing") {
			const field = (args as { field?: string; value?: number }).field;
			const value = (args as { value?: number }).value;
			if (typeof value !== "number") return;
			if (field === "layerSpacing") this.layerSpacing = value;
			else if (field === "siblingGap") this.siblingGap = value;
			else return;
			this.syncReorganizeOptionsJson();
			this.rebuildShellMode();
			this.emit();
			return;
		}
		if (command === "setOrientation") {
			const orientation = (args as { orientation?: ProceduralLayoutOrientation }).orientation;
			if (orientation !== "leftRight" && orientation !== "topBottom") return;
			this.orientation = orientation;
			this.syncReorganizeOptionsJson();
			this.rebuildShellMode();
			this.emit();
			return;
		}
		if (command === "reorganize") {
			this.triggerReorganize();
			return;
		}
		if (command === "canvasCommand") {
			const canvasCommand = (args as { command?: string; argsJson?: string }).command;
			if (typeof canvasCommand !== "string" || !canvasCommand) return;
			const argsJson = (args as { argsJson?: string }).argsJson;
			this.commandRequestPayload = { command: canvasCommand, ...(argsJson !== undefined ? { argsJson } : {}) };
			this.commandRequestEpoch += 1;
			this.emit();
			return;
		}
		if (command === "setFixtureJson") {
			const { json, resetInteraction } = args as { json?: string; resetInteraction?: boolean };
			if (typeof json === "string") {
				this.applyFixtureJson(json, resetInteraction === true);
			}
			return;
		}
		if (command === "setActiveExample") {
			if (isPlaygroundExampleLocked()) return;
			const fixtureId = (args as { fixtureId?: string }).fixtureId ?? "";
			this.loadFixtureById(fixtureId);
			return;
		}
		if (command === "saveStored") {
			this.fixtureStore.save(this.getFixtureJson());
			this.rebuildShellMode();
			this.emit();
			return;
		}
		if (command === "saveDownload" || command === "loadRequest") {
			this.hostBridge?.runHostCommand(command, args);
			return;
		}
		if (command === "loadStored") {
			const json = this.fixtureStore.load();
			if (json) this.applyFixtureJson(json, true);
			return;
		}
		if (command === "resetFixture") {
			this.fixtureStore.clear();
			this.activeExampleId = PLAYGROUND_NO_EXAMPLE_ID;
			this.applyFixtureJson(PROCEDURAL_2D_PLAY_EMPTY_FIXTURE_JSON, true);
			return;
		}
		if (command === "setPreviewText") {
			const text = (args as { text?: string }).text;
			if (typeof text === "string" && text !== this.previewText) {
				this.previewText = text;
				this.emit();
			}
			return;
		}
		if (command === "setEvalOutputs") {
			const outputsJson = (args as { outputsJson?: string }).outputsJson;
			const previewMeshes = (args as { previewMeshes?: Readonly<Record<string, unknown>> }).previewMeshes;
			if (typeof outputsJson === "string") {
				this.previewItems = previewItemsWithScenes(
					extractChannelPreviewItems(outputsJson),
					previewMeshes,
					this.previewItems,
				);
				this.interactionRevision += 1;
				this.notifySnapshot();
				this.rebuildShellMode();
				this.emit();
			}
			return;
		}
		if (command === "setSelection") {
			const ids = (args as { ids?: string[] }).ids;
			const mode = (args as { mode?: ProceduralPlaySelectionMode }).mode ?? "default";
			if (!Array.isArray(ids)) return;
			const next = selectionMergeIds(mode, this.getSelectedNodeIds(), ids);
			if (JSON.stringify(next) === JSON.stringify(this.getSelectedNodeIds())) return;
			this.pointerFocus.setSelection(next);
			this.selectedChannels = [];
			this.preselectNodeIds = [];
			this.preselectRemovedNodeIds = [];
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.rebuildToolbarTools();
			this.emit();
			return;
		}
		if (command === "renameFlowWidget") {
			const oldId = (args as { oldId?: string }).oldId;
			const value = (args as { value?: string }).value;
			if (typeof oldId === "string" && typeof value === "string") {
				this.renameFlowWidget(oldId, value);
			}
			return;
		}
		if (command === "patchFlowWidget") {
			const widgetId = (args as { widgetId?: string }).widgetId;
			const field = (args as { field?: string }).field;
			const value = (args as { value?: unknown }).value;
			if (typeof widgetId === "string" && typeof field === "string") {
				this.patchFlowWidget(widgetId, field, value);
			}
			return;
		}
		if (command === "setPreselect") {
			const ids = (args as { ids?: string[] }).ids;
			const removedIds = (args as { removedIds?: string[] }).removedIds;
			if (!Array.isArray(ids) || !Array.isArray(removedIds)) return;
			this.preselectNodeIds = [...ids];
			this.preselectRemovedNodeIds = [...removedIds];
			this.interactionRevision += 1;
			this.notifySnapshot();
			return;
		}
		if (command === "setSelectionMode") {
			const mode = (args as { mode?: ProceduralPlaySelectionMode }).mode;
			if (mode !== "default" && mode !== "additive" && mode !== "subtractive" && mode !== "invertive") return;
			if (this.selectionMode === mode) return;
			this.selectionMode = mode;
			this.rebuildShellMode();
			this.emit();
			return;
		}
		if (command === "setSelectionMethod") {
			const method = (args as { method?: ProceduralPlaySelectionMethod }).method;
			if (method !== "rectangle" && method !== "lasso") return;
			if (this.selectionMethod === method) return;
			this.selectionMethod = method;
			this.rebuildShellMode();
			this.emit();
			return;
		}
		if (command === "selectAll") {
			const ids = [...new Set(this.previewItems.map((entry) => entry.widgetId))];
			this.setSelectedNodeIds(ids);
			this.preselectNodeIds = [];
			this.preselectRemovedNodeIds = [];
			return;
		}
		if (command === "clearSelection") {
			if (!this.getSelectedNodeIds().length) return;
			this.pointerFocus.setSelection([]);
			this.preselectNodeIds = [];
			this.preselectRemovedNodeIds = [];
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.rebuildToolbarTools();
			this.emit();
			return;
		}
		if (command === "deleteSelection") {
			this.run("canvasCommand", { command: "deleteSelection" });
			return;
		}
		if (command === "setHover") {
			const id = (args as { id?: string | null }).id;
			const channel = (args as { channel?: ProceduralChannelRef | null }).channel ?? null;
			const next = typeof id === "string" ? id : null;
			const channelJson = channel ? JSON.stringify(channel) : "null";
			const currentChannelJson = this.hoveredChannel ? JSON.stringify(this.hoveredChannel) : "null";
			if (next === this.getHoveredNodeId() && channelJson === currentChannelJson) return;
			this.pointerFocus.setHoverFromSource("canvas", next);
			this.hoveredChannel = channel;
			this.interactionRevision += 1;
			this.notifySnapshot();
			return;
		}
		if (command === "setSelectedChannels" || command === "setSelectChannels") {
			const channels = (args as { channels?: ProceduralChannelRef[] }).channels;
			if (!Array.isArray(channels)) return;
			const next = [...channels];
			if (JSON.stringify(next) === JSON.stringify(this.selectedChannels)) return;
			this.selectedChannels = next;
			this.pointerFocus.setSelection([...new Set(next.map((channel) => channel.widgetId))]);
			this.preselectNodeIds = [];
			this.preselectRemovedNodeIds = [];
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.rebuildToolbarTools();
			this.emit();
			return;
		}
		if (command === "setHoverChannel") {
			const channel = (args as { channel?: ProceduralChannelRef | null }).channel ?? null;
			this.run("setHover", { id: channel?.widgetId ?? null, channel });
			return;
		}
		if (command === "togglePreview") {
			const id = (args as { id?: string }).id;
			if (typeof id !== "string") return;
			const off = new Set(this.previewOffNodeIds);
			if (off.has(id)) off.delete(id);
			else off.add(id);
			this.previewOffNodeIds = [...off];
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setPreviewOff") {
			const ids = (args as { ids?: string[] }).ids;
			const fromFlow = (args as { fromFlow?: boolean }).fromFlow === true;
			if (!Array.isArray(ids)) return;
			this.previewOffNodeIds = [...ids];
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setShowMode") {
			const id = (args as { id?: string }).id ?? (args as { value?: string }).value;
			if (id !== "everything" && id !== "selected") return;
			if (this.showMode === id) return;
			this.showMode = id;
			this.interactionRevision += 1;
			this.rebuildShellMode();
			this.emit();
			return;
		}

		if (command === "exportSvg" || command === "exportPdf" || command === "exportPng") {
			this.hostBridge?.runHostCommand(command, { handle: this.getPrimaryDrawingHandle() });
			return;
		}
		if (command === "setCatalogueSections") {
			const sections = (args as { sections?: CatalogueSection[] }).sections;
			if (Array.isArray(sections)) {
				this.catalogueSections = sections;
				this.catalogueRevision += 1;
				this.notifySnapshot();
				this.emit();
			}
			return;
		}
		if (command === "toggleExtension") {
			const id = (args as { id?: string }).id;
			const enabled = (args as { enabled?: boolean }).enabled;
			if (typeof id !== "string" || typeof enabled !== "boolean") return;
			void procedural2dExtensionHost.setActive(id, enabled).then(() => {
				this.extensionRevision += 1;
				this.notifySnapshot();
				this.emit();
			});
			return;
		}
		if (command === "runExtensionCommand") {
			const commandId = (args as { commandId?: string }).commandId;
			if (typeof commandId !== "string") return;
			const result = procedural2dExtensionHost.executeCommand(commandId);
			console.log(`[DEBUG] procedural extension command ${commandId}: ${result}`);
			this.emit();
			return;
		}
		if (command === "addGeneration" || command === "removeGeneration" || command === "selectGeneration" || command === "renameGeneration" || command === "updateGenerationValues") {
			void runGenerationCommand({
				command,
				args,
				generations: this.generations,
				selectedGenerationId: this.selectedGenerationId,
				fixtureJson: this.getFixtureJson(),
				client: this.getEvalClient(),
			}).then((next) => {
				if (!next) return;
				this.generations = [...next.generations];
				this.selectedGenerationId = next.selectedGenerationId;
				if (next.generatePreviewText) this.generatePreviewText = next.generatePreviewText;
				this.rebuildGenerateMode();
				this.interactionRevision += 1;
				this.emit();
			});
			return;
		}
	}

	private applyEngagement(value: string): void {
		const trimmed = value.trim().toLowerCase();
		if (!trimmed) return;
		if (trimmed === "reorganize" || trimmed === "layout") {
			this.triggerReorganize();
			return;
		}
		if (trimmed === "lr" || trimmed === "left" || trimmed === "left to right") {
			this.orientation = "leftRight";
			this.syncReorganizeOptionsJson();
			this.rebuildShellMode();
			this.emit();
			return;
		}
		if (trimmed === "tb" || trimmed === "top" || trimmed === "top to bottom") {
			this.orientation = "topBottom";
			this.syncReorganizeOptionsJson();
			this.rebuildShellMode();
			this.emit();
			return;
		}
		this.engagementInput = "";
		this.rebuildShellMode();
		this.emit();
	}

}

export const procedural2dPlayWindowBodies: Readonly<Record<string, (ctx: WindowBodyViewContext) => UiNode>> = {
	[PROCEDURAL_2D_PLAY_BODY_KEY_MAIN]: (_ctx: WindowBodyViewContext) =>
		buildFlowWindowBody(PROCEDURAL_2D_PLAY_SURFACE_ID, PROCEDURAL_2D_PLAY_CONTROLLER_ID, PROCEDURAL_2D_PLAY_WINDOW_KIND_ID),
	[PROCEDURAL_2D_PLAY_BODY_KEY_PREVIEW]: (_ctx: WindowBodyViewContext) =>
		buildPuzzle2dWindowBody(PROCEDURAL_2D_PLAY_SURFACE_ID_PREVIEW, PROCEDURAL_2D_PLAY_CONTROLLER_ID),
	[PROCEDURAL_2D_PLAY_BODY_KEY_GENERATE]: (_ctx: WindowBodyViewContext) =>
		buildFormsWindowBody(PROCEDURAL_2D_PLAY_SURFACE_ID_GENERATE, PROCEDURAL_2D_PLAY_CONTROLLER_ID, "generate"),
};

export function registerProcedural2dPlayDeclarativeBodies(): void {
	for (const [key, build] of Object.entries(procedural2dPlayWindowBodies)) registerWindowBody(key, build);
	for (const [key, build] of Object.entries(procedural2dPlaySidePanelBodies)) registerSidePanelBody(key, build);
}

function buildProcedural2dPlayHierarchyPanelBody(ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext): UiTreeNode {
	return buildControllerTreeSidePanelBody(ctx, (ctrl) => {
		const proceduralCtrl = ctrl as Procedural2dPlayController;
		return buildProcedural2dPlayHierarchyTree(proceduralCtrl.getFixtureJson(), proceduralCtrl.getSelectedNodeIds()) as UiTreeNode;
	});
}

function buildProcedural2dPlayCataloguePanelBody(ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext): UiTreeNode {
	return buildControllerTreeSidePanelBody(ctx, (ctrl) => {
		const proceduralCtrl = ctrl as Procedural2dPlayController;
		return buildProcedural2dPlayCatalogueTree(proceduralCtrl.getCatalogueSections(), proceduralCtrl.getExtensionEntries()) as UiTreeNode;
	});
}

function buildProcedural2dPlayInspectionPanelBody(ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext): UiTreeNode {
	return buildControllerTreeSidePanelBody(ctx, (ctrl) => {
		const proceduralCtrl = ctrl as Procedural2dPlayController;
		return buildProcedural2dPlayInspectorTree(proceduralCtrl.getFixtureJson(), proceduralCtrl.getSelectedNodeIds()) as UiTreeNode;
	});
}

export const procedural2dPlaySidePanelBodies: Readonly<Record<string, (ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext) => UiTreeNode>> = {
	[PROCEDURAL_2D_PLAY_HIERARCHY_BODY_KEY]: buildProcedural2dPlayHierarchyPanelBody,
	[PROCEDURAL_2D_PLAY_CATALOGUE_BODY_KEY]: buildProcedural2dPlayCataloguePanelBody,
	[PROCEDURAL_2D_PLAY_INSPECTION_BODY_KEY]: buildProcedural2dPlayInspectionPanelBody,
};

export function buildProcedural2dPlayAppRuntime(controller: Procedural2dPlayController): AppRuntime {
	const app = createPlayAppRuntime(PROCEDURAL_2D_PLAY_APP_ID, "Procedural 2D", controller, PROCEDURAL_2D_PLAY_LAYOUT, controller.mainMode);
	app.addMode(controller.generateMode);
	app.panelTabs = [
		{ id: PROCEDURAL_2D_PLAY_HIERARCHY_TAB_ID, iconId: FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, panel: "workbench", order: 0, bodyKey: PROCEDURAL_2D_PLAY_HIERARCHY_BODY_KEY, label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL },
		{ id: PROCEDURAL_2D_PLAY_CATALOGUE_TAB_ID, iconId: FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, panel: "workbench", order: 1, bodyKey: PROCEDURAL_2D_PLAY_CATALOGUE_BODY_KEY, label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL },
		{ id: PROCEDURAL_2D_PLAY_INSPECTION_TAB_ID, iconId: FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, panel: "details", order: 0, bodyKey: PROCEDURAL_2D_PLAY_INSPECTION_BODY_KEY, label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL },
	] satisfies SideTabSpec[];
	return app;
}

//#region 🔖Play

const proceduralFixtureModules = eagerPlayExampleGlob("../../example/*.procedural2d.json");

function proceduralFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.procedural2d\.json$/, "");
}

function proceduralFixtureLabelFromId(id: string): string {
	return id
		.split("-")
		.filter(Boolean)
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

const PROCEDURAL_2D_PLAY_FILE_EXAMPLE_JSON_BY_ID: Record<string, string> = Object.fromEntries(
	Object.entries(proceduralFixtureModules).map(([path, mod]) => {
		const id = proceduralFixtureIdFromGlobPath(path);
		const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
		return [id, json];
	}),
);

export const PROCEDURAL_2D_PLAY_EMPTY_FIXTURE: FlowFixture = {
	schema: "flow.fixture",
	camera: { x: 0, y: 0, zoom: 1 },
	widgets: [],
	synapses: [],
};

export const PROCEDURAL_2D_PLAY_EMPTY_FIXTURE_JSON = proceduralFixtureToJson(PROCEDURAL_2D_PLAY_EMPTY_FIXTURE);

export const PROCEDURAL_2D_PLAY_EXAMPLE_OPTIONS: ReadonlyArray<{ readonly id: string; readonly label: string }> = [
	{ id: PROCEDURAL_2D_PLAY_EXAMPLE_DEFAULT_ID, label: "Draw rect + fill" },
	...Object.keys(PROCEDURAL_2D_PLAY_FILE_EXAMPLE_JSON_BY_ID)
		.sort()
		.map((id) => ({ id, label: proceduralFixtureLabelFromId(id) })),
];


function proceduralFixtureJsonForId(fixtureId: string): string {
	if (isPlaygroundNoExampleId(fixtureId)) {
		return proceduralFixtureToJson(PROCEDURAL_2D_PLAY_EMPTY_FIXTURE);
	}
	if (fixtureId === PROCEDURAL_2D_PLAY_EXAMPLE_DEFAULT_ID) {
		return PROCEDURAL_2D_PLAY_DEFAULT_FIXTURE_JSON;
	}
	const fileJson = PROCEDURAL_2D_PLAY_FILE_EXAMPLE_JSON_BY_ID[fixtureId];
	if (fileJson) return fileJson;
	return PROCEDURAL_2D_PLAY_EMPTY_FIXTURE_JSON;
}

/** @emoji 🧪 Resolves procedural play fixture JSON by catalog id. */
export function procedural2dPlayFixtureJson(fixtureId: string = PROCEDURAL_2D_PLAY_EXAMPLE_DEFAULT_ID): string {
	return proceduralFixtureJsonForId(fixtureId);
}



/** @emoji 🛝 Procedural playground app. */


export const procedural2dPlayAppDefinition = createPlaygroundApp({
	id: PROCEDURAL_2D_PLAY_APP_ID,
	label: "Procedural 2D",
	controllerId: PROCEDURAL_2D_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "procedural-2d",
		resolveDedupe: ["react", "react-dom", "scheduler", "@semio-tech/flow-react", "@semio-tech/procedural-2d-react"],
		watchIgnored: ["../../../flow/core/lib.rs",
		"../../../flow/core/target/**",
		"../../../flow/module/**/lib.rs",
		"../../../flow/module/**/target/**",],
		optimizeDeps: { include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "@semio-tech/infinite-cavas-react-renderer"] },
	},
	runtimeBootstrap: {
		createController: (bus, notify) => new Procedural2dPlayController(bus, notify),
		buildAppRuntime: buildProcedural2dPlayAppRuntime,
	},
	keybindings: [
		{ key: "ctrl+a,meta+a", controllerId: PROCEDURAL_2D_PLAY_CONTROLLER_ID, command: "selectAll" },
		{ key: "Delete", controllerId: PROCEDURAL_2D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
		{ key: "Backspace", controllerId: PROCEDURAL_2D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
	],
});
//#endregion 🔖Play

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@semio-tech/procedural-2d-play", () => {
		it("exports default fixture json", () => {
			expect(PROCEDURAL_2D_PLAY_DEFAULT_FIXTURE_JSON).toContain("flow.fixture");
		});

		it("starts with the default draw fixture selected", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			expect(ctrl.getExampleCatalog().activeExampleId).toBe(PROCEDURAL_2D_PLAY_EXAMPLE_DEFAULT_ID);
			expect(ctrl.getFixtureJson()).toContain("draw.shape.rect");
		});

		it("does not auto-load stored fixture on startup", () => {
			const backing = new Map<string, string>();
			const store = createProcedural2dPlayFixtureStore({
				getItem: (k) => backing.get(k) ?? null,
				setItem: (k, v) => {
					backing.set(k, v);
				},
				removeItem: (k) => {
					backing.delete(k);
				},
			});
			store.save('{"schema":"flow.fixture","camera":{"x":0,"y":0,"zoom":1},"widgets":[{"kind":"neuron","id":"stored","neuronKind":"draw.shape.circle"}],"synapses":[]}');
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {}, store);
			expect(ctrl.getFixtureJson()).toContain("draw.shape.rect");
			expect(ctrl.getFixtureJson()).not.toContain("draw.shape.circle");
		});

		it("controller stores fixture json", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			ctrl.run("setFixtureJson", { json: '{"schema":"flow.fixture"}' });
			expect(ctrl.getFixtureJson()).toContain("flow.fixture");
		});

		it("kinds tree marks nested catalogue rows draggable", () => {
			const tree = buildProcedural2dPlayKindsTree([
				{
					id: "brep",
					title: "Brep",
					items: [],
					groups: [
						{
							id: "brep.primitives-3d",
							title: "Primitives 3D",
							items: [{ kind: "neuron", neuronKind: "brep.prim3d.box", name: "Box", abbreviation: "Box", icon: "emoji:📦", summary: "Axis-aligned box" }],
						},
					],
				},
			]);
			expect(tree.type).toBe("tree");
			const leaf = tree.sections?.[0]?.items?.[0]?.items?.[0];
			expect(leaf?.draggable).toBe(true);
			expect(leaf?.dragData).toBeDefined();
		});

		it("catalogue snapshot listeners fire when sections arrive", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			let revision = ctrl.getCatalogueRevision();
			const unsubscribe = ctrl.subscribeSnapshot(() => {
				revision = ctrl.getCatalogueRevision();
			});
			ctrl.run("setCatalogueSections", { sections: [{ id: "brep", title: "Brep", items: [] }] });
			unsubscribe();
			expect(revision).toBe(1);
		});

		it("catalogue revision bumps when sections arrive", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			expect(ctrl.getCatalogueRevision()).toBe(0);
			ctrl.run("setCatalogueSections", {
				sections: [
					{
						id: "brep",
						title: "Brep",
						items: [],
						groups: [
							{
								id: "brep.primitives-3d",
								title: "Primitives 3D",
								items: [{ kind: "neuron", neuronKind: "brep.prim3d.box", name: "Box", abbreviation: "Box", icon: "emoji:📦", summary: "Box" }],
							},
							{
								id: "brep.curves",
								title: "Curves",
								items: [{ kind: "neuron", neuronKind: "brep.curve.line", name: "Line", abbreviation: "Line", icon: "emoji:〰️", summary: "Line edge" }],
							},
						],
					},
				],
			});
			expect(ctrl.getCatalogueRevision()).toBe(1);
		});

		it("catalogue revision bumps for nested brep groups", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			ctrl.run("setCatalogueSections", {
				sections: [
					{
						id: "brep",
						title: "Brep",
						items: [],
						groups: [
							{ id: "brep.primitives-3d", title: "Primitives 3D", items: [] },
							{ id: "brep.solid", title: "Solid", items: [] },
						],
					},
				],
			});
			expect(ctrl.getCatalogueSections()[0]?.groups?.length).toBe(2);
		});

		it("flow window has no shell measures", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			expect(ctrl.mainMode.windowKinds[0]?.measures ?? []).toEqual([]);
		});

		it("controller exposes flow and preview window kinds", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			expect(ctrl.mainMode.windowKinds).toHaveLength(2);
			expect(ctrl.mainMode.windowKinds[1]?.id).toBe(PROCEDURAL_2D_PLAY_WINDOW_KIND_PREVIEW);
		});

		it("setShowMode updates preview filter", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			expect(ctrl.getShowMode()).toBe("everything");
			ctrl.run("setShowMode", { id: "selected" });
			expect(ctrl.getShowMode()).toBe("selected");
		});

		it("setShowMode accepts shell measure value", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			ctrl.run("setShowMode", { value: "selected" });
			expect(ctrl.getShowMode()).toBe("selected");
			ctrl.run("setShowMode", { value: "everything" });
			expect(ctrl.getShowMode()).toBe("everything");
		});

		it("canvasCommand bumps command request epoch", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			ctrl.run("canvasCommand", { command: "deleteSelection" });
			expect(ctrl.getCommandRequest().command).toBe("deleteSelection");
			expect(ctrl.getCommandRequest().epoch).toBe(1);
		});

		it("deleteSelection forwards to flow canvas command request", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			ctrl.run("setSelection", { ids: ["node-a"] });
			ctrl.run("deleteSelection");
			expect(ctrl.getCommandRequest().command).toBe("deleteSelection");
			expect(ctrl.getSelectedNodeIds()).toEqual(["node-a"]);
		});

		it("setPreviewOff stores preview-off node ids", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			ctrl.run("setPreviewOff", { ids: ["a", "b"] });
			expect(ctrl.getPreviewOffNodeIds()).toEqual(["a", "b"]);
		});

		it("buildProcedural2dPlayCanvasContextMenu adds isolate in preview for hovered node", () => {
			const items = buildProcedural2dPlayCanvasContextMenu(
				{
					hoveredNodeId: "box",
					selectedNodeIds: ["box"],
					clusterNodeIds: [],
					isImageWidget: false,
					isBackground: false,
					previewOffNodeIds: [],
					screen: { x: 0, y: 0 },
					world: { x: 0, y: 0 },
					clientX: 0,
					clientY: 0,
				},
				() => {},
			);
			expect(items.some((item) => item.id === "procedural2d.ctx.isolatePreview")).toBe(true);
		});

		it("setFixtureJson sync preserves preview items after flow interaction", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			ctrl.run("setEvalOutputs", {
				outputsJson: JSON.stringify({
					rect: { in: {}, out: { "draw.drawing": { $schema: "draw.drawing", handle: "drawing-1", kind: "rect" } } },
				}),
			});
			const base = ctrl.getFixtureJson();
			const interacted = JSON.stringify({
				...JSON.parse(base),
				camera: { x: 12, y: -4, zoom: 2.5 },
				widgets: [
					{ kind: "neuron", id: "sketch", neuronKind: "brep.sketch2d.rectangle" },
					{ kind: "neuron", id: "solid", neuronKind: "brep.solid.extrude" },
					{ kind: "outputPreview", id: "preview", preview: { geometry: "solid-9" } },
				],
			});
			ctrl.run("setFixtureJson", { json: interacted });
			expect(ctrl.getPreviewItems()).toEqual([
				{ widgetId: "rect", port: "draw.drawing", direction: "out", kind: "drawing", handle: "drawing-1" },
			]);
		});

		it("setFixtureJson with resetInteraction clears preview items", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			ctrl.run("setEvalOutputs", {
				outputsJson: JSON.stringify({
					preview: { out: { "": { $schema: "draw.drawing", handle: "drawing-1", kind: "rect" } } },
				}),
			});
			ctrl.run("setFixtureJson", {
				json: '{"schema":"flow.fixture","camera":{"x":0,"y":0,"zoom":1},"widgets":[],"synapses":[]}',
				resetInteraction: true,
			});
			expect(ctrl.getPreviewItems()).toEqual([]);
		});

		it("setEvalOutputs stores preview items per widget", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			ctrl.run("setEvalOutputs", {
				outputsJson: JSON.stringify({
					preview: { out: { "": { $schema: "draw.drawing", handle: "drawing-1", kind: "rect" } } },
				}),
			});
			expect(ctrl.getPreviewItems()).toEqual([
				{ widgetId: "preview", port: "", direction: "out", kind: "drawing", handle: "drawing-1" },
			]);
		});

		it("setEvalOutputs merges worker drawing scenes into preview items", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			const scene = { width: 10, height: 20, nodes: [] };
			ctrl.run("setEvalOutputs", {
				outputsJson: JSON.stringify({
					preview: { out: { "": { $schema: "draw.drawing", handle: "drawing-1", kind: "rect" } } },
				}),
				previewMeshes: { "drawing-1": scene },
			});
			expect(ctrl.getPreviewItems()[0]).toMatchObject({ handle: "drawing-1", scene });
		});

		it("setSelectChannels stores channel selection and parent nodes", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			ctrl.run("setSelectChannels", {
				channels: [{ widgetId: "box", port: "solid", direction: "out" }],
			});
			expect(ctrl.getSelectedChannels()).toEqual([{ widgetId: "box", port: "solid", direction: "out" }]);
			expect(ctrl.getSelectedNodeIds()).toEqual(["box"]);
		});

		it("setSelection and setHover update interaction revision", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			ctrl.run("setSelection", { ids: ["box"] });
			ctrl.run("setHover", { id: "box" });
			expect(ctrl.getSelectedNodeIds()).toEqual(["box"]);
			expect(ctrl.getHoveredNodeId()).toBe("box");
			expect(ctrl.getInteractionRevision()).toBeGreaterThan(0);
		});

		it("setHover stores hovered channel", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			ctrl.run("setHover", { id: "offset", channel: { widgetId: "offset", port: "geometry", direction: "in" } });
			expect(ctrl.getHoveredChannel()).toEqual({ widgetId: "offset", port: "geometry", direction: "in" });
		});

		it("setSelection merges additively when mode is additive", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			ctrl.run("setSelection", { ids: ["a"], mode: "default" });
			ctrl.run("setSelection", { ids: ["b"], mode: "additive" });
			expect(ctrl.getSelectedNodeIds()).toEqual(["a", "b"]);
		});

		it("setSelectionMethod updates marquee method", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			ctrl.run("setSelectionMethod", { method: "lasso" });
			expect(ctrl.getSelectionMethod()).toBe("lasso");
		});

		it("buildProcedural2dPlayToolbarTools registers selection, save, view, and actions", () => {
			const tools = buildProcedural2dPlayToolbarTools(
				{
					selectionMethod: "rectangle",
					selectionMode: "default",
					showMode: "everything",
					selectionCount: 0,
					hasStoredFixture: false,
				},
				PROCEDURAL_2D_PLAY_CONTROLLER_ID,
			);
			expect(tools.selection?.some((row) => row.id === "procedural2d.select.rectangle")).toBe(true);
			expect(tools.save?.map((row) => row.id)).toEqual([
				"procedural2d.save.stored",
				"procedural2d.save.download",
				"procedural2d.save.load",
				"procedural2d.save.loadStored",
				"procedural2d.save.reset",
			]);
			expect(tools.save?.[3]?.disabled).toBe(true);
			expect(tools.view?.length).toBe(2);
			expect(tools.actions?.some((row) => row.id === "procedural2d.action.reorganize")).toBe(true);
			expect(tools.export?.map((row) => row.id)).toEqual([
				"procedural2d.export.svg",
				"procedural2d.export.pdf",
				"procedural2d.export.png",
			]);
		});

		it("controller exposes toolbar tools when host bridge is attached", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			expect(ctrl.mainMode.tools).toBeUndefined();
			ctrl.setHostBridge({
				getToolbarState: () => ({
					selectionMethod: "rectangle",
					selectionMode: "default",
					showMode: "everything",
					selectionCount: 0,
					hasStoredFixture: false,
				}),
				runHostCommand: () => {},
			});
			expect(ctrl.mainMode.tools?.find((node) => node.kind === "collection" && node.id === "selection")?.kind === "collection").toBe(true);
		});

		it("fixture store round-trips json", () => {
			const backing = new Map<string, string>();
			const store = createProcedural2dPlayFixtureStore({
				getItem: (k) => backing.get(k) ?? null,
				setItem: (k, v) => {
					backing.set(k, v);
				},
				removeItem: (k) => {
					backing.delete(k);
				},
			});
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {}, store);
			ctrl.run("saveStored");
			expect(ctrl.hasStoredFixture()).toBe(true);
			ctrl.run("setFixtureJson", { json: '{"schema":"flow.fixture","widgets":[],"synapses":[]}' });
			ctrl.run("loadStored");
			expect(ctrl.getFixtureJson()).toContain("flow.fixture");
		});

		it("setActiveExample loads default and empty fixtures", () => {
			const bus = new CommandBus();
			const ctrl = new Procedural2dPlayController(bus, () => {});
			ctrl.run("setActiveExample", { exampleId: PLAYGROUND_NO_EXAMPLE_ID });
			expect(ctrl.getFixtureJson()).toContain('"widgets":[]');
			ctrl.run("setActiveExample", { exampleId: PROCEDURAL_2D_PLAY_EXAMPLE_DEFAULT_ID });
			expect(ctrl.getFixtureJson()).toContain("draw.shape.rect");
		});

		it("extensions tree lists installed modules", () => {
			const tree = buildProcedural2dPlayExtensionsTree([
				{
					id: "brep",
					active: true,
					manifest: {
						schema: "flow.module",
						id: "brep",
						name: "Brep",
						version: "0.1.0",
						activationEvents: ["onStartup"],
						contributes: {
							neuronKinds: [{ id: "brep.prim3d.box", module: "brep", name: "Box", abbreviation: "Box", icon: "emoji:📦", summary: "Box", inputs: [], outputs: ["geometry"] }],
							widgets: [],
							commands: [],
							settings: [],
						},
					},
				},
			]);
			const labels = tree.sections?.flatMap((section) => section.items?.map((item) => item.label) ?? []) ?? [];
			expect(labels).toContain("Brep");
		});

	});
}

//#region 🔖MediaExport
async function evaluateProcedural2dDrawingScene(fixture: FlowFixture): Promise<DrawingScene | null> {
	const client = new FlowOrchestratorClient();
	await client.loadFixtureJson(proceduralFixtureToJson(fixture));
	const result = await client.evaluate();
	const previewMeshes = await client.tessellatePreviews(result.outputsJson);
	for (const item of extractChannelPreviewItems(result.outputsJson)) {
		if (item.kind !== "drawing" || item.direction !== "out") continue;
		const scene = drawingSceneFromPreviewPayload(previewMeshes[item.handle]);
		if (scene) return scene;
	}
	return null;
}

/** @emoji 💾 Registers procedural 2d flow fixture SVG/PNG export handlers for the OS media graph. */
export function registerProcedural2dMediaExportHandlers(): void {
	registerOsMediaExportHandler("2d.procedural", "svg", async (doc) => {
		const scene = await evaluateProcedural2dDrawingScene(doc as FlowFixture);
		const svg = scene ? drawingSceneToSvgMarkup(scene) : `<svg xmlns="http://www.w3.org/2000/svg" width="1024" height="1024"/>`;
		return { data: svg, mimeType: "image/svg+xml", fileName: "procedural2d.svg" };
	});
	registerOsMediaExportHandler("2d.procedural", "png", async (doc) => {
		const scene = await evaluateProcedural2dDrawingScene(doc as FlowFixture);
		const width = scene?.width ?? 1024;
		const height = scene?.height ?? 1024;
		const svg = scene ? drawingSceneToSvgMarkup(scene) : `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}"/>`;
		const dataUrl = await rasterizeSvgMarkupToPngDataUrl(svg, width, height);
		const blob = await fetch(dataUrl).then((response) => response.blob());
		return { data: new Uint8Array(await blob.arrayBuffer()), mimeType: "image/png", fileName: "procedural2d.png" };
	});
}
//#endregion 🔖MediaExport

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for procedural 2d. */
export function buildProcedural2dProgramDefinition(): PlatformDefinition {
	return {
		id: "procedural.2d",
		name: "Procedural 2D",
		apiVersion: "1",
		apps: [{ id: "procedural2d", label: "Procedural 2D", controllerId: PROCEDURAL_2D_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🔖OsProgram
import { mergeOsProgramDefinition, osBaselineResource, registerAppVcsHandler } from "@semio-tech/framework-os-core";
import type { OsProgramContribution } from "@semio-tech/framework-platform-core";

const procedural2dProgramContributionResources = {
		"procedural2d": osBaselineResource("2d.procedural", "procedural.2d", "puzzle2d"),
	};

/** @emoji 🧩 OS program contribution for procedural.2d. */
export const procedural2dProgramContribution: OsProgramContribution = {
	programId: "procedural.2d",
	register() {
		mergeOsProgramDefinition("procedural.2d", buildProcedural2dProgramDefinition(), procedural2dProgramContributionResources);
		registerProcedural2dMediaExportHandlers();
		registerAppVcsHandler(createProcedural2dAppVcsHandler());
	},
};
//#endregion 🔖OsProgram
//#region 🔖DocumentVcs
import { createTypedAppVcsHandler } from "@semio-tech/framework-os-core";

/** @emoji 📏 S app VCS handler for procedural 2d documents. */
export function createProcedural2dAppVcsHandler() {
	type Doc = { readonly revision: number };
	type Op = { readonly op: "setRevision"; readonly revision: number };
	return createTypedAppVcsHandler<Doc, Op>("procedural.2d", "procedural.2d", () => ({ revision: 0 }), (doc, op) => ({ revision: op.revision }));
}
//#endregion 🔖DocumentVcs
