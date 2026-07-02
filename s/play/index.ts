// #region 🧲Header
/** @emoji 🖥️ S play harness — unified designer OS shell over all technologies. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	Platform,
	Playground,
	WindowKindRuntime,
	buildSWindowBody,
	createDefaultLayout,
	createPlayAppRuntime,
	createProductPlaygroundPlatform,
	playgroundResolvedFixtureId,
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
} from "@semio-tech/framework-playground-core";
import {
	DevJsonBackbone,
	listSPrograms,
	materializeStudioProjection,
	mergeComposeSketchpadProgramDefinition,
	appInstanceResourceProjection,
	parseSStudioDocument,
	registerAppVcsHandler,
	registerSFixtureJsonResolver,
	createFlowDocumentAppVcsHandler,
	createFlowDagAppVcsHandler,
	createProcedural2dAppVcsHandler,
	createProcedural3dAppVcsHandler,
	createShootingAppVcsHandler,
	createTrinityGraphAppVcsHandler,
	createGisMapAppVcsHandler,
	createPresentationDeckAppVcsHandler,
	createPuzzle2dAppVcsHandler,
	createPuzzle3dAppVcsHandler,
	createCatalogueKindsAppVcsHandler,
	createTypedAppVcsHandler,
	StudioStore,
	type SAppInstance,
	type SMediaGraphNode,
	type SStudioDocumentV1,
	type StudioCommand,
} from "@semio-tech/s-core";
import { createDrawAppVcsHandler } from "@semio-tech/draw-core";
import { createFormsAppVcsHandler } from "@semio-tech/forms-core";
import { createPresentationAppVcsHandler } from "@semio-tech/framework-presentation-core";
import { createRasterAppVcsHandler } from "@semio-tech/raster-core";
import { createWriterAppVcsHandler } from "@semio-tech/writer-core";
import { DEFAULT_SHOOTING_FIXTURE, type ShootingFixture } from "@semio-tech/shooting-react";
import {
	parseModel,
	project2d,
	project3d,
	puzzle5dDefaultManifestCatalogBundle,
	type KindCatalogBundle,
	type Model,
} from "@semio-tech/puzzle-5d-react";
import { S_PLAY_FIXTURE_DEFAULT_ID, resolveSPlayFixtureSlug } from "./fixture-slugs.ts";

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

export const S_PLAY_LAYOUT = createDefaultLayout(
	[S_PLAY_WINDOW_MEDIA_GRAPH, S_PLAY_WINDOW_APP_HOST, S_PLAY_WINDOW_LAUNCHER, S_PLAY_WINDOW_HISTORY],
	"row",
	[34, 46, 10, 10],
	["Media Graph", "App Host", "Launcher", "History"],
);

export { S_PLAY_FIXTURE_DEFAULT_ID, resolveSPlayFixtureSlug };

const sFixtureModules = import.meta.glob("../fixture/*.s.json", { eager: true }) as Record<string, { default: unknown }>;
const technologyFixtureModules = import.meta.glob(["../../draw/fixture/*.json", "../../writer/fixture/*.json"], {
	eager: true,
}) as Record<string, { default: unknown }>;

function fixtureSlugFromPath(path: string): string {
	return path.split("/").pop() ?? path;
}

const S_FIXTURE_JSON_BY_SLUG: Readonly<Record<string, string>> = {
	...Object.fromEntries(
		Object.entries(technologyFixtureModules).map(([path, module]) => [fixtureSlugFromPath(path), JSON.stringify(module.default)]),
	),
};

function sFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.s\.json$/, "");
}

export const S_PLAY_FIXTURE_JSON_BY_ID: Readonly<Record<string, string>> = Object.fromEntries(
	Object.entries(sFixtureModules).map(([path, module]) => [sFixtureIdFromGlobPath(path), JSON.stringify(module.default)]),
);

export const S_PLAY_FIXTURE_OPTIONS = Object.keys(S_PLAY_FIXTURE_JSON_BY_ID)
	.sort()
	.map((id) => ({ id, label: id }));

function meshUrlFromModel(model: Model): string | null {
	const fixture3d = project3d(model);
	for (const object of fixture3d.objects) {
		if (object.meshUrl) return object.meshUrl;
	}
	return null;
}

function createSPlayPuzzle5dAppVcsHandler() {
	return createTypedAppVcsHandler<Model, { readonly op: "setRevision"; readonly revision: number }>(
		"puzzle.5d",
		"puzzle.5d",
		() => ({
			schema: "puzzle.5d",
			domain: "architecture",
			camera2d: { x: 0, y: 0, zoom: 1 },
			camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
			parts: [],
			fasteners: [],
		}),
		(doc, op) => doc,
		undefined,
		{
			applyInputBindings: (model, inputBindings) => {
				const catalogue = inputBindings.catalogue as KindCatalogBundle | undefined;
				if (!catalogue) return model;
				return { ...model, kindCatalogs: catalogue };
			},
			projectOutput: (model, portId) => {
				if (portId === "graph2d") return project2d(model);
				if (portId === "mesh3d") {
					const url = meshUrlFromModel(model);
					return { url: url ?? "/mesh/base.glb" };
				}
				return model;
			},
		},
	);
}

function registerSTechnologyAppVcsHandlers(): void {
	registerAppVcsHandler(createDrawAppVcsHandler());
	registerAppVcsHandler(createWriterAppVcsHandler());
	registerAppVcsHandler(createRasterAppVcsHandler());
	registerAppVcsHandler(createFormsAppVcsHandler());
	registerAppVcsHandler(createFlowDocumentAppVcsHandler());
	registerAppVcsHandler(createFlowDagAppVcsHandler());
	registerAppVcsHandler(createProcedural2dAppVcsHandler());
	registerAppVcsHandler(createProcedural3dAppVcsHandler());
	registerAppVcsHandler(
		createTypedAppVcsHandler<ShootingFixture, { readonly op: "noop" }>(
			"shooting.scene",
			"shooting.fixture",
			() => DEFAULT_SHOOTING_FIXTURE,
			(fixture) => fixture,
			undefined,
			{
				applyInputBindings: (fixture, inputBindings) => {
					const mesh = inputBindings.mesh as { readonly url?: string } | undefined;
					if (!mesh?.url) return fixture;
					const activeId = fixture.activeAssetId ?? fixture.assets[0]?.id;
					if (!activeId) return fixture;
					return {
						...fixture,
						assets: fixture.assets.map((asset) => (asset.id === activeId ? { ...asset, url: mesh.url! } : asset)),
					};
				},
			},
		),
	);
	registerAppVcsHandler(createTrinityGraphAppVcsHandler());
	registerAppVcsHandler(createGisMapAppVcsHandler());
	registerAppVcsHandler(createPresentationDeckAppVcsHandler());
	registerAppVcsHandler(createPresentationAppVcsHandler());
	registerAppVcsHandler(createPuzzle2dAppVcsHandler());
	registerAppVcsHandler(createPuzzle3dAppVcsHandler());
	registerAppVcsHandler(createSPlayPuzzle5dAppVcsHandler());
	registerAppVcsHandler(createCatalogueKindsAppVcsHandler(() => puzzle5dDefaultManifestCatalogBundle() ?? {}));
}

registerSTechnologyAppVcsHandlers();
registerSFixtureJsonResolver((slug) => S_FIXTURE_JSON_BY_SLUG[slug] ?? null);
void import("@semio-tech/compose-sketchpad").then(({ buildSketchpadProgramDefinition }) => {
	mergeComposeSketchpadProgramDefinition(buildSketchpadProgramDefinition());
});

function loadStudioDocument(fixtureId: string): SStudioDocumentV1 {
	const json = S_PLAY_FIXTURE_JSON_BY_ID[fixtureId];
	if (!json) throw new Error(`unknown s fixture: ${fixtureId}`);
	return parseSStudioDocument(JSON.parse(json));
}

function createStudioStore(document: SStudioDocumentV1): StudioStore {
	const backbone = new DevJsonBackbone();
	if (document.backbone?.uri) backbone.attach(document.backbone.uri);
	const store = new StudioStore(document, {
		onAfterMutation: () => {
			backbone.sync(store.getDocument());
		},
	});
	return store;
}

export class SPlayController extends Controller {
	private store: StudioStore;
	private activeInstanceId: string | null = null;
	private selectedMediaNodeIds: string[] = [];
	private selectedAppInstanceIds: string[] = [];
	private fixtureId = S_PLAY_FIXTURE_DEFAULT_ID;
	private launcherProgramId = listSPrograms()[0]?.id ?? "";
	private launcherEngagementInput = "";
	private historyEngagementInput = "";
	private appHostEngagementInput = "";
	private focusedInstanceId: string | null = null;
	private mediaGraphEngagementInput = "";
	readonly mainMode = new ModeRuntime("main", "S", undefined);

	constructor(commandBus: CommandBus, notify: () => void) {
		super(S_PLAY_CONTROLLER_ID, commandBus, notify);
		this.store = createStudioStore(loadStudioDocument(S_PLAY_FIXTURE_DEFAULT_ID));
		const projection = this.store.projection();
		this.activeInstanceId = projection.appInstances[0]?.id ?? null;
		this.rebuildShellMode();
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
		return this.selectedMediaNodeIds;
	}

	getSelectedAppInstanceIds(): readonly string[] {
		return this.selectedAppInstanceIds;
	}

	getFixtureId(): string {
		return this.fixtureId;
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
				this.selectedMediaNodeIds = nodeIds;
				const projection = this.store.projection();
				this.selectedAppInstanceIds = nodeIds
					.map((nodeId) => projection.mediaGraph.nodes.find((node) => node.id === nodeId)?.instanceId)
					.filter((id): id is string => Boolean(id));
				if (this.selectedAppInstanceIds.length === 1) {
					this.activeInstanceId = this.selectedAppInstanceIds[0]!;
				}
				this.emit();
				return;
			}
			case "setAppInstanceSelection": {
				const instanceIds = Array.isArray(args?.instanceIds) ? args!.instanceIds.map((id) => String(id)) : [];
				this.selectedAppInstanceIds = instanceIds;
				const projection = this.store.projection();
				this.selectedMediaNodeIds = instanceIds
					.map((instanceId) => projection.mediaGraph.nodes.find((node) => node.instanceId === instanceId)?.id)
					.filter((id): id is string => Boolean(id));
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
					this.selectedAppInstanceIds = [this.activeInstanceId];
					const node = this.store.projection().mediaGraph.nodes.find((row) => row.instanceId === this.activeInstanceId);
					this.selectedMediaNodeIds = node ? [node.id] : [];
				} else {
					this.selectedAppInstanceIds = [];
					this.selectedMediaNodeIds = [];
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
				const fixtureId = playgroundResolvedFixtureId(String(args?.fixtureId ?? S_PLAY_FIXTURE_DEFAULT_ID));
				this.fixtureId = fixtureId;
				this.store = createStudioStore(loadStudioDocument(fixtureId));
				const projection = this.store.projection();
				this.activeInstanceId = projection.appInstances[0]?.id ?? null;
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
}

export class PlaygroundS extends Playground {
	readonly id = S_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id, "S");
		const ctrl = new SPlayController(runtime.commandBus, () => runtime.notify());
		const resolved = playgroundResolvedFixtureId(S_PLAY_FIXTURE_DEFAULT_ID);
		if (S_PLAY_FIXTURE_JSON_BY_ID[resolved]) ctrl.run("setActiveFixture", { fixtureId: resolved });
		runtime.addApp(buildSPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerSPlayDeclarativeBodies();
	}
}

export function sPlayProgramCatalog() {
	return listSPrograms();
}

/** @emoji 🧩 Verifies compose sketchpad program id aligns with s registry. */
export async function sSketchpadProgramFromCompose() {
	const { buildSketchpadProgramDefinition } = await import("@semio-tech/compose-sketchpad");
	return buildSketchpadProgramDefinition();
}

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "s") {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		try {
			await import("./globals.css");
			const { bootSPlay } = await import("@semio-tech/framework-playground-renderer-react/s");
			bootSPlay(new PlaygroundS());
		} catch (error) {
			console.error("[DEBUG] S play boot failed:", error);
		}
	})();
}

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("S_PLAY_FIXTURE_OPTIONS", () => {
		it("includes demo fixture", () => {
			expect(S_PLAY_FIXTURE_OPTIONS.some((row) => row.id === "demo")).toBe(true);
		});
	});

	describe("SPlayController", () => {
		it("loads demo studio projection", () => {
			const runtime = createProductPlaygroundPlatform("s-test");
			const ctrl = new SPlayController(runtime.commandBus, () => runtime.notify());
			ctrl.run("setActiveFixture", { fixtureId: "demo" });
			const projection = ctrl.getStore().projection();
			expect(projection.appInstances.length).toBeGreaterThanOrEqual(2);
			expect(projection.mediaGraph.nodes.length).toBeGreaterThanOrEqual(2);
		});

		it("round-trips checkpoint save on studio document", () => {
			const runtime = createProductPlaygroundPlatform("s-test");
			const ctrl = new SPlayController(runtime.commandBus, () => runtime.notify());
			ctrl.dispatch({ kind: "commitCheckpoint", message: "snapshot" });
			const doc = ctrl.getStore().getDocument();
			const rematerialized = materializeStudioProjection(doc, doc.vcs.checkpoints[0]?.changeIds ?? []);
			expect(rematerialized.appInstances.length).toBe(ctrl.getStore().projection().appInstances.length);
		});

		it("demo fixture includes cross-instance media edge", () => {
			const runtime = createProductPlaygroundPlatform("s-test");
			const ctrl = new SPlayController(runtime.commandBus, () => runtime.notify());
			ctrl.run("setActiveFixture", { fixtureId: "demo" });
			const projection = ctrl.getStore().projection();
			expect(projection.mediaGraph.edges.length).toBeGreaterThanOrEqual(1);
			expect(projection.appInstances.length).toBeGreaterThanOrEqual(5);
		});

		it("aligns compose sketchpad program with s registry", async () => {
			const composeProgram = await sSketchpadProgramFromCompose();
			mergeComposeSketchpadProgramDefinition(composeProgram);
			const sProgram = listSPrograms().find((program) => program.id === "compose.sketchpad");
			expect(composeProgram.id).toBe("compose.sketchpad");
			expect(sProgram?.apps.length).toBe(composeProgram.apps.length);
		});

		it("resolves draw fixture payload refs", () => {
			const runtime = createProductPlaygroundPlatform("s-test");
			const ctrl = new SPlayController(runtime.commandBus, () => runtime.notify());
			ctrl.run("setActiveFixture", { fixtureId: "demo" });
			const drawInstance = ctrl.getStore().projection().appInstances.find((entry) => entry.programId === "draw");
			expect(drawInstance).toBeTruthy();
			const bundle = appInstanceResourceProjection(
				ctrl.getStore().projection().mediaGraph,
				ctrl.getStore().projection().appInstances,
				drawInstance!.id,
			);
			expect(bundle?.projection).toBeTruthy();
		});

		it("openInstance and closeFocusedInstance toggle drill-in focus", () => {
			const runtime = createProductPlaygroundPlatform("s-test");
			const ctrl = new SPlayController(runtime.commandBus, () => runtime.notify());
			ctrl.run("setActiveFixture", { fixtureId: "demo" });
			const instanceId = ctrl.getStore().projection().appInstances[0]?.id;
			expect(instanceId).toBeTruthy();
			ctrl.run("openInstance", { instanceId });
			expect(ctrl.getFocusedInstanceId()).toBe(instanceId);
			ctrl.run("closeFocusedInstance");
			expect(ctrl.getFocusedInstanceId()).toBeNull();
		});

		it("spawns puzzle5d and shooting with multi-port registrations", () => {
			const runtime = createProductPlaygroundPlatform("s-test");
			const ctrl = new SPlayController(runtime.commandBus, () => runtime.notify());
			ctrl.run("spawnApp", { programId: "puzzle.5d", appId: "puzzle5d", position: { x: 100, y: 100 } });
			ctrl.run("spawnApp", { programId: "shooting", appId: "shooting", position: { x: 300, y: 100 } });
			const projection = ctrl.getStore().projection();
			const puzzleNode = projection.mediaGraph.nodes.find((node) => node.instanceId === projection.appInstances.at(-2)?.id);
			const shootingNode = projection.mediaGraph.nodes.find((node) => node.instanceId === projection.appInstances.at(-1)?.id);
			expect(puzzleNode?.outputs.length).toBe(2);
			expect(shootingNode?.inputs.length).toBe(1);
		});

		it("buildSPlayInspectorTree exposes batch label editing for selected instances", () => {
			const runtime = createProductPlaygroundPlatform("s-test");
			const ctrl = new SPlayController(runtime.commandBus, () => runtime.notify());
			ctrl.run("setActiveFixture", { fixtureId: "demo" });
			const instances = ctrl.getStore().projection().appInstances;
			expect(instances.length).toBeGreaterThanOrEqual(2);
			ctrl.run("setAppInstanceSelection", { instanceIds: instances.slice(0, 2).map((row) => row.id) });
			const tree = buildSPlayInspectorTree(ctrl);
			const section = tree.sections.find((row) => row.id === "s-play-inspector.app-instances");
			const labelField = section?.items.find((item) => item.id === "s-play-inspector.app-instance.label");
			expect(labelField?.control?.type).toBe("input");
			expect(labelField?.control?.onChange?.command).toBe("patchAppInstances");
		});

		it("patchAppInstances updates labels in batch", () => {
			const runtime = createProductPlaygroundPlatform("s-test");
			const ctrl = new SPlayController(runtime.commandBus, () => runtime.notify());
			ctrl.run("setActiveFixture", { fixtureId: "demo" });
			const ids = ctrl.getStore().projection().appInstances.slice(0, 2).map((row) => row.id);
			ctrl.run("patchAppInstances", { instanceIds: ids, field: "label", value: "Batch Label" });
			const labels = ctrl.getStore().projection().appInstances.filter((row) => ids.includes(row.id)).map((row) => row.label);
			expect(labels.every((label) => label === "Batch Label")).toBe(true);
		});
	});
}
// #endregion 🧪Tests
