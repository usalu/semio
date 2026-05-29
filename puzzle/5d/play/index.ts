// #region 🧲Header
// 💻 elements/lib/react/topology/play/index.ts — Topology play on `@framework/playground`: paired board+scene fixtures, LOD measures, relocate tools (no React).
// #endregion 🧲Header

import {
	CommandBus,
	Controller,
	ProductRuntime,
	AppRuntime,
	ModeRuntime,
	WindowKindRuntime,
	buildBoardWindowBody,
	buildScene3dWindowBody,
	createDefaultLayout,
	type ToolItem,
	type WindowBodyViewContext,
	type WindowMeasure,
	type UiNode,
	Playground,
	playgroundTreePanelRootItems,
	type UiTreeItemNode,
	type UiTreeNode,
} from "@framework/playground";

import { buildBoardPlayHierarchySections } from "../../2d/play/index.ts";
import nakaginBoardJson from "../../2d/play/fixtures/nakagin-capsule-tower.board.json";
import {
	BOARD_LOD_MODE_AUTOMATIC,
	boardLodAutomaticSelectLabel,
	boardLodCanvasProps,
	isBoardDrawLodKind,
	parseBoardFixtureV1,
	type BoardDrawLodKind,
	type BoardFixtureV1,
	type BoardLodModeKind,
	type CameraState,
} from "../../2d/react/index.tsx";
import nakaginSceneJson from "../../3d/play/fixtures/nakagin-capsule-tower.scene.json";
import { buildScenePlayHierarchyTree, SCENE_PLAY_EMPTY_SELECTION } from "../../3d/play/index.ts";
import {
	DEFAULT_MANUAL_LOD,
	SCENE_LOD_SLIDER_MAX,
	SCENE_LOD_SLIDER_MIN,
	formatSceneLod,
	lodFromSliderValue,
	parseFixtureV1,
	sceneLodCanvasProps,
	sliderValueFromLod,
	type FixtureV1 as SceneFixtureV1,
	type RelocateMode as SceneRelocateMode,
} from "../../3d/react/index.tsx";
import { parseTopologyFixtureV1, topologySharedKindsFromPairedMetas } from "../react/index.tsx";
import topologyManifestJson from "./fixtures/nakagin-capsule-tower.topology.json";

//#region 🔖Ids
export const TOPOLOGY_PLAY_APP_ID = "elements-topology-play";
export const TOPOLOGY_PLAY_CONTROLLER_ID = "topology-play";
export const TOPOLOGY_PLAY_BOARD_WINDOW_ID = "topology-board";
export const TOPOLOGY_PLAY_SCENE_WINDOW_ID = "topology-scene";
export const TOPOLOGY_PLAY_BOARD_WINDOW_LABEL = "Sketch board";
export const TOPOLOGY_PLAY_SCENE_WINDOW_LABEL = "Spatial scene";
export const TOPOLOGY_PLAY_BOARD_BODY_KEY = "elements.topology.play.board";
export const TOPOLOGY_PLAY_SCENE_BODY_KEY = "elements.topology.play.scene";
export const TOPOLOGY_PLAY_BOARD_SURFACE_ID = "elements.topology.play.board/v1";
export const TOPOLOGY_PLAY_SCENE_SURFACE_ID = "elements.topology.play.scene/v1";
export const TOPOLOGY_PLAY_HIERARCHY_TAB_ID = "topology-play-hierarchy";

const TOPOLOGY_PLAY_LOD_TIERS_BOARD: readonly BoardDrawLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];
//#endregion 🔖Ids

//#region 🔖TopologyPlayHierarchy
export interface TopologyPlayHierarchySelectHandlers {
	readonly onSelectBoard: (id: string) => void;
	readonly onSelectSceneObject: (objectId: string) => void;
	readonly onSelectSceneVortex: (vortexFullId: string) => void;
	readonly onSelectSceneAttraction: (attractionId: string) => void;
}

/** @emoji 🌳 Paired topology tree: manifest → Board + Scene composition subtrees. */
export function buildTopologyPlayHierarchySections(
	snapshot: TopologyPlaySnapshot,
	handlers: TopologyPlayHierarchySelectHandlers,
): UiTreeNode {
	const branches: UiTreeItemNode[] = [];
	if (snapshot.boardFixture) {
		const boardRoot = buildBoardPlayHierarchySections(snapshot.boardFixture, [...snapshot.boardSelected], handlers.onSelectBoard)
			.sections[0]?.items?.[0];
		branches.push({
			id: "topology-play-hierarchy.board",
			label: "Board",
			defaultOpen: true,
			items: boardRoot?.items ?? [{ id: "topology-play-hierarchy.board.empty", label: "(empty)" }],
		});
	}
	if (snapshot.sceneFixture) {
		const sceneSelection = snapshot.sceneSelected
			? { ...SCENE_PLAY_EMPTY_SELECTION, objectIds: [snapshot.sceneSelected] }
			: SCENE_PLAY_EMPTY_SELECTION;
		const sceneRoot = buildScenePlayHierarchyTree(snapshot.sceneFixture, sceneSelection).sections[0]?.items?.[0];
		branches.push({
			id: "topology-play-hierarchy.scene",
			label: "Scene",
			defaultOpen: true,
			items: sceneRoot?.items ?? [{ id: "topology-play-hierarchy.scene.empty", label: "(empty)" }],
		});
	}
	const topologyRoot: UiTreeItemNode = {
		id: "topology-play-hierarchy.topology",
		label: snapshot.manifestLabel ?? "Topology",
		defaultOpen: true,
		items: branches.length ? branches : [{ id: "topology-play-hierarchy.topology.empty", label: "(no fixtures)" }],
	};
	return playgroundTreePanelRootItems("topology-play-hierarchy.root", [topologyRoot]);
}
//#endregion 🔖TopologyPlayHierarchy

//#region 🔖Helpers
function topologyPlayLodTierMenuLabel(tier: string): string {
	return tier.charAt(0).toUpperCase() + tier.slice(1);
}

function topologyControllerFromContext(ctx: WindowBodyViewContext): TopologyPlayShellController | undefined {
	return ctx.runtime.getActiveApp()?.controller as TopologyPlayShellController | undefined;
}

function sameCamera(a: CameraState | null, b: CameraState): boolean {
	return Boolean(a && a.x === b.x && a.y === b.y && a.zoom === b.zoom);
}
//#endregion 🔖Helpers

//#region 🔖Controller
export interface TopologyPlaySnapshot {
	readonly manifestLabel: string | undefined;
	readonly boardFixture: BoardFixtureV1 | null;
	readonly sceneFixture: SceneFixtureV1 | null;
	readonly boardSelected: ReadonlySet<string>;
	readonly boardCamera: CameraState | null;
	readonly sceneCamera: CameraState | null;
	readonly sceneSelected: string | null;
	readonly relocateMode: SceneRelocateMode;
	readonly sceneLodTag: number;
	readonly boardLodTag: BoardDrawLodKind;
	readonly boardLodProps: ReturnType<typeof boardLodCanvasProps>;
	readonly sceneLodProps: ReturnType<typeof sceneLodCanvasProps>;
	readonly sceneAutomaticLod: boolean;
	readonly sceneDepthVariableLod: boolean;
	readonly sceneLodSlider: number;
	readonly sharedKinds: ReturnType<typeof topologySharedKindsFromPairedMetas>;
	readonly connectBoard: number;
	readonly connectScene: number;
	readonly proximityBoard: number;
	readonly proximityScene: number;
}

/** @emoji 🎛 Topology play shell controller shared by declarative board and scene windows. */
export class TopologyPlayShellController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Topology", undefined);
	readonly manifest = parseTopologyFixtureV1(topologyManifestJson as unknown);
	readonly boardFixture = parseBoardFixtureV1(nakaginBoardJson as unknown);
	readonly sceneFixture = parseFixtureV1(nakaginSceneJson as unknown);
	private relocateMode: SceneRelocateMode = "translate";
	private boardSelected: ReadonlySet<string> = new Set();
	private sceneSelected: string | null = null;
	private boardCamera: CameraState | null = this.boardFixture ? { ...this.boardFixture.camera } : null;
	private sceneCamera: CameraState | null = this.sceneFixture ? { ...this.sceneFixture.camera } : null;
	private sceneLodTag = DEFAULT_MANUAL_LOD;
	private sceneAutomaticLod = true;
	private sceneDepthVariableLod = false;
	private sceneManualLod = DEFAULT_MANUAL_LOD;
	private sceneLodSlider = sliderValueFromLod(DEFAULT_MANUAL_LOD);
	private boardLodTag: BoardDrawLodKind = "normal";
	private boardLodMode: BoardLodModeKind = BOARD_LOD_MODE_AUTOMATIC;
	private connectBoard = 0;
	private connectScene = 0;
	private proximityBoard = 0;
	private proximityScene = 0;

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(TOPOLOGY_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.rebuildShellMode();
	}

	private rebuildShellMode(): void {
		const relocateTools: ToolItem[] = (["translate", "rotate", "scale"] as const).map((mode, order) => ({
			id: `topology.relocate.${mode}`,
			kind: "toggle" as const,
			text: mode.charAt(0).toUpperCase() + mode.slice(1),
			order,
			pressed: this.relocateMode === mode,
			controllerId: TOPOLOGY_PLAY_CONTROLLER_ID,
			command: "setRelocateMode",
			args: { mode },
		}));
		this.mainMode.tools = { actions: relocateTools };
		this.mainMode.windowKinds = this.getWindowKinds();
	}

	private boardLodMeasure(): WindowMeasure {
		return {
			kind: "select",
			id: `${TOPOLOGY_PLAY_BOARD_WINDOW_ID}-lod`,
			label: "LOD",
			value: this.boardLodMode,
			items: [
				{ id: "automatic", label: boardLodAutomaticSelectLabel(this.boardLodTag), value: BOARD_LOD_MODE_AUTOMATIC },
				...TOPOLOGY_PLAY_LOD_TIERS_BOARD.map((tier) => ({ id: tier, label: topologyPlayLodTierMenuLabel(tier), value: tier })),
			],
			onChange: { controllerId: TOPOLOGY_PLAY_CONTROLLER_ID, command: "setBoardLodMode" },
		};
	}

	private sceneLodMeasures(): readonly WindowMeasure[] {
		return [
			{
				kind: "toggle",
				id: `${TOPOLOGY_PLAY_SCENE_WINDOW_ID}-auto`,
				label: "LOD",
				text: "Auto zoom",
				pressed: this.sceneAutomaticLod,
				onChange: { controllerId: TOPOLOGY_PLAY_CONTROLLER_ID, command: "setSceneAutoLod" },
			},
			{
				kind: "toggle",
				id: `${TOPOLOGY_PLAY_SCENE_WINDOW_ID}-depth`,
				text: "Depth-variable",
				pressed: this.sceneDepthVariableLod,
				onChange: { controllerId: TOPOLOGY_PLAY_CONTROLLER_ID, command: "setSceneDepthLod" },
			},
			{
				kind: "slider",
				id: `${TOPOLOGY_PLAY_SCENE_WINDOW_ID}-lod`,
				label: formatSceneLod(this.sceneLodTag),
				value: this.sceneLodSlider,
				min: SCENE_LOD_SLIDER_MIN,
				max: SCENE_LOD_SLIDER_MAX,
				step: 1,
				onChange: { controllerId: TOPOLOGY_PLAY_CONTROLLER_ID, command: "setSceneManualLod" },
			},
		];
	}

	getWindowKinds(): readonly WindowKindRuntime[] {
		return [
			new WindowKindRuntime(TOPOLOGY_PLAY_BOARD_WINDOW_ID, TOPOLOGY_PLAY_BOARD_WINDOW_LABEL, TOPOLOGY_PLAY_BOARD_BODY_KEY, undefined, [this.boardLodMeasure()]),
			new WindowKindRuntime(TOPOLOGY_PLAY_SCENE_WINDOW_ID, TOPOLOGY_PLAY_SCENE_WINDOW_LABEL, TOPOLOGY_PLAY_SCENE_BODY_KEY, undefined, [...this.sceneLodMeasures()]),
		];
	}

	override run(command: string, args?: unknown): void {
		let changed = true;
		switch (command) {
			case "setBoardLodMode": {
				const value = (args as { value?: string }).value;
				if ((value === BOARD_LOD_MODE_AUTOMATIC || (typeof value === "string" && isBoardDrawLodKind(value))) && this.boardLodMode !== value) this.boardLodMode = value as BoardLodModeKind;
				else changed = false;
				break;
			}
			case "setSceneAutoLod": {
				const pressed = (args as { pressed?: boolean }).pressed;
				if (typeof pressed === "boolean" && this.sceneAutomaticLod !== pressed) this.sceneAutomaticLod = pressed;
				else changed = false;
				break;
			}
			case "setSceneDepthLod": {
				const pressed = (args as { pressed?: boolean }).pressed;
				if (typeof pressed === "boolean" && this.sceneDepthVariableLod !== pressed) this.sceneDepthVariableLod = pressed;
				else changed = false;
				break;
			}
			case "setSceneManualLod": {
				const value = (args as { value?: number }).value;
				if (typeof value === "number" && Number.isFinite(value)) {
					this.sceneLodSlider = value;
					this.sceneManualLod = lodFromSliderValue(value);
				} else changed = false;
				break;
			}
			case "setBoardLodTag": {
				const lod = (args as { lod: BoardDrawLodKind }).lod;
				if (this.boardLodTag !== lod) this.boardLodTag = lod;
				else changed = false;
				break;
			}
			case "setSceneLodTag": {
				const lod = (args as { lod: number }).lod;
				if (typeof lod === "number" && Number.isFinite(lod) && lod > 0) {
					this.sceneLodTag = lod;
				}
				changed = false;
				break;
			}
			case "setBoardSelection": {
				const ids = (args as { ids: readonly string[] }).ids;
				if (ids.length !== this.boardSelected.size || ids.some((id) => !this.boardSelected.has(id))) this.boardSelected = new Set(ids);
				else changed = false;
				break;
			}
			case "setSceneSelection": {
				const selected = (args as { objectIds: readonly string[] }).objectIds[0] ?? null;
				if (this.sceneSelected !== selected) this.sceneSelected = selected;
				else changed = false;
				break;
			}
			case "setBoardCamera": {
				const camera = (args as { camera: CameraState }).camera;
				if (!sameCamera(this.boardCamera, camera)) this.boardCamera = { ...camera };
				else changed = false;
				break;
			}
			case "setSceneCamera": {
				const camera = (args as { camera: CameraState }).camera;
				if (!sameCamera(this.sceneCamera, camera)) this.sceneCamera = { ...camera };
				else changed = false;
				break;
			}
			case "setRelocateMode": {
				const mode = (args as { mode: SceneRelocateMode }).mode;
				if (this.relocateMode !== mode) this.relocateMode = mode;
				else changed = false;
				break;
			}
			case "noteBoardConnect":
				this.connectBoard += 1;
				break;
			case "noteSceneConnect":
				this.connectScene += 1;
				break;
			case "noteBoardProximity":
				this.proximityBoard += 1;
				break;
			case "noteSceneProximity":
				this.proximityScene += 1;
				break;
			default:
				changed = false;
				break;
		}
		if (changed) {
			this.rebuildShellMode();
			this.emit();
		}
	}

	getSnapshot(): TopologyPlaySnapshot {
		return {
			manifestLabel: this.manifest?.label,
			boardFixture: this.boardFixture,
			sceneFixture: this.sceneFixture,
			boardSelected: this.boardSelected,
			boardCamera: this.boardCamera,
			sceneCamera: this.sceneCamera,
			sceneSelected: this.sceneSelected,
			relocateMode: this.relocateMode,
			sceneLodTag: this.sceneLodTag,
			boardLodTag: this.boardLodTag,
			boardLodProps: boardLodCanvasProps(this.boardLodMode),
			sceneLodProps: sceneLodCanvasProps({
				automaticLod: this.sceneAutomaticLod,
				depthVariableLod: this.sceneDepthVariableLod,
				manualLod: this.sceneManualLod,
			}),
			sceneAutomaticLod: this.sceneAutomaticLod,
			sceneDepthVariableLod: this.sceneDepthVariableLod,
			sceneLodSlider: this.sceneLodSlider,
			sharedKinds: topologySharedKindsFromPairedMetas({ boardMeta: this.boardFixture?.meta, sceneMeta: this.sceneFixture?.meta }),
			connectBoard: this.connectBoard,
			connectScene: this.connectScene,
			proximityBoard: this.proximityBoard,
			proximityScene: this.proximityScene,
		};
	}
}
//#endregion 🔖Controller

//#region 🔖TopologyPlayRuntime
export function buildTopologyPlayAppRuntime(controller: TopologyPlayShellController): AppRuntime {
	const app = new AppRuntime(
		TOPOLOGY_PLAY_APP_ID,
		"Topology play",
		undefined,
		controller,
		createDefaultLayout([TOPOLOGY_PLAY_BOARD_WINDOW_ID, TOPOLOGY_PLAY_SCENE_WINDOW_ID], "row", [50, 50], [TOPOLOGY_PLAY_BOARD_WINDOW_LABEL, TOPOLOGY_PLAY_SCENE_WINDOW_LABEL]) as never,
		controller.getWindowKinds(),
	);
	app.defaultModeId = controller.mainMode.id;
	app.addMode(controller.mainMode);
	app.leftTabs = [];
	app.rightTabs = [];
	return app;
}

export function buildTopologyPlayRuntime(): ProductRuntime {
	const runtime = new ProductRuntime();
	const controller = new TopologyPlayShellController(runtime.commandBus, () => runtime.notify());
	runtime.addApp(buildTopologyPlayAppRuntime(controller));
	return runtime;
}

/** @emoji 🛝 Topology play harness as a single {@link Playground} instance. */
export class TopologyPlayground extends Playground {
	readonly id = TOPOLOGY_PLAY_APP_ID;
	readonly puzzleChrome = "topology" as const;
	readonly initialPanelVisibility = { leftSidePanel: true, rightSidePanel: true };

	createRuntime(): ProductRuntime {
		return buildTopologyPlayRuntime();
	}

	registerBodies(): void {
		/* window bodies registered with surface hosts in {@link registerTopologyPlaySurfaceHosts} */
	}
}
//#endregion 🔖TopologyPlayRuntime

//#region 🔖DeclarativeBodies
export function buildTopologyBoardDeclarativeBody(ctx: WindowBodyViewContext): UiNode {
	const ctrl = topologyControllerFromContext(ctx);
	const snap = ctrl?.getSnapshot();
	if (!snap?.boardFixture) return { type: "text", value: "Invalid board fixture" };
	return buildBoardWindowBody(TOPOLOGY_PLAY_BOARD_SURFACE_ID, TOPOLOGY_PLAY_CONTROLLER_ID, TOPOLOGY_PLAY_BOARD_WINDOW_ID);
}

export function buildTopologySceneDeclarativeBody(ctx: WindowBodyViewContext): UiNode {
	const ctrl = topologyControllerFromContext(ctx);
	const snap = ctrl?.getSnapshot();
	if (!snap?.sceneFixture) return { type: "text", value: "Invalid scene fixture" };
	return buildScene3dWindowBody(TOPOLOGY_PLAY_SCENE_SURFACE_ID, TOPOLOGY_PLAY_CONTROLLER_ID);
}
//#endregion 🔖DeclarativeBodies

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("topology play hierarchy", () => {
		it("buildTopologyPlayHierarchySections includes Board and Scene branches", () => {
			const runtime = buildTopologyPlayRuntime();
			const controller = runtime.getActiveApp()?.controller as TopologyPlayShellController;
			expect(controller).toBeTruthy();
			const tree = buildTopologyPlayHierarchySections(controller!.getSnapshot(), {
				onSelectBoard: () => {},
				onSelectSceneObject: () => {},
				onSelectSceneVortex: () => {},
				onSelectSceneAttraction: () => {},
			});
			const topologyRoot = tree.sections[0]?.items?.[0];
			expect(topologyRoot?.label).toBeTruthy();
			const labels = topologyRoot?.items?.map((row) => row.label);
			expect(labels).toContain("Board");
			expect(labels).toContain("Scene");
		});
	});

	describe("topology play fixtures", () => {
		it("parses nakagin board and scene", () => {
			const b = parseBoardFixtureV1(nakaginBoardJson as unknown);
			const s = parseFixtureV1(nakaginSceneJson as unknown);
			expect(b?.nodes.length).toBeGreaterThan(0);
			expect(s?.objects.length).toBeGreaterThan(0);
		});
		it("parses topology manifest", () => {
			const t = parseTopologyFixtureV1(topologyManifestJson as unknown);
			expect(t?.schema).toBe("elements.topology.fixture/v1");
		});
		it("shared kinds merge metas like the play harness", () => {
			const sk = topologySharedKindsFromPairedMetas({
				boardMeta: undefined,
				sceneMeta: { kindCompatibility: [{ source: "u", target: "v" }] },
			});
			expect(sk.kindCompatibility?.length).toBeGreaterThan(0);
		});
		it("builds declarative board and scene canvas-only bodies", () => {
			const wb = buildTopologyPlayRuntime();
			const board = buildTopologyBoardDeclarativeBody({
				runtime: wb,
				windowKindId: TOPOLOGY_PLAY_BOARD_WINDOW_ID,
				bodyKey: TOPOLOGY_PLAY_BOARD_BODY_KEY,
				activeModeId: "main",
				generation: 0,
			});
			const scene = buildTopologySceneDeclarativeBody({
				runtime: wb,
				windowKindId: TOPOLOGY_PLAY_SCENE_WINDOW_ID,
				bodyKey: TOPOLOGY_PLAY_SCENE_BODY_KEY,
				activeModeId: "main",
				generation: 0,
			});
			expect(board).toEqual(buildBoardWindowBody(TOPOLOGY_PLAY_BOARD_SURFACE_ID, TOPOLOGY_PLAY_CONTROLLER_ID, TOPOLOGY_PLAY_BOARD_WINDOW_ID));
			expect(scene).toEqual(buildScene3dWindowBody(TOPOLOGY_PLAY_SCENE_SURFACE_ID, TOPOLOGY_PLAY_CONTROLLER_ID));
		});
	});
}
//#endregion 🧪Tests
