// #region 🧲Header
// 💻 elements/client/lib/topology/play/react.tsx — Class-based topology play runtime for paired board and scene surfaces.
// #endregion 🧲Header

import { useGLTF } from "@react-three/drei";
import * as React from "react";

import {
	Button,
	Controller,
	LevelProvider,
	ToolbarGroup,
	ToolbarItem,
	ToolbarZone,
	Workbench,
	WorkbenchApp,
	WorkbenchWindowKind,
	WorkbenchView,
	createDefaultLayout,
	getLevelBgClass,
	type CommandBus,
	type UIWindowKindDefinition,
} from "@elements/ui";
import { Move3d, Rotate3d, Scaling } from "lucide-react";

import nakaginBoardJson from "../../board/play/fixtures/nakagin-capsule-tower.board.json";
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
} from "../../board/index.ts";
import nakaginSceneJson from "../../scene/play/fixtures/nakagin-capsule-tower.scene.json";
import {
	LOD_MODE_AUTOMATIC as SCENE_LOD_MODE_AUTOMATIC,
	isLodKind,
	parseFixtureV1,
	lodAutomaticSelectLabel as sceneLodAutomaticSelectLabel,
	lodCanvasProps as sceneLodCanvasProps,
	type FixtureV1 as SceneFixtureV1,
	type LodKind as SceneLodKind,
	type LodModeKind as SceneLodModeKind,
	type RelocateMode as SceneRelocateMode,
} from "../../scene/index.tsx";
import {
	buildTopologyDualSurfaceBindings,
	parseTopologyFixtureV1,
	TopologyBoardPane,
	TopologyScenePane,
	topologyMirrorConnectHandlers,
	topologyMirrorProximityHandlers,
	topologySceneChromeDefaults,
	topologySharedKindsFromPairedMetas,
} from "../react/index.tsx";
import topologyManifestJson from "./fixtures/nakagin-capsule-tower.topology.json";
import "./globals.css";

const TOPOLOGY_PLAY_APP_ID = "elements-topology-play";

const TOPOLOGY_PLAY_WINDOWS = {
	board: "topology-board",
	scene: "topology-scene",
} as const;

const TOPOLOGY_PLAY_WINDOW_LABELS = {
	board: "Sketch board",
	scene: "Spatial scene",
} as const;

const TOPOLOGY_PLAY_LOD_TIERS_BOARD: BoardDrawLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];
const TOPOLOGY_PLAY_LOD_TIERS_SCENE: SceneLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];
const TOPOLOGY_PLAY_CHROME_STRIP_CLASS = "flex shrink-0 gap-2 border-b border-border bg-muted/40 p-2";

interface TopologyPlayShellValue {
	readonly manifestLabel: string | undefined;
	readonly boardFixture: BoardFixtureV1;
	readonly sceneFixture: SceneFixtureV1;
	readonly bindings: ReturnType<typeof buildTopologyDualSurfaceBindings>;
	readonly boardSelected: ReadonlySet<string>;
	readonly boardCamera: CameraState;
	readonly sceneCamera: CameraState;
	readonly sceneSelected: string | null;
	readonly relocateMode: SceneRelocateMode;
	readonly sceneLodTag: SceneLodKind;
	readonly boardLodTag: BoardDrawLodKind;
	readonly onBoardLodChange: (lod: BoardDrawLodKind) => void;
	readonly boardLodProps: ReturnType<typeof boardLodCanvasProps>;
	readonly sceneLodProps: ReturnType<typeof sceneLodCanvasProps>;
	readonly connectBoard: number;
	readonly connectScene: number;
	readonly proximityBoard: number;
	readonly proximityScene: number;
	readonly setSceneRelocateMode: (mode: SceneRelocateMode) => void;
}

interface TopologyPlayControllerState {
	readonly relocateMode: SceneRelocateMode;
	readonly boardSelected: ReadonlySet<string>;
	readonly sceneSelected: string | null;
	readonly boardCamera: CameraState;
	readonly sceneCamera: CameraState;
	readonly sceneLodTag: SceneLodKind;
	readonly boardLodTag: BoardDrawLodKind;
	readonly boardLodMode: BoardLodModeKind;
	readonly sceneLodMode: SceneLodModeKind;
	readonly connectBoard: number;
	readonly connectScene: number;
	readonly proximityBoard: number;
	readonly proximityScene: number;
}

const TopologyPlayShellContext = React.createContext<TopologyPlayShellValue | null>(null);

function topologyPlayLodTierMenuLabel(tier: string): string {
	return tier.charAt(0).toUpperCase() + tier.slice(1);
}

class TopologyPlayChromeStrip extends React.Component<{ readonly leading: React.ReactNode; readonly trailing: React.ReactNode }> {
	render(): React.ReactElement {
		return (
			<div className={TOPOLOGY_PLAY_CHROME_STRIP_CLASS}>
				<ToolbarZone>{this.props.leading}</ToolbarZone>
				<div className="ml-auto flex flex-wrap items-center gap-3 text-xs text-muted-foreground">{this.props.trailing}</div>
			</div>
		);
	}
}

class TopologyBoardWindow extends React.Component<{ readonly shell: TopologyPlayShellValue }> {
	render(): React.ReactElement {
		const s = this.props.shell;
		return (
			<div className="flex h-full w-full flex-col">
				<TopologyPlayChromeStrip
					leading={
						<ToolbarGroup>
							<ToolbarItem>
								<span className="px-2 text-xs text-muted-foreground">{TOPOLOGY_PLAY_WINDOW_LABELS.board}</span>
							</ToolbarItem>
						</ToolbarGroup>
					}
					trailing={
						<>
							<span data-e2e-topology-manifest>{s.manifestLabel ?? "topology"}</span>
							<span data-e2e-topology-board-selected>{[...s.boardSelected].join(", ") || "—"}</span>
							<span data-e2e-topology-connect-board>{s.connectBoard}</span>
							<span data-e2e-topology-proximity-board>{s.proximityBoard}</span>
						</>
					}
				/>
				<div className="relative min-h-0 flex-1">
					<TopologyBoardPane fixture={s.boardFixture} bindings={s.bindings} selectedIds={s.boardSelected} board={{ camera: s.boardCamera, onLodChange: s.onBoardLodChange, ...s.boardLodProps }} />
				</div>
			</div>
		);
	}
}

class TopologyBoardWindowHost extends React.Component {
	render(): React.ReactElement | null {
		return <TopologyPlayShellContext.Consumer>{(shell) => (shell ? <TopologyBoardWindow shell={shell} /> : null)}</TopologyPlayShellContext.Consumer>;
	}
}

class TopologySceneWindow extends React.Component<{ readonly shell: TopologyPlayShellValue }> {
	render(): React.ReactElement {
		const s = this.props.shell;
		return (
			<div className="flex h-full w-full flex-col">
				<TopologyPlayChromeStrip
					leading={
						<ToolbarGroup>
							<ToolbarItem asChild>
								<Button variant={s.relocateMode === "translate" ? "default" : "outline"} size="sm" onClick={() => s.setSceneRelocateMode("translate")}>
									<Move3d className="mr-1 size-4" />
									Translate
								</Button>
							</ToolbarItem>
							<ToolbarItem asChild>
								<Button variant={s.relocateMode === "rotate" ? "default" : "outline"} size="sm" onClick={() => s.setSceneRelocateMode("rotate")}>
									<Rotate3d className="mr-1 size-4" />
									Rotate
								</Button>
							</ToolbarItem>
							<ToolbarItem asChild>
								<Button variant={s.relocateMode === "scale" ? "default" : "outline"} size="sm" onClick={() => s.setSceneRelocateMode("scale")}>
									<Scaling className="mr-1 size-4" />
									Scale
								</Button>
							</ToolbarItem>
						</ToolbarGroup>
					}
					trailing={
						<>
							<span data-e2e-topology-scene-lod>{s.sceneLodTag}</span>
							<span data-e2e-topology-scene-selected>{s.sceneSelected ?? "—"}</span>
							<span data-e2e-topology-connect-scene>{s.connectScene}</span>
							<span data-e2e-topology-proximity-scene>{s.proximityScene}</span>
						</>
					}
				/>
				<div className="relative min-h-0 flex-1">
					<TopologyScenePane fixture={s.sceneFixture} bindings={s.bindings} relocateMode={s.relocateMode} selectedObjectId={s.sceneSelected} scene={{ ...topologySceneChromeDefaults(), ...s.sceneLodProps }} />
				</div>
			</div>
		);
	}
}

class TopologySceneWindowHost extends React.Component {
	render(): React.ReactElement | null {
		return <TopologyPlayShellContext.Consumer>{(shell) => (shell ? <TopologySceneWindow shell={shell} /> : null)}</TopologyPlayShellContext.Consumer>;
	}
}

function topologyWindowKindsWithLodMeasures(
	boardLodMode: BoardLodModeKind,
	boardEffectiveLod: BoardDrawLodKind,
	sceneLodMode: SceneLodModeKind,
	sceneEffectiveLod: SceneLodKind,
	setBoardLodMode: (mode: BoardLodModeKind) => void,
	setSceneLodMode: (mode: SceneLodModeKind) => void,
): UIWindowKindDefinition[] {
	const boardLodMeasure: UIWindowKindDefinition["measures"] = [
		{
			id: `${TOPOLOGY_PLAY_WINDOWS.board}-lod`,
			items: [
				{ id: "automatic", label: boardLodAutomaticSelectLabel(boardEffectiveLod), value: BOARD_LOD_MODE_AUTOMATIC },
				...TOPOLOGY_PLAY_LOD_TIERS_BOARD.map((tier) => ({ id: tier, label: topologyPlayLodTierMenuLabel(tier), value: tier })),
			],
			kind: "select",
			label: "LOD",
			onValueChange: (value) => {
				if (value === BOARD_LOD_MODE_AUTOMATIC || isBoardDrawLodKind(value)) {
					setBoardLodMode(value as BoardLodModeKind);
				}
			},
			value: boardLodMode,
		},
	];
	const sceneLodMeasure: UIWindowKindDefinition["measures"] = [
		{
			id: `${TOPOLOGY_PLAY_WINDOWS.scene}-lod`,
			items: [
				{ id: "automatic", label: sceneLodAutomaticSelectLabel(sceneEffectiveLod), value: SCENE_LOD_MODE_AUTOMATIC },
				...TOPOLOGY_PLAY_LOD_TIERS_SCENE.map((tier) => ({ id: tier, label: topologyPlayLodTierMenuLabel(tier), value: tier })),
			],
			kind: "select",
			label: "LOD",
			onValueChange: (value) => {
				if (value === SCENE_LOD_MODE_AUTOMATIC || isLodKind(value)) {
					setSceneLodMode(value as SceneLodModeKind);
				}
			},
			value: sceneLodMode,
		},
	];
	return [
		{ id: TOPOLOGY_PLAY_WINDOWS.board, label: TOPOLOGY_PLAY_WINDOW_LABELS.board, component: TopologyBoardWindowHost, measures: boardLodMeasure },
		{ id: TOPOLOGY_PLAY_WINDOWS.scene, label: TOPOLOGY_PLAY_WINDOW_LABELS.scene, component: TopologySceneWindowHost, measures: sceneLodMeasure },
	];
}

const TOPOLOGY_PLAY_SHELL_CONTROLLER_ID = "topology-play";

class TopologyPlayShellController extends Controller {
	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(TOPOLOGY_PLAY_SHELL_CONTROLLER_ID, commandBus, hostNotify);
	}

	override run(_command: string, _args?: unknown): void {}
}

function buildTopologyWorkbenchApp(controller: TopologyPlayShellController): WorkbenchApp {
	return new WorkbenchApp(
		TOPOLOGY_PLAY_APP_ID,
		"Topology play",
		undefined,
		controller,
		createDefaultLayout([TOPOLOGY_PLAY_WINDOWS.board, TOPOLOGY_PLAY_WINDOWS.scene], "row", [50, 50], [TOPOLOGY_PLAY_WINDOW_LABELS.board, TOPOLOGY_PLAY_WINDOW_LABELS.scene]) as never,
		[
			new WorkbenchWindowKind(TOPOLOGY_PLAY_WINDOWS.board, TOPOLOGY_PLAY_WINDOW_LABELS.board, "elements.topology.placeholder"),
			new WorkbenchWindowKind(TOPOLOGY_PLAY_WINDOWS.scene, TOPOLOGY_PLAY_WINDOW_LABELS.scene, "elements.topology.placeholder"),
		],
	);
}

function buildInvalidTopologyWorkbenchApp(controller: TopologyPlayShellController): WorkbenchApp {
	return new WorkbenchApp(
		TOPOLOGY_PLAY_APP_ID,
		"Topology play",
		undefined,
		controller,
		createDefaultLayout(["topology-error"], "row", [100], ["Error"]) as never,
		[new WorkbenchWindowKind("topology-error", "Error", "elements.topology.placeholder")],
	);
}

let invalidTopologyWorkbench: Workbench | null = null;

function getInvalidTopologyWorkbench(): Workbench {
	if (!invalidTopologyWorkbench) {
		const wb = new Workbench();
		const ctrl = new TopologyPlayShellController(wb.commandBus, () => wb.notify());
		wb.addApp(buildInvalidTopologyWorkbenchApp(ctrl));
		invalidTopologyWorkbench = wb;
	}
	return invalidTopologyWorkbench;
}

class InvalidTopologyWindow extends React.Component {
	render(): React.ReactElement {
		return <div className="p-4 text-destructive">Invalid board or scene fixture</div>;
	}
}

class TopologyPlayController extends React.Component<{ readonly boardFixture: BoardFixtureV1; readonly sceneFixture: SceneFixtureV1 }, TopologyPlayControllerState> {
	private readonly manifest = parseTopologyFixtureV1(topologyManifestJson as unknown);
	private readonly mirrorConnect = topologyMirrorConnectHandlers((payload) => {
		if (payload.surface === "board") {
			this.setState((current) => ({ connectBoard: current.connectBoard + 1 }));
			return;
		}
		this.setState((current) => ({ connectScene: current.connectScene + 1 }));
	});
	private readonly mirrorProximity = topologyMirrorProximityHandlers((payload) => {
		if (payload.surface === "board") {
			this.setState((current) => ({ proximityBoard: current.proximityBoard + 1 }));
			return;
		}
		this.setState((current) => ({ proximityScene: current.proximityScene + 1 }));
	});

	private topologyWorkbench: Workbench | null = null;

	state: TopologyPlayControllerState = {
		relocateMode: "translate",
		boardSelected: new Set(),
		sceneSelected: null,
		boardCamera: { ...this.props.boardFixture.camera },
		sceneCamera: { ...this.props.sceneFixture.camera },
		sceneLodTag: "normal",
		boardLodTag: "normal",
		boardLodMode: BOARD_LOD_MODE_AUTOMATIC,
		sceneLodMode: SCENE_LOD_MODE_AUTOMATIC,
		connectBoard: 0,
		connectScene: 0,
		proximityBoard: 0,
		proximityScene: 0,
	};

	componentDidMount(): void {
		const urls = [...new Set(this.props.sceneFixture.objects.map((object) => object.meshUrl))];
		for (const url of urls) {
			useGLTF.preload(url);
		}
		if (!this.manifest) {
			console.warn("[DEBUG] topology manifest parse failed");
		} else if (this.manifest.label) {
			console.log("[DEBUG] topology manifest", this.manifest.label);
		}
	}

	private readonly onBoardLodChange = (lod: BoardDrawLodKind): void => {
		this.setState({ boardLodTag: lod });
	};

	private readonly onBoardSelect = (snap: { ids: readonly string[] }): void => {
		this.setState({ boardSelected: new Set(snap.ids) });
	};

	private readonly onSceneSelect = (snap: { objectIds: readonly string[] }): void => {
		this.setState({ sceneSelected: snap.objectIds[0] ?? null });
	};

	private buildShellValue(): TopologyPlayShellValue {
		const sharedKinds = topologySharedKindsFromPairedMetas({ boardMeta: this.props.boardFixture.meta, sceneMeta: this.props.sceneFixture.meta });
		const bindings = buildTopologyDualSurfaceBindings({
			...sharedKinds,
			onBoardSelect: this.onBoardSelect,
			onSceneSelect: this.onSceneSelect,
			onBoardCamera: (boardCamera) => this.setState({ boardCamera }),
			onSceneCamera: (sceneCamera) => this.setState({ sceneCamera }),
			onSceneLodChange: (sceneLodTag) => this.setState({ sceneLodTag }),
			...this.mirrorConnect,
			...this.mirrorProximity,
		});
		return {
			manifestLabel: this.manifest?.label,
			boardFixture: this.props.boardFixture,
			sceneFixture: this.props.sceneFixture,
			bindings,
			boardSelected: this.state.boardSelected,
			boardCamera: this.state.boardCamera,
			sceneCamera: this.state.sceneCamera,
			sceneSelected: this.state.sceneSelected,
			relocateMode: this.state.relocateMode,
			sceneLodTag: this.state.sceneLodTag,
			boardLodTag: this.state.boardLodTag,
			onBoardLodChange: this.onBoardLodChange,
			boardLodProps: boardLodCanvasProps(this.state.boardLodMode),
			sceneLodProps: sceneLodCanvasProps(this.state.sceneLodMode),
			connectBoard: this.state.connectBoard,
			connectScene: this.state.connectScene,
			proximityBoard: this.state.proximityBoard,
			proximityScene: this.state.proximityScene,
			setSceneRelocateMode: (relocateMode) => this.setState({ relocateMode }),
		};
	}

	render(): React.ReactElement {
		const shellValue = this.buildShellValue();
		if (!this.topologyWorkbench) {
			const wb = new Workbench();
			const ctrl = new TopologyPlayShellController(wb.commandBus, () => wb.notify());
			wb.addApp(buildTopologyWorkbenchApp(ctrl));
			this.topologyWorkbench = wb;
		}
		const windowKinds = topologyWindowKindsWithLodMeasures(
			this.state.boardLodMode,
			this.state.boardLodTag,
			this.state.sceneLodMode,
			this.state.sceneLodTag,
			(boardLodMode) => this.setState({ boardLodMode }),
			(sceneLodMode) => this.setState({ sceneLodMode }),
		);
		return (
			<TopologyPlayShellContext.Provider value={shellValue}>
				<WorkbenchView
					workbench={this.topologyWorkbench}
					defaultAppId={TOPOLOGY_PLAY_APP_ID}
					className={getLevelBgClass(0)}
					resolvedWindowKindsOverride={windowKinds}
				/>
			</TopologyPlayShellContext.Provider>
		);
	}
}

function invalidTopologyWindowKinds(): UIWindowKindDefinition[] {
	return [{ id: "topology-error", label: "Error", component: InvalidTopologyWindow }];
}

class TopologyPlayApp extends React.Component {
	private readonly boardFixture = parseBoardFixtureV1(nakaginBoardJson as unknown);
	private readonly sceneFixture = parseFixtureV1(nakaginSceneJson as unknown);

	render(): React.ReactElement {
		if (!this.boardFixture || !this.sceneFixture) {
			return (
				<LevelProvider>
					<WorkbenchView
						workbench={getInvalidTopologyWorkbench()}
						defaultAppId={TOPOLOGY_PLAY_APP_ID}
						className={getLevelBgClass(0)}
						resolvedWindowKindsOverride={invalidTopologyWindowKinds()}
					/>
				</LevelProvider>
			);
		}

		return (
			<LevelProvider>
				<TopologyPlayController boardFixture={this.boardFixture} sceneFixture={this.sceneFixture} />
			</LevelProvider>
		);
	}
}

export function createTopologyPlayElement(): React.ReactElement {
	return <TopologyPlayApp />;
}

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
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
			const sk = topologySharedKindsFromPairedMetas({ boardMeta: undefined, sceneMeta: { kindCompatibility: [{ source: "u", target: "v" }] } });
			expect(sk.kindCompatibility?.length).toBeGreaterThan(0);
		});
	});
}