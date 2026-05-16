// #region 🧲Header
// 💻 elements/client/lib/board/play/index.tsx — Board play: triptych Nakagin views, in-app fixture drag shelf, selection inspector, `UI` shell (same `@elements/ui` + globals pattern as semio rendering / algorithms).
// #endregion 🧲Header

// #region 📥Imports
import {
	Button,
	Input,
	Label,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	Tree,
	TreeStateProvider,
	UI,
	LevelProvider,
	createWindowLayout,
	getLevelBgClass,
	type TreeDataSection,
	type UIAppConfig,
	type UIWindowKindDefinition,
	type UIWindowLayout,
} from "@elements/ui";
import { ClipboardList, Library } from "lucide-react";
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
import { createRoot } from "react-dom/client";

import nakaginFixtureJson from "../../../../../.storybook/fixtures/nakagin-capsule-tower.board.json";
import {
	BOARD_CAMERA_ZOOM_MAX,
	BOARD_CAMERA_ZOOM_MIN,
	BOARD_FIXTURE_DRAG_V1_MIME,
	encodeBoardFixtureForDragV1,
	parseBoardFixtureV1,
	type BoardFixtureEdgeV1,
	type BoardFixtureNodeV1,
	type BoardFixtureRectangleNodeV1,
	type BoardFixtureV1,
	type CameraState,
} from "../index";
import { BoardCanvas, Edge, Handle, Node, useBoardEvent } from "../index.tsx";
import "./globals.css";
// #endregion 📥Imports

// #region 🔖Kinds
export type BoardPlayPaneId = "board-overview" | "board-detail" | "board-selection";

const BOARD_PLAY_APP_ID = "elements-board-play";
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
	activePaneId: BoardPlayPaneId;
	setActivePaneId: (id: BoardPlayPaneId) => void;
	selectionByPane: Record<BoardPlayPaneId, Set<string>>;
	setSelectionForPane: (pane: BoardPlayPaneId, ids: readonly string[]) => void;
	camerasByPane: Record<BoardPlayPaneId, CameraState>;
	/** @emoji 🗑️ Drops ids from the shared fixture after the canvas emits structural delete events. */
	applyStructuralDelete: (kind: "edge" | "node", id: string) => void;
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

// #region 🔖Scene
/** @emoji 🗼 Marker tree for {@link BoardCanvas} — must stay a Fragment of {@link Node}/{@link Edge} so {@link buildBoardSceneDescriptor} sees markers (custom wrappers are opaque to the static walk). */
function nakaginBoardMarkers(fixture: BoardFixtureV1, selectedIds: Set<string>): ReactElement {
	return (
		<>
			{fixture.nodes.map((node) =>
				node.shape === "rectangle" ? (
					<Node
						draggable
						height={node.height}
						id={node.id}
						key={node.id}
						shape="rectangle"
						selected={selectedIds.has(node.id)}
						text={node.text}
						textAutofit={node.textAutofit === true}
						width={node.width}
						x={node.x}
						y={node.y}
					>
						{node.handles.map((handle) => (
							<Handle
								angle={handle.angle}
								id={handle.id}
								key={handle.id}
								radius={handle.radius}
								selected={selectedIds.has(handle.id)}
							/>
						))}
					</Node>
				) : (
					<Node draggable id={node.id} key={node.id} radius={node.radius} selected={selectedIds.has(node.id)} text={node.text} textAutofit={node.textAutofit === true} x={node.x} y={node.y}>
						{node.handles.map((handle) => (
							<Handle
								angle={handle.angle}
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
				<Edge from={edge.from} id={edge.id} key={edge.id} selected={selectedIds.has(edge.id)} to={edge.to} />
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
	const { fixture, setFixture, camerasByPane, selectionByPane } = useBoardPlayShell();
	const paneId: BoardPlayPaneId = "board-overview";
	const camera = camerasByPane[paneId];
	const selectedIds = selectionByPane[paneId];
	return (
		<BoardPaneChrome paneId={paneId}>
			<BoardCanvas camera={camera} className="min-h-0 flex-1" fixtureDragDrop onFixtureDrop={setFixture}>
				<BoardSelectionReporter paneId={paneId} />
				<BoardStructuralDeleteReporter />
				{nakaginBoardMarkers(fixture, selectedIds)}
			</BoardCanvas>
		</BoardPaneChrome>
	);
}

function BoardDetailPane(): ReactElement {
	const { fixture, setFixture, camerasByPane, selectionByPane } = useBoardPlayShell();
	const paneId: BoardPlayPaneId = "board-detail";
	const camera = camerasByPane[paneId];
	const selectedIds = selectionByPane[paneId];
	return (
		<BoardPaneChrome paneId={paneId}>
			<BoardCanvas camera={camera} className="min-h-0 flex-1" fixtureDragDrop onFixtureDrop={setFixture}>
				<BoardSelectionReporter paneId={paneId} />
				<BoardStructuralDeleteReporter />
				{nakaginBoardMarkers(fixture, selectedIds)}
			</BoardCanvas>
		</BoardPaneChrome>
	);
}

function BoardSelectionPane(): ReactElement {
	const { fixture, setFixture, camerasByPane, selectionByPane } = useBoardPlayShell();
	const paneId: BoardPlayPaneId = "board-selection";
	const camera = camerasByPane[paneId];
	const selectedIds = selectionByPane[paneId];
	return (
		<BoardPaneChrome paneId={paneId}>
			<BoardCanvas camera={camera} className="min-h-0 flex-1" fixtureDragDrop onFixtureDrop={setFixture}>
				<BoardSelectionReporter paneId={paneId} />
				<BoardStructuralDeleteReporter />
				{nakaginBoardMarkers(fixture, selectedIds)}
			</BoardCanvas>
		</BoardPaneChrome>
	);
}
// #endregion 🔖Panes

// #region 🔖SidePanels
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
			<div className="text-muted-foreground text-xs uppercase tracking-wide">Fixture shelf</div>
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
function AngleTRing({ value, onChange }: { onChange: (next: number) => void; value: number }): ReactElement {
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
				className="border-element bg-muted/20 touch-none select-none rounded-full border"
				onPointerCancel={onPointerUp}
				onPointerDown={onPointerDown}
				onPointerMove={onPointerMove}
				onPointerUp={onPointerUp}
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
			<div className="text-muted-foreground font-mono text-[10px]">t = {value.toFixed(4)} rad</div>
		</div>
	);
}

function NumericStepperRow({
	id,
	label,
	onChange,
	step,
	value,
}: {
	id: string;
	label: string;
	onChange: (next: number) => void;
	step: number;
	value: number;
}): ReactElement {
	return (
		<Label id={id} label={label}>
			<div className="flex min-w-0 items-center gap-1">
				<Button className="h-7 shrink-0 px-2" onClick={() => onChange(value - step)} type="button" variant="outline">
					−
				</Button>
				<Input
					className="h-7 min-w-0 flex-1 font-mono text-xs"
					onChange={(e: ChangeEvent<HTMLInputElement>) => {
						const parsed = Number(e.target.value);
						if (Number.isFinite(parsed)) {
							onChange(parsed);
						}
					}}
					value={Number.isFinite(value) ? String(value) : ""}
				/>
				<Button className="h-7 shrink-0 px-2" onClick={() => onChange(value + step)} type="button" variant="outline">
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
	setFixture,
}: {
	fixture: BoardFixtureV1;
	nodeIds: readonly string[];
	setFixture: (next: BoardFixtureV1 | ((prev: BoardFixtureV1) => BoardFixtureV1)) => void;
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
	const shapeValue = shapeUniform ? shapes[0] : "__mixed__";

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
			setFixture((prev) => ({
				...prev,
				nodes: prev.nodes.map((n) => (idSet.has(n.id) ? updater(n) : n)),
			}));
		},
		[idSet, setFixture],
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
					onValueChange={(v) => {
						if (v === "circle" || v === "rectangle") {
							onShape(v);
						}
					}}
					value={shapeValue === "__mixed__" ? undefined : shapeValue}
				>
					<SelectTrigger className="h-7 font-mono text-xs">
						<SelectValue placeholder={shapeUniform ? undefined : "Mixed"} />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="circle">circle</SelectItem>
						<SelectItem value="rectangle">rectangle</SelectItem>
					</SelectContent>
				</Select>
			</Label>
			<NumericStepperRow id="board-play.inspector.node.x" label="x" onChange={(v) => patchNodes((n) => ({ ...n, x: v }))} step={1} value={xValue} />
			<NumericStepperRow id="board-play.inspector.node.y" label="y" onChange={(v) => patchNodes((n) => ({ ...n, y: v }))} step={1} value={yValue} />
			{targets.some((n) => !nodeIsRectangle(n)) ? (
				<NumericStepperRow id="board-play.inspector.node.r" label="radius" onChange={(v) => patchNodes((n) => (nodeIsRectangle(n) ? n : { ...n, radius: Math.max(1e-6, v) }))} step={1} value={rValue} />
			) : null}
			{targets.some(nodeIsRectangle) ? (
				<>
					<NumericStepperRow id="board-play.inspector.node.w" label="width" onChange={(v) => patchNodes((n) => (nodeIsRectangle(n) ? { ...n, width: Math.max(1e-6, v) } : n))} step={1} value={wValue} />
					<NumericStepperRow id="board-play.inspector.node.h" label="height" onChange={(v) => patchNodes((n) => (nodeIsRectangle(n) ? { ...n, height: Math.max(1e-6, v) } : n))} step={1} value={hValue} />
				</>
			) : null}
		</div>
	);
}

/** @emoji 🟣 Batch handle inspector: polar `t`, hit radius, optional id when single selection. */
function InspectorHandleBatch({
	fixture,
	handleIds,
	setFixture,
}: {
	fixture: BoardFixtureV1;
	handleIds: readonly string[];
	setFixture: (next: BoardFixtureV1 | ((prev: BoardFixtureV1) => BoardFixtureV1)) => void;
}): ReactElement {
	const idSet = useMemo(() => new Set(handleIds), [handleIds]);
	const handles = useMemo(
		() => handleIds.map((id) => findHandle(fixture, id)).filter((h): h is BoardFixtureHandleV1 => Boolean(h)),
		[fixture, handleIds],
	);
	const angles = handles.map((h) => h.angle);
	const angleUniform = allEqual(angles);
	const angleValue = angleUniform ? angles[0] : 0;
	const radii = handles.map((h) => h.radius ?? 8);
	const radiusUniform = allEqual(radii);
	const radiusValue = radiusUniform ? radii[0] : Number.NaN;

	const patchHandles = useCallback(
		(updater: (h: BoardFixtureHandleV1) => BoardFixtureHandleV1) => {
			setFixture((prev) => ({
				...prev,
				nodes: prev.nodes.map((node) => ({
					...node,
					handles: node.handles.map((h) => (idSet.has(h.id) ? updater(h) : h)),
				})),
			}));
		},
		[idSet, setFixture],
	);

	return (
		<div className="border-element/60 space-y-3 border-l pl-2">
			<div className="flex flex-wrap items-start gap-4">
				<AngleTRing
					onChange={(t) => {
						patchHandles((h) => ({ ...h, angle: t }));
					}}
					value={angleValue}
				/>
				<div className="min-w-0 flex-1 space-y-3">
					<NumericStepperRow
						id="board-play.inspector.handle.t"
						label="t (rad)"
						onChange={(v) => patchHandles((h) => ({ ...h, angle: normalizeAngleRad(v) }))}
						step={0.05}
						value={angleUniform ? angleValue : Number.NaN}
					/>
					<NumericStepperRow
						id="board-play.inspector.handle.radius"
						label="Hit radius"
						onChange={(v) => patchHandles((h) => ({ ...h, radius: Math.max(1e-6, v) }))}
						step={1}
						value={radiusValue}
					/>
					{handleIds.length === 1 ? (
						<Label id="board-play.inspector.handle.id" label="Id">
							<Input
								className="h-7 font-mono text-xs"
								onChange={(e: ChangeEvent<HTMLInputElement>) => {
									const nextId = e.target.value;
									const oldId = handleIds[0];
									if (!oldId || nextId === oldId) {
										return;
									}
									setFixture((prev) => ({
										...prev,
										edges: prev.edges.map((edge) => ({
											...edge,
											from: edge.from === oldId ? nextId : edge.from,
											to: edge.to === oldId ? nextId : edge.to,
										})),
										nodes: prev.nodes.map((node) => ({
											...node,
											handles: node.handles.map((h) => (h.id === oldId ? { ...h, id: nextId } : h)),
										})),
									}));
								}}
								value={handleIds[0]}
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
	setFixture,
}: {
	fixture: BoardFixtureV1;
	edgeIds: readonly string[];
	setFixture: (next: BoardFixtureV1 | ((prev: BoardFixtureV1) => BoardFixtureV1)) => void;
}): ReactElement {
	const idSet = useMemo(() => new Set(edgeIds), [edgeIds]);
	const edges = useMemo(
		() => edgeIds.map((id) => findEdge(fixture, id)).filter((e): e is BoardFixtureEdgeV1 => Boolean(e)),
		[edgeIds, fixture],
	);
	const froms = edges.map((e) => e.from);
	const tos = edges.map((e) => e.to);
	const fromUniform = allEqual(froms);
	const toUniform = allEqual(tos);
	const handleOptions = useMemo(() => listHandleIds(fixture), [fixture]);

	const patchEdges = useCallback(
		(updater: (e: BoardFixtureEdgeV1) => BoardFixtureEdgeV1) => {
			setFixture((prev) => ({
				...prev,
				edges: prev.edges.map((e) => (idSet.has(e.id) ? updater(e) : e)),
			}));
		},
		[idSet, setFixture],
	);

	return (
		<div className="border-element/60 space-y-3 border-l pl-2">
			<Label id="board-play.inspector.edge.from" label="From">
				<Select
					onValueChange={(v) => {
						patchEdges((e) => ({ ...e, from: v }));
					}}
					value={fromUniform ? froms[0] : undefined}
				>
					<SelectTrigger className="h-7 font-mono text-xs">
						<SelectValue placeholder={fromUniform ? undefined : "Mixed"} />
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
			<Label id="board-play.inspector.edge.to" label="To">
				<Select
					onValueChange={(v) => {
						patchEdges((e) => ({ ...e, to: v }));
					}}
					value={toUniform ? tos[0] : undefined}
				>
					<SelectTrigger className="h-7 font-mono text-xs">
						<SelectValue placeholder={toUniform ? undefined : "Mixed"} />
					</SelectTrigger>
					<SelectContent>
						{handleOptions.map((hid) => (
							<SelectItem key={`to-${hid}`} value={hid}>
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
						onChange={(e: ChangeEvent<HTMLInputElement>) => {
							const nextId = e.target.value;
							const oldId = edgeIds[0];
							if (!oldId || nextId === oldId) {
								return;
							}
							setFixture((prev) => ({
								...prev,
								edges: prev.edges.map((edge) => (edge.id === oldId ? { ...edge, id: nextId } : edge)),
							}));
						}}
						value={edgeIds[0]}
					/>
				</Label>
			) : null}
		</div>
	);
}

/** @emoji 🔎 Sketchpad-style tree inspector with batch edits for the active pane selection. */
function BoardSelectionInspectorPanel(): ReactElement {
	const { activePaneId, fixture, selectionByPane, setFixture } = useBoardPlayShell();
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
				content: <InspectorNodeBatch fixture={fixture} nodeIds={nodeIds} setFixture={setFixture} />,
				defaultOpen: true,
				id: "board-play-inspector-nodes",
				label: `Nodes (${nodeIds.length})`,
			});
		}
		if (handleIds.length > 0) {
			sections.push({
				content: <InspectorHandleBatch fixture={fixture} handleIds={handleIds} setFixture={setFixture} />,
				defaultOpen: true,
				id: "board-play-inspector-handles",
				label: `Handles (${handleIds.length})`,
			});
		}
		if (edgeIds.length > 0) {
			sections.push({
				content: <InspectorEdgeBatch edgeIds={edgeIds} fixture={fixture} setFixture={setFixture} />,
				defaultOpen: true,
				id: "board-play-inspector-edges",
				label: `Edges (${edgeIds.length})`,
			});
		}
		if (sections.length === 0) {
			sections.push({
				content: (
					<div className="text-amber-300 px-1 py-2 font-mono text-xs">
						Unknown ids: {ids.join(", ")}
					</div>
				),
				id: "board-play-inspector-unknown",
				label: "Selection",
			});
		}
		return sections;
	}, [edgeIds, fixture, handleIds, ids, ids.length, nodeIds, setFixture]);

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

const boardWindowKinds: UIWindowKindDefinition[] = [
	{ component: BoardOverviewPane, id: "board-overview", label: "Overview" },
	{ component: BoardDetailPane, id: "board-detail", label: "Zoom" },
	{ component: BoardSelectionPane, id: "board-selection", label: "Selection" },
];
// #endregion 🔖Layout

// #region 🔖Theme
/** @emoji 🌓 Locks document `dark` mode so Golden Layout’s dark theme sheet matches the shell. */
function useBoardPlayDocumentChrome(): void {
	useEffect(() => {
		const root = document.documentElement;
		const body = document.body;
		root.classList.add("dark");
		body.style.backgroundColor = "var(--base)";
		body.style.color = "var(--foreground)";
		return () => {
			root.classList.remove("dark");
			body.style.backgroundColor = "";
			body.style.color = "";
		};
	}, []);
}
// #endregion 🔖Theme

// #region 🔖Entrypoint
const initialFixture = parseBoardFixtureV1(nakaginFixtureJson as unknown) ?? (nakaginFixtureJson as BoardFixtureV1);

function BoardPlayInner(): ReactElement {
	const [fixture, setFixtureState] = useState<BoardFixtureV1>(initialFixture);
	const fixtureRef = useRef<BoardFixtureV1>(fixture);
	fixtureRef.current = fixture;
	const [activePaneId, setActivePaneId] = useState<BoardPlayPaneId>("board-overview");
	const [selectionByPane, setSelectionByPane] = useState<Record<BoardPlayPaneId, Set<string>>>(() => selectionSeedForFixture(initialFixture));

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
				edges: prev.edges.filter((e) => !hset.has(e.from) && !hset.has(e.to)),
				nodes: prev.nodes.filter((x) => x.id !== id),
			};
		});
		pruneSelections([id, ...handleIds]);
	}, []);

	const setFixture = useCallback((next: BoardFixtureV1) => {
		setFixtureState(next);
		setSelectionByPane(selectionSeedForFixture(next));
	}, []);

	const setSelectionForPane = useCallback((pane: BoardPlayPaneId, ids: readonly string[]) => {
		setSelectionByPane((prev) => ({ ...prev, [pane]: new Set(ids) }));
	}, []);

	const camerasByPane = useMemo(() => triptychCamerasFromFixture(fixture), [fixture]);

	const shellValue = useMemo<BoardPlayShellValue>(
		() => ({
			activePaneId,
			applyStructuralDelete,
			camerasByPane,
			fixture,
			setActivePaneId,
			setFixture,
			selectionByPane,
			setSelectionForPane,
		}),
		[activePaneId, applyStructuralDelete, camerasByPane, fixture, setFixture, selectionByPane, setSelectionForPane],
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
			],
			windowKinds: boardWindowKinds,
		}),
		[setActivePaneId],
	);

	return (
		<BoardPlayShellContext.Provider value={shellValue}>
			<UI
				apps={[boardPlayApp]}
				defaultAppId={BOARD_PLAY_APP_ID}
				initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }}
			/>
		</BoardPlayShellContext.Provider>
	);
}

function BoardPlayApp(): ReactElement {
	useBoardPlayDocumentChrome();
	return (
		<LevelProvider level="window">
			<div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>
				<BoardPlayInner />
			</div>
		</LevelProvider>
	);
}

const mount = document.getElementById("root");
if (!mount) {
	throw new Error("Board play root #root missing.");
}

createRoot(mount).render(<BoardPlayApp />);
// #endregion 🔖Entrypoint
