// #region 🧲Header
/** @emoji 🖥️ S play app — unified designer OS shell. */
// #endregion 🧲Header

export * from "./internal.ts";

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	WindowKindRuntime,
	buildSWindowBody,
	createDefaultLayout,
	createPlayAppRuntime,
	createTabStackLayout,
	registerWindowBody,
	type CommandDescriptor,
	type AppTools,
	type ToolLeaf,
	type WindowEngagement,
	type WindowMeasure,
	toolCollection,
	enforcePlaygroundWindowEngagementInput,
	type UiNode,
	type UiTreeNode,
	uiDeclarativeSectionsToTree,
	uiInspectorAllEqual,
	FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
	FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
	buildWriterWindowBody,
  createPlaygroundApp,
  createProductPlaygroundPlatform,
  eagerPlayExampleGlob,
  Playground,
  playgroundResolvedExampleId,
  type PlaygroundExampleCatalog,
} from "@semio-tech/framework-playground-core";
import {
	registerAppVirtualFileSystem,
	virtualFileSystemSurfaceId,
	buildVirtualFileSystemWindowBody,
} from "@semio-tech/framework-platform-core";
import { downloadMediaExportResult, type NavigationLevel, type Platform } from "@semio-tech/framework-core";
import { createWriterDocument, type WriterDocument } from "@semio-tech/writer-core/internal";
import { wireLiteralFromDagFixtureJson } from "@semio-tech/graph-dsl-core";
import {
	DevJsonBackbone,
	RemoteOsBackbone,
	listSPrograms,
	materializeStudioProjection,
	mergeSProgramDefinition,
	COMPOSE_SKETCHPAD_PROGRAM_ID,
	appInstanceResourceProjection,
	parseSStudioDocument,
	registerSFixtureJsonResolver,
	seedSProgramRegistryFromResourceMap,
	sExtensionRegistrySize,
	StudioStore,
	sProgramById,
	sMediaGraphToDagFixtureJson,
	OsMediaGraphVirtualFileSystemController,
	OS_MEDIA_GRAPH_VFS_ROOT_ID,
	registerGenericMeshMediaExportHandlers,
	assertOsMediaExportCoverage,
	exportOsAppInstanceMedia,
	materializeAppInstanceProjection,
	type OsMediaExportFormat,
	type OsMediaExportResult,
	sAppRegistration,
	osParameterTypesCompatible,
	osParameterValue,
	type OsParameter,
	type OsParameterType,
} from "./internal.ts";
import {
	OsHomeVirtualFileSystemController,
	OS_HOME_VFS_ROOT_ID,
	createEmptyOsDocument,
	createOsStudio,
	deleteOsStudio,
	importOsStudioFromJson,
	loadOsStudioDocument,
	seedOsStudioCatalogIfEmpty,
	listOsStudioCatalogEntries,
} from "@semio-tech/framework-os-core";

export const S_HOME_APP_ID = "home";
export const S_HOME_CONTROLLER_ID = "s-home";
export const S_HOME_WINDOW = "s-home-main";
export const S_HOME_BODY = "s.home.vfs";
export const S_HOME_SURFACE = virtualFileSystemSurfaceId(S_HOME_APP_ID);
export const S_HOME_LAYOUT = createTabStackLayout([S_HOME_WINDOW], ["Studios"]);

export const S_PLAY_APP_ID = "studio";
export const S_PLAY_CONTROLLER_ID = "s-play";
export const S_PLAY_SURFACE_MEDIA_GRAPH = "s.play.media-graph";
export const S_PLAY_SURFACE_MEDIA_VFS = "s.play.media-vfs";
export const S_PLAY_BODY_MEDIA_GRAPH = "s.play.media-graph";
export const S_PLAY_BODY_MEDIA_VFS = "s.play.media-vfs";
export const S_PLAY_WINDOW_MEDIA_GRAPH = "s-media-graph";
export const S_PLAY_WINDOW_MEDIA_VFS = "s-media-vfs";
export const S_PLAY_WINDOW_COMPILED_DAG = "s-compiled-dag";
export const S_PLAY_BODY_COMPILED_DAG = "s.play.compiled-dag";
export const S_PLAY_SURFACE_COMPILED_DAG = "s.play.compiled-dag";

export const S_PLAY_LAYOUT = createDefaultLayout(
	[S_PLAY_WINDOW_MEDIA_GRAPH, S_PLAY_WINDOW_MEDIA_VFS, S_PLAY_WINDOW_COMPILED_DAG],
	"row",
	[40, 30, 30],
	["Media Graph", "Media VFS", "Compiled DAG"],
);

export type SPlayFixtureLoader = (fixtureId: string) => SStudioDocument;

/** @emoji 🏗️ Creates a studio store for s play. */
export function createStudioStore(document: SStudioDocument): StudioStore {
	const backbone = document.backbone?.kind === "remote" ? new RemoteOsBackbone() : new DevJsonBackbone();
	if (document.backbone?.uri) backbone.attach(document.backbone.uri);
	const store = new StudioStore(document, {
		onAfterMutation: () => {
			backbone.sync(store.getDocument());
		},
	});
	backbone.subscribe?.((remoteDocument) => {
		const known = new Set(store.getDocument().vcs.operations.map((entry) => entry.id));
		for (const change of remoteDocument.vcs.operations) {
			if (!known.has(change.id)) store.applyRemoteChange(change);
		}
	});
	return store;
}

//#region 🔖SExtensionWiring
/** @emoji 🧩 Registers all s technology extensions and VCS handlers. */
export async function bootstrapSPlayExtensions(): Promise<void> {
	seedSProgramRegistryFromResourceMap();
	const { loadAllOsProgramContributions } = await import("@semio-tech/framework-playground-core/app-registry");
	const contributions = await loadAllOsProgramContributions();
	for (const contribution of contributions) {
		await contribution.register();
	}
	registerGenericMeshMediaExportHandlers();
}
//#endregion 🔖SExtensionWiring

function sPlayMediaNodePointerKey(id: string): string {
	return `media-node:${id}`;
}

function sPlayAppInstancePointerKey(id: string): string {
	return `app-instance:${id}`;
}

function sPlayPointerKeysToMediaNodeIds(keys: readonly string[]): string[] {
	return keys.filter((key) => key.startsWith("media-node:")).map((key) => key.slice("media-node:".length));
}

function sPlayPointerKeysToAppInstanceIds(keys: readonly string[]): string[] {
	return keys.filter((key) => key.startsWith("app-instance:")).map((key) => key.slice("app-instance:".length));
}

function sHomeCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: S_HOME_CONTROLLER_ID, command, args };
}

function sOsNavigationDestination(id: string, label: string, uri: string) {
	return { id, label, uri };
}

function sOsNavigationLevel(node: ReturnType<typeof sOsNavigationDestination>, alternatives: readonly ReturnType<typeof sOsNavigationDestination>[]): NavigationLevel {
	return { node, alternatives };
}

export interface SOsShellRefs {
	readonly home: SHomeController;
	readonly studio: SPlayController;
}

let sOsPlatformSingleton: Platform | null = null;
let sOsShellRefsSingleton: SOsShellRefs | null = null;

/** @emoji 🔍 Returns the live OS shell {@link Platform}, if built. */
export function getSOsPlatform(): Platform | null {
	return sOsPlatformSingleton;
}

/** @emoji 🧭 Applies an OS shell URI to home/studio app routing. */
export function applySOsUri(platform: Platform, uri: string, shell: SOsShellRefs): void {
	const pathOnly = uri.split("?")[0] ?? "/";
	platform.uri = uri;
	const studioMatch = /^\/studios\/([^/]+)$/.exec(pathOnly);
	if (studioMatch) {
		platform.activeAppId = S_PLAY_APP_ID;
		shell.studio.openStudio(studioMatch[1]!);
	} else {
		platform.activeAppId = S_HOME_APP_ID;
		shell.studio.goHome();
	}
	platform.notify();
}

/** @emoji 🧭 Breadcrumb trail for the OS shell home/studio routes. */
export function osNavigation(_platform: Platform, uri: string, shell?: SOsShellRefs | null): readonly NavigationLevel[] {
	const pathOnly = uri.split("?")[0] ?? "/";
	const homeLevel = sOsNavigationLevel(sOsNavigationDestination("os.nav.home", "Home", "/"), []);
	const studioMatch = /^\/studios\/([^/]+)$/.exec(pathOnly);
	if (!studioMatch) return [homeLevel];
	const studioId = studioMatch[1]!;
	const studioName = shell?.studio.getStudioStore().getDocument().name ?? studioId;
	return [
		homeLevel,
		sOsNavigationLevel(sOsNavigationDestination(`os.nav.studio.${studioId}`, studioName, `/studios/${studioId}`), []),
	];
}

/** @emoji 🧭 Navigates the OS shell (updates browser history when mounted). */
export function navigateOsTo(uri: string): void {
	const platform = sOsPlatformSingleton;
	if (!platform) throw new Error("s: OS platform not initialized");
	if (platform.onNavigate) {
		platform.onNavigate(uri);
		return;
	}
	if (sOsShellRefsSingleton) applySOsUri(platform, uri, sOsShellRefsSingleton);
}

export class SHomeController extends Controller {
	readonly mainMode = new ModeRuntime("explore", "Explore", undefined);
	private readonly homeVfsController: OsHomeVirtualFileSystemController;

	constructor(commandBus: CommandBus, notify: () => void) {
		super(S_HOME_CONTROLLER_ID, commandBus, notify);
		this.homeVfsController = new OsHomeVirtualFileSystemController(`${S_HOME_CONTROLLER_ID}-vfs`, commandBus, notify, {
			onOpenStudio: (studioId) => navigateOsTo(`/studios/${studioId}`),
			onDeleteStudio: (studioId) => {
				deleteOsStudio(studioId);
				this.invalidateHomeCatalog();
			},
		});
		this.rebuildShellMode();
	}

	attachHomeVirtualFileSystem(platform: Platform, app: AppRuntime): void {
		registerAppVirtualFileSystem(platform, app, this.homeVfsController, {
			bodyKey: S_HOME_BODY,
			surfaceId: S_HOME_SURFACE,
			initialExpanded: [OS_HOME_VFS_ROOT_ID],
		});
	}

	invalidateHomeCatalog(): void {
		this.homeVfsController.invalidateHomeVirtualFileSystem({ appId: S_HOME_APP_ID, surfaceId: S_HOME_SURFACE });
		this.emit();
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = buildSHomeToolbarTools(this.id);
		this.mainMode.windowKinds = [new WindowKindRuntime(S_HOME_WINDOW, "Studios", S_HOME_BODY)];
	}

	run(command: string, args?: Record<string, unknown>): void {
		switch (command) {
			case "createStudio": {
				const entry = createOsStudio(typeof args?.name === "string" ? args.name : "Untitled Studio");
				this.invalidateHomeCatalog();
				navigateOsTo(`/studios/${entry.id}`);
				return;
			}
			case "importStudio": {
				if (typeof window === "undefined") return;
				const input = document.createElement("input");
				input.type = "file";
				input.accept = ".json,application/json";
				input.onchange = () => {
					const file = input.files?.[0];
					if (!file) return;
					void file.text().then((json) => {
						const entry = importOsStudioFromJson(json);
						this.invalidateHomeCatalog();
						navigateOsTo(`/studios/${entry.id}`);
					});
				};
				input.click();
				return;
			}
			case "openStudio": {
				const studioId = String(args?.studioId ?? "");
				if (!studioId) return;
				navigateOsTo(`/studios/${studioId}`);
				return;
			}
			case "goHome":
				navigateOsTo("/");
				return;
			default:
				return;
		}
	}
}

export function buildSHomeAppRuntime(ctrl: SHomeController): AppRuntime {
	return createPlayAppRuntime(S_HOME_APP_ID, "Home", ctrl, S_HOME_LAYOUT, ctrl.mainMode);
}

/** @emoji 🧰 OS home footer toolbar. */
export function buildSHomeToolbarTools(controllerId: string): AppTools {
	return [
		toolCollection("studios", "layout-grid", [
			{ kind: "button", id: "s.home.createStudio", label: "New Studio", iconId: "plus", controllerId, command: "createStudio" },
			{ kind: "button", id: "s.home.importStudio", label: "Import Studio", iconId: "upload", controllerId, command: "importStudio" },
		]),
	];
}

export class SPlayController extends Controller {
	private store: StudioStore;
	private activeInstanceId: string | null = null;
	private focusedInstanceId: string | null = null;
	private fixtureId: string;
	private studioId: string | null = null;
	private mediaGraphEngagementInput = "";
	private readonly mediaGraphVfsController: OsMediaGraphVirtualFileSystemController;
	private mediaGraphVfsUnsubscribe?: () => void;
	private readonly snapshotListeners = new Set<() => void>();
	readonly mainMode = new ModeRuntime("main", "Studio", undefined);

	constructor(commandBus: CommandBus, notify: () => void, store: StudioStore, fixtureId: string, private readonly loadFixture: SPlayFixtureLoader) {
		super(S_PLAY_CONTROLLER_ID, commandBus, notify);
		this.store = store;
		this.fixtureId = fixtureId;
		const projection = this.store.projection();
		this.activeInstanceId = projection.appInstances[0]?.id ?? null;
		this.mediaGraphVfsController = new OsMediaGraphVirtualFileSystemController(
			`${S_PLAY_CONTROLLER_ID}-media-vfs`,
			commandBus,
			notify,
			{
				store: () => this.store,
				onExport: async (instanceId, _portSpecId, format) => {
					const projection = this.store.projection();
					const instance = projection.appInstances.find((entry) => entry.id === instanceId);
					if (!instance) return;
					const source = materializeAppInstanceProjection(instance, { graph: projection.mediaGraph, instances: projection.appInstances });
					const result = await exportOsAppInstanceMedia(instance, source, format);
					downloadMediaExportResult(result);
				},
				onSpawnApp: (programId, appId) => {
					this.run("spawnApp", { programId, appId });
				},
				onOpenInstance: (instanceId) => {
					this.run("openInstance", { instanceId });
				},
			},
		);
		this.mediaGraphVfsUnsubscribe = this.store.subscribe(() => {
			this.mediaGraphVfsController.invalidateMediaGraphVirtualFileSystem({
				appId: S_PLAY_APP_ID,
				surfaceId: S_PLAY_SURFACE_MEDIA_VFS,
			});
			this.notifySnapshot();
		});
		this.rebuildShellMode();
	}

	attachMediaGraphVirtualFileSystem(platform: import("@semio-tech/framework-core").Platform, app: AppRuntime): void {
		registerAppVirtualFileSystem(platform, app, this.mediaGraphVfsController, {
			bodyKey: S_PLAY_BODY_MEDIA_VFS,
			surfaceId: S_PLAY_SURFACE_MEDIA_VFS,
			initialExpanded: [OS_MEDIA_GRAPH_VFS_ROOT_ID],
		});
	}

	disposeMediaGraphVirtualFileSystem(): void {
		this.mediaGraphVfsUnsubscribe?.();
	}

	subscribeSnapshot(listener: () => void): () => void {
		this.snapshotListeners.add(listener);
		return () => {
			this.snapshotListeners.delete(listener);
		};
	}

	getCompiledWireLiteral(): string {
		const projection = this.store.projection();
		return wireLiteralFromDagFixtureJson(
			sMediaGraphToDagFixtureJson(projection.mediaGraph, projection.appInstances, undefined, projection.parameters),
		);
	}

	getWriterDocumentCompiledDag(): WriterDocument {
		return createWriterDocument({ id: "s-compiled-dag", languageId: "wire", text: this.getCompiledWireLiteral() });
	}

	private notifySnapshot(): void {
		for (const listener of this.snapshotListeners) {
			listener();
		}
	}

	private mediaGraphMeasures(): readonly WindowMeasure[] {
		const projection = this.store.projection();
		return [
			{
				kind: "select",
				id: "s-media-active-instance",
				label: "Active app",
				value: this.activeInstanceId ?? "",
				items: projection.appInstances.map((instance) => ({ id: instance.id, value: instance.id, label: instance.label })),
				onChange: sPlayCmd("selectInstance"),
			},
		];
	}

	private mediaGraphEngagement(): WindowEngagement {
		const projection = this.store.projection();
		return {
			sessionActive: false,
			input: {
				id: "s-media-catalogue-hint",
				value: this.mediaGraphEngagementInput,
				placeholder: "Drag apps from Catalogue workbench tab",
				onChange: sPlayCmd("mediaGraphEngagementInput"),
			},
			status: [{ id: "s-media-count", text: `${projection.mediaGraph.nodes.length} nodes · ${projection.appInstances.length} apps` }],
		};
	}

	private compiledDagEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: "s-compiled-dag-engagement",
				value: "",
				placeholder: "Compiled DAG is read-only",
				onChange: sPlayCmd("compiledDagEngagementInput"),
				onSubmit: sPlayCmd("compiledDagEngagementSubmit"),
			},
			status: [{ id: "s-compiled-dag-status", text: this.getCompiledWireLiteral() ? "Compiled" : "Empty" }],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = buildSPlayToolbarTools(this.id);
		this.mainMode.windowKinds = [
			new WindowKindRuntime(
				S_PLAY_WINDOW_MEDIA_GRAPH,
				"Media Graph",
				S_PLAY_BODY_MEDIA_GRAPH,
				undefined,
				this.mediaGraphMeasures(),
				this.mediaGraphEngagement(),
			),
			new WindowKindRuntime(S_PLAY_WINDOW_MEDIA_VFS, "Media VFS", S_PLAY_BODY_MEDIA_VFS),
			new WindowKindRuntime(S_PLAY_WINDOW_COMPILED_DAG, "Compiled DAG", S_PLAY_BODY_COMPILED_DAG, undefined, undefined, this.compiledDagEngagement()),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `S play window "${windowKind.id}"`);
		}
	}

	getStudioStore(): StudioStore {
		return this.store;
	}

	getActiveInstanceId(): string | null {
		return this.activeInstanceId;
	}

	getFocusedInstanceId(): string | null {
		return this.focusedInstanceId;
	}

	getSelectedMediaNodeIds(): readonly string[] {
		return sPlayPointerKeysToMediaNodeIds(this.pointerFocus.getSnapshot().selection);
	}

	getSelectedAppInstanceIds(): readonly string[] {
		return sPlayPointerKeysToAppInstanceIds(this.pointerFocus.getSnapshot().selection);
	}

	getFixtureId(): string {
		return this.fixtureId;
	}

	getStudioId(): string | null {
		return this.studioId;
	}

	/** @emoji 📂 Loads a persisted studio document into the studio app store. */
	openStudio(studioId: string): void {
		if (this.studioId === studioId) {
			this.rebuildShellMode();
			this.emit();
			return;
		}
		this.mediaGraphVfsUnsubscribe?.();
		const document = loadOsStudioDocument(studioId);
		this.studioId = studioId;
		this.fixtureId = studioId;
		this.store = createStudioStore(document);
		this.mediaGraphVfsUnsubscribe = this.store.subscribe(() => {
			this.mediaGraphVfsController.invalidateMediaGraphVirtualFileSystem({
				appId: S_PLAY_APP_ID,
				surfaceId: S_PLAY_SURFACE_MEDIA_VFS,
			});
			this.notifySnapshot();
		});
		const projection = this.store.projection();
		this.activeInstanceId = projection.appInstances[0]?.id ?? null;
		this.focusedInstanceId = null;
		this.rebuildShellMode();
		this.emit();
		this.notifySnapshot();
	}

	/** @emoji 🏠 Clears studio drill-in state when returning to the home app. */
	goHome(): void {
		this.focusedInstanceId = null;
		this.emit();
	}

	getExampleCatalog(): PlaygroundExampleCatalog {
		return {
			activeExampleId: this.fixtureId,
			options: S_PLAY_EXAMPLE_OPTIONS(),
		};
	}

	dispatch(command: StudioCommand): void {
		this.store.dispatch(command);
		this.emit();
	}

	run(command: string, args?: Record<string, unknown>): void {
		switch (command) {
			case "mediaGraphEngagementInput": {
				const value = typeof args?.value === "string" ? args.value : "";
				if (value !== this.mediaGraphEngagementInput) {
					this.mediaGraphEngagementInput = value;
					this.rebuildShellMode();
					this.emit();
				}
				return;
			}
			case "mediaGraphEngagementSubmit": {
				const raw = String(args?.value ?? this.mediaGraphEngagementInput).trim();
				const [programId, appId] = raw.split(/\s+/u);
				if (programId && appId) this.run("spawnApp", { programId, appId });
				return;
			}
			case "compiledDagEngagementInput":
			case "compiledDagEngagementSubmit":
				return;
			case "setMediaNodeSelection": {
				const nodeIds = Array.isArray(args?.nodeIds) ? args!.nodeIds.map((id) => String(id)) : [];
				const projection = this.store.projection();
				const instanceIds = nodeIds
					.map((nodeId) => projection.mediaGraph.nodes.find((node) => node.id === nodeId)?.instanceId)
					.filter((id): id is string => Boolean(id));
				this.pointerFocus.setSelection([
					...nodeIds.map(sPlayMediaNodePointerKey),
					...instanceIds.map(sPlayAppInstancePointerKey),
				]);
				if (instanceIds.length === 1) {
					this.activeInstanceId = instanceIds[0]!;
				}
				this.emit();
				return;
			}
			case "setAppInstanceSelection": {
				const instanceIds = Array.isArray(args?.instanceIds) ? args!.instanceIds.map((id) => String(id)) : [];
				const projection = this.store.projection();
				const nodeIds = instanceIds
					.map((instanceId) => projection.mediaGraph.nodes.find((node) => node.instanceId === instanceId)?.id)
					.filter((id): id is string => Boolean(id));
				this.pointerFocus.setSelection([
					...nodeIds.map(sPlayMediaNodePointerKey),
					...instanceIds.map(sPlayAppInstancePointerKey),
				]);
				if (instanceIds.length === 1) {
					this.activeInstanceId = instanceIds[0]!;
				}
				this.emit();
				return;
			}
			case "patchMediaNodes": {
				const nodeIds = Array.isArray(args?.nodeIds) ? (args!.nodeIds as readonly string[]) : [];
				const field = args?.field;
				const axis = args?.axis;
				const numeric = typeof args?.value === "number" ? args.value : Number(args?.value);
				if (!nodeIds.length || field !== "position" || (axis !== "x" && axis !== "y") || !Number.isFinite(numeric)) return;
				const projection = this.store.projection();
				for (const nodeId of nodeIds) {
					const node = projection.mediaGraph.nodes.find((row) => row.id === nodeId);
					if (!node) continue;
					const x = axis === "x" ? numeric : node.x;
					const y = axis === "y" ? numeric : node.y;
					this.store.dispatch({ kind: "moveMediaNode", nodeId, x, y });
				}
				this.emit();
				return;
			}
			case "patchAppInstances": {
				const instanceIds = Array.isArray(args?.instanceIds) ? (args!.instanceIds as readonly string[]) : [];
				const field = args?.field;
				const value = args?.value;
				if (!instanceIds.length || field !== "label" || typeof value !== "string") return;
				this.store.dispatch({ kind: "patchAppInstances", instanceIds, field: "label", value });
				this.emit();
				return;
			}
			case "selectInstance": {
				this.activeInstanceId = typeof args?.instanceId === "string" ? args.instanceId : null;
				if (this.activeInstanceId) {
					const node = this.store.projection().mediaGraph.nodes.find((row) => row.instanceId === this.activeInstanceId);
					this.pointerFocus.setSelection(
						node
							? [sPlayAppInstancePointerKey(this.activeInstanceId), sPlayMediaNodePointerKey(node.id)]
							: [sPlayAppInstancePointerKey(this.activeInstanceId)],
					);
				} else {
					this.pointerFocus.setSelection([]);
				}
				this.rebuildShellMode();
				this.emit();
				return;
			}
			case "spawnApp": {
				const programId = String(args?.programId ?? "");
				const appId = String(args?.appId ?? "");
				if (!programId || !appId) return;
				const position =
					args?.position && typeof args.position === "object"
						? { x: Number((args.position as { x?: number }).x ?? 80), y: Number((args.position as { y?: number }).y ?? 80) }
						: { x: 80, y: 80 };
				this.store.dispatch({ kind: "spawnAppInstance", programId, appId, position });
				const created = this.store.projection().appInstances.at(-1);
				if (created) this.activeInstanceId = created.id;
				this.rebuildShellMode();
				this.emit();
				return;
			}
			case "openInstance": {
				const instanceId = typeof args?.instanceId === "string" ? args.instanceId : null;
				if (!instanceId) return;
				if (!this.store.projection().appInstances.some((entry) => entry.id === instanceId)) return;
				this.focusedInstanceId = instanceId;
				this.activeInstanceId = instanceId;
				const node = this.store.projection().mediaGraph.nodes.find((row) => row.instanceId === instanceId);
				this.pointerFocus.setSelection(
					node
						? [sPlayAppInstancePointerKey(instanceId), sPlayMediaNodePointerKey(node.id)]
						: [sPlayAppInstancePointerKey(instanceId)],
				);
				this.rebuildShellMode();
				this.emit();
				return;
			}
			case "goHome":
				navigateOsTo("/");
				return;
			case "closeFocusedInstance": {
				this.focusedInstanceId = null;
				this.emit();
				return;
			}
			case "undo":
				this.store.dispatch({ kind: "undo" });
				this.rebuildShellMode();
				this.emit();
				return;
			case "redo":
				this.store.dispatch({ kind: "redo" });
				this.rebuildShellMode();
				this.emit();
				return;
			case "commitCheckpoint":
				this.store.dispatch({ kind: "commitCheckpoint", message: typeof args?.message === "string" ? args.message : undefined });
				this.rebuildShellMode();
				this.emit();
				return;
			case "setActiveExample": {
				const fixtureId = String(args?.exampleId ?? args?.fixtureId ?? this.fixtureId);
				this.fixtureId = fixtureId;
				this.store = createStudioStore(this.loadFixture(fixtureId));
				const projection = this.store.projection();
				this.activeInstanceId = projection.appInstances[0]?.id ?? null;
				this.focusedInstanceId = null;
				this.rebuildShellMode();
				this.emit();
				return;
			}
			case "addParameter": {
				this.store.dispatch({
					kind: "addParameter",
					type: (args?.type as OsParameterType | undefined) ?? "numeric",
					name: typeof args?.name === "string" ? args.name : undefined,
				});
				this.emit();
				return;
			}
			case "removeParameter": {
				const parameterId = String(args?.parameterId ?? "");
				if (!parameterId) return;
				this.store.dispatch({ kind: "removeParameter", parameterId });
				this.emit();
				return;
			}
			case "patchParameter": {
				const parameterId = String(args?.parameterId ?? "");
				const field = typeof args?.field === "string" ? args.field : undefined;
				if (!parameterId || !field) return;
				let patch: Record<string, unknown>;
				if (field === "addOption" && typeof args?.value === "string") {
					const current = this.store.projection().parameters.find((entry) => entry.id === parameterId);
					if (current?.type !== "categorical") return;
					patch = { options: [...current.options, args.value], value: args.value };
				} else if (field === "removeOption" && typeof args?.value === "string") {
					const current = this.store.projection().parameters.find((entry) => entry.id === parameterId);
					if (current?.type !== "categorical") return;
					const options = current.options.filter((option) => option !== args.value);
					patch = { options, value: options.includes(current.value) ? current.value : (options[0] ?? "") };
				} else {
					patch = { [field]: args?.value };
				}
				this.store.dispatch({ kind: "patchParameter", parameterId, patch });
				this.emit();
				return;
			}
			case "bindParameterField": {
				const instanceId = String(args?.instanceId ?? "");
				const fieldPath = String(args?.fieldPath ?? "");
				const parameterId = String(args?.parameterId ?? args?.value ?? "");
				if (!instanceId || !fieldPath) return;
				if (!parameterId || parameterId === "__direct__") {
					this.store.dispatch({ kind: "unbindParameterField", instanceId, fieldPath });
				} else {
					this.store.dispatch({ kind: "bindParameterField", instanceId, fieldPath, parameterId });
				}
				this.emit();
				return;
			}
			case "unbindParameterField": {
				const instanceId = String(args?.instanceId ?? "");
				const fieldPath = String(args?.fieldPath ?? "");
				if (!instanceId || !fieldPath) return;
				this.store.dispatch({ kind: "unbindParameterField", instanceId, fieldPath });
				this.emit();
				return;
			}
			default:
				return;
		}
	}
}

export function buildSPlayAppRuntime(ctrl: SPlayController): AppRuntime {
	return createPlayAppRuntime(S_PLAY_APP_ID, "Studio", ctrl, S_PLAY_LAYOUT, ctrl.mainMode);
}

function sPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: S_PLAY_CONTROLLER_ID, command, args };
}

/** @emoji 🧰 S play footer toolbar. */
export function buildSPlayToolbarTools(controllerId: string): AppTools {
	return [
		toolCollection("history", "history", [
			{ kind: "button", id: "s.undo", label: "Undo", iconId: "undo-2", controllerId, command: "undo" },
			{ kind: "button", id: "s.redo", label: "Redo", iconId: "redo-2", controllerId, command: "redo" },
			{ kind: "button", id: "s.checkpoint", label: "Checkpoint", iconId: "git-commit", controllerId, command: "commitCheckpoint" },
		]),
	];
}

const S_PLAY_CATALOGUE_DRAG_MIME = "application/x-semio-catalogue-item";

/** @emoji 📚 Declarative catalogue tree for spawning program apps into the studio media graph. */
export function buildSPlayCatalogueTree(): UiTreeNode {
	const programs = listSPrograms().filter((program) => program.id !== "s.system");
	return uiDeclarativeSectionsToTree([
		{
			type: "section",
			id: "s-play-catalogue",
			label: "Apps",
			children: programs.map((program) => ({
				type: "section",
				id: `s-play-catalogue.program.${program.id}`,
				label: program.name,
				children: program.apps.map((app) => ({
					type: "item",
					id: `s-play-catalogue.app.${program.id}.${app.id}`,
					label: app.label,
					dragData: { [S_PLAY_CATALOGUE_DRAG_MIME]: JSON.stringify({ programId: program.id, appId: app.id }) },
					meta: sAppRegistration(program.id, app.id)?.outputs.map((port) => port.resourceKind).join(", ") ?? "",
				})),
			})),
		},
	]);
}

function sPlayParameterValueControl(parameter: OsParameter): UiNode {
	switch (parameter.type) {
		case "numeric":
			return {
				type: "numberStepper",
				id: `s-play-parameters.${parameter.id}.value`,
				value: parameter.value,
				step: parameter.step ?? 1,
				uniform: true,
				onAbsolute: sPlayCmd("patchParameter", { parameterId: parameter.id, field: "value" }),
				onDelta: sPlayCmd("patchParameter", { parameterId: parameter.id, field: "value" }),
			};
		case "categorical":
			return {
				type: "select",
				id: `s-play-parameters.${parameter.id}.value`,
				value: parameter.value,
				items: parameter.options.map((option) => ({ id: option, value: option, label: option })),
				onChange: sPlayCmd("patchParameter", { parameterId: parameter.id, field: "value" }),
			};
		case "toggle":
			return {
				type: "toggle",
				id: `s-play-parameters.${parameter.id}.value`,
				iconId: "toggle-left",
				pressed: parameter.value,
				text: parameter.value ? "On" : "Off",
				onChange: sPlayCmd("patchParameter", { parameterId: parameter.id, field: "value" }),
			};
		case "text":
			return {
				type: "input",
				id: `s-play-parameters.${parameter.id}.value`,
				inputKind: "text",
				value: parameter.value,
				onChange: sPlayCmd("patchParameter", { parameterId: parameter.id, field: "value" }),
			};
	}
}

function sPlayParameterConstraintFields(parameter: OsParameter): UiNode[] {
	if (parameter.type === "numeric") {
		return [
			{
				type: "field",
				id: `s-play-parameters.${parameter.id}.min`,
				label: "Min",
				child: {
					type: "input",
					id: `s-play-parameters.${parameter.id}.min.input`,
					inputKind: "number",
					value: parameter.min !== undefined ? String(parameter.min) : "",
					onChange: sPlayCmd("patchParameter", { parameterId: parameter.id, field: "min" }),
				},
			},
			{
				type: "field",
				id: `s-play-parameters.${parameter.id}.max`,
				label: "Max",
				child: {
					type: "input",
					id: `s-play-parameters.${parameter.id}.max.input`,
					inputKind: "number",
					value: parameter.max !== undefined ? String(parameter.max) : "",
					onChange: sPlayCmd("patchParameter", { parameterId: parameter.id, field: "max" }),
				},
			},
			{
				type: "field",
				id: `s-play-parameters.${parameter.id}.step`,
				label: "Step",
				child: {
					type: "input",
					id: `s-play-parameters.${parameter.id}.step.input`,
					inputKind: "number",
					value: parameter.step !== undefined ? String(parameter.step) : "",
					onChange: sPlayCmd("patchParameter", { parameterId: parameter.id, field: "step" }),
				},
			},
		];
	}
	if (parameter.type === "categorical") {
		const optionRows: UiNode[] = parameter.options.map((option) => ({
			type: "field",
			id: `s-play-parameters.${parameter.id}.option.${option}`,
			label: option,
			child: {
				type: "button",
				id: `s-play-parameters.${parameter.id}.option.${option}.remove`,
				iconId: "trash-2",
				label: "Remove",
				command: sPlayCmd("patchParameter", { parameterId: parameter.id, field: "removeOption", value: option }),
			},
		}));
		return [
			...optionRows,
			{
				type: "field",
				id: `s-play-parameters.${parameter.id}.add-option`,
				label: "Add option",
				child: {
					type: "input",
					id: `s-play-parameters.${parameter.id}.add-option.input`,
					inputKind: "text",
					value: "",
					placeholder: "New option",
					onChange: sPlayCmd("patchParameter", { parameterId: parameter.id, field: "addOption" }),
				},
			},
		];
	}
	return [];
}

/** @emoji 🎛️ Declarative parameters tree for studio-level parameter definitions. */
export function buildSPlayParametersTree(ctrl: SPlayController): UiTreeNode {
	const projection = ctrl.getStudioStore().projection();
	const children: UiNode[] = [
		{
			type: "section",
			id: "s-play-parameters.header",
			label: FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
			children: [
				{
					type: "button",
					id: "s-play-parameters.add",
					iconId: "plus",
					label: "Add Parameter",
					command: sPlayCmd("addParameter", { type: "numeric" }),
				},
				{ type: "text", value: `${projection.parameters.length} parameter(s)` },
			],
		},
	];
	for (const parameter of projection.parameters) {
		children.push({
			type: "section",
			id: `s-play-parameters.${parameter.id}`,
			label: parameter.name,
			children: [
				{
					type: "field",
					id: `s-play-parameters.${parameter.id}.name`,
					label: "Name",
					child: {
						type: "input",
						id: `s-play-parameters.${parameter.id}.name.input`,
						inputKind: "text",
						value: parameter.name,
						onChange: sPlayCmd("patchParameter", { parameterId: parameter.id, field: "name" }),
					},
				},
				{
					type: "field",
					id: `s-play-parameters.${parameter.id}.type`,
					label: "Type",
					child: {
						type: "select",
						id: `s-play-parameters.${parameter.id}.type.select`,
						value: parameter.type,
						items: [
							{ id: "numeric", value: "numeric", label: "Numeric" },
							{ id: "categorical", value: "categorical", label: "Categorical" },
							{ id: "toggle", value: "toggle", label: "Toggle" },
							{ id: "text", value: "text", label: "Text" },
						],
						onChange: sPlayCmd("patchParameter", { parameterId: parameter.id, field: "type" }),
					},
				},
				{
					type: "field",
					id: `s-play-parameters.${parameter.id}.value-field`,
					label: "Value",
					child: sPlayParameterValueControl(parameter),
				},
				...sPlayParameterConstraintFields(parameter),
				{
					type: "button",
					id: `s-play-parameters.${parameter.id}.remove`,
					iconId: "trash-2",
					label: "Remove",
					command: sPlayCmd("removeParameter", { parameterId: parameter.id }),
				},
			],
		});
	}
	return uiDeclarativeSectionsToTree(children);
}

/** @emoji 🔎 Declarative inspection tree for s play media graph and app instances. */
export function buildSPlayInspectorTree(ctrl: SPlayController): UiTreeNode {
	const projection = ctrl.getStudioStore().projection();
	const mediaNodeIds = [...ctrl.getSelectedMediaNodeIds()];
	const instanceIds = [...ctrl.getSelectedAppInstanceIds()];
	const children: UiNode[] = [
		{
			type: "section",
			id: "s-play-inspector.header",
			label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
			children: [
				{
					type: "text",
					value: `${mediaNodeIds.length} media node(s) · ${instanceIds.length} app instance(s)`,
				},
			],
		},
	];
	if (mediaNodeIds.length > 0) {
		const nodes = mediaNodeIds
			.map((nodeId) => projection.mediaGraph.nodes.find((node) => node.id === nodeId))
			.filter((node): node is SMediaGraphNode => node !== undefined);
		const xs = nodes.map((node) => node.x);
		const ys = nodes.map((node) => node.y);
		const xUniform = uiInspectorAllEqual(xs);
		const yUniform = uiInspectorAllEqual(ys);
		const nodeFields: UiNode[] = [];
		if (mediaNodeIds.length === 1) {
			nodeFields.push({
				type: "field",
				id: "s-play-inspector.media-node.id",
				label: "Node id",
				child: { type: "text", value: mediaNodeIds[0]! },
			});
		}
		nodeFields.push(
			{
				type: "field",
				id: "s-play-inspector.media-node.x",
				label: "X",
				child: {
					type: "input",
					id: "s-play-inspector.media-node.x.input",
					inputKind: "number",
					value: xUniform ? String(xs[0] ?? 0) : "",
					placeholder: xUniform ? undefined : "Mixed",
					onChange: sPlayCmd("patchMediaNodes", { nodeIds: mediaNodeIds, field: "position", axis: "x" }),
				},
			},
			{
				type: "field",
				id: "s-play-inspector.media-node.y",
				label: "Y",
				child: {
					type: "input",
					id: "s-play-inspector.media-node.y.input",
					inputKind: "number",
					value: yUniform ? String(ys[0] ?? 0) : "",
					placeholder: yUniform ? undefined : "Mixed",
					onChange: sPlayCmd("patchMediaNodes", { nodeIds: mediaNodeIds, field: "position", axis: "y" }),
				},
			},
		);
		children.push({
			type: "section",
			id: "s-play-inspector.media-nodes",
			label: mediaNodeIds.length === 1 ? "Media graph node" : `Media graph nodes (${mediaNodeIds.length})`,
			children: nodeFields,
		});
	}
	if (instanceIds.length > 0) {
		const instances = instanceIds
			.map((instanceId) => projection.appInstances.find((instance) => instance.id === instanceId))
			.filter((instance): instance is SAppInstance => instance !== undefined);
		const labels = instances.map((instance) => instance.label);
		const programIds = instances.map((instance) => instance.programId);
		const appIds = instances.map((instance) => instance.appId);
		const labelUniform = uiInspectorAllEqual(labels);
		const programUniform = uiInspectorAllEqual(programIds);
		const appUniform = uiInspectorAllEqual(appIds);
		const instanceFields: UiNode[] = [];
		if (instanceIds.length === 1) {
			instanceFields.push({
				type: "field",
				id: "s-play-inspector.app-instance.id",
				label: "Instance id",
				child: { type: "text", value: instanceIds[0]! },
			});
		}
		instanceFields.push(
			{
				type: "field",
				id: "s-play-inspector.app-instance.label",
				label: "Label",
				child: {
					type: "input",
					id: "s-play-inspector.app-instance.label.input",
					inputKind: "text",
					value: labelUniform ? (labels[0] ?? "") : "",
					placeholder: labelUniform ? undefined : "Mixed",
					onChange: sPlayCmd("patchAppInstances", { instanceIds, field: "label" }),
				},
			},
			{
				type: "field",
				id: "s-play-inspector.app-instance.program",
				label: "Program",
				child: { type: "text", value: programUniform ? (programIds[0] ?? "") : "Mixed" },
			},
			{
				type: "field",
				id: "s-play-inspector.app-instance.app",
				label: "App",
				child: { type: "text", value: appUniform ? (appIds[0] ?? "") : "Mixed" },
			},
		);
		if (instanceIds.length === 1) {
			const instance = instances[0]!;
			const registration = sAppRegistration(instance.programId, instance.appId);
			const parameterFields = registration?.parameterFields ?? [];
			if (parameterFields.length > 0) {
				const bindingRows: UiNode[] = parameterFields.flatMap((fieldSpec) => {
					const binding = projection.parameterBindings.find(
						(entry) => entry.instanceId === instance.id && entry.fieldPath === fieldSpec.fieldPath,
					);
					const compatibleParameters = projection.parameters.filter((parameter) =>
						osParameterTypesCompatible(parameter.type, fieldSpec.type),
					);
					const boundParameter = binding
						? projection.parameters.find((parameter) => parameter.id === binding.parameterId)
						: undefined;
					const rows: UiNode[] = [
						{
							type: "field",
							id: `s-play-inspector.app-parameter.${fieldSpec.fieldPath}`,
							label: fieldSpec.label,
							child: {
								type: "select",
								id: `s-play-inspector.app-parameter.${fieldSpec.fieldPath}.select`,
								value: binding?.parameterId ?? "__direct__",
								items: [
									{ id: "__direct__", value: "__direct__", label: "Direct value" },
									...compatibleParameters.map((parameter) => ({
										id: parameter.id,
										value: parameter.id,
										label: parameter.name,
									})),
								],
								onChange: sPlayCmd("bindParameterField", {
									instanceId: instance.id,
									fieldPath: fieldSpec.fieldPath,
								}),
							},
						},
					];
					if (boundParameter) {
						rows.push({
							type: "text",
							value: `Bound value: ${String(osParameterValue(boundParameter))}`,
						});
					}
					return rows;
				});
				instanceFields.push({
					type: "section",
					id: "s-play-inspector.app-parameters",
					label: "Parameters",
					children: bindingRows,
				});
			}
		}
		children.push({
			type: "section",
			id: "s-play-inspector.app-instances",
			label: instanceIds.length === 1 ? "App instance" : `App instances (${instanceIds.length})`,
			children: instanceFields,
		});
	}
	if (mediaNodeIds.length === 0 && instanceIds.length === 0) {
		children[0]!.children!.push({ type: "text", value: "Select media graph nodes or app instances in the canvas." });
	}
	return uiDeclarativeSectionsToTree(children);
}

export const sHomeWindowBodies: Readonly<Record<string, (ctx: import("@semio-tech/framework-platform-core").WindowBodyViewContext) => UiNode>> = {
	[S_HOME_BODY]: () => buildVirtualFileSystemWindowBody(S_HOME_SURFACE, S_HOME_CONTROLLER_ID, S_HOME_WINDOW),
};

export const sPlayWindowBodies: Readonly<Record<string, (ctx: import("@semio-tech/framework-platform-core").WindowBodyViewContext) => UiNode>> = {
	[S_PLAY_BODY_MEDIA_GRAPH]: () =>
		buildSWindowBody(S_PLAY_SURFACE_MEDIA_GRAPH, S_PLAY_CONTROLLER_ID, "mediaGraph", "media-graph"),
	[S_PLAY_BODY_COMPILED_DAG]: () =>
		buildWriterWindowBody(S_PLAY_SURFACE_COMPILED_DAG, S_PLAY_CONTROLLER_ID, S_PLAY_WINDOW_COMPILED_DAG),
};

export function registerSPlayDeclarativeBodies(): void {
	for (const [key, build] of Object.entries(sHomeWindowBodies)) registerWindowBody(key, build);
	for (const [key, build] of Object.entries(sPlayWindowBodies)) registerWindowBody(key, build);
}

export function registerSOsShellDeclarativeBodies(): void {
	registerSPlayDeclarativeBodies();
}

export async function sSketchpadProgramFromCompose() {
	const { buildSketchpadProgramDefinition } = await import("@semio-tech/compose-sketchpad");
	return buildSketchpadProgramDefinition();
}

//#region 🔖Play
import { S_PLAY_EXAMPLE_DEFAULT_ID, resolveSPlayExampleSlug } from "./example-slugs.ts";
import demoSFixture from "../../example/demo.s.json";


export { S_PLAY_EXAMPLE_DEFAULT_ID, resolveSPlayExampleSlug };

let sPlayFixtureJsonByIdCache: Readonly<Record<string, string>> | undefined;
let sFixtureResolverRegistered = false;

function fixtureSlugFromPath(path: string): string {
	return path.split("/").pop() ?? path;
}

function sFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.s\.json$/, "");
}

function ensureSPlayFixtureCatalog(): Readonly<Record<string, string>> {
	if (sPlayFixtureJsonByIdCache) return sPlayFixtureJsonByIdCache;
	const sFixtureModules = eagerPlayExampleGlob("../../example/*.s.json");
	const sFixtureFromGlob = Object.keys(sFixtureModules).length
		? Object.fromEntries(
				Object.entries(sFixtureModules).map(([path, module]) => [sFixtureIdFromGlobPath(path), JSON.stringify(module.default)]),
			)
		: { demo: JSON.stringify(demoSFixture) };
	const technologyFixtureModules = {
		...eagerPlayExampleGlob("../../draw/example/*.json"),
		...eagerPlayExampleGlob("../../writer/example/*.json"),
		...eagerPlayExampleGlob("../../note/example/*.note.json"),
	};
	const slugJsonByPath = Object.fromEntries(
		Object.entries(technologyFixtureModules).map(([path, module]) => [fixtureSlugFromPath(path), JSON.stringify(module.default)]),
	);
	if (!sFixtureResolverRegistered) {
		registerSFixtureJsonResolver((slug) => slugJsonByPath[slug] ?? null);
		sFixtureResolverRegistered = true;
	}
	sPlayFixtureJsonByIdCache = sFixtureFromGlob;
	return sPlayFixtureJsonByIdCache;
}

export function getSPlayFixtureJsonById(): Readonly<Record<string, string>> {
	return ensureSPlayFixtureCatalog();
}

export const S_PLAY_EXAMPLE_OPTIONS = (): ReadonlyArray<{ readonly id: string; readonly label: string }> =>
	Object.keys(ensureSPlayFixtureCatalog())
		.sort()
		.map((id) => ({ id, label: id }));

/** @emoji 📂 Loads an s studio document from a playground example id. */
export function loadSPlayStudioDocument(exampleId: string): SStudioDocument {
	const json = ensureSPlayFixtureCatalog()[exampleId];
	if (!json) throw new Error(`unknown s example: ${exampleId}`);
	return parseSStudioDocument(JSON.parse(json));
}

/** @emoji 🎮 Creates an s play controller wired to playground examples. */
export function createSPlayController(
	commandBus: CommandBus,
	notify: () => void,
	exampleId: string = S_PLAY_EXAMPLE_DEFAULT_ID,
): SPlayController {
	const resolved = playgroundResolvedExampleId(exampleId);
	const store = createStudioStore(loadSPlayStudioDocument(resolved));
	return new SPlayController(commandBus, notify, store, resolved, loadSPlayStudioDocument);
}

/** @emoji 🧪 Test helper for s play controller with example. */
export function createSPlayTestController(exampleId: string): SPlayController {
	const bus = new CommandBus();
	return createSPlayController(bus, () => {}, exampleId);
}

export const OS_BOOT_STUDIO_ID = "default";

/** @emoji 💾 Seeds the demo studio into the catalog when storage is empty. */
export function seedOsBootStudioCatalog(): SStudioDocument {
	const seed = loadSPlayStudioDocument(S_PLAY_EXAMPLE_DEFAULT_ID);
	const entry = seedOsStudioCatalogIfEmpty({ ...seed, id: OS_BOOT_STUDIO_ID, name: seed.name ?? "Demo Studio" });
	if (entry) return loadOsStudioDocument(entry.id);
	const first = listOsStudioCatalogEntries()[0];
	return first ? loadOsStudioDocument(first.id) : seed;
}

class OsDevPlayground extends Playground {
	readonly id = "os";

	createRuntime() {
		const runtime = createProductPlaygroundPlatform("os", "S");
		seedOsBootStudioCatalog();
		const homeCtrl = new SHomeController(runtime.commandBus, () => runtime.notify());
		const studioCtrl = new SPlayController(
			runtime.commandBus,
			() => runtime.notify(),
			createStudioStore(createEmptyOsDocument("studio-bootstrap", "Bootstrap")),
			S_PLAY_EXAMPLE_DEFAULT_ID,
			loadSPlayStudioDocument,
		);
		const shell: SOsShellRefs = { home: homeCtrl, studio: studioCtrl };
		sOsPlatformSingleton = runtime;
		sOsShellRefsSingleton = shell;
		runtime.applyUri = (uri) => applySOsUri(runtime, uri, shell);
		runtime.navigation = (uri) => osNavigation(runtime, uri, shell);
		runtime.defaultActiveAppId = S_HOME_APP_ID;
		const homeApp = buildSHomeAppRuntime(homeCtrl);
		const studioApp = buildSPlayAppRuntime(studioCtrl);
		runtime.addApp(homeApp);
		runtime.addApp(studioApp);
		homeCtrl.attachHomeVirtualFileSystem(runtime, homeApp);
		studioCtrl.attachMediaGraphVirtualFileSystem(runtime, studioApp);
		applySOsUri(runtime, "/", shell);
		return runtime;
	}

	registerBodies() {
		registerSOsShellDeclarativeBodies();
	}
}

/** @emoji 🖥️ Boots S as the OS studio shell (extensions, storage-first document, renderer). */
export async function bootOsDev(rootId = "root"): Promise<void> {
	await bootstrapSPlayExtensions();
	const { bootPlaygroundApp } = await import("@semio-tech/framework-playground-renderer-react");
	await bootPlaygroundApp(sPlayAppDefinition, new OsDevPlayground(), rootId);
}

export const sPlayAppDefinition = createPlaygroundApp({
	id: S_PLAY_APP_ID,
	label: "Studio",
	controllerId: S_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "s",
		resolveDedupe: ["react", "react-dom", "@semio-tech/s-react", "three", "@react-three/fiber", "@react-three/drei"],
		optimizeDeps: { include: ["react", "react-dom", "three"] },
	},
	createRuntime: () => {
		const runtime = createProductPlaygroundPlatform(S_PLAY_APP_ID, "Studio");
		const resolved = playgroundResolvedExampleId(S_PLAY_EXAMPLE_DEFAULT_ID);
		const ctrl = createSPlayController(runtime.commandBus, () => runtime.notify(), resolved);
		const app = buildSPlayAppRuntime(ctrl);
		runtime.addApp(app);
		ctrl.attachMediaGraphVirtualFileSystem(runtime, app);
		runtime.activeAppId = S_PLAY_APP_ID;
		return runtime;
	},
	registerBodies: () => {
		registerSPlayDeclarativeBodies();
	},
	loadRenderer: async () => (await import("@semio-tech/s-react/play")).sAppRenderer,
});
//#endregion 🔖Play

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it, beforeAll } = import.meta.vitest;

	beforeAll(async () => {
		await bootstrapSPlayExtensions();
	});

	describe("SPlayController", () => {
		it("loads demo studio projection", async () => {
			const ctrl = createSPlayTestController("demo");
			const projection = ctrl.getStudioStore().projection();
			expect(projection.appInstances.length).toBeGreaterThanOrEqual(2);
			expect(projection.mediaGraph.nodes.length).toBeGreaterThanOrEqual(2);
		});

		it("round-trips checkpoint save on studio document", async () => {
			const ctrl = createSPlayTestController("demo");
			ctrl.dispatch({ kind: "commitCheckpoint", message: "snapshot" });
			const doc = ctrl.getStudioStore().getDocument();
			const rematerialized = materializeStudioProjection(doc, doc.vcs.checkpoints[0]?.changeIds ?? []);
			expect(rematerialized.appInstances.length).toBe(ctrl.getStudioStore().projection().appInstances.length);
		});

		it("demo fixture includes cross-instance media edge", async () => {
			const ctrl = createSPlayTestController("demo");
			const projection = ctrl.getStudioStore().projection();
			expect(projection.mediaGraph.edges.length).toBeGreaterThanOrEqual(1);
			expect(projection.appInstances.length).toBeGreaterThanOrEqual(5);
		});

		it("aligns compose sketchpad program with s registry", async () => {
			const composeProgram = await sSketchpadProgramFromCompose();
			mergeSProgramDefinition(COMPOSE_SKETCHPAD_PROGRAM_ID, composeProgram);
			const sProgram = listSPrograms().find((program) => program.id === COMPOSE_SKETCHPAD_PROGRAM_ID);
			expect(composeProgram.id).toBe(COMPOSE_SKETCHPAD_PROGRAM_ID);
			expect(sProgram?.apps.length).toBe(composeProgram.apps.length);
		});

		it("registers all technology extensions", () => {
			expect(sExtensionRegistrySize()).toBeGreaterThanOrEqual(24);
			expect(sProgramById("draw")).toBeTruthy();
			expect(sProgramById("lowpoly")).toBeTruthy();
			expect(sProgramById("reasoning.mindmap")).toBeTruthy();
			expect(sProgramById(COMPOSE_SKETCHPAD_PROGRAM_ID)).toBeTruthy();
		});

		it("resolves draw fixture payload refs", async () => {
			const ctrl = createSPlayTestController("demo");
			const drawInstance = ctrl.getStudioStore().projection().appInstances.find((entry) => entry.programId === "draw");
			expect(drawInstance).toBeTruthy();
			const bundle = appInstanceResourceProjection(
				ctrl.getStudioStore().projection().mediaGraph,
				ctrl.getStudioStore().projection().appInstances,
				drawInstance!.id,
			);
			expect(bundle?.projection).toBeTruthy();
		});

		it("spawns puzzle5d and shooting with multi-port registrations", async () => {
			const ctrl = createSPlayTestController("demo");
			ctrl.run("spawnApp", { programId: "puzzle.5d", appId: "puzzle5d", position: { x: 200, y: 100 } });
			ctrl.run("spawnApp", { programId: "shooting", appId: "shooting", position: { x: 300, y: 100 } });
			const projection = ctrl.getStudioStore().projection();
			const puzzleNode = projection.mediaGraph.nodes.find((node) => node.instanceId === projection.appInstances.at(-2)?.id);
			const shootingNode = projection.mediaGraph.nodes.find((node) => node.instanceId === projection.appInstances.at(-1)?.id);
			expect(puzzleNode?.outputs.length).toBe(2);
			expect(shootingNode?.inputs.length).toBe(1);
		});

		it("buildSPlayCatalogueTree returns declarative sections", async () => {
			await bootstrapSPlayExtensions();
			const tree = buildSPlayCatalogueTree();
			expect(tree.sections.length).toBeGreaterThan(0);
			expect(tree.sections[0]?.items.length).toBeGreaterThan(0);
		});

		it("buildSPlayInspectorTree exposes batch label editing for selected instances", async () => {
			const ctrl = createSPlayTestController("demo");
			const instances = ctrl.getStudioStore().projection().appInstances;
			expect(instances.length).toBeGreaterThanOrEqual(2);
			ctrl.run("setAppInstanceSelection", { instanceIds: instances.slice(0, 2).map((row) => row.id) });
			const tree = buildSPlayInspectorTree(ctrl);
			const section = tree.sections.find((row) => row.id === "s-play-inspector.app-instances");
			const labelField = section?.items.find((item) => item.id === "s-play-inspector.app-instance.label");
			expect(labelField?.control?.type).toBe("input");
			expect(labelField?.control?.onChange?.command).toBe("patchAppInstances");
		});

		it("patchAppInstances updates labels in batch", async () => {
			const ctrl = createSPlayTestController("demo");
			const ids = ctrl.getStudioStore().projection().appInstances.slice(0, 2).map((row) => row.id);
			ctrl.run("patchAppInstances", { instanceIds: ids, field: "label", value: "Batch Label" });
			const labels = ctrl.getStudioStore().projection().appInstances.filter((row) => ids.includes(row.id)).map((row) => row.label);
			expect(labels.every((label) => label === "Batch Label")).toBe(true);
		});

		it("openInstance and closeFocusedInstance toggle drill-in focus", async () => {
			const ctrl = createSPlayTestController("demo");
			const instance = ctrl.getStudioStore().projection().appInstances[0];
			expect(instance).toBeTruthy();
			expect(ctrl.getFocusedInstanceId()).toBeNull();
			ctrl.run("openInstance", { instanceId: instance!.id });
			expect(ctrl.getFocusedInstanceId()).toBe(instance!.id);
			ctrl.run("closeFocusedInstance");
			expect(ctrl.getFocusedInstanceId()).toBeNull();
		});

		it("registry completeness: every registered app resolves handler, componentKind, and ports", () => {
			for (const program of listSPrograms()) {
				if (program.id === "s.system" || program.id === COMPOSE_SKETCHPAD_PROGRAM_ID) continue;
				for (const app of program.apps) {
					const registration = sAppRegistration(program.id, app.id);
					expect(registration, `${program.id}/${app.id} registration`).toBeTruthy();
					expect(registration?.componentKind).toBeTruthy();
					expect(registration?.sourceFormat).toBeTruthy();
				}
			}
		});

		it("seedOsBootStudioCatalog seeds storage when empty", () => {
			const document = seedOsBootStudioCatalog();
			expect(document.backbone?.uri).toBe(`dev://studio/${OS_BOOT_STUDIO_ID}`);
			expect(document.vcs.initialProjection.appInstances.length).toBeGreaterThan(0);
		});

		it("applySOsUri routes home and studio apps", () => {
			const runtime = createProductPlaygroundPlatform("os-test", "S");
			const homeCtrl = new SHomeController(runtime.commandBus, () => runtime.notify());
			const studioCtrl = new SPlayController(
				runtime.commandBus,
				() => runtime.notify(),
				createStudioStore(createEmptyOsDocument("bootstrap", "Bootstrap")),
				S_PLAY_EXAMPLE_DEFAULT_ID,
				loadSPlayStudioDocument,
			);
			const shell: SOsShellRefs = { home: homeCtrl, studio: studioCtrl };
			const entry = createOsStudio("Routing Studio");
			applySOsUri(runtime, "/", shell);
			expect(runtime.activeAppId).toBe(S_HOME_APP_ID);
			applySOsUri(runtime, `/studios/${entry.id}`, shell);
			expect(runtime.activeAppId).toBe(S_PLAY_APP_ID);
			expect(studioCtrl.getStudioId()).toBe(entry.id);
			deleteOsStudio(entry.id);
		});

		it("checkoutCheckpoint restores studio projection", () => {
			const ctrl = createSPlayTestController("demo");
			const before = ctrl.getStudioStore().projection().appInstances.length;
			ctrl.getStudioStore().dispatch({ kind: "commitCheckpoint", message: "before" });
			ctrl.getStudioStore().dispatch({ kind: "spawnAppInstance", programId: "draw", appId: "draw" });
			const after = ctrl.getStudioStore().projection().appInstances.length;
			expect(after).toBeGreaterThan(before);
			const checkpointId = ctrl.getStudioStore().getDocument().vcs.checkpoints.at(-1)?.id;
			expect(checkpointId).toBeTruthy();
			ctrl.getStudioStore().dispatch({ kind: "checkoutCheckpoint", checkpointId: checkpointId! });
			expect(ctrl.getStudioStore().projection().appInstances.length).toBe(before);
		});
	});
}
// #endregion 🧪Tests

export function sPlayProgramCatalog() {
	return listSPrograms();
}

