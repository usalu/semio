// #region 🧲Header
// 💻 .storybook/stories/elements/board/Board.stories.tsx
// Specs: Host the elements board canvas for Storybook + Playwright raster/LOD/selection checks.
// Summary: Raster modes, full Nakagin board fixture (180 nodes / 179 kit connections), and Playwright harness stories.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import { useCallback, useState, type Dispatch, type ReactElement, type SetStateAction } from "react";

import {
	BoardCanvas,
	Edge,
	Handle,
	Node,
	useBoardEvent,
} from "../../../../elements/client/lib/board/react/index.tsx";
import nakaginCapsuleTowerBoardFixture from "../../../fixtures/nakagin-capsule-tower.board.json";

const meta = {
	title: "elements/board",
	component: BoardCanvas,
	parameters: {
		layout: "fullscreen",
	},
	tags: ["autodocs"],
} satisfies Meta<typeof BoardCanvas>;

export default meta;

type Story = StoryObj<typeof meta>;

interface BoardFixtureHandleJson {
	angle: number;
	id: string;
}

interface BoardFixtureNodeJson {
	cad?: { x: number; y: number; z: number } | null;
	handles: BoardFixtureHandleJson[];
	height?: number;
	id: string;
	label?: string;
	radius?: number;
	shape?: "circle" | "rectangle";
	text?: string;
	width?: number;
	x: number;
	y: number;
}

interface BoardFixtureEdgeJson {
	from: string;
	id: string;
	to: string;
}

interface BoardFixtureV1 {
	camera: { x: number; y: number; zoom: number };
	edges: BoardFixtureEdgeJson[];
	meta: Record<string, unknown>;
	nodes: BoardFixtureNodeJson[];
	schema: string;
}

const nakaginCapsuleTowerBoard = nakaginCapsuleTowerBoardFixture as BoardFixtureV1;

type DefaultBoardGraphNode = {
	handles: { angle: number; id: string }[];
	id: string;
	radius: number;
	x: number;
	y: number;
};

type DefaultBoardGraphEdge = { from: string; id: string; to: string };

type DefaultBoardGraph = { edges: DefaultBoardGraphEdge[]; nodes: DefaultBoardGraphNode[] };

const defaultBoardGraph: DefaultBoardGraph = {
	edges: [{ from: "alpha.out", id: "link-1", to: "beta.in" }],
	nodes: [
		{ handles: [{ angle: 0, id: "alpha.out" }], id: "alpha", radius: 44, x: 0, y: 0 },
		{ handles: [{ angle: Math.PI, id: "beta.in" }], id: "beta", radius: 40, x: 280, y: 120 },
	],
};

function BoardDeleteReconciler({ setGraph }: { setGraph: Dispatch<SetStateAction<DefaultBoardGraph>> }): null {
	useBoardEvent(
		"edgeDelete",
		useCallback(({ id }: { id: string }) => {
			setGraph((graph) => ({ ...graph, edges: graph.edges.filter((edge) => edge.id !== id) }));
		}, [setGraph]),
	);
	useBoardEvent(
		"nodeDelete",
		useCallback(({ id }: { id: string }) => {
			setGraph((graph) => {
				const node = graph.nodes.find((entry) => entry.id === id);
				const handleIds = new Set(node?.handles.map((handle) => handle.id) ?? []);
				return {
					edges: graph.edges.filter((edge) => !handleIds.has(edge.from) && !handleIds.has(edge.to)),
					nodes: graph.nodes.filter((entry) => entry.id !== id),
				};
			});
		}, [setGraph]),
	);
	return null;
}

function StatefulInteractiveBoardScene(): ReactElement {
	const [graph, setGraph] = useState(() => defaultBoardGraph);
	return (
		<>
			<BoardDeleteReconciler setGraph={setGraph} />
			{graph.nodes.map((node) => (
				<Node id={node.id} key={node.id} radius={node.radius} x={node.x} y={node.y}>
					{node.handles.map((handle) => (
						<Handle angle={handle.angle} id={handle.id} key={handle.id} />
					))}
				</Node>
			))}
			{graph.edges.map((edge) => (
				<Edge from={edge.from} id={edge.id} key={edge.id} to={edge.to} />
			))}
		</>
	);
}

/** 🗼 Full Nakagin Capsule Tower board from `nakagin-capsule-tower.board.json` (regenerate via `nakagin-capsule-tower-board.generate.script.ts`). */
const nakaginCapsuleTowerBoardScene: ReactElement = (
	<>
		{nakaginCapsuleTowerBoard.nodes.map((node) =>
			node.shape === "rectangle" && node.width != null && node.height != null ? (
				<Node
					draggable={false}
					height={node.height}
					id={node.id}
					key={node.id}
					shape="rectangle"
					text={node.text ?? node.label}
					width={node.width}
					x={node.x}
					y={node.y}
				>
					{node.handles.map((handle) => (
						<Handle angle={handle.angle} id={handle.id} key={handle.id} />
					))}
				</Node>
			) : (
				<Node draggable={false} id={node.id} key={node.id} radius={node.radius ?? 0} text={node.text ?? node.label} x={node.x} y={node.y}>
					{node.handles.map((handle) => (
						<Handle angle={handle.angle} id={handle.id} key={handle.id} />
					))}
				</Node>
			),
		)}
		{nakaginCapsuleTowerBoard.edges.map((edge) => (
			<Edge from={edge.from} id={edge.id} key={edge.id} to={edge.to} />
		))}
	</>
);

export const Default: Story = {
	render: (args) => (
		<BoardCanvas {...args}>
			<StatefulInteractiveBoardScene />
		</BoardCanvas>
	),
	args: {
		camera: { x: 0, y: 0, zoom: 1 },
		height: 520,
		width: 720,
		worldRasterTiling: "none",
	},
};

export const WorldTileClip: Story = {
	render: (args) => (
		<BoardCanvas {...args}>
			<StatefulInteractiveBoardScene />
		</BoardCanvas>
	),
	args: {
		...Default.args,
		worldRasterTiling: "world-clip",
	},
};

export const NakaginCapsuleTowerFlatSelection: Story = {
	render: (args) => (
		<BoardCanvas {...args}>
			{nakaginCapsuleTowerBoardScene}
		</BoardCanvas>
	),
	args: {
		...Default.args,
		camera: { ...nakaginCapsuleTowerBoard.camera },
	},
};
