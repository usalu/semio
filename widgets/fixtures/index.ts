import {
	graphWidgetDataFromSemioLanguageGraph,
	lensFromNodeTypes,
	networkGraphDataFromTopologyExport,
	type NetworkGraphData,
	type NetworkLens,
	type SemioLanguageGraph,
} from "../index";
import topology from "./topology.json";

export const semioLanguageGraphFixture = {
	kind: "semio.graph",
	title: "Semio Graph",
	statements: [
		{ kind: "node", id: "brief", label: "Brief", at: [84, 84], tone: "accent" },
		{ kind: "node", id: "rules", label: "Rules", at: [240, 58], tone: "neutral" },
		{ kind: "node", id: "parts", label: "Parts", at: [400, 84], tone: "success" },
		{ kind: "node", id: "layout", label: "Layout", at: [156, 184], tone: "neutral" },
		{ kind: "node", id: "eval", label: "Eval", at: [324, 184], tone: "warning" },
		{ kind: "edge", source: "brief", target: "rules", label: "drives", tone: "accent" },
		{ kind: "edge", source: "rules", target: "parts", label: "selects", tone: "neutral" },
		{ kind: "edge", source: "brief", target: "layout", label: "frames", tone: "success" },
		{ kind: "edge", source: "layout", target: "eval", label: "measures", tone: "warning" },
		{ kind: "edge", source: "parts", target: "eval", label: "checks", tone: "accent" },
	],
} as const satisfies SemioLanguageGraph;

export const semioGraphWidgetFixture = graphWidgetDataFromSemioLanguageGraph(semioLanguageGraphFixture);

const topologyBase = networkGraphDataFromTopologyExport(topology);

export const topologyLenses: ReadonlyArray<NetworkLens> = [
	{
		id: "lens-reuse-flow",
		name: "Reuse Flow",
		description: "Component groups and reuse modes.",
		nodeTypes: ["Bauteilgruppe", "WiederverwendungsArt"],
		edgeTypes: ["HAT_WIEDERVERWENDUNGSART"],
	},
	{
		id: "lens-processing",
		name: "Processing",
		description: "Preparation and deconstruction vocabulary.",
		nodeTypes: ["Bauteilgruppe", "Aufbereitungsverfahren", "Rueckbauverfahren"],
		edgeTypes: ["HAT_AUFBEREITUNG", "HAT_RUECKBAUVERFAHREN", "IST_UNTERVERFAHREN_VON"],
	},
	{
		id: "lens-projects",
		name: "Projects",
		description: "Projects linked to component groups.",
		nodeTypes: ["Projekt", "Bauteilgruppe"],
		edgeTypes: ["HAT_BAUTEILGRUPPE"],
	},
];

export const topologyStatDefinitions = [
	{ id: "nodes-total", label: "Visible nodes", kind: "count" as const },
	{ id: "edges-total", label: "Visible edges", kind: "count" as const, numerator: "edges" },
	{ id: "density", label: "Edge density", kind: "ratio" as const },
	{ id: "isolated", label: "Isolated nodes", kind: "isolated" as const },
	{ id: "components", label: "Connected groups", kind: "components" as const },
	{ id: "degree", label: "Avg degree", kind: "degree" as const },
	{ id: "count-bauteilgruppe", label: "Bauteilgruppe", kind: "count" as const, nodeType: "Bauteilgruppe" },
	{ id: "count-projekt", label: "Projekt", kind: "count" as const, nodeType: "Projekt" },
	{
		id: "coverage-aufbereitung",
		label: "Bauteilgruppe · Aufbereitung",
		kind: "coverage" as const,
		nodeType: "Bauteilgruppe",
		edgeType: "HAT_AUFBEREITUNG",
	},
	{
		id: "compare-main",
		label: "Bauteilgruppe vs Projekt",
		kind: "compare" as const,
		nodeTypes: ["Bauteilgruppe", "Projekt"],
	},
];

export const topologyNetworkGraphFixture: NetworkGraphData = {
	...topologyBase,
	lenses: topologyLenses,
	statDefinitions: topologyStatDefinitions,
};

const curatedNetworkGraphBase: NetworkGraphData = {
	nodes: [
		{ id: "p1", type: "Projekt", label: "Pilot A" },
		{ id: "bg1", type: "Bauteilgruppe", label: "Batch Alpha" },
		{ id: "bg2", type: "Bauteilgruppe", label: "Batch Beta" },
		{ id: "av1", type: "Aufbereitungsverfahren", label: "Cleaning" },
		{ id: "wva1", type: "WiederverwendungsArt", label: "Direct reuse" },
	],
	edges: [
		{ id: "e1", source: "p1", target: "bg1", type: "HAT_BAUTEILGRUPPE" },
		{ id: "e2", source: "p1", target: "bg2", type: "HAT_BAUTEILGRUPPE" },
		{ id: "e3", source: "bg1", target: "av1", type: "HAT_AUFBEREITUNG" },
		{ id: "e4", source: "bg2", target: "wva1", type: "HAT_WIEDERVERWENDUNGSART" },
		{ id: "e5", source: "bg1", target: "wva1", type: "HAT_WIEDERVERWENDUNGSART" },
	],
	nodeTypes: [
		{ id: "Projekt", label: "Projekt", count: 1 },
		{ id: "Bauteilgruppe", label: "Bauteilgruppe", count: 2 },
		{ id: "Aufbereitungsverfahren", label: "Aufbereitungsverfahren", count: 1 },
		{ id: "WiederverwendungsArt", label: "WiederverwendungsArt", count: 1 },
	],
	edgeTypes: [
		{ id: "HAT_BAUTEILGRUPPE", label: "HAT_BAUTEILGRUPPE", count: 2 },
		{ id: "HAT_AUFBEREITUNG", label: "HAT_AUFBEREITUNG", count: 1 },
		{ id: "HAT_WIEDERVERWENDUNGSART", label: "HAT_WIEDERVERWENDUNGSART", count: 2 },
	],
};

/** @emoji 🧪 Small curated graph for fast Storybook previews. */
export const curatedNetworkGraphFixture: NetworkGraphData = {
	...curatedNetworkGraphBase,
	lenses: [
		lensFromNodeTypes(curatedNetworkGraphBase, ["Projekt", "Bauteilgruppe"]),
		lensFromNodeTypes(curatedNetworkGraphBase, ["Bauteilgruppe", "WiederverwendungsArt"]),
	],
};

export interface NetworkGraphStoryCombo {
	readonly name: string;
	readonly nodeTypes: ReadonlyArray<string>;
}

export const NETWORK_GRAPH_STORY_COMBOS: ReadonlyArray<NetworkGraphStoryCombo> = [
	{ name: "ProjectsAndComponents", nodeTypes: ["Projekt", "Bauteilgruppe"] },
	{ name: "ReuseModes", nodeTypes: ["Bauteilgruppe", "WiederverwendungsArt"] },
	{
		name: "ProcessingChain",
		nodeTypes: ["Bauteilgruppe", "Aufbereitungsverfahren", "Rueckbauverfahren"],
	},
	{ name: "SourcingToReuse", nodeTypes: ["Projekt", "Bauteilgruppe", "WiederverwendungsArt"] },
	{
		name: "FullRecoveryPath",
		nodeTypes: ["Projekt", "Bauteilgruppe", "Aufbereitungsverfahren", "WiederverwendungsArt"],
	},
];

export function networkGraphFixtureForCombo(combo: NetworkGraphStoryCombo): NetworkGraphData {
	return {
		...topologyNetworkGraphFixture,
		lenses: [
			...topologyLenses,
			lensFromNodeTypes(topologyNetworkGraphFixture, combo.nodeTypes),
		],
	};
}
