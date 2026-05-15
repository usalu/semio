// #region 🧲Header
// 💻 elements/client/lib/board/play/index.tsx — Board play: triptych Nakagin views, fixture library (DnD), selection inspector, `UI` shell.
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
import { ClipboardList, FileUp } from "lucide-react";
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
import { parseBoardFixtureV1, type BoardFixtureEdgeV1, type BoardFixtureNodeV1, type BoardFixtureV1, type CameraState } from "../js/index";
import { BoardCanvas, Edge, Handle, Node, useBoardEvent } from "../react/index.tsx";
import "./globals.css";
// #endregion 📥Imports

// #region 🔖Kinds
export type BoardPlayPaneId = "board-overview" | "board-detail" | "board-selection";

const BOARD_PLAY_APP_ID = "elements-board-play";
const MIN_ZOOM = 0.2;
const MAX_ZOOM = 8;
const REF_VIEWPORT_SHORT_PX = 640;
// #endregion 🔖Kinds

// #region 🔖Geometry
function clampZoom(value: number): number {
	return Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, value));
}

/** @emoji 📐 Axis-aligned bounds of all fixture nodes (world units). */
function fixtureWorldBounds(fixture: BoardFixtureV1): { cx: number; cy: number; halfSpan: number } {
	let minX = Number.POSITIVE_INFINITY;
	let minY = Number.POSITIVE_INFINITY;
	let maxX = Number.NEGATIVE_INFINITY;
	let maxY = Number.NEGATIVE_INFINITY;
	for (const node of fixture.nodes) {
		minX = Math.min(minX, node.x - node.radius);
		maxX = Math.max(maxX, node.x + node.radius);
		minY = Math.min(minY, node.y - node.radius);
		maxY = Math.max(maxY, node.y + node.radius);
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
/** @emoji 🗼 Renders the shared Nakagin graph with per-pane selection highlights. */
function NakaginBoardScene({ fixture, selectedIds }: { fixture: BoardFixtureV1; selectedIds: Set<string> }): ReactElement {
	return (
		<>
			{fixture.nodes.map((node) => (
				<Node draggable={false} id={node.id} key={node.id} radius={node.radius} selected={selectedIds.has(node.id)} x={node.x} y={node.y}>
					{node.handles.map((handle) => (
						<Handle angle={handle.angle} id={handle.id} key={handle.id} selected={selectedIds.has(handle.id)} />
					))}
				</Node>
			))}
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
			<BoardCanvas camera={camera} className="min-h-0 flex-1" fixtureFileDrop onFixtureFileDrop={setFixture}>
				<BoardSelectionReporter paneId={paneId} />
				<NakaginBoardScene fixture={fixture} selectedIds={selectedIds} />
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
			<BoardCanvas camera={camera} className="min-h-0 flex-1" fixtureFileDrop onFixtureFileDrop={setFixture}>
				<BoardSelectionReporter paneId={paneId} />
				<NakaginBoardScene fixture={fixture} selectedIds={selectedIds} />
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
			<BoardCanvas camera={camera} className="min-h-0 flex-1" fixtureFileDrop onFixtureFileDrop={setFixture}>
				<BoardSelectionReporter paneId={paneId} />
				<NakaginBoardScene fixture={fixture} selectedIds={selectedIds} />
			</BoardCanvas>
		</BoardPaneChrome>
	);
}
// #endregion 🔖Panes

// #region 🔖SidePanels
function applyDroppedFixtureJson(text: string, setFixture: (f: BoardFixtureV1) => void): void {
	let raw: unknown;
	try {
		raw = JSON.parse(text) as unknown;
	} catch {
		return;
	}
	const parsed = parseBoardFixtureV1(raw);
	if (parsed) {
		setFixture(parsed);
	}
}

/** @emoji 📥 Left rail: drop `.board.json` fixtures (same contract as canvas file drop). */
function BoardFixtureLibraryPanel(): ReactElement {
	const { fixture, setFixture } = useBoardPlayShell();
	const [dragOver, setDragOver] = useState(false);

	const onDragEnter = useCallback((e: DragEvent<HTMLDivElement>) => {
		if ([...e.dataTransfer.types].includes("Files")) {
			setDragOver(true);
		}
	}, []);

	const onDragLeave = useCallback((e: DragEvent<HTMLDivElement>) => {
		if (!e.currentTarget.contains(e.relatedTarget as Node)) {
			setDragOver(false);
		}
	}, []);

	const onDragOver = useCallback((e: DragEvent<HTMLDivElement>) => {
		if ([...e.dataTransfer.types].includes("Files")) {
			e.preventDefault();
			e.dataTransfer.dropEffect = "copy";
		}
	}, []);

	const onDrop = useCallback(
		async (e: DragEvent<HTMLDivElement>) => {
			e.preventDefault();
			setDragOver(false);
			const file = e.dataTransfer.files[0];
			if (!file) {
				return;
			}
			const text = await file.text();
			applyDroppedFixtureJson(text, setFixture);
		},
		[setFixture],
	);

	return (
		<div className="flex h-full min-h-0 flex-col gap-3 p-3 text-sm">
			<div className="text-muted-foreground text-xs uppercase tracking-wide">Fixture library</div>
			<div
				className={`flex min-h-[140px] flex-1 flex-col items-center justify-center rounded-md border border-dashed p-4 text-center transition-colors ${
					dragOver ? "border-primary bg-primary/10" : "border-element bg-muted/30"
				}`}
				onDragEnter={onDragEnter}
				onDragLeave={onDragLeave}
				onDragOver={onDragOver}
				onDrop={(ev) => void onDrop(ev)}
			>
				<FileUp className="mb-2 size-8 opacity-70" />
				<p className="font-medium">Drop a board JSON</p>
				<p className="text-muted-foreground mt-1 text-xs">Replaces all three windows with the same graph; cameras re-fit automatically.</p>
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
								{node.label ? (
									<>
										<dt className="text-muted-foreground">label</dt>
										<dd>{node.label}</dd>
									</>
								) : null}
								<dt className="text-muted-foreground">x · y · r</dt>
								<dd>
									{node.x.toFixed(3)} · {node.y.toFixed(3)} · {node.radius}
								</dd>
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
		body.style.backgroundColor = "var(--background)";
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
				{ content: () => <BoardFixtureLibraryPanel />, icon: FileUp, id: "board-play-library", order: 0 },
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
