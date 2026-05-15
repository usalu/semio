// #region 🧲Header
// 💻 elements/client/lib/board/play/index.tsx — Board play shell: `UI` + Golden Layout + Nakagin fixture triptych + fixture file drop.
// #endregion 🧲Header

// #region 📥Imports
import {
	UI,
	createWindowLayout,
	type UIAppConfig,
	type UIWindowKindDefinition,
	type UIWindowLayout,
} from "@elements/ui";
import { createContext, useCallback, useContext, useMemo, useState, type ReactElement } from "react";
import { createRoot } from "react-dom/client";

import nakaginFixtureJson from "../../../../../.storybook/fixtures/nakagin-capsule-tower.board.json";
import { parseBoardFixtureV1 } from "../js/index";
import {
	BoardCanvas,
	Edge,
	Handle,
	Node,
	type BoardFixtureV1,
} from "../react/index.tsx";
import "./globals.css";
// #endregion 📥Imports

// #region 🔖FixtureContext
interface FixtureContextValue {
	fixture: BoardFixtureV1;
	setFixture: (next: BoardFixtureV1) => void;
}

const FixtureContext = createContext<FixtureContextValue | null>(null);

function useFixture(): FixtureContextValue {
	const value = useContext(FixtureContext);
	if (!value) {
		throw new Error("useFixture must be used under FixtureContext.");
	}
	return value;
}
// #endregion 🔖FixtureContext

// #region 🔖Scene
const CAMERA_OVERVIEW: Partial<BoardFixtureV1["camera"]> = { x: 35, y: -25, zoom: 0.22 };
const CAMERA_DETAIL: Partial<BoardFixtureV1["camera"]> = { x: -195, y: 155, zoom: 0.52 };
const CAMERA_WIDE: Partial<BoardFixtureV1["camera"]> = { x: 95, y: -210, zoom: 0.11 };

const SELECTION_OVERVIEW = new Set<string>(["01890804-66f2-4544-98f0-b6f0c0615492"]);
const SELECTION_DETAIL = new Set<string>(["0a23d9c7-b75b-4166-8730-351367df9f8a", "0a23d9c7-b75b-4166-8730-351367df9f8a:link"]);
const SELECTION_EDGE = new Set<string>(["015032e9-67ed-4736-adab-a6e10351079b"]);

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
// #endregion 🔖Scene

// #region 🔖Panes
function BoardOverviewPane(): ReactElement {
	const { fixture, setFixture } = useFixture();
	const base = fixture.camera;
	const camera = { x: CAMERA_OVERVIEW.x ?? base.x, y: CAMERA_OVERVIEW.y ?? base.y, zoom: CAMERA_OVERVIEW.zoom ?? base.zoom };
	return (
		<div className="h-full min-h-0 w-full">
			<BoardCanvas camera={camera} className="h-full w-full" fixtureFileDrop onFixtureFileDrop={setFixture}>
				<NakaginBoardScene fixture={fixture} selectedIds={SELECTION_OVERVIEW} />
			</BoardCanvas>
		</div>
	);
}

function BoardDetailPane(): ReactElement {
	const { fixture, setFixture } = useFixture();
	const base = fixture.camera;
	const camera = { x: CAMERA_DETAIL.x ?? base.x, y: CAMERA_DETAIL.y ?? base.y, zoom: CAMERA_DETAIL.zoom ?? base.zoom };
	return (
		<div className="h-full min-h-0 w-full">
			<BoardCanvas camera={camera} className="h-full w-full" fixtureFileDrop onFixtureFileDrop={setFixture}>
				<NakaginBoardScene fixture={fixture} selectedIds={SELECTION_DETAIL} />
			</BoardCanvas>
		</div>
	);
}

function BoardSelectionPane(): ReactElement {
	const { fixture, setFixture } = useFixture();
	const base = fixture.camera;
	const camera = { x: CAMERA_WIDE.x ?? base.x, y: CAMERA_WIDE.y ?? base.y, zoom: CAMERA_WIDE.zoom ?? base.zoom };
	return (
		<div className="h-full min-h-0 w-full">
			<BoardCanvas camera={camera} className="h-full w-full" fixtureFileDrop onFixtureFileDrop={setFixture}>
				<NakaginBoardScene fixture={fixture} selectedIds={SELECTION_EDGE} />
			</BoardCanvas>
		</div>
	);
}
// #endregion 🔖Panes

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

const boardPlayApp: UIAppConfig = {
	defaultLayout: boardPlayLayout,
	id: "elements-board-play",
	label: "Board",
	windowKinds: boardWindowKinds,
};
// #endregion 🔖Layout

// #region 🔖Entrypoint
const initialFixture = parseBoardFixtureV1(nakaginFixtureJson as unknown) ?? (nakaginFixtureJson as BoardFixtureV1);

function BoardPlayApp(): ReactElement {
	const [fixture, setFixture] = useState<BoardFixtureV1>(initialFixture);
	const value = useMemo(() => ({ fixture, setFixture }), [fixture]);
	return (
		<FixtureContext.Provider value={value}>
			<div className="h-screen w-screen">
				<UI apps={[boardPlayApp]} defaultAppId={boardPlayApp.id} />
			</div>
		</FixtureContext.Provider>
	);
}

const mount = document.getElementById("root");
if (!mount) {
	throw new Error("Board play root #root missing.");
}

createRoot(mount).render(<BoardPlayApp />);
// #endregion 🔖Entrypoint
