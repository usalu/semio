// #region 🧲Header
// 💻 .storybook/stories/elements/board/Board.stories.tsx
// Specs: Host the elements board canvas for Storybook + Playwright raster/LOD/selection checks.
// Summary: Two raster modes (`none` vs `world-clip`) over one stable node/handle/edge layout.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import type { ReactElement } from "react";

import { BoardCanvas, Edge, Handle, Node } from "../../../../elements/client/lib/board/react/index.tsx";

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

function BoardSceneFixture(): ReactElement {
	return (
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
}

export const Default: Story = {
	render: (args) => (
		<BoardCanvas {...args}>
			<BoardSceneFixture />
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
			<BoardSceneFixture />
		</BoardCanvas>
	),
	args: {
		...Default.args,
		worldRasterTiling: "world-clip",
	},
};
