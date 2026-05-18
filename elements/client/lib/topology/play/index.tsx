// #region 🧲Header
// 💻 elements/client/lib/topology/play/index.tsx — Topology play harness: Nakagin board + scene in two `windowKinds`, shared `buildTopologyDualSurfaceBindings`.
// #endregion 🧲Header

// #region 📥Imports
import { useGLTF } from "@react-three/drei";
import { createContext, useCallback, useContext, useEffect, useMemo, useState, type ReactElement } from "react";
import { createRoot } from "react-dom/client";

import {
	Button,
	LevelProvider,
	ToolbarGroup,
	ToolbarItem,
	ToolbarZone,
	UI,
	createDefaultLayout,
	createStackLayout,
	getLevelBgClass,
	type UIAppConfig,
} from "@elements/ui";
import { Move3d, Rotate3d, Scaling } from "lucide-react";

import nakaginBoardJson from "../../../../../.storybook/fixtures/nakagin-capsule-tower.board.json";
import {
	boardFixtureMetaKindCatalogBundle,
	parseBoardFixtureV1,
	type BoardFixtureV1,
	type BoardKindCatalogBundle,
	type BoardKindCompatEntry,
	type CameraState,
} from "../../board/index.ts";
import nakaginSceneJson from "../../scene/fixtures/nakagin-capsule-tower.scene.json";
import {
	buildTopologyDualSurfaceBindings,
	parseTopologyFixtureV1,
	TopologyBoardPane,
	TopologyScenePane,
	topologyMirrorConnectHandlers,
	DEFAULT_BOARD_GRID_FACTOR,
	DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
} from "../react/index.tsx";
import {
	parseSceneFixtureV1,
	type SceneCameraState,
	type SceneFixtureV1,
	type SceneKindCatalogBundle,
	type SceneLodKind,
	type SceneRelocateMode,
} from "../../scene/react/index.tsx";
import topologyManifestJson from "../fixtures/nakagin-capsule-tower.topology.json";
import "./globals.css";
// #endregion 📥Imports

// #region 🧾Meta
function parseKindCompatibility(meta: Record<string, unknown> | undefined): readonly BoardKindCompatEntry[] {
	if (!meta || typeof meta !== "object") return [];
	const arr = (meta as { kindCompatibility?: unknown }).kindCompatibility;
	if (!Array.isArray(arr)) return [];
	const out: BoardKindCompatEntry[] = [];
	for (const entry of arr) {
		if (!entry || typeof entry !== "object") continue;
		const e = entry as Record<string, unknown>;
		const source = typeof e.source === "string" ? e.source.trim() : "";
		const target = typeof e.target === "string" ? e.target.trim() : "";
		if (!source || !target) continue;
		const specificity =
			e.specificity === "general" ||
			e.specificity === "node" ||
			e.specificity === "edge" ||
			e.specificity === "handle" ||
			e.specificity === "wire" ||
			e.specificity === "object" ||
			e.specificity === "tie"
				? e.specificity
				: undefined;
		out.push({
			source,
			target,
			...(e.bidirectional === true ? { bidirectional: true } : {}),
			...(e.important === true ? { important: true } : {}),
			...(specificity ? { specificity } : {}),
		});
	}
	return out;
}

function parseKindCatalogs(meta: Record<string, unknown> | undefined): SceneKindCatalogBundle | undefined {
	const kc = meta?.kindCatalogs;
	if (!kc || typeof kc !== "object") return undefined;
	return kc as SceneKindCatalogBundle;
}
// #endregion 🧾Meta

// #region 🎛️Shell
const TOPOLOGY_PLAY_APP_ID = "elements-topology-play";

interface TopologyPlayShellValue {
	readonly manifestLabel: string | undefined;
	readonly boardFixture: BoardFixtureV1;
	readonly sceneFixture: SceneFixtureV1;
	readonly bindings: ReturnType<typeof buildTopologyDualSurfaceBindings>;
	readonly boardSelected: ReadonlySet<string>;
	readonly boardCamera: CameraState;
	readonly sceneCamera: SceneCameraState;
	readonly sceneSelected: string | null;
	readonly relocateMode: SceneRelocateMode;
	readonly sceneLodTag: SceneLodKind;
	readonly connectBoard: number;
	readonly connectScene: number;
	readonly proximityBoard: number;
	readonly proximityScene: number;
	readonly setRelocateMode: (mode: SceneRelocateMode) => void;
}

const TopologyPlayShellContext = createContext<TopologyPlayShellValue | null>(null);

function useTopologyPlayShell(): TopologyPlayShellValue {
	const v = useContext(TopologyPlayShellContext);
	if (!v) {
		throw new Error("TopologyPlayShellContext missing");
	}
	return v;
}

function TopologyBoardWindow(): ReactElement {
	const s = useTopologyPlayShell();
	return (
		<div className="flex h-full w-full flex-col">
			<div className="flex shrink-0 gap-2 border-b border-border bg-muted/40 p-2">
				<ToolbarZone>
					<ToolbarGroup>
						<ToolbarItem>
							<span className="px-2 text-xs text-muted-foreground">Sketch board</span>
						</ToolbarItem>
					</ToolbarGroup>
				</ToolbarZone>
				<div className="ml-auto flex items-center gap-3 text-xs text-muted-foreground">
					<span data-e2e-topology-manifest>{s.manifestLabel ?? "topology"}</span>
					<span data-e2e-topology-board-selected>{[...s.boardSelected].join(", ") || "—"}</span>
					<span data-e2e-topology-connect-board>{s.connectBoard}</span>
					<span data-e2e-topology-proximity-board>{s.proximityBoard}</span>
				</div>
			</div>
			<div className="relative min-h-0 flex-1">
				<TopologyBoardPane
					fixture={s.boardFixture}
					bindings={s.bindings}
					selectedIds={s.boardSelected}
					board={{ camera: s.boardCamera }}
				/>
			</div>
		</div>
	);
}

function TopologySceneWindow(): ReactElement {
	const s = useTopologyPlayShell();
	return (
		<div className="flex h-full w-full flex-col">
			<div className="flex shrink-0 flex-wrap gap-2 border-b border-border bg-muted/40 p-2">
				<ToolbarZone>
					<ToolbarGroup>
						<ToolbarItem asChild>
							<Button
								variant={s.relocateMode === "translate" ? "default" : "outline"}
								size="sm"
								onClick={() => s.setRelocateMode("translate")}
							>
								<Move3d className="mr-1 size-4" />
								Translate
							</Button>
						</ToolbarItem>
						<ToolbarItem asChild>
							<Button
								variant={s.relocateMode === "rotate" ? "default" : "outline"}
								size="sm"
								onClick={() => s.setRelocateMode("rotate")}
							>
								<Rotate3d className="mr-1 size-4" />
								Rotate
							</Button>
						</ToolbarItem>
						<ToolbarItem asChild>
							<Button
								variant={s.relocateMode === "scale" ? "default" : "outline"}
								size="sm"
								onClick={() => s.setRelocateMode("scale")}
							>
								<Scaling className="mr-1 size-4" />
								Scale
							</Button>
						</ToolbarItem>
					</ToolbarGroup>
				</ToolbarZone>
				<div className="ml-auto flex items-center gap-3 text-xs text-muted-foreground">
					<span data-e2e-topology-scene-lod>{s.sceneLodTag}</span>
					<span data-e2e-topology-scene-selected>{s.sceneSelected ?? "—"}</span>
					<span data-e2e-topology-connect-scene>{s.connectScene}</span>
					<span data-e2e-topology-proximity-scene>{s.proximityScene}</span>
				</div>
			</div>
			<div className="relative min-h-0 flex-1">
				<TopologyScenePane
					fixture={s.sceneFixture}
					bindings={s.bindings}
					relocateMode={s.relocateMode}
					selectedObjectId={s.sceneSelected}
					scene={{ camera: s.sceneCamera, showLodGrid: true, proximityRadius: 24 }}
				/>
			</div>
		</div>
	);
}
// #endregion 🎛️Shell

// #region 🎬Controller
function TopologyPlayController({
	boardFixture,
	sceneFixture,
}: {
	readonly boardFixture: BoardFixtureV1;
	readonly sceneFixture: SceneFixtureV1;
}): ReactElement {
	const manifest = useMemo(() => parseTopologyFixtureV1(topologyManifestJson as unknown), []);
	const [relocateMode, setRelocateMode] = useState<SceneRelocateMode>("translate");
	const [boardSelected, setBoardSelected] = useState<ReadonlySet<string>>(() => new Set());
	const [sceneSelected, setSceneSelected] = useState<string | null>(null);
	const [boardCamera, setBoardCamera] = useState<CameraState>(() => ({ ...boardFixture.camera }));
	const [sceneCamera, setSceneCamera] = useState<SceneCameraState>(() => ({
		...sceneFixture.camera,
	}));
	const [sceneLodTag, setSceneLodTag] = useState<SceneLodKind>("normal");
	const [connectBoard, setConnectBoard] = useState(0);
	const [connectScene, setConnectScene] = useState(0);
	const [proximityBoard, setProximityBoard] = useState(0);
	const [proximityScene, setProximityScene] = useState(0);

	const kindCompatibility = useMemo(() => parseKindCompatibility(sceneFixture.meta), [sceneFixture.meta]);
	const kindCatalogs = useMemo(() => {
		const fromBoard = boardFixtureMetaKindCatalogBundle(boardFixture.meta);
		if (fromBoard) return fromBoard;
		return parseKindCatalogs(sceneFixture.meta) as BoardKindCatalogBundle | undefined;
	}, [boardFixture.meta, sceneFixture.meta]);

	const onBoardSelect = useCallback((snap: { ids: readonly string[] }) => {
		setBoardSelected(new Set(snap.ids));
	}, []);

	const onSceneSelect = useCallback((snap: { objectIds: readonly string[] }) => {
		setSceneSelected(snap.objectIds[0] ?? null);
	}, []);

	const mirrorConnect = useMemo(
		() =>
			topologyMirrorConnectHandlers((p) => {
				if (p.surface === "board") setConnectBoard((c) => c + 1);
				else setConnectScene((c) => c + 1);
			}),
		[],
	);

	const onBoardProximity = useCallback(() => {
		setProximityBoard((c) => c + 1);
	}, []);
	const onSceneProximity = useCallback(() => {
		setProximityScene((c) => c + 1);
	}, []);

	const bindings = useMemo(
		() =>
			buildTopologyDualSurfaceBindings({
				lodZoomThresholds: DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
				gridFactor: DEFAULT_BOARD_GRID_FACTOR,
				gridSnapEnabled: true,
				kindCatalogs,
				kindCompatibility: [...kindCompatibility],
				onBoardSelect,
				onSceneSelect,
				onBoardCamera: setBoardCamera,
				onSceneCamera: setSceneCamera,
				onSceneLodChange: setSceneLodTag,
				onBoardProximityConnect: onBoardProximity,
				onSceneProximityConnect: onSceneProximity,
				...mirrorConnect,
			}),
		[
			kindCatalogs,
			kindCompatibility,
			mirrorConnect,
			onBoardProximity,
			onBoardSelect,
			onSceneProximity,
			onSceneSelect,
		],
	);

	useEffect(() => {
		const urls = [...new Set(sceneFixture.objects.map((o) => o.meshUrl))];
		for (const u of urls) {
			useGLTF.preload(u);
		}
	}, [sceneFixture.objects]);

	useEffect(() => {
		if (!manifest) {
			console.warn("[DEBUG] topology manifest parse failed");
			return;
		}
		if (manifest.label) {
			console.log("[DEBUG] topology manifest", manifest.label);
		}
	}, [manifest]);

	const shellValue = useMemo<TopologyPlayShellValue>(
		() => ({
			manifestLabel: manifest?.label,
			boardFixture,
			sceneFixture,
			bindings,
			boardSelected,
			boardCamera,
			sceneCamera,
			sceneSelected,
			relocateMode,
			sceneLodTag,
			connectBoard,
			connectScene,
			proximityBoard,
			proximityScene,
			setRelocateMode,
		}),
		[
			manifest?.label,
			boardFixture,
			sceneFixture,
			bindings,
			boardSelected,
			boardCamera,
			sceneCamera,
			sceneSelected,
			relocateMode,
			sceneLodTag,
			connectBoard,
			connectScene,
			proximityBoard,
			proximityScene,
		],
	);

	const apps = useMemo<UIAppConfig[]>(
		() => [
			{
				id: TOPOLOGY_PLAY_APP_ID,
				label: "Topology play",
				windowKinds: [
					{ id: "topology-board", label: "Sketch board", component: TopologyBoardWindow },
					{ id: "topology-scene", label: "Spatial scene", component: TopologySceneWindow },
				],
				defaultLayout: createDefaultLayout(
					["topology-board", "topology-scene"],
					"row",
					[50, 50],
					["Sketch board", "Spatial scene"],
				),
			},
		],
		[],
	);

	return (
		<TopologyPlayShellContext.Provider value={shellValue}>
			<UI apps={apps} defaultAppId={TOPOLOGY_PLAY_APP_ID} className={getLevelBgClass(0)} />
		</TopologyPlayShellContext.Provider>
	);
}
// #endregion 🎬Controller

// #region 🚀Mount
function invalidFixtureApps(): UIAppConfig[] {
	return [
		{
			id: TOPOLOGY_PLAY_APP_ID,
			label: "Topology play",
			windowKinds: [
				{
					id: "topology-error",
					label: "Error",
					component: () => <div className="p-4 text-destructive">Invalid board or scene fixture</div>,
				},
			],
			defaultLayout: createStackLayout(["topology-error"], ["Error"]),
		},
	];
}

function TopologyPlayApp(): ReactElement {
	const boardFixture = useMemo(() => parseBoardFixtureV1(nakaginBoardJson as unknown), []);
	const sceneFixture = useMemo(() => parseSceneFixtureV1(nakaginSceneJson as unknown), []);

	if (!boardFixture || !sceneFixture) {
		return (
			<LevelProvider>
				<UI apps={invalidFixtureApps()} defaultAppId={TOPOLOGY_PLAY_APP_ID} className={getLevelBgClass(0)} />
			</LevelProvider>
		);
	}

	return (
		<LevelProvider>
			<TopologyPlayController boardFixture={boardFixture} sceneFixture={sceneFixture} />
		</LevelProvider>
	);
}

const rootEl = document.getElementById("root");
if (rootEl) {
	createRoot(rootEl).render(<TopologyPlayApp />);
}
// #endregion 🚀Mount

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("topology play fixtures", () => {
		it("parses nakagin board and scene", () => {
			const b = parseBoardFixtureV1(nakaginBoardJson as unknown);
			const s = parseSceneFixtureV1(nakaginSceneJson as unknown);
			expect(b?.nodes.length).toBeGreaterThan(0);
			expect(s?.objects.length).toBeGreaterThan(0);
		});
		it("parses topology manifest", () => {
			const t = parseTopologyFixtureV1(topologyManifestJson as unknown);
			expect(t?.schema).toBe("elements.topology.fixture/v1");
		});
	});
}
