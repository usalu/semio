// #region 🧲Header
/** @emoji 🔧 Procedural play harness on `@framework/playground/core`. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	Platform,
	Playground,
	WindowKindRuntime,
	buildFlowWindowBody,
	buildPuzzle3dWindowBody,
	createDefaultLayout,
	enforcePlaygroundWindowEngagementInput,
	registerWindowBody,
	type CommandDescriptor,
	type UiNode,
	type UiTreeItemNode,
	type UiTreeSectionNode,
	type WindowBodyViewContext,
	type WindowEngagement,
} from "@framework/playground/core";
import { bootstrapElementsSurfaceChromeDocument, selectionMergeIds, type SelectionMergeMode } from "@ui/react";
import {
	DAG_LOD_MODE_AUTOMATIC,
	dagPlayLodTiers,
	dagLodAutomaticSelectLabel,
	dagPlayLodTierMenuLabel,
	flowPlayCatalogueItemDragData,
	isDagDrawLodKind,
	type CatalogueItem,
	type CatalogueSection,
	type DagDrawLodKind,
	type DagLodModeKind,
	type FlowExtensionEntry,
	type FlowReorganizeRequest,
} from "@flow/react";
import type { WindowMeasure } from "@framework/playground/core";
import {
	extractGeometryHandles,
	PROCEDURAL_DEFAULT_FIXTURE,
	proceduralExtensionHost,
	proceduralFixtureToJson,
	type FlowFixtureV1,
	type ProceduralGeometryHandle,
	type ProceduralPreviewShowMode,
} from "@procedural/react";

export const PROCEDURAL_PLAY_APP_ID = "procedural-play";
export const PROCEDURAL_PLAY_CONTROLLER_ID = "procedural-play";
export const PROCEDURAL_PLAY_SURFACE_ID = "procedural.play/v1";
export const PROCEDURAL_PLAY_BODY_KEY_MAIN = "procedural.play.main";
export const PROCEDURAL_PLAY_WINDOW_KIND_ID = "procedural-main";
export const PROCEDURAL_PLAY_WINDOW_KIND_PREVIEW = "procedural-preview";
export const PROCEDURAL_PLAY_BODY_KEY_PREVIEW = "procedural.play.preview";
export const PROCEDURAL_PLAY_SURFACE_ID_PREVIEW = "procedural.play.preview/v1";

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

export type ProceduralLayoutOrientation = "leftRight" | "topBottom";
export type ProceduralPlaySelectionMode = SelectionMergeMode;
export type ProceduralPlaySelectionMethod = "rectangle" | "lasso";

const DEFAULT_LAYER_SPACING = 120;
const DEFAULT_SIBLING_GAP = 40;

function proceduralPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: PROCEDURAL_PLAY_CONTROLLER_ID, command, args };
}

function buildProceduralLayoutOptionsJson(layerSpacing: number, siblingGap: number, orientation: ProceduralLayoutOrientation): string {
	return JSON.stringify({ layerSpacing, siblingGap, orientation });
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
					defaultOpen: true,
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
			defaultOpen: true,
			items: entries.map((entry) => ({
				id: `procedural-play-extensions.${entry.id}`,
				label: entry.manifest.name,
				description: `${entry.manifest.version} · ${entry.active ? "enabled" : "disabled"} · ${entry.manifest.contributes.neuronKinds.length} kinds · ${entry.manifest.contributes.commands.length} commands`,
				command: proceduralPlayCmd("toggleExtension", { id: entry.id, enabled: !entry.active }),
			})),
		},
	];
	if (commandItems.length) {
		sections.push({
			id: "procedural-play-extensions.commands",
			label: "Commands",
			defaultOpen: true,
			items: commandItems,
		});
	}
	return { type: "tree", sections };
}

function proceduralPlayKindsTreeItem(sectionId: string, index: number, item: CatalogueItem): UiTreeItemNode {
	return {
		id: `procedural-play-kinds.${sectionId}.${index}.${item.neuronKind ?? item.kind}`,
		label: item.name,
		description: item.summary,
		draggable: true,
		dragData: flowPlayCatalogueItemDragData(item),
	};
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
					defaultOpen: true,
					items: [{ id: "procedural-play-kinds.empty.msg", label: "Loading catalogue…" }],
				},
			],
		};
	}
	const treeSections: UiTreeSectionNode[] = sections.map((section) => ({
		id: `procedural-play-kinds.${section.id}`,
		label: section.title,
		defaultOpen: true,
		items: section.items.map((item, index) => proceduralPlayKindsTreeItem(section.id, index, item)),
	}));
	return { type: "tree", sections: treeSections };
}

/** @emoji 🎛 Procedural play shell controller. */
export class ProceduralPlayController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Procedural", undefined);
	private fixtureJson = PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON;
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
	private extensionRevision = 0;
	private geometryHandles: ProceduralGeometryHandle[] = [];
	private selectedNodeIds: string[] = [];
	private preselectNodeIds: string[] = [];
	private preselectRemovedNodeIds: string[] = [];
	private hoveredNodeId: string | null = null;
	private previewOffNodeIds: string[] = [];
	private showMode: ProceduralPreviewShowMode = "everything";
	private selectionMode: ProceduralPlaySelectionMode = "default";
	private selectionMethod: ProceduralPlaySelectionMethod = "rectangle";
	private interactionRevision = 0;
	private lodMode: DagLodModeKind = DAG_LOD_MODE_AUTOMATIC;
	private lodModeByInstance: Record<string, DagLodModeKind> = {};
	private effectiveLod: DagDrawLodKind = "normal";

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(PROCEDURAL_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.rebuildShellMode();
	}

	getFixtureJson(): string {
		return this.fixtureJson;
	}

	getPreviewText(): string {
		return this.previewText;
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

	getGeometryHandles(): readonly ProceduralGeometryHandle[] {
		return this.geometryHandles;
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

	getPreviewOffNodeIds(): readonly string[] {
		return this.previewOffNodeIds;
	}

	getShowMode(): ProceduralPreviewShowMode {
		return this.showMode;
	}

	getInteractionRevision(): number {
		return this.interactionRevision;
	}

	lodModeForScope(scopeId: string): DagLodModeKind {
		return this.lodModeByInstance[scopeId] ?? this.lodMode;
	}

	private lodMeasure(scopeId: string): WindowMeasure {
		return {
			kind: "select",
			id: `${scopeId}-lod`,
			value: this.lodModeForScope(scopeId),
			items: [
				{ id: "automatic", value: DAG_LOD_MODE_AUTOMATIC, label: dagLodAutomaticSelectLabel(this.effectiveLod) },
				...dagPlayLodTiers().map((tier) => ({ id: tier, value: tier, label: dagPlayLodTierMenuLabel(tier) })),
			],
			onChange: { controllerId: PROCEDURAL_PLAY_CONTROLLER_ID, command: "setLodMode", args: { instanceId: scopeId } },
		};
	}

	private flowWindowMeasures(): readonly WindowMeasure[] {
		return [{ kind: "group", id: `${PROCEDURAL_PLAY_WINDOW_KIND_ID}-lod`, label: "LOD", children: [this.lodMeasure(PROCEDURAL_PLAY_WINDOW_KIND_ID)] }];
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

	private syncReorganizeOptionsJson(): void {
		this.reorganizeOptionsJson = buildProceduralLayoutOptionsJson(this.layerSpacing, this.siblingGap, this.orientation);
	}

	private triggerReorganize(): void {
		this.syncReorganizeOptionsJson();
		this.reorganizeEpoch += 1;
		this.rebuildShellMode();
		this.emit();
	}

	private selectionMeasures() {
		return [
			{
				kind: "toggle" as const,
				id: "procedural-flow-marquee-rectangle",
				iconId: "square",
				text: "Rectangle",
				pressed: this.selectionMethod === "rectangle",
				onChange: proceduralPlayCmd("setSelectionMethod", { method: "rectangle" }),
			},
			{
				kind: "toggle" as const,
				id: "procedural-flow-marquee-lasso",
				iconId: "lasso",
				text: "Lasso",
				pressed: this.selectionMethod === "lasso",
				onChange: proceduralPlayCmd("setSelectionMethod", { method: "lasso" }),
			},
		];
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
			measures: this.selectionMeasures(),
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
			measures: this.selectionMeasures(),
			control: {
				kind: "ring",
				id: "procedural-preview-show",
				label: "Show",
				value: this.showMode,
				options: [
					{ id: "everything", label: "Everything" },
					{ id: "selected", label: "Selected" },
				],
				onSelect: proceduralPlayCmd("setShowMode"),
			},
			status: [{ id: "procedural-preview-geometry-count", text: `${this.geometryHandles.length} geometries` }],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.windowKinds = [
			new WindowKindRuntime(PROCEDURAL_PLAY_WINDOW_KIND_ID, "Flow", PROCEDURAL_PLAY_BODY_KEY_MAIN, undefined, this.flowWindowMeasures(), this.flowWindowEngagement()),
			new WindowKindRuntime(PROCEDURAL_PLAY_WINDOW_KIND_PREVIEW, "Preview", PROCEDURAL_PLAY_BODY_KEY_PREVIEW, undefined, [], this.previewWindowEngagement()),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Procedural play window "${windowKind.id}"`);
		}
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
		if (command === "setFixtureJson") {
			const json = (args as { json?: string }).json;
			if (typeof json === "string" && json !== this.fixtureJson) {
				this.fixtureJson = json;
				this.emit();
			}
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
			if (typeof outputsJson === "string") {
				this.geometryHandles = extractGeometryHandles(outputsJson);
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
			const next = selectionMergeIds(mode, this.selectedNodeIds, ids);
			if (JSON.stringify(next) === JSON.stringify(this.selectedNodeIds)) return;
			this.selectedNodeIds = next;
			this.preselectNodeIds = [];
			this.preselectRemovedNodeIds = [];
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.emit();
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
			this.emit();
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
			const ids = this.geometryHandles.map((entry) => entry.widgetId);
			this.selectedNodeIds = [...new Set(ids)];
			this.preselectNodeIds = [];
			this.preselectRemovedNodeIds = [];
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "deleteSelection") {
			if (!this.selectedNodeIds.length) return;
			this.selectedNodeIds = [];
			this.preselectNodeIds = [];
			this.preselectRemovedNodeIds = [];
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setHover") {
			const id = (args as { id?: string | null }).id;
			const next = typeof id === "string" ? id : null;
			if (next === this.hoveredNodeId) return;
			this.hoveredNodeId = next;
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.emit();
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
		if (command === "setShowMode") {
			const id = (args as { id?: string }).id;
			if (id !== "everything" && id !== "selected") return;
			if (this.showMode === id) return;
			this.showMode = id;
			this.rebuildShellMode();
			this.emit();
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
		buildFlowWindowBody(PROCEDURAL_PLAY_SURFACE_ID, PROCEDURAL_PLAY_CONTROLLER_ID, PROCEDURAL_PLAY_WINDOW_KIND_ID));
	registerWindowBody(PROCEDURAL_PLAY_BODY_KEY_PREVIEW, (_ctx: WindowBodyViewContext) =>
		buildPuzzle3dWindowBody(PROCEDURAL_PLAY_SURFACE_ID_PREVIEW, PROCEDURAL_PLAY_CONTROLLER_ID));
}

export function buildProceduralPlayAppRuntime(controller: ProceduralPlayController): AppRuntime {
	const app = new AppRuntime(PROCEDURAL_PLAY_APP_ID, "Procedural", undefined, controller, PROCEDURAL_PLAY_LAYOUT, []);
	app.defaultModeId = controller.mainMode.id;
	app.addMode(controller.mainMode);
	return app;
}

/** @emoji 🛝 Procedural playground app. */
export class PlaygroundProcedural extends Playground {
	readonly id = PROCEDURAL_PLAY_APP_ID;
	readonly keybindings = [
		{ key: "ctrl+a,meta+a", controllerId: PROCEDURAL_PLAY_CONTROLLER_ID, command: "selectAll" },
		{ key: "Delete", controllerId: PROCEDURAL_PLAY_CONTROLLER_ID, command: "deleteSelection" },
		{ key: "Backspace", controllerId: PROCEDURAL_PLAY_CONTROLLER_ID, command: "deleteSelection" },
	];

	createRuntime(): Platform {
		const runtime = new Platform({ id: this.id });
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

	describe("@procedural/play", () => {
		it("exports default fixture json", () => {
			expect(PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON).toContain("flow.fixture/v1");
		});

		it("controller stores fixture json", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setFixtureJson", { json: '{"schema":"flow.fixture/v1"}' });
			expect(ctrl.getFixtureJson()).toContain("flow.fixture/v1");
		});

		it("kinds tree marks catalogue rows draggable", () => {
			const tree = buildProceduralPlayKindsTree([
				{
					id: "brep-prim3d",
					title: "Brep · Primitives 3D",
					items: [{ kind: "neuron", neuronKind: "brep.prim3d.box", name: "Box", summary: "Axis-aligned box" }],
				},
			]);
			expect(tree.type).toBe("tree");
			const item = tree.sections?.[0]?.items?.[0];
			expect(item?.draggable).toBe(true);
			expect(item?.dragData).toBeDefined();
		});

		it("catalogue revision bumps when sections arrive", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			expect(ctrl.getCatalogueRevision()).toBe(0);
			ctrl.run("setCatalogueSections", {
				sections: [
					{ id: "brep-prim3d", title: "Brep · Primitives 3D", items: [{ kind: "neuron", neuronKind: "brep.prim3d.box", name: "Box", summary: "Box" }] },
					{ id: "brep-curves", title: "Brep · Curves", items: [{ kind: "neuron", neuronKind: "brep.curve.line", name: "Line", summary: "Line edge" }] },
				],
			});
			expect(ctrl.getCatalogueRevision()).toBe(1);
		});

		it("catalogue revision bumps for multiple brep sections", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setCatalogueSections", {
				sections: [
					{ id: "brep-prim3d", title: "Brep · Primitives 3D", items: [] },
					{ id: "brep-solid", title: "Brep · Solid Tools", items: [] },
				],
			});
			expect(ctrl.getCatalogueSections().length).toBe(2);
		});

		it("controller exposes flow and preview window kinds", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			expect(ctrl.mainMode.windowKinds).toHaveLength(2);
			expect(ctrl.mainMode.windowKinds[1]?.id).toBe(PROCEDURAL_PLAY_WINDOW_KIND_PREVIEW);
		});

		it("flow window exposes lod measure group", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			const measures = ctrl.mainMode.windowKinds[0]?.measures ?? [];
			expect(measures.some((measure) => measure.kind === "group" && measure.label === "LOD")).toBe(true);
		});

		it("setShowMode updates preview filter", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setShowMode", { id: "selected" });
			expect(ctrl.getShowMode()).toBe("selected");
		});

		it("setEvalOutputs stores geometry handles per widget", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setEvalOutputs", { outputsJson: JSON.stringify({ box: { geometry: "solid-1" } }) });
			expect(ctrl.getGeometryHandles()).toEqual([{ widgetId: "box", handle: "solid-1" }]);
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
							neuronKinds: [{ id: "brep.box", module: "brep", name: "Box", summary: "Box", inputs: [], outputs: ["brep"] }],
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
// #endregion 🧪Tests

// #region 🔖Boot
if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "procedural") {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootProceduralPlay } = await import("@framework/playground/renderer/react/procedural");
		bootProceduralPlay(new PlaygroundProcedural());
	})();
}
// #endregion 🔖Boot
