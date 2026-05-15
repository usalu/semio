// #region 🧲Header
// 💻 elements/client/lib/board/play/index.tsx — Board play: triptych Nakagin views, in-app fixture drag shelf, selection inspector, `UI` shell (same `@elements/ui` + globals pattern as semio rendering / algorithms).
// #endregion 🧲Header

// #region 📥Imports
import {
	UI,
	LevelProvider,
	createWindowLayout,
	getLevelBgClass,
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
	useState,
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
	type BoardFixtureV1,
	type CameraState,
} from "../js/index";
import { BoardCanvas, Edge, Handle, Node, useBoardEvent } from "../react/index.tsx";
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
						draggable={false}
						height={node.height}
						id={node.id}
						key={node.id}
						shape="rectangle"
						selected={selectedIds.has(node.id)}
						text={node.text}
						width={node.width}
						x={node.x}
						y={node.y}
					>
						{node.handles.map((handle) => (
							<Handle angle={handle.angle} id={handle.id} key={handle.id} selected={selectedIds.has(handle.id)} />
						))}
					</Node>
				) : (
					<Node draggable={false} id={node.id} key={node.id} radius={node.radius} selected={selectedIds.has(node.id)} text={node.text} x={node.x} y={node.y}>
						{node.handles.map((handle) => (
							<Handle angle={handle.angle} id={handle.id} key={handle.id} selected={selectedIds.has(handle.id)} />
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

/** @emoji 🔎 Right rail: JSON-ish detail for the active pane’s current selection. */
function BoardSelectionInspectorPanel(): ReactElement {
	const { fixture, activePaneId, selectionByPane } = useBoardPlayShell();
	const ids = [...selectionByPane[activePaneId]].sort((a, b) => a.localeCompare(b));

	return (
		<div className="flex h-full min-h-0 flex-col gap-2 p-3 text-xs">
			<div className="text-muted-foreground flex items-center gap-2 border-b border-element pb-2">
				<ClipboardList className="size-4 shrink-0" />
				<div>
					<div className="font-semibold uppercase tracking-wide">Selection</div>
					<div className="text-[11px] opacity-80">pane: {activePaneId}</div>
				</div>
			</div>
			<div className="min-h-0 flex-1 space-y-3 overflow-y-auto pr-1">
				{ids.length === 0 ? <p className="text-muted-foreground">No selection. Click the graph or pick another tab.</p> : null}
				{ids.map((id) => {
					const node = findNode(fixture, id);
					if (node) {
						return (
							<dl key={id} className="border-element space-y-1 rounded border p-2 font-mono">
								<dt className="text-muted-foreground">node</dt>
								<dd className="break-all">{node.id}</dd>
								<dt className="text-muted-foreground">shape</dt>
								<dd>{node.shape === "rectangle" ? "rectangle" : "circle"}</dd>
								{node.text ? (
									<>
										<dt className="text-muted-foreground">text</dt>
										<dd className="break-all">{node.text}</dd>
									</>
								) : null}
								{node.shape === "rectangle" ? (
									<>
										<dt className="text-muted-foreground">x · y · w · h</dt>
										<dd>
											{node.x.toFixed(3)} · {node.y.toFixed(3)} · {node.width} · {node.height}
										</dd>
									</>
								) : (
									<>
										<dt className="text-muted-foreground">x · y · r</dt>
										<dd>
											{node.x.toFixed(3)} · {node.y.toFixed(3)} · {node.radius}
										</dd>
									</>
								)}
								<dt className="text-muted-foreground">handles</dt>
								<dd>{node.handles.length}</dd>
							</dl>
						);
					}
					const edge = findEdge(fixture, id);
					if (edge) {
						return (
							<dl key={id} className="border-element space-y-1 rounded border p-2 font-mono">
								<dt className="text-muted-foreground">edge</dt>
								<dd className="break-all">{edge.id}</dd>
								<dt className="text-muted-foreground">from</dt>
								<dd className="break-all">{edge.from}</dd>
								<dt className="text-muted-foreground">to</dt>
								<dd className="break-all">{edge.to}</dd>
							</dl>
						);
					}
					const handleOwner = findHandleOwner(fixture, id);
					if (handleOwner) {
						return (
							<dl key={id} className="border-element space-y-1 rounded border p-2 font-mono">
								<dt className="text-muted-foreground">handle</dt>
								<dd className="break-all">{id}</dd>
								<dt className="text-muted-foreground">node</dt>
								<dd className="break-all">{handleOwner.node.id}</dd>
								<dt className="text-muted-foreground">angle</dt>
								<dd>{handleOwner.node.handles.find((h) => h.id === id)?.angle.toFixed(4)}</dd>
							</dl>
						);
					}
					return (
						<div key={id} className="border-element rounded border p-2 font-mono text-amber-300">
							unknown id: {id}
						</div>
					);
				})}
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
	const [activePaneId, setActivePaneId] = useState<BoardPlayPaneId>("board-overview");
	const [selectionByPane, setSelectionByPane] = useState<Record<BoardPlayPaneId, Set<string>>>(() => selectionSeedForFixture(initialFixture));

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
			camerasByPane,
			fixture,
			setActivePaneId,
			setFixture,
			selectionByPane,
			setSelectionForPane,
		}),
		[activePaneId, camerasByPane, fixture, setFixture, selectionByPane, setSelectionForPane],
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
