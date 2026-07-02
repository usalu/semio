// #region 🧲Header
// 💻 .storybook/story/puzzle/2d/Puzzle2d.stories.tsx
// Specs: Host the elements puzzle 2d canvas for Storybook + Playwright raster/LOD/selection checks.
// Summary: Raster modes, full Nakagin puzzle 2d fixture (180 nodes / 179 kit connections), and Playwright harness stories.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import { useCallback, useState, type Dispatch, type ReactElement, type SetStateAction } from "react";

import {
	Puzzle2dCanvas,
	Edge,
	Handle,
	Node,
	usePuzzle2dEvent,
	BUILTIN_PORT_HANDLE_KIND,
	DEFAULT_KIND_CATALOG_BUNDLE,
	fixtureMetaKindCatalogBundle,
	puzzle2dFixtureMetaKindCompatibility,
	mergeKindCatalogBundleByRowId,
} from "@semio-tech/puzzle-2d-react";
import nakaginCapsuleTowerPuzzle2dFixture from "../../../../puzzle/2d/fixture/nakagin-capsule-tower.2d.json";

const meta = {
	title: "🧩puzzle🩻2d",
	component: Puzzle2dCanvas,
	parameters: {
		layout: "fullscreen",
	},
	tags: ["autodocs"],
} satisfies Meta<typeof Puzzle2dCanvas>;

export default meta;

type Story = StoryObj<typeof meta>;

interface Puzzle2dFixtureHandleJson {
	angle: number;
	color?: string;
	handleKind?: string;
	id: string;
	radius?: number;
}

interface Puzzle2dFixtureNodeJson {
	cad?: { x: number; y: number; z: number } | null;
	handles: Puzzle2dFixtureHandleJson[];
	height?: number;
	iconKind?: string;
	id: string;
	label?: string;
	nodeKind?: string;
	radius?: number;
	shape?: "circle" | "rectangle";
	text?: string;
	width?: number;
	x: number;
	y: number;
}

interface Puzzle2dFixtureEdgeJson {
	id: string;
	source: string;
	target: string;
}

interface Puzzle2dFixture {
	camera: { x: number; y: number; zoom: number };
	edges: Puzzle2dFixtureEdgeJson[];
	meta?: Record<string, unknown>;
	nodes: Puzzle2dFixtureNodeJson[];
	schema: string;
}

const nakaginCapsuleTowerPuzzle2d = nakaginCapsuleTowerPuzzle2dFixture as Puzzle2dFixture;

const nakaginStoryKindCatalogs = mergeKindCatalogBundleByRowId(
	{ ...DEFAULT_KIND_CATALOG_BUNDLE },
	fixtureMetaKindCatalogBundle(nakaginCapsuleTowerPuzzle2dFixture) ?? {},
);

const nakaginStoryKindCompatibility = puzzle2dFixtureMetaKindCompatibility(nakaginCapsuleTowerPuzzle2dFixture) ?? [];

type DefaultPuzzle2dGraphNode = {
	handles: { angle: number; handleKind: string; id: string }[];
	id: string;
	radius: number;
	x: number;
	y: number;
};

type DefaultPuzzle2dGraphEdge = { id: string; source: string; target: string };

type DefaultPuzzle2dGraph = { edges: DefaultPuzzle2dGraphEdge[]; nodes: DefaultPuzzle2dGraphNode[] };

const defaultPuzzle2dGraph: DefaultPuzzle2dGraph = {
	edges: [{ id: "link-1", source: "alpha.out", target: "beta.in" }],
	nodes: [
		{ handles: [{ angle: 0, handleKind: BUILTIN_PORT_HANDLE_KIND, id: "alpha.out" }], id: "alpha", radius: 44, x: 0, y: 0 },
		{ handles: [{ angle: Math.PI, handleKind: BUILTIN_PORT_HANDLE_KIND, id: "beta.in" }], id: "beta", radius: 40, x: 280, y: 120 },
	],
};

function Puzzle2dDeleteReconciler({ setGraph }: { setGraph: Dispatch<SetStateAction<DefaultPuzzle2dGraph>> }): null {
	usePuzzle2dEvent(
		"edgeDelete",
		useCallback(({ id }: { id: string }) => {
			setGraph((graph) => ({ ...graph, edges: graph.edges.filter((edge) => edge.id !== id) }));
		}, [setGraph]),
	);
	usePuzzle2dEvent(
		"nodeDelete",
		useCallback(({ id }: { id: string }) => {
			setGraph((graph) => {
				const node = graph.nodes.find((entry) => entry.id === id);
				const handleIds = new Set(node?.handles.map((handle) => handle.id) ?? []);
				return {
					edges: graph.edges.filter((edge) => !handleIds.has(edge.source) && !handleIds.has(edge.target)),
					nodes: graph.nodes.filter((entry) => entry.id !== id),
				};
			});
		}, [setGraph]),
	);
	return null;
}

function StatefulInteractivePuzzle2dScene(): ReactElement {
	const [graph, setGraph] = useState(() => defaultPuzzle2dGraph);
	return (
		<>
			<Puzzle2dDeleteReconciler setGraph={setGraph} />
			{graph.nodes.map((node) => (
				<Node id={node.id} key={node.id} radius={node.radius} x={node.x} y={node.y}>
					{node.handles.map((handle) => (
						<Handle angle={handle.angle} handleKind={handle.handleKind} id={handle.id} key={handle.id} />
					))}
				</Node>
			))}
			{graph.edges.map((edge) => (
				<Edge id={edge.id} key={edge.id} source={edge.source} target={edge.target} />
			))}
		</>
	);
}

/** 🗼 Full Nakagin Capsule Tower puzzle 2d from `nakagin-capsule-tower.2d.json` (regenerate via `nakagin-capsule-tower-board.generate.script.ts`). */
const nakaginCapsuleTowerPuzzle2dScene: ReactElement = (
	<>
		{nakaginCapsuleTowerPuzzle2d.nodes.map((node) =>
			node.shape === "rectangle" && node.width != null && node.height != null ? (
				<Node
					draggable={false}
					height={node.height}
					id={node.id}
					key={node.id}
					{...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {})}
					shape="rectangle"
					text={node.text}
					width={node.width}
					x={node.x}
					y={node.y}
				>
					{node.handles.map((handle) => (
						<Handle
							angle={handle.angle}
							color={handle.color}
							handleKind={handle.handleKind ?? BUILTIN_PORT_HANDLE_KIND}
							id={handle.id}
							key={handle.id}
							radius={handle.radius}
						/>
					))}
				</Node>
			) : (
				<Node
					draggable={false}
					id={node.id}
					key={node.id}
					{...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {})}
					radius={node.radius ?? 0}
					text={node.text}
					x={node.x}
					y={node.y}
				>
					{node.handles.map((handle) => (
						<Handle
							angle={handle.angle}
							color={handle.color}
							handleKind={handle.handleKind ?? BUILTIN_PORT_HANDLE_KIND}
							id={handle.id}
							key={handle.id}
							radius={handle.radius}
						/>
					))}
				</Node>
			),
		)}
		{nakaginCapsuleTowerPuzzle2d.edges.map((edge) => (
			<Edge id={edge.id} key={edge.id} source={edge.source} target={edge.target} />
		))}
	</>
);

export const Default: Story = {
	render: (args) => (
		<Puzzle2dCanvas {...args}>
			<StatefulInteractivePuzzle2dScene />
		</Puzzle2dCanvas>
	),
	args: {
		camera: { x: 0, y: 0, zoom: 1 },
		kindCatalogs: { ...DEFAULT_KIND_CATALOG_BUNDLE },
		height: 520,
		width: 720,
		worldRasterTiling: "none",
	},
};

export const WorldTileClip: Story = {
	render: (args) => (
		<Puzzle2dCanvas {...args}>
			<StatefulInteractivePuzzle2dScene />
		</Puzzle2dCanvas>
	),
	args: {
		...Default.args,
		worldRasterTiling: "world-clip",
	},
};

export const NakaginCapsuleTowerFlatSelection: Story = {
	render: (args) => (
		<Puzzle2dCanvas {...args}>
			{nakaginCapsuleTowerPuzzle2dScene}
		</Puzzle2dCanvas>
	),
	args: {
		...Default.args,
		camera: { ...nakaginCapsuleTowerPuzzle2d.camera },
		kindCatalogs: nakaginStoryKindCatalogs,
		kindCompatibility: nakaginStoryKindCompatibility,
	},
};
