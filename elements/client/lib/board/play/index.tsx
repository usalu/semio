// #region 🧲Header
// 💻 elements/client/lib/board/play/index.tsx — Board play: triptych Nakagin views, in-app fixture drag shelf, selection inspector, `UI` shell (same `@elements/ui` + globals pattern as semio rendering / algorithms).
// #endregion 🧲Header

// #region 📥Imports
import {
	Button,
	Expertise,
	Input,
	Label,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	Slider,
	Tree,
	TreeStateProvider,
	UI,
	LevelProvider,
	createWindowLayout,
	getLevelBgClass,
	useElementsSurfaceChrome,
	ToolbarDivider,
	ToolbarGroup,
	ToolbarItem,
	ToolbarZone,
	type ElementsSurfaceDevice,
	type ElementsSurfaceTheme,
	type FooterItem,
	type TreeDataSection,
	type ContextMenuItem,
	type UIAppConfig,
	type UIWindowKindDefinition,
	type UIWindowLayout,
	type UIWindowOption,
} from "@elements/ui";
import { BoxSelect, Circle, ClipboardList, Lasso, Library, Minus, Pause, Play, Plus, Repeat2, Settings, Square } from "lucide-react";
import {
	createContext,
	useCallback,
	useContext,
	useEffect,
	useMemo,
	useRef,
	useState,
	type ChangeEvent,
	type PointerEvent,
	type DragEvent,
	type ReactElement,
	type ReactNode,
} from "react";
import { createPortal } from "react-dom";
import { createRoot, type Root } from "react-dom/client";

import nakaginFixtureJson from "../../../../../.storybook/fixtures/nakagin-capsule-tower.board.json";
import {
	BOARD_BUILTIN_PORT_HANDLE_KIND,
	BOARD_CAMERA_ZOOM_MAX,
	BOARD_CAMERA_ZOOM_MIN,
	BOARD_DEFAULT_HANDLE_KIND_CATALOG,
	BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE,
	BOARD_FIXTURE_DRAG_V1_MIME,
	BOARD_SELECTION_TARGETS_DEFAULT,
	encodeBoardFixtureForDragV1,
	layoutBoardFixtureRedraw,
	parseBoardFixtureV1,
	type BoardFixtureDropDetail,
	type BoardFixtureCircleNodeV1,
	type BoardFixtureEdgeV1,
	type BoardFixtureHandleV1,
	type BoardFixtureNodeV1,
	type BoardFixtureRectangleNodeV1,
	type BoardFixtureV1,
	type BoardHierarchicalTreeDirectionKind,
	type BoardRedrawLayoutOptions,
	type BoardRedrawModeKind,
	type BoardForceGraphLayoutOptions,
	type BoardSelectionMethod,
	type BoardSelectionMode,
	type BoardSelectionTargets,
	type CameraState,
} from "../index";
import { BoardCanvas, Edge, Handle, Node, useBoardEvent } from "../index.tsx";
import "./globals.css";
// #endregion 📥Imports

// #region 🔖Kinds
export type BoardPlayPaneId = "board-overview" | "board-detail" | "board-selection";

const BOARD_PLAY_APP_ID = "elements-board-play";

const boardPlayOverviewWindowContextMenu: ContextMenuItem[] = [{ id: "win-demo", label: "Overview window menu demo" }];
const boardPlayDemoNodeContextMenu: ContextMenuItem[] = [
	{ id: "demo-node", label: "Demo capsule action" },
	{ children: [{ id: "demo-sub-1", label: "Nested item" }], id: "demo-sub", label: "Demo nested" },
];
const boardPlayDemoEdgeContextMenu: ContextMenuItem[] = [{ id: "demo-edge", label: "Demo edge action" }];
const boardPlayCanvasBackgroundMenu: ContextMenuItem[] = [{ id: "demo-bg", label: "Board background menu" }];

const LS_THEME = "elements.board-play.surface.theme";
const LS_DEVICE = "elements.board-play.surface.device";
const LS_EXPERTISE = "elements.board-play.surface.expertise";

function parseStoredTheme(raw: string | null): ElementsSurfaceTheme {
	if (raw === "light" || raw === "dark" || raw === "system") return raw;
	return "system";
}

function parseStoredDevice(raw: string | null): ElementsSurfaceDevice {
	if (raw === "desktop" || raw === "tablet" || raw === "mobile") return raw;
	return "desktop";
}

function parseStoredExpertise(raw: string | null): Expertise {
	if (raw === Expertise.BEGINNER || raw === Expertise.NORMAL || raw === Expertise.EXPERT) return raw;
	return Expertise.NORMAL;
}

function readTheme(): ElementsSurfaceTheme {
	if (typeof localStorage === "undefined") return "system";
	try {
		return parseStoredTheme(localStorage.getItem(LS_THEME));
	} catch {
		return "system";
	}
}

function readDevice(): ElementsSurfaceDevice {
	if (typeof localStorage === "undefined") return "desktop";
	try {
		return parseStoredDevice(localStorage.getItem(LS_DEVICE));
	} catch {
		return "desktop";
	}
}

function readExpertise(): Expertise {
	if (typeof localStorage === "undefined") return Expertise.NORMAL;
	try {
		return parseStoredExpertise(localStorage.getItem(LS_EXPERTISE));
	} catch {
		return Expertise.NORMAL;
	}
}
// #endregion 🔖Kinds

// #region 🔖Geometry
const REF_VIEWPORT_SHORT_PX = 640;

function clampZoom(value: number): number {
	return Math.min(BOARD_CAMERA_ZOOM_MAX, Math.max(BOARD_CAMERA_ZOOM_MIN, value));
}

/** @emoji 📐 Axis-aligned bounds of all fixture nodes (world units). */
function fixtureWorldBounds(fixture: BoardFixtureV1): { cx: number; cy: number; halfSpan: number } {
	let minX = Number.POSITIVE_INFINITY;
	let minY = Number.POSITIVE_INFINITY;
	let maxX = Number.NEGATIVE_INFINITY;
	let maxY = Number.NEGATIVE_INFINITY;
	for (const node of fixture.nodes) {
		if (node.shape === "rectangle") {
			const hw = node.width / 2;
			const hh = node.height / 2;
			minX = Math.min(minX, node.x - hw);
			maxX = Math.max(maxX, node.x + hw);
			minY = Math.min(minY, node.y - hh);
			maxY = Math.max(maxY, node.y + hh);
		} else {
			minX = Math.min(minX, node.x - node.radius);
			maxX = Math.max(maxX, node.x + node.radius);
			minY = Math.min(minY, node.y - node.radius);
			maxY = Math.max(maxY, node.y + node.radius);
		}
	}
	if (!Number.isFinite(minX)) {
		return { cx: 0, cy: 0, halfSpan: 400 };
	}
	const cx = (minX + maxX) / 2;
	const cy = (minY + maxY) / 2;
	const halfSpan = Math.max(maxX - minX, maxY - minY, 1) / 2;
	return { cx, cy, halfSpan };
}

/** @emoji 📷 Three cameras derived from the same Nakagin fixture (wide overview, tight detail, regional wide). */
function triptychCamerasFromFixture(fixture: BoardFixtureV1): Record<BoardPlayPaneId, CameraState> {
	const { cx, cy, halfSpan } = fixtureWorldBounds(fixture);
	const base = fixture.camera;
	const fitZoom = clampZoom((REF_VIEWPORT_SHORT_PX * 0.44) / halfSpan);
	const detailNode = fixture.nodes[Math.min(42, Math.max(0, fixture.nodes.length - 1))];
	const detailZoom = clampZoom(fitZoom * 2.15);
	return {
		"board-overview": {
			x: cx + base.x * 0.04,
			y: cy + base.y * 0.03,
			zoom: clampZoom(fitZoom * 0.68),
		},
		"board-detail": {
			x: detailNode ? detailNode.x + base.x * 0.02 : cx,
			y: detailNode ? detailNode.y + base.y * 0.02 : cy,
			zoom: detailZoom,
		},
		"board-selection": {
			x: cx - halfSpan * 0.28 + base.x * 0.06,
			y: cy + halfSpan * 0.22 + base.y * 0.05,
			zoom: clampZoom(fitZoom * 0.36),
		},
	};
}

/** @emoji ✅ Distinct default selections per pane (indices stable on full Nakagin graph). */
function selectionSeedForFixture(fixture: BoardFixtureV1): Record<BoardPlayPaneId, Set<string>> {
	const nodeA = fixture.nodes[0];
	const nodeB = fixture.nodes[Math.min(42, Math.max(0, fixture.nodes.length - 1))];
	const handleB = nodeB?.handles[0];
	const edge = fixture.edges[Math.min(9, Math.max(0, fixture.edges.length - 1))];
	return {
		"board-overview": new Set(nodeA?.id ? [nodeA.id] : []),
		"board-detail": new Set([nodeB?.id, handleB?.id].filter(Boolean) as string[]),
		"board-selection": new Set(edge?.id ? [edge.id] : []),
	};
}
// #endregion 🔖Geometry

// #region 🔖ShellContext
interface BoardPlayShellValue {
	fixture: BoardFixtureV1;
	setFixture: (next: BoardFixtureV1) => void;
	/** @emoji 🎯 Palette drags merge one node at the pointer; full fixtures replace the graph. */
	handleCanvasFixtureDrop: (pane: BoardPlayPaneId, detail: BoardFixtureDropDetail) => void;
	patchFixture: (updater: (prev: BoardFixtureV1) => BoardFixtureV1) => void;
	activePaneId: BoardPlayPaneId;
	setActivePaneId: (id: BoardPlayPaneId) => void;
	selectionByPane: Record<BoardPlayPaneId, Set<string>>;
	setSelectionForPane: (pane: BoardPlayPaneId, ids: readonly string[]) => void;
	/** @emoji 🔁 Rewrites selection ids when an object id changes (`replacedId` → `replacementId`); unrelated to edge endpoint fields. */
	remapIdInSelections: (replacedId: string, replacementId: string) => void;
	camerasByPane: Record<BoardPlayPaneId, CameraState>;
	boardSelectionMethod: BoardSelectionMethod;
	setBoardSelectionMethod: (value: BoardSelectionMethod) => void;
	boardSelectionMode: BoardSelectionMode;
	setBoardSelectionMode: (value: BoardSelectionMode) => void;
	boardSelectionTargets: BoardSelectionTargets;
	setBoardSelectionTargets: (value: BoardSelectionTargets | ((prev: BoardSelectionTargets) => BoardSelectionTargets)) => void;
	/** @emoji 🗑️ Drops ids from the shared fixture after the canvas emits structural delete events. */
	applyStructuralDelete: (kind: "edge" | "node", id: string) => void;
	/** @emoji ⏯️ When true, the toolbar play control runs periodic redraw ticks for the active {@link BoardRedrawModeKind}. */
	boardRedrawPlaying: boolean;
	setBoardRedrawPlaying: (value: boolean) => void;
	boardRedrawMode: BoardRedrawModeKind;
	setBoardRedrawMode: (value: BoardRedrawModeKind) => void;
	forceLayoutFullIterations: number;
	setForceLayoutFullIterations: (value: number) => void;
	forceLayoutIdealEdgeLength: number;
	setForceLayoutIdealEdgeLength: (value: number) => void;
	forceLayoutGravity: number;
	setForceLayoutGravity: (value: number) => void;
	forceLayoutRepulsionStrength: number;
	setForceLayoutRepulsionStrength: (value: number) => void;
	boardRedrawTickMs: number;
	setBoardRedrawTickMs: (value: number) => void;
	boardRedrawTickIterations: number;
	setBoardRedrawTickIterations: (value: number) => void;
	treeLayoutLayerSpacing: number;
	setTreeLayoutLayerSpacing: (value: number) => void;
	treeLayoutSiblingGap: number;
	setTreeLayoutSiblingGap: (value: number) => void;
	treeLayoutDirection: BoardHierarchicalTreeDirectionKind;
	setTreeLayoutDirection: (value: BoardHierarchicalTreeDirectionKind) => void;
	applyBoardRedrawOnce: () => void;
}

const BoardPlayShellContext = createContext<BoardPlayShellValue | null>(null);

function useBoardPlayShell(): BoardPlayShellValue {
	const value = useContext(BoardPlayShellContext);
	if (!value) {
		throw new Error("useBoardPlayShell must be used inside BoardPlayShellContext.");
	}
	return value;
}
// #endregion 🔖ShellContext

// #region 🔖Toolbar
function newBoardAuthoringId(prefix: string): string {
	if (typeof globalThis.crypto !== "undefined" && typeof globalThis.crypto.randomUUID === "function") {
		return `${prefix}-${globalThis.crypto.randomUUID()}`;
	}
	return `${prefix}-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;
}

function boardToolbarToggleClass(active: boolean): string {
	return [
		"inline-flex shrink-0 items-center justify-center rounded px-2 py-1 text-xs font-medium transition-colors",
		active ? "bg-accent text-accent-foreground border border-element" : "text-muted-foreground hover:bg-hover-panel border border-transparent",
	].join(" ");
}

/** @emoji 📐 Default node span in px: circle radius = span/2; rectangle width = height = span (40×40). */
const BOARD_PLAY_DEFAULT_NODE_SIZE_PX = 40;

/** @emoji 📐 Builds {@link BoardRedrawLayoutOptions} for the active pane camera center and redraw mode. */
function boardPlayRedrawLayoutOpts(
	pane: BoardPlayPaneId,
	camerasByPane: Record<BoardPlayPaneId, CameraState>,
	mode: BoardRedrawModeKind,
	forceIters: number,
	forceIdealEdge: number,
	forceGravity: number,
	forceRepulsion: number,
	treeLayerSpacing: number,
	treeSiblingGap: number,
	treeDirection: BoardHierarchicalTreeDirectionKind,
): BoardRedrawLayoutOptions {
	const cam = camerasByPane[pane];
	const cx = cam.x;
	const cy = cam.y;
	if (mode === "hierarchical-tree") {
		return {
			centerX: cx,
			centerY: cy,
			hierarchicalTree: {
				direction: treeDirection,
				layerSpacing: Math.max(24, treeLayerSpacing),
				siblingGap: Math.max(0, treeSiblingGap),
			},
			mode: "hierarchical-tree",
		};
	}
	const fg: BoardForceGraphLayoutOptions = {
		centerX: cx,
		centerY: cy,
		gravity: Math.max(0, forceGravity),
		idealEdgeLength: Math.max(8, forceIdealEdge),
		iterations: Math.max(1, Math.min(5000, Math.round(forceIters))),
		repulsionStrength: Math.max(40, forceRepulsion),
	};
	return { centerX: cx, centerY: cy, forceGraph: fg, mode: "force-graph" };
}

/** @emoji 🧰 Sketchpad-style tools: marquee kind, merge mode, hit target, and circle or rectangle authoring at the active pane camera. */
function BoardPlayToolbar(): ReactElement {
	const {
		activePaneId,
		boardSelectionMethod,
		boardSelectionMode,
		boardSelectionTargets,
		camerasByPane,
		boardRedrawPlaying,
		patchFixture,
		setBoardSelectionMethod,
		setBoardSelectionMode,
		setBoardSelectionTargets,
		setBoardRedrawPlaying,
		setSelectionForPane,
	} = useBoardPlayShell();

	const camera = camerasByPane[activePaneId];

	const appendCircle = useCallback(() => {
		const id = newBoardAuthoringId("node");
		const handleId = `${id}.h0`;
		const node: BoardFixtureCircleNodeV1 = {
			handles: [{ angle: 0, handleKind: BOARD_BUILTIN_PORT_HANDLE_KIND, id: handleId }],
			id,
			radius: BOARD_PLAY_DEFAULT_NODE_SIZE_PX / 2,
			x: camera.x,
			y: camera.y,
		};
		patchFixture((prev) => ({ ...prev, nodes: [...prev.nodes, node] }));
		setSelectionForPane(activePaneId, [id]);
	}, [activePaneId, camera.x, camera.y, patchFixture, setSelectionForPane]);

	const appendRectangle = useCallback(() => {
		const id = newBoardAuthoringId("node");
		const handleId = `${id}.h0`;
		const d = BOARD_PLAY_DEFAULT_NODE_SIZE_PX;
		const node: BoardFixtureRectangleNodeV1 = {
			handles: [{ angle: 0, handleKind: BOARD_BUILTIN_PORT_HANDLE_KIND, id: handleId }],
			height: d,
			id,
			shape: "rectangle",
			width: d,
			x: camera.x,
			y: camera.y,
		};
		patchFixture((prev) => ({ ...prev, nodes: [...prev.nodes, node] }));
		setSelectionForPane(activePaneId, [id]);
	}, [activePaneId, camera.x, camera.y, patchFixture, setSelectionForPane]);

	return (
		<div className="pointer-events-none flex w-full justify-center px-2 py-1">
			<ToolbarZone className="pointer-events-auto max-w-full flex-wrap justify-center gap-[var(--toolbar-gap)] px-2">
				<ToolbarGroup className="min-w-0 items-center gap-1">
					<ToolbarItem>
						<span className="text-muted-foreground pr-1 text-[10px] font-semibold uppercase tracking-wide">Select</span>
					</ToolbarItem>
					<ToolbarItem>
						<button
							type="button"
							className={boardToolbarToggleClass(boardSelectionMethod === "rectangle")}
							title="Rectangle selection"
							onClick={() => setBoardSelectionMethod("rectangle")}
						>
							<BoxSelect className="size-4" aria-hidden />
						</button>
					</ToolbarItem>
					<ToolbarItem>
						<button type="button" className={boardToolbarToggleClass(boardSelectionMethod === "lasso")} title="Lasso selection" onClick={() => setBoardSelectionMethod("lasso")}>
							<Lasso className="size-4" aria-hidden />
						</button>
					</ToolbarItem>
					<ToolbarItem>
						<button type="button" className={boardToolbarToggleClass(boardSelectionMode === "additive")} title="Additive" onClick={() => setBoardSelectionMode("additive")}>
							<Plus className="size-4" aria-hidden />
						</button>
					</ToolbarItem>
					<ToolbarItem>
						<button type="button" className={boardToolbarToggleClass(boardSelectionMode === "subtractive")} title="Subtractive" onClick={() => setBoardSelectionMode("subtractive")}>
							<Minus className="size-4" aria-hidden />
						</button>
					</ToolbarItem>
					<ToolbarItem>
						<button type="button" className={boardToolbarToggleClass(boardSelectionMode === "invertive")} title="Invertive" onClick={() => setBoardSelectionMode("invertive")}>
							<Repeat2 className="size-4" aria-hidden />
						</button>
					</ToolbarItem>
					<ToolbarItem>
						<span className="text-muted-foreground pr-1 text-[10px] font-semibold uppercase tracking-wide">Targets</span>
					</ToolbarItem>
					<ToolbarItem>
						<button
							type="button"
							className={boardToolbarToggleClass(boardSelectionTargets.nodes)}
							title="Select nodes"
							onClick={() => setBoardSelectionTargets((p) => ({ ...p, nodes: !p.nodes }))}
						>
							<span className="px-0.5">Nodes</span>
						</button>
					</ToolbarItem>
					<ToolbarItem>
						<button
							type="button"
							className={boardToolbarToggleClass(boardSelectionTargets.edges)}
							title="Select edges"
							onClick={() => setBoardSelectionTargets((p) => ({ ...p, edges: !p.edges }))}
						>
							<span className="px-0.5">Edges</span>
						</button>
					</ToolbarItem>
					<ToolbarItem>
						<button
							type="button"
							className={boardToolbarToggleClass(boardSelectionTargets.handles)}
							title="Select handles"
							onClick={() => setBoardSelectionTargets((p) => ({ ...p, handles: !p.handles }))}
						>
							<span className="px-0.5">Handles</span>
						</button>
					</ToolbarItem>
				</ToolbarGroup>
				<ToolbarDivider />
				<ToolbarGroup className="min-w-0 items-center gap-1">
					<ToolbarItem>
						<span className="text-muted-foreground pr-1 text-[10px] font-semibold uppercase tracking-wide">Create</span>
					</ToolbarItem>
					<ToolbarItem>
						<button type="button" className={boardToolbarToggleClass(false)} title="Circle" onClick={appendCircle}>
							<Circle className="size-4" aria-hidden />
						</button>
					</ToolbarItem>
					<ToolbarItem>
						<button type="button" className={boardToolbarToggleClass(false)} title="Rectangle" onClick={appendRectangle}>
							<Square className="size-4" aria-hidden />
						</button>
					</ToolbarItem>
				</ToolbarGroup>
				<ToolbarDivider />
				<ToolbarGroup className="min-w-0 items-center gap-1">
					<ToolbarItem>
						<span className="text-muted-foreground pr-1 text-[10px] font-semibold uppercase tracking-wide">Layout</span>
					</ToolbarItem>
					<ToolbarItem>
						<button
							type="button"
							className={boardToolbarToggleClass(boardRedrawPlaying)}
							title={
								boardRedrawPlaying
									? "Pause automatic redraw ticks"
									: "Play automatic redraw ticks (interval and mode in Settings)"
							}
							onClick={() => setBoardRedrawPlaying(!boardRedrawPlaying)}
						>
							{boardRedrawPlaying ? <Pause className="size-4" aria-hidden /> : <Play className="size-4" aria-hidden />}
						</button>
					</ToolbarItem>
				</ToolbarGroup>
			</ToolbarZone>
		</div>
	);
}
// #endregion 🔖Toolbar

// #region 🔖SettingsPanel
/** @emoji ⚙️ Board play redraw settings: shared tick timing, mode switch, and per-mode layout parameters. */
function BoardPlaySettingsPanel(): ReactElement {
	const {
		activePaneId,
		applyBoardRedrawOnce,
		boardRedrawMode,
		boardRedrawTickIterations,
		boardRedrawTickMs,
		forceLayoutFullIterations,
		forceLayoutGravity,
		forceLayoutIdealEdgeLength,
		forceLayoutRepulsionStrength,
		setBoardRedrawMode,
		setBoardRedrawTickIterations,
		setBoardRedrawTickMs,
		setForceLayoutFullIterations,
		setForceLayoutGravity,
		setForceLayoutIdealEdgeLength,
		setForceLayoutRepulsionStrength,
		setTreeLayoutLayerSpacing,
		setTreeLayoutDirection,
		setTreeLayoutSiblingGap,
		treeLayoutLayerSpacing,
		treeLayoutDirection,
		treeLayoutSiblingGap,
	} = useBoardPlayShell();

	return (
		<div className="flex h-full min-h-0 flex-col gap-2 p-3 text-xs">
			<div className="text-muted-foreground flex shrink-0 items-center gap-2 border-b border-element pb-2">
				<Settings className="size-4 shrink-0" />
				<div>
					<div className="font-semibold uppercase tracking-wide">Settings</div>
					<div className="text-[11px] opacity-80">pane: {activePaneId}</div>
				</div>
			</div>
			<div className="text-muted-foreground shrink-0 text-[11px] font-medium uppercase tracking-wide">Redraw</div>
			<div className="min-h-0 flex-1 space-y-4 overflow-y-auto">
				<Label id="board.play.settings.redraw.mode" label="Redraw mode">
					<Select onValueChange={(v) => setBoardRedrawMode(v as BoardRedrawModeKind)} value={boardRedrawMode}>
						<SelectTrigger className="h-8 w-full" id="board-play-redraw-mode" size="sm">
							<SelectValue />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value="force-graph">Force graph</SelectItem>
							<SelectItem value="hierarchical-tree">Hierarchical tree</SelectItem>
						</SelectContent>
					</Select>
				</Label>
				<Label id="board.play.settings.redraw.tickMs" label="Play interval (ms)">
					<Slider
						id="board-play-slider-redraw-tick-ms"
						max={400}
						min={24}
						step={8}
						value={[boardRedrawTickMs]}
						onValueChange={(vals) => setBoardRedrawTickMs(vals[0] ?? 96)}
					/>
				</Label>
				{boardRedrawMode === "force-graph" ? (
					<Label id="board.play.settings.redraw.tickIterations" label="Iterations per play tick">
						<Slider
							id="board-play-slider-redraw-tick-iters"
							max={72}
							min={1}
							step={1}
							value={[boardRedrawTickIterations]}
							onValueChange={(vals) => setBoardRedrawTickIterations(vals[0] ?? 10)}
						/>
					</Label>
				) : (
					<p className="text-muted-foreground text-[11px] leading-snug">
						Hierarchical tree redraw is deterministic; each tick re-centers the ranked layout on the active pane camera.
					</p>
				)}
				{boardRedrawMode === "force-graph" ? (
					<>
						<div className="text-muted-foreground pt-1 text-[11px] font-medium uppercase tracking-wide">Force graph</div>
						<Label id="board.play.settings.force.fullIterations" label="Iterations (apply once)">
							<Slider
								id="board-play-slider-force-full-iters"
								max={720}
								min={24}
								step={4}
								value={[forceLayoutFullIterations]}
								onValueChange={(vals) => setForceLayoutFullIterations(vals[0] ?? 200)}
							/>
						</Label>
						<Label id="board.play.settings.force.idealEdge" label="Ideal edge (px)">
							<Slider
								id="board-play-slider-force-ideal"
								max={160}
								min={20}
								step={2}
								value={[forceLayoutIdealEdgeLength]}
								onValueChange={(vals) => setForceLayoutIdealEdgeLength(vals[0] ?? 64)}
							/>
						</Label>
						<Label id="board.play.settings.force.repulsion" label="Repulsion">
							<Slider
								id="board-play-slider-force-repulsion"
								max={1800}
								min={80}
								step={20}
								value={[forceLayoutRepulsionStrength]}
								onValueChange={(vals) => setForceLayoutRepulsionStrength(vals[0] ?? 480)}
							/>
						</Label>
						<Label id="board.play.settings.force.gravity" label="Gravity">
							<Slider
								id="board-play-slider-force-gravity"
								max={0.05}
								min={0}
								step={0.002}
								value={[forceLayoutGravity]}
								onValueChange={(vals) => setForceLayoutGravity(vals[0] ?? 0)}
							/>
						</Label>
					</>
				) : (
					<>
						<div className="text-muted-foreground pt-1 text-[11px] font-medium uppercase tracking-wide">Hierarchical tree</div>
						<Label id="board.play.settings.tree.layerSpacing" label="Layer spacing (px)">
							<Slider
								id="board-play-slider-tree-layer"
								max={280}
								min={40}
								step={4}
								value={[treeLayoutLayerSpacing]}
								onValueChange={(vals) => setTreeLayoutLayerSpacing(vals[0] ?? 120)}
							/>
						</Label>
						<Label id="board.play.settings.tree.siblingGap" label="Sibling gap (px)">
							<Slider
								id="board-play-slider-tree-sibling"
								max={120}
								min={0}
								step={2}
								value={[treeLayoutSiblingGap]}
								onValueChange={(vals) => setTreeLayoutSiblingGap(vals[0] ?? 28)}
							/>
						</Label>
						<Label id="board.play.settings.tree.direction" label="Direction">
							<Select onValueChange={(v) => setTreeLayoutDirection(v as BoardHierarchicalTreeDirectionKind)} value={treeLayoutDirection}>
								<SelectTrigger className="h-8 w-full" id="board-play-tree-direction" size="sm">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="downwards">Downwards</SelectItem>
									<SelectItem value="upwards">Upwards</SelectItem>
									<SelectItem value="right">Right</SelectItem>
									<SelectItem value="left">Left</SelectItem>
								</SelectContent>
							</Select>
						</Label>
					</>
				)}
				<Button className="h-8 w-full text-xs" type="button" variant="secondary" onClick={applyBoardRedrawOnce}>
					Apply redraw once
				</Button>
				<p className="text-muted-foreground text-[11px] leading-snug">
					While play is on, camera framing stays pinned to the graph as it was when you pressed play, so zoom and pan no longer jump each tick. Pause to re-frame from the latest layout.
				</p>
			</div>
		</div>
	);
}
// #endregion 🔖SettingsPanel

// #region 🔖Scene
/** @emoji 🗼 Marker tree for {@link BoardCanvas} — must stay a Fragment of {@link Node}/{@link Edge} so {@link buildBoardSceneDescriptor} sees markers (custom wrappers are opaque to the static walk). */
function nakaginBoardMarkers(fixture: BoardFixtureV1, selectedIds: Set<string>): ReactElement {
	const demoNodeId = fixture.nodes[0]?.id;
	const demoEdgeId = fixture.edges[0]?.id;
	return (
		<>
			{fixture.nodes.map((node) =>
				node.shape === "rectangle" ? (
					<Node
						contextMenu={node.id === demoNodeId ? boardPlayDemoNodeContextMenu : undefined}
						draggable
						height={node.height}
						id={node.id}
						key={node.id}
						shape="rectangle"
						selected={selectedIds.has(node.id)}
						text={node.text}
						textAlignment={node.textAlignment}
						textAutofit={node.textAutofit === true}
						textFontFamily={node.textFontFamily}
						textFontSize={node.textFontSize}
						width={node.width}
						x={node.x}
						y={node.y}
					>
						{node.handles.map((handle) => (
							<Handle
								angle={handle.angle}
								color={handle.color}
								handleKind={handle.handleKind}
								id={handle.id}
								key={handle.id}
								radius={handle.radius}
								selected={selectedIds.has(handle.id)}
							/>
						))}
					</Node>
				) : (
					<Node
						contextMenu={node.id === demoNodeId ? boardPlayDemoNodeContextMenu : undefined}
						draggable
						id={node.id}
						key={node.id}
						radius={node.radius}
						selected={selectedIds.has(node.id)}
						text={node.text}
						textAlignment={node.textAlignment}
						textAutofit={node.textAutofit === true}
						textFontFamily={node.textFontFamily}
						textFontSize={node.textFontSize}
						x={node.x}
						y={node.y}
					>
						{node.handles.map((handle) => (
							<Handle
								angle={handle.angle}
								color={handle.color}
								handleKind={handle.handleKind}
								id={handle.id}
								key={handle.id}
								radius={handle.radius}
								selected={selectedIds.has(handle.id)}
							/>
						))}
					</Node>
				),
			)}
			{fixture.edges.map((edge) => (
				<Edge
					contextMenu={edge.id === demoEdgeId ? boardPlayDemoEdgeContextMenu : undefined}
					id={edge.id}
					key={edge.id}
					selected={selectedIds.has(edge.id)}
					source={edge.source}
					target={edge.target}
				/>
			))}
		</>
	);
}

/** @emoji 📡 Mirrors canvas selection into shell state for the owning pane. */
function BoardSelectionReporter({ paneId }: { paneId: BoardPlayPaneId }): null {
	const { setSelectionForPane } = useBoardPlayShell();
	const handler = useCallback(
		(snapshot: { ids: string[] }) => {
			setSelectionForPane(paneId, snapshot.ids);
		},
		[paneId, setSelectionForPane],
	);
	useBoardEvent("select", handler);
	return null;
}
/** @emoji 🗑️ Keeps the shared shell fixture aligned with canvas `edgeDelete` / `nodeDelete` events. */
function BoardStructuralDeleteReporter(): null {
	const { applyStructuralDelete } = useBoardPlayShell();
	const onEdgeDelete = useCallback(
		(event: { id: string }) => {
			applyStructuralDelete("edge", event.id);
		},
		[applyStructuralDelete],
	);
	const onNodeDelete = useCallback(
		(event: { id: string }) => {
			applyStructuralDelete("node", event.id);
		},
		[applyStructuralDelete],
	);
	useBoardEvent("edgeDelete", onEdgeDelete);
	useBoardEvent("nodeDelete", onNodeDelete);
	return null;
}
// #endregion 🔖Scene

// #region 🔖Panes
/** @emoji 🪟 Captures pointer focus for the active pane (tabs + canvas). */
function BoardPaneChrome({ children, paneId }: { children: ReactNode; paneId: BoardPlayPaneId }): ReactElement {
	const { setActivePaneId } = useBoardPlayShell();
	return (
		<div
			className="flex h-full min-h-0 w-full flex-col"
			onPointerDownCapture={() => {
				setActivePaneId(paneId);
			}}
		>
			{children}
		</div>
	);
}

function BoardOverviewPane(): ReactElement {
	const { boardSelectionMethod, boardSelectionMode, boardSelectionTargets, fixture, handleCanvasFixtureDrop, camerasByPane, selectionByPane } =
		useBoardPlayShell();
	const paneId: BoardPlayPaneId = "board-overview";
	const camera = camerasByPane[paneId];
	const selectedIds = selectionByPane[paneId];
	return (
		<BoardPaneChrome paneId={paneId}>
			<BoardCanvas
				camera={camera}
				className="min-h-0 flex-1"
				contextMenu={boardPlayCanvasBackgroundMenu}
				fixtureDragDrop
				handleKinds={BOARD_DEFAULT_HANDLE_KIND_CATALOG}
				onFixtureDrop={(d) => handleCanvasFixtureDrop(paneId, d)}
				selectionMethod={boardSelectionMethod}
				selectionMode={boardSelectionMode}
				selectionTargets={boardSelectionTargets}
			>
				<BoardSelectionReporter paneId={paneId} />
				<BoardStructuralDeleteReporter />
				{nakaginBoardMarkers(fixture, selectedIds)}
			</BoardCanvas>
		</BoardPaneChrome>
	);
}

function BoardDetailPane(): ReactElement {
	const { boardSelectionMethod, boardSelectionMode, boardSelectionTargets, fixture, handleCanvasFixtureDrop, camerasByPane, selectionByPane } =
		useBoardPlayShell();
	const paneId: BoardPlayPaneId = "board-detail";
	const camera = camerasByPane[paneId];
	const selectedIds = selectionByPane[paneId];
	return (
		<BoardPaneChrome paneId={paneId}>
			<BoardCanvas
				camera={camera}
				className="min-h-0 flex-1"
				fixtureDragDrop
				handleKinds={BOARD_DEFAULT_HANDLE_KIND_CATALOG}
				onFixtureDrop={(d) => handleCanvasFixtureDrop(paneId, d)}
				selectionMethod={boardSelectionMethod}
				selectionMode={boardSelectionMode}
				selectionTargets={boardSelectionTargets}
			>
				<BoardSelectionReporter paneId={paneId} />
				<BoardStructuralDeleteReporter />
				{nakaginBoardMarkers(fixture, selectedIds)}
			</BoardCanvas>
		</BoardPaneChrome>
	);
}

function BoardSelectionPane(): ReactElement {
	const { boardSelectionMethod, boardSelectionMode, boardSelectionTargets, fixture, handleCanvasFixtureDrop, camerasByPane, selectionByPane } =
		useBoardPlayShell();
	const paneId: BoardPlayPaneId = "board-selection";
	const camera = camerasByPane[paneId];
	const selectedIds = selectionByPane[paneId];
	return (
		<BoardPaneChrome paneId={paneId}>
			<BoardCanvas
				camera={camera}
				className="min-h-0 flex-1"
				fixtureDragDrop
				handleKinds={BOARD_DEFAULT_HANDLE_KIND_CATALOG}
				onFixtureDrop={(d) => handleCanvasFixtureDrop(paneId, d)}
				selectionMethod={boardSelectionMethod}
				selectionMode={boardSelectionMode}
				selectionTargets={boardSelectionTargets}
			>
				<BoardSelectionReporter paneId={paneId} />
				<BoardStructuralDeleteReporter />
				{nakaginBoardMarkers(fixture, selectedIds)}
			</BoardCanvas>
		</BoardPaneChrome>
	);
}
// #endregion 🔖Panes

// #region 🔖SidePanels
// #region 🔖PaletteFixtureShelf
/** @emoji 📐 Palette seeds match {@link BOARD_PLAY_DEFAULT_NODE_SIZE_PX} (circle radius = span/2). */

const BOARD_PLAY_PALETTE_CIRCLE_DRAG_FIXTURE: BoardFixtureV1 =
	parseBoardFixtureV1({
		camera: { x: 0, y: 0, zoom: 1 },
		edges: [],
		meta: { boardFixtureDragKind: BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE },
		nodes: [{ handles: [{ angle: 0, id: "palette-seed-circle.h0" }], id: "palette-seed-circle", radius: BOARD_PLAY_DEFAULT_NODE_SIZE_PX / 2, x: 0, y: 0 }],
		schema: "elements.board.fixture/v1",
	}) ?? (() => {
		throw new Error("Board play: palette circle drag fixture failed validation.");
	})();

const BOARD_PLAY_PALETTE_RECTANGLE_DRAG_FIXTURE: BoardFixtureV1 =
	parseBoardFixtureV1({
		camera: { x: 0, y: 0, zoom: 1 },
		edges: [],
		meta: { boardFixtureDragKind: BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE },
		nodes: [
			{
				handles: [{ angle: 0, id: "palette-seed-rectangle.h0" }],
				height: BOARD_PLAY_DEFAULT_NODE_SIZE_PX,
				id: "palette-seed-rectangle",
				shape: "rectangle",
				width: BOARD_PLAY_DEFAULT_NODE_SIZE_PX,
				x: 0,
				y: 0,
			},
		],
		schema: "elements.board.fixture/v1",
	}) ?? (() => {
		throw new Error("Board play: palette rectangle drag fixture failed validation.");
	})();

/** @emoji 🧩 When {@link BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE} is on meta, returns one node placed at the drop world point; else null so the scene should be replaced. */
function mergePaletteNodeFromDrop(detail: BoardFixtureDropDetail): BoardFixtureNodeV1 | null {
	if (detail.fixture.meta?.boardFixtureDragKind !== BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE) {
		return null;
	}
	const template = detail.fixture.nodes[0];
	if (!template) {
		return null;
	}
	const newId = newBoardAuthoringId("node");
	return {
		...template,
		handles: template.handles.map((h, i) => ({ ...h, id: `${newId}.h${i}` })),
		id: newId,
		x: detail.world.x,
		y: detail.world.y,
	};
}

/** @emoji 👻 Draggable chip with drag image rendered under `document.body` so host panel overflow does not clip the preview. */
function BoardFixturePaletteDraggable(props: { fixture: BoardFixtureV1; label: string; preview: ReactNode }): ReactElement {
	const { fixture: dragFixture, label, preview } = props;
	const ghostRef = useRef<HTMLDivElement>(null);
	const onDragStart = useCallback(
		(e: DragEvent<HTMLDivElement>) => {
			e.dataTransfer.setData(BOARD_FIXTURE_DRAG_V1_MIME, encodeBoardFixtureForDragV1(dragFixture));
			e.dataTransfer.effectAllowed = "copy";
			const ghost = ghostRef.current;
			if (ghost) {
				const { height, width } = ghost.getBoundingClientRect();
				e.dataTransfer.setDragImage(ghost, width / 2, height / 2);
			}
		},
		[dragFixture],
	);
	return (
		<>
			{typeof document !== "undefined"
				? createPortal(
						<div
							aria-hidden
							className="border-element bg-muted/40 pointer-events-none fixed z-[2147483000] flex items-center justify-center rounded-lg border shadow-sm"
							ref={ghostRef}
							style={{ height: BOARD_PLAY_DEFAULT_NODE_SIZE_PX, left: -9999, top: 0, width: BOARD_PLAY_DEFAULT_NODE_SIZE_PX }}
						>
							{preview}
						</div>,
						document.body,
					)
				: null}
			<div
				className="border-element bg-background flex h-10 w-10 shrink-0 cursor-grab items-center justify-center rounded-lg border active:cursor-grabbing"
				draggable
				onDragStart={onDragStart}
				title={label}
			>
				{preview}
			</div>
		</>
	);
}
// #endregion 🔖PaletteFixtureShelf

/** @emoji 📥 Left rail: drag the active graph onto a board pane (in-app MIME payload, not filesystem JSON files). */
function BoardFixtureLibraryPanel(): ReactElement {
	const { fixture } = useBoardPlayShell();

	const onShelfDragStart = useCallback(
		(e: DragEvent<HTMLDivElement>) => {
			e.dataTransfer.setData(BOARD_FIXTURE_DRAG_V1_MIME, encodeBoardFixtureForDragV1(fixture));
			e.dataTransfer.effectAllowed = "copy";
		},
		[fixture],
	);

	return (
		<div className="flex h-full min-h-0 flex-col gap-3 p-3 text-sm">
			<div className="text-muted-foreground text-xs uppercase tracking-wide" data-testid="board-play-fixture-shelf">
				Fixture shelf
			</div>
			<div className="flex flex-col gap-2">
				<div className="text-muted-foreground text-[11px] uppercase tracking-wide">Shapes</div>
				<div className="flex flex-wrap gap-2">
					<BoardFixturePaletteDraggable
						fixture={BOARD_PLAY_PALETTE_CIRCLE_DRAG_FIXTURE}
						label="Drag circle onto the board"
						preview={<div className="border-primary size-10 shrink-0 rounded-full border-2 bg-accent/30" />}
					/>
					<BoardFixturePaletteDraggable
						fixture={BOARD_PLAY_PALETTE_RECTANGLE_DRAG_FIXTURE}
						label="Drag rectangle onto the board"
						preview={<div className="border-primary size-10 shrink-0 rounded-sm border-2 bg-accent/30" />}
					/>
				</div>
			</div>
			<div
				className="border-element bg-muted/30 flex min-h-[120px] cursor-grab flex-col justify-center gap-2 rounded-md border p-4 active:cursor-grabbing"
				draggable
				onDragStart={onShelfDragStart}
			>
				<p className="font-medium">Active graph</p>
				<p className="text-muted-foreground text-xs">Drag onto any board tab to load this graph (same payload for all panes).</p>
			</div>
			<div className="border-element space-y-1 rounded border p-2 text-xs">
				<div className="text-muted-foreground">Loaded</div>
				<div>schema: {fixture.schema}</div>
				<div>
					nodes: {fixture.nodes.length} · edges: {fixture.edges.length}
				</div>
			</div>
		</div>
	);
}

function findNode(fixture: BoardFixtureV1, id: string): BoardFixtureNodeV1 | undefined {
	return fixture.nodes.find((n) => n.id === id);
}

function findEdge(fixture: BoardFixtureV1, id: string): BoardFixtureEdgeV1 | undefined {
	return fixture.edges.find((e) => e.id === id);
}

function findHandleOwner(fixture: BoardFixtureV1, handleId: string): { node: BoardFixtureNodeV1; handleId: string } | undefined {
	for (const node of fixture.nodes) {
		if (node.handles.some((h) => h.id === handleId)) {
			return { handleId, node };
		}
	}
	return undefined;
}

function findHandle(fixture: BoardFixtureV1, handleId: string): BoardFixtureHandleV1 | undefined {
	for (const node of fixture.nodes) {
		const h = node.handles.find((x) => x.id === handleId);
		if (h) {
			return h;
		}
	}
	return undefined;
}

function nodeIsRectangle(n: BoardFixtureNodeV1): n is BoardFixtureRectangleNodeV1 {
	return n.shape === "rectangle";
}

function allEqual<T>(values: T[]): boolean {
	if (values.length === 0) {
		return true;
	}
	const first = values[0];
	return values.every((v) => v === first);
}

function listHandleIds(fixture: BoardFixtureV1): string[] {
	const out: string[] = [];
	for (const node of fixture.nodes) {
		for (const h of node.handles) {
			out.push(h.id);
		}
	}
	out.sort((a, b) => a.localeCompare(b));
	return out;
}

function toCircleNode(n: BoardFixtureRectangleNodeV1): BoardFixtureCircleNodeV1 {
	const { width, height, shape: _s, ...rest } = n;
	const radius = Math.min(width, height) / 2;
	return { ...rest, radius, shape: "circle" };
}

function toRectangleNode(n: BoardFixtureCircleNodeV1): BoardFixtureRectangleNodeV1 {
	const { radius, shape: _s, ...rest } = n;
	return { ...rest, shape: "rectangle", width: radius * 2, height: radius * 2 };
}

/** @emoji 🎯 Normalizes θ to `[0, 2π)`. */
function normalizeAngleRad(t: number): number {
	const twoPi = Math.PI * 2;
	let x = t % twoPi;
	if (x < 0) {
		x += twoPi;
	}
	return x;
}

/** @emoji ⭕ Draggable ring control for handle polar angle `t` (radians, east-zero CCW in board space). */
function AngleTRing({ angleUniform, onChange, value }: { angleUniform: boolean; onChange: (next: number) => void; value: number }): ReactElement {
	const ref = useRef<HTMLDivElement | null>(null);
	const dragging = useRef(false);

	const setFromClient = useCallback(
		(clientX: number, clientY: number) => {
			const el = ref.current;
			if (!el) {
				return;
			}
			const r = el.getBoundingClientRect();
			const cx = r.left + r.width / 2;
			const cy = r.top + r.height / 2;
			const dx = clientX - cx;
			const dy = clientY - cy;
			onChange(normalizeAngleRad(Math.atan2(dy, dx)));
		},
		[onChange],
	);

	const onPointerDown = useCallback(
		(e: PointerEvent) => {
			e.preventDefault();
			dragging.current = true;
			ref.current?.setPointerCapture(e.pointerId);
			setFromClient(e.clientX, e.clientY);
		},
		[setFromClient],
	);

	const onPointerMove = useCallback(
		(e: PointerEvent) => {
			if (!dragging.current) {
				return;
			}
			setFromClient(e.clientX, e.clientY);
		},
		[setFromClient],
	);

	const onPointerUp = useCallback((e: PointerEvent) => {
		dragging.current = false;
		try {
			ref.current?.releasePointerCapture(e.pointerId);
		} catch {
			/* ignore */
		}
	}, []);

	const size = 88;
	const stroke = 3;
	const r = size / 2 - stroke * 2;
	const cx = size / 2;
	const cy = size / 2;
	const knobX = cx + r * Math.cos(value);
	const knobY = cy + r * Math.sin(value);

	return (
		<div className="flex flex-col items-center gap-1">
			<div
				className={`border-element bg-muted/20 touch-none select-none rounded-full border ${angleUniform ? "" : "pointer-events-none opacity-40"}`}
				onPointerCancel={onPointerUp}
				onPointerDown={angleUniform ? onPointerDown : undefined}
				onPointerMove={angleUniform ? onPointerMove : undefined}
				onPointerUp={angleUniform ? onPointerUp : undefined}
				ref={ref}
				style={{ height: size, width: size }}
			>
				<svg aria-label="Angle t" height={size} viewBox={`0 0 ${size} ${size}`} width={size}>
					<circle cx={cx} cy={cy} fill="none" r={r} stroke="currentColor" strokeOpacity={0.35} strokeWidth={stroke} />
					<line stroke="currentColor" strokeOpacity={0.45} strokeWidth={1} x1={cx} x2={cx + r} y1={cy} y2={cy} />
					<line stroke="currentColor" strokeOpacity={0.25} strokeWidth={1} x1={cx} x2={cx} y1={cy} y2={cy - r} />
					<circle cx={knobX} cy={knobY} fill="var(--foreground)" r={5} stroke="var(--background)" strokeWidth={2} />
				</svg>
			</div>
			<div className="text-muted-foreground font-mono text-[10px]">{angleUniform ? `t = ${value.toFixed(4)} rad` : "Mixed t"}</div>
		</div>
	);
}

function NumericStepperRow({
	id,
	label,
	onAbsolute,
	onDelta,
	step,
	uniform,
	value,
}: {
	id: string;
	label: string;
	onAbsolute: (next: number) => void;
	onDelta: (delta: number) => void;
	step: number;
	uniform: boolean;
	value: number;
}): ReactElement {
	return (
		<Label id={id} label={label}>
			<div className="flex min-w-0 items-center gap-1">
				<Button className="h-7 shrink-0 px-2" onClick={() => onDelta(-step)} type="button" variant="outline">
					−
				</Button>
				<Input
					className="h-7 min-w-0 flex-1 font-mono text-xs"
					onChange={(e: ChangeEvent<HTMLInputElement>) => {
						const parsed = Number(e.target.value);
						if (Number.isFinite(parsed)) {
							onAbsolute(parsed);
						}
					}}
					placeholder={uniform ? undefined : "Mixed"}
					value={uniform && Number.isFinite(value) ? String(value) : ""}
				/>
				<Button className="h-7 shrink-0 px-2" onClick={() => onDelta(step)} type="button" variant="outline">
					+
				</Button>
			</div>
		</Label>
	);
}

/** @emoji 🟠 Batch node inspector: name (`text`), shape, center, size fields apply to every selected node. */
function InspectorNodeBatch({
	fixture,
	nodeIds,
	patchFixture,
	remapIdInSelections,
}: {
	fixture: BoardFixtureV1;
	nodeIds: readonly string[];
	patchFixture: (updater: (prev: BoardFixtureV1) => BoardFixtureV1) => void;
	remapIdInSelections: (replacedId: string, replacementId: string) => void;
}): ReactElement {
	const idSet = useMemo(() => new Set(nodeIds), [nodeIds]);
	const targets = useMemo(
		() => nodeIds.map((id) => findNode(fixture, id)).filter((n): n is BoardFixtureNodeV1 => Boolean(n)),
		[fixture, nodeIds],
	);

	const textValues = targets.map((n) => n.text ?? "");
	const textUniform = allEqual(textValues);
	const textValue = textUniform ? (textValues[0] ?? "") : "";

	const shapes = targets.map((n) => (nodeIsRectangle(n) ? "rectangle" : "circle"));
	const shapeUniform = allEqual(shapes);
	const shapeValue = shapeUniform ? shapes[0] : undefined;

	const xs = targets.map((n) => n.x);
	const ys = targets.map((n) => n.y);
	const xUniform = allEqual(xs);
	const yUniform = allEqual(ys);
	const xValue = xUniform ? xs[0] : Number.NaN;
	const yValue = yUniform ? ys[0] : Number.NaN;

	const radii = targets.filter((n) => !nodeIsRectangle(n)).map((n) => n.radius);
	const widths = targets.filter(nodeIsRectangle).map((n) => n.width);
	const heights = targets.filter(nodeIsRectangle).map((n) => n.height);
	const rUniform = radii.length > 0 && allEqual(radii);
	const wUniform = widths.length > 0 && allEqual(widths);
	const hUniform = heights.length > 0 && allEqual(heights);
	const rValue = rUniform ? radii[0] : Number.NaN;
	const wValue = wUniform ? widths[0] : Number.NaN;
	const hValue = hUniform ? heights[0] : Number.NaN;

	const patchNodes = useCallback(
		(updater: (n: BoardFixtureNodeV1) => BoardFixtureNodeV1) => {
			patchFixture((prev) => ({
				...prev,
				nodes: prev.nodes.map((n) => (idSet.has(n.id) ? updater(n) : n)),
			}));
		},
		[idSet, patchFixture],
	);

	const onText = useCallback(
		(next: string) => {
			patchNodes((n) => ({ ...n, text: next === "" ? undefined : next }));
		},
		[patchNodes],
	);

	const onShape = useCallback(
		(next: "circle" | "rectangle") => {
			patchNodes((n) => {
				if (next === "rectangle" && !nodeIsRectangle(n)) {
					return toRectangleNode(n);
				}
				if (next === "circle" && nodeIsRectangle(n)) {
					return toCircleNode(n);
				}
				return n;
			});
		},
		[patchNodes],
	);

	return (
		<div className="border-element/60 space-y-3 border-l pl-2">
			{nodeIds.length === 1 ? (
				<Label id="board-play.inspector.node.id" label="Id">
					<Input
						className="h-7 font-mono text-xs"
						defaultValue={nodeIds[0]}
						key={nodeIds[0]}
						onBlur={(e) => {
							const nextId = e.currentTarget.value.trim();
							const oldId = nodeIds[0];
							if (!oldId || !nextId || nextId === oldId) {
								return;
							}
							patchFixture((prev) => ({
								...prev,
								nodes: prev.nodes.map((n) => (n.id === oldId ? { ...n, id: nextId } : n)),
							}));
							remapIdInSelections(oldId, nextId);
						}}
					/>
				</Label>
			) : null}
			<Label id="board-play.inspector.node.name" label="Name">
				<Input
					className="h-7 font-mono text-xs"
					onChange={(e: ChangeEvent<HTMLInputElement>) => onText(e.target.value)}
					placeholder={textUniform ? undefined : "Mixed"}
					value={textValue}
				/>
			</Label>
			<Label id="board-play.inspector.node.shape" label="Shape">
				<Select
					key={shapeUniform && shapeValue ? `shape-${shapeValue}` : "shape-mixed"}
					onValueChange={(v) => {
						if (v === "circle" || v === "rectangle") {
							onShape(v);
						}
					}}
					value={shapeUniform && shapeValue ? shapeValue : undefined}
				>
					<SelectTrigger className="h-7 font-mono text-xs">
						<SelectValue placeholder={shapeUniform ? "shape" : "Mixed"} />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="circle">circle</SelectItem>
						<SelectItem value="rectangle">rectangle</SelectItem>
					</SelectContent>
				</Select>
			</Label>
			<NumericStepperRow
				id="board-play.inspector.node.x"
				label="x"
				onAbsolute={(v) => patchNodes((n) => ({ ...n, x: v }))}
				onDelta={(d) => patchNodes((n) => ({ ...n, x: n.x + d }))}
				step={1}
				uniform={xUniform}
				value={xValue}
			/>
			<NumericStepperRow
				id="board-play.inspector.node.y"
				label="y"
				onAbsolute={(v) => patchNodes((n) => ({ ...n, y: v }))}
				onDelta={(d) => patchNodes((n) => ({ ...n, y: n.y + d }))}
				step={1}
				uniform={yUniform}
				value={yValue}
			/>
			{targets.some((n) => !nodeIsRectangle(n)) ? (
				<NumericStepperRow
					id="board-play.inspector.node.r"
					label="radius"
					onAbsolute={(v) => patchNodes((n) => (nodeIsRectangle(n) ? n : { ...n, radius: Math.max(1e-6, v) }))}
					onDelta={(d) => patchNodes((n) => (nodeIsRectangle(n) ? n : { ...n, radius: Math.max(1e-6, n.radius + d) }))}
					step={1}
					uniform={rUniform}
					value={rValue}
				/>
			) : null}
			{targets.some(nodeIsRectangle) ? (
				<>
					<NumericStepperRow
						id="board-play.inspector.node.w"
						label="width"
						onAbsolute={(v) => patchNodes((n) => (nodeIsRectangle(n) ? { ...n, width: Math.max(1e-6, v) } : n))}
						onDelta={(d) => patchNodes((n) => (nodeIsRectangle(n) ? { ...n, width: Math.max(1e-6, n.width + d) } : n))}
						step={1}
						uniform={wUniform}
						value={wValue}
					/>
					<NumericStepperRow
						id="board-play.inspector.node.h"
						label="height"
						onAbsolute={(v) => patchNodes((n) => (nodeIsRectangle(n) ? { ...n, height: Math.max(1e-6, v) } : n))}
						onDelta={(d) => patchNodes((n) => (nodeIsRectangle(n) ? { ...n, height: Math.max(1e-6, n.height + d) } : n))}
						step={1}
						uniform={hUniform}
						value={hValue}
					/>
				</>
			) : null}
		</div>
	);
}

/** @emoji 🟣 Batch handle inspector: polar `t`, hit radius, optional id when single selection. */
function InspectorHandleBatch({
	fixture,
	handleIds,
	patchFixture,
	remapIdInSelections,
}: {
	fixture: BoardFixtureV1;
	handleIds: readonly string[];
	patchFixture: (updater: (prev: BoardFixtureV1) => BoardFixtureV1) => void;
	remapIdInSelections: (replacedId: string, replacementId: string) => void;
}): ReactElement {
	const idSet = useMemo(() => new Set(handleIds), [handleIds]);
	const handles = useMemo(
		() => handleIds.map((id) => findHandle(fixture, id)).filter((h): h is BoardFixtureHandleV1 => Boolean(h)),
		[fixture, handleIds],
	);
	const angles = handles.map((h) => h.angle);
	const angleUniform = allEqual(angles);
	const angleValue = angleUniform ? angles[0]! : 0;
	const radii = handles.map((h) => h.radius ?? 8);
	const radiusUniform = allEqual(radii);
	const radiusValue = radiusUniform ? radii[0]! : Number.NaN;

	const patchHandles = useCallback(
		(updater: (h: BoardFixtureHandleV1) => BoardFixtureHandleV1) => {
			patchFixture((prev) => ({
				...prev,
				nodes: prev.nodes.map((node) => ({
					...node,
					handles: node.handles.map((h) => (idSet.has(h.id) ? updater(h) : h)),
				})),
			}));
		},
		[idSet, patchFixture],
	);

	return (
		<div className="border-element/60 space-y-3 border-l pl-2">
			<div className="flex flex-wrap items-start gap-4">
				<AngleTRing
					angleUniform={angleUniform}
					onChange={(t) => {
						patchHandles((h) => ({ ...h, angle: t }));
					}}
					value={angleValue}
				/>
				<div className="min-w-0 flex-1 space-y-3">
					<NumericStepperRow
						id="board-play.inspector.handle.t"
						label="t (rad)"
						onAbsolute={(v) => patchHandles((h) => ({ ...h, angle: normalizeAngleRad(v) }))}
						onDelta={(d) => patchHandles((h) => ({ ...h, angle: normalizeAngleRad(h.angle + d) }))}
						step={0.05}
						uniform={angleUniform}
						value={angleUniform ? angleValue : Number.NaN}
					/>
					<NumericStepperRow
						id="board-play.inspector.handle.radius"
						label="Hit radius"
						onAbsolute={(v) => patchHandles((h) => ({ ...h, radius: Math.max(1e-6, v) }))}
						onDelta={(d) => patchHandles((h) => ({ ...h, radius: Math.max(1e-6, (h.radius ?? 8) + d) }))}
						step={1}
						uniform={radiusUniform}
						value={radiusValue}
					/>
					{handleIds.length === 1 ? (
						<Label id="board-play.inspector.handle.id" label="Id">
							<Input
								className="h-7 font-mono text-xs"
								defaultValue={handleIds[0]}
								key={handleIds[0]}
								onBlur={(e) => {
									const nextId = e.currentTarget.value.trim();
									const oldId = handleIds[0];
									if (!oldId || !nextId || nextId === oldId) {
										return;
									}
									patchFixture((prev) => ({
										...prev,
										edges: prev.edges.map((edge) => ({
											...edge,
											source: edge.source === oldId ? nextId : edge.source,
											target: edge.target === oldId ? nextId : edge.target,
										})),
										nodes: prev.nodes.map((node) => ({
											...node,
											handles: node.handles.map((h) => (h.id === oldId ? { ...h, id: nextId } : h)),
										})),
									}));
									remapIdInSelections(oldId, nextId);
								}}
							/>
						</Label>
					) : null}
				</div>
			</div>
		</div>
	);
}

/** @emoji 🪢 Batch edge inspector: endpoints and id (single). */
function InspectorEdgeBatch({
	fixture,
	edgeIds,
	patchFixture,
	remapIdInSelections,
}: {
	fixture: BoardFixtureV1;
	edgeIds: readonly string[];
	patchFixture: (updater: (prev: BoardFixtureV1) => BoardFixtureV1) => void;
	remapIdInSelections: (replacedId: string, replacementId: string) => void;
}): ReactElement {
	const idSet = useMemo(() => new Set(edgeIds), [edgeIds]);
	const edges = useMemo(
		() => edgeIds.map((id) => findEdge(fixture, id)).filter((e): e is BoardFixtureEdgeV1 => Boolean(e)),
		[edgeIds, fixture],
	);
	const sources = edges.map((e) => e.source);
	const targets = edges.map((e) => e.target);
	const sourceUniform = allEqual(sources);
	const targetUniform = allEqual(targets);
	const handleOptions = useMemo(() => listHandleIds(fixture), [fixture]);

	const patchEdges = useCallback(
		(updater: (e: BoardFixtureEdgeV1) => BoardFixtureEdgeV1) => {
			patchFixture((prev) => ({
				...prev,
				edges: prev.edges.map((e) => (idSet.has(e.id) ? updater(e) : e)),
			}));
		},
		[idSet, patchFixture],
	);

	return (
		<div className="border-element/60 space-y-3 border-l pl-2">
			<Label id="board-play.inspector.edge.source" label="Source">
				<Select
					onValueChange={(v) => {
						patchEdges((e) => ({ ...e, source: v }));
					}}
					value={sourceUniform ? sources[0] : undefined}
				>
					<SelectTrigger className="h-7 font-mono text-xs">
						<SelectValue placeholder={sourceUniform ? undefined : "Mixed"} />
					</SelectTrigger>
					<SelectContent>
						{handleOptions.map((hid) => (
							<SelectItem key={hid} value={hid}>
								{hid}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</Label>
			<Label id="board-play.inspector.edge.target" label="Target">
				<Select
					onValueChange={(v) => {
						patchEdges((e) => ({ ...e, target: v }));
					}}
					value={targetUniform ? targets[0] : undefined}
				>
					<SelectTrigger className="h-7 font-mono text-xs">
						<SelectValue placeholder={targetUniform ? undefined : "Mixed"} />
					</SelectTrigger>
					<SelectContent>
						{handleOptions.map((hid) => (
							<SelectItem key={`target-${hid}`} value={hid}>
								{hid}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</Label>
			{edgeIds.length === 1 ? (
				<Label id="board-play.inspector.edge.id" label="Id">
					<Input
						className="h-7 font-mono text-xs"
						defaultValue={edgeIds[0]}
						key={edgeIds[0]}
						onBlur={(e) => {
							const nextId = e.currentTarget.value.trim();
							const oldId = edgeIds[0];
							if (!oldId || !nextId || nextId === oldId) {
								return;
							}
							patchFixture((prev) => ({
								...prev,
								edges: prev.edges.map((edge) => (edge.id === oldId ? { ...edge, id: nextId } : edge)),
							}));
							remapIdInSelections(oldId, nextId);
						}}
					/>
				</Label>
			) : null}
		</div>
	);
}

/** @emoji 🔎 Sketchpad-style tree inspector with batch edits for the active pane selection. */
function BoardSelectionInspectorPanel(): ReactElement {
	const { activePaneId, fixture, patchFixture, remapIdInSelections, selectionByPane } = useBoardPlayShell();
	const ids = useMemo(() => [...selectionByPane[activePaneId]].sort((a, b) => a.localeCompare(b)), [activePaneId, selectionByPane]);

	const { edgeIds, handleIds, nodeIds } = useMemo(() => {
		const nodeIds: string[] = [];
		const handleIds: string[] = [];
		const edgeIds: string[] = [];
		for (const id of ids) {
			if (findNode(fixture, id)) {
				nodeIds.push(id);
			} else if (findEdge(fixture, id)) {
				edgeIds.push(id);
			} else if (findHandleOwner(fixture, id)) {
				handleIds.push(id);
			}
		}
		return { edgeIds, handleIds, nodeIds };
	}, [fixture, ids]);

	const treeSections = useMemo<TreeDataSection[]>(() => {
		if (ids.length === 0) {
			return [
				{
					content: <p className="text-muted-foreground px-1 py-2 text-xs">No selection. Click the graph or pick another tab.</p>,
					id: "board-play-inspector-empty",
					label: null,
				},
			];
		}
		const sections: TreeDataSection[] = [];
		if (nodeIds.length > 0) {
			sections.push({
				content: <InspectorNodeBatch fixture={fixture} nodeIds={nodeIds} patchFixture={patchFixture} remapIdInSelections={remapIdInSelections} />,
				defaultOpen: true,
				id: "board-play-inspector-nodes",
				label: `Nodes (${nodeIds.length})`,
			});
		}
		if (handleIds.length > 0) {
			sections.push({
				content: <InspectorHandleBatch fixture={fixture} handleIds={handleIds} patchFixture={patchFixture} remapIdInSelections={remapIdInSelections} />,
				defaultOpen: true,
				id: "board-play-inspector-handles",
				label: `Handles (${handleIds.length})`,
			});
		}
		if (edgeIds.length > 0) {
			sections.push({
				content: <InspectorEdgeBatch edgeIds={edgeIds} fixture={fixture} patchFixture={patchFixture} remapIdInSelections={remapIdInSelections} />,
				defaultOpen: true,
				id: "board-play-inspector-edges",
				label: `Edges (${edgeIds.length})`,
			});
		}
		if (sections.length === 0) {
			sections.push({
				content: (
					<div className="px-1 py-2 font-mono text-xs" style={{ color: "var(--warning-foreground)" }}>
						Unknown ids: {ids.join(", ")}
					</div>
				),
				id: "board-play-inspector-unknown",
				label: "Selection",
			});
		}
		return sections;
	}, [edgeIds, fixture, handleIds, ids, nodeIds, patchFixture, remapIdInSelections]);

	return (
		<div className="flex h-full min-h-0 flex-col gap-2 p-3 text-xs">
			<div className="text-muted-foreground flex shrink-0 items-center gap-2 border-b border-element pb-2">
				<ClipboardList className="size-4 shrink-0" />
				<div>
					<div className="font-semibold uppercase tracking-wide">Detail</div>
					<div className="text-[11px] opacity-80">pane: {activePaneId}</div>
				</div>
			</div>
			<div className="min-h-0 flex-1 overflow-hidden">
				<TreeStateProvider>
					<Tree className="min-h-0 min-w-0 flex-1 overflow-y-auto overflow-x-hidden" sections={treeSections} />
				</TreeStateProvider>
			</div>
		</div>
	);
}
// #endregion 🔖SidePanels

// #region 🔖Layout
const boardPlayLayout: UIWindowLayout = {
	root: {
		kind: "row",
		children: [
			{
				kind: "stack",
				size: 50,
				children: [createWindowLayout("board-overview", "Overview")],
			},
			{
				kind: "column",
				size: 50,
				children: [
					{ kind: "stack", size: 50, children: [createWindowLayout("board-detail", "Zoom")] },
					{ kind: "stack", size: 50, children: [createWindowLayout("board-selection", "Selection")] },
				],
			},
		],
	},
};

// #endregion 🔖Layout

// #region 🔖Surface
function BoardPlaySurfaceFooter(props: {
	theme: ElementsSurfaceTheme;
	device: ElementsSurfaceDevice;
	expertise: Expertise;
	onTheme: (v: ElementsSurfaceTheme) => void;
	onDevice: (v: ElementsSurfaceDevice) => void;
	onExpertise: (v: Expertise) => void;
}): ReactElement {
	const { theme, device, expertise, onDevice, onExpertise, onTheme } = props;
	return (
		<div className="flex min-w-0 flex-wrap items-center gap-double px-single py-tiny">
			<span className="shrink-0 text-xs text-muted-foreground">Theme</span>
			<Select onValueChange={(v) => onTheme(v as ElementsSurfaceTheme)} value={theme}>
				<SelectTrigger className="h-medium w-[7.5rem]" id="board-play-surface-theme" size="sm">
					<SelectValue />
				</SelectTrigger>
				<SelectContent>
					<SelectItem value="system">System</SelectItem>
					<SelectItem value="light">Light</SelectItem>
					<SelectItem value="dark">Dark</SelectItem>
				</SelectContent>
			</Select>
			<span className="shrink-0 text-xs text-muted-foreground">Device</span>
			<Select onValueChange={(v) => onDevice(v as ElementsSurfaceDevice)} value={device}>
				<SelectTrigger className="h-medium w-[7.5rem]" id="board-play-surface-device" size="sm">
					<SelectValue />
				</SelectTrigger>
				<SelectContent>
					<SelectItem value="desktop">Desktop</SelectItem>
					<SelectItem value="tablet">Tablet</SelectItem>
					<SelectItem value="mobile">Mobile</SelectItem>
				</SelectContent>
			</Select>
			<span className="shrink-0 text-xs text-muted-foreground">Expertise</span>
			<Select onValueChange={(v) => onExpertise(v as Expertise)} value={expertise}>
				<SelectTrigger className="h-medium w-[7.5rem]" id="board-play-surface-expertise" size="sm">
					<SelectValue />
				</SelectTrigger>
				<SelectContent>
					<SelectItem value={Expertise.BEGINNER}>Beginner</SelectItem>
					<SelectItem value={Expertise.NORMAL}>Normal</SelectItem>
					<SelectItem value={Expertise.EXPERT}>Expert</SelectItem>
				</SelectContent>
			</Select>
		</div>
	);
}
// #endregion 🔖Surface

// #region 🔖Entrypoint
const initialFixture = parseBoardFixtureV1(nakaginFixtureJson as unknown) ?? (nakaginFixtureJson as BoardFixtureV1);

function BoardPlayInner(): ReactElement {
	const [fixture, setFixtureState] = useState<BoardFixtureV1>(initialFixture);
	const fixtureRef = useRef<BoardFixtureV1>(fixture);
	fixtureRef.current = fixture;
	const [activePaneId, setActivePaneId] = useState<BoardPlayPaneId>("board-overview");
	const [selectionByPane, setSelectionByPane] = useState<Record<BoardPlayPaneId, Set<string>>>(() => selectionSeedForFixture(initialFixture));
	const [theme, setTheme] = useState<ElementsSurfaceTheme>(readTheme);
	const [device, setDevice] = useState<ElementsSurfaceDevice>(readDevice);
	const [expertise, setExpertise] = useState<Expertise>(readExpertise);
	const { mobile } = useElementsSurfaceChrome({ theme, device, expertise });
	const [boardSelectionMethod, setBoardSelectionMethod] = useState<BoardSelectionMethod>("rectangle");
	const [boardSelectionMode, setBoardSelectionMode] = useState<BoardSelectionMode>("invertive");
	const [boardSelectionTargets, setBoardSelectionTargets] = useState<BoardSelectionTargets>(() => ({ ...BOARD_SELECTION_TARGETS_DEFAULT }));
	const [boardRedrawPlaying, setBoardRedrawPlaying] = useState(false);
	const [forceLayoutFullIterations, setForceLayoutFullIterations] = useState(200);
	const [forceLayoutIdealEdgeLength, setForceLayoutIdealEdgeLength] = useState(64);
	const [forceLayoutGravity, setForceLayoutGravity] = useState(0.012);
	const [forceLayoutRepulsionStrength, setForceLayoutRepulsionStrength] = useState(480);
	const [boardRedrawTickMs, setBoardRedrawTickMs] = useState(96);
	const [boardRedrawTickIterations, setBoardRedrawTickIterations] = useState(10);
	const [boardRedrawMode, setBoardRedrawMode] = useState<BoardRedrawModeKind>("force-graph");
	const [treeLayoutLayerSpacing, setTreeLayoutLayerSpacing] = useState(120);
	const [treeLayoutSiblingGap, setTreeLayoutSiblingGap] = useState(28);
	const [treeLayoutDirection, setTreeLayoutDirection] = useState<BoardHierarchicalTreeDirectionKind>("downwards");
	const [windowOptionDemo, setWindowOptionDemo] = useState({
		ovToggle: true,
		ovSelect: "fit",
		ovCombo: "alpha",
		ovCycle: "a",
		ovInput: "sample",
		ovText: "notes",
		ovCheck: true,
		ovRadio: "one",
		ovSlider: 40,
		ovNumber: 3,
		ovColor: "#3366cc",
		detailSlider: 50,
		selMode: "nodes",
	});

	const boardWindowKinds = useMemo<UIWindowKindDefinition[]>(
		() => [
			{
				component: BoardOverviewPane,
				contextMenu: boardPlayOverviewWindowContextMenu,
				id: "board-overview",
				label: "Overview",
				options: [
					{ id: "board-ov-sec", kind: "section", title: "Window options" },
					{ id: "board-ov-sep0", kind: "separator" },
					{
						icon: <Square className="size-small" />,
						id: "board-ov-toggle",
						kind: "toggle",
						label: "Preview",
						pressed: windowOptionDemo.ovToggle,
						onPressedChange: (pressed) => setWindowOptionDemo((p) => ({ ...p, ovToggle: pressed })),
					},
					{
						id: "board-ov-select",
						items: [
							{ id: "fit", label: "Fit", value: "fit" },
							{ id: "fill", label: "Fill", value: "fill" },
							{ id: "1x", label: "1×", value: "1x" },
						],
						kind: "select",
						label: "Scale",
						onValueChange: (value) => setWindowOptionDemo((p) => ({ ...p, ovSelect: value })),
						value: windowOptionDemo.ovSelect,
					},
					{
						id: "board-ov-combo",
						kind: "combobox",
						label: "Search preset",
						onValueChange: (value) => setWindowOptionDemo((p) => ({ ...p, ovCombo: value })),
						options: [
							{ label: "Alpha", value: "alpha" },
							{ label: "Beta", value: "beta" },
							{ label: "Gamma", value: "gamma" },
						],
						placeholder: "Pick…",
						value: windowOptionDemo.ovCombo,
					},
					{
						id: "board-ov-cycle",
						items: [
							{ label: "A", value: "a" },
							{ label: "B", value: "b" },
							{ label: "C", value: "c" },
						],
						kind: "buttonCycle",
						label: "Cycle",
						onValueChange: (value) => setWindowOptionDemo((p) => ({ ...p, ovCycle: value })),
						value: windowOptionDemo.ovCycle,
					},
					{ id: "board-ov-btn", kind: "button", label: "Action", onClick: () => undefined, text: "Ping" },
					{
						id: "board-ov-input",
						kind: "input",
						label: "Tag",
						onLazyChange: (value) => setWindowOptionDemo((p) => ({ ...p, ovInput: value })),
						placeholder: "id…",
						value: windowOptionDemo.ovInput,
					},
					{
						id: "board-ov-textarea",
						kind: "textarea",
						label: "Memo",
						onLazyChange: (value) => setWindowOptionDemo((p) => ({ ...p, ovText: value })),
						rows: 3,
						value: windowOptionDemo.ovText,
					},
					{
						checked: windowOptionDemo.ovCheck,
						id: "board-ov-check",
						kind: "checkbox",
						label: "Snap",
						onCheckedChange: (checked) => setWindowOptionDemo((p) => ({ ...p, ovCheck: checked })),
					},
					{
						id: "board-ov-radio",
						items: [
							{ label: "One", value: "one" },
							{ label: "Two", value: "two" },
						],
						kind: "radio",
						label: "Band",
						onChange: (value) => setWindowOptionDemo((p) => ({ ...p, ovRadio: value })),
						value: windowOptionDemo.ovRadio,
					},
					{
						id: "board-ov-slider",
						kind: "slider",
						label: "Opacity",
						max: 100,
						min: 0,
						onValueChange: (value) => setWindowOptionDemo((p) => ({ ...p, ovSlider: value })),
						value: windowOptionDemo.ovSlider,
					},
					{
						id: "board-ov-number",
						kind: "number",
						label: "Copies",
						max: 9,
						min: 0,
						onChange: (value) => setWindowOptionDemo((p) => ({ ...p, ovNumber: value })),
						value: windowOptionDemo.ovNumber,
					},
					{
						id: "board-ov-color",
						kind: "color",
						label: "Accent",
						onChange: (value) => setWindowOptionDemo((p) => ({ ...p, ovColor: value })),
						value: windowOptionDemo.ovColor,
					},
				] satisfies UIWindowOption[],
			},
			{
				component: BoardDetailPane,
				id: "board-detail",
				label: "Zoom",
				options: [
					{ id: "board-d-sec", kind: "section", title: "Zoom" },
					{
						id: "board-d-slider",
						kind: "slider",
						label: "Focus",
						max: 100,
						min: 0,
						onValueChange: (value) => setWindowOptionDemo((p) => ({ ...p, detailSlider: value })),
						value: windowOptionDemo.detailSlider,
					},
				] satisfies UIWindowOption[],
			},
			{
				component: BoardSelectionPane,
				id: "board-selection",
				label: "Selection",
				options: [
					{
						id: "board-s-sel",
						items: [
							{ id: "n", label: "Nodes", value: "nodes" },
							{ id: "e", label: "Edges", value: "edges" },
							{ id: "h", label: "Handles", value: "handles" },
						],
						kind: "select",
						label: "Target",
						onValueChange: (value) => setWindowOptionDemo((p) => ({ ...p, selMode: value })),
						value: windowOptionDemo.selMode,
					},
				] satisfies UIWindowOption[],
			},
		],
		[windowOptionDemo],
	);

	useEffect(() => {
		try {
			localStorage.setItem(LS_THEME, theme);
		} catch {
			/* ignore */
		}
	}, [theme]);

	useEffect(() => {
		try {
			localStorage.setItem(LS_DEVICE, device);
		} catch {
			/* ignore */
		}
	}, [device]);

	useEffect(() => {
		try {
			localStorage.setItem(LS_EXPERTISE, expertise);
		} catch {
			/* ignore */
		}
	}, [expertise]);

	const surfaceFooterItems = useMemo<FooterItem[]>(
		() => [
			{
				content: (
					<BoardPlaySurfaceFooter
						device={device}
						expertise={expertise}
						onDevice={setDevice}
						onExpertise={setExpertise}
						onTheme={setTheme}
						theme={theme}
					/>
				),
				id: "board-play-surface",
				order: 0,
			},
		],
		[device, expertise, theme],
	);

	const applyStructuralDelete = useCallback((kind: "edge" | "node", id: string) => {
		const pruneSelections = (removeIds: readonly string[]): void => {
			const remove = new Set(removeIds);
			setSelectionByPane((selPrev) => {
				const paneIds: BoardPlayPaneId[] = ["board-overview", "board-detail", "board-selection"];
				const next: Record<BoardPlayPaneId, Set<string>> = { ...selPrev };
				for (const pane of paneIds) {
					next[pane] = new Set([...selPrev[pane]].filter((x) => !remove.has(x)));
				}
				return next;
			});
		};
		if (kind === "edge") {
			setFixtureState((prev) => {
				if (!prev.edges.some((e) => e.id === id)) {
					return prev;
				}
				return { ...prev, edges: prev.edges.filter((e) => e.id !== id) };
			});
			pruneSelections([id]);
			return;
		}
		const node = fixtureRef.current.nodes.find((n) => n.id === id);
		const handleIds = node?.handles.map((h) => h.id) ?? [];
		setFixtureState((prev) => {
			const n = prev.nodes.find((x) => x.id === id);
			if (!n) {
				return prev;
			}
			const hset = new Set(n.handles.map((h) => h.id));
			return {
				...prev,
				edges: prev.edges.filter((e) => !hset.has(e.source) && !hset.has(e.target)),
				nodes: prev.nodes.filter((x) => x.id !== id),
			};
		});
		pruneSelections([id, ...handleIds]);
	}, []);

	const setFixture = useCallback((next: BoardFixtureV1) => {
		setFixtureState(next);
		setSelectionByPane(selectionSeedForFixture(next));
	}, []);

	const patchFixture = useCallback((updater: (prev: BoardFixtureV1) => BoardFixtureV1) => {
		setFixtureState((prev) => updater(prev));
	}, []);

	const setSelectionForPane = useCallback((pane: BoardPlayPaneId, ids: readonly string[]) => {
		setSelectionByPane((prev) => ({ ...prev, [pane]: new Set(ids) }));
	}, []);

	const handleCanvasFixtureDrop = useCallback((pane: BoardPlayPaneId, detail: BoardFixtureDropDetail) => {
		const merged = mergePaletteNodeFromDrop(detail);
		if (merged) {
			patchFixture((prev) => ({ ...prev, nodes: [...prev.nodes, merged] }));
			setSelectionForPane(pane, [merged.id]);
			return;
		}
		setFixture(detail.fixture);
	}, [patchFixture, setFixture, setSelectionForPane]);

	const remapIdInSelections = useCallback((replacedId: string, replacementId: string) => {
		if (replacedId === replacementId) {
			return;
		}
		const panes: BoardPlayPaneId[] = ["board-overview", "board-detail", "board-selection"];
		setSelectionByPane((prev) => {
			const next: Record<BoardPlayPaneId, Set<string>> = { ...prev };
			for (const p of panes) {
				next[p] = new Set([...prev[p]].map((id) => (id === replacedId ? replacementId : id)));
			}
			return next;
		});
	}, []);

	const cameraBasisFixtureRef = useRef<BoardFixtureV1>(fixture);
	const [cameraBasisTick, setCameraBasisTick] = useState(0);
	const prevBoardRedrawPlayingRef = useRef(false);

	useEffect(() => {
		if (!boardRedrawPlaying) {
			cameraBasisFixtureRef.current = fixture;
			setCameraBasisTick((t) => t + 1);
		}
	}, [fixture, boardRedrawPlaying]);

	useEffect(() => {
		if (boardRedrawPlaying && !prevBoardRedrawPlayingRef.current) {
			cameraBasisFixtureRef.current = fixture;
			setCameraBasisTick((t) => t + 1);
		}
		prevBoardRedrawPlayingRef.current = boardRedrawPlaying;
	}, [boardRedrawPlaying, fixture]);

	const camerasByPane = useMemo(() => triptychCamerasFromFixture(cameraBasisFixtureRef.current), [cameraBasisTick]);

	const applyBoardRedrawOnce = useCallback(() => {
		const full = Math.max(1, Math.min(5000, Math.round(forceLayoutFullIterations)));
		patchFixture((prev) =>
			layoutBoardFixtureRedraw(
				prev,
				boardPlayRedrawLayoutOpts(
					activePaneId,
					camerasByPane,
					boardRedrawMode,
					full,
					forceLayoutIdealEdgeLength,
					forceLayoutGravity,
					forceLayoutRepulsionStrength,
					treeLayoutLayerSpacing,
					treeLayoutSiblingGap,
					treeLayoutDirection,
				),
			),
		);
	}, [
		activePaneId,
		boardRedrawMode,
		camerasByPane,
		forceLayoutFullIterations,
		forceLayoutGravity,
		forceLayoutIdealEdgeLength,
		forceLayoutRepulsionStrength,
		patchFixture,
		treeLayoutLayerSpacing,
		treeLayoutDirection,
		treeLayoutSiblingGap,
	]);

	useEffect(() => {
		if (!boardRedrawPlaying) {
			return;
		}
		const tickMs = Math.max(33, Math.min(5000, Math.round(boardRedrawTickMs)));
		const tickIters = Math.max(1, Math.min(500, Math.round(boardRedrawTickIterations)));
		const id = window.setInterval(() => {
			patchFixture((prev) => {
				if (prev.nodes.length === 0) {
					return prev;
				}
				return layoutBoardFixtureRedraw(
					prev,
					boardPlayRedrawLayoutOpts(
						activePaneId,
						camerasByPane,
						boardRedrawMode,
						tickIters,
						forceLayoutIdealEdgeLength,
						forceLayoutGravity,
						forceLayoutRepulsionStrength,
						treeLayoutLayerSpacing,
						treeLayoutSiblingGap,
						treeLayoutDirection,
					),
				);
			});
		}, tickMs);
		return () => window.clearInterval(id);
	}, [
		activePaneId,
		boardRedrawMode,
		camerasByPane,
		forceLayoutGravity,
		forceLayoutIdealEdgeLength,
		boardRedrawPlaying,
		forceLayoutRepulsionStrength,
		boardRedrawTickIterations,
		boardRedrawTickMs,
		patchFixture,
		treeLayoutLayerSpacing,
		treeLayoutDirection,
		treeLayoutSiblingGap,
	]);

	const shellValue = useMemo<BoardPlayShellValue>(
		() => ({
			activePaneId,
			applyBoardRedrawOnce,
			applyStructuralDelete,
			boardRedrawMode,
			boardRedrawPlaying,
			boardRedrawTickIterations,
			boardRedrawTickMs,
			boardSelectionMethod,
			boardSelectionMode,
			boardSelectionTargets,
			camerasByPane,
			fixture,
			forceLayoutFullIterations,
			forceLayoutGravity,
			forceLayoutIdealEdgeLength,
			forceLayoutRepulsionStrength,
			handleCanvasFixtureDrop,
			patchFixture,
			remapIdInSelections,
			setActivePaneId,
			setBoardRedrawMode,
			setBoardRedrawPlaying,
			setBoardRedrawTickIterations,
			setBoardRedrawTickMs,
			setBoardSelectionMethod,
			setBoardSelectionMode,
			setBoardSelectionTargets,
			setFixture,
			setForceLayoutFullIterations,
			setForceLayoutGravity,
			setForceLayoutIdealEdgeLength,
			setForceLayoutRepulsionStrength,
			setTreeLayoutLayerSpacing,
			setTreeLayoutDirection,
			setTreeLayoutSiblingGap,
			selectionByPane,
			setSelectionForPane,
			treeLayoutLayerSpacing,
			treeLayoutDirection,
			treeLayoutSiblingGap,
		}),
		[
			activePaneId,
			applyBoardRedrawOnce,
			applyStructuralDelete,
			boardRedrawMode,
			boardRedrawPlaying,
			boardRedrawTickIterations,
			boardRedrawTickMs,
			boardSelectionMethod,
			boardSelectionMode,
			boardSelectionTargets,
			camerasByPane,
			fixture,
			forceLayoutFullIterations,
			forceLayoutGravity,
			forceLayoutIdealEdgeLength,
			forceLayoutRepulsionStrength,
			handleCanvasFixtureDrop,
			patchFixture,
			remapIdInSelections,
			selectionByPane,
			setSelectionForPane,
			treeLayoutLayerSpacing,
			treeLayoutDirection,
			treeLayoutSiblingGap,
		],
	);

	const boardPlayApp: UIAppConfig = useMemo(
		() => ({
			defaultLayout: boardPlayLayout,
			id: BOARD_PLAY_APP_ID,
			label: "Board",
			leftPanelTabs: [
				{ content: () => <BoardFixtureLibraryPanel />, icon: Library, id: "board-play-library", order: 0 },
			],
			onActiveWindowChange: (windowKindId) => {
				if (windowKindId === "board-overview" || windowKindId === "board-detail" || windowKindId === "board-selection") {
					setActivePaneId(windowKindId);
				}
			},
			rightPanelTabs: [
				{ content: () => <BoardSelectionInspectorPanel />, icon: ClipboardList, id: "board-play-inspector", order: 0 },
				{ content: () => <BoardPlaySettingsPanel />, icon: Settings, id: "board-play-settings", order: 1 },
			],
			toolbarContent: <BoardPlayToolbar />,
			windowKinds: boardWindowKinds,
		}),
		[setActivePaneId],
	);

	return (
		<BoardPlayShellContext.Provider value={shellValue}>
			<UI
				apps={[boardPlayApp]}
				defaultAppId={BOARD_PLAY_APP_ID}
				footerItems={surfaceFooterItems}
				initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }}
				mobile={mobile}
			/>
		</BoardPlayShellContext.Provider>
	);
}

function BoardPlayApp(): ReactElement {
	return (
		<LevelProvider level="window">
			<div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>
				<BoardPlayInner />
			</div>
		</LevelProvider>
	);
}

type BoardPlayDomRoot = HTMLElement & { __boardPlayReactRoot?: Root };

const mount = document.getElementById("root") as BoardPlayDomRoot | null;
if (!mount) {
	throw new Error("Board play root #root missing.");
}

mount.__boardPlayReactRoot ??= createRoot(mount);
mount.__boardPlayReactRoot.render(<BoardPlayApp />);
// #endregion 🔖Entrypoint
