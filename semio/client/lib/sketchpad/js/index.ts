// #region 🧲Header
// 2024-2026 Ueli Saluz <ueli@semio-tech.com>
// Render-agnostic sketchpad product: {@link Platform} apps, {@link Component} models, kit registry bridge.
// #endregion 🧲Header

//#region 🔌Adapters
import type { Design, Kit, Type } from "@semio/js";
import {
	CommandBus,
	Component,
	Controller,
	Panel,
	Platform,
	PluginHost,
	Table,
	buildCadWindowBody,
	buildPanelWindowBody,
	buildPuzzle2dWindowBody,
	buildPuzzle5dWindowBody,
	buildTableWindowBody,
	createDefaultLayout,
	createTabStackLayout,
	registerPlatformComponent,
	registerSidePanelBody,
	registerWindowBody,
	type CadModel,
	type ComponentKind,
	type PanelModel,
	type PlatformSpec,
	type PluginManifest,
	type PluginModule,
	type Puzzle2dModel,
	type Puzzle5dModel,
	type TableModel,
	type UiNode,
	type WindowBodyViewContext,
} from "@framework/platform/core";
//#endregion 🔌Adapters

//#region 🔖KitRegistryBridge
/** @emoji 🗄️ Host store snapshot used by open-kit rows (wired by the host, not sketchpad). */
export type KitHostStore = {
	getSnapshot: () => { kit: Kit };
	subscribe?: (listener: () => void) => () => void;
	replace?: (next: Kit) => void;
} & Record<string, unknown>;

/** @emoji 🌉 Registry row exposed to sketchpad {@link Component} models. */
export type SketchpadKitRegistryEntry = {
	readonly store?: KitHostStore;
	readonly kind?: string;
	readonly persistence?: { readonly kind?: string };
};

/** @emoji 🌉 Ordered open-kit registry (host installs via {@link setSketchpadKitRegistryBridge}). */
export type SketchpadKitRegistryBridge = {
	list(): readonly string[];
	get(id: string): SketchpadKitRegistryEntry | undefined;
	subscribe?(listener: () => void) => () => void;
};

let sketchpadKitRegistryBridge: SketchpadKitRegistryBridge | null = null;

/** @emoji 🔌 Host hook: connect kit registry before models read open kits. */
export function setSketchpadKitRegistryBridge(bridge: SketchpadKitRegistryBridge | null): void {
	sketchpadKitRegistryBridge = bridge;
}

/** @emoji 🔍 Active kit registry for sketchpad platform components. */
export function getKitRegistryBridge(): SketchpadKitRegistryBridge | null {
	return sketchpadKitRegistryBridge;
}
//#endregion 🔖KitRegistryBridge

//#region 🔖SketchpadRouteScope
/** @emoji 🧭 Kit/design/type ids parsed from a sketchpad URL path (render-agnostic). */
export function parseSketchpadRouteScopeFromPath(path: string): {
	readonly kitId: string | null;
	readonly designId: string | null;
	readonly typeId: string | null;
	readonly docsPath: string;
} {
	const pathParts = path.split("/").filter((part) => part.length > 0);
	const isUuidPattern = (value: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(value);
	if (pathParts[0] === "docs") {
		const docsPath = pathParts.slice(1).join("/") || "index";
		return { kitId: null, designId: null, typeId: null, docsPath };
	}
	if (pathParts[0] !== "kits") {
		return { kitId: null, designId: null, typeId: null, docsPath: "index" };
	}
	const kitId = pathParts[1] && isUuidPattern(pathParts[1]) ? pathParts[1] : null;
	const designId = pathParts[2] === "designs" && pathParts[3] && isUuidPattern(pathParts[3]) ? pathParts[3] : null;
	const typeId = pathParts[2] === "types" && pathParts[3] && isUuidPattern(pathParts[3]) ? pathParts[3] : null;
	return { kitId, designId, typeId, docsPath: "index" };
}

/** @emoji 🧭 Maps a location path to the sketchpad {@link Platform} active app id. */
export function sketchpadAppIdFromPath(path: string): string {
	const pathParts = path.split("/").filter((part) => part.length > 0);
	const isUuidPattern = (value: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(value);
	if (pathParts[0] === "docs") return SKETCHPAD_DOCS_APP_ID;
	if (pathParts[0] === "feedback") return SKETCHPAD_FEEDBACK_APP_ID;
	if (pathParts[0] !== "kits") return SKETCHPAD_HOME_APP_ID;
	if (pathParts.length >= 4 && pathParts[2] === "designs" && isUuidPattern(pathParts[3] ?? "")) return SKETCHPAD_DESIGN_APP_ID;
	if (pathParts.length >= 4 && pathParts[2] === "types" && isUuidPattern(pathParts[3] ?? "")) return SKETCHPAD_TYPE_APP_ID;
	if (pathParts.length >= 2 && isUuidPattern(pathParts[1] ?? "")) return SKETCHPAD_KIT_APP_ID;
	return SKETCHPAD_HOME_APP_ID;
}
//#endregion 🔖SketchpadRouteScope

export const SKETCHPAD_SHELL_CONTROLLER_ID = "semio.sketchpad.shell";
const SKETCHPAD_EXTENSION_ID = "semio.sketchpad.builtin";
export const SKETCHPAD_HOME_APP_ID = "home";
export const SKETCHPAD_KIT_APP_ID = "kit";
export const SKETCHPAD_DESIGN_APP_ID = "design";
export const SKETCHPAD_TYPE_APP_ID = "type";
export const SKETCHPAD_DOCS_APP_ID = "docs";
export const SKETCHPAD_FEEDBACK_APP_ID = "feedback";
const SKETCHPAD_BODY_HOME = "semio.sketchpad.window.home";
const SKETCHPAD_BODY_KIT_TABLE = "semio.sketchpad.window.kit.table";
const SKETCHPAD_BODY_KIT_DIAGRAM = "semio.sketchpad.window.kit.diagram";
const SKETCHPAD_BODY_DESIGN_SCENE = "semio.sketchpad.window.design.scene";
const SKETCHPAD_BODY_DESIGN_DIAGRAM = "semio.sketchpad.window.design.diagram";
const SKETCHPAD_BODY_TYPE = "semio.sketchpad.window.type";
const SKETCHPAD_BODY_DOCS = "semio.sketchpad.window.docs";
const SKETCHPAD_BODY_FEEDBACK = "semio.sketchpad.window.feedback";
const SKETCHPAD_SURFACE_KIT_TABLE = "semio.sketchpad.surface.kit.table/v1";
const SKETCHPAD_SURFACE_KIT_DIAGRAM = "semio.sketchpad.surface.kit.diagram/v1";
const SKETCHPAD_SURFACE_DESIGN_SCENE = "semio.sketchpad.surface.design.scene/v1";
const SKETCHPAD_SURFACE_DESIGN_DIAGRAM = "semio.sketchpad.surface.design.diagram/v1";
const SKETCHPAD_SURFACE_PANEL_MAIN = "semio.sketchpad.surface.panel.main/v1";
const SKETCHPAD_SURFACE_HOME_TABLE = "semio.sketchpad.surface.home.table/v1";
const SKETCHPAD_SURFACE_TYPE_SCENE = "semio.sketchpad.surface.type.scene/v1";
const SKETCHPAD_SURFACE_DOCS_PAGE = "semio.sketchpad.surface.docs.page/v1";
const SKETCHPAD_SURFACE_FEEDBACK_FORM = "semio.sketchpad.surface.feedback.form/v1";
const SKETCHPAD_PANEL_WORKBENCH_BODY = "semio.sketchpad.panel.workbench";
const SKETCHPAD_PANEL_DETAILS_BODY = "semio.sketchpad.panel.details";

//#region 🔖SketchpadPlatformComponents
abstract class SketchpadRoutedComponent<TModel> extends Component<TModel> {
	protected route = parseSketchpadRouteScopeFromPath("/");
	private readonly detachRoute: () => void;
	private readonly detachKitRegistry?: () => void;
	private detachKitStore?: () => void;

	constructor(componentKind: ComponentKind, surfaceId: string, controllerId: string, initialModel: TModel, platform: Platform) {
		super(componentKind, surfaceId, controllerId, initialModel);
		this.route = parseSketchpadRouteScopeFromPath(platform.uri.split("?")[0] ?? "/");
		this.detachRoute = platform.subscribe(() => {
			const nextRoute = parseSketchpadRouteScopeFromPath(platform.uri.split("?")[0] ?? "/");
			if (
				nextRoute.kitId !== this.route.kitId ||
				nextRoute.designId !== this.route.designId ||
				nextRoute.typeId !== this.route.typeId ||
				nextRoute.docsPath !== this.route.docsPath
			) {
				this.route = nextRoute;
				this.attachActiveKitStore();
				this.refresh();
			}
		});
		const registry = getKitRegistryBridge();
		if (registry?.subscribe) {
			this.detachKitRegistry = registry.subscribe(() => this.refresh());
		}
		this.attachActiveKitStore();
	}

	protected attachActiveKitStore(): void {
		this.detachKitStore?.();
		this.detachKitStore = undefined;
		const { kitId } = this.route;
		if (!kitId) return;
		const store = getKitRegistryBridge()?.get(kitId)?.store;
		if (store?.subscribe) {
			this.detachKitStore = store.subscribe(() => this.refresh());
		}
	}

	dispose(): void {
		this.detachRoute();
		this.detachKitRegistry?.();
		this.detachKitStore?.();
	}
}

/** @emoji 🏠 Home kits table backed by the kit registry bridge. */
export class SketchpadHomeTable extends Table {
	constructor(platform: Platform) {
		super(SKETCHPAD_SURFACE_HOME_TABLE, SKETCHPAD_SHELL_CONTROLLER_ID);
		platform.subscribe(() => this.refresh());
		const registry = getKitRegistryBridge();
		if (registry?.subscribe) {
			registry.subscribe(() => this.refresh());
		}
	}

	override buildModel(): TableModel {
		const registry = getKitRegistryBridge();
		const ids = registry?.list() ?? [];
		return {
			columns: [
				{ id: "name", label: "Name" },
				{ id: "kind", label: "Kind" },
			],
			rows: ids.map((id) => {
				let name = id;
				let kind = "";
				try {
					const snapshot = registry?.get(id)?.store?.getSnapshot?.();
					const kit = snapshot?.kit;
					if (kit?.name) name = kit.name;
					kind = registry?.get(id)?.kind ?? "";
				} catch {
					/* registry row may still be opening */
				}
				return { id, cells: { name, kind }, navigateUri: `/kits/${id}` };
			}),
			emptyMessage: "No kits open — use Open to add kits",
		};
	}
}

/** @emoji 📊 Active kit table surface. */
export class SketchpadKitTable extends SketchpadRoutedComponent<TableModel> {
	constructor(platform: Platform) {
		super("table", SKETCHPAD_SURFACE_KIT_TABLE, SKETCHPAD_SHELL_CONTROLLER_ID, { columns: [], rows: [] }, platform);
	}

	override buildModel(): TableModel {
		const { kitId } = this.route;
		if (!kitId) {
			return { columns: [], rows: [], emptyMessage: "Open a kit to view the table" };
		}
		const registry = getKitRegistryBridge();
		const store = registry?.get(kitId)?.store;
		if (!store) {
			return { columns: [], rows: [], emptyMessage: "Kit loading…" };
		}
		const kit = store.getSnapshot().kit;
		const types = kit.types ?? [];
		const designs = kit.designs ?? [];
		return {
			columns: [
				{ id: "name", label: "Name" },
				{ id: "kind", label: "Kind" },
			],
			rows: [
				...types
					.filter((t): t is Type => typeof t === "object" && t !== null && "id" in t)
					.map((t) => ({
						id: `type:${t.id}`,
						cells: { name: t.name ?? t.id, kind: "type" },
						navigateUri: `/kits/${kitId}/types/${t.id}`,
					})),
				...designs
					.filter((d): d is Design => typeof d === "object" && d !== null && "id" in d)
					.map((d) => ({
						id: `design:${d.id}`,
						cells: { name: d.name ?? d.id, kind: "design" },
						navigateUri: `/kits/${kitId}/designs/${d.id}`,
					})),
			],
			emptyMessage: "No types or designs in this kit",
		};
	}
}

/** @emoji 📋 Kit diagram surface (topology summary as nodes). */
export class SketchpadKitDiagram extends SketchpadRoutedComponent<Puzzle2dModel> {
	constructor(platform: Platform) {
		super("puzzle2d", SKETCHPAD_SURFACE_KIT_DIAGRAM, SKETCHPAD_SHELL_CONTROLLER_ID, { nodes: [], edges: [] }, platform);
	}

	override buildModel(): Puzzle2dModel {
		const { kitId } = this.route;
		if (!kitId) {
			return { nodes: [], edges: [], emptyMessage: "Open a kit to view the diagram" };
		}
		const registry = getKitRegistryBridge();
		const store = registry?.get(kitId)?.store;
		if (!store) {
			return { nodes: [], edges: [], emptyMessage: "Kit loading…" };
		}
		const kit = store.getSnapshot().kit;
		const nodes = (kit.types ?? []).map((t, index) => ({
			id: `type:${t.name}`,
			label: t.name,
			x: (index % 6) * 120,
			y: Math.floor(index / 6) * 80,
		}));
		return { nodes, edges: [], emptyMessage: nodes.length ? undefined : "No types to diagram" };
	}
}

/** @emoji 🎬 Design scene (5D volume). */
export class SketchpadDesignScene extends SketchpadRoutedComponent<Puzzle5dModel> {
	constructor(platform: Platform) {
		super(
			"puzzle5d",
			SKETCHPAD_SURFACE_DESIGN_SCENE,
			SKETCHPAD_SHELL_CONTROLLER_ID,
			{ presentation: "volume", instanceId: SKETCHPAD_SURFACE_DESIGN_SCENE },
			platform,
		);
	}

	override buildModel(): Puzzle5dModel {
		const { kitId, designId } = this.route;
		if (!kitId || !designId) {
			return { presentation: "volume", instanceId: SKETCHPAD_SURFACE_DESIGN_SCENE, emptyMessage: "Open a design to view the scene" };
		}
		return { presentation: "volume", instanceId: `${kitId}:${designId}:scene` };
	}
}

/** @emoji 📐 Design diagram (5D flat). */
export class SketchpadDesignDiagram extends SketchpadRoutedComponent<Puzzle5dModel> {
	constructor(platform: Platform) {
		super(
			"puzzle5d",
			SKETCHPAD_SURFACE_DESIGN_DIAGRAM,
			SKETCHPAD_SHELL_CONTROLLER_ID,
			{ presentation: "flat", instanceId: SKETCHPAD_SURFACE_DESIGN_DIAGRAM },
			platform,
		);
	}

	override buildModel(): Puzzle5dModel {
		const { kitId, designId } = this.route;
		if (!kitId || !designId) {
			return { presentation: "flat", instanceId: SKETCHPAD_SURFACE_DESIGN_DIAGRAM, emptyMessage: "Open a design to view the diagram" };
		}
		return { presentation: "flat", instanceId: `${kitId}:${designId}:diagram` };
	}
}

/** @emoji 📐 Type CAD surface. */
export class SketchpadTypeCad extends SketchpadRoutedComponent<CadModel> {
	constructor(platform: Platform) {
		super("cad", SKETCHPAD_SURFACE_TYPE_SCENE, SKETCHPAD_SHELL_CONTROLLER_ID, {}, platform);
	}

	override buildModel(): CadModel {
		const { kitId, typeId } = this.route;
		if (!kitId || !typeId) {
			return { emptyMessage: "Open a type to view the CAD scene" };
		}
		return { instanceId: `${kitId}:${typeId}` };
	}
}

/** @emoji 📄 Docs panel surface. */
export class SketchpadDocsPanel extends SketchpadRoutedComponent<PanelModel> {
	constructor(platform: Platform) {
		super("panel", SKETCHPAD_SURFACE_DOCS_PAGE, SKETCHPAD_SHELL_CONTROLLER_ID, { body: { type: "text", value: "Docs" } }, platform);
	}

	override buildModel(): PanelModel {
		return {
			body: {
				type: "stack",
				direction: "vertical",
				padding: "standard",
				children: [
					{ type: "text", value: `Docs · ${this.route.docsPath}`, emphasize: true },
					{ type: "text", value: "Navigate to /docs/… to browse documentation." },
				],
			},
		};
	}
}

/** @emoji 💬 Feedback panel surface. */
export class SketchpadFeedbackPanel extends Panel {
	constructor(_platform: Platform) {
		super(SKETCHPAD_SURFACE_FEEDBACK_FORM, SKETCHPAD_SHELL_CONTROLLER_ID, {
			body: {
				type: "stack",
				direction: "vertical",
				padding: "standard",
				children: [
					{ type: "text", value: "Feedback", emphasize: true },
					{ type: "text", value: "Send feedback from the footer or command palette." },
				],
			},
		});
	}
}

/** @emoji 🧩 Workbench side panel placeholder. */
class SketchpadWorkbenchPanel extends Panel {
	constructor() {
		super(SKETCHPAD_SURFACE_PANEL_MAIN, SKETCHPAD_SHELL_CONTROLLER_ID, {
			body: { type: "text", value: "Workbench panel" },
		});
	}
}

class SketchpadPlatformComponents {
	readonly components: readonly Component<unknown>[];

	constructor(platform: Platform) {
		this.components = [
			new SketchpadHomeTable(platform),
			new SketchpadKitTable(platform),
			new SketchpadKitDiagram(platform),
			new SketchpadDesignScene(platform),
			new SketchpadDesignDiagram(platform),
			new SketchpadTypeCad(platform),
			new SketchpadDocsPanel(platform),
			new SketchpadFeedbackPanel(platform),
			new SketchpadWorkbenchPanel(),
		];
		for (const component of this.components) {
			registerPlatformComponent(platform, component);
			component.refresh();
		}
		platform.subscribe(() => {
			for (const component of this.components) {
				component.refresh();
			}
		});
	}
}
//#endregion 🔖SketchpadPlatformComponents

/** @emoji 🧭 Routes sketchpad navigation and panel chrome through {@link CommandBus}. */
export class SketchpadShellController extends Controller {
	navigationPath = "/";
	panelVisibility = { leftSidePanel: false, rightSidePanel: false };

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(SKETCHPAD_SHELL_CONTROLLER_ID, commandBus, hostNotify);
	}

	override run(command: string, args?: unknown): void {
		switch (command) {
			case "setNavigation": {
				this.navigationPath = (args as { path: string }).path;
				break;
			}
			case "togglePanel": {
				const panel = (args as { panel: "leftSidePanel" | "rightSidePanel" }).panel;
				this.panelVisibility = { ...this.panelVisibility, [panel]: !this.panelVisibility[panel] };
				break;
			}
			default:
				break;
		}
		this.emit();
	}
}

let sketchpadPlatformSingleton: Platform | null = null;
let sketchpadPluginHostSingleton: PluginHost | null = null;
let sketchpadPlatformReady: Promise<Platform> | null = null;
let sketchpadBodiesRegistered = false;
let sketchpadPopstateBound = false;

function buildSketchpadExtensionManifest(): PluginManifest {
	return {
		id: SKETCHPAD_EXTENSION_ID,
		label: "Semio Sketchpad",
		contributes: {
			apps: [
				{
					id: SKETCHPAD_HOME_APP_ID,
					label: "Home",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [{ id: "home-main", label: "Home", bodyKey: SKETCHPAD_BODY_HOME }],
					defaultLayout: createTabStackLayout(["home-main"], ["Home"]),
					leftTabs: [{ id: "workbench", iconId: "semio.sketchpad.icon.workbench", bodyKey: SKETCHPAD_PANEL_WORKBENCH_BODY }],
					rightTabs: [{ id: "details", iconId: "semio.sketchpad.icon.details", bodyKey: SKETCHPAD_PANEL_DETAILS_BODY }],
				},
				{
					id: SKETCHPAD_KIT_APP_ID,
					label: "Kit",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [
						{ id: "table", label: "Table", bodyKey: SKETCHPAD_BODY_KIT_TABLE },
						{ id: "diagram", label: "Diagram", bodyKey: SKETCHPAD_BODY_KIT_DIAGRAM },
					],
					defaultLayout: createDefaultLayout(["table", "diagram"], "row", [50, 50], ["Table", "Diagram"]),
					leftTabs: [{ id: "workbench", iconId: "semio.sketchpad.icon.workbench", bodyKey: SKETCHPAD_PANEL_WORKBENCH_BODY }],
					rightTabs: [{ id: "details", iconId: "semio.sketchpad.icon.details", bodyKey: SKETCHPAD_PANEL_DETAILS_BODY }],
				},
				{
					id: SKETCHPAD_DESIGN_APP_ID,
					label: "Design",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [
						{ id: "scene", label: "Scene", bodyKey: SKETCHPAD_BODY_DESIGN_SCENE },
						{ id: "diagram", label: "Diagram", bodyKey: SKETCHPAD_BODY_DESIGN_DIAGRAM },
					],
					defaultLayout: createDefaultLayout(["scene", "diagram"], "row", [60, 40], ["Scene", "Diagram"]),
					leftTabs: [{ id: "workbench", iconId: "semio.sketchpad.icon.workbench", bodyKey: SKETCHPAD_PANEL_WORKBENCH_BODY }],
					rightTabs: [{ id: "details", iconId: "semio.sketchpad.icon.details", bodyKey: SKETCHPAD_PANEL_DETAILS_BODY }],
				},
				{
					id: SKETCHPAD_TYPE_APP_ID,
					label: "Type",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [{ id: "type-main", label: "Type", bodyKey: SKETCHPAD_BODY_TYPE }],
					defaultLayout: createTabStackLayout(["type-main"], ["Type"]),
					leftTabs: [{ id: "workbench", iconId: "semio.sketchpad.icon.workbench", bodyKey: SKETCHPAD_PANEL_WORKBENCH_BODY }],
					rightTabs: [{ id: "details", iconId: "semio.sketchpad.icon.details", bodyKey: SKETCHPAD_PANEL_DETAILS_BODY }],
				},
				{
					id: SKETCHPAD_DOCS_APP_ID,
					label: "Docs",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [{ id: "docs-main", label: "Docs", bodyKey: SKETCHPAD_BODY_DOCS }],
					defaultLayout: createTabStackLayout(["docs-main"], ["Docs"]),
				},
				{
					id: SKETCHPAD_FEEDBACK_APP_ID,
					label: "Feedback",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [{ id: "feedback-main", label: "Feedback", bodyKey: SKETCHPAD_BODY_FEEDBACK }],
					defaultLayout: createTabStackLayout(["feedback-main"], ["Feedback"]),
				},
			],
		},
	};
}

function declarativePanelMain(_ctx: WindowBodyViewContext): UiNode {
	return buildPanelWindowBody(SKETCHPAD_SURFACE_PANEL_MAIN, SKETCHPAD_SHELL_CONTROLLER_ID);
}

function registerSketchpadWindowBodies(): void {
	if (sketchpadBodiesRegistered) return;
	sketchpadBodiesRegistered = true;
	registerWindowBody(SKETCHPAD_BODY_HOME, () =>
		buildTableWindowBody(SKETCHPAD_SURFACE_HOME_TABLE, SKETCHPAD_SHELL_CONTROLLER_ID, "home-main"),
	);
	registerWindowBody(SKETCHPAD_BODY_KIT_TABLE, () =>
		buildTableWindowBody(SKETCHPAD_SURFACE_KIT_TABLE, SKETCHPAD_SHELL_CONTROLLER_ID, "table"),
	);
	registerWindowBody(SKETCHPAD_BODY_KIT_DIAGRAM, () =>
		buildPuzzle2dWindowBody(SKETCHPAD_SURFACE_KIT_DIAGRAM, SKETCHPAD_SHELL_CONTROLLER_ID, "diagram"),
	);
	registerWindowBody(SKETCHPAD_BODY_DESIGN_SCENE, () =>
		buildPuzzle5dWindowBody(SKETCHPAD_SURFACE_DESIGN_SCENE, SKETCHPAD_SHELL_CONTROLLER_ID, "scene"),
	);
	registerWindowBody(SKETCHPAD_BODY_DESIGN_DIAGRAM, () =>
		buildPuzzle5dWindowBody(SKETCHPAD_SURFACE_DESIGN_DIAGRAM, SKETCHPAD_SHELL_CONTROLLER_ID, "diagram"),
	);
	registerWindowBody(SKETCHPAD_BODY_TYPE, () => buildCadWindowBody(SKETCHPAD_SURFACE_TYPE_SCENE, SKETCHPAD_SHELL_CONTROLLER_ID));
	registerWindowBody(SKETCHPAD_BODY_DOCS, () => buildPanelWindowBody(SKETCHPAD_SURFACE_DOCS_PAGE, SKETCHPAD_SHELL_CONTROLLER_ID));
	registerWindowBody(SKETCHPAD_BODY_FEEDBACK, () => buildPanelWindowBody(SKETCHPAD_SURFACE_FEEDBACK_FORM, SKETCHPAD_SHELL_CONTROLLER_ID));
	registerSidePanelBody(SKETCHPAD_PANEL_WORKBENCH_BODY, declarativePanelMain);
	registerSidePanelBody(SKETCHPAD_PANEL_DETAILS_BODY, declarativePanelMain);
}

function applySketchpadUri(platform: Platform, uri: string): void {
	const path = uri.split("?")[0] ?? "/";
	platform.uri = uri;
	platform.activeAppId = sketchpadAppIdFromPath(path);
	platform.notify();
}

function wireSketchpadBrowserNavigation(platform: Platform): void {
	platform.onNavigate = (uri) => {
		if (typeof window !== "undefined" && window.location.pathname + window.location.search !== uri) {
			window.history.pushState(null, "", uri);
		}
		applySketchpadUri(platform, uri);
	};
	if (typeof window === "undefined" || sketchpadPopstateBound) return;
	sketchpadPopstateBound = true;
	window.addEventListener("popstate", () => {
		const uri = `${window.location.pathname}${window.location.search}`;
		applySketchpadUri(platform, uri);
	});
}

const SKETCHPAD_PLATFORM_SPEC: PlatformSpec = {
	id: "semio.sketchpad",
	name: "Semio Sketchpad",
	defaultActiveAppId: SKETCHPAD_HOME_APP_ID,
};

/** @emoji 🧱 Builds the sketchpad {@link Platform} (apps, window bodies, {@link Component} registry). */
export async function buildSketchpadPlatform(): Promise<Platform> {
	registerSketchpadWindowBodies();
	const platform = new Platform(SKETCHPAD_PLATFORM_SPEC);
	const controller = new SketchpadShellController(platform.commandBus, () => platform.notify());
	const host = new PluginHost(platform);
	host.register(buildSketchpadExtensionManifest(), {
		id: SKETCHPAD_EXTENSION_ID,
		activate() {},
	} satisfies PluginModule);
	await host.activateAll((controllerId) => (controllerId === SKETCHPAD_SHELL_CONTROLLER_ID ? controller : undefined));
	new SketchpadPlatformComponents(platform);
	wireSketchpadBrowserNavigation(platform);
	if (typeof window !== "undefined" && window.location) {
		applySketchpadUri(platform, `${window.location.pathname}${window.location.search}`);
	} else {
		platform.activeAppId = SKETCHPAD_HOME_APP_ID;
		platform.notify();
	}
	sketchpadPlatformSingleton = platform;
	sketchpadPluginHostSingleton = host;
	return platform;
}

/** @emoji 🚀 Ensures the sketchpad {@link Platform} is initialized once per session. */
export async function ensureSketchpadPlatform(): Promise<Platform> {
	if (sketchpadPlatformSingleton) return sketchpadPlatformSingleton;
	if (!sketchpadPlatformReady) {
		sketchpadPlatformReady = buildSketchpadPlatform();
	}
	return sketchpadPlatformReady;
}

/** @emoji 🔍 Returns the live sketchpad {@link Platform}, if built. */
export function getSketchpadPlatform(): Platform | null {
	return sketchpadPlatformSingleton;
}

/** @emoji 🚀 @deprecated Use {@link ensureSketchpadPlatform}. */
export const ensureSketchpadDeclarativeShell = ensureSketchpadPlatform;

/** @emoji 🔍 @deprecated Use {@link getSketchpadPlatform}. */
export const getSketchpadProductRuntime = getSketchpadPlatform;
