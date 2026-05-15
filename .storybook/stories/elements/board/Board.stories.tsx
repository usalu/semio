// #region 🧲Header
// 💻 .storybook/stories/elements/board/Board.stories.tsx
// Specs: Host the elements board canvas for Storybook + Playwright raster/LOD/selection checks.
// Summary: Raster modes, JSON fixture (Nakagin flat center cluster), and Playwright harness stories.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import type { ReactElement } from "react";

import { BoardCanvas, Edge, Handle, Node } from "../../../../elements/client/lib/board/react/index.tsx";
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
	id: string;
	label: string;
	radius: number;
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

const boardSceneFixture: ReactElement = (
	<>
		<Node draggable id="alpha" radius={44} x={0} y={0}>
			<Handle angle={0} id="alpha.out" />
		</Node>
		<Node id="beta" radius={40} x={280} y={120}>
			<Handle angle={Math.PI} id="beta.in" />
		</Node>
		<Edge from="alpha.out" id="link-1" to="beta.in" />
	</>
);

/** 🗼 Nakagin center cluster from `nakagin-capsule-tower.board.json` (parent-design connections; handle ids follow piece-kind connector names, `link` when the kind exposes a single port). */
const nakaginCapsuleTowerBoardScene: ReactElement = (
	<>
		{nakaginCapsuleTowerBoard.nodes.map((node) => (
			<Node draggable={false} id={node.id} key={node.id} radius={node.radius} x={node.x} y={node.y}>
				{node.handles.map((handle) => (
					<Handle angle={handle.angle} id={handle.id} key={handle.id} />
				))}
			</Node>
		))}
			{nakaginCapsuleTowerBoard.edges.map((edge) => (
				<Edge from={edge.from} id={edge.id} key={edge.id} to={edge.to} />
			))}
	</>
);

export const Default: Story = {
	render: (args) => (
		<BoardCanvas {...args}>
			{boardSceneFixture}
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
			{boardSceneFixture}
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
