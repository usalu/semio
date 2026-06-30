// #region 🧲Header
/** @emoji 🖼️ Raster play harness on `@semio-tech/framework-playground-core`. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	Platform,
	Playground,
	WindowKindRuntime,
	buildRasterWindowBody,
	createDefaultLayout,
	createPlayAppRuntime,
	createProductPlaygroundPlatform,
	isPlaygroundFixtureLocked,
	isPlaygroundNoFixtureId,
	PLAYGROUND_NO_FIXTURE_ID,
	playgroundResolvedFixtureId,
	registerWindowBody,
	FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
	FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
	type AppTools,
	type PlaygroundFixtureCatalog,
	type PlaygroundFixtureHost,
	type ToolLeaf,
	toolCollection,
	type UiNode,
	type UiSectionNode,
	type UiTreeItemNode,
	type UiTreeNode,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import {
	applyRasterEditOp,
	defaultRasterDocument,
	flattenRasterLayers,
	parseRasterDocument,
	rasterDocumentFromJson,
	rasterDocumentToJson,
	rasterPlayBlendModeTreeRowId,
	rasterPlayHoverPayloadFromTreeRowId,
	rasterPlayLayersTreeHighlightedIds,
	rasterPlayLayersTreeRowId,
	rasterPlayMaskTreeRowId,
	type RasterBlendMode,
	type RasterDocument,
	type RasterHoverPayload,
	type RasterKindHover,
	type RasterLayerNode,
	type RasterToolId,
	RASTER_BLEND_MODES,
} from "@semio-tech/raster-core";
import { RASTER_PLAY_FIXTURE_DEFAULT_ID, resolveRasterPlayFixtureSlug } from "./fixture-slugs.ts";

export const RASTER_PLAY_APP_ID = "raster-play";
export const RASTER_PLAY_CONTROLLER_ID = "raster-play";
export const RASTER_PLAY_SURFACE_ID_COMPOSITE = "raster.play.composite/v1";
export const RASTER_PLAY_SURFACE_ID_NAVIGATOR = "raster.play.navigator/v1";
export const RASTER_PLAY_SURFACE_ID_LAYER_PREFIX = "raster.play.layer.";
export const RASTER_PLAY_SURFACE_ID_MASK_PREFIX = "raster.play.mask.";
export const RASTER_PLAY_BODY_KEY_COMPOSITE = "raster.play.composite";
export const RASTER_PLAY_BODY_KEY_NAVIGATOR = "raster.play.navigator";
export const RASTER_PLAY_WINDOW_KIND_COMPOSITE = "raster-composite";
export const RASTER_PLAY_WINDOW_KIND_NAVIGATOR = "raster-navigator";
export const RASTER_PLAY_LAYERS_TAB_ID = "framework.panel.hierarchy";
export const RASTER_PLAY_MASKS_TAB_ID = "raster.panel.masks";
export const RASTER_PLAY_PROPERTIES_TAB_ID = "framework.panel.inspection";

export const RASTER_PLAY_LAYOUT = createDefaultLayout(
	[RASTER_PLAY_WINDOW_KIND_COMPOSITE, RASTER_PLAY_WINDOW_KIND_NAVIGATOR],
	"row",
	[72, 28],
	["Composite", "Navigator"],
);

export { RASTER_PLAY_FIXTURE_DEFAULT_ID, resolveRasterPlayFixtureSlug };

const rasterFixtureModules = import.meta.glob("../fixture/*.raster.json", { eager: true }) as Record<string, { default: unknown }>;

function rasterFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.raster\.json$/, "");
}

function rasterFixtureLabelFromId(id: string): string {
	return id
		.split("-")
		.filter(Boolean)
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

const RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID: Record<string, string> = Object.fromEntries(
	Object.entries(rasterFixtureModules).map(([path, mod]) => {
		const id = rasterFixtureIdFromGlobPath(path);
		const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
		return [id, json];
	}),
);

export const RASTER_PLAY_FIXTURE_OPTIONS: ReadonlyArray<{ readonly id: string; readonly label: string }> = [
	{ id: RASTER_PLAY_FIXTURE_DEFAULT_ID, label: "Default Composite" },
	...Object.keys(RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID)
		.sort()
		.map((id) => ({ id, label: rasterFixtureLabelFromId(id) })),
];

export const RASTER_PLAY_EMPTY_DOCUMENT: RasterDocument = defaultRasterDocument("empty");

function rasterPlayCmd(command: string, args: Record<string, unknown> = {}): { controllerId: string; command: string; args: Record<string, unknown> } {
	return { controllerId: RASTER_PLAY_CONTROLLER_ID, command, args };
}

function rasterPlayHierarchyHoverHandlers(
	onHover: ((payload: RasterHoverPayload) => void) | undefined,
	doc: RasterDocument,
	rowId: string,
): Pick<UiTreeItemNode, "onPointerEnter" | "onPointerLeave"> {
	if (!onHover) return {};
	return {
		onPointerEnter: () => onHover(rasterPlayHoverPayloadFromTreeRowId(doc, rowId)),
		onPointerLeave: () => onHover({ id: null, kind: null }),
	};
}

function rasterPlayLayerTreeItem(
	doc: RasterDocument,
	layer: RasterLayerNode,
	selectedIds: readonly string[],
	hoverSink?: (payload: RasterHoverPayload) => void,
): UiTreeItemNode {
	const rowId = rasterPlayLayersTreeRowId(layer);
	return {
		id: rowId,
		label: layer.name,
		icon: layer.kind === "group" ? "folder" : layer.kind === "adjustment" ? "sliders-horizontal" : "image",
		command: rasterPlayCmd("setSelection", { ids: [layer.id] }),
		children:
			layer.kind === "group"
				? layer.children.map((child) => rasterPlayLayerTreeItem(doc, child, selectedIds, hoverSink))
				: layer.mask?.enabled
					? [
							{
								id: rasterPlayMaskTreeRowId(layer.id),
								label: "Mask",
								icon: "square-dashed",
								command: rasterPlayCmd("setSelection", { ids: [layer.id], focus: "mask" }),
								...rasterPlayHierarchyHoverHandlers(hoverSink, doc, rasterPlayMaskTreeRowId(layer.id)),
							},
						]
					: undefined,
		...rasterPlayHierarchyHoverHandlers(hoverSink, doc, rowId),
	};
}

export function buildRasterPlayLayersTree(
	doc: RasterDocument,
	selectedIds: readonly string[],
	hoveredId: string | null,
	kindHover: RasterKindHover | null,
	hoverSink?: (payload: RasterHoverPayload) => void,
): UiTreeNode {
	const highlightedIds = rasterPlayLayersTreeHighlightedIds(doc, hoveredId, kindHover);
	const selectedTreeIds = selectedIds
		.map((id) => flattenRasterLayers(doc.layers).find((layer) => layer.id === id))
		.filter((layer): layer is RasterLayerNode => Boolean(layer))
		.map((layer) => rasterPlayLayersTreeRowId(layer));
	return {
		type: "tree",
		sections: [
			{
				id: "raster-play-layers",
				label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
				items:
					doc.layers.length > 0
						? doc.layers.map((layer) => rasterPlayLayerTreeItem(doc, layer, selectedIds, hoverSink))
						: [{ id: "raster-play-layers.empty", label: "No layers", icon: "image" as const }],
			},
		],
		selectedIds: selectedTreeIds,
		highlightedIds: [...highlightedIds],
	};
}

export function buildRasterPlayMasksTree(
	doc: RasterDocument,
	selectedIds: readonly string[],
	hoveredId: string | null,
	kindHover: RasterKindHover | null,
	hoverSink?: (payload: RasterHoverPayload) => void,
): UiTreeNode {
	const masks = flattenRasterLayers(doc.layers).filter((layer) => layer.mask?.enabled);
	const highlightedIds = rasterPlayLayersTreeHighlightedIds(doc, hoveredId, kindHover);
	return {
		type: "tree",
		sections: [
			{
				id: "raster-play-masks",
				label: "Masks",
				items:
					masks.length > 0
						? masks.map((layer) => ({
								id: rasterPlayMaskTreeRowId(layer.id),
								label: `${layer.name} Mask`,
								icon: "square-dashed" as const,
								command: rasterPlayCmd("setSelection", { ids: [layer.id], focus: "mask" }),
								...rasterPlayHierarchyHoverHandlers(hoverSink, doc, rasterPlayMaskTreeRowId(layer.id)),
							}))
						: [{ id: "raster-play-masks.empty", label: "No layer masks", icon: "square-dashed" as const }],
			},
		],
		selectedIds: selectedIds.map((id) => rasterPlayMaskTreeRowId(id)),
		highlightedIds: [...highlightedIds],
	};
}

export function buildRasterPlayPropertiesTree(doc: RasterDocument, selectedIds: readonly string[]): UiTreeNode {
	const layer = selectedIds[0] ? flattenRasterLayers(doc.layers).find((row) => row.id === selectedIds[0]) : undefined;
	const items: UiTreeItemNode[] = layer
		? [
				{ id: "raster-play-prop.name", label: `Name: ${layer.name}`, icon: "tag" },
				{ id: "raster-play-prop.opacity", label: `Opacity: ${Math.round(layer.opacity * 100)}%`, icon: "droplet" },
				{ id: "raster-play-prop.blend", label: `Blend: ${layer.blendMode}`, icon: "layers" },
			]
		: [{ id: "raster-play-prop.empty", label: "Select a layer", icon: "mouse-pointer-2" }];
	return {
		type: "tree",
		sections: [{ id: "raster-play-properties", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, items }],
	};
}

export function buildRasterPlayBlendCatalogueTree(hoverSink?: (payload: RasterHoverPayload) => void): UiTreeNode {
	return {
		type: "tree",
		sections: [
			{
				id: "raster-play-blend-catalogue",
				label: "Blend Modes",
				items: RASTER_BLEND_MODES.map((mode) => ({
					id: rasterPlayBlendModeTreeRowId(mode),
					label: mode,
					icon: "blend" as const,
					onPointerEnter: hoverSink ? () => hoverSink({ id: null, kind: { domain: "blendMode", kindId: mode } }) : undefined,
					onPointerLeave: hoverSink ? () => hoverSink({ id: null, kind: null }) : undefined,
				})),
			},
		],
	};
}

export class RasterPlayController extends Controller implements PlaygroundFixtureHost {
	readonly mainMode = new ModeRuntime("main", "Raster", undefined);
	private document: RasterDocument = RASTER_PLAY_EMPTY_DOCUMENT;
	private selectedIds: string[] = [];
	private hoveredId: string | null = null;
	private hoveredKind: RasterKindHover | null = null;
	private interactionRevision = 0;
	private listeners = new Set<() => void>();

	constructor(bus: CommandBus, notifyPlatform: () => void) {
		super(RASTER_PLAY_CONTROLLER_ID, bus, notifyPlatform);
		this.rebuildShellMode();
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = RASTER_PLAY_TOOLS;
		this.mainMode.windowKinds = [
			new WindowKindRuntime(RASTER_PLAY_WINDOW_KIND_COMPOSITE, "Composite", RASTER_PLAY_BODY_KEY_COMPOSITE),
			new WindowKindRuntime(RASTER_PLAY_WINDOW_KIND_NAVIGATOR, "Navigator", RASTER_PLAY_BODY_KEY_NAVIGATOR),
		];
	}

	subscribe(listener: () => void): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	private bump(): void {
		this.interactionRevision += 1;
		for (const listener of this.listeners) listener();
		this.emit();
	}

	getInteractionRevision(): number {
		return this.interactionRevision;
	}

	getDocument(): RasterDocument {
		return this.document;
	}

	getSelectedIds(): readonly string[] {
		return this.selectedIds;
	}

	getHoveredId(): string | null {
		return this.hoveredId;
	}

	getHoveredKind(): RasterKindHover | null {
		return this.hoveredKind;
	}

	getFixtureCatalog(): PlaygroundFixtureCatalog | null {
		if (isPlaygroundFixtureLocked()) return null;
		return {
			activeFixtureId: playgroundResolvedFixtureId(
				this.document.id === "empty" ? PLAYGROUND_NO_FIXTURE_ID : this.document.id,
				RASTER_PLAY_FIXTURE_DEFAULT_ID,
			),
			options: RASTER_PLAY_FIXTURE_OPTIONS,
		};
	}

	run(command: string, args: Record<string, unknown> = {}): void {
		switch (command) {
			case "setActiveFixture": {
				const fixtureId = String(args.fixtureId ?? "");
				if (isPlaygroundNoFixtureId(fixtureId)) {
					this.document = RASTER_PLAY_EMPTY_DOCUMENT;
					this.selectedIds = [];
					this.bump();
					return;
				}
				const json = RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID[fixtureId];
				if (json) {
					this.document = rasterDocumentFromJson(json);
					this.selectedIds = this.document.layers[0] ? [this.document.layers[0].id] : [];
					console.log("[DEBUG] raster fixture loaded", fixtureId);
					this.bump();
				}
				return;
			}
			case "setSelection": {
				const ids = Array.isArray(args.ids) ? args.ids.map(String) : [];
				this.selectedIds = ids;
				this.bump();
				return;
			}
			case "setHover": {
				this.hoveredId = typeof args.id === "string" ? args.id : null;
				this.hoveredKind = (args.kind as RasterKindHover | null) ?? null;
				this.bump();
				return;
			}
			case "setActiveTool": {
				const tool = String(args.tool ?? "") as RasterToolId;
				this.document = applyRasterEditOp(this.document, { op: "setActiveTool", tool });
				this.bump();
				return;
			}
			case "setLayerVisible": {
				this.document = applyRasterEditOp(this.document, {
					op: "setLayerVisible",
					layerId: String(args.layerId),
					visible: args.visible !== false,
				});
				this.bump();
				return;
			}
			case "setLayerBlendMode": {
				this.document = applyRasterEditOp(this.document, {
					op: "setLayerBlendMode",
					layerId: String(args.layerId),
					blendMode: String(args.blendMode) as RasterBlendMode,
				});
				this.bump();
				return;
			}
			case "setCamera": {
				const camera = args.camera as RasterDocument["camera"];
				if (camera) {
					this.document = applyRasterEditOp(this.document, { op: "setCamera", camera });
					this.bump();
				}
				return;
			}
			case "selectAll": {
				this.selectedIds = flattenRasterLayers(this.document.layers).map((layer) => layer.id);
				this.bump();
				return;
			}
			default:
				return;
		}
	}
}

function rasterPlayTool(id: string, label: string, iconId: string, command: string, args?: Record<string, unknown>): ToolLeaf {
	return { id, kind: "button", label, iconId, controllerId: RASTER_PLAY_CONTROLLER_ID, command, args };
}

function selectionTools(): readonly ToolLeaf[] {
	return [
		rasterPlayTool("selectMarquee", "Marquee", "square-dashed", "setActiveTool", { tool: "selectMarquee" }),
		rasterPlayTool("selectLasso", "Lasso", "lasso", "setActiveTool", { tool: "selectLasso" }),
		rasterPlayTool("selectWand", "Wand", "wand-2", "setActiveTool", { tool: "selectWand" }),
	];
}

function paintTools(): readonly ToolLeaf[] {
	return [
		rasterPlayTool("paintBrush", "Brush", "paintbrush", "setActiveTool", { tool: "paintBrush" }),
		rasterPlayTool("paintEraser", "Eraser", "eraser", "setActiveTool", { tool: "paintEraser" }),
		rasterPlayTool("paintClone", "Clone", "copy", "setActiveTool", { tool: "paintClone" }),
	];
}

function transformTools(): readonly ToolLeaf[] {
	return [
		rasterPlayTool("transformMove", "Move", "move", "setActiveTool", { tool: "transformMove" }),
		rasterPlayTool("transformScale", "Scale", "scaling", "setActiveTool", { tool: "transformScale" }),
		rasterPlayTool("transformRotate", "Rotate", "rotate-cw", "setActiveTool", { tool: "transformRotate" }),
	];
}

export const RASTER_PLAY_TOOLS: AppTools = [
	toolCollection("selection", "mouse-pointer-2", selectionTools()),
	toolCollection("paint", "paintbrush", paintTools()),
	toolCollection("transform", "move", transformTools()),
	toolCollection("adjust", "sliders-horizontal", [
		rasterPlayTool("adjust-brightness", "Brightness", "sun", "setActiveTool", { tool: "selectMarquee" }),
	]),
	toolCollection("filter", "sparkles", [
		rasterPlayTool("filter-blur", "Blur", "blur", "setActiveTool", { tool: "selectMarquee" }),
	]),
];

export function buildRasterPlayAppRuntime(ctrl: RasterPlayController): AppRuntime {
	return createPlayAppRuntime(RASTER_PLAY_APP_ID, "Raster", ctrl, RASTER_PLAY_LAYOUT, ctrl.mainMode);
}

export function registerRasterPlayDeclarativeBodies(): void {
	registerWindowBody(RASTER_PLAY_BODY_KEY_COMPOSITE, () =>
		buildRasterWindowBody(RASTER_PLAY_SURFACE_ID_COMPOSITE, RASTER_PLAY_CONTROLLER_ID, "composite", "composite"));
	registerWindowBody(RASTER_PLAY_BODY_KEY_NAVIGATOR, () =>
		buildRasterWindowBody(RASTER_PLAY_SURFACE_ID_NAVIGATOR, RASTER_PLAY_CONTROLLER_ID, "navigator", "navigator"));
}

export class PlaygroundRaster extends Playground {
	readonly id = RASTER_PLAY_APP_ID;
	readonly keybindings = [{ key: "ctrl+a,meta+a", controllerId: RASTER_PLAY_CONTROLLER_ID, command: "selectAll" }];

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new RasterPlayController(runtime.commandBus, () => runtime.notify());
		const resolved = playgroundResolvedFixtureId("default");
		const fixtureJson = RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID[resolved];
		if (fixtureJson) {
			ctrl.run("setActiveFixture", { fixtureId: resolved });
		}
		runtime.addApp(buildRasterPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerRasterPlayDeclarativeBodies();
	}
}

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "raster") {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootRasterPlay } = await import("@semio-tech/framework-playground-renderer-react/raster");
		bootRasterPlay(new PlaygroundRaster());
	})();
}

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("RASTER_PLAY_FIXTURE_OPTIONS", () => {
		it("includes default fixture", () => {
			expect(RASTER_PLAY_FIXTURE_OPTIONS.some((row) => row.id === RASTER_PLAY_FIXTURE_DEFAULT_ID)).toBe(true);
		});
	});

	describe("buildRasterPlayAppRuntime", () => {
		it("wires composite and navigator window kinds", () => {
			const bus = new CommandBus();
			const ctrl = new RasterPlayController(bus, () => {});
			const app = buildRasterPlayAppRuntime(ctrl);
			expect(app.windowKinds.map((kind) => kind.id)).toEqual([
				RASTER_PLAY_WINDOW_KIND_COMPOSITE,
				RASTER_PLAY_WINDOW_KIND_NAVIGATOR,
			]);
		});
	});
	describe("buildRasterPlayMasksTree", () => {
		it("keeps a placeholder row when the fixture has no masks", () => {
			const doc = rasterDocumentFromJson(RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID.paint!);
			const tree = buildRasterPlayMasksTree(doc, [], null, null);
			expect(tree.sections[0]?.items).toEqual([
				expect.objectContaining({ id: "raster-play-masks.empty", label: "No layer masks" }),
			]);
		});
	});

	describe("buildRasterPlayLayersTree", () => {
		it("builds layer rows", () => {
			const doc = rasterDocumentFromJson(RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID.default!);
			const tree = buildRasterPlayLayersTree(doc, [], null, null);
			expect(tree.sections[0]?.items.length).toBeGreaterThan(0);
		});
	});

	describe("RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID", () => {
		for (const fixtureId of Object.keys(RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID)) {
			it(`loads ${fixtureId} fixture into trees without empty sections`, () => {
				const doc = rasterDocumentFromJson(RASTER_PLAY_FILE_FIXTURE_JSON_BY_ID[fixtureId]!);
				const layers = buildRasterPlayLayersTree(doc, [], null, null);
				const masks = buildRasterPlayMasksTree(doc, [], null, null);
				const properties = buildRasterPlayPropertiesTree(doc, doc.layers[0] ? [doc.layers[0].id] : []);
				expect(layers.sections[0]?.items.length).toBeGreaterThan(0);
				expect(masks.sections[0]?.items.length).toBeGreaterThan(0);
				expect(properties.sections[0]?.items.length).toBeGreaterThan(0);
			});
		}
	});
}
// #endregion 🧪Tests
