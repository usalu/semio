// #region 🧲Header
/** @emoji 🖥️ S play app — unified designer OS shell. */
// #endregion 🧲Header

export * from "./internal.ts";
export { sPlayAppDefinition, PlaygroundS } from "./playground.ts";

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	WindowKindRuntime,
	buildSWindowBody,
	createDefaultLayout,
	createPlayAppRuntime,
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
	JackHoverBridge,
	buildWriterWindowBody,
} from "@semio-tech/framework-playground-core";
import { createWriterDocument, type WriterDocumentV1 } from "@semio-tech/writer-core";
import { runJackOnMediaGraph, wireLiteralFromDagFixtureJson } from "@semio-tech/graph-dsl-core";
import {
	DevJsonBackbone,
	listSPrograms,
	materializeStudioProjection,
	mergeSProgramDefinition,
	COMPOSE_SKETCHPAD_PROGRAM_ID,
	appInstanceResourceProjection,
	parseSStudioDocument,
	registerAppVcsHandler,
	registerSFixtureJsonResolver,
	seedSProgramRegistryFromResourceMap,
	sExtensionRegistrySize,
	StudioStore,
	sProgramById,
	createFlowDocumentAppVcsHandler,
	createFlowDagAppVcsHandler,
	createProcedural2dAppVcsHandler,
	createProcedural3dAppVcsHandler,
	createTrinityGraphAppVcsHandler,
	createGisMapAppVcsHandler,
	createPresentationDeckAppVcsHandler,
	createPuzzle2dAppVcsHandler,
	createPuzzle3dAppVcsHandler,
	createSequenceAppVcsHandler,
	createLayoutAppVcsHandler,
	createImperativeAppVcsHandler,
	createLowpolyAppVcsHandler,
	createVcsDemoAppVcsHandler,
	createCatalogueKindsAppVcsHandler,
	sMediaGraphToDagFixtureJson,
	type SAppInstance,
	type SMediaGraphNode,
	type SStudioDocumentV1,
	type StudioCommand,
} from "./internal.ts";
import { createDrawAppVcsHandler } from "@semio-tech/draw-core";
import { createNoteAppVcsHandler } from "@semio-tech/note-core";

export const S_PLAY_APP_ID = "s-play";
export const S_PLAY_CONTROLLER_ID = "s-play";
export const S_PLAY_SURFACE_MEDIA_GRAPH = "s.play.media-graph/v1";
export const S_PLAY_SURFACE_APP_HOST = "s.play.app-host/v1";
export const S_PLAY_SURFACE_LAUNCHER = "s.play.launcher/v1";
export const S_PLAY_SURFACE_HISTORY = "s.play.history/v1";
export const S_PLAY_BODY_LAUNCHER = "s.play.launcher";
export const S_PLAY_BODY_HISTORY = "s.play.history";
export const S_PLAY_WINDOW_LAUNCHER = "s-launcher";
export const S_PLAY_WINDOW_HISTORY = "s-history";
export const S_PLAY_BODY_MEDIA_GRAPH = "s.play.media-graph";
export const S_PLAY_BODY_APP_HOST = "s.play.app-host";
export const S_PLAY_WINDOW_MEDIA_GRAPH = "s-media-graph";
export const S_PLAY_WINDOW_APP_HOST = "s-app-host";
export const S_PLAY_WINDOW_JACK = "s-jack";
export const S_PLAY_WINDOW_COMPILED_DAG = "s-compiled-dag";
export const S_PLAY_BODY_JACK = "s.play.jack";
export const S_PLAY_BODY_COMPILED_DAG = "s.play.compiled-dag";
export const S_PLAY_SURFACE_JACK = "s.play.jack/v1";
export const S_PLAY_SURFACE_COMPILED_DAG = "s.play.compiled-dag/v1";
export const S_PLAY_DEFAULT_JACK_QUERY = "MATCH (n:flow) RETURN n.id";

export const S_PLAY_LAYOUT = createDefaultLayout(
	[S_PLAY_WINDOW_MEDIA_GRAPH, S_PLAY_WINDOW_APP_HOST, S_PLAY_WINDOW_LAUNCHER, S_PLAY_WINDOW_HISTORY, S_PLAY_WINDOW_JACK, S_PLAY_WINDOW_COMPILED_DAG],
	"row",
	[26, 32, 9, 9, 12, 12],
	["Media Graph", "App Host", "Launcher", "History", "Jack", "Compiled DAG"],
);

export type SPlayFixtureLoader = (fixtureId: string) => SStudioDocumentV1;

/** @emoji 🏗️ Creates a studio store for s play. */
export function createStudioStore(document: SStudioDocumentV1): StudioStore {
	const backbone = new DevJsonBackbone();
	if (document.backbone?.uri) backbone.attach(document.backbone.uri);
	const store = new StudioStore(document, {
		onAfterMutation: () => {
			backbone.sync(store.getDocument());
		},
	});
	return store;
}

//#region 🔖SExtensionWiring
import { createFormsAppVcsHandler } from "@semio-tech/forms-core";
import { createPresentationAppVcsHandler } from "@semio-tech/framework-presentation-core";
import { createRasterAppVcsHandler } from "@semio-tech/raster-core";
import { createWriterAppVcsHandler } from "@semio-tech/writer-core";
import { puzzle5dDefaultManifestCatalogBundle } from "@semio-tech/puzzle-5d-react";
import { loadAllSProgramExtensions } from "./program-extensions.ts";
import { createSPlayPuzzle5dAppVcsHandler } from "./puzzle5d-extension.ts";
import { createSPlayShootingAppVcsHandler } from "./shooting-extension.ts";

/** @emoji 🧩 Registers all s technology extensions and VCS handlers. */
export async function bootstrapSPlayExtensions(): Promise<void> {
	seedSProgramRegistryFromResourceMap();
	registerAppVcsHandler(createDrawAppVcsHandler());
	registerAppVcsHandler(createNoteAppVcsHandler());
	registerAppVcsHandler(createWriterAppVcsHandler());
	registerAppVcsHandler(createRasterAppVcsHandler());
	registerAppVcsHandler(createFormsAppVcsHandler());
	registerAppVcsHandler(createFlowDocumentAppVcsHandler());
	registerAppVcsHandler(createFlowDagAppVcsHandler());
	registerAppVcsHandler(createProcedural2dAppVcsHandler());
	registerAppVcsHandler(createProcedural3dAppVcsHandler());
	registerAppVcsHandler(createSPlayShootingAppVcsHandler());
	registerAppVcsHandler(createTrinityGraphAppVcsHandler());
	registerAppVcsHandler(createGisMapAppVcsHandler());
	registerAppVcsHandler(createPresentationDeckAppVcsHandler());
	registerAppVcsHandler(createPresentationAppVcsHandler());
	registerAppVcsHandler(createPuzzle2dAppVcsHandler());
	registerAppVcsHandler(createPuzzle3dAppVcsHandler());
	registerAppVcsHandler(createSPlayPuzzle5dAppVcsHandler());
	registerAppVcsHandler(createSequenceAppVcsHandler());
	registerAppVcsHandler(createLayoutAppVcsHandler());
	registerAppVcsHandler(createImperativeAppVcsHandler());
	registerAppVcsHandler(createLowpolyAppVcsHandler());
	registerAppVcsHandler(createVcsDemoAppVcsHandler());
	registerAppVcsHandler(createCatalogueKindsAppVcsHandler(() => puzzle5dDefaultManifestCatalogBundle() ?? {}));
	const { buildSketchpadProgramDefinition } = await import("@semio-tech/compose-sketchpad");
	mergeSProgramDefinition(COMPOSE_SKETCHPAD_PROGRAM_ID, buildSketchpadProgramDefinition());
	await loadAllSProgramExtensions();
}
//#endregion 🔖SExtensionWiring

/** @emoji 🃏 Builds a Jack-queryable media graph from studio projection. */
export function sPlayMediaGraphForJack(projection: {
	readonly mediaGraph: { readonly nodes: readonly { readonly id: string; readonly instanceId: string }[] };
	readonly appInstances: readonly SAppInstance[];
}): { readonly nodes: readonly { readonly id: string; readonly kind: string; readonly label?: string }[] } {
	return {
		nodes: projection.mediaGraph.nodes.map((node) => {
			const instance = projection.appInstances.find((row) => row.id === node.instanceId);
			return {
				id: node.id,
				kind: instance?.programId ?? "app",
				label: instance?.label ?? node.id,
			};
		}),
	};
}

function sPlayJackBoardFixtureJson(projection: ReturnType<StudioStore["projection"]>): string {
	const graph = sPlayMediaGraphForJack(projection);
	return JSON.stringify({
		nodes: graph.nodes.map((node) => ({ id: node.id, nodeKind: node.kind, text: node.label ?? node.id })),
		edges: [],
	});
}

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

export class SPlayController extends Controller {
	private store: StudioStore;
	private activeInstanceId: string | null = null;
	private fixtureId: string;
	private launcherProgramId = listSPrograms()[0]?.id ?? "";
	private launcherEngagementInput = "";
	private historyEngagementInput = "";
	private appHostEngagementInput = "";
	private focusedInstanceId: string | null = null;
	private mediaGraphEngagementInput = "";
	private readonly jackBridge = new JackHoverBridge();
	private readonly snapshotListeners = new Set<() => void>();
	readonly mainMode = new ModeRuntime("main", "S", undefined);

	constructor(commandBus: CommandBus, notify: () => void, store: StudioStore, fixtureId: string, private readonly loadFixture: SPlayFixtureLoader) {
		super(S_PLAY_CONTROLLER_ID, commandBus, notify);
		this.store = store;
		this.fixtureId = fixtureId;
		const projection = this.store.projection();
		this.activeInstanceId = projection.appInstances[0]?.id ?? null;
		this.jackBridge.setJackQueryText(S_PLAY_DEFAULT_JACK_QUERY);
		this.syncJackFixtureJson();
		this.jackBridge.bindPointerFocus(this.pointerFocus);
		this.rebuildShellMode();
	}

	private syncJackFixtureJson(): void {
		this.jackBridge.setFixtureJson(sPlayJackBoardFixtureJson(this.store.projection()));
	}

	subscribeSnapshot(listener: () => void): () => void {
		this.snapshotListeners.add(listener);
		const unsubJack = this.jackBridge.subscribe(listener);
		return () => {
			this.snapshotListeners.delete(listener);
			unsubJack();
		};
	}

	getJackQueryText(): string {
		return this.jackBridge.getJackQueryText();
	}

	getWriterDocumentJack(): WriterDocumentV1 {
		return createWriterDocument({ id: "s-jack", languageId: "jack", text: this.jackBridge.getJackQueryText() });
	}

	getCompiledWireLiteral(): string {
		const projection = this.store.projection();
		return wireLiteralFromDagFixtureJson(sMediaGraphToDagFixtureJson(projection.mediaGraph, projection.appInstances));
	}

	getWriterDocumentCompiledDag(): WriterDocumentV1 {
		return createWriterDocument({ id: "s-compiled-dag", languageId: "wire", text: this.getCompiledWireLiteral() });
	}

	getJackHoverOccurrences(): readonly { readonly start: number; readonly end: number }[] {
		return this.jackBridge.getJackHoverOccurrences();
	}

	getJackSelectOccurrences(): readonly { readonly start: number; readonly end: number }[] {
		return this.jackBridge.getJackSelectOccurrences();
	}

	getHoverEpoch(): number {
		return this.jackBridge.getHoverEpoch();
	}

	getSelectEpoch(): number {
		return this.jackBridge.getSelectEpoch();
	}

	getGraphHighlightedNodeIds(): readonly string[] {
		return this.jackBridge.getGraphHoveredNodeIds();
	}

	private notifySnapshot(): void {
		for (const listener of this.snapshotListeners) {
			listener();
		}
	}

	private syncJackGraphSelect(): void {
		this.jackBridge.setGraphSelect(this.getSelectedMediaNodeIds());
		this.notifySnapshot();
	}

	getFocusedInstanceId(): string | null {
		return this.focusedInstanceId;
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

	private appHostMeasures(): readonly WindowMeasure[] {
		const projection = this.store.projection();
		return [
			{
				kind: "select",
				id: "s-app-host-instance",
				label: "Instance",
				value: this.activeInstanceId ?? "",
				items: projection.appInstances.map((instance) => ({ id: instance.id, value: instance.id, label: instance.label })),
				onChange: sPlayCmd("selectInstance"),
			},
		];
	}

	private appHostEngagement(): WindowEngagement {
		const active = this.getActiveInstance();
		return {
			sessionActive: false,
			input: {
				id: "s-app-host-label",
				value: this.appHostEngagementInput || active?.label || "",
				placeholder: "Instance label",
				onChange: sPlayCmd("appHostEngagementInput"),
				onSubmit: sPlayCmd("appHostEngagementSubmit"),
			},
			status: active ? [{ id: "s-app-host-program", text: `${active.programId} · ${active.appId}` }] : [],
		};
	}

	private launcherMeasures(): readonly WindowMeasure[] {
		const programs = listSPrograms();
		return [
			{
				kind: "select",
				id: "s-launcher-program",
				label: "Program",
				value: this.launcherProgramId,
				items: programs.map((program) => ({ id: program.id, value: program.id, label: program.name })),
				onChange: sPlayCmd("setLauncherProgram"),
			},
		];
	}

	private launcherEngagement(): WindowEngagement {
		const programs = listSPrograms();
		return {
			sessionActive: false,
			input: {
				id: "s-launcher-spawn",
				value: this.launcherEngagementInput,
				placeholder: "appId to spawn",
				onChange: sPlayCmd("launcherEngagementInput"),
				onSubmit: sPlayCmd("launcherEngagementSubmit"),
			},
			possibleEngagements: programs.slice(0, 4).map((program) => ({
				id: `s-launcher-${program.id}`,
				label: program.name,
				command: sPlayCmd("spawnApp", { programId: program.id, appId: program.apps[0]?.id ?? program.id }),
			})),
		};
	}

	private historyMeasures(): readonly WindowMeasure[] {
		const checkpoints = this.store.getDocument().vcs.checkpoints.length;
		return [
			{
				kind: "slider",
				id: "s-history-checkpoints",
				label: "Checkpoints",
				value: checkpoints,
				min: 0,
				max: Math.max(checkpoints, 1),
				step: 1,
				onChange: sPlayCmd("commitCheckpoint"),
			},
		];
	}

	private historyEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: "s-history-checkpoint",
				value: this.historyEngagementInput,
				placeholder: "Checkpoint message",
				onChange: sPlayCmd("historyEngagementInput"),
				onSubmit: sPlayCmd("historyEngagementSubmit"),
			},
			possibleEngagements: [
				{ id: "s-history-undo", label: "Undo", command: sPlayCmd("undo") },
				{ id: "s-history-redo", label: "Redo", command: sPlayCmd("redo") },
			],
		};
	}

	private jackEngagementInput = "";

	private jackEngagement(): WindowEngagement {
		return {
			input: {
				id: "s-jack-query",
				value: this.jackEngagementInput,
				placeholder: "Jack query on media graph",
				onChange: sPlayCmd("jackEngagementInput"),
				onSubmit: sPlayCmd("runJackQuery"),
			},
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
			new WindowKindRuntime(
				S_PLAY_WINDOW_APP_HOST,
				"App Host",
				S_PLAY_BODY_APP_HOST,
				undefined,
				this.appHostMeasures(),
				this.appHostEngagement(),
			),
			new WindowKindRuntime(
				S_PLAY_WINDOW_LAUNCHER,
				"Launcher",
				S_PLAY_BODY_LAUNCHER,
				undefined,
				this.launcherMeasures(),
				this.launcherEngagement(),
			),
			new WindowKindRuntime(
				S_PLAY_WINDOW_HISTORY,
				"History",
				S_PLAY_BODY_HISTORY,
				undefined,
				this.historyMeasures(),
				this.historyEngagement(),
			),
			new WindowKindRuntime(S_PLAY_WINDOW_JACK, "Jack", S_PLAY_BODY_JACK, undefined, undefined, this.jackEngagement()),
			new WindowKindRuntime(S_PLAY_WINDOW_COMPILED_DAG, "Compiled DAG", S_PLAY_BODY_COMPILED_DAG),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `S play window "${windowKind.id}"`);
		}
	}

	getStore(): StudioStore {
		return this.store;
	}

	getActiveInstanceId(): string | null {
		return this.activeInstanceId;
	}

	getActiveInstance(): SAppInstance | null {
		if (!this.activeInstanceId) return null;
		return this.store.projection().appInstances.find((instance) => instance.id === this.activeInstanceId) ?? null;
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

	dispatch(command: StudioCommand): void {
		this.store.dispatch(command);
		this.syncJackFixtureJson();
		this.emit();
	}

	run(command: string, args?: Record<string, unknown>): void {
		if (command === "setJackQuery") {
			const text = typeof args?.text === "string" ? args.text : "";
			if (text) {
				this.jackBridge.setJackQueryText(text);
				this.notifySnapshot();
				this.emit();
			}
			return;
		}
		if (command === "setJackHover") {
			this.jackBridge.setJackHover((args?.offset as number | null | undefined) ?? null);
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setJackSelect") {
			this.jackBridge.setJackSelect((args as { start: number; end: number } | null) ?? null);
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setGraphHover") {
			this.jackBridge.setGraphHover(typeof args?.id === "string" ? args.id : null);
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setGraphSelect") {
			const ids = Array.isArray(args?.ids) ? args!.ids.map((id) => String(id)) : [];
			this.jackBridge.setGraphSelect(ids);
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "runJackQuery") {
			const projection = this.store.projection();
			runJackOnMediaGraph(projection.mediaGraph, projection.appInstances, this.jackBridge.getJackQueryText());
			this.notifySnapshot();
			this.emit();
			return;
		}
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
			case "appHostEngagementInput": {
				const value = typeof args?.value === "string" ? args.value : "";
				if (value !== this.appHostEngagementInput) {
					this.appHostEngagementInput = value;
					this.rebuildShellMode();
					this.emit();
				}
				return;
			}
			case "appHostEngagementSubmit": {
				const value = String(args?.value ?? this.appHostEngagementInput).trim();
				if (this.activeInstanceId && value) {
					this.store.dispatch({ kind: "patchAppInstances", instanceIds: [this.activeInstanceId], field: "label", value });
					this.appHostEngagementInput = value;
					this.rebuildShellMode();
					this.emit();
				}
				return;
			}
			case "launcherEngagementInput": {
				const value = typeof args?.value === "string" ? args.value : "";
				if (value !== this.launcherEngagementInput) {
					this.launcherEngagementInput = value;
					this.rebuildShellMode();
					this.emit();
				}
				return;
			}
			case "launcherEngagementSubmit": {
				const appId = String(args?.value ?? this.launcherEngagementInput).trim();
				if (this.launcherProgramId && appId) this.run("spawnApp", { programId: this.launcherProgramId, appId });
				return;
			}
			case "jackEngagementInput": {
				const value = typeof args?.value === "string" ? args.value : "";
				if (value !== this.jackEngagementInput) {
					this.jackEngagementInput = value;
					this.rebuildShellMode();
					this.emit();
				}
				return;
			}
			case "historyEngagementInput": {
				const value = typeof args?.value === "string" ? args.value : "";
				if (value !== this.historyEngagementInput) {
					this.historyEngagementInput = value;
					this.rebuildShellMode();
					this.emit();
				}
				return;
			}
			case "historyEngagementSubmit": {
				this.run("commitCheckpoint", { message: String(args?.value ?? this.historyEngagementInput).trim() || undefined });
				this.historyEngagementInput = "";
				this.rebuildShellMode();
				return;
			}
			case "setLauncherProgram": {
				const programId = String(args?.value ?? "");
				if (programId && programId !== this.launcherProgramId) {
					this.launcherProgramId = programId;
					this.rebuildShellMode();
					this.emit();
				}
				return;
			}
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
				this.syncJackGraphSelect();
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
				this.syncJackGraphSelect();
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
				this.syncJackGraphSelect();
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
				this.focusedInstanceId = typeof args?.instanceId === "string" ? args.instanceId : null;
				if (this.focusedInstanceId) this.activeInstanceId = this.focusedInstanceId;
				this.rebuildShellMode();
				this.emit();
				return;
			}
			case "closeFocusedInstance": {
				this.focusedInstanceId = null;
				this.rebuildShellMode();
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
			case "setActiveFixture": {
				const fixtureId = String(args?.fixtureId ?? this.fixtureId);
				this.fixtureId = fixtureId;
				this.store = createStudioStore(this.loadFixture(fixtureId));
				const projection = this.store.projection();
				this.activeInstanceId = projection.appInstances[0]?.id ?? null;
				this.syncJackFixtureJson();
				this.rebuildShellMode();
				this.emit();
				return;
			}
			default:
				return;
		}
	}
}

export function buildSPlayAppRuntime(ctrl: SPlayController): AppRuntime {
	return createPlayAppRuntime(S_PLAY_APP_ID, "S", ctrl, S_PLAY_LAYOUT, ctrl.mainMode);
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

/** @emoji 🔎 Declarative inspection tree for s play media graph and app instances. */
export function buildSPlayInspectorTree(ctrl: SPlayController): UiTreeNode {
	const projection = ctrl.getStore().projection();
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

export function registerSPlayDeclarativeBodies(): void {
	registerWindowBody(S_PLAY_BODY_MEDIA_GRAPH, () =>
		buildSWindowBody(S_PLAY_SURFACE_MEDIA_GRAPH, S_PLAY_CONTROLLER_ID, "mediaGraph", "media-graph"));
	registerWindowBody(S_PLAY_BODY_APP_HOST, () =>
		buildSWindowBody(S_PLAY_SURFACE_APP_HOST, S_PLAY_CONTROLLER_ID, "appHost", "app-host"));
	registerWindowBody(S_PLAY_BODY_LAUNCHER, () =>
		buildSWindowBody(S_PLAY_SURFACE_LAUNCHER, S_PLAY_CONTROLLER_ID, "launcher", "launcher"));
	registerWindowBody(S_PLAY_BODY_HISTORY, () =>
		buildSWindowBody(S_PLAY_SURFACE_HISTORY, S_PLAY_CONTROLLER_ID, "history", "history"));
	registerWindowBody(S_PLAY_BODY_JACK, () =>
		buildWriterWindowBody(S_PLAY_SURFACE_JACK, S_PLAY_CONTROLLER_ID, S_PLAY_WINDOW_JACK));
	registerWindowBody(S_PLAY_BODY_COMPILED_DAG, () =>
		buildWriterWindowBody(S_PLAY_SURFACE_COMPILED_DAG, S_PLAY_CONTROLLER_ID, S_PLAY_WINDOW_COMPILED_DAG));
}

export async function sSketchpadProgramFromCompose() {
	const { buildSketchpadProgramDefinition } = await import("@semio-tech/compose-sketchpad");
	return buildSketchpadProgramDefinition();
}

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it, beforeAll } = import.meta.vitest;

	beforeAll(async () => {
		await bootstrapSPlayExtensions();
	});

	describe("SPlayController", () => {
		it("loads demo studio projection", async () => {
			const { createSPlayTestController } = await import("./playground.ts");
			const ctrl = createSPlayTestController("demo");
			const projection = ctrl.getStore().projection();
			expect(projection.appInstances.length).toBeGreaterThanOrEqual(2);
			expect(projection.mediaGraph.nodes.length).toBeGreaterThanOrEqual(2);
		});

		it("round-trips checkpoint save on studio document", async () => {
			const { createSPlayTestController } = await import("./playground.ts");
			const ctrl = createSPlayTestController("demo");
			ctrl.dispatch({ kind: "commitCheckpoint", message: "snapshot" });
			const doc = ctrl.getStore().getDocument();
			const rematerialized = materializeStudioProjection(doc, doc.vcs.checkpoints[0]?.changeIds ?? []);
			expect(rematerialized.appInstances.length).toBe(ctrl.getStore().projection().appInstances.length);
		});

		it("demo fixture includes cross-instance media edge", async () => {
			const { createSPlayTestController } = await import("./playground.ts");
			const ctrl = createSPlayTestController("demo");
			const projection = ctrl.getStore().projection();
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
			const { createSPlayTestController } = await import("./playground.ts");
			const ctrl = createSPlayTestController("demo");
			const drawInstance = ctrl.getStore().projection().appInstances.find((entry) => entry.programId === "draw");
			expect(drawInstance).toBeTruthy();
			const bundle = appInstanceResourceProjection(
				ctrl.getStore().projection().mediaGraph,
				ctrl.getStore().projection().appInstances,
				drawInstance!.id,
			);
			expect(bundle?.projection).toBeTruthy();
		});

		it("openInstance and closeFocusedInstance toggle drill-in focus", async () => {
			const { createSPlayTestController } = await import("./playground.ts");
			const ctrl = createSPlayTestController("demo");
			const instanceId = ctrl.getStore().projection().appInstances[0]?.id;
			expect(instanceId).toBeTruthy();
			ctrl.run("openInstance", { instanceId });
			expect(ctrl.getFocusedInstanceId()).toBe(instanceId);
			ctrl.run("closeFocusedInstance");
			expect(ctrl.getFocusedInstanceId()).toBeNull();
		});

		it("spawns puzzle5d and shooting with multi-port registrations", async () => {
			const { createSPlayTestController } = await import("./playground.ts");
			const ctrl = createSPlayTestController("demo");
			ctrl.run("spawnApp", { programId: "puzzle.5d", appId: "puzzle5d", position: { x: 100, y: 100 } });
			ctrl.run("spawnApp", { programId: "shooting", appId: "shooting", position: { x: 300, y: 100 } });
			const projection = ctrl.getStore().projection();
			const puzzleNode = projection.mediaGraph.nodes.find((node) => node.instanceId === projection.appInstances.at(-2)?.id);
			const shootingNode = projection.mediaGraph.nodes.find((node) => node.instanceId === projection.appInstances.at(-1)?.id);
			expect(puzzleNode?.outputs.length).toBe(2);
			expect(shootingNode?.inputs.length).toBe(1);
		});

		it("buildSPlayInspectorTree exposes batch label editing for selected instances", async () => {
			const { createSPlayTestController } = await import("./playground.ts");
			const ctrl = createSPlayTestController("demo");
			const instances = ctrl.getStore().projection().appInstances;
			expect(instances.length).toBeGreaterThanOrEqual(2);
			ctrl.run("setAppInstanceSelection", { instanceIds: instances.slice(0, 2).map((row) => row.id) });
			const tree = buildSPlayInspectorTree(ctrl);
			const section = tree.sections.find((row) => row.id === "s-play-inspector.app-instances");
			const labelField = section?.items.find((item) => item.id === "s-play-inspector.app-instance.label");
			expect(labelField?.control?.type).toBe("input");
			expect(labelField?.control?.onChange?.command).toBe("patchAppInstances");
		});

		it("patchAppInstances updates labels in batch", async () => {
			const { createSPlayTestController } = await import("./playground.ts");
			const ctrl = createSPlayTestController("demo");
			const ids = ctrl.getStore().projection().appInstances.slice(0, 2).map((row) => row.id);
			ctrl.run("patchAppInstances", { instanceIds: ids, field: "label", value: "Batch Label" });
			const labels = ctrl.getStore().projection().appInstances.filter((row) => ids.includes(row.id)).map((row) => row.label);
			expect(labels.every((label) => label === "Batch Label")).toBe(true);
		});
	});
}
// #endregion 🧪Tests

export function sPlayProgramCatalog() {
	return listSPrograms();
}
