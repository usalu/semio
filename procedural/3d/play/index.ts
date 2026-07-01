// #region 🧲Header
/** @emoji 🔧 Procedural play harness on `@semio-tech/framework-playground-core`. */
// #endregion 🧲Header

import {
    buildFlowPlayCatalogueTree,
    buildFlowPlayHierarchyTree,
    buildFlowPlayInspectorTree,
    createDefaultGenerations,
    parseFlowPlayFixtureJson,
    runGenerationCommand,
} from "@semio-tech/flow-play";
import { flowFixtureToFormSpec, type FlowGeneration } from "@semio-tech/forms-react";
import { FlowOrchestratorClient } from "../../../flow/worker-client.ts";
import {
    buildCatalogueKindsTreeSections,
    buildFlowContextMenuItems,
    DAG_LOD_MODE_AUTOMATIC,
    dagLodAutomaticSelectLabel,
    dagPlayLodTierMenuLabel,
    dagPlayLodTiers,
    FLOW_DEFAULT_PROXIMITY_DISTANCE,
    flowPlayCatalogueItemDragData,
    flowSensibleSliderRange,
    isDagDrawLodKind,
    type CatalogueSection,
    type DagDrawLodKind,
    type DagLodModeKind,
    type FlowCanvasCommandRequest,
    type FlowCanvasContextMenuContext,
    type FlowContextMenuDispatch,
    type FlowExtensionEntry,
    type FlowGraphEditOp,
    type FlowReorganizeRequest,
} from "@semio-tech/flow-react";
import type { WindowMeasure } from "@semio-tech/framework-playground-core";
import {
	DocumentVcsStore,
	applyJsonReplaceOp,
	createDocumentVcsEnvelope,
	recordJsonProjectionChange,
	type JsonReplaceOp,
} from "@semio-tech/framework-core";
import {
    AppRuntime,
    buildFlowWindowBody,
    buildFormsWindowBody,
    buildPuzzle3dWindowBody,
    CommandBus,
    Controller,
    createDefaultLayout,
    createPlayAppRuntime,
    createProductPlaygroundPlatform,
    enforcePlaygroundWindowEngagementInput,
    isPlaygroundFixtureLocked,
    isPlaygroundNoFixtureId,
    ModeRuntime,
    Platform,
    Playground,
    PLAYGROUND_NO_FIXTURE_ID,
    playgroundResolvedFixtureId,
    registerWindowBody,
    WindowKindRuntime,
    type AppTools,
    type CommandDescriptor,
    type PlaygroundFixtureCatalog,
    type PlaygroundFixtureHost,
    type ToolLeaf,
    toolCollection,
    type UiNode,
    type UiTreeSectionNode,
    type WindowBodyViewContext,
    type WindowEngagement,
} from "@semio-tech/framework-playground-core";
import { meshTransferFromPreviewPayload } from "@semio-tech/geometry-brep-js";
import {
    extractChannelPreviewItems,
    filterVisiblePreviewItems,
    PROCEDURAL_DEFAULT_FIXTURE,
    proceduralExtensionHost,
    proceduralFixtureToJson,
    resolveGeometryTargets,
    type FlowFixtureV1,
    type ProceduralChannelRef,
    type ProceduralFixtureEdge,
    type ProceduralGumballTransformDelta,
    type ProceduralGumballTransformOp,
    type ProceduralGumballTransformPhase,
    type ProceduralGumballTransformRequest,
    type ProceduralPreviewItem,
    type ProceduralPreviewShowMode,
    type ProceduralTransformGranularity,
} from "@semio-tech/procedural-3d-react";
import type { ContextMenuItem } from "@semio-tech/ui-react";
import { bootstrapElementsSurfaceChromeDocument, selectionMergeIds, type SelectionMergeMode } from "@semio-tech/ui-react";

function previewItemKey(item: ProceduralPreviewItem): string {
	return `${item.widgetId}:${item.port}:${item.direction}`;
}

function previewItemsWithMeshes(
	items: ProceduralPreviewItem[],
	previewMeshes?: Readonly<Record<string, unknown>>,
	previous: readonly ProceduralPreviewItem[] = [],
): ProceduralPreviewItem[] {
	const previousByKey = new Map(previous.map((item) => [previewItemKey(item), item]));
	return items.map((item) => {
		if (item.kind !== "geometry" || item.direction !== "out") return item;
		const previousItem = previousByKey.get(previewItemKey(item));
		const mesh =
			meshTransferFromPreviewPayload(previewMeshes?.[item.handle]) ??
			(previousItem?.handle === item.handle ? previousItem.mesh : undefined);
		return mesh ? { ...item, mesh } : item;
	});
}

export const PROCEDURAL_3D_PLAY_APP_ID = "procedural-3d-play";
export const PROCEDURAL_3D_PLAY_CONTROLLER_ID = "procedural-3d-play";
export const PROCEDURAL_PLAY_SURFACE_ID = "procedural.play/v1";
export const PROCEDURAL_PLAY_BODY_KEY_MAIN = "procedural.play.main";
export const PROCEDURAL_PLAY_WINDOW_KIND_ID = "procedural-main";
export const PROCEDURAL_PLAY_WINDOW_KIND_PREVIEW = "procedural-preview";
export const PROCEDURAL_PLAY_BODY_KEY_PREVIEW = "procedural.play.preview";
export const PROCEDURAL_PLAY_BODY_KEY_GENERATE = "procedural.play.generate";
export const PROCEDURAL_PLAY_SURFACE_ID_PREVIEW = "procedural.play.preview/v1";
export const PROCEDURAL_PLAY_SURFACE_ID_GENERATE = "procedural.play.generate/v1";

export const PROCEDURAL_PLAY_DEFAULT_FIXTURE: FlowFixtureV1 = PROCEDURAL_DEFAULT_FIXTURE;
export const PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON = proceduralFixtureToJson(PROCEDURAL_DEFAULT_FIXTURE);
export const PROCEDURAL_PLAY_LAYOUT = createDefaultLayout(
	[PROCEDURAL_PLAY_WINDOW_KIND_ID, PROCEDURAL_PLAY_WINDOW_KIND_PREVIEW],
	"row",
	[55, 45],
	["Flow", "Preview"],
);
export const PROCEDURAL_PLAY_KINDS_TAB_ID = "procedural-play-kinds";
export const PROCEDURAL_PLAY_EXTENSIONS_TAB_ID = "procedural-play-extensions";
export const PROCEDURAL_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const PROCEDURAL_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const PROCEDURAL_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";
export const PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID = "procedural-default";

import {
    PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID,
    resolveProceduralPlayFixtureSlug,
} from "./fixture-slugs.js";

export { PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID, resolveProceduralPlayFixtureSlug };

const proceduralFixtureModules = import.meta.glob("../fixture/*.procedural.json", { eager: true }) as Record<
	string,
	{ default: unknown }
>;

function proceduralFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.procedural\.json$/, "");
}

function proceduralFixtureLabelFromId(id: string): string {
	return id
		.split("-")
		.filter(Boolean)
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

const PROCEDURAL_PLAY_FILE_FIXTURE_JSON_BY_ID: Record<string, string> = Object.fromEntries(
	Object.entries(proceduralFixtureModules).map(([path, mod]) => {
		const id = proceduralFixtureIdFromGlobPath(path);
		const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
		return [id, json];
	}),
);

export const PROCEDURAL_PLAY_EMPTY_FIXTURE: FlowFixtureV1 = {
	schema: "flow.fixture/v1",
	camera: { x: 0, y: 0, zoom: 1 },
	widgets: [],
	synapses: [],
};

export const PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON = proceduralFixtureToJson(PROCEDURAL_PLAY_EMPTY_FIXTURE);

export const PROCEDURAL_PLAY_FIXTURE_OPTIONS: ReadonlyArray<{ readonly id: string; readonly label: string }> = [
	{ id: PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID, label: "Box fillet move" },
	...Object.keys(PROCEDURAL_PLAY_FILE_FIXTURE_JSON_BY_ID)
		.sort()
		.map((id) => ({ id, label: proceduralFixtureLabelFromId(id) })),
];

const PROCEDURAL_PLAY_STORE_KEY = "procedural.fixture/v1";

/** @emoji 💾 Local persistence for procedural flow fixtures. */
export interface ProceduralPlayFixtureStore {
	load(): string | null;
	save(fixtureJson: string): void;
	clear(): void;
}

export function createProceduralPlayFixtureStore(storage?: Pick<Storage, "getItem" | "setItem" | "removeItem">): ProceduralPlayFixtureStore {
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
			return resolved.getItem(PROCEDURAL_PLAY_STORE_KEY);
		},
		save(fixtureJson: string): void {
			resolved.setItem(PROCEDURAL_PLAY_STORE_KEY, fixtureJson);
		},
		clear(): void {
			resolved.removeItem(PROCEDURAL_PLAY_STORE_KEY);
		},
	};
}

export type ProceduralLayoutOrientation = "leftRight" | "topBottom";
export type ProceduralPlaySelectionMode = SelectionMergeMode;
export type ProceduralPlaySelectionMethod = "rectangle" | "lasso";

const DEFAULT_LAYER_SPACING = 120;
const DEFAULT_SIBLING_GAP = 40;

export type {
    ProceduralGumballTransformDelta,
    ProceduralGumballTransformOp,
    ProceduralGumballTransformPhase,
    ProceduralGumballTransformRequest,
    ProceduralTransformGranularity
} from "@semio-tech/procedural-3d-react";

interface GumballTransformBinding {
	readonly sourceWidgetId: string;
	readonly transformId: string;
	readonly op: ProceduralGumballTransformOp;
	readonly granularity: ProceduralTransformGranularity;
	readonly valueWidgetIds: string[];
	readonly vectorId?: string;
	readonly values: { offset: [number, number, number]; angle: number; factor: number };
}

interface GumballDragSession {
	readonly binding: GumballTransformBinding;
	readonly baseValues: { offset: [number, number, number]; angle: number; factor: number };
}

const BREP_XFORM_NEURON_KIND: Record<ProceduralGumballTransformOp, string> = {
	translate: "brep.xform.translate",
	rotate: "brep.xform.rotate",
	scale: "brep.xform.scale",
};

const GUMBALL_SLIDER_HALF_WIDTH = 42;
const GUMBALL_NEURON_HALF_WIDTH = 48;
const GUMBALL_VECTOR_HALF_WIDTH = 52;
const GUMBALL_SOURCE_HALF_WIDTH = 48;

function gumballColumnEdgeGap(layerSpacing: number, siblingGap: number): number {
	return Math.max(siblingGap, layerSpacing * 0.2, 28);
}

function gumballColumnAfter(prevCenterX: number, prevHalfWidth: number, nextHalfWidth: number, edgeGap: number): number {
	return prevCenterX + prevHalfWidth + edgeGap + nextHalfWidth;
}

function gumballValueRowGap(siblingGap: number): number {
	return Math.max(siblingGap, 32);
}

function gumballMakeSpaceDx(transformColumnX: number, transformHalfWidth: number, sourceX: number, edgeGap: number): number {
	return transformColumnX + transformHalfWidth + edgeGap - sourceX;
}

function widgetLayoutFromFixture(fixtureJson: string, widgetId: string): { x: number; y: number } {
	try {
		const fixture = JSON.parse(fixtureJson) as FlowFixtureV1;
		return fixture.layout?.[widgetId] ?? { x: 0, y: 0 };
	} catch {
		return { x: 0, y: 0 };
	}
}

function gumballZeroDelta(op: ProceduralGumballTransformOp): ProceduralGumballTransformDelta {
	if (op === "translate") return { op: "translate", offset: [0, 0, 0] };
	if (op === "rotate") return { op: "rotate", angle: 0 };
	return { op: "scale", factor: 1 };
}

function copyGumballValues(binding: GumballTransformBinding): GumballDragSession["baseValues"] {
	return {
		offset: [binding.values.offset[0], binding.values.offset[1], binding.values.offset[2]],
		angle: binding.values.angle,
		factor: binding.values.factor,
	};
}

function setGumballBindingValues(binding: GumballTransformBinding, values: GumballDragSession["baseValues"]): void {
	binding.values.offset = [values.offset[0], values.offset[1], values.offset[2]];
	binding.values.angle = values.angle;
	binding.values.factor = values.factor;
}

function applyGumballDeltaToBase(
	base: GumballDragSession["baseValues"],
	op: ProceduralGumballTransformOp,
	delta: ProceduralGumballTransformDelta,
): GumballDragSession["baseValues"] {
	if (op === "translate" && delta.op === "translate") {
		return {
			offset: [base.offset[0] + delta.offset[0], base.offset[1] + delta.offset[1], base.offset[2] + delta.offset[2]],
			angle: base.angle,
			factor: base.factor,
		};
	}
	if (op === "rotate" && delta.op === "rotate") {
		return { offset: base.offset, angle: base.angle + delta.angle, factor: base.factor };
	}
	if (op === "scale" && delta.op === "scale") {
		return { offset: base.offset, angle: base.angle, factor: base.factor * delta.factor };
	}
	return base;
}

function gumballBindingNodeIds(binding: GumballTransformBinding): string[] {
	return [...binding.valueWidgetIds, ...(binding.vectorId ? [binding.vectorId] : []), binding.transformId];
}

function accumulateGumballDelta(binding: GumballTransformBinding, delta: ProceduralGumballTransformDelta): void {
	if (delta.op === "translate" && binding.op === "translate") {
		binding.values.offset = [
			binding.values.offset[0] + delta.offset[0],
			binding.values.offset[1] + delta.offset[1],
			binding.values.offset[2] + delta.offset[2],
		];
		return;
	}
	if (delta.op === "rotate" && binding.op === "rotate") {
		binding.values.angle += delta.angle;
		return;
	}
	if (delta.op === "scale" && binding.op === "scale") {
		binding.values.factor *= delta.factor;
	}
}

function compactNeuronParams(binding: GumballTransformBinding): Record<string, unknown> {
	if (binding.op === "translate") {
		const [x, y, z] = binding.values.offset;
		return { offset: [x, y, z] };
	}
	if (binding.op === "rotate") {
		return { angle: binding.values.angle };
	}
	return { factor: binding.values.factor };
}

function sliderDescriptor(id: string, value: number): string {
	const { min, max, step } = flowSensibleSliderRange(value);
	return JSON.stringify({ kind: "inputSlider", id, value, min, max, step });
}

function neuronDescriptor(id: string, neuronKind: string): string {
	return JSON.stringify({ kind: "neuron", id, neuronKind });
}

function proceduralPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command, args };
}

function buildProceduralLayoutOptionsJson(layerSpacing: number, siblingGap: number, orientation: ProceduralLayoutOrientation): string {
	return JSON.stringify({ layerSpacing, siblingGap, orientation });
}

/** @emoji 🖱️ Procedural play canvas right-click menu with preview actions. */
export function buildProceduralPlayCanvasContextMenu(ctx: FlowCanvasContextMenuContext, dispatch: FlowContextMenuDispatch): ContextMenuItem[] {
	const items = [...buildFlowContextMenuItems(ctx, dispatch)];
	if (ctx.hoveredNodeId) {
		items.splice(items.length - 1, 0, {
			id: "procedural.ctx.isolatePreview",
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
export function buildProceduralPlayExtensionsTree(entries: readonly FlowExtensionEntry[]): UiNode {
	if (!entries.length) {
		return {
			type: "tree",
			sections: [
				{
					id: "procedural-play-extensions.empty",
					label: "Extensions",
					defaultOpen: false,
					items: [{ id: "procedural-play-extensions.empty.msg", label: "Loading extensions…" }],
				},
			],
		};
	}
	const commandItems = proceduralExtensionHost.activeCommands().map((command) => ({
		id: `procedural-play-extensions.command.${command.id}`,
		label: command.title,
		description: command.id,
		command: proceduralPlayCmd("runExtensionCommand", { commandId: command.id }),
	}));
	const sections: UiTreeSectionNode[] = [
		{
			id: "procedural-play-extensions.installed",
			label: "Installed",
			defaultOpen: false,
			items: entries.map((entry) => {
				const operators = entry.manifest.contributes.operators ?? [];
				const schemas = entry.manifest.contributes.schemas ?? [];
				const commands = entry.manifest.contributes.commands ?? [];
				return {
					id: `procedural-play-extensions.${entry.id}`,
					label: entry.manifest.name,
					description: `${entry.manifest.version} · ${entry.active ? "enabled" : "disabled"} · ${operators.length} operators · ${schemas.length} schemas · ${commands.length} commands`,
					command: proceduralPlayCmd("toggleExtension", { id: entry.id, enabled: !entry.active }),
				};
			}),
		},
	];
	if (commandItems.length) {
		sections.push({
			id: "procedural-play-extensions.commands",
			label: "Commands",
			defaultOpen: false,
			items: commandItems,
		});
	}
	return { type: "tree", sections };
}

/** @emoji 🏷️ Workbench catalogue tab: module sections plus Inputs and Outputs. */
export function buildProceduralPlayKindsTree(sections: readonly CatalogueSection[]): UiNode {
	if (!sections.length) {
		return {
			type: "tree",
			sections: [
				{
					id: "procedural-play-kinds.empty",
					label: "Catalogue",
					defaultOpen: false,
					items: [{ id: "procedural-play-kinds.empty.msg", label: "Loading catalogue…" }],
				},
			],
		};
	}
	const treeSections: UiTreeSectionNode[] = buildCatalogueKindsTreeSections(sections, "procedural-play-kinds", flowPlayCatalogueItemDragData);
	return { type: "tree", sections: treeSections };
}

export function buildProceduralPlayHierarchyTree(fixtureJson: string, selectedNodeIds: readonly string[]): UiNode {
	return buildFlowPlayHierarchyTree(fixtureJson, selectedNodeIds, PROCEDURAL_3D_PLAY_CONTROLLER_ID);
}

export function buildProceduralPlayCatalogueTree(sections: readonly CatalogueSection[], extensionEntries: readonly FlowExtensionEntry[]): UiNode {
	return buildFlowPlayCatalogueTree(sections, extensionEntries);
}

export function buildProceduralPlayInspectorTree(fixtureJson: string, selectedNodeIds: readonly string[]): UiNode {
	return buildFlowPlayInspectorTree(fixtureJson, selectedNodeIds, PROCEDURAL_3D_PLAY_CONTROLLER_ID);
}

/** @emoji 🧰 Snapshot read by {@link buildProceduralPlayToolbarTools}. */
export interface ProceduralPlayToolbarState {
	readonly selectionMethod: ProceduralPlaySelectionMethod;
	readonly selectionMode: ProceduralPlaySelectionMode;
	readonly showMode: ProceduralPreviewShowMode;
	readonly selectionCount: number;
	readonly hasStoredFixture: boolean;
}

/** @emoji 🔗 Host bridge for toolbar commands that need React (file picker, download). */
export interface ProceduralPlayHostBridge {
	getToolbarState(): ProceduralPlayToolbarState;
	runHostCommand(command: string, args?: unknown): void;
}

/** @emoji 🧰 Playground {@link AppTools} for procedural play (selection, save, view, actions). */
export function buildProceduralPlayToolbarTools(state: ProceduralPlayToolbarState, controllerId: string): AppTools {
	const selectionTools: ToolLeaf[] = [
		{
			id: "procedural.select.rectangle",
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
			id: "procedural.select.lasso",
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
			id: "procedural.select.mode.default",
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
			id: "procedural.select.mode.additive",
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
			id: "procedural.select.mode.subtractive",
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
			id: "procedural.select.mode.invertive",
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
			id: "procedural.selection.clear",
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
			id: "procedural.save.stored",
			kind: "button",
			iconId: "hard-drive",
			label: "Store",
			order: 0,
			controllerId,
			command: "saveStored",
		},
		{
			id: "procedural.save.download",
			kind: "button",
			iconId: "save",
			label: "Download",
			order: 1,
			controllerId,
			command: "saveDownload",
		},
		{
			id: "procedural.save.load",
			kind: "button",
			iconId: "folder-open",
			label: "Load",
			order: 2,
			controllerId,
			command: "loadRequest",
		},
		{
			id: "procedural.save.loadStored",
			kind: "button",
			iconId: "rotate-ccw",
			label: "Restore",
			order: 3,
			disabled: !state.hasStoredFixture,
			controllerId,
			command: "loadStored",
		},
		{
			id: "procedural.save.reset",
			kind: "button",
			iconId: "refresh-cw",
			label: "Reset",
			order: 4,
			controllerId,
			command: "resetFixture",
		},
	];
	return [
		toolCollection("selection", "mouse-pointer-2", selectionTools),
		toolCollection("save", "save", saveTools),
		toolCollection("view", "layout-grid", [
			{
				id: "procedural.view.everything",
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
				id: "procedural.view.selected",
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
		toolCollection("actions", "more-horizontal", [
			{
				id: "procedural.action.reorganize",
				kind: "button",
				iconId: "layout-grid",
				label: "Reorganize",
				order: 0,
				controllerId,
				command: "reorganize",
			},
			{
				id: "procedural.action.delete",
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

function proceduralFixtureJsonForId(fixtureId: string): string {
	if (isPlaygroundNoFixtureId(fixtureId)) {
		return proceduralFixtureToJson(PROCEDURAL_PLAY_EMPTY_FIXTURE);
	}
	if (fixtureId === PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID) {
		return PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON;
	}
	const fileJson = PROCEDURAL_PLAY_FILE_FIXTURE_JSON_BY_ID[fixtureId];
	if (fileJson) return fileJson;
	return PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON;
}

/** @emoji 🧪 Resolves procedural play fixture JSON by catalog id. */
export function proceduralPlayFixtureJson(fixtureId: string = PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID): string {
	return proceduralFixtureJsonForId(fixtureId);
}

/** @emoji 🎛 Procedural play shell controller. */
export class ProceduralPlayController extends Controller implements PlaygroundFixtureHost {
	readonly mainMode = new ModeRuntime("main", "Edit", undefined);
	readonly generateMode = new ModeRuntime("generate", "Generate", undefined);
	private activeFixtureId = playgroundResolvedFixtureId(PLAYGROUND_NO_FIXTURE_ID);
	private readonly docStore = new DocumentVcsStore<FlowFixtureV1, JsonReplaceOp<FlowFixtureV1>>({
		envelope: createDocumentVcsEnvelope(
			"flow.fixture/v1",
			"procedural-3d-play",
			parseFlowPlayFixtureJson(proceduralFixtureJsonForId(playgroundResolvedFixtureId(PLAYGROUND_NO_FIXTURE_ID))) ?? PROCEDURAL_PLAY_EMPTY_FIXTURE,
		),
		applyOp: applyJsonReplaceOp,
	});
	private generations: FlowGeneration[] = createDefaultGenerations();
	private selectedGenerationId: string | null = null;
	private generatePreviewText = "—";
	private evalClient: FlowOrchestratorClient | null = null;
	private readonly fixtureStore: ProceduralPlayFixtureStore;
	private hostBridge: ProceduralPlayHostBridge | null = null;
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
	private selectedNodeIds: string[] = [];
	private preselectNodeIds: string[] = [];
	private preselectRemovedNodeIds: string[] = [];
	private hoveredNodeId: string | null = null;
	private hoveredChannel: ProceduralChannelRef | null = null;
	private selectedChannels: ProceduralChannelRef[] = [];
	private fixtureEdges: ProceduralFixtureEdge[] = [];
	private previewOffNodeIds: string[] = [];
	private showMode: ProceduralPreviewShowMode = "everything";
	private selectionMode: ProceduralPlaySelectionMode = "default";
	private selectionMethod: ProceduralPlaySelectionMethod = "rectangle";
	private interactionRevision = 0;
	private transformGranularity: ProceduralTransformGranularity = "full";
	private gumballBindings = new Map<string, GumballTransformBinding>();
	private gumballBindingByTransformId = new Map<string, GumballTransformBinding>();
	private gumballDragSession: GumballDragSession | null = null;
	private gumballActiveWidgetIds: string[] = [];
	private lodMode: DagLodModeKind = DAG_LOD_MODE_AUTOMATIC;
	private lodModeByInstance: Record<string, DagLodModeKind> = {};
	private effectiveLod: DagDrawLodKind = "normal";
	private proximityDistance = FLOW_DEFAULT_PROXIMITY_DISTANCE;

	constructor(commandBus: CommandBus, hostNotify: () => void, fixtureStore: ProceduralPlayFixtureStore = createProceduralPlayFixtureStore()) {
		super(PROCEDURAL_3D_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.fixtureStore = fixtureStore;
		this.fixtureEdges = this.parseFixtureEdges(this.getFixtureJson());
		this.selectedGenerationId = this.generations[0]?.id ?? null;
		this.rebuildShellMode();
		this.rebuildGenerateMode();
	}

	hasStoredFixture(): boolean {
		return this.fixtureStore.load() != null;
	}

	getFixtureCatalog(): PlaygroundFixtureCatalog | null {
		if (isPlaygroundFixtureLocked()) return null;
		return { activeFixtureId: this.activeFixtureId, options: [...PROCEDURAL_PLAY_FIXTURE_OPTIONS] };
	}

	/** @emoji 🔗 Attaches the React host bridge for toolbar file IO. */
	setHostBridge(bridge: ProceduralPlayHostBridge | null): void {
		this.hostBridge = bridge;
		this.rebuildToolbarTools();
	}

	private toolbarState(): ProceduralPlayToolbarState {
		return (
			this.hostBridge?.getToolbarState() ?? {
				selectionMethod: this.selectionMethod,
				selectionMode: this.selectionMode,
				showMode: this.showMode,
				selectionCount: this.selectedNodeIds.length,
				hasStoredFixture: this.hasStoredFixture(),
			}
		);
	}

	/** @emoji 🔄 Rebuilds {@link ModeRuntime.tools} from the latest toolbar snapshot. */
	rebuildToolbarTools(): void {
		if (!this.hostBridge) {
			this.mainMode.tools = undefined;
			return;
		}
		this.mainMode.tools = buildProceduralPlayToolbarTools(this.toolbarState(), this.id);
	}

	private resetInteractionState(): void {
		this.selectedNodeIds = [];
		this.preselectNodeIds = [];
		this.preselectRemovedNodeIds = [];
		this.hoveredNodeId = null;
		this.hoveredChannel = null;
		this.selectedChannels = [];
		this.previewOffNodeIds = [];
		this.previewItems = [];
		this.gumballBindings.clear();
		this.gumballBindingByTransformId.clear();
		this.clearGumballDrag();
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

	private projection(): FlowFixtureV1 {
		return this.docStore.projection();
	}

	private commitFixture(next: FlowFixtureV1): void {
		recordJsonProjectionChange(this.docStore, next);
	}

	getDocumentVcsStore(): DocumentVcsStore<FlowFixtureV1, JsonReplaceOp<FlowFixtureV1>> {
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
		const widgets = fixture.widgets.map((widget) => (widget.id === oldId ? ({ ...widget, id: trimmed } as import("@semio-tech/flow-react").FlowWidgetV1) : widget));
		const synapses = fixture.synapses.map((synapse) => ({
			...synapse,
			from: synapse.from === oldId ? trimmed : synapse.from,
			to: synapse.to === oldId ? trimmed : synapse.to,
		}));
		this.selectedNodeIds = this.selectedNodeIds.map((id) => (id === oldId ? trimmed : id));
		this.applyFixtureJson(proceduralFixtureToJson({ ...fixture, widgets, synapses }));
	}

	private patchFlowWidget(widgetId: string, field: string, value: unknown): void {
		const fixture = this.projection();
		const widgets = fixture.widgets.map((widget) => {
			if (widget.id !== widgetId) return widget;
			if (field === "value" || field === "min" || field === "max" || field === "step") {
				const numeric = typeof value === "number" ? value : Number(value);
				if (!Number.isFinite(numeric)) return widget;
				return { ...widget, [field]: numeric } as import("@semio-tech/flow-react").FlowWidgetV1;
			}
			if (typeof value !== "string") return widget;
			return { ...widget, [field]: value } as import("@semio-tech/flow-react").FlowWidgetV1;
		});
		this.applyFixtureJson(proceduralFixtureToJson({ ...fixture, widgets }));
	}

	private loadFixtureById(fixtureId: string): void {
		const nextId = isPlaygroundNoFixtureId(fixtureId) ? PLAYGROUND_NO_FIXTURE_ID : fixtureId;
		const nextJson = proceduralFixtureJsonForId(nextId);
		if (nextId === this.activeFixtureId && nextJson === this.getFixtureJson()) return;
		this.activeFixtureId = nextId;
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
		return proceduralExtensionHost.listEntries();
	}

	getPreviewItems(): readonly ProceduralPreviewItem[] {
		return this.previewItems;
	}

	getSelectedNodeIds(): readonly string[] {
		return this.selectedNodeIds;
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
		return this.hoveredNodeId;
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
		if (this.hoveredNodeId) {
			return resolveGeometryTargets([], this.hoveredNodeId, this.previewItems, this.fixtureEdges);
		}
		return [];
	}

	getSelectedGeometryTargets(): readonly ProceduralChannelRef[] {
		if (this.selectedChannels.length > 0) {
			return resolveGeometryTargets(this.selectedChannels, null, this.previewItems, this.fixtureEdges);
		}
		if (this.selectedNodeIds.length > 0) {
			const targets: ProceduralChannelRef[] = [];
			for (const widgetId of this.selectedNodeIds) {
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

	getTransformGranularity(): ProceduralTransformGranularity {
		return this.transformGranularity;
	}

	getGumballActiveWidgetIds(): readonly string[] {
		return this.gumballActiveWidgetIds;
	}

	private gumballBindingKey(sourceWidgetId: string, op: ProceduralGumballTransformOp): string {
		return `${sourceWidgetId}:${op}`;
	}

	private registerGumballBinding(binding: GumballTransformBinding): void {
		this.gumballBindings.set(this.gumballBindingKey(binding.sourceWidgetId, binding.op), binding);
		this.gumballBindingByTransformId.set(binding.transformId, binding);
	}

	private findGumballBinding(widgetId: string, op: ProceduralGumballTransformOp): GumballTransformBinding | null {
		const byTransform = this.gumballBindingByTransformId.get(widgetId);
		if (byTransform && byTransform.op === op) return byTransform;
		const bySource = this.gumballBindings.get(this.gumballBindingKey(widgetId, op));
		return bySource ?? null;
	}

	private resolveGumballSourceWidgetId(widgetId: string, op: ProceduralGumballTransformOp): string {
		const byTransform = this.gumballBindingByTransformId.get(widgetId);
		if (byTransform && byTransform.op === op) return byTransform.sourceWidgetId;
		return widgetId;
	}

	private clearGumballDrag(): void {
		this.gumballDragSession = null;
		this.gumballActiveWidgetIds = [];
	}

	private syncGumballActiveChrome(binding: GumballTransformBinding): void {
		const nextActive = [binding.transformId, binding.sourceWidgetId];
		if (JSON.stringify(nextActive) !== JSON.stringify(this.gumballActiveWidgetIds)) {
			this.gumballActiveWidgetIds = nextActive;
			this.interactionRevision += 1;
			this.notifySnapshot();
		}
	}

	private dispatchFlowCanvasSelection(ids: readonly string[]): void {
		this.run("canvasCommand", { command: "setSelection", argsJson: JSON.stringify({ ids: [...ids] }) });
	}

	private dispatchGraphEdit(ops: readonly FlowGraphEditOp[], selectTransformId?: string): void {
		this.run("canvasCommand", { command: "graphEdit", argsJson: JSON.stringify({ ops }) });
		const binding = this.gumballDragSession?.binding;
		if (binding) {
			this.dispatchFlowCanvasSelection(gumballBindingNodeIds(binding));
			this.syncGumballActiveChrome(binding);
			return;
		}
		if (selectTransformId) {
			this.run("setSelection", { ids: [selectTransformId], mode: "default" });
		}
	}

	private applyLiveGumballDrag(request: ProceduralGumballTransformRequest): void {
		const session = this.gumballDragSession;
		if (!session) return;
		const values = applyGumballDeltaToBase(session.baseValues, session.binding.op, request.delta);
		setGumballBindingValues(session.binding, values);
		this.dispatchGraphEdit(this.buildGumballUpdateOps(session.binding));
	}

	private beginGumballDrag(request: ProceduralGumballTransformRequest): void {
		const op = request.delta.op;
		let binding = this.findGumballBinding(request.widgetId, op);
		let insertOps: FlowGraphEditOp[] | null = null;
		if (!binding) {
			const sourceWidgetId = this.resolveGumballSourceWidgetId(request.widgetId, op);
			const created = this.buildGumballInsertOps(sourceWidgetId, op, gumballZeroDelta(op), request.granularity);
			this.registerGumballBinding(created.binding);
			binding = created.binding;
			insertOps = created.ops;
			console.log(`[DEBUG] gumball insert ${binding.transformId} source=${sourceWidgetId} op=${op} granularity=${request.granularity}`);
		}
		this.gumballDragSession = { binding, baseValues: copyGumballValues(binding) };
		const values = applyGumballDeltaToBase(this.gumballDragSession.baseValues, op, request.delta);
		setGumballBindingValues(binding, values);
		if (insertOps) {
			this.dispatchGraphEdit(insertOps);
			return;
		}
		this.dispatchGraphEdit(this.buildGumballUpdateOps(binding));
	}

	private finishGumballDrag(request: ProceduralGumballTransformRequest): void {
		const session = this.gumballDragSession;
		if (session) {
			const binding = session.binding;
			const values = applyGumballDeltaToBase(session.baseValues, binding.op, request.delta);
			setGumballBindingValues(binding, values);
			console.log(`[DEBUG] gumball end ${binding.transformId} op=${binding.op}`);
			this.clearGumballDrag();
			this.dispatchGraphEdit(this.buildGumballUpdateOps(binding));
			this.run("setSelection", { ids: [binding.transformId], mode: "default" });
			return;
		}
		this.applyGumballTransformCommitted(request);
	}

	private applyGumballTransformCommitted(request: ProceduralGumballTransformRequest): void {
		const op = request.delta.op;
		const granularity = request.granularity;
		const existing = this.findGumballBinding(request.widgetId, op);
		if (existing) {
			accumulateGumballDelta(existing, request.delta);
			const ops = this.buildGumballUpdateOps(existing);
			console.log(`[DEBUG] gumball update ${existing.transformId} op=${op} granularity=${granularity}`);
			this.dispatchGraphEdit(ops, existing.transformId);
			return;
		}
		const sourceWidgetId = this.resolveGumballSourceWidgetId(request.widgetId, op);
		const { ops, binding } = this.buildGumballInsertOps(sourceWidgetId, op, request.delta, granularity);
		this.registerGumballBinding(binding);
		console.log(`[DEBUG] gumball insert ${binding.transformId} source=${sourceWidgetId} op=${op} granularity=${granularity}`);
		this.dispatchGraphEdit(ops, binding.transformId);
	}

	private buildGumballUpdateOps(binding: GumballTransformBinding): FlowGraphEditOp[] {
		if (binding.granularity === "compact") {
			return [{ op: "setNeuronParams", id: binding.transformId, paramsJson: JSON.stringify(compactNeuronParams(binding)) }];
		}
		if (binding.op === "translate" && binding.vectorId && binding.valueWidgetIds.length === 3) {
			const [sx, sy, sz] = binding.valueWidgetIds;
			const [x, y, z] = binding.values.offset;
			return [
				{ op: "setSliderValue", id: sx, value: x },
				{ op: "setSliderValue", id: sy, value: y },
				{ op: "setSliderValue", id: sz, value: z },
			];
		}
		const sliderId = binding.valueWidgetIds[0];
		if (!sliderId) return [];
		if (binding.op === "rotate") {
			return [{ op: "setSliderValue", id: sliderId, value: binding.values.angle }];
		}
		return [{ op: "setSliderValue", id: sliderId, value: binding.values.factor }];
	}

	private buildGumballInsertOps(
		sourceWidgetId: string,
		op: ProceduralGumballTransformOp,
		delta: ProceduralGumballTransformDelta,
		granularity: ProceduralTransformGranularity,
	): { ops: FlowGraphEditOp[]; binding: GumballTransformBinding } {
		const sourceLayout = widgetLayoutFromFixture(this.getFixtureJson(), sourceWidgetId);
		const edgeGap = gumballColumnEdgeGap(this.layerSpacing, this.siblingGap);
		const valueRowGap = gumballValueRowGap(this.siblingGap);
		const sourceHalf = GUMBALL_SOURCE_HALF_WIDTH;
		const sliderHalf = GUMBALL_SLIDER_HALF_WIDTH;
		const vectorHalf = GUMBALL_VECTOR_HALF_WIDTH;
		const transformHalf = GUMBALL_NEURON_HALF_WIDTH;
		const transformId = `${sourceWidgetId}_gumball_${op}`;
		const vectorId = `${transformId}_vector`;
		const sliderXId = `${transformId}_sx`;
		const sliderYId = `${transformId}_sy`;
		const sliderZId = `${transformId}_sz`;
		const scalarSliderId = `${transformId}_value`;
		const binding: GumballTransformBinding = {
			sourceWidgetId,
			transformId,
			op,
			granularity,
			valueWidgetIds: [],
			vectorId: undefined,
			values: {
				offset:
					delta.op === "translate" ? [delta.offset[0], delta.offset[1], delta.offset[2]] : ([0, 0, 0] as [number, number, number]),
				angle: delta.op === "rotate" ? delta.angle : 0,
				factor: delta.op === "scale" ? delta.factor : 1,
			},
		};
		let transformColumnX = gumballColumnAfter(sourceLayout.x, sourceHalf, transformHalf, edgeGap);
		const ops: FlowGraphEditOp[] = [];
		if (granularity === "compact") {
			ops.push({ op: "makeSpace", anchor: sourceWidgetId, dx: gumballMakeSpaceDx(transformColumnX, transformHalf, sourceLayout.x, edgeGap), dy: 0 });
			ops.push({
				op: "addWidget",
				descriptor: neuronDescriptor(transformId, BREP_XFORM_NEURON_KIND[op]),
				x: transformColumnX,
				y: sourceLayout.y,
			});
			ops.push({ op: "setNeuronParams", id: transformId, paramsJson: JSON.stringify(compactNeuronParams(binding)) });
		} else if (op === "translate") {
			binding.valueWidgetIds = [sliderXId, sliderYId, sliderZId];
			binding.vectorId = vectorId;
			const valueColumnX = gumballColumnAfter(sourceLayout.x, sourceHalf, sliderHalf, edgeGap);
			const vectorColumnX = gumballColumnAfter(valueColumnX, sliderHalf, vectorHalf, edgeGap);
			transformColumnX = gumballColumnAfter(vectorColumnX, vectorHalf, transformHalf, edgeGap);
			ops.push({ op: "makeSpace", anchor: sourceWidgetId, dx: gumballMakeSpaceDx(transformColumnX, transformHalf, sourceLayout.x, edgeGap), dy: 0 });
			const [x, y, z] = binding.values.offset;
			ops.push(
				{ op: "addWidget", descriptor: sliderDescriptor(sliderXId, x), x: valueColumnX, y: sourceLayout.y - valueRowGap },
				{ op: "addWidget", descriptor: sliderDescriptor(sliderYId, y), x: valueColumnX, y: sourceLayout.y },
				{ op: "addWidget", descriptor: sliderDescriptor(sliderZId, z), x: valueColumnX, y: sourceLayout.y + valueRowGap },
				{ op: "addWidget", descriptor: neuronDescriptor(vectorId, "brep.vector"), x: vectorColumnX, y: sourceLayout.y },
				{ op: "addWidget", descriptor: neuronDescriptor(transformId, BREP_XFORM_NEURON_KIND.translate), x: transformColumnX, y: sourceLayout.y },
				{ op: "connectPorts", from: sliderXId, fromPort: "number", to: vectorId, toPort: "x" },
				{ op: "connectPorts", from: sliderYId, fromPort: "number", to: vectorId, toPort: "y" },
				{ op: "connectPorts", from: sliderZId, fromPort: "number", to: vectorId, toPort: "z" },
				{ op: "connectPorts", from: vectorId, fromPort: "vector", to: transformId, toPort: "offset" },
			);
		} else {
			binding.valueWidgetIds = [scalarSliderId];
			const valueColumnX = gumballColumnAfter(sourceLayout.x, sourceHalf, sliderHalf, edgeGap);
			transformColumnX = gumballColumnAfter(valueColumnX, sliderHalf, transformHalf, edgeGap);
			ops.push({ op: "makeSpace", anchor: sourceWidgetId, dx: gumballMakeSpaceDx(transformColumnX, transformHalf, sourceLayout.x, edgeGap), dy: 0 });
			const scalarValue = op === "rotate" ? binding.values.angle : binding.values.factor;
			ops.push(
				{ op: "addWidget", descriptor: sliderDescriptor(scalarSliderId, scalarValue), x: valueColumnX, y: sourceLayout.y },
				{ op: "addWidget", descriptor: neuronDescriptor(transformId, BREP_XFORM_NEURON_KIND[op]), x: transformColumnX, y: sourceLayout.y },
				{
					op: "connectPorts",
					from: scalarSliderId,
					fromPort: "number",
					to: transformId,
					toPort: op === "rotate" ? "angle" : "factor",
				},
			);
		}
		ops.push({
			op: "insertBetween",
			anchor: sourceWidgetId,
			anchorOutPort: "solid",
			mid: transformId,
			midInPort: "geometry",
			midOutPort: "geometry",
		});
		ops.push({ op: "setPreviewOff", ids: [sourceWidgetId] });
		return { ops, binding };
	}

	/** @emoji 🎛 Inserts or updates gumball-driven transform nodes in the flow graph. */
	applyGumballTransform(request: ProceduralGumballTransformRequest): void {
		const phase: ProceduralGumballTransformPhase = request.phase ?? "end";
		if (phase === "start") {
			this.beginGumballDrag(request);
			return;
		}
		if (phase === "live") {
			this.applyLiveGumballDrag(request);
			return;
		}
		this.finishGumballDrag(request);
	}

	lodModeForScope(scopeId: string): DagLodModeKind {
		return this.lodModeByInstance[scopeId] ?? this.lodMode;
	}

	proximityDistanceValue(): number {
		return this.proximityDistance;
	}

	private lodMeasure(scopeId: string): WindowMeasure {
		return {
			kind: "select",
			id: `${scopeId}-lod`,
			label: "LOD",
			value: this.lodModeForScope(scopeId),
			items: [
				{ id: "automatic", value: DAG_LOD_MODE_AUTOMATIC, label: dagLodAutomaticSelectLabel(this.effectiveLod) },
				...dagPlayLodTiers().map((tier) => ({ id: tier, value: tier, label: dagPlayLodTierMenuLabel(tier) })),
			],
			onChange: { controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "setLodMode", args: { instanceId: scopeId } },
		};
	}

	private proximityMeasure(): WindowMeasure {
		return {
			kind: "slider",
			id: "procedural-flow-proximity-distance",
			label: "Proximity",
			value: this.proximityDistance,
			min: 0,
			max: 240,
			step: 4,
			onChange: { controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "setProximityDistance" },
		};
	}

	private flowWindowMeasures(): readonly WindowMeasure[] {
		return [this.lodMeasure(PROCEDURAL_PLAY_WINDOW_KIND_ID), this.proximityMeasure()];
	}

	private previewWindowMeasures(): readonly WindowMeasure[] {
		return [
			{
				kind: "select",
				id: `${PROCEDURAL_PLAY_WINDOW_KIND_PREVIEW}-show`,
				label: "Show",
				value: this.showMode,
				items: [
					{ id: "everything", value: "everything", label: "Everything" },
					{ id: "selected", value: "selected", label: "Selected" },
				],
				onChange: { controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "setShowMode" },
			},
			{
				kind: "select",
				id: `${PROCEDURAL_PLAY_WINDOW_KIND_PREVIEW}-transform-granularity`,
				label: "Transform Detail",
				value: this.transformGranularity,
				items: [
					{ id: "full", value: "full", label: "Full (sliders + vector)" },
					{ id: "compact", value: "compact", label: "Compact (node params)" },
				],
				onChange: { controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "setTransformGranularity" },
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
				onChange: proceduralPlayCmd("engagementInput"),
				onSubmit: proceduralPlayCmd("engagementSubmit"),
			},
			possibleEngagements: [
				{ id: "procedural.tool.reorganize", label: "Reorganize", command: proceduralPlayCmd("reorganize") },
				{ id: "procedural.layout.leftRight", label: "Left to Right", command: proceduralPlayCmd("setOrientation", { orientation: "leftRight" }) },
				{ id: "procedural.layout.topBottom", label: "Top to Bottom", command: proceduralPlayCmd("setOrientation", { orientation: "topBottom" }) },
			],
			controls: [
				{
					kind: "slider",
					id: "procedural-layer-spacing",
					label: "Layer spacing",
					value: this.layerSpacing,
					min: 40,
					max: 320,
					step: 10,
					onChange: proceduralPlayCmd("setSpacing", { field: "layerSpacing" }),
				},
				{
					kind: "slider",
					id: "procedural-sibling-gap",
					label: "Sibling gap",
					value: this.siblingGap,
					min: 10,
					max: 160,
					step: 5,
					onChange: proceduralPlayCmd("setSpacing", { field: "siblingGap" }),
				},
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
				onChange: proceduralPlayCmd("previewEngagementInput"),
				onSubmit: proceduralPlayCmd("previewEngagementSubmit"),
			},
			status: [{ id: "procedural-preview-item-count", text: `${this.previewItems.length} preview items` }],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.windowKinds = [
			new WindowKindRuntime(PROCEDURAL_PLAY_WINDOW_KIND_ID, "Flow", PROCEDURAL_PLAY_BODY_KEY_MAIN, undefined, this.flowWindowMeasures(), this.flowWindowEngagement()),
			new WindowKindRuntime(
				PROCEDURAL_PLAY_WINDOW_KIND_PREVIEW,
				"Preview",
				PROCEDURAL_PLAY_BODY_KEY_PREVIEW,
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
		this.generateMode.windowKinds = [new WindowKindRuntime(PROCEDURAL_PLAY_WINDOW_KIND_ID, "Generate", PROCEDURAL_PLAY_BODY_KEY_GENERATE)];
	}

	override run(command: string, args?: unknown): void {
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
		if (command === "setActiveFixture") {
			if (isPlaygroundFixtureLocked()) return;
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
			this.activeFixtureId = PLAYGROUND_NO_FIXTURE_ID;
			this.applyFixtureJson(PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON, true);
			return;
		}
		if (command === "setLodMode") {
			const { value, instanceId } = args as { value?: string; instanceId?: string };
			const scopeId = instanceId ?? PROCEDURAL_PLAY_WINDOW_KIND_ID;
			if (typeof value !== "string") return;
			if (value !== DAG_LOD_MODE_AUTOMATIC && !isDagDrawLodKind(value)) return;
			this.lodModeByInstance = { ...this.lodModeByInstance, [scopeId]: value as DagLodModeKind };
			if (scopeId === PROCEDURAL_PLAY_WINDOW_KIND_ID) {
				this.lodMode = value as DagLodModeKind;
			}
			this.rebuildShellMode();
			this.emit();
			return;
		}
		if (command === "setEffectiveLod") {
			const { lod, instanceId } = args as { lod?: DagDrawLodKind; instanceId?: string };
			const scopeId = instanceId ?? PROCEDURAL_PLAY_WINDOW_KIND_ID;
			if (!lod || !isDagDrawLodKind(lod)) return;
			if (scopeId !== PROCEDURAL_PLAY_WINDOW_KIND_ID) return;
			if (this.effectiveLod === lod) return;
			this.effectiveLod = lod;
			this.rebuildShellMode();
			this.emit();
			return;
		}
		if (command === "setProximityDistance") {
			const value = (args as { value?: number }).value;
			if (typeof value !== "number" || !Number.isFinite(value)) return;
			const next = Math.max(0, value);
			if (this.proximityDistance === next) return;
			this.proximityDistance = next;
			this.rebuildShellMode();
			this.emit();
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
				const nextItems = previewItemsWithMeshes(
					extractChannelPreviewItems(outputsJson),
					previewMeshes,
					this.previewItems,
				);
				this.previewItems = nextItems;
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
			const fromFlow = (args as { fromFlow?: boolean }).fromFlow === true;
			if (!Array.isArray(ids)) return;
			if (fromFlow && this.gumballDragSession) return;
			const next = selectionMergeIds(mode, this.selectedNodeIds, ids);
			if (JSON.stringify(next) === JSON.stringify(this.selectedNodeIds)) return;
			this.selectedNodeIds = next;
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
			this.selectedNodeIds = [...new Set(ids)];
			this.preselectNodeIds = [];
			this.preselectRemovedNodeIds = [];
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "clearSelection") {
			if (!this.selectedNodeIds.length) return;
			this.selectedNodeIds = [];
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
			if (next === this.hoveredNodeId && channelJson === currentChannelJson) return;
			this.hoveredNodeId = next;
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
			this.selectedNodeIds = [...new Set(next.map((channel) => channel.widgetId))];
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
			if (fromFlow && this.gumballDragSession) {
				const next = [...ids];
				if (JSON.stringify(next) === JSON.stringify(this.previewOffNodeIds)) return;
				this.previewOffNodeIds = next;
				this.interactionRevision += 1;
				this.notifySnapshot();
				return;
			}
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
		if (command === "setTransformGranularity") {
			const granularity =
				(args as { granularity?: ProceduralTransformGranularity }).granularity ??
				(args as { value?: string }).value;
			if (granularity !== "compact" && granularity !== "full") return;
			if (this.transformGranularity === granularity) return;
			this.transformGranularity = granularity;
			this.rebuildShellMode();
			this.emit();
			return;
		}
		if (command === "applyGumballTransform") {
			const widgetId = (args as { widgetId?: string }).widgetId;
			const delta = (args as { delta?: ProceduralGumballTransformDelta }).delta;
			const granularity = (args as { granularity?: ProceduralTransformGranularity }).granularity ?? this.transformGranularity;
			const phase = (args as { phase?: ProceduralGumballTransformPhase }).phase;
			if (typeof widgetId !== "string" || !delta) return;
			this.applyGumballTransform({ widgetId, delta, granularity, phase });
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
			void proceduralExtensionHost.setActive(id, enabled).then(() => {
				this.extensionRevision += 1;
				this.notifySnapshot();
				this.emit();
			});
			return;
		}
		if (command === "runExtensionCommand") {
			const commandId = (args as { commandId?: string }).commandId;
			if (typeof commandId !== "string") return;
			const result = proceduralExtensionHost.executeCommand(commandId);
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

export function registerProceduralPlayDeclarativeBodies(): void {
	registerWindowBody(PROCEDURAL_PLAY_BODY_KEY_MAIN, (_ctx: WindowBodyViewContext) =>
		buildFlowWindowBody(PROCEDURAL_PLAY_SURFACE_ID, PROCEDURAL_3D_PLAY_CONTROLLER_ID, PROCEDURAL_PLAY_WINDOW_KIND_ID));
	registerWindowBody(PROCEDURAL_PLAY_BODY_KEY_PREVIEW, (_ctx: WindowBodyViewContext) =>
		buildPuzzle3dWindowBody(PROCEDURAL_PLAY_SURFACE_ID_PREVIEW, PROCEDURAL_3D_PLAY_CONTROLLER_ID));
	registerWindowBody(PROCEDURAL_PLAY_BODY_KEY_GENERATE, (_ctx: WindowBodyViewContext) =>
		buildFormsWindowBody(PROCEDURAL_PLAY_SURFACE_ID_GENERATE, PROCEDURAL_3D_PLAY_CONTROLLER_ID, "generate"));
}

export function buildProceduralPlayAppRuntime(controller: ProceduralPlayController): AppRuntime {
	const app = createPlayAppRuntime(PROCEDURAL_3D_PLAY_APP_ID, "Procedural 3D", controller, PROCEDURAL_PLAY_LAYOUT, controller.mainMode);
	app.addMode(controller.generateMode);
	return app;
}

/** @emoji 🛝 Procedural playground app. */
export class PlaygroundProcedural extends Playground {
	readonly id = PROCEDURAL_3D_PLAY_APP_ID;
	readonly keybindings = [
		{ key: "ctrl+a,meta+a", controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "selectAll" },
		{ key: "Delete", controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
		{ key: "Backspace", controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
	];

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new ProceduralPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildProceduralPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerProceduralPlayDeclarativeBodies();
	}
}

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@semio-tech/procedural-3d-play", () => {
		it("exports default fixture json", () => {
			expect(PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON).toContain("flow.fixture/v1");
		});

		it("starts with no fixture selected", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			expect(ctrl.getFixtureCatalog().activeFixtureId).toBe(PLAYGROUND_NO_FIXTURE_ID);
			expect(ctrl.getFixtureJson()).toContain('"widgets":[]');
		});

		it("does not auto-load stored fixture on startup", () => {
			const backing = new Map<string, string>();
			const store = createProceduralPlayFixtureStore({
				getItem: (k) => backing.get(k) ?? null,
				setItem: (k, v) => {
					backing.set(k, v);
				},
				removeItem: (k) => {
					backing.delete(k);
				},
			});
			store.save(PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON);
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {}, store);
			expect(ctrl.getFixtureJson()).toContain('"widgets":[]');
		});

		it("controller stores fixture json", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setFixtureJson", { json: '{"schema":"flow.fixture/v1"}' });
			expect(ctrl.getFixtureJson()).toContain("flow.fixture/v1");
		});

		it("kinds tree marks nested catalogue rows draggable", () => {
			const tree = buildProceduralPlayKindsTree([
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
			const ctrl = new ProceduralPlayController(bus, () => {});
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
			const ctrl = new ProceduralPlayController(bus, () => {});
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
			const ctrl = new ProceduralPlayController(bus, () => {});
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

		it("controller exposes flow and preview window kinds", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			expect(ctrl.mainMode.windowKinds).toHaveLength(2);
			expect(ctrl.mainMode.windowKinds[1]?.id).toBe(PROCEDURAL_PLAY_WINDOW_KIND_PREVIEW);
		});

		it("flow window exposes inline lod select", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			const measures = ctrl.mainMode.windowKinds[0]?.measures ?? [];
			expect(measures.some((measure) => measure.kind === "select" && measure.label === "LOD")).toBe(true);
		});

		it("flow window proximity measure defaults and updates via command", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			expect(ctrl.proximityDistanceValue()).toBe(FLOW_DEFAULT_PROXIMITY_DISTANCE);
			const measures = ctrl.mainMode.windowKinds[0]?.measures ?? [];
			const proximity = measures.find((measure) => measure.kind === "slider" && measure.label === "Proximity");
			expect(proximity?.kind).toBe("slider");
			if (proximity?.kind === "slider") {
				expect(proximity.value).toBe(FLOW_DEFAULT_PROXIMITY_DISTANCE);
			}
			ctrl.run("setProximityDistance", { value: 0 });
			expect(ctrl.proximityDistanceValue()).toBe(0);
			const updated = ctrl.mainMode.windowKinds[0]?.measures?.find((measure) => measure.kind === "slider" && measure.label === "Proximity");
			expect(updated?.kind).toBe("slider");
			if (updated?.kind === "slider") {
				expect(updated.value).toBe(0);
			}
		});

		it("preview window exposes show mode and transform detail in shell measures", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			const measures = ctrl.mainMode.windowKinds[1]?.measures ?? [];
			const show = measures.find((measure) => measure.kind === "select" && measure.label === "Show");
			expect(show?.kind === "select" && show.value).toBe("everything");
			expect(measures.some((measure) => measure.kind === "select" && measure.label === "Transform Detail")).toBe(true);
		});

		it("setTransformGranularity accepts shell measure value", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setTransformGranularity", { value: "compact" });
			expect(ctrl.getTransformGranularity()).toBe("compact");
		});

		it("setShowMode updates preview filter", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			expect(ctrl.getShowMode()).toBe("everything");
			ctrl.run("setShowMode", { id: "selected" });
			expect(ctrl.getShowMode()).toBe("selected");
		});

		it("setShowMode accepts shell measure value", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setShowMode", { value: "selected" });
			expect(ctrl.getShowMode()).toBe("selected");
			ctrl.run("setShowMode", { value: "everything" });
			expect(ctrl.getShowMode()).toBe("everything");
		});

		it("canvasCommand bumps command request epoch", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("canvasCommand", { command: "deleteSelection" });
			expect(ctrl.getCommandRequest().command).toBe("deleteSelection");
			expect(ctrl.getCommandRequest().epoch).toBe(1);
		});

		it("deleteSelection forwards to flow canvas command request", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setSelection", { ids: ["node-a"] });
			ctrl.run("deleteSelection");
			expect(ctrl.getCommandRequest().command).toBe("deleteSelection");
			expect(ctrl.getSelectedNodeIds()).toEqual(["node-a"]);
		});

		it("setPreviewOff stores preview-off node ids", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setPreviewOff", { ids: ["a", "b"] });
			expect(ctrl.getPreviewOffNodeIds()).toEqual(["a", "b"]);
		});

		it("buildProceduralPlayCanvasContextMenu adds isolate in preview for hovered node", () => {
			const items = buildProceduralPlayCanvasContextMenu(
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
			expect(items.some((item) => item.id === "procedural.ctx.isolatePreview")).toBe(true);
		});

		it("setFixtureJson sync preserves preview items after flow interaction", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setEvalOutputs", {
				outputsJson: JSON.stringify({ box: { in: {}, out: { solid: { geometry: "solid-1" } } } }),
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
				{ widgetId: "box", port: "solid", direction: "out", kind: "geometry", handle: "solid-1" },
			]);
		});

		it("setFixtureJson with resetInteraction clears preview items", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setEvalOutputs", {
				outputsJson: JSON.stringify({ box: { in: {}, out: { solid: { geometry: "solid-1" } } } }),
			});
			ctrl.run("setFixtureJson", {
				json: '{"schema":"flow.fixture/v1","camera":{"x":0,"y":0,"zoom":1},"widgets":[],"synapses":[]}',
				resetInteraction: true,
			});
			expect(ctrl.getPreviewItems()).toEqual([]);
		});

		it("setEvalOutputs stores preview items per widget", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setEvalOutputs", {
				outputsJson: JSON.stringify({ box: { in: {}, out: { solid: { geometry: "solid-1" } } } }),
			});
			expect(ctrl.getPreviewItems()).toEqual([
				{ widgetId: "box", port: "solid", direction: "out", kind: "geometry", handle: "solid-1" },
			]);
		});

		it("setEvalOutputs attaches nested face meshes by handle", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setEvalOutputs", {
				outputsJson: JSON.stringify({
					get: { in: {}, out: { value: { 0: { $schema: "face", handle: "face-42" } } } },
				}),
				previewMeshes: {
					"face-42": {
						position: [0, 0, 0, 1, 0, 0, 0, 1, 0],
						normal: [0, 0, 1, 0, 0, 1, 0, 0, 1],
						index: [0, 1, 2],
						edges: [],
						faceGroups: [{ start: 0, count: 3, entityId: "face-42" }],
					},
				},
			});
			expect(ctrl.getPreviewItems()).toHaveLength(1);
			const item = ctrl.getPreviewItems()[0];
			expect(item?.kind).toBe("geometry");
			expect(item?.kind === "geometry" ? item.mesh?.position.length : 0).toBe(9);
		});

		it("setEvalOutputs stores point and vector preview items", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setEvalOutputs", {
				outputsJson: JSON.stringify({
					pt: { in: {}, out: { point: { $schema: "point", x: 1, y: 0, z: 0 } } },
					vec: { in: {}, out: { vector: { $schema: "vector", x: 0, y: 1, z: 0 } } },
				}),
			});
			expect(ctrl.getPreviewItems()).toEqual([
				{ widgetId: "pt", port: "point", direction: "out", kind: "point", position: [1, 0, 0] },
				{ widgetId: "vec", port: "vector", direction: "out", kind: "vector", directionVec: [0, 1, 0] },
			]);
		});

		it("selectAll includes widgets with point and vector preview items", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setEvalOutputs", {
				outputsJson: JSON.stringify({
					pt: { in: {}, out: { point: { $schema: "point", x: 0, y: 0, z: 0 } } },
					vec: { in: {}, out: { vector: { $schema: "vector", x: 1, y: 0, z: 0 } } },
				}),
			});
			ctrl.run("selectAll");
			expect(ctrl.getSelectedNodeIds().sort()).toEqual(["pt", "vec"]);
		});

		it("setHoverChannel and geometry target getters resolve upstream output", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setEvalOutputs", {
				outputsJson: JSON.stringify({
					circle: { in: {}, out: { wire: { geometry: "drawing-1" } } },
					offset: { in: { geometry: "drawing-1" }, out: { geometry: { geometry: "wire-2" } } },
				}),
			});
			ctrl.run("setFixtureJson", {
				json: JSON.stringify({
					schema: "flow.fixture/v1",
					camera: { x: 0, y: 0, zoom: 1 },
					widgets: [
						{ kind: "neuron", id: "circle", neuronKind: "brep.sketch2d.circle" },
						{ kind: "neuron", id: "offset", neuronKind: "brep.xform.offset" },
					],
					synapses: [{ id: "s1", from: "circle", to: "offset", from_port: "wire", to_port: "geometry" }],
				}),
			});
			ctrl.run("setHoverChannel", {
				channel: { widgetId: "offset", port: "geometry", direction: "in" },
			});
			expect(ctrl.getHoveredChannel()).toEqual({ widgetId: "offset", port: "geometry", direction: "in" });
			expect(ctrl.getHoveredGeometryTargets()).toEqual([{ widgetId: "circle", port: "wire", direction: "out" }]);
		});

		it("parseFixtureEdges reads camelCase flow synapse ports", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setFixtureJson", {
				json: JSON.stringify({
					schema: "flow.fixture/v1",
					camera: { x: 0, y: 0, zoom: 1 },
					widgets: [],
					synapses: [
						{ id: "e101", from: "brep_prim3d_sphere_2", to: "brep_bool_cut_5", fromPort: "solid", toPort: "a" },
						{ id: "e102", from: "brep_prim3d_torus_4", to: "brep_bool_cut_5", fromPort: "solid", toPort: "b" },
					],
				}),
			});
			expect(ctrl.getSelectedGeometryTargets()).toEqual([]);
			ctrl.run("setSelectChannels", {
				channels: [{ widgetId: "brep_bool_cut_5", port: "a", direction: "in" }],
			});
			ctrl.run("setEvalOutputs", {
				outputsJson: JSON.stringify({
					brep_prim3d_sphere_2: { in: {}, out: { solid: { geometry: "solid-sphere" } } },
					brep_bool_cut_5: { in: { a: { geometry: "solid-sphere" } }, out: { solid: { geometry: "solid-cut" } } },
				}),
			});
			expect(ctrl.getSelectedGeometryTargets()).toEqual([
				{ widgetId: "brep_prim3d_sphere_2", port: "solid", direction: "out" },
			]);
		});

		it("show selected reveals upstream geometry for preview-off input channels", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			const outputsJson = JSON.stringify({
				brep_prim3d_sphere_2: { in: {}, out: { solid: { geometry: "solid-sphere" } } },
				brep_prim3d_torus_4: { in: {}, out: { solid: { geometry: "solid-torus" } } },
				brep_bool_cut_5: {
					in: { a: { geometry: "solid-sphere" }, b: { geometry: "solid-torus" } },
					out: { solid: { geometry: "solid-cut" } },
				},
			});
			ctrl.run("setEvalOutputs", { outputsJson });
			ctrl.run("setFixtureJson", {
				json: JSON.stringify({
					schema: "flow.fixture/v1",
					camera: { x: 0, y: 0, zoom: 1 },
					widgets: [
						{ kind: "neuron", id: "brep_prim3d_sphere_2", neuronKind: "brep.prim3d.sphere", preview: false },
						{ kind: "neuron", id: "brep_prim3d_torus_4", neuronKind: "brep.prim3d.torus", preview: false },
						{ kind: "neuron", id: "brep_bool_cut_5", neuronKind: "brep.bool.cut", preview: true },
					],
					synapses: [
						{ id: "e1", from: "brep_prim3d_sphere_2", to: "brep_bool_cut_5", fromPort: "solid", toPort: "a" },
						{ id: "e2", from: "brep_prim3d_torus_4", to: "brep_bool_cut_5", fromPort: "solid", toPort: "b" },
					],
				}),
			});
			ctrl.run("setPreviewOff", {
				ids: ["brep_prim3d_sphere_2", "brep_prim3d_torus_4"],
				fromFlow: true,
			});
			ctrl.run("setShowMode", { id: "selected" });
			ctrl.run("setSelectChannels", {
				channels: [{ widgetId: "brep_bool_cut_5", port: "a", direction: "in" }],
			});
			const visible = filterVisiblePreviewItems(ctrl.getPreviewItems(), {
				showMode: ctrl.getShowMode(),
				selectedNodeIds: [...ctrl.getSelectedNodeIds()],
				selectedChannels: [...ctrl.getSelectedChannels()],
				selectedGeometryTargets: [...ctrl.getSelectedGeometryTargets()],
				hoveredNodeId: null,
				hoveredChannel: null,
			});
			expect(visible).toEqual([
				{
					widgetId: "brep_prim3d_sphere_2",
					port: "solid",
					direction: "out",
					kind: "geometry",
					handle: "solid-sphere",
				},
			]);
		});

		it("setSelectChannels stores channel selection and parent nodes", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setSelectChannels", {
				channels: [{ widgetId: "box", port: "solid", direction: "out" }],
			});
			expect(ctrl.getSelectedChannels()).toEqual([{ widgetId: "box", port: "solid", direction: "out" }]);
			expect(ctrl.getSelectedNodeIds()).toEqual(["box"]);
		});

		it("setSelection and setHover update interaction revision", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setSelection", { ids: ["box"] });
			ctrl.run("setHover", { id: "box" });
			expect(ctrl.getSelectedNodeIds()).toEqual(["box"]);
			expect(ctrl.getHoveredNodeId()).toBe("box");
			expect(ctrl.getInteractionRevision()).toBeGreaterThan(0);
		});

		it("setHover stores hovered channel", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setHover", { id: "offset", channel: { widgetId: "offset", port: "geometry", direction: "in" } });
			expect(ctrl.getHoveredChannel()).toEqual({ widgetId: "offset", port: "geometry", direction: "in" });
		});

		it("setSelection merges additively when mode is additive", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setSelection", { ids: ["a"], mode: "default" });
			ctrl.run("setSelection", { ids: ["b"], mode: "additive" });
			expect(ctrl.getSelectedNodeIds()).toEqual(["a", "b"]);
		});

		it("setSelectionMethod updates marquee method", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setSelectionMethod", { method: "lasso" });
			expect(ctrl.getSelectionMethod()).toBe("lasso");
		});

		it("buildProceduralPlayToolbarTools registers selection, save, view, and actions", () => {
			const tools = buildProceduralPlayToolbarTools(
				{
					selectionMethod: "rectangle",
					selectionMode: "default",
					showMode: "everything",
					selectionCount: 0,
					hasStoredFixture: false,
				},
				PROCEDURAL_3D_PLAY_CONTROLLER_ID,
			);
			expect(tools.selection?.some((row) => row.id === "procedural.select.rectangle")).toBe(true);
			expect(tools.save?.map((row) => row.id)).toEqual([
				"procedural.save.stored",
				"procedural.save.download",
				"procedural.save.load",
				"procedural.save.loadStored",
				"procedural.save.reset",
			]);
			expect(tools.save?.[3]?.disabled).toBe(true);
			expect(tools.view?.length).toBe(2);
			expect(tools.actions?.some((row) => row.id === "procedural.action.reorganize")).toBe(true);
		});

		it("controller exposes toolbar tools when host bridge is attached", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
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
			const store = createProceduralPlayFixtureStore({
				getItem: (k) => backing.get(k) ?? null,
				setItem: (k, v) => {
					backing.set(k, v);
				},
				removeItem: (k) => {
					backing.delete(k);
				},
			});
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {}, store);
			ctrl.run("saveStored");
			expect(ctrl.hasStoredFixture()).toBe(true);
			ctrl.run("setFixtureJson", { json: '{"schema":"flow.fixture/v1","widgets":[],"synapses":[]}' });
			ctrl.run("loadStored");
			expect(ctrl.getFixtureJson()).toContain("flow.fixture/v1");
		});

		it("setActiveFixture loads default and empty fixtures", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setActiveFixture", { fixtureId: PLAYGROUND_NO_FIXTURE_ID });
			expect(ctrl.getFixtureJson()).toContain('"widgets":[]');
			ctrl.run("setActiveFixture", { fixtureId: PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID });
			expect(ctrl.getFixtureJson()).toContain("brep.prim3d.box");
		});

		it("fixture catalog includes procedural/fixture files", () => {
			expect(PROCEDURAL_PLAY_FIXTURE_OPTIONS.some((option) => option.id === "sphere-cut-with-torus")).toBe(true);
			expect(PROCEDURAL_PLAY_FIXTURE_OPTIONS.find((option) => option.id === "sphere-cut-with-torus")?.label).toBe(
				"Sphere Cut With Torus",
			);
			expect(PROCEDURAL_PLAY_FIXTURE_OPTIONS.some((option) => option.id === PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID)).toBe(
				true,
			);
		});

		it("resolveProceduralPlayFixtureSlug maps hexagonal-column shorthand", async () => {
			const { resolveProceduralPlayFixtureSlug, PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID } = await import(
				"./fixture-slugs.js"
			);
			expect(resolveProceduralPlayFixtureSlug("hexagonal-column")).toBe(PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID);
			expect(resolveProceduralPlayFixtureSlug("column")).toBe(PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID);
		});

		it("getFixtureCatalog returns null when fixture host is locked", () => {
			const prev = import.meta.env.PLAYGROUND_LOCKED_FIXTURE_ID;
			(import.meta.env as { PLAYGROUND_LOCKED_FIXTURE_ID?: string }).PLAYGROUND_LOCKED_FIXTURE_ID =
				PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID;
			try {
				const bus = new CommandBus();
				const ctrl = new ProceduralPlayController(bus, () => {});
				expect(ctrl.getFixtureCatalog()).toBeNull();
				ctrl.run("setActiveFixture", { fixtureId: PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID });
				expect(ctrl.getFixtureCatalog()).toBeNull();
			} finally {
				if (prev === undefined) {
					delete (import.meta.env as { PLAYGROUND_LOCKED_FIXTURE_ID?: string }).PLAYGROUND_LOCKED_FIXTURE_ID;
				} else {
					(import.meta.env as { PLAYGROUND_LOCKED_FIXTURE_ID?: string }).PLAYGROUND_LOCKED_FIXTURE_ID = prev;
				}
			}
		});

		it("locked fixture host loads file fixture on construct", () => {
			const prev = import.meta.env.PLAYGROUND_LOCKED_FIXTURE_ID;
			(import.meta.env as { PLAYGROUND_LOCKED_FIXTURE_ID?: string }).PLAYGROUND_LOCKED_FIXTURE_ID =
				PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID;
			try {
				const bus = new CommandBus();
				const ctrl = new ProceduralPlayController(bus, () => {});
				expect(ctrl.getFixtureJson()).toContain("brep.solid.extrude");
				expect(ctrl.getFixtureJson()).toContain("brep_curve_polygon_9");
			} finally {
				if (prev === undefined) {
					delete (import.meta.env as { PLAYGROUND_LOCKED_FIXTURE_ID?: string }).PLAYGROUND_LOCKED_FIXTURE_ID;
				} else {
					(import.meta.env as { PLAYGROUND_LOCKED_FIXTURE_ID?: string }).PLAYGROUND_LOCKED_FIXTURE_ID = prev;
				}
			}
		});

		it("setActiveFixture loads file fixtures from procedural/fixture", () => {
			const sphereCutId = "sphere-cut-with-torus";
			expect(proceduralPlayFixtureJson(sphereCutId)).toContain("brep.bool.cut");
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setActiveFixture", { fixtureId: sphereCutId });
			expect(ctrl.getFixtureJson()).toContain("brep.bool.cut");
			expect(ctrl.getFixtureJson()).toContain("brep.prim3d.sphere");
		});

		it("extensions tree lists installed modules", () => {
			const tree = buildProceduralPlayExtensionsTree([
				{
					id: "brep",
					active: true,
					manifest: {
						schema: "flow.module/v1",
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

		it("applyGumballTransform dispatches graphEdit insert then update", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setFixtureJson", {
				json: JSON.stringify({
					schema: "flow.fixture/v1",
					camera: { x: 0, y: 0, zoom: 1 },
					widgets: [{ kind: "neuron", id: "solid", neuronKind: "brep.prim3d.box" }],
					synapses: [],
					layout: { solid: { x: 100, y: 50 } },
				}),
			});
			ctrl.applyGumballTransform({
				widgetId: "solid",
				granularity: "compact",
				delta: { op: "translate", offset: [1, 0, 0] },
			});
			const insert = ctrl.getCommandRequest();
			expect(insert.command).toBe("graphEdit");
			const insertOps = JSON.parse(insert.argsJson ?? "{}").ops as FlowGraphEditOp[];
			expect(insertOps.some((op) => op.op === "insertBetween")).toBe(true);
			const makeSpace = insertOps.find((op) => op.op === "makeSpace");
			expect(makeSpace?.op === "makeSpace" && makeSpace.dx).toBeGreaterThan(120);
			ctrl.applyGumballTransform({
				widgetId: "solid_gumball_translate",
				granularity: "compact",
				delta: { op: "translate", offset: [0, 2, 0] },
			});
			const update = ctrl.getCommandRequest();
			const updateOps = JSON.parse(update.argsJson ?? "{}").ops as FlowGraphEditOp[];
			expect(updateOps).toEqual([{ op: "setNeuronParams", id: "solid_gumball_translate", paramsJson: JSON.stringify({ offset: [1, 2, 0] }) }]);
		});

		it("applyGumballTransform live drag updates without accumulating per frame", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setFixtureJson", {
				json: JSON.stringify({
					schema: "flow.fixture/v1",
					camera: { x: 0, y: 0, zoom: 1 },
					widgets: [{ kind: "neuron", id: "solid", neuronKind: "brep.prim3d.box" }],
					synapses: [],
					layout: { solid: { x: 100, y: 50 } },
				}),
			});
			ctrl.applyGumballTransform({
				widgetId: "solid",
				granularity: "compact",
				phase: "start",
				delta: { op: "translate", offset: [0, 0, 0] },
			});
			expect(ctrl.getGumballActiveWidgetIds()).toEqual(["solid_gumball_translate", "solid"]);
			ctrl.applyGumballTransform({
				widgetId: "solid",
				granularity: "compact",
				phase: "live",
				delta: { op: "translate", offset: [2, 0, 0] },
			});
			ctrl.applyGumballTransform({
				widgetId: "solid",
				granularity: "compact",
				phase: "end",
				delta: { op: "translate", offset: [3, 0, 0] },
			});
			const end = ctrl.getCommandRequest();
			const endOps = JSON.parse(end.argsJson ?? "{}").ops as FlowGraphEditOp[];
			expect(endOps).toEqual([
				{ op: "setNeuronParams", id: "solid_gumball_translate", paramsJson: JSON.stringify({ offset: [3, 0, 0] }) },
			]);
			expect(ctrl.getGumballActiveWidgetIds()).toEqual([]);
		});

		it("applyGumballTransform full translate lays out value, vector, and transform columns without overlap", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setFixtureJson", {
				json: JSON.stringify({
					schema: "flow.fixture/v1",
					camera: { x: 0, y: 0, zoom: 1 },
					widgets: [{ kind: "neuron", id: "solid", neuronKind: "brep.prim3d.box" }],
					synapses: [],
					layout: { solid: { x: 200, y: 0 } },
				}),
			});
			ctrl.applyGumballTransform({
				widgetId: "solid",
				granularity: "full",
				delta: { op: "translate", offset: [1, 2, 3] },
			});
			const insertOps = JSON.parse(ctrl.getCommandRequest().argsJson ?? "{}").ops as FlowGraphEditOp[];
			const positions = insertOps
				.filter((op): op is Extract<FlowGraphEditOp, { op: "addWidget" }> => op.op === "addWidget")
				.map((op) => ({ id: JSON.parse(op.descriptor).id as string, x: op.x, y: op.y }));
			const byId = Object.fromEntries(positions.map((entry) => [entry.id, entry]));
			expect(byId.solid_gumball_translate_sx.x).toBeLessThan(byId.solid_gumball_translate_vector.x);
			expect(byId.solid_gumball_translate_vector.x).toBeLessThan(byId.solid_gumball_translate.x);
			expect(byId.solid_gumball_translate_sx.x - byId.solid_gumball_translate_sy.x).toBe(0);
			expect(Math.abs(byId.solid_gumball_translate_sx.y - byId.solid_gumball_translate_sy.y)).toBeGreaterThanOrEqual(32);
			const makeSpace = insertOps.find((op) => op.op === "makeSpace");
			expect(makeSpace?.op === "makeSpace" && makeSpace.dx).toBeGreaterThan(240);
			const sliderX = insertOps.find((op) => op.op === "addWidget" && JSON.parse(op.descriptor).id === "solid_gumball_translate_sx");
			expect(sliderX?.op).toBe("addWidget");
			expect(JSON.parse(sliderX!.descriptor)).toEqual({ kind: "inputSlider", id: "solid_gumball_translate_sx", value: 1, min: 0, max: 1, step: 1 });
		});
	});
}
// #endregion 🧪Tests

// #region 🔖Boot
if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "procedural-3d") {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootProceduralPlay } = await import("@semio-tech/framework-playground-renderer-react/procedural-3d");
		bootProceduralPlay(new PlaygroundProcedural());
	})();
}
// #endregion 🔖Boot
