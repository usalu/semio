// #region 🧲Header
// 💻 .storybook/stories/semio/widgets/Graph.stories.tsx
// Specs: Storybook hosts the graph preview; widget components and fixture data stay in `widgets`.
// Summary: Setup story for the standalone graph widget rendered with semio styling tokens.
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import { GraphWidget } from "@widgets/react";
import { semioGraphWidgetFixture, semioLanguageGraphFixture } from "@widgets/react/fixtures";

const meta: Meta<typeof GraphWidget> = {
	title: "semio/widgets/Graph",
	component: GraphWidget,
	tags: ["autodocs"],
	parameters: { layout: "padded" },
};

export default meta;

type Story = StoryObj<typeof GraphWidget>;

const frame = (node: React.ReactNode) => (
	<div className="w-[min(42rem,100%)] border border-border bg-panel p-4 text-foreground shadow-none">{node}</div>
);

export const Default: Story = {
	args: {
		title: semioLanguageGraphFixture.title,
		subtitle: "Storybook target with components and fixture data kept in widgets.",
		nodes: semioGraphWidgetFixture.nodes,
		edges: semioGraphWidgetFixture.edges,
		width: 560,
		height: 300,
	},
	render: (args) => frame(<GraphWidget {...args} />),
};

export const Compact: Story = {
	args: {
		...Default.args,
		title: "Compact Graph",
		width: 420,
		height: 240,
	},
	render: (args) => frame(<GraphWidget {...args} />),
};
