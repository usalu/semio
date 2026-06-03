// #region 🧲Header
// 💻 .storybook/stories/semio/widgets/NetworkGraph.stories.tsx
// Specs: Storybook hosts network graph previews; widget and fixture data stay in `widgets`.
// Summary: Data-driven node-type combination stories over the topology fixture.
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import { lensFromNodeTypes, NetworkGraphWidget } from "@widgets/react";
import {
	curatedNetworkGraphFixture,
	NETWORK_GRAPH_STORY_COMBOS,
	networkGraphFixtureForCombo,
	topologyNetworkGraphFixture,
	type NetworkGraphStoryCombo,
} from "@widgets/react/fixtures";

const meta: Meta<typeof NetworkGraphWidget> = {
	title: "semio/widgets/NetworkGraph",
	component: NetworkGraphWidget,
	tags: ["autodocs"],
	parameters: { layout: "fullscreen" },
};

export default meta;

type Story = StoryObj<typeof NetworkGraphWidget>;

const frame = (node: React.ReactNode) => (
	<div style={{ width: "min(72rem, 100%)", height: "70vh", margin: "0 auto" }}>{node}</div>
);

export const Curated: Story = {
	render: () =>
		frame(
			<NetworkGraphWidget
				data={curatedNetworkGraphFixture}
				initialActiveNodeTypes={["Projekt", "Bauteilgruppe", "Aufbereitungsverfahren", "WiederverwendungsArt"]}
				initialLensName={curatedNetworkGraphFixture.lenses?.[0]?.name}
			/>,
		),
};

export const Topology: Story = {
	render: () =>
		frame(
			<NetworkGraphWidget
				data={topologyNetworkGraphFixture}
				initialLensName={topologyNetworkGraphFixture.lenses?.[0]?.name}
			/>,
		),
};

export const SchemaView: Story = {
	render: () =>
		frame(<NetworkGraphWidget data={topologyNetworkGraphFixture} initialActiveNodeTypes={topologyNetworkGraphFixture.nodeTypes.map((t) => t.id)} />),
};

export const EgoCurated: Story = {
	render: () =>
		frame(
			<NetworkGraphWidget
				data={curatedNetworkGraphFixture}
				initialSelectedNodeId="p1"
				initialActiveNodeTypes={["Projekt", "Bauteilgruppe", "Aufbereitungsverfahren", "WiederverwendungsArt"]}
			/>,
		),
};

function comboStory(combo: NetworkGraphStoryCombo): Story {
	const fixture = networkGraphFixtureForCombo(combo);
	const lens = lensFromNodeTypes(fixture, combo.nodeTypes);
	return {
		render: () =>
			frame(
				<NetworkGraphWidget
					data={fixture}
					initialActiveNodeTypes={[...combo.nodeTypes]}
					initialActiveEdgeTypes={[...lens.edgeTypes]}
					initialLensName={lens.name}
				/>,
			),
	};
}

export const ProjectsAndComponents = comboStory(NETWORK_GRAPH_STORY_COMBOS[0]!);
export const ReuseModes = comboStory(NETWORK_GRAPH_STORY_COMBOS[1]!);
export const ProcessingChain = comboStory(NETWORK_GRAPH_STORY_COMBOS[2]!);
export const SourcingToReuse = comboStory(NETWORK_GRAPH_STORY_COMBOS[3]!);
export const FullRecoveryPath = comboStory(NETWORK_GRAPH_STORY_COMBOS[4]!);
