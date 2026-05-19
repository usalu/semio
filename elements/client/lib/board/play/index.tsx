// #region 🧲Header
// 💻 elements/client/lib/board/play/index.tsx — Board play: triptych Nakagin views, in-app fixture drag shelf, selection inspector, `UI` shell (same `@elements/ui` + globals pattern as semio rendering / algorithms).
// #endregion 🧲Header

// #region 📥Imports
import {
	Button,
	Expertise,
	IconSelector,
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
	type TreeDataItem,
	type TreeDataSection,
	type TreeHeaderAction,
	type ContextMenuItem,
	type UIAppConfig,
	type UIWindowKindDefinition,
	type UIWindowLayout,
} from "@elements/ui";
import {
	BoxSelect,
	Circle,
	ClipboardList,
	Eye,
	EyeOff,
	FolderTree,
	Lasso,
	Library,
	Link2,
	Lock,
	Magnet,
	Minus,
	MousePointer2,
	Plus,
	Repeat2,
	Settings,
	Square,
	Layers,
	Trash2,
	Unlock,
	ZoomIn,
} from "lucide-react";
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
	BOARD_DEFAULT_KIND_CATALOG_BUNDLE,
	BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE,
	BOARD_FIXTURE_DRAG_V1_MIME,
	BOARD_SELECTION_TARGETS_DEFAULT,
	boardFixtureMetaKindCatalogBundle,
	encodeBoardFixtureForDragV1,
	layoutBoardFixtureRedrawHandles,
	layoutBoardFixtureRedrawNodes,
	mergeBoardKindCatalogBundleByRowId,
	parseBoardFixtureV1,
	type BoardEdgeKindCatalogEntry,
	type BoardFixtureDropDetail,
	type BoardFixtureCircleNodeV1,
	type BoardFixtureEdgeV1,
	type BoardFixtureHandleV1,
	type BoardFixtureNodeV1,
	type BoardFixtureRectangleNodeV1,
	type BoardFixtureV1,
	type BoardHierarchicalTreeDirectionKind,
	type BoardKindCatalogBundle,
	type BoardKindCompatEntry,
	type BoardRedrawLayoutOptions,
	type BoardRedrawModeKind,
	type BoardForceGraphLayoutOptions,
	type BoardNodeKindCatalogEntry,
	type BoardSelectionMethod,
	type BoardSelectionMode,
	type BoardSelectionSnapshot,
	type BoardSelectionTargets,
	type BoardDrawLodKind,
	type BoardWireKindCatalogEntry,
	type CameraState,
	DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
	type BoardLodZoomThresholds,
	classifyElementsBoardIconSelectorMode,
} from "../index";
// @ts-ignore Vite resolves the board host entrypoint for the play harness.
import { BoardCanvas, Edge, Handle, Node, Wire, useBoardEvent } from "../index.tsx";
import "./globals.css";
// #endregion 📥Imports

// #region 🔖Kinds
export type BoardPlayPaneId = "board-overview" | "board-detail" | "board-selection";

interface BoardPlayWireRecord {
	endX?: number;
	endY?: number;
	hidden?: boolean;
	id: string;
	source: string;
	target?: string;
	wireKind?: string;
}

interface BoardPlayDocument {
	fixture: BoardFixtureV1;
	kindCatalogs: BoardKindCatalogBundle;
	kindCompatibility: BoardKindCompatEntry[];
	lockedIds: string[];
	wires: BoardPlayWireRecord[];
}

type BoardWorkbenchSelection =
	| { id: string; kind: "constraint" }
	| { id: string; kind: "edge-kind" }
	| { id: string; kind: "node-kind" }
	| { id: string; kind: "wire-kind" };

const BOARD_PLAY_APP_ID = "elements-board-play";

const LS_THEME = "elements.board-play.surface.theme";
const LS_DEVICE = "elements.board-play.surface.device";
const LS_EXPERTISE = "elements.board-play.surface.expertise";

function isRecord(value: unknown): value is Record<string, unknown> {
	return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function uniqueSortedStrings(values: readonly string[]): string[] {
	return [...new Set(values.filter((value) => value.trim() !== ""))].sort((left, right) => left.localeCompare(right));
}

function parseBoardPlayWireRecord(raw: unknown): BoardPlayWireRecord | null {
	if (!isRecord(raw)) {
		return null;
	}
	const id = typeof raw.id === "string" ? raw.id.trim() : "";
	const source = typeof raw.source === "string" ? raw.source.trim() : "";
	const target = typeof raw.target === "string" && raw.target.trim() !== "" ? raw.target.trim() : undefined;
	const wireKind = typeof raw.wireKind === "string" && raw.wireKind.trim() !== "" ? raw.wireKind.trim() : undefined;
	const endX = Number(raw.endX);
	const endY = Number(raw.endY);
	if (id === "" || source === "") {
		return null;
	}
	return {
		...(Number.isFinite(endX) ? { endX } : {}),
		...(Number.isFinite(endY) ? { endY } : {}),
		...(raw.hidden === true ? { hidden: true } : {}),
		id,
		source,
		...(target ? { target } : {}),
		...(wireKind ? { wireKind } : {}),
	};
}

function parseBoardPlayKindCompatibility(raw: unknown): BoardKindCompatEntry[] {
	if (!Array.isArray(raw)) {
		return [];
	}
	const out: BoardKindCompatEntry[] = [];
	for (const entry of raw) {
		if (!isRecord(entry)) {
			continue;
		}
		const source = typeof entry.source === "string" ? entry.source.trim() : "";
		const target = typeof entry.target === "string" ? entry.target.trim() : "";
		const specificity =
			entry.specificity === "general" ||
			entry.specificity === "node" ||
			entry.specificity === "edge" ||
			entry.specificity === "handle" ||
			entry.specificity === "wire"
				? entry.specificity
				: undefined;
		if (source === "" || target === "") {
			continue;
		}
		out.push({
			...(entry.bidirectional === true ? { bidirectional: true } : {}),
			...(entry.important === true ? { important: true } : {}),
			...(specificity ? { specificity } : {}),
			source,
			target,
		});
	}
	return out;
}

function parseBoardPlayDocument(raw: unknown): BoardPlayDocument {
	const fixture = parseBoardFixtureV1(raw) ?? (raw as BoardFixtureV1);
	const root = isRecord(raw) ? raw : {};
	const meta = isRecord(root.meta) ? root.meta : fixture.meta ?? {};
	const wireSource = Array.isArray(root.wires) ? root.wires : Array.isArray(meta.boardPlayWires) ? meta.boardPlayWires : [];
	const lockedSource = Array.isArray(meta.boardPlayLockedIds) ? meta.boardPlayLockedIds : [];
	const kindCatalogs = mergeBoardKindCatalogBundleByRowId(
		{ ...BOARD_DEFAULT_KIND_CATALOG_BUNDLE },
		boardFixtureMetaKindCatalogBundle(raw) ?? {},
	);
	const kindCompatibility = parseBoardPlayKindCompatibility(meta.kindCompatibility);
	const wires = wireSource.map(parseBoardPlayWireRecord).filter((wire): wire is BoardPlayWireRecord => wire !== null);
	const lockedIds = uniqueSortedStrings(lockedSource.filter((value): value is string => typeof value === "string"));
	return { fixture, kindCatalogs, kindCompatibility, lockedIds, wires };
}

function buildBoardPlayFixturePayload(document: BoardPlayDocument): BoardFixtureV1 {
	return {
		...document.fixture,
		meta: {
			...(document.fixture.meta ?? {}),
			boardPlayLockedIds: document.lockedIds,
			boardPlayWires: document.wires,
			kindCatalogs: document.kindCatalogs,
			kindCompatibility: document.kindCompatibility,
		},
	};
}

function boardPlayKindLabel(catalog: readonly { id: string; label: string; name?: string }[] | undefined, id: string): string {
	const trimmed = id.trim();
	if (trimmed === "") {
		return "Unassigned";
	}
	for (const row of catalog ?? []) {
		if (row.id === trimmed) {
			return row.label || row.name || row.id;
		}
	}
	return trimmed;
}

function boardPlayConstraintLabel(kindCatalogs: BoardKindCatalogBundle, entry: BoardKindCompatEntry): string {
	const arrow = entry.bidirectional ? "--" : "->";
	const source = boardPlayKindLabel(kindCatalogs.handles, entry.source);
	const target = boardPlayKindLabel(kindCatalogs.handles, entry.target);
	const specificity = entry.specificity ?? "general";
	return `${source} ${arrow} ${target} · ${specificity}`;
}

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

/** @emoji 📷 Default cameras for all play panes: center on fixture bounds; zoom fits the graph’s longest axis into the reference short viewport (margin padding). */
function triptychCamerasFromFixture(fixture: BoardFixtureV1): Record<BoardPlayPaneId, CameraState> {
	const { cx, cy, halfSpan } = fixtureWorldBounds(fixture);
	const base = fixture.camera;
	const margin = 0.06;
	const usable = REF_VIEWPORT_SHORT_PX * (1 - 2 * margin);
	const worldSpan = Math.max(2 * halfSpan, 1);
	const zoom = clampZoom(usable / worldSpan);
	const cam: CameraState = { x: cx + base.x, y: cy + base.y, zoom };
	return {
		"board-detail": { ...cam },
		"board-overview": { ...cam },
		"board-selection": { ...cam },
	};
}

/** @emoji ⏱️ After redraw play stops: camera stays fixed for the first third of this span, then eases in the remaining two thirds to bbox fit (3s total). */
const BOARD_PLAY_CAMERA_POST_REDRAW_TOTAL_MS = 3000;

/** @emoji ⏱️ After one-shot “Redraw nodes”, shell cameras ease to bbox fit (first third hold, last two thirds smooth). */
const BOARD_PLAY_NODES_REDRAW_CAMERA_EASE_TOTAL_MS = 1800;

/** @emoji 📷 Linear blend toward bbox-fit cameras each fixture commit while redraw play is on (damped follow). */
const BOARD_PLAY_REDRAW_CAMERA_CHASE_BLEND = 0.22;

function easeInOutCubic01(t: number): number {
	const x = Math.min(1, Math.max(0, t));
	return x < 0.5 ? 4 * x * x * x : 1 - (-2 * x + 2) ** 3 / 2;
}

function lerpCameraState(a: CameraState, b: CameraState, tLinear: number): CameraState {
	const w = easeInOutCubic01(tLinear);
	const zoom =
		a.zoom > 1e-9 && b.zoom > 1e-9 ? a.zoom * (b.zoom / a.zoom) ** w : a.zoom + (b.zoom - a.zoom) * w;
	return {
		x: a.x + (b.x - a.x) * w,
		y: a.y + (b.y - a.y) * w,
		zoom: clampZoom(zoom),
	};
}

/** @emoji 🎯 Lerps only `activePane` between `from` and `to`; other panes keep shallow copies of `from`. */
function blendTriptychCamerasActivePaneOnly(
	from: Record<BoardPlayPaneId, CameraState>,
	to: Record<BoardPlayPaneId, CameraState>,
	tLinear: number,
	activePane: BoardPlayPaneId,
): Record<BoardPlayPaneId, CameraState> {
	const out: Record<BoardPlayPaneId, CameraState> = {
		"board-detail": { ...from["board-detail"] },
		"board-overview": { ...from["board-overview"] },
		"board-selection": { ...from["board-selection"] },
	};
	out[activePane] = lerpCameraState(from[activePane], to[activePane], tLinear);
	return out;
}

function dampCameraStateLinear(a: CameraState, b: CameraState, w: number): CameraState {
	const t = Math.min(1, Math.max(0, w));
	const zoom =
		a.zoom > 1e-9 && b.zoom > 1e-9 ? a.zoom * (b.zoom / a.zoom) ** t : a.zoom + (b.zoom - a.zoom) * t;
	return {
		x: a.x + (b.x - a.x) * t,
		y: a.y + (b.y - a.y) * t,
		zoom: clampZoom(zoom),
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
	wires: BoardPlayWireRecord[];
	kindCatalogs: BoardKindCatalogBundle;
	kindCompatibility: BoardKindCompatEntry[];
	lockedIds: Set<string>;
	setFixture: (next: BoardFixtureV1) => void;
	/** @emoji 🎯 Palette drags merge one node at the pointer; full fixtures replace the graph. */
	handleCanvasFixtureDrop: (pane: BoardPlayPaneId, detail: BoardFixtureDropDetail) => void;
	patchFixture: (updater: (prev: BoardFixtureV1) => BoardFixtureV1) => void;
	patchWires: (updater: (prev: BoardPlayWireRecord[]) => BoardPlayWireRecord[]) => void;
	patchKindCatalogs: (updater: (prev: BoardKindCatalogBundle) => BoardKindCatalogBundle) => void;
	setKindCompatibility: (value: BoardKindCompatEntry[] | ((prev: BoardKindCompatEntry[]) => BoardKindCompatEntry[])) => void;
	setLockedIds: (value: string[] | ((prev: string[]) => string[])) => void;
	activePaneId: BoardPlayPaneId;
	setActivePaneId: (id: BoardPlayPaneId) => void;
	selectionByPane: Record<BoardPlayPaneId, Set<string>>;
	setSelectionForPane: (pane: BoardPlayPaneId, ids: readonly string[]) => void;
	workbenchSelection: BoardWorkbenchSelection | null;
	setWorkbenchSelection: (value: BoardWorkbenchSelection | null) => void;
	focusGraphSelection: (ids: readonly string[]) => void;
	focusWorkbenchSelection: (value: BoardWorkbenchSelection) => void;
	setGraphObjectsHidden: (ids: readonly string[], hidden: boolean) => void;
	setGraphObjectsLocked: (ids: readonly string[], locked: boolean) => void;
	deleteGraphObjects: (ids: readonly string[]) => void;
	appendConstraint: () => void;
	appendKind: (kind: "edge-kind" | "node-kind" | "wire-kind") => void;
	deleteWorkbenchSelection: () => void;
	/** @emoji 🔁 Rewrites selection ids when an object id changes (`replacedId` → `replacementId`); unrelated to edge endpoint fields. */
	remapIdInSelections: (replacedId: string, replacementId: string) => void;
	camerasByPane: Record<BoardPlayPaneId, CameraState>;
	/** @emoji 📷 Writes the **active** pane’s imperative camera (wheel/pan) into that pane’s entry in {@link boardPlayPaneCamerasBaseline}; other panes unchanged. */
	syncBaselineFromViewportCamera: (cam: CameraState) => void;
	boardSelectionMethod: BoardSelectionMethod;
	setBoardSelectionMethod: (value: BoardSelectionMethod) => void;
	boardSelectionMode: BoardSelectionMode;
	setBoardSelectionMode: (value: BoardSelectionMode) => void;
	/** @emoji 🖱️ Transient toolbar highlight from modifier override on the active pane’s last `select` emission. */
	boardSelectionGestureHighlight: BoardSelectionMode | null;
	setBoardSelectionGestureHighlight: (value: BoardSelectionMode | null) => void;
	boardSelectionTargets: BoardSelectionTargets;
	setBoardSelectionTargets: (value: BoardSelectionTargets | ((prev: BoardSelectionTargets) => BoardSelectionTargets)) => void;
	boardGridSnapEnabled: boolean;
	setBoardGridSnapEnabled: (value: boolean) => void;
	/** @emoji 📶 Per-pane WASM automatic LOD (zoom-driven); when false, optional pinned tier or follow-zoom. */
	boardAutomaticLodByPane: Record<BoardPlayPaneId, boolean>;
	setBoardAutomaticLodForPane: (pane: BoardPlayPaneId, value: boolean) => void;
	/** @emoji 📶 When automatic LOD is off: undefined = follow camera zoom bands on the WASM host. */
	boardPinnedLodByPane: Record<BoardPlayPaneId, BoardDrawLodKind | undefined>;
	setBoardPinnedLodForPane: (pane: BoardPlayPaneId, value: BoardDrawLodKind | undefined) => void;
	/** @emoji 🗑️ Drops ids from the shared fixture after the canvas emits structural delete events. */
	applyStructuralDelete: (kind: "edge" | "node", id: string) => void;
	/** @emoji ⏯️ When true, play runs layout work on `requestAnimationFrame` (graph packs multiple WASM passes per ~14ms frame; tree one pass per frame). */
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
	boardRedrawPlayMaxItersPerFrame: number;
	setBoardRedrawPlayMaxItersPerFrame: (value: number) => void;
	boardRedrawProgressiveEnabled: boolean;
	setBoardRedrawProgressiveEnabled: (value: boolean) => void;
	boardRedrawProgressiveAutoStopMs: number;
	setBoardRedrawProgressiveAutoStopMs: (value: number) => void;
	/** @emoji 🔁 Restarts progressive iteration ramp and auto-stop clock (used when the user drags a node during play). */
	resetBoardRedrawProgressiveEpoch: () => void;
	treeLayoutLayerSpacing: number;
	setTreeLayoutLayerSpacing: (value: number) => void;
	treeLayoutSiblingGap: number;
	setTreeLayoutSiblingGap: (value: number) => void;
	treeLayoutDirection: BoardHierarchicalTreeDirectionKind;
	setTreeLayoutDirection: (value: BoardHierarchicalTreeDirectionKind) => void;
	applyBoardRedrawOnce: (modeOverride?: BoardRedrawModeKind) => void;
	applyBoardRedrawHandlesOnce: () => void;
	boardRedrawHandlesAfterNodes: boolean;
	setBoardRedrawHandlesAfterNodes: (value: boolean) => void;
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

const BOARD_PLAYRedraw_FRAME_BUDGET_MS = 14;

/** @emoji 📈 Force-graph play: iteration budget per inner WASM call ramps from 2 up to `playMax` over `autoStopMs` (or ~3.8s when stop is off). */
function boardPlayProgressiveForceIters(elapsedMs: number, autoStopMs: number, playMax: number): number {
	const cap = Math.max(4, Math.min(500, Math.round(playMax)));
	const rampWindow = autoStopMs > 0 ? autoStopMs * 0.88 : 3800;
	const t = Math.min(1, elapsedMs / Math.max(100, rampWindow));
	return Math.max(2, Math.round(2 + t * (cap - 2)));
}

/** @emoji 🔒 Node ids in `lockedIds` that refer to graph nodes on the fixture (for WASM redraw pin list). */
function boardPlayLockedGraphNodeIds(fixture: BoardFixtureV1, lockedIds: ReadonlySet<string>): string[] {
	const known = new Set(fixture.nodes.map((n) => n.id));
	return [...lockedIds].filter((id) => known.has(id));
}

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
	redrawHandlesAfter: boolean,
	lockedNodeIds: readonly string[],
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
			lockedNodeIds: [...lockedNodeIds],
			mode: "hierarchical-tree",
			redrawHandlesAfter,
		};
	}
	const fg: BoardForceGraphLayoutOptions = {
		centerX: cx,
		centerY: cy,
		gravity: Math.max(0, forceGravity),
		idealEdgeLength: Math.max(8, forceIdealEdge),
		iterations: Math.max(1, Math.min(5000, Math.round(forceIters))),
		lockedNodeIds: [...lockedNodeIds],
		repulsionStrength: Math.max(40, Math.min(120, Math.round(forceRepulsion))),
	};
	return { centerX: cx, centerY: cy, forceGraph: fg, lockedNodeIds: [...lockedNodeIds], mode: "force-graph", redrawHandlesAfter };
}

/** @emoji 🧰 Sketchpad-style tools: marquee kind, merge mode, hit target, and circle or rectangle authoring at the active pane camera. */
function BoardPlayToolbar(): ReactElement {
	const {
		activePaneId,
		applyBoardRedrawHandlesOnce,
		applyBoardRedrawOnce,
		boardGridSnapEnabled,
		boardSelectionMethod,
		boardSelectionGestureHighlight,
		boardSelectionMode,
		boardSelectionTargets,
		camerasByPane,
		patchFixture,
		setBoardGridSnapEnabled,
		setBoardSelectionGestureHighlight,
		setBoardSelectionMethod,
		setBoardSelectionMode,
		setBoardSelectionTargets,
		setSelectionForPane,
	} = useBoardPlayShell();

	const camera = camerasByPane[activePaneId];
	const mergeToggleActive = boardSelectionGestureHighlight ?? boardSelectionMode;

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
			<ToolbarZone className="pointer-events-auto max-w-full flex-wrap justify-center gap-(--toolbar-gap) px-2">
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
						<button
							type="button"
							className={boardToolbarToggleClass(mergeToggleActive === "replace")}
							title="Replace selection (default; Shift additive, Ctrl subtractive, Ctrl+Shift invertive)"
							onClick={() => {
								setBoardSelectionGestureHighlight(null);
								setBoardSelectionMode("replace");
							}}
						>
							<MousePointer2 className="size-4" aria-hidden />
						</button>
					</ToolbarItem>
					<ToolbarItem>
						<button
							type="button"
							className={boardToolbarToggleClass(mergeToggleActive === "additive")}
							title="Additive"
							onClick={() => {
								setBoardSelectionGestureHighlight(null);
								setBoardSelectionMode("additive");
							}}
						>
							<Plus className="size-4" aria-hidden />
						</button>
					</ToolbarItem>
					<ToolbarItem>
						<button
							type="button"
							className={boardToolbarToggleClass(mergeToggleActive === "subtractive")}
							title="Subtractive"
							onClick={() => {
								setBoardSelectionGestureHighlight(null);
								setBoardSelectionMode("subtractive");
							}}
						>
							<Minus className="size-4" aria-hidden />
						</button>
					</ToolbarItem>
					<ToolbarItem>
						<button
							type="button"
							className={boardToolbarToggleClass(mergeToggleActive === "invertive")}
							title="Invertive"
							onClick={() => {
								setBoardSelectionGestureHighlight(null);
								setBoardSelectionMode("invertive");
							}}
						>
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
						<span className="text-muted-foreground pr-1 text-[10px] font-semibold uppercase tracking-wide">Grid</span>
					</ToolbarItem>
					<ToolbarItem>
						<button
							type="button"
							className={boardToolbarToggleClass(boardGridSnapEnabled)}
							title="Snap node drags to the finest visible LOD grid"
							onClick={() => setBoardGridSnapEnabled(!boardGridSnapEnabled)}
						>
							<Magnet className="size-4" aria-hidden />
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
						<span className="text-muted-foreground pr-1 text-[10px] font-semibold uppercase tracking-wide">Redraw</span>
					</ToolbarItem>
					<ToolbarItem>
						<button
							type="button"
							className={boardToolbarToggleClass(false)}
							title="Redraw graph"
							onClick={() => applyBoardRedrawOnce("force-graph")}
						>
							<span className="px-0.5">Graph</span>
						</button>
					</ToolbarItem>
					<ToolbarItem>
						<button
							type="button"
							className={boardToolbarToggleClass(false)}
							title="Redraw handles"
							onClick={() => applyBoardRedrawHandlesOnce()}
						>
							<span className="px-0.5">Handles</span>
						</button>
					</ToolbarItem>
				</ToolbarGroup>
			</ToolbarZone>
		</div>
	);
}
// #endregion 🔖Toolbar

// #region 🔖SettingsPanel
/** @emoji ⚙️ Board play redraw settings: play uses requestAnimationFrame (packed WASM per frame), progressive ramp, and per-mode layout parameters. */
function BoardPlaySettingsPanel(): ReactElement {
	const {
		activePaneId,
		applyBoardRedrawHandlesOnce,
		applyBoardRedrawOnce,
		boardRedrawHandlesAfterNodes,
		boardRedrawMode,
		boardRedrawPlayMaxItersPerFrame,
		boardRedrawProgressiveAutoStopMs,
		boardRedrawProgressiveEnabled,
		forceLayoutFullIterations,
		forceLayoutGravity,
		forceLayoutIdealEdgeLength,
		forceLayoutRepulsionStrength,
		setBoardRedrawMode,
		setBoardRedrawHandlesAfterNodes,
		setBoardRedrawPlayMaxItersPerFrame,
		setBoardRedrawProgressiveAutoStopMs,
		setBoardRedrawProgressiveEnabled,
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
				<div className="text-muted-foreground text-[11px] font-medium uppercase tracking-wide">Redraw nodes</div>
				<Label id="board.play.settings.redraw.mode" label="Layout kind">
					<Select id="board-play-redraw-mode-select" onValueChange={(v) => setBoardRedrawMode(v as BoardRedrawModeKind)} value={boardRedrawMode}>
						<SelectTrigger className="h-8 w-full" id="board-play-redraw-mode" size="sm">
							<SelectValue />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value="force-graph">Graph</SelectItem>
							<SelectItem value="hierarchical-tree">Tree</SelectItem>
						</SelectContent>
					</Select>
				</Label>
				<div className="flex items-center gap-2">
					<input
						checked={boardRedrawHandlesAfterNodes}
						className="accent-accent size-3.5 shrink-0"
						id="board-play-redraw-handles-after-nodes"
						onChange={(e) => setBoardRedrawHandlesAfterNodes(e.target.checked)}
						type="checkbox"
					/>
					<label className="text-muted-foreground cursor-pointer select-none text-[11px] leading-snug" htmlFor="board-play-redraw-handles-after-nodes">
						Also redraw handles after node redraw
					</label>
				</div>
				<div className="flex items-center gap-2">
					<input
						checked={boardRedrawProgressiveEnabled}
						className="accent-accent size-3.5 shrink-0"
						id="board-play-redraw-progressive"
						onChange={(e) => setBoardRedrawProgressiveEnabled(e.target.checked)}
						type="checkbox"
					/>
					<label className="text-muted-foreground cursor-pointer select-none text-[11px] leading-snug" htmlFor="board-play-redraw-progressive">
						Progressive iterations while play is on (graph ramps up; tree still one pass per frame)
					</label>
				</div>
				<Label id="board.play.settings.redraw.autoStopMs" label="Auto-stop play after (ms, 0 = off)">
					<Slider
						id="board-play-slider-redraw-autostop"
						max={12000}
						min={0}
						step={250}
						value={[boardRedrawProgressiveAutoStopMs]}
						onValueChange={(vals) => setBoardRedrawProgressiveAutoStopMs(vals[0] ?? 3000)}
					/>
				</Label>
				{boardRedrawMode === "force-graph" ? (
					<Label id="board.play.settings.redraw.playMaxIters" label="Max iterations per WASM call (play ramp ceiling)">
						<Slider
							id="board-play-slider-redraw-play-max-iters"
							max={220}
							min={12}
							step={2}
							value={[boardRedrawPlayMaxItersPerFrame]}
							onValueChange={(vals) => setBoardRedrawPlayMaxItersPerFrame(vals[0] ?? 96)}
						/>
					</Label>
				) : (
					<p className="text-muted-foreground text-[11px] leading-snug">
						Tree redraw runs once per animation frame while play is on; use auto-stop to end play after a duration.
					</p>
				)}
				{boardRedrawMode === "force-graph" ? (
					<>
						<div className="text-muted-foreground pt-1 text-[11px] font-medium uppercase tracking-wide">Graph</div>
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
						<Label id="board.play.settings.force.repulsion" label="Repulsion (medium 80, ±40)">
							<Slider
								id="board-play-slider-force-repulsion"
								max={120}
								min={40}
								step={2}
								value={[forceLayoutRepulsionStrength]}
								onValueChange={(vals) => setForceLayoutRepulsionStrength(vals[0] ?? 80)}
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
						<div className="text-muted-foreground pt-1 text-[11px] font-medium uppercase tracking-wide">Tree</div>
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
							<Select id="board-play-tree-direction-select" onValueChange={(v) => setTreeLayoutDirection(v as BoardHierarchicalTreeDirectionKind)} value={treeLayoutDirection}>
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
				<Button className="h-8 w-full text-xs" type="button" variant="outline" onClick={applyBoardRedrawOnce}>
					Redraw nodes
				</Button>
				<div className="text-muted-foreground border-t border-element pt-2 text-[11px] font-medium uppercase tracking-wide">Redraw handles</div>
				<p className="text-muted-foreground text-[11px] leading-snug">
					Each edge uses the straight segment between node centers; handle anchors move to where that segment meets each shape (shortest chord through the bodies).
				</p>
				<Button className="h-8 w-full text-xs" type="button" variant="outline" onClick={applyBoardRedrawHandlesOnce}>
					Redraw handles
				</Button>
				<p className="text-muted-foreground text-[11px] leading-snug">
					Enable Redraw zoom on a board window to let that window follow redraw toward the current layout fit. When it is off, redraw keeps the current camera. Dragging a node resets progressive ramp and the auto-stop timer.
				</p>
			</div>
		</div>
	);
}
// #endregion 🔖SettingsPanel

// #region 🔖Scene
/** @emoji 🗼 Marker tree for {@link BoardCanvas} — must stay a Fragment of {@link Node}/{@link Edge} so {@link buildBoardSceneDescriptor} sees markers (custom wrappers are opaque to the static walk). */
function nakaginBoardMarkers(props: {
	fixture: BoardFixtureV1;
	lockedIds: ReadonlySet<string>;
	selectedIds: Set<string>;
	contextMenuById: (id: string | null) => ContextMenuItem[];
	wires: readonly BoardPlayWireRecord[];
}): ReactElement {
	const { contextMenuById, fixture, lockedIds, selectedIds, wires } = props;
	return (
		<>
			{fixture.nodes.map((node) =>
				node.shape === "rectangle" ? (
					<Node
						contextMenu={contextMenuById(node.id)}
						draggable={!lockedIds.has(node.id)}
						height={node.height}
						id={node.id}
						key={node.id}
						{...(node.hidden === true ? { hidden: true } : {})}
						{...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {})}
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
						{...(node.iconKind ? { iconKind: node.iconKind } : {})}
					>
						{node.handles.map((handle) => (
							<Handle
								angle={handle.angle}
								color={handle.color}
								contextMenu={contextMenuById(handle.id)}
								handleKind={handle.handleKind}
								{...(handle.hidden === true ? { hidden: true } : {})}
								id={handle.id}
								key={handle.id}
								radius={handle.radius}
								selected={selectedIds.has(handle.id)}
								{...(handle.iconKind ? { iconKind: handle.iconKind } : {})}
							/>
						))}
					</Node>
				) : (
					<Node
						contextMenu={contextMenuById(node.id)}
						draggable={!lockedIds.has(node.id)}
						id={node.id}
						key={node.id}
						{...(node.hidden === true ? { hidden: true } : {})}
						{...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {})}
						radius={node.radius}
						selected={selectedIds.has(node.id)}
						text={node.text}
						textAlignment={node.textAlignment}
						textAutofit={node.textAutofit === true}
						textFontFamily={node.textFontFamily}
						textFontSize={node.textFontSize}
						x={node.x}
						y={node.y}
						{...(node.iconKind ? { iconKind: node.iconKind } : {})}
					>
						{node.handles.map((handle) => (
							<Handle
								angle={handle.angle}
								color={handle.color}
								contextMenu={contextMenuById(handle.id)}
								handleKind={handle.handleKind}
								{...(handle.hidden === true ? { hidden: true } : {})}
								id={handle.id}
								key={handle.id}
								radius={handle.radius}
								selected={selectedIds.has(handle.id)}
								{...(handle.iconKind ? { iconKind: handle.iconKind } : {})}
							/>
						))}
					</Node>
				),
			)}
			{fixture.edges.map((edge) => (
				<Edge
					contextMenu={contextMenuById(edge.id)}
					edgeKind={edge.edgeKind}
					{...(edge.hidden === true ? { hidden: true } : {})}
					id={edge.id}
					key={edge.id}
					selected={selectedIds.has(edge.id)}
					source={edge.source}
					target={edge.target}
				/>
			))}
			{wires.map((wire) => (
				<Wire
					contextMenu={contextMenuById(wire.id)}
					{...(typeof wire.endX === "number" ? { endX: wire.endX } : {})}
					{...(typeof wire.endY === "number" ? { endY: wire.endY } : {})}
					{...(wire.hidden === true ? { hidden: true } : {})}
					id={wire.id}
					key={wire.id}
					selected={selectedIds.has(wire.id)}
					source={wire.source}
					{...(wire.target ? { target: wire.target } : {})}
					{...(wire.wireKind ? { wireKind: wire.wireKind } : {})}
				/>
			))}
		</>
	);
}

/** @emoji 📡 Mirrors canvas selection into shell state for the owning pane. */
function BoardSelectionReporter({ paneId }: { paneId: BoardPlayPaneId }): null {
	const { activePaneId, setBoardSelectionGestureHighlight, setSelectionForPane, setWorkbenchSelection } = useBoardPlayShell();
	const handler = useCallback(
		(snapshot: BoardSelectionSnapshot) => {
			setSelectionForPane(paneId, snapshot.ids);
			setWorkbenchSelection(null);
			if (paneId === activePaneId) {
				setBoardSelectionGestureHighlight(snapshot.gestureMergeMode ?? null);
			}
		},
		[activePaneId, paneId, setBoardSelectionGestureHighlight, setSelectionForPane, setWorkbenchSelection],
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

/** @emoji 🔁 While play is on, each user `nodeMove` restarts the progressive graph ramp and auto-stop clock. */
function BoardPlayRedrawProgressReset(): null {
	const { boardRedrawPlaying, resetBoardRedrawProgressiveEpoch } = useBoardPlayShell();
	const handler = useCallback(() => {
		if (!boardRedrawPlaying) {
			return;
		}
		resetBoardRedrawProgressiveEpoch();
	}, [boardRedrawPlaying, resetBoardRedrawProgressiveEpoch]);
	useBoardEvent("nodeMove", handler);
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

function useBoardPaneContextMenus(paneId: BoardPlayPaneId, selectedIds: Set<string>): {
	backgroundMenu: ContextMenuItem[];
	contextMenuById: (id: string | null) => ContextMenuItem[];
} {
	const {
		deleteGraphObjects,
		fixture,
		focusGraphSelection,
		lockedIds,
		setGraphObjectsHidden,
		setGraphObjectsLocked,
		setSelectionForPane,
		setWorkbenchSelection,
		wires,
	} = useBoardPlayShell();
	const selectedList = useMemo(() => [...selectedIds].sort((left, right) => left.localeCompare(right)), [selectedIds]);

	const idsForTarget = useCallback(
		(id: string | null): string[] => {
			if (!id) {
				return selectedList;
			}
			if (selectedIds.has(id) && selectedList.length > 0) {
				return selectedList;
			}
			return [id];
		},
		[selectedIds, selectedList],
	);

	const menuForIds = useCallback(
		(ids: readonly string[]): ContextMenuItem[] => {
			if (ids.length === 0) {
				return [
					{
						id: `${paneId}-clear-selection`,
						label: "Clear selection",
						onSelect: () => {
							setSelectionForPane(paneId, []);
							setWorkbenchSelection(null);
						},
					},
				];
			}
			const hiddenSummary = boardPlayBoolSummary(ids.map((id) => graphObjectHiddenById(fixture, wires, id)));
			const lockedSummary = boardPlayBoolSummary(ids.map((id) => lockedIds.has(id)));
			const countLabel = ids.length === 1 ? "item" : `${ids.length} items`;
			return [
				{
					id: `${paneId}-focus-${ids.join("|")}`,
					label: `Select ${countLabel}`,
					onSelect: () => {
						focusGraphSelection(ids);
					},
				},
				{
					id: `${paneId}-${hiddenSummary.value ? "show" : "hide"}-${ids.join("|")}`,
					icon: hiddenSummary.value ? Eye : EyeOff,
					label: hiddenSummary.value ? `Show ${countLabel}` : `Hide ${countLabel}`,
					onSelect: () => {
						setGraphObjectsHidden(ids, !hiddenSummary.value);
					},
				},
				{
					id: `${paneId}-${lockedSummary.value ? "unlock" : "lock"}-${ids.join("|")}`,
					icon: lockedSummary.value ? Unlock : Lock,
					label: lockedSummary.value ? `Unlock ${countLabel}` : `Lock ${countLabel}`,
					onSelect: () => {
						setGraphObjectsLocked(ids, !lockedSummary.value);
					},
				},
				{
					id: `${paneId}-delete-${ids.join("|")}`,
					icon: Trash2,
					destructive: true,
					label: `Delete ${countLabel}`,
					onSelect: () => {
						deleteGraphObjects(ids);
					},
				},
			];
		},
		[deleteGraphObjects, fixture, focusGraphSelection, lockedIds, paneId, setGraphObjectsHidden, setGraphObjectsLocked, setSelectionForPane, setWorkbenchSelection, wires],
	);

	const contextMenuById = useCallback((id: string | null) => menuForIds(idsForTarget(id)), [idsForTarget, menuForIds]);
	const backgroundMenu = useMemo(() => menuForIds(selectedList), [menuForIds, selectedList]);
	return { backgroundMenu, contextMenuById };
}

function BoardOverviewPane(): ReactElement {
	const {
		activePaneId,
		boardAutomaticLodByPane,
		boardGridSnapEnabled,
		boardPinnedLodByPane,
		kindCatalogs,
		kindCompatibility,
		boardSelectionMethod,
		boardSelectionMode,
		boardSelectionTargets,
		fixture,
		handleCanvasFixtureDrop,
		lockedIds,
		camerasByPane,
		selectionByPane,
		syncBaselineFromViewportCamera,
		wires,
	} = useBoardPlayShell();
	const paneId: BoardPlayPaneId = "board-overview";
	const camera = camerasByPane[paneId];
	const selectedIds = selectionByPane[paneId];
	const { backgroundMenu, contextMenuById } = useBoardPaneContextMenus(paneId, selectedIds);
	return (
		<BoardPaneChrome paneId={paneId}>
			<BoardCanvas
				automaticLod={boardAutomaticLodByPane[paneId]}
				camera={camera}
				className="min-h-0 flex-1"
				contextMenu={backgroundMenu}
				fixtureDragDrop
				gridSnapEnabled={boardGridSnapEnabled}
				kindCatalogs={kindCatalogs}
				kindCompatibility={kindCompatibility}
				lod={boardPinnedLodByPane[paneId]}
				lodZoomThresholds={DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS}
				onCamera={activePaneId === paneId ? syncBaselineFromViewportCamera : undefined}
				onFixtureDrop={(d) => handleCanvasFixtureDrop(paneId, d)}
				selectionMethod={boardSelectionMethod}
				selectionMode={boardSelectionMode}
				selectionTargets={boardSelectionTargets}
			>
				<BoardSelectionReporter paneId={paneId} />
				<BoardStructuralDeleteReporter />
				<BoardPlayRedrawProgressReset />
				{nakaginBoardMarkers({ contextMenuById, fixture, lockedIds, selectedIds, wires })}
			</BoardCanvas>
		</BoardPaneChrome>
	);
}

function BoardDetailPane(): ReactElement {
	const {
		activePaneId,
		boardAutomaticLodByPane,
		boardGridSnapEnabled,
		boardPinnedLodByPane,
		kindCatalogs,
		kindCompatibility,
		boardSelectionMethod,
		boardSelectionMode,
		boardSelectionTargets,
		fixture,
		handleCanvasFixtureDrop,
		lockedIds,
		camerasByPane,
		selectionByPane,
		syncBaselineFromViewportCamera,
		wires,
	} = useBoardPlayShell();
	const paneId: BoardPlayPaneId = "board-detail";
	const camera = camerasByPane[paneId];
	const selectedIds = selectionByPane[paneId];
	const { backgroundMenu, contextMenuById } = useBoardPaneContextMenus(paneId, selectedIds);
	return (
		<BoardPaneChrome paneId={paneId}>
			<BoardCanvas
				automaticLod={boardAutomaticLodByPane[paneId]}
				camera={camera}
				className="min-h-0 flex-1"
				contextMenu={backgroundMenu}
				fixtureDragDrop
				gridSnapEnabled={boardGridSnapEnabled}
				kindCatalogs={kindCatalogs}
				kindCompatibility={kindCompatibility}
				lod={boardPinnedLodByPane[paneId]}
				lodZoomThresholds={DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS}
				onCamera={activePaneId === paneId ? syncBaselineFromViewportCamera : undefined}
				onFixtureDrop={(d) => handleCanvasFixtureDrop(paneId, d)}
				selectionMethod={boardSelectionMethod}
				selectionMode={boardSelectionMode}
				selectionTargets={boardSelectionTargets}
			>
				<BoardSelectionReporter paneId={paneId} />
				<BoardStructuralDeleteReporter />
				<BoardPlayRedrawProgressReset />
				{nakaginBoardMarkers({ contextMenuById, fixture, lockedIds, selectedIds, wires })}
			</BoardCanvas>
		</BoardPaneChrome>
	);
}

function BoardSelectionPane(): ReactElement {
	const {
		activePaneId,
		boardAutomaticLodByPane,
		boardGridSnapEnabled,
		boardPinnedLodByPane,
		kindCatalogs,
		kindCompatibility,
		boardSelectionMethod,
		boardSelectionMode,
		boardSelectionTargets,
		fixture,
		handleCanvasFixtureDrop,
		lockedIds,
		camerasByPane,
		selectionByPane,
		syncBaselineFromViewportCamera,
		wires,
	} = useBoardPlayShell();
	const paneId: BoardPlayPaneId = "board-selection";
	const camera = camerasByPane[paneId];
	const selectedIds = selectionByPane[paneId];
	const { backgroundMenu, contextMenuById } = useBoardPaneContextMenus(paneId, selectedIds);
	return (
		<BoardPaneChrome paneId={paneId}>
			<BoardCanvas
				automaticLod={boardAutomaticLodByPane[paneId]}
				camera={camera}
				className="min-h-0 flex-1"
				contextMenu={backgroundMenu}
				fixtureDragDrop
				gridSnapEnabled={boardGridSnapEnabled}
				kindCatalogs={kindCatalogs}
				kindCompatibility={kindCompatibility}
				lod={boardPinnedLodByPane[paneId]}
				lodZoomThresholds={DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS}
				onCamera={activePaneId === paneId ? syncBaselineFromViewportCamera : undefined}
				onFixtureDrop={(d) => handleCanvasFixtureDrop(paneId, d)}
				selectionMethod={boardSelectionMethod}
				selectionMode={boardSelectionMode}
				selectionTargets={boardSelectionTargets}
			>
				<BoardSelectionReporter paneId={paneId} />
				<BoardStructuralDeleteReporter />
				<BoardPlayRedrawProgressReset />
				{nakaginBoardMarkers({ contextMenuById, fixture, lockedIds, selectedIds, wires })}
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
							className="border-element bg-muted/40 pointer-events-none fixed z-2147483000 flex items-center justify-center rounded-lg border shadow-sm"
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
function boardTreeAddAction(id: string, onClick: () => void, title: string): TreeHeaderAction {
	return { icon: <Plus className="size-3.5" />, id, onClick, title };
}

function boardTreeItem({
	description,
	id,
	items,
	label,
	onClick,
	selected,
}: {
	description?: ReactNode;
	id: string;
	items?: TreeDataItem[];
	label: ReactNode;
	onClick?: () => void;
	selected?: boolean;
}): TreeDataItem {
	const childItems = items?.length ? items : undefined;
	const childSelected = childItems?.some((item) => item.isSelected) ?? false;
	return {
		...(childItems ? { defaultOpen: childSelected, items: childItems } : {}),
		...(description ? { description } : {}),
		...(onClick
			? {
				onClick: () => {
					onClick();
				},
			}
			: {}),
		id,
		isSelected: selected === true,
		label,
	};
}

function BoardFixtureLibraryPanel(): ReactElement {
	const { fixture, kindCatalogs, kindCompatibility, lockedIds, wires } = useBoardPlayShell();

	const onShelfDragStart = useCallback(
		(e: DragEvent<HTMLDivElement>) => {
			e.dataTransfer.setData(
				BOARD_FIXTURE_DRAG_V1_MIME,
				encodeBoardFixtureForDragV1(
					buildBoardPlayFixturePayload({
						fixture,
						kindCatalogs,
						kindCompatibility,
						lockedIds: [...lockedIds],
						wires,
					}),
				),
			);
			e.dataTransfer.effectAllowed = "copy";
		},
		[fixture, kindCatalogs, kindCompatibility, lockedIds, wires],
	);
	const sections = useMemo<TreeDataSection[]>(
		() => [
			{
				content: (
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
				),
				defaultOpen: true,
				id: "board-play-library-shapes",
				label: "Shapes",
			},
			{
				content: (
					<div
						className="border-element bg-muted/30 flex min-h-30 cursor-grab flex-col justify-center gap-2 rounded-md border p-4 active:cursor-grabbing"
				draggable
				onDragStart={onShelfDragStart}
			>
				<p className="font-medium">Active graph</p>
				<p className="text-muted-foreground text-xs">Drag onto any board tab to load this graph (same payload for all panes).</p>
			</div>
				),
				defaultOpen: true,
				id: "board-play-library-active-graph",
				label: "Payload",
			},
			{
				defaultOpen: true,
				id: "board-play-library-loaded",
				items: [
					boardTreeItem({ description: fixture.schema, id: "board-play-library-loaded-schema", label: "Schema" }),
					boardTreeItem({ description: `${fixture.nodes.length} nodes · ${fixture.edges.length} edges · ${wires.length} wires`, id: "board-play-library-loaded-graph", label: "Graph" }),
				],
				label: "Loaded",
			},
		],
		[fixture.edges.length, fixture.nodes.length, fixture.schema, onShelfDragStart, wires.length],
	);

	return <BoardSideTreePanel header="Library" icon={<Library className="size-4 shrink-0" />} sections={sections} subtitle={<span data-testid="board-play-fixture-shelf">Fixture shelf</span>} />;
}

function findNode(fixture: BoardFixtureV1, id: string): BoardFixtureNodeV1 | undefined {
	return fixture.nodes.find((n) => n.id === id);
}

function findEdge(fixture: BoardFixtureV1, id: string): BoardFixtureEdgeV1 | undefined {
	return fixture.edges.find((e) => e.id === id);
}

function findWire(wires: readonly BoardPlayWireRecord[], id: string): BoardPlayWireRecord | undefined {
	return wires.find((wire) => wire.id === id);
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

function listNodeKindIds(kindCatalogs: BoardKindCatalogBundle): string[] {
	return uniqueSortedStrings((kindCatalogs.nodes ?? []).map((entry) => entry.id));
}

function listHandleKindIds(kindCatalogs: BoardKindCatalogBundle): string[] {
	return uniqueSortedStrings((kindCatalogs.handles ?? []).map((entry) => entry.id));
}

function listEdgeKindIds(kindCatalogs: BoardKindCatalogBundle): string[] {
	return uniqueSortedStrings((kindCatalogs.edges ?? []).map((entry) => entry.id));
}

function listWireKindIds(kindCatalogs: BoardKindCatalogBundle): string[] {
	return uniqueSortedStrings((kindCatalogs.wires ?? []).map((entry) => entry.id));
}

function boardPlayBoolSummary(values: boolean[]): { uniform: boolean; value: boolean } {
	return { uniform: allEqual(values), value: values.every(Boolean) };
}

function boardPlayOptionalStringSummary(values: string[]): { uniform: boolean; value: string } {
	return { uniform: allEqual(values), value: values[0] ?? "" };
}

function graphObjectHiddenById(fixture: BoardFixtureV1, wires: readonly BoardPlayWireRecord[], id: string): boolean {
	const node = findNode(fixture, id);
	if (node) {
		return node.hidden === true;
	}
	const handle = findHandle(fixture, id);
	if (handle) {
		return handle.hidden === true;
	}
	const edge = findEdge(fixture, id);
	if (edge) {
		return edge.hidden === true;
	}
	const wire = findWire(wires, id);
	if (wire) {
		return wire.hidden === true;
	}
	return false;
}

function graphObjectKindById(fixture: BoardFixtureV1, wires: readonly BoardPlayWireRecord[], id: string): "edge" | "handle" | "node" | "wire" | null {
	if (findNode(fixture, id)) {
		return "node";
	}
	if (findHandle(fixture, id)) {
		return "handle";
	}
	if (findEdge(fixture, id)) {
		return "edge";
	}
	if (findWire(wires, id)) {
		return "wire";
	}
	return null;
}

function expandDeletedGraphIds(fixture: BoardFixtureV1, wires: readonly BoardPlayWireRecord[], ids: readonly string[]): string[] {
	const remove = new Set(ids);
	const handleIds = new Set<string>();
	const edgeIds = new Set<string>();
	const wireIds = new Set<string>();
	for (const id of ids) {
		const node = findNode(fixture, id);
		if (node) {
			for (const handle of node.handles) {
				handleIds.add(handle.id);
			}
			continue;
		}
		if (findHandle(fixture, id)) {
			handleIds.add(id);
			continue;
		}
		if (findEdge(fixture, id)) {
			edgeIds.add(id);
			continue;
		}
		if (findWire(wires, id)) {
			wireIds.add(id);
		}
	}
	for (const node of fixture.nodes) {
		if (remove.has(node.id)) {
			for (const handle of node.handles) {
				handleIds.add(handle.id);
			}
		}
	}
	for (const edge of fixture.edges) {
		if (handleIds.has(edge.source) || handleIds.has(edge.target)) {
			edgeIds.add(edge.id);
		}
	}
	for (const wire of wires) {
		if (handleIds.has(wire.source) || (wire.target && handleIds.has(wire.target))) {
			wireIds.add(wire.id);
		}
	}
	return uniqueSortedStrings([...remove, ...handleIds, ...edgeIds, ...wireIds]);
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
					id={id}
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
	kindCatalogs,
	lockedIds,
	nodeIds,
	patchFixture,
	remapIdInSelections,
	setGraphObjectsHidden,
	setGraphObjectsLocked,
}: {
	fixture: BoardFixtureV1;
	kindCatalogs: BoardKindCatalogBundle;
	lockedIds: ReadonlySet<string>;
	nodeIds: readonly string[];
	patchFixture: (updater: (prev: BoardFixtureV1) => BoardFixtureV1) => void;
	remapIdInSelections: (replacedId: string, replacementId: string) => void;
	setGraphObjectsHidden: (ids: readonly string[], hidden: boolean) => void;
	setGraphObjectsLocked: (ids: readonly string[], locked: boolean) => void;
}): ReactElement {
	const idSet = useMemo(() => new Set(nodeIds), [nodeIds]);
	const targets = useMemo(
		() => nodeIds.map((id) => findNode(fixture, id)).filter((n): n is BoardFixtureNodeV1 => Boolean(n)),
		[fixture, nodeIds],
	);

	const textValues = targets.map((n) => n.text ?? "");
	const textUniform = allEqual(textValues);
	const textValue = textUniform ? (textValues[0] ?? "") : "";

	const iconKinds = targets.map((n) => n.iconKind ?? "");
	const iconKindUniform = allEqual(iconKinds);
	const iconKindValue = iconKindUniform ? (iconKinds[0] ?? "") : "";

	const shapes = targets.map((n) => (nodeIsRectangle(n) ? "rectangle" : "circle"));
	const shapeUniform = allEqual(shapes);
	const shapeValue = shapeUniform ? shapes[0] : undefined;
	const hiddenSummary = boardPlayBoolSummary(targets.map((node) => node.hidden === true));
	const lockedSummary = boardPlayBoolSummary(nodeIds.map((id) => lockedIds.has(id)));
	const rootSummary = boardPlayBoolSummary(targets.map((node) => node.root === true));
	const textAutofitSummary = boardPlayBoolSummary(targets.map((node) => node.textAutofit === true));
	const nodeKindSummary = boardPlayOptionalStringSummary(targets.map((node) => node.nodeKind ?? ""));
	const textAlignmentSummary = boardPlayOptionalStringSummary(targets.map((node) => node.textAlignment ?? ""));
	const textFontFamilySummary = boardPlayOptionalStringSummary(targets.map((node) => node.textFontFamily ?? ""));
	const textFontSizeValues = targets.map((node) => node.textFontSize ?? 14);
	const textFontSizeUniform = allEqual(textFontSizeValues);
	const textFontSizeValue = textFontSizeUniform ? textFontSizeValues[0] ?? 14 : Number.NaN;

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

	const onIconKind = useCallback(
		(next: string) => {
			const t = next.trim();
			patchNodes((n) => ({ ...n, ...(t === "" ? { iconKind: undefined } : { iconKind: t }) }));
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
			<div className="grid grid-cols-2 gap-2">
				<label className="text-muted-foreground flex items-center gap-2 text-[11px]">
					<input checked={hiddenSummary.value} className="accent-accent size-3.5" onChange={(e) => setGraphObjectsHidden(nodeIds, e.target.checked)} type="checkbox" />
					<span>{hiddenSummary.uniform ? "Hidden" : "Hidden (mixed)"}</span>
				</label>
				<label className="text-muted-foreground flex items-center gap-2 text-[11px]">
					<input checked={lockedSummary.value} className="accent-accent size-3.5" onChange={(e) => setGraphObjectsLocked(nodeIds, e.target.checked)} type="checkbox" />
					<span>{lockedSummary.uniform ? "Locked" : "Locked (mixed)"}</span>
				</label>
				<label className="text-muted-foreground flex items-center gap-2 text-[11px]">
					<input
						checked={rootSummary.value}
						className="accent-accent size-3.5"
						onChange={(e) => patchNodes((node) => ({ ...node, ...(e.target.checked ? { root: true } : { root: undefined }) }))}
						type="checkbox"
					/>
					<span>{rootSummary.uniform ? "Root" : "Root (mixed)"}</span>
				</label>
				<label className="text-muted-foreground flex items-center gap-2 text-[11px]">
					<input
						checked={textAutofitSummary.value}
						className="accent-accent size-3.5"
						onChange={(e) => patchNodes((node) => ({ ...node, ...(e.target.checked ? { textAutofit: true } : { textAutofit: undefined }) }))}
						type="checkbox"
					/>
					<span>{textAutofitSummary.uniform ? "Text autofit" : "Text autofit (mixed)"}</span>
				</label>
			</div>
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
			<Label id="board-play.inspector.node.icon" label="Icon">
				<IconSelector
					classifyElementsBoardIconSelectorMode={classifyElementsBoardIconSelectorMode}
					id="board-play.inspector.node.icon.selector"
					onChange={onIconKind}
					uniform={iconKindUniform}
					value={iconKindValue}
				/>
			</Label>
			<Label id="board-play.inspector.node.kind" label="Node kind">
				<Select
					onValueChange={(value) => {
						patchNodes((node) => ({ ...node, ...(value === "__none__" ? { nodeKind: undefined } : { nodeKind: value }) }));
					}}
					value={nodeKindSummary.uniform ? (nodeKindSummary.value || "__none__") : undefined}
				>
					<SelectTrigger className="h-7 font-mono text-xs">
						<SelectValue placeholder={nodeKindSummary.uniform ? "node kind" : "Mixed"} />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="__none__">Unassigned</SelectItem>
						{listNodeKindIds(kindCatalogs).map((kindId) => (
							<SelectItem key={kindId} value={kindId}>
								{boardPlayKindLabel(kindCatalogs.nodes, kindId)}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
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
			<Label id="board-play.inspector.node.textAlignment" label="Text alignment">
				<Select
					onValueChange={(value) => {
						patchNodes((node) => ({ ...node, ...(value === "__none__" ? { textAlignment: undefined } : { textAlignment: value as BoardFixtureNodeV1["textAlignment"] }) }));
					}}
					value={textAlignmentSummary.uniform ? (textAlignmentSummary.value || "__none__") : undefined}
				>
					<SelectTrigger className="h-7 font-mono text-xs">
						<SelectValue placeholder={textAlignmentSummary.uniform ? "alignment" : "Mixed"} />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="__none__">Default</SelectItem>
						{["c", "e", "n", "ne", "nw", "s", "se", "sw", "w"].map((alignment) => (
							<SelectItem key={alignment} value={alignment}>
								{alignment}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</Label>
			<Label id="board-play.inspector.node.fontFamily" label="Text font family">
				<Input
					className="h-7 font-mono text-xs"
					onChange={(e: ChangeEvent<HTMLInputElement>) => {
						const next = e.target.value.trim();
						patchNodes((node) => ({ ...node, ...(next === "" ? { textFontFamily: undefined } : { textFontFamily: next }) }));
					}}
					placeholder={textFontFamilySummary.uniform ? undefined : "Mixed"}
					value={textFontFamilySummary.uniform ? textFontFamilySummary.value : ""}
				/>
			</Label>
			<NumericStepperRow
				id="board-play.inspector.node.fontSize"
				label="Text font size"
				onAbsolute={(value) => patchNodes((node) => ({ ...node, textFontSize: Math.max(1, value) }))}
				onDelta={(delta) => patchNodes((node) => ({ ...node, textFontSize: Math.max(1, (node.textFontSize ?? 14) + delta) }))}
				step={1}
				uniform={textFontSizeUniform}
				value={textFontSizeValue}
			/>
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
	kindCatalogs,
	lockedIds,
	patchFixture,
	remapIdInSelections,
	setGraphObjectsHidden,
	setGraphObjectsLocked,
}: {
	fixture: BoardFixtureV1;
	handleIds: readonly string[];
	kindCatalogs: BoardKindCatalogBundle;
	lockedIds: ReadonlySet<string>;
	patchFixture: (updater: (prev: BoardFixtureV1) => BoardFixtureV1) => void;
	remapIdInSelections: (replacedId: string, replacementId: string) => void;
	setGraphObjectsHidden: (ids: readonly string[], hidden: boolean) => void;
	setGraphObjectsLocked: (ids: readonly string[], locked: boolean) => void;
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
	const hiddenSummary = boardPlayBoolSummary(handles.map((handle) => handle.hidden === true));
	const lockedSummary = boardPlayBoolSummary(handleIds.map((id) => lockedIds.has(id)));
	const handleKindSummary = boardPlayOptionalStringSummary(handles.map((handle) => handle.handleKind ?? ""));
	const colorSummary = boardPlayOptionalStringSummary(handles.map((handle) => handle.color ?? ""));

	const iconKinds = handles.map((h) => h.iconKind ?? "");
	const iconKindUniform = allEqual(iconKinds);
	const iconKindValue = iconKindUniform ? (iconKinds[0] ?? "") : "";

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

	const onIconKind = useCallback(
		(next: string) => {
			const t = next.trim();
			patchHandles((h) => ({ ...h, ...(t === "" ? { iconKind: undefined } : { iconKind: t }) }));
		},
		[patchHandles],
	);

	return (
		<div className="border-element/60 space-y-3 border-l pl-2">
			<div className="grid grid-cols-2 gap-2">
				<label className="text-muted-foreground flex items-center gap-2 text-[11px]">
					<input checked={hiddenSummary.value} className="accent-accent size-3.5" onChange={(e) => setGraphObjectsHidden(handleIds, e.target.checked)} type="checkbox" />
					<span>{hiddenSummary.uniform ? "Hidden" : "Hidden (mixed)"}</span>
				</label>
				<label className="text-muted-foreground flex items-center gap-2 text-[11px]">
					<input checked={lockedSummary.value} className="accent-accent size-3.5" onChange={(e) => setGraphObjectsLocked(handleIds, e.target.checked)} type="checkbox" />
					<span>{lockedSummary.uniform ? "Locked" : "Locked (mixed)"}</span>
				</label>
			</div>
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
					<Label id="board-play.inspector.handle.kind" label="Handle kind">
						<Select
							onValueChange={(value) => {
								patchHandles((handle) => ({ ...handle, handleKind: value === "__none__" ? BOARD_BUILTIN_PORT_HANDLE_KIND : value }));
							}}
							value={handleKindSummary.uniform ? (handleKindSummary.value || "__none__") : undefined}
						>
							<SelectTrigger className="h-7 font-mono text-xs">
								<SelectValue placeholder={handleKindSummary.uniform ? "handle kind" : "Mixed"} />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="__none__">Default port</SelectItem>
								{listHandleKindIds(kindCatalogs).map((kindId) => (
									<SelectItem key={kindId} value={kindId}>
										{boardPlayKindLabel(kindCatalogs.handles, kindId)}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</Label>
					<Label id="board-play.inspector.handle.color" label="Color">
						<Input
							className="h-7 font-mono text-xs"
							onChange={(e: ChangeEvent<HTMLInputElement>) => {
								const next = e.target.value.trim();
								patchHandles((handle) => ({ ...handle, ...(next === "" ? { color: undefined } : { color: next }) }));
							}}
							placeholder={colorSummary.uniform ? "#rrggbb" : "Mixed"}
							value={colorSummary.uniform ? colorSummary.value : ""}
						/>
					</Label>
					<Label id="board-play.inspector.handle.icon" label="Icon">
						<IconSelector
							classifyElementsBoardIconSelectorMode={classifyElementsBoardIconSelectorMode}
							id="board-play.inspector.handle.icon.selector"
							onChange={onIconKind}
							uniform={iconKindUniform}
							value={iconKindValue}
						/>
					</Label>
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
	kindCatalogs,
	lockedIds,
	patchFixture,
	remapIdInSelections,
	setGraphObjectsHidden,
	setGraphObjectsLocked,
}: {
	fixture: BoardFixtureV1;
	edgeIds: readonly string[];
	kindCatalogs: BoardKindCatalogBundle;
	lockedIds: ReadonlySet<string>;
	patchFixture: (updater: (prev: BoardFixtureV1) => BoardFixtureV1) => void;
	remapIdInSelections: (replacedId: string, replacementId: string) => void;
	setGraphObjectsHidden: (ids: readonly string[], hidden: boolean) => void;
	setGraphObjectsLocked: (ids: readonly string[], locked: boolean) => void;
}): ReactElement {
	const idSet = useMemo(() => new Set(edgeIds), [edgeIds]);
	const edges = useMemo(
		() => edgeIds.map((id) => findEdge(fixture, id)).filter((e): e is BoardFixtureEdgeV1 => Boolean(e)),
		[edgeIds, fixture],
	);
	const sources = edges.map((e) => e.source);
	const targets = edges.map((e) => e.target);
	const edgeKinds = edges.map((edge) => edge.edgeKind ?? "");
	const sourceUniform = allEqual(sources);
	const targetUniform = allEqual(targets);
	const edgeKindSummary = boardPlayOptionalStringSummary(edgeKinds);
	const hiddenSummary = boardPlayBoolSummary(edges.map((edge) => edge.hidden === true));
	const lockedSummary = boardPlayBoolSummary(edgeIds.map((id) => lockedIds.has(id)));
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
			<div className="grid grid-cols-2 gap-2">
				<label className="text-muted-foreground flex items-center gap-2 text-[11px]">
					<input checked={hiddenSummary.value} className="accent-accent size-3.5" onChange={(e) => setGraphObjectsHidden(edgeIds, e.target.checked)} type="checkbox" />
					<span>{hiddenSummary.uniform ? "Hidden" : "Hidden (mixed)"}</span>
				</label>
				<label className="text-muted-foreground flex items-center gap-2 text-[11px]">
					<input checked={lockedSummary.value} className="accent-accent size-3.5" onChange={(e) => setGraphObjectsLocked(edgeIds, e.target.checked)} type="checkbox" />
					<span>{lockedSummary.uniform ? "Locked" : "Locked (mixed)"}</span>
				</label>
			</div>
			<Label id="board-play.inspector.edge.kind" label="Edge kind">
				<Select
					onValueChange={(value) => {
						patchEdges((edge) => ({ ...edge, ...(value === "__none__" ? { edgeKind: undefined } : { edgeKind: value }) }));
					}}
					value={edgeKindSummary.uniform ? (edgeKindSummary.value || "__none__") : undefined}
				>
					<SelectTrigger className="h-7 font-mono text-xs">
						<SelectValue placeholder={edgeKindSummary.uniform ? "edge kind" : "Mixed"} />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="__none__">Unassigned</SelectItem>
						{listEdgeKindIds(kindCatalogs).map((kindId) => (
							<SelectItem key={kindId} value={kindId}>
								{boardPlayKindLabel(kindCatalogs.edges, kindId)}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</Label>
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

function InspectorWireBatch({
	fixture,
	kindCatalogs,
	lockedIds,
	patchWires,
	remapIdInSelections,
	setGraphObjectsHidden,
	setGraphObjectsLocked,
	wireIds,
	wires,
}: {
	fixture: BoardFixtureV1;
	kindCatalogs: BoardKindCatalogBundle;
	lockedIds: ReadonlySet<string>;
	patchWires: (updater: (prev: BoardPlayWireRecord[]) => BoardPlayWireRecord[]) => void;
	remapIdInSelections: (replacedId: string, replacementId: string) => void;
	setGraphObjectsHidden: (ids: readonly string[], hidden: boolean) => void;
	setGraphObjectsLocked: (ids: readonly string[], locked: boolean) => void;
	wireIds: readonly string[];
	wires: readonly BoardPlayWireRecord[];
}): ReactElement {
	const idSet = useMemo(() => new Set(wireIds), [wireIds]);
	const targets = useMemo(() => wireIds.map((id) => findWire(wires, id)).filter((wire): wire is BoardPlayWireRecord => Boolean(wire)), [wireIds, wires]);
	const handleOptions = useMemo(() => listHandleIds(fixture), [fixture]);
	const sourceSummary = boardPlayOptionalStringSummary(targets.map((wire) => wire.source));
	const targetSummary = boardPlayOptionalStringSummary(targets.map((wire) => wire.target ?? ""));
	const wireKindSummary = boardPlayOptionalStringSummary(targets.map((wire) => wire.wireKind ?? ""));
	const hiddenSummary = boardPlayBoolSummary(targets.map((wire) => wire.hidden === true));
	const lockedSummary = boardPlayBoolSummary(wireIds.map((id) => lockedIds.has(id)));
	const endXValues = targets.map((wire) => wire.endX ?? 0);
	const endYValues = targets.map((wire) => wire.endY ?? 0);
	const endXUniform = allEqual(endXValues);
	const endYUniform = allEqual(endYValues);
	const endXValue = endXUniform ? endXValues[0] ?? 0 : Number.NaN;
	const endYValue = endYUniform ? endYValues[0] ?? 0 : Number.NaN;

	const patchTargetWires = useCallback(
		(updater: (wire: BoardPlayWireRecord) => BoardPlayWireRecord) => {
			patchWires((prev) => prev.map((wire) => (idSet.has(wire.id) ? updater(wire) : wire)));
		},
		[idSet, patchWires],
	);

	return (
		<div className="border-element/60 space-y-3 border-l pl-2">
			<div className="grid grid-cols-2 gap-2">
				<label className="text-muted-foreground flex items-center gap-2 text-[11px]">
					<input checked={hiddenSummary.value} className="accent-accent size-3.5" onChange={(e) => setGraphObjectsHidden(wireIds, e.target.checked)} type="checkbox" />
					<span>{hiddenSummary.uniform ? "Hidden" : "Hidden (mixed)"}</span>
				</label>
				<label className="text-muted-foreground flex items-center gap-2 text-[11px]">
					<input checked={lockedSummary.value} className="accent-accent size-3.5" onChange={(e) => setGraphObjectsLocked(wireIds, e.target.checked)} type="checkbox" />
					<span>{lockedSummary.uniform ? "Locked" : "Locked (mixed)"}</span>
				</label>
			</div>
			<Label id="board-play.inspector.wire.kind" label="Wire kind">
				<Select
					onValueChange={(value) => {
						patchTargetWires((wire) => ({ ...wire, ...(value === "__none__" ? { wireKind: undefined } : { wireKind: value }) }));
					}}
					value={wireKindSummary.uniform ? (wireKindSummary.value || "__none__") : undefined}
				>
					<SelectTrigger className="h-7 font-mono text-xs">
						<SelectValue placeholder={wireKindSummary.uniform ? "wire kind" : "Mixed"} />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="__none__">Unassigned</SelectItem>
						{listWireKindIds(kindCatalogs).map((kindId) => (
							<SelectItem key={kindId} value={kindId}>
								{boardPlayKindLabel(kindCatalogs.wires, kindId)}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</Label>
			<Label id="board-play.inspector.wire.source" label="Source">
				<Select
					onValueChange={(value) => {
						patchTargetWires((wire) => ({ ...wire, source: value }));
					}}
					value={sourceSummary.uniform ? sourceSummary.value : undefined}
				>
					<SelectTrigger className="h-7 font-mono text-xs">
						<SelectValue placeholder={sourceSummary.uniform ? "source" : "Mixed"} />
					</SelectTrigger>
					<SelectContent>
						{handleOptions.map((handleId) => (
							<SelectItem key={`wire-source-${handleId}`} value={handleId}>
								{handleId}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</Label>
			<Label id="board-play.inspector.wire.target" label="Target handle">
				<Select
					onValueChange={(value) => {
						patchTargetWires((wire) => ({ ...wire, ...(value === "__free__" ? { target: undefined } : { target: value }) }));
					}}
					value={targetSummary.uniform ? (targetSummary.value || "__free__") : undefined}
				>
					<SelectTrigger className="h-7 font-mono text-xs">
						<SelectValue placeholder={targetSummary.uniform ? "target" : "Mixed"} />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="__free__">Free end</SelectItem>
						{handleOptions.map((handleId) => (
							<SelectItem key={`wire-target-${handleId}`} value={handleId}>
								{handleId}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</Label>
			<NumericStepperRow
				id="board-play.inspector.wire.endX"
				label="Free end x"
				onAbsolute={(value) => patchTargetWires((wire) => ({ ...wire, endX: value, target: undefined }))}
				onDelta={(delta) => patchTargetWires((wire) => ({ ...wire, endX: (wire.endX ?? 0) + delta, target: undefined }))}
				step={1}
				uniform={endXUniform}
				value={endXValue}
			/>
			<NumericStepperRow
				id="board-play.inspector.wire.endY"
				label="Free end y"
				onAbsolute={(value) => patchTargetWires((wire) => ({ ...wire, endY: value, target: undefined }))}
				onDelta={(delta) => patchTargetWires((wire) => ({ ...wire, endY: (wire.endY ?? 0) + delta, target: undefined }))}
				step={1}
				uniform={endYUniform}
				value={endYValue}
			/>
			{wireIds.length === 1 ? (
				<Label id="board-play.inspector.wire.id" label="Id">
					<Input
						className="h-7 font-mono text-xs"
						defaultValue={wireIds[0]}
						key={wireIds[0]}
						onBlur={(e) => {
							const nextId = e.currentTarget.value.trim();
							const oldId = wireIds[0];
							if (!oldId || !nextId || nextId === oldId) {
								return;
							}
							patchWires((prev) => prev.map((wire) => (wire.id === oldId ? { ...wire, id: nextId } : wire)));
							remapIdInSelections(oldId, nextId);
						}}
					/>
				</Label>
			) : null}
		</div>
	);
}

function BoardSideTreePanel({
	header,
	icon,
	sections,
	subtitle,
}: {
	header: string;
	icon: ReactNode;
	sections: TreeDataSection[];
	subtitle?: ReactNode;
}): ReactElement {
	return (
		<div className="flex h-full min-h-0 flex-col gap-2 p-3 text-xs">
			<div className="text-muted-foreground flex shrink-0 items-center gap-2 border-b border-element pb-2">
				{icon}
				<div>
					<div className="font-semibold uppercase tracking-wide">{header}</div>
					{subtitle ? <div className="text-[11px] opacity-80">{subtitle}</div> : null}
				</div>
			</div>
			<TreeStateProvider>
				<Tree className="min-h-0 min-w-0 flex-1 overflow-y-auto overflow-x-hidden" sections={sections} />
			</TreeStateProvider>
		</div>
	);
}

function BoardWorkbenchGraphPanel(): ReactElement {
	const {
		activePaneId,
		fixture,
		focusGraphSelection,
		kindCatalogs,
		selectionByPane,
		wires,
		workbenchSelection,
	} = useBoardPlayShell();
	const selectedIds = selectionByPane[activePaneId];
	const graphSections = useMemo<TreeDataSection[]>(
		() => [
			{
				defaultOpen: true,
				id: "board-play-workbench-graph-nodes",
				items: fixture.nodes.map((node) =>
					boardTreeItem({
						id: node.id,
						items: node.handles.map((handle) =>
							boardTreeItem({
								description: handle.id,
								id: handle.id,
								label: boardPlayKindLabel(kindCatalogs.handles, handle.handleKind),
								onClick: () => focusGraphSelection([handle.id]),
								selected: selectedIds.has(handle.id) && !workbenchSelection,
							}),
						),
						label: node.text || node.id,
						description: node.id,
						onClick: () => focusGraphSelection([node.id]),
						selected: selectedIds.has(node.id) && !workbenchSelection,
					}),
				),
				label: `Nodes (${fixture.nodes.length})`,
			},
			{
				defaultOpen: true,
				id: "board-play-workbench-graph-edges",
				items: [
					...fixture.edges.map((edge) =>
						boardTreeItem({
							description: edge.id,
							id: edge.id,
							label: `${edge.source} -> ${edge.target}`,
							onClick: () => focusGraphSelection([edge.id]),
							selected: selectedIds.has(edge.id) && !workbenchSelection,
						}),
					),
					...wires.map((wire) =>
						boardTreeItem({
							description: wire.id,
							id: wire.id,
							label: `${wire.source} ${wire.target ? `-> ${wire.target}` : "-> free"}`,
							onClick: () => focusGraphSelection([wire.id]),
							selected: selectedIds.has(wire.id) && !workbenchSelection,
						}),
					),
				],
				label: `Edges (${fixture.edges.length + wires.length})`,
			},
		],
		[fixture.edges, fixture.nodes, focusGraphSelection, kindCatalogs.handles, selectedIds, wires, workbenchSelection],
	);
	return <BoardSideTreePanel header="Graph" icon={<FolderTree className="size-4 shrink-0" />} sections={graphSections} subtitle={`pane: ${activePaneId}`} />;
}

function BoardWorkbenchKindsPanel(): ReactElement {
	const { appendKind, focusWorkbenchSelection, kindCatalogs, workbenchSelection } = useBoardPlayShell();
	const kindSections = useMemo<TreeDataSection[]>(
		() => [
			{
				actions: [boardTreeAddAction("board-play-kind-add-node", () => appendKind("node-kind"), "Add node kind")],
				defaultOpen: true,
				id: "board-play-workbench-kind-nodes",
				items: (kindCatalogs.nodes ?? []).map((entry) =>
					boardTreeItem({
						description: entry.id,
						id: entry.id,
						label: entry.label || entry.id,
						onClick: () => focusWorkbenchSelection({ id: entry.id, kind: "node-kind" }),
						selected: workbenchSelection?.kind === "node-kind" && workbenchSelection.id === entry.id,
					}),
				),
				label: `Node kinds (${kindCatalogs.nodes?.length ?? 0})`,
			},
			{
				actions: [boardTreeAddAction("board-play-kind-add-edge", () => appendKind("edge-kind"), "Add edge kind")],
				defaultOpen: true,
				id: "board-play-workbench-kind-edges",
				items: (kindCatalogs.edges ?? []).map((entry) =>
					boardTreeItem({
						description: entry.id,
						id: entry.id,
						label: entry.label || entry.id,
						onClick: () => focusWorkbenchSelection({ id: entry.id, kind: "edge-kind" }),
						selected: workbenchSelection?.kind === "edge-kind" && workbenchSelection.id === entry.id,
					}),
				),
				label: `Edge kinds (${kindCatalogs.edges?.length ?? 0})`,
			},
			{
				actions: [boardTreeAddAction("board-play-kind-add-wire", () => appendKind("wire-kind"), "Add wire kind")],
				defaultOpen: true,
				id: "board-play-workbench-kind-wires",
				items: (kindCatalogs.wires ?? []).map((entry) =>
					boardTreeItem({
						description: entry.id,
						id: entry.id,
						label: entry.label || entry.id,
						onClick: () => focusWorkbenchSelection({ id: entry.id, kind: "wire-kind" }),
						selected: workbenchSelection?.kind === "wire-kind" && workbenchSelection.id === entry.id,
					}),
				),
				label: `Wire kinds (${kindCatalogs.wires?.length ?? 0})`,
			},
		],
		[appendKind, focusWorkbenchSelection, kindCatalogs.edges, kindCatalogs.nodes, kindCatalogs.wires, workbenchSelection],
	);
	return <BoardSideTreePanel header="Kinds" icon={<Circle className="size-4 shrink-0" />} sections={kindSections} subtitle="Node, edge, and wire kind catalogs" />;
}

function BoardWorkbenchConstraintsPanel(): ReactElement {
	const { appendConstraint, focusWorkbenchSelection, kindCatalogs, kindCompatibility, workbenchSelection } = useBoardPlayShell();
	const constraintSections = useMemo<TreeDataSection[]>(
		() => [
			{
				actions: [boardTreeAddAction("board-play-constraint-add", appendConstraint, "Add constraint")],
				defaultOpen: true,
				id: "board-play-workbench-constraints",
				items: kindCompatibility.map((entry, index) =>
					boardTreeItem({
						description: entry.important ? "important" : "normal",
						id: `constraint:${index}`,
						label: boardPlayConstraintLabel(kindCatalogs, entry),
						onClick: () => focusWorkbenchSelection({ id: `constraint:${index}`, kind: "constraint" }),
						selected: workbenchSelection?.kind === "constraint" && workbenchSelection.id === `constraint:${index}`,
					}),
				),
				label: `Constraints (${kindCompatibility.length})`,
			},
		],
		[appendConstraint, focusWorkbenchSelection, kindCatalogs, kindCompatibility, workbenchSelection],
	);
	return <BoardSideTreePanel header="Constraints" icon={<Link2 className="size-4 shrink-0" />} sections={constraintSections} subtitle="Compatibility rules and priority" />;
}

function InspectorNodeKindDetails({ entry, patchKindCatalogs, renameKind }: { entry: BoardNodeKindCatalogEntry; patchKindCatalogs: (updater: (prev: BoardKindCatalogBundle) => BoardKindCatalogBundle) => void; renameKind: (previousId: string, nextId: string) => void }): ReactElement {
	const update = (patch: Partial<BoardNodeKindCatalogEntry>): void => {
		patchKindCatalogs((prev) => ({ ...prev, nodes: (prev.nodes ?? []).map((row) => (row.id === entry.id ? { ...row, ...patch } : row)) }));
	};
	return (
		<div className="border-element/60 space-y-3 border-l pl-2">
			<Label id="board-play.kind.node.id" label="Id"><Input className="h-7 font-mono text-xs" defaultValue={entry.id} key={entry.id} onBlur={(e) => renameKind(entry.id, e.currentTarget.value.trim() || entry.id)} /></Label>
			<Label id="board-play.kind.node.label" label="Label"><Input className="h-7 font-mono text-xs" value={entry.label} onChange={(e: ChangeEvent<HTMLInputElement>) => update({ label: e.target.value })} /></Label>
			<Label id="board-play.kind.node.defaultHandleKind" label="Default handle kind"><Input className="h-7 font-mono text-xs" value={entry.defaultHandleKind ?? ""} onChange={(e: ChangeEvent<HTMLInputElement>) => update({ defaultHandleKind: e.target.value || undefined })} /></Label>
			<Label id="board-play.kind.node.shape" label="Shape"><Select onValueChange={(value) => update({ shape: value === "__none__" ? undefined : (value as BoardNodeKindCatalogEntry["shape"]) })} value={entry.shape ?? "__none__"}><SelectTrigger className="h-7 font-mono text-xs"><SelectValue /></SelectTrigger><SelectContent><SelectItem value="__none__">Unassigned</SelectItem><SelectItem value="circle">circle</SelectItem><SelectItem value="rectangle">rectangle</SelectItem></SelectContent></Select></Label>
			<Label id="board-play.kind.node.color" label="Color"><Input className="h-7 font-mono text-xs" value={entry.color ?? ""} onChange={(e: ChangeEvent<HTMLInputElement>) => update({ color: e.target.value || undefined })} /></Label>
			<Label id="board-play.kind.node.stroke" label="Stroke"><Input className="h-7 font-mono text-xs" value={entry.stroke ?? ""} onChange={(e: ChangeEvent<HTMLInputElement>) => update({ stroke: e.target.value || undefined })} /></Label>
			<Label id="board-play.kind.node.icon" label="Icon"><Input className="h-7 font-mono text-xs" value={entry.icon ?? ""} onChange={(e: ChangeEvent<HTMLInputElement>) => update({ icon: e.target.value || undefined })} /></Label>
		</div>
	);
}

function InspectorEdgeKindDetails({ entry, patchKindCatalogs, renameKind }: { entry: BoardEdgeKindCatalogEntry; patchKindCatalogs: (updater: (prev: BoardKindCatalogBundle) => BoardKindCatalogBundle) => void; renameKind: (previousId: string, nextId: string) => void }): ReactElement {
	const update = (patch: Partial<BoardEdgeKindCatalogEntry>): void => {
		patchKindCatalogs((prev) => ({ ...prev, edges: (prev.edges ?? []).map((row) => (row.id === entry.id ? { ...row, ...patch } : row)) }));
	};
	return (
		<div className="border-element/60 space-y-3 border-l pl-2">
			<Label id="board-play.kind.edge.id" label="Id"><Input className="h-7 font-mono text-xs" defaultValue={entry.id} key={entry.id} onBlur={(e) => renameKind(entry.id, e.currentTarget.value.trim() || entry.id)} /></Label>
			<Label id="board-play.kind.edge.label" label="Label"><Input className="h-7 font-mono text-xs" value={entry.label} onChange={(e: ChangeEvent<HTMLInputElement>) => update({ label: e.target.value })} /></Label>
			<Label id="board-play.kind.edge.shape" label="Shape"><Select onValueChange={(value) => update({ shape: value === "__none__" ? undefined : (value as BoardEdgeKindCatalogEntry["shape"]) })} value={entry.shape ?? "__none__"}><SelectTrigger className="h-7 font-mono text-xs"><SelectValue /></SelectTrigger><SelectContent><SelectItem value="__none__">Unassigned</SelectItem><SelectItem value="bezier">bezier</SelectItem><SelectItem value="line">line</SelectItem></SelectContent></Select></Label>
			<Label id="board-play.kind.edge.color" label="Color"><Input className="h-7 font-mono text-xs" value={entry.color ?? ""} onChange={(e: ChangeEvent<HTMLInputElement>) => update({ color: e.target.value || undefined })} /></Label>
			<Label id="board-play.kind.edge.stroke" label="Stroke"><Input className="h-7 font-mono text-xs" value={entry.stroke ?? ""} onChange={(e: ChangeEvent<HTMLInputElement>) => update({ stroke: e.target.value || undefined })} /></Label>
			<Label id="board-play.kind.edge.pattern" label="Pattern"><Input className="h-7 font-mono text-xs" value={entry.pattern ?? ""} onChange={(e: ChangeEvent<HTMLInputElement>) => update({ pattern: e.target.value || undefined })} /></Label>
		</div>
	);
}

function InspectorWireKindDetails({ entry, kindCatalogs, patchKindCatalogs, renameKind }: { entry: BoardWireKindCatalogEntry; kindCatalogs: BoardKindCatalogBundle; patchKindCatalogs: (updater: (prev: BoardKindCatalogBundle) => BoardKindCatalogBundle) => void; renameKind: (previousId: string, nextId: string) => void }): ReactElement {
	const update = (patch: Partial<BoardWireKindCatalogEntry>): void => {
		patchKindCatalogs((prev) => ({ ...prev, wires: (prev.wires ?? []).map((row) => (row.id === entry.id ? { ...row, ...patch } : row)) }));
	};
	return (
		<div className="border-element/60 space-y-3 border-l pl-2">
			<Label id="board-play.kind.wire.id" label="Id"><Input className="h-7 font-mono text-xs" defaultValue={entry.id} key={entry.id} onBlur={(e) => renameKind(entry.id, e.currentTarget.value.trim() || entry.id)} /></Label>
			<Label id="board-play.kind.wire.label" label="Label"><Input className="h-7 font-mono text-xs" value={entry.label} onChange={(e: ChangeEvent<HTMLInputElement>) => update({ label: e.target.value })} /></Label>
			<Label id="board-play.kind.wire.defaultEdgeKind" label="Default edge kind"><Select onValueChange={(value) => update({ defaultEdgeKind: value === "__none__" ? undefined : value })} value={entry.defaultEdgeKind ?? "__none__"}><SelectTrigger className="h-7 font-mono text-xs"><SelectValue /></SelectTrigger><SelectContent><SelectItem value="__none__">Unassigned</SelectItem>{listEdgeKindIds(kindCatalogs).map((kindId) => (<SelectItem key={kindId} value={kindId}>{boardPlayKindLabel(kindCatalogs.edges, kindId)}</SelectItem>))}</SelectContent></Select></Label>
		</div>
	);
}

function InspectorConstraintDetails({ entry, index, kindCatalogs, setKindCompatibility }: { entry: BoardKindCompatEntry; index: number; kindCatalogs: BoardKindCatalogBundle; setKindCompatibility: (value: BoardKindCompatEntry[] | ((prev: BoardKindCompatEntry[]) => BoardKindCompatEntry[])) => void }): ReactElement {
	const update = (patch: Partial<BoardKindCompatEntry>): void => {
		setKindCompatibility((prev) => prev.map((row, rowIndex) => (rowIndex === index ? { ...row, ...patch } : row)));
	};
	return (
		<div className="border-element/60 space-y-3 border-l pl-2">
			<Label id="board-play.constraint.source" label="Source handle kind"><Select onValueChange={(value) => update({ source: value })} value={entry.source}><SelectTrigger className="h-7 font-mono text-xs"><SelectValue /></SelectTrigger><SelectContent>{listHandleKindIds(kindCatalogs).map((kindId) => (<SelectItem key={`constraint-source-${kindId}`} value={kindId}>{boardPlayKindLabel(kindCatalogs.handles, kindId)}</SelectItem>))}</SelectContent></Select></Label>
			<Label id="board-play.constraint.target" label="Target handle kind"><Select onValueChange={(value) => update({ target: value })} value={entry.target}><SelectTrigger className="h-7 font-mono text-xs"><SelectValue /></SelectTrigger><SelectContent>{listHandleKindIds(kindCatalogs).map((kindId) => (<SelectItem key={`constraint-target-${kindId}`} value={kindId}>{boardPlayKindLabel(kindCatalogs.handles, kindId)}</SelectItem>))}</SelectContent></Select></Label>
			<Label id="board-play.constraint.specificity" label="Specificity"><Select onValueChange={(value) => update({ specificity: value === "__none__" ? undefined : (value as BoardKindCompatEntry["specificity"]) })} value={entry.specificity ?? "__none__"}><SelectTrigger className="h-7 font-mono text-xs"><SelectValue /></SelectTrigger><SelectContent><SelectItem value="__none__">general</SelectItem><SelectItem value="general">general</SelectItem><SelectItem value="node">node</SelectItem><SelectItem value="edge">edge</SelectItem><SelectItem value="handle">handle</SelectItem><SelectItem value="wire">wire</SelectItem></SelectContent></Select></Label>
			<div className="grid grid-cols-2 gap-2">
				<label className="text-muted-foreground flex items-center gap-2 text-[11px]"><input checked={entry.bidirectional === true} className="accent-accent size-3.5" onChange={(e) => update({ bidirectional: e.target.checked || undefined })} type="checkbox" /><span>Bidirectional (--)</span></label>
				<label className="text-muted-foreground flex items-center gap-2 text-[11px]"><input checked={entry.important === true} className="accent-accent size-3.5" onChange={(e) => update({ important: e.target.checked || undefined })} type="checkbox" /><span>Important</span></label>
			</div>
		</div>
	);
}

/** @emoji 🔎 Sketchpad-style detail inspector for graph selection, kind rows, and compatibility entries. */
function BoardSelectionInspectorPanel(): ReactElement {
	const {
		activePaneId,
		deleteWorkbenchSelection,
		fixture,
		kindCatalogs,
		kindCompatibility,
		lockedIds,
		patchFixture,
		patchKindCatalogs,
		patchWires,
		remapIdInSelections,
		selectionByPane,
		setGraphObjectsHidden,
		setGraphObjectsLocked,
		setKindCompatibility,
		setWorkbenchSelection,
		wires,
		workbenchSelection,
	} = useBoardPlayShell();
	const renameNodeKind = useCallback((previousId: string, nextId: string) => {
		if (!previousId || !nextId || previousId === nextId) {
			return;
		}
		patchKindCatalogs((prev) => ({ ...prev, nodes: (prev.nodes ?? []).map((row) => (row.id === previousId ? { ...row, id: nextId } : row)) }));
		patchFixture((prev) => ({ ...prev, nodes: prev.nodes.map((node) => (node.nodeKind === previousId ? { ...node, nodeKind: nextId } : node)) }));
		setWorkbenchSelection((selection) => (selection?.kind === "node-kind" && selection.id === previousId ? { ...selection, id: nextId } : selection));
	}, [patchFixture, patchKindCatalogs, setWorkbenchSelection]);
	const renameEdgeKind = useCallback((previousId: string, nextId: string) => {
		if (!previousId || !nextId || previousId === nextId) {
			return;
		}
		patchKindCatalogs((prev) => ({
			...prev,
			edges: (prev.edges ?? []).map((row) => (row.id === previousId ? { ...row, id: nextId } : row)),
			wires: (prev.wires ?? []).map((row) => (row.defaultEdgeKind === previousId ? { ...row, defaultEdgeKind: nextId } : row)),
		}));
		patchFixture((prev) => ({ ...prev, edges: prev.edges.map((edge) => (edge.edgeKind === previousId ? { ...edge, edgeKind: nextId } : edge)) }));
		setWorkbenchSelection((selection) => (selection?.kind === "edge-kind" && selection.id === previousId ? { ...selection, id: nextId } : selection));
	}, [patchFixture, patchKindCatalogs, setWorkbenchSelection]);
	const renameWireKind = useCallback((previousId: string, nextId: string) => {
		if (!previousId || !nextId || previousId === nextId) {
			return;
		}
		patchKindCatalogs((prev) => ({
			...prev,
			handles: (prev.handles ?? []).map((row) => (row.defaultWireKind === previousId ? { ...row, defaultWireKind: nextId } : row)),
			wires: (prev.wires ?? []).map((row) => (row.id === previousId ? { ...row, id: nextId } : row)),
		}));
		patchWires((prev) => prev.map((wire) => (wire.wireKind === previousId ? { ...wire, wireKind: nextId } : wire)));
		setWorkbenchSelection((selection) => (selection?.kind === "wire-kind" && selection.id === previousId ? { ...selection, id: nextId } : selection));
	}, [patchKindCatalogs, patchWires, setWorkbenchSelection]);
	const ids = useMemo(() => [...selectionByPane[activePaneId]].sort((a, b) => a.localeCompare(b)), [activePaneId, selectionByPane]);

	const { edgeIds, handleIds, nodeIds, wireIds } = useMemo(() => {
		const nodeIds: string[] = [];
		const handleIds: string[] = [];
		const edgeIds: string[] = [];
		const wireIds: string[] = [];
		for (const id of ids) {
			if (findNode(fixture, id)) {
				nodeIds.push(id);
			} else if (findEdge(fixture, id)) {
				edgeIds.push(id);
			} else if (findHandleOwner(fixture, id)) {
				handleIds.push(id);
			} else if (findWire(wires, id)) {
				wireIds.push(id);
			}
		}
		return { edgeIds, handleIds, nodeIds, wireIds };
	}, [fixture, ids, wires]);

	const treeSections = useMemo<TreeDataSection[]>(() => {
		if (workbenchSelection) {
			if (workbenchSelection.kind === "node-kind") {
				const entry = (kindCatalogs.nodes ?? []).find((row) => row.id === workbenchSelection.id);
				if (entry) {
					return [{ content: <InspectorNodeKindDetails entry={entry} patchKindCatalogs={patchKindCatalogs} renameKind={renameNodeKind} />, defaultOpen: true, id: "board-play-inspector-node-kind", label: `Node kind · ${entry.label || entry.id}` }];
				}
			}
			if (workbenchSelection.kind === "edge-kind") {
				const entry = (kindCatalogs.edges ?? []).find((row) => row.id === workbenchSelection.id);
				if (entry) {
					return [{ content: <InspectorEdgeKindDetails entry={entry} patchKindCatalogs={patchKindCatalogs} renameKind={renameEdgeKind} />, defaultOpen: true, id: "board-play-inspector-edge-kind", label: `Edge kind · ${entry.label || entry.id}` }];
				}
			}
			if (workbenchSelection.kind === "wire-kind") {
				const entry = (kindCatalogs.wires ?? []).find((row) => row.id === workbenchSelection.id);
				if (entry) {
					return [{ content: <InspectorWireKindDetails entry={entry} kindCatalogs={kindCatalogs} patchKindCatalogs={patchKindCatalogs} renameKind={renameWireKind} />, defaultOpen: true, id: "board-play-inspector-wire-kind", label: `Wire kind · ${entry.label || entry.id}` }];
				}
			}
			if (workbenchSelection.kind === "constraint") {
				const index = Number(workbenchSelection.id.split(":")[1] ?? "-1");
				const entry = kindCompatibility[index];
				if (entry) {
					return [{ content: <InspectorConstraintDetails entry={entry} index={index} kindCatalogs={kindCatalogs} setKindCompatibility={setKindCompatibility} />, defaultOpen: true, id: "board-play-inspector-constraint", label: boardPlayConstraintLabel(kindCatalogs, entry) }];
				}
			}
		}
		if (ids.length === 0) {
			return [{ content: <p className="text-muted-foreground px-1 py-2 text-xs">No selection. Click the graph or choose a workbench row.</p>, id: "board-play-inspector-empty", label: null }];
		}
		const sections: TreeDataSection[] = [];
		if (nodeIds.length > 0) {
			sections.push({ content: <InspectorNodeBatch fixture={fixture} kindCatalogs={kindCatalogs} lockedIds={lockedIds} nodeIds={nodeIds} patchFixture={patchFixture} remapIdInSelections={remapIdInSelections} setGraphObjectsHidden={setGraphObjectsHidden} setGraphObjectsLocked={setGraphObjectsLocked} />, defaultOpen: true, id: "board-play-inspector-nodes", label: `Nodes (${nodeIds.length})` });
		}
		if (handleIds.length > 0) {
			sections.push({ content: <InspectorHandleBatch fixture={fixture} handleIds={handleIds} kindCatalogs={kindCatalogs} lockedIds={lockedIds} patchFixture={patchFixture} remapIdInSelections={remapIdInSelections} setGraphObjectsHidden={setGraphObjectsHidden} setGraphObjectsLocked={setGraphObjectsLocked} />, defaultOpen: true, id: "board-play-inspector-handles", label: `Handles (${handleIds.length})` });
		}
		if (edgeIds.length > 0) {
			sections.push({ content: <InspectorEdgeBatch edgeIds={edgeIds} fixture={fixture} kindCatalogs={kindCatalogs} lockedIds={lockedIds} patchFixture={patchFixture} remapIdInSelections={remapIdInSelections} setGraphObjectsHidden={setGraphObjectsHidden} setGraphObjectsLocked={setGraphObjectsLocked} />, defaultOpen: true, id: "board-play-inspector-edges", label: `Edges (${edgeIds.length})` });
		}
		if (wireIds.length > 0) {
			sections.push({ content: <InspectorWireBatch fixture={fixture} kindCatalogs={kindCatalogs} lockedIds={lockedIds} patchWires={patchWires} remapIdInSelections={remapIdInSelections} setGraphObjectsHidden={setGraphObjectsHidden} setGraphObjectsLocked={setGraphObjectsLocked} wireIds={wireIds} wires={wires} />, defaultOpen: true, id: "board-play-inspector-wires", label: `Wires (${wireIds.length})` });
		}
		if (sections.length === 0) {
			sections.push({ content: <div className="px-1 py-2 font-mono text-xs" style={{ color: "var(--warning-foreground)" }}>Unknown ids: {ids.join(", ")}</div>, id: "board-play-inspector-unknown", label: "Selection" });
		}
		return sections;
	}, [deleteWorkbenchSelection, edgeIds, fixture, handleIds, ids, kindCatalogs, kindCompatibility, lockedIds, nodeIds, patchFixture, patchKindCatalogs, patchWires, remapIdInSelections, renameEdgeKind, renameNodeKind, renameWireKind, setGraphObjectsHidden, setGraphObjectsLocked, setKindCompatibility, wireIds, wires, workbenchSelection]);

	return (
		<div className="flex h-full min-h-0 flex-col gap-2 p-3 text-xs">
			<div className="text-muted-foreground flex shrink-0 items-center gap-2 border-b border-element pb-2">
				<ClipboardList className="size-4 shrink-0" />
				<div className="min-w-0 flex-1">
					<div className="font-semibold uppercase tracking-wide">Detail</div>
					<div className="text-[11px] opacity-80">pane: {activePaneId}</div>
				</div>
				{workbenchSelection ? <Button className="h-7 px-2 text-[11px]" onClick={deleteWorkbenchSelection} type="button" variant="outline">Clear row</Button> : null}
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

const BOARD_PLAY_LOD_FOLLOW_VALUE = "__follow_zoom__";

const BOARD_PLAY_LOD_TIERS: BoardDrawLodKind[] = ["minimap", "overview", "normal", "detail", "micro"];

function boardPlayLodTierMenuLabel(tier: BoardDrawLodKind): string {
	return tier.charAt(0).toUpperCase() + tier.slice(1);
}

function boardWindowKindsWithPlayMeasures(
	redrawZoomEnabledByPane: Record<BoardPlayPaneId, boolean>,
	setRedrawZoomEnabledByPane: (updater: (prev: Record<BoardPlayPaneId, boolean>) => Record<BoardPlayPaneId, boolean>) => void,
	automaticLodByPane: Record<BoardPlayPaneId, boolean>,
	setAutomaticLodForPane: (pane: BoardPlayPaneId, value: boolean) => void,
	pinnedLodByPane: Record<BoardPlayPaneId, BoardDrawLodKind | undefined>,
	setPinnedLodForPane: (pane: BoardPlayPaneId, value: BoardDrawLodKind | undefined) => void,
): UIWindowKindDefinition[] {
	const measuresForPane = (paneId: BoardPlayPaneId): UIWindowKindDefinition["measures"] => {
		const measures: UIWindowKindDefinition["measures"] = [
			{
				icon: <ZoomIn className="size-small" />,
				id: `${paneId}-redraw-interactive-zoom`,
				kind: "toggle",
				label: "Redraw zoom",
				pressed: redrawZoomEnabledByPane[paneId],
				onPressedChange: (pressed) => {
					setRedrawZoomEnabledByPane((prev) => ({ ...prev, [paneId]: pressed }));
				},
			},
			{
				icon: <Layers className="size-small" />,
				id: `${paneId}-automatic-lod`,
				kind: "toggle",
				label: "Automatic LOD",
				pressed: automaticLodByPane[paneId],
				onPressedChange: (pressed) => {
					setAutomaticLodForPane(paneId, pressed);
				},
			},
		];
		if (!automaticLodByPane[paneId]) {
			const pin = pinnedLodByPane[paneId];
			measures.push({
				id: `${paneId}-lod-tier`,
				items: [
					{ id: BOARD_PLAY_LOD_FOLLOW_VALUE, label: "Follow zoom", value: BOARD_PLAY_LOD_FOLLOW_VALUE },
					...BOARD_PLAY_LOD_TIERS.map((tier) => ({
						id: tier,
						label: boardPlayLodTierMenuLabel(tier),
						value: tier,
					})),
				],
				kind: "select",
				label: "LOD",
				onValueChange: (value) => {
					setPinnedLodForPane(
						paneId,
						value === BOARD_PLAY_LOD_FOLLOW_VALUE ? undefined : (value as BoardDrawLodKind),
					);
				},
				value: pin === undefined ? BOARD_PLAY_LOD_FOLLOW_VALUE : pin,
			});
		}
		return measures;
	};
	return [
		{ component: BoardOverviewPane, id: "board-overview", label: "Overview", measures: measuresForPane("board-overview") },
		{ component: BoardDetailPane, id: "board-detail", label: "Zoom", measures: measuresForPane("board-detail") },
		{ component: BoardSelectionPane, id: "board-selection", label: "Selection", measures: measuresForPane("board-selection") },
	];
}

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
				<SelectTrigger className="h-medium w-30" id="board-play-surface-theme" size="sm">
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
				<SelectTrigger className="h-medium w-30" id="board-play-surface-device" size="sm">
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
				<SelectTrigger className="h-medium w-30" id="board-play-surface-expertise" size="sm">
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

interface BoardPlayRedrawLoopSnapshot {
	activePaneId: BoardPlayPaneId;
	boardRedrawHandlesAfterNodes: boolean;
	boardRedrawProgressiveAutoStopMs: number;
	boardRedrawProgressiveEnabled: boolean;
	boardRedrawPlayMaxItersPerFrame: number;
	camerasByPane: Record<BoardPlayPaneId, CameraState>;
	forceLayoutGravity: number;
	forceLayoutIdealEdgeLength: number;
	forceLayoutRepulsionStrength: number;
	lockedNodeIds: string[];
	mode: BoardRedrawModeKind;
	treeLayoutDirection: BoardHierarchicalTreeDirectionKind;
	treeLayoutLayerSpacing: number;
	treeLayoutSiblingGap: number;
}

// #region 🔖Entrypoint
const initialDocument = parseBoardPlayDocument(nakaginFixtureJson as unknown);
const initialFixture = initialDocument.fixture;

function BoardPlayInner(): ReactElement {
	const [fixture, setFixtureState] = useState<BoardFixtureV1>(initialFixture);
	const fixtureRef = useRef<BoardFixtureV1>(fixture);
	fixtureRef.current = fixture;
	const [wires, setWiresState] = useState<BoardPlayWireRecord[]>(initialDocument.wires);
	const [kindCatalogs, setKindCatalogsState] = useState<BoardKindCatalogBundle>(initialDocument.kindCatalogs);
	const [kindCompatibility, setKindCompatibilityState] = useState<BoardKindCompatEntry[]>(initialDocument.kindCompatibility);
	const [lockedIdList, setLockedIdList] = useState<string[]>(initialDocument.lockedIds);
	const lockedIds = useMemo(() => new Set(lockedIdList), [lockedIdList]);
	const [workbenchSelection, setWorkbenchSelection] = useState<BoardWorkbenchSelection | null>(null);
	const [boardPlayPaneCamerasBaseline, setBoardPlayPaneCamerasBaseline] = useState<
		Record<BoardPlayPaneId, CameraState>
	>(() => triptychCamerasFromFixture(initialFixture));
	const boardPlayPaneCamerasBaselineRef = useRef(boardPlayPaneCamerasBaseline);
	boardPlayPaneCamerasBaselineRef.current = boardPlayPaneCamerasBaseline;
	const [activePaneId, setActivePaneId] = useState<BoardPlayPaneId>("board-overview");
	const activePaneIdRef = useRef(activePaneId);
	activePaneIdRef.current = activePaneId;
	const [selectionByPane, setSelectionByPane] = useState<Record<BoardPlayPaneId, Set<string>>>(() => selectionSeedForFixture(initialFixture));
	const [theme, setTheme] = useState<ElementsSurfaceTheme>(readTheme);
	const [device, setDevice] = useState<ElementsSurfaceDevice>(readDevice);
	const [expertise, setExpertise] = useState<Expertise>(readExpertise);
	const { mobile } = useElementsSurfaceChrome({ theme, device, expertise });
	const [boardSelectionMethod, setBoardSelectionMethod] = useState<BoardSelectionMethod>("rectangle");
	const [boardSelectionMode, setBoardSelectionMode] = useState<BoardSelectionMode>("replace");
	const [boardSelectionGestureHighlight, setBoardSelectionGestureHighlight] = useState<BoardSelectionMode | null>(null);
	const [boardSelectionTargets, setBoardSelectionTargets] = useState<BoardSelectionTargets>(() => ({ ...BOARD_SELECTION_TARGETS_DEFAULT }));
	const [boardGridSnapEnabled, setBoardGridSnapEnabled] = useState(false);
	const [boardAutomaticLodByPane, setBoardAutomaticLodByPane] = useState<Record<BoardPlayPaneId, boolean>>({
		"board-detail": true,
		"board-overview": true,
		"board-selection": true,
	});
	const [boardPinnedLodByPane, setBoardPinnedLodByPane] = useState<
		Record<BoardPlayPaneId, BoardDrawLodKind | undefined>
	>({
		"board-detail": undefined,
		"board-overview": undefined,
		"board-selection": undefined,
	});
	const [boardRedrawPlaying, setBoardRedrawPlaying] = useState(false);
	const [forceLayoutFullIterations, setForceLayoutFullIterations] = useState(200);
	const [forceLayoutIdealEdgeLength, setForceLayoutIdealEdgeLength] = useState(64);
	const [forceLayoutGravity, setForceLayoutGravity] = useState(0.012);
	const [forceLayoutRepulsionStrength, setForceLayoutRepulsionStrength] = useState(80);
	const [boardRedrawPlayMaxItersPerFrame, setBoardRedrawPlayMaxItersPerFrame] = useState(96);
	const [boardRedrawProgressiveEnabled, setBoardRedrawProgressiveEnabled] = useState(true);
	const [boardRedrawProgressiveAutoStopMs, setBoardRedrawProgressiveAutoStopMs] = useState(3000);
	const [boardRedrawMode, setBoardRedrawMode] = useState<BoardRedrawModeKind>("force-graph");
	const [boardRedrawHandlesAfterNodes, setBoardRedrawHandlesAfterNodes] = useState(false);
	const [boardRedrawInteractiveZoomByPane, setBoardRedrawInteractiveZoomByPane] = useState<Record<BoardPlayPaneId, boolean>>({
		"board-detail": false,
		"board-overview": false,
		"board-selection": false,
	});
	const boardRedrawInteractiveZoomByPaneRef = useRef(boardRedrawInteractiveZoomByPane);
	boardRedrawInteractiveZoomByPaneRef.current = boardRedrawInteractiveZoomByPane;
	const [treeLayoutLayerSpacing, setTreeLayoutLayerSpacing] = useState(120);
	const [treeLayoutSiblingGap, setTreeLayoutSiblingGap] = useState(28);
	const [treeLayoutDirection, setTreeLayoutDirection] = useState<BoardHierarchicalTreeDirectionKind>("downwards");

	const boardRedrawPlayingRef = useRef(boardRedrawPlaying);
	boardRedrawPlayingRef.current = boardRedrawPlaying;

	useEffect(() => {
		setBoardSelectionGestureHighlight(null);
	}, [activePaneId]);

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
		setBoardPlayPaneCamerasBaseline(triptychCamerasFromFixture(next));
		setWorkbenchSelection(null);
	}, []);

	const patchFixture = useCallback((updater: (prev: BoardFixtureV1) => BoardFixtureV1) => {
		setFixtureState((prev) => updater(prev));
	}, []);

	const patchWires = useCallback((updater: (prev: BoardPlayWireRecord[]) => BoardPlayWireRecord[]) => {
		setWiresState((prev) => updater(prev));
	}, []);

	const patchKindCatalogs = useCallback((updater: (prev: BoardKindCatalogBundle) => BoardKindCatalogBundle) => {
		setKindCatalogsState((prev) => updater(prev));
	}, []);

	const setLockedIds = useCallback((value: string[] | ((prev: string[]) => string[])) => {
		setLockedIdList((prev) => uniqueSortedStrings(typeof value === "function" ? value(prev) : value));
	}, []);

	const setKindCompatibility = useCallback((value: BoardKindCompatEntry[] | ((prev: BoardKindCompatEntry[]) => BoardKindCompatEntry[])) => {
		setKindCompatibilityState((prev) => (typeof value === "function" ? value(prev) : value));
	}, []);

	const setSelectionForPane = useCallback((pane: BoardPlayPaneId, ids: readonly string[]) => {
		setSelectionByPane((prev) => ({ ...prev, [pane]: new Set(ids) }));
	}, []);

	const focusGraphSelection = useCallback(
		(ids: readonly string[]) => {
			setSelectionForPane(activePaneIdRef.current, ids);
			setWorkbenchSelection(null);
		},
		[setSelectionForPane],
	);

	const focusWorkbenchSelection = useCallback((value: BoardWorkbenchSelection) => {
		setWorkbenchSelection(value);
	}, []);

	const handleCanvasFixtureDrop = useCallback((pane: BoardPlayPaneId, detail: BoardFixtureDropDetail) => {
		skipNextCameraBasisResyncRef.current = true;
		const merged = mergePaletteNodeFromDrop(detail);
		if (merged) {
			patchFixture((prev) => ({ ...prev, nodes: [...prev.nodes, merged] }));
			setSelectionForPane(pane, [merged.id]);
			setWorkbenchSelection(null);
			return;
		}
		const dropped = parseBoardPlayDocument(detail.fixture);
		setFixtureState(dropped.fixture);
		setWiresState(dropped.wires);
		setKindCatalogsState(dropped.kindCatalogs);
		setKindCompatibilityState(dropped.kindCompatibility);
		setLockedIdList(dropped.lockedIds);
		setSelectionByPane(selectionSeedForFixture(dropped.fixture));
		setBoardPlayPaneCamerasBaseline(triptychCamerasFromFixture(dropped.fixture));
		setWorkbenchSelection(null);
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

	const setGraphObjectsHidden = useCallback((ids: readonly string[], hidden: boolean) => {
		const target = new Set(ids);
		patchFixture((prev) => ({
			...prev,
			edges: prev.edges.map((edge) => (target.has(edge.id) ? { ...edge, ...(hidden ? { hidden: true } : { hidden: undefined }) } : edge)),
			nodes: prev.nodes.map((node) => ({
				...node,
				...(target.has(node.id) ? (hidden ? { hidden: true } : { hidden: undefined }) : {}),
				handles: node.handles.map((handle) => (target.has(handle.id) ? { ...handle, ...(hidden ? { hidden: true } : { hidden: undefined }) } : handle)),
			})),
		}));
		patchWires((prev) => prev.map((wire) => (target.has(wire.id) ? { ...wire, ...(hidden ? { hidden: true } : { hidden: undefined }) } : wire)));
	}, [patchFixture, patchWires]);

	const setGraphObjectsLocked = useCallback((ids: readonly string[], locked: boolean) => {
		const target = new Set(ids);
		setLockedIds((prev) => (locked ? uniqueSortedStrings([...prev, ...ids]) : prev.filter((id) => !target.has(id))));
	}, [setLockedIds]);

	const deleteGraphObjects = useCallback((ids: readonly string[]) => {
		const expandedIds = expandDeletedGraphIds(fixtureRef.current, wires, ids);
		const remove = new Set(expandedIds);
		patchFixture((prev) => ({
			...prev,
			edges: prev.edges.filter((edge) => !remove.has(edge.id) && !remove.has(edge.source) && !remove.has(edge.target)),
			nodes: prev.nodes
				.filter((node) => !remove.has(node.id))
				.map((node) => ({ ...node, handles: node.handles.filter((handle) => !remove.has(handle.id)) })),
		}));
		patchWires((prev) => prev.filter((wire) => !remove.has(wire.id) && !remove.has(wire.source) && !(wire.target && remove.has(wire.target))));
		setLockedIds((prev) => prev.filter((id) => !remove.has(id)));
		setSelectionByPane((prev) => ({
			"board-detail": new Set([...prev["board-detail"]].filter((id) => !remove.has(id))),
			"board-overview": new Set([...prev["board-overview"]].filter((id) => !remove.has(id))),
			"board-selection": new Set([...prev["board-selection"]].filter((id) => !remove.has(id))),
		}));
		setWorkbenchSelection((prev) => (prev && remove.has(prev.id) ? null : prev));
	}, [patchFixture, patchWires, setLockedIds, wires]);

	const appendKind = useCallback((kind: "edge-kind" | "node-kind" | "wire-kind") => {
		const nextId = newBoardAuthoringId(kind);
		if (kind === "node-kind") {
			setKindCatalogsState((prev) => ({ ...prev, nodes: [...(prev.nodes ?? []), { id: nextId, label: nextId }] }));
			setWorkbenchSelection({ id: nextId, kind });
			return;
		}
		if (kind === "edge-kind") {
			setKindCatalogsState((prev) => ({ ...prev, edges: [...(prev.edges ?? []), { id: nextId, label: nextId }] }));
			setWorkbenchSelection({ id: nextId, kind });
			return;
		}
		setKindCatalogsState((prev) => ({ ...prev, wires: [...(prev.wires ?? []), { id: nextId, label: nextId }] }));
		setWorkbenchSelection({ id: nextId, kind });
	}, []);

	const appendConstraint = useCallback(() => {
		const source = kindCatalogs.handles?.[0]?.id ?? BOARD_BUILTIN_PORT_HANDLE_KIND;
		const target = kindCatalogs.handles?.[0]?.id ?? BOARD_BUILTIN_PORT_HANDLE_KIND;
		setKindCompatibilityState((prev) => [...prev, { source, target }]);
		setWorkbenchSelection({ id: `constraint:${kindCompatibility.length}`, kind: "constraint" });
	}, [kindCatalogs.handles, kindCompatibility.length]);

	const deleteWorkbenchSelection = useCallback(() => {
		setWorkbenchSelection((selection) => {
			if (!selection) {
				return selection;
			}
			if (selection.kind === "constraint") {
				const index = Number(selection.id.split(":")[1] ?? "-1");
				if (index >= 0) {
					setKindCompatibilityState((prev) => prev.filter((_, rowIndex) => rowIndex !== index));
				}
				return null;
			}
			if (selection.kind === "node-kind") {
				patchFixture((prev) => ({ ...prev, nodes: prev.nodes.map((node) => (node.nodeKind === selection.id ? { ...node, nodeKind: undefined } : node)) }));
			}
			if (selection.kind === "edge-kind") {
				patchFixture((prev) => ({ ...prev, edges: prev.edges.map((edge) => (edge.edgeKind === selection.id ? { ...edge, edgeKind: undefined } : edge)) }));
				patchKindCatalogs((prev) => ({ ...prev, wires: (prev.wires ?? []).map((row) => (row.defaultEdgeKind === selection.id ? { ...row, defaultEdgeKind: undefined } : row)) }));
			}
			if (selection.kind === "wire-kind") {
				patchWires((prev) => prev.map((wire) => (wire.wireKind === selection.id ? { ...wire, wireKind: undefined } : wire)));
				patchKindCatalogs((prev) => ({ ...prev, handles: (prev.handles ?? []).map((row) => (row.defaultWireKind === selection.id ? { ...row, defaultWireKind: undefined } : row)) }));
			}
			setKindCatalogsState((prev) => ({
				...prev,
				edges: selection.kind === "edge-kind" ? (prev.edges ?? []).filter((row) => row.id !== selection.id) : prev.edges,
				nodes: selection.kind === "node-kind" ? (prev.nodes ?? []).filter((row) => row.id !== selection.id) : prev.nodes,
				wires: selection.kind === "wire-kind" ? (prev.wires ?? []).filter((row) => row.id !== selection.id) : prev.wires,
			}));
			return null;
		});
	}, [patchFixture, patchKindCatalogs, patchWires]);

	const cameraBasisFixtureRef = useRef<BoardFixtureV1>(fixture);
	/** @emoji 📌 One-shot: sync {@link cameraBasisFixtureRef} without resetting {@link boardPlayPaneCamerasBaseline} after palette / shelf fixture drop. */
	const skipNextCameraBasisResyncRef = useRef(false);
	const prevBoardRedrawPlayingRef = useRef(false);
	const [cameraDisplayOverrideByPane, setCameraDisplayOverrideByPane] = useState<Record<BoardPlayPaneId, CameraState> | null>(null);
	const cameraDisplayOverrideRef = useRef<Record<BoardPlayPaneId, CameraState> | null>(null);
	cameraDisplayOverrideRef.current = cameraDisplayOverrideByPane;
	const suppressCameraBasisSyncRef = useRef(false);
	const cameraPlayEndAnimRafRef = useRef<number | null>(null);
	const boardPlayNodesRedrawCameraAnimRafRef = useRef<number | null>(null);
	const boardPlayRedrawCameraChaseRef = useRef<Record<BoardPlayPaneId, CameraState> | null>(null);
	const lastPlayingForCameraEaseRef = useRef(false);
	const [nodesRedrawCameraEaseTick, setNodesRedrawCameraEaseTick] = useState(0);
	/** @emoji 📷 Cameras shown on canvases at click time; set before {@link patchFixture} so `from` cannot lag one commit behind the graph. */
	const nodesRedrawEaseFromRef = useRef<Record<BoardPlayPaneId, CameraState> | null>(null);
	/** @emoji 🔢 Bumped on each redraw click / competing camera path so stale RAF ticks never call {@link setBoardPlayPaneCamerasBaseline}. */
	const nodesRedrawEaseGenerationRef = useRef(0);

	const syncBaselineFromViewportCamera = useCallback((cam: CameraState) => {
		if (boardRedrawPlayingRef.current) {
			return;
		}
		if (suppressCameraBasisSyncRef.current) {
			return;
		}
		if (cameraDisplayOverrideRef.current !== null) {
			return;
		}
		const c = { x: cam.x, y: cam.y, zoom: cam.zoom };
		setBoardPlayPaneCamerasBaseline((prev) => {
			const pane = activePaneIdRef.current;
			const p = prev[pane];
			if (
				Math.abs(p.x - c.x) < 1e-6 &&
				Math.abs(p.y - c.y) < 1e-6 &&
				Math.abs(p.zoom - c.zoom) < 1e-9
			) {
				return prev;
			}
			return { ...prev, [pane]: { ...c } };
		});
	}, []);

	useEffect(() => {
		if (boardRedrawPlaying) {
			return;
		}
		if (suppressCameraBasisSyncRef.current) {
			return;
		}
		if (skipNextCameraBasisResyncRef.current) {
			skipNextCameraBasisResyncRef.current = false;
			cameraBasisFixtureRef.current = fixture;
			return;
		}
		cameraBasisFixtureRef.current = fixture;
	}, [fixture, boardRedrawPlaying]);

	useEffect(() => {
		const prevPlaying = prevBoardRedrawPlayingRef.current;
		const playJustStarted = boardRedrawPlaying && !prevPlaying;

		if (playJustStarted) {
			nodesRedrawEaseGenerationRef.current += 1;
			nodesRedrawEaseFromRef.current = null;
			if (cameraPlayEndAnimRafRef.current != null) {
				cancelAnimationFrame(cameraPlayEndAnimRafRef.current);
				cameraPlayEndAnimRafRef.current = null;
			}
			if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
				cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
				boardPlayNodesRedrawCameraAnimRafRef.current = null;
			}
			setCameraDisplayOverrideByPane(null);
			suppressCameraBasisSyncRef.current = false;
			cameraBasisFixtureRef.current = fixture;
			const prevCam = boardPlayPaneCamerasBaselineRef.current;
			boardPlayRedrawCameraChaseRef.current = {
				"board-detail": { ...prevCam["board-detail"] },
				"board-overview": { ...prevCam["board-overview"] },
				"board-selection": { ...prevCam["board-selection"] },
			};
		} else if (!suppressCameraBasisSyncRef.current) {
			if (cameraPlayEndAnimRafRef.current != null) {
				cancelAnimationFrame(cameraPlayEndAnimRafRef.current);
				cameraPlayEndAnimRafRef.current = null;
			}
		}
		prevBoardRedrawPlayingRef.current = boardRedrawPlaying;
	}, [boardRedrawPlaying, fixture]);

	useEffect(() => {
		if (!boardRedrawPlaying) {
			boardPlayRedrawCameraChaseRef.current = null;
			return;
		}
		if (suppressCameraBasisSyncRef.current) {
			return;
		}
		const pane = activePaneIdRef.current;
		if (!boardRedrawInteractiveZoomByPaneRef.current[pane]) {
			boardPlayRedrawCameraChaseRef.current = null;
			return;
		}
		const target = triptychCamerasFromFixture(fixture);
		setBoardPlayPaneCamerasBaseline((baselinePrev) => {
			const prevChase = boardPlayRedrawCameraChaseRef.current ?? baselinePrev;
			const damped = dampCameraStateLinear(
				prevChase[pane],
				target[pane],
				BOARD_PLAY_REDRAW_CAMERA_CHASE_BLEND,
			);
			const nextChase: Record<BoardPlayPaneId, CameraState> = {
				"board-detail": { ...prevChase["board-detail"] },
				"board-overview": { ...prevChase["board-overview"] },
				"board-selection": { ...prevChase["board-selection"] },
			};
			nextChase[pane] = damped;
			boardPlayRedrawCameraChaseRef.current = nextChase;
			return nextChase;
		});
	}, [boardRedrawInteractiveZoomByPane, boardRedrawPlaying, fixture]);

	useEffect(() => {
		if (boardRedrawPlaying) {
			lastPlayingForCameraEaseRef.current = true;
			return () => {
				if (cameraPlayEndAnimRafRef.current != null) {
					cancelAnimationFrame(cameraPlayEndAnimRafRef.current);
					cameraPlayEndAnimRafRef.current = null;
				}
				if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
					cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
					boardPlayNodesRedrawCameraAnimRafRef.current = null;
				}
			};
		}
		if (!lastPlayingForCameraEaseRef.current) {
			return;
		}
		lastPlayingForCameraEaseRef.current = false;
		const postPlayEasePaneId = activePaneIdRef.current;
		if (!boardRedrawInteractiveZoomByPaneRef.current[postPlayEasePaneId]) {
			suppressCameraBasisSyncRef.current = false;
			setCameraDisplayOverrideByPane(null);
			return;
		}

		const snapshotFixture = fixtureRef.current;
		const from: Record<BoardPlayPaneId, CameraState> = {
			"board-detail": { ...boardPlayPaneCamerasBaseline["board-detail"] },
			"board-overview": { ...boardPlayPaneCamerasBaseline["board-overview"] },
			"board-selection": { ...boardPlayPaneCamerasBaseline["board-selection"] },
		};
		cameraBasisFixtureRef.current = snapshotFixture;
		const to = triptychCamerasFromFixture(snapshotFixture);
		suppressCameraBasisSyncRef.current = true;
		if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
			cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
			boardPlayNodesRedrawCameraAnimRafRef.current = null;
		}
		nodesRedrawEaseGenerationRef.current += 1;
		setCameraDisplayOverrideByPane(from);

		const total = BOARD_PLAY_CAMERA_POST_REDRAW_TOTAL_MS;
		const holdEnd = total / 3;
		const animSpan = total - holdEnd;
		const t0 = typeof performance !== "undefined" ? performance.now() : Date.now();
		const tickInner = () => {
			const now = typeof performance !== "undefined" ? performance.now() : Date.now();
			const elapsed = now - t0;
			if (elapsed >= total) {
				const endCameras = blendTriptychCamerasActivePaneOnly(from, to, 1, postPlayEasePaneId);
				setCameraDisplayOverrideByPane(endCameras);
				suppressCameraBasisSyncRef.current = false;
				cameraBasisFixtureRef.current = fixtureRef.current;
				cameraPlayEndAnimRafRef.current = requestAnimationFrame(() => {
					setCameraDisplayOverrideByPane(null);
					const fit = triptychCamerasFromFixture(fixtureRef.current);
					const p = postPlayEasePaneId;
					setBoardPlayPaneCamerasBaseline((prev) => ({ ...prev, [p]: { ...fit[p] } }));
					cameraPlayEndAnimRafRef.current = null;
				});
				return;
			}
			if (elapsed >= holdEnd) {
				const u = Math.min(1, Math.max(0, (elapsed - holdEnd) / animSpan));
				setCameraDisplayOverrideByPane(blendTriptychCamerasActivePaneOnly(from, to, u, postPlayEasePaneId));
			}
			cameraPlayEndAnimRafRef.current = requestAnimationFrame(tickInner);
		};
		cameraPlayEndAnimRafRef.current = requestAnimationFrame(tickInner);

		return () => {
			if (cameraPlayEndAnimRafRef.current != null) {
				cancelAnimationFrame(cameraPlayEndAnimRafRef.current);
				cameraPlayEndAnimRafRef.current = null;
			}
		};
	}, [boardRedrawPlaying]);

	const camerasByPane = cameraDisplayOverrideByPane ?? boardPlayPaneCamerasBaseline;

	useEffect(() => {
		if (nodesRedrawCameraEaseTick === 0) {
			return;
		}
		if (boardRedrawPlayingRef.current) {
			return;
		}
		if (suppressCameraBasisSyncRef.current) {
			return;
		}
		if (cameraDisplayOverrideRef.current !== null) {
			return;
		}
		const fromSnapshot = nodesRedrawEaseFromRef.current;
		if (fromSnapshot === null) {
			return;
		}
		const nodesRedrawEasePaneId = activePaneIdRef.current;
		if (!boardRedrawInteractiveZoomByPaneRef.current[nodesRedrawEasePaneId]) {
			nodesRedrawEaseFromRef.current = null;
			return;
		}
		const generationAtStart = nodesRedrawEaseGenerationRef.current;
		if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
			cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
			boardPlayNodesRedrawCameraAnimRafRef.current = null;
		}
		const snapshotFixture = fixtureRef.current;
		const from: Record<BoardPlayPaneId, CameraState> = {
			"board-detail": { ...fromSnapshot["board-detail"] },
			"board-overview": { ...fromSnapshot["board-overview"] },
			"board-selection": { ...fromSnapshot["board-selection"] },
		};
		const to = triptychCamerasFromFixture(snapshotFixture);
		const total = BOARD_PLAY_NODES_REDRAW_CAMERA_EASE_TOTAL_MS;
		const holdEnd = total / 3;
		const animSpan = total - holdEnd;
		const t0 = typeof performance !== "undefined" ? performance.now() : Date.now();
		const tickInner = () => {
			if (nodesRedrawEaseGenerationRef.current !== generationAtStart) {
				return;
			}
			const now = typeof performance !== "undefined" ? performance.now() : Date.now();
			const elapsed = now - t0;
			if (elapsed >= total) {
				const endCameras = blendTriptychCamerasActivePaneOnly(from, to, 1, nodesRedrawEasePaneId);
				setBoardPlayPaneCamerasBaseline(endCameras);
				boardPlayNodesRedrawCameraAnimRafRef.current = null;
				nodesRedrawEaseFromRef.current = null;
				return;
			}
			if (elapsed >= holdEnd) {
				const u = Math.min(1, Math.max(0, (elapsed - holdEnd) / animSpan));
				setBoardPlayPaneCamerasBaseline(blendTriptychCamerasActivePaneOnly(from, to, u, nodesRedrawEasePaneId));
			}
			boardPlayNodesRedrawCameraAnimRafRef.current = requestAnimationFrame(tickInner);
		};
		boardPlayNodesRedrawCameraAnimRafRef.current = requestAnimationFrame(tickInner);
		return () => {
			if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
				cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
				boardPlayNodesRedrawCameraAnimRafRef.current = null;
			}
		};
	}, [boardRedrawInteractiveZoomByPane, nodesRedrawCameraEaseTick]);

	useEffect(() => {
		if (cameraDisplayOverrideByPane === null) {
			return;
		}
		nodesRedrawEaseGenerationRef.current += 1;
		if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
			cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
			boardPlayNodesRedrawCameraAnimRafRef.current = null;
		}
	}, [cameraDisplayOverrideByPane]);

	useEffect(() => {
		const pane = activePaneIdRef.current;
		if (boardRedrawInteractiveZoomByPane[pane]) {
			return;
		}
		boardPlayRedrawCameraChaseRef.current = null;
		nodesRedrawEaseGenerationRef.current += 1;
		nodesRedrawEaseFromRef.current = null;
		if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
			cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
			boardPlayNodesRedrawCameraAnimRafRef.current = null;
		}
		if (cameraPlayEndAnimRafRef.current != null) {
			cancelAnimationFrame(cameraPlayEndAnimRafRef.current);
			cameraPlayEndAnimRafRef.current = null;
		}
		setCameraDisplayOverrideByPane(null);
		suppressCameraBasisSyncRef.current = false;
	}, [activePaneId, boardRedrawInteractiveZoomByPane]);

	const redrawPlayingRef = useRef(false);
	const redrawProgressiveEpochRef = useRef(0);
	const redrawLoopSnapshotRef = useRef<BoardPlayRedrawLoopSnapshot>({
		activePaneId: "board-overview",
		boardRedrawHandlesAfterNodes: false,
		boardRedrawProgressiveAutoStopMs: 3000,
		boardRedrawProgressiveEnabled: true,
		boardRedrawPlayMaxItersPerFrame: 96,
		camerasByPane: triptychCamerasFromFixture(initialFixture),
		forceLayoutGravity: 0.012,
		forceLayoutIdealEdgeLength: 64,
		forceLayoutRepulsionStrength: 80,
		lockedNodeIds: [],
		mode: "force-graph",
		treeLayoutDirection: "downwards",
		treeLayoutLayerSpacing: 120,
		treeLayoutSiblingGap: 28,
	});

	const resetBoardRedrawProgressiveEpoch = useCallback(() => {
		redrawProgressiveEpochRef.current = typeof performance !== "undefined" ? performance.now() : Date.now();
	}, []);

	redrawLoopSnapshotRef.current = {
		activePaneId,
		boardRedrawHandlesAfterNodes,
		boardRedrawProgressiveAutoStopMs,
		boardRedrawProgressiveEnabled,
		boardRedrawPlayMaxItersPerFrame,
		camerasByPane,
		forceLayoutGravity,
		forceLayoutIdealEdgeLength,
		forceLayoutRepulsionStrength,
		lockedNodeIds: boardPlayLockedGraphNodeIds(fixture, lockedIds),
		mode: boardRedrawMode,
		treeLayoutDirection,
		treeLayoutLayerSpacing,
		treeLayoutSiblingGap,
	};

	const applyBoardRedrawHandlesOnce = useCallback(() => {
		patchFixture((prev) => layoutBoardFixtureRedrawHandles(prev));
	}, [patchFixture]);

	const applyBoardRedrawOnce = useCallback((modeOverride?: BoardRedrawModeKind) => {
		if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
			cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
			boardPlayNodesRedrawCameraAnimRafRef.current = null;
		}
		nodesRedrawEaseGenerationRef.current += 1;
		const mode = modeOverride ?? boardRedrawMode;
		const interactiveZoomEnabled = boardRedrawInteractiveZoomByPaneRef.current[activePaneId];
		nodesRedrawEaseFromRef.current = interactiveZoomEnabled
			? {
			"board-detail": { ...camerasByPane["board-detail"] },
			"board-overview": { ...camerasByPane["board-overview"] },
			"board-selection": { ...camerasByPane["board-selection"] },
				}
			: null;
		const full = Math.max(1, Math.min(5000, Math.round(forceLayoutFullIterations)));
		patchFixture((prev) => {
			const laidOut = layoutBoardFixtureRedrawNodes(
				prev,
				boardPlayRedrawLayoutOpts(
					activePaneId,
					camerasByPane,
					mode,
					full,
					forceLayoutIdealEdgeLength,
					forceLayoutGravity,
					forceLayoutRepulsionStrength,
					treeLayoutLayerSpacing,
					treeLayoutSiblingGap,
					treeLayoutDirection,
					boardRedrawHandlesAfterNodes,
					boardPlayLockedGraphNodeIds(prev, lockedIds),
				),
			);
			return { ...laidOut, camera: { ...prev.camera } };
		});
		if (interactiveZoomEnabled) {
		setNodesRedrawCameraEaseTick((n) => n + 1);
		}
	}, [
		activePaneId,
		boardRedrawHandlesAfterNodes,
		boardRedrawMode,
		camerasByPane,
		forceLayoutFullIterations,
		forceLayoutGravity,
		forceLayoutIdealEdgeLength,
		forceLayoutRepulsionStrength,
		lockedIds,
		patchFixture,
		treeLayoutLayerSpacing,
		treeLayoutDirection,
		treeLayoutSiblingGap,
	]);

	useEffect(() => {
		if (!boardRedrawPlaying) {
			redrawPlayingRef.current = false;
			return;
		}
		redrawPlayingRef.current = true;
		redrawProgressiveEpochRef.current = typeof performance !== "undefined" ? performance.now() : Date.now();
		let raf = 0;
		const step = () => {
			if (!redrawPlayingRef.current) {
				return;
			}
			const snap = redrawLoopSnapshotRef.current;
			const now = typeof performance !== "undefined" ? performance.now() : Date.now();
			const elapsed = now - redrawProgressiveEpochRef.current;
			if (snap.boardRedrawProgressiveAutoStopMs > 0 && elapsed >= snap.boardRedrawProgressiveAutoStopMs) {
				redrawPlayingRef.current = false;
				setBoardRedrawPlaying(false);
				return;
			}
			let innerIters = 1;
			if (snap.mode === "force-graph") {
				if (snap.boardRedrawProgressiveEnabled) {
					innerIters = boardPlayProgressiveForceIters(elapsed, snap.boardRedrawProgressiveAutoStopMs, snap.boardRedrawPlayMaxItersPerFrame);
				} else {
					innerIters = Math.max(1, Math.min(500, Math.round(snap.boardRedrawPlayMaxItersPerFrame)));
				}
			}
			patchFixture((prev) => {
				if (prev.nodes.length === 0) {
					return prev;
				}
				if (snap.mode === "hierarchical-tree") {
					return layoutBoardFixtureRedrawNodes(
						prev,
						boardPlayRedrawLayoutOpts(
							snap.activePaneId,
							snap.camerasByPane,
							snap.mode,
							1,
							snap.forceLayoutIdealEdgeLength,
							snap.forceLayoutGravity,
							snap.forceLayoutRepulsionStrength,
							snap.treeLayoutLayerSpacing,
							snap.treeLayoutSiblingGap,
							snap.treeLayoutDirection,
							snap.boardRedrawHandlesAfterNodes,
							snap.lockedNodeIds,
						),
					);
				}
				const t0 = typeof performance !== "undefined" ? performance.now() : Date.now();
				let cur = prev;
				while (redrawPlayingRef.current && (typeof performance !== "undefined" ? performance.now() : Date.now()) - t0 < BOARD_PLAYRedraw_FRAME_BUDGET_MS) {
					cur = layoutBoardFixtureRedrawNodes(
						cur,
						boardPlayRedrawLayoutOpts(
							snap.activePaneId,
							snap.camerasByPane,
							snap.mode,
							innerIters,
							snap.forceLayoutIdealEdgeLength,
							snap.forceLayoutGravity,
							snap.forceLayoutRepulsionStrength,
							snap.treeLayoutLayerSpacing,
							snap.treeLayoutSiblingGap,
							snap.treeLayoutDirection,
							snap.boardRedrawHandlesAfterNodes,
							snap.lockedNodeIds,
						),
					);
				}
				return cur;
			});
			raf = requestAnimationFrame(step);
		};
		raf = requestAnimationFrame(step);
		return () => {
			redrawPlayingRef.current = false;
			cancelAnimationFrame(raf);
		};
	}, [boardRedrawPlaying, patchFixture, setBoardRedrawPlaying]);

	const setBoardAutomaticLodForPane = useCallback((pane: BoardPlayPaneId, value: boolean) => {
		setBoardAutomaticLodByPane((prev) => ({ ...prev, [pane]: value }));
	}, []);

	const setBoardPinnedLodForPane = useCallback((pane: BoardPlayPaneId, value: BoardDrawLodKind | undefined) => {
		setBoardPinnedLodByPane((prev) => ({ ...prev, [pane]: value }));
	}, []);

	const shellValue = useMemo<BoardPlayShellValue>(
		() => ({
			activePaneId,
			applyBoardRedrawHandlesOnce,
			applyBoardRedrawOnce,
			applyStructuralDelete,
			appendConstraint,
			appendKind,
			boardRedrawHandlesAfterNodes,
			boardRedrawMode,
			boardRedrawPlayMaxItersPerFrame,
			boardRedrawPlaying,
			boardRedrawProgressiveAutoStopMs,
			boardRedrawProgressiveEnabled,
			boardSelectionGestureHighlight,
			boardSelectionMethod,
			boardSelectionMode,
			boardSelectionTargets,
			boardGridSnapEnabled,
			boardAutomaticLodByPane,
			boardPinnedLodByPane,
			camerasByPane,
			deleteGraphObjects,
			deleteWorkbenchSelection,
			focusGraphSelection,
			focusWorkbenchSelection,
			syncBaselineFromViewportCamera,
			fixture,
			kindCatalogs,
			kindCompatibility,
			forceLayoutFullIterations,
			forceLayoutGravity,
			forceLayoutIdealEdgeLength,
			forceLayoutRepulsionStrength,
			handleCanvasFixtureDrop,
			lockedIds,
			patchFixture,
			patchKindCatalogs,
			patchWires,
			remapIdInSelections,
			resetBoardRedrawProgressiveEpoch,
			setActivePaneId,
			setBoardRedrawHandlesAfterNodes,
			setBoardRedrawMode,
			setBoardRedrawPlayMaxItersPerFrame,
			setBoardRedrawPlaying,
			setBoardRedrawProgressiveAutoStopMs,
			setBoardRedrawProgressiveEnabled,
			setBoardGridSnapEnabled,
			setBoardAutomaticLodForPane,
			setBoardPinnedLodForPane,
			setBoardSelectionGestureHighlight,
			setBoardSelectionMethod,
			setBoardSelectionMode,
			setBoardSelectionTargets,
			setFixture,
			setGraphObjectsHidden,
			setGraphObjectsLocked,
			setForceLayoutFullIterations,
			setForceLayoutGravity,
			setForceLayoutIdealEdgeLength,
			setForceLayoutRepulsionStrength,
			setKindCompatibility,
			setLockedIds,
			setTreeLayoutLayerSpacing,
			setTreeLayoutDirection,
			setTreeLayoutSiblingGap,
			selectionByPane,
			setSelectionForPane,
			setWorkbenchSelection,
			treeLayoutLayerSpacing,
			treeLayoutDirection,
			treeLayoutSiblingGap,
			wires,
			workbenchSelection,
		}),
		[
			activePaneId,
			applyBoardRedrawHandlesOnce,
			applyBoardRedrawOnce,
			applyStructuralDelete,
			appendConstraint,
			appendKind,
			boardRedrawHandlesAfterNodes,
			boardRedrawMode,
			boardRedrawPlayMaxItersPerFrame,
			boardRedrawPlaying,
			boardRedrawProgressiveAutoStopMs,
			boardRedrawProgressiveEnabled,
			boardSelectionGestureHighlight,
			boardSelectionMethod,
			boardSelectionMode,
			boardSelectionTargets,
			boardGridSnapEnabled,
			boardAutomaticLodByPane,
			boardPinnedLodByPane,
			camerasByPane,
			deleteGraphObjects,
			deleteWorkbenchSelection,
			focusGraphSelection,
			focusWorkbenchSelection,
			syncBaselineFromViewportCamera,
			fixture,
			kindCatalogs,
			kindCompatibility,
			forceLayoutFullIterations,
			forceLayoutGravity,
			forceLayoutIdealEdgeLength,
			forceLayoutRepulsionStrength,
			handleCanvasFixtureDrop,
			lockedIds,
			patchFixture,
			patchKindCatalogs,
			patchWires,
			remapIdInSelections,
			resetBoardRedrawProgressiveEpoch,
			selectionByPane,
			setActivePaneId,
			setBoardRedrawHandlesAfterNodes,
			setBoardRedrawMode,
			setBoardRedrawPlayMaxItersPerFrame,
			setBoardRedrawPlaying,
			setBoardRedrawProgressiveAutoStopMs,
			setBoardRedrawProgressiveEnabled,
			setBoardGridSnapEnabled,
			setBoardAutomaticLodForPane,
			setBoardPinnedLodForPane,
			setBoardSelectionGestureHighlight,
			setBoardSelectionMethod,
			setBoardSelectionMode,
			setBoardSelectionTargets,
			setFixture,
			setGraphObjectsHidden,
			setGraphObjectsLocked,
			setForceLayoutFullIterations,
			setForceLayoutGravity,
			setForceLayoutIdealEdgeLength,
			setForceLayoutRepulsionStrength,
			setKindCompatibility,
			setLockedIds,
			setSelectionForPane,
			setTreeLayoutLayerSpacing,
			setTreeLayoutDirection,
			setTreeLayoutSiblingGap,
			setWorkbenchSelection,
			treeLayoutLayerSpacing,
			treeLayoutDirection,
			treeLayoutSiblingGap,
			wires,
			workbenchSelection,
		],
	);

	const boardWindowKinds = useMemo(
		() =>
			boardWindowKindsWithPlayMeasures(
				boardRedrawInteractiveZoomByPane,
				setBoardRedrawInteractiveZoomByPane,
				boardAutomaticLodByPane,
				setBoardAutomaticLodForPane,
				boardPinnedLodByPane,
				setBoardPinnedLodForPane,
			),
		[
			boardRedrawInteractiveZoomByPane,
			boardAutomaticLodByPane,
			boardPinnedLodByPane,
			setBoardAutomaticLodForPane,
			setBoardPinnedLodForPane,
		],
	);

	const boardPlayApp: UIAppConfig = useMemo(
		() => ({
			defaultLayout: boardPlayLayout,
			id: BOARD_PLAY_APP_ID,
			label: "Board",
			leftPanelTabs: [
				{ content: () => <BoardFixtureLibraryPanel />, icon: Library, id: "board-play-library", order: 0 },
				{ content: () => <BoardWorkbenchGraphPanel />, icon: FolderTree, id: "board-play-workbench-graph", order: 1 },
				{ content: () => <BoardWorkbenchKindsPanel />, icon: Circle, id: "board-play-workbench-kinds", order: 2 },
				{ content: () => <BoardWorkbenchConstraintsPanel />, icon: Link2, id: "board-play-workbench-constraints", order: 3 },
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
		[boardWindowKinds, setActivePaneId],
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
