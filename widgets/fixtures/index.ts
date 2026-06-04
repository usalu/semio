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
	{
		id: "lens-deconstruction",
		name: "Deconstruction",
		description: "Component groups and deconstruction methods.",
		nodeTypes: ["Bauteilgruppe", "Rueckbauverfahren"],
		edgeTypes: ["HAT_RUECKBAUVERFAHREN"],
	},
	{
		id: "lens-method-hierarchy",
		name: "Method Hierarchy",
		description: "Sub-method relationships between processes.",
		nodeTypes: ["Aufbereitungsverfahren", "Rueckbauverfahren"],
		edgeTypes: ["IST_UNTERVERFAHREN_VON"],
	},
	{
		id: "lens-overview",
		name: "Full Overview",
		description: "Every node and edge type at once.",
		nodeTypes: ["Projekt", "Bauteilgruppe", "Aufbereitungsverfahren", "WiederverwendungsArt", "Rueckbauverfahren"],
		edgeTypes: [],
	},
];

export const topologyStatDefinitions = [
	{
		id: "compare-main",
		label: "Bauteilgruppe vs Projekt",
		kind: "compare" as const,
		nodeTypes: ["Bauteilgruppe", "Projekt"],
	},
	{ id: "nodes-total", label: "Visible nodes", kind: "count" as const },
	{ id: "edges-total", label: "Visible edges", kind: "count" as const, numerator: "edges" },
	{ id: "density", label: "Edge density", kind: "ratio" as const },
	{ id: "degree", label: "Avg degree", kind: "degree" as const },
	{ id: "components", label: "Connected groups", kind: "components" as const },
	{ id: "isolated", label: "Isolated nodes", kind: "isolated" as const },
	{
		id: "coverage-aufbereitung",
		label: "Bauteilgruppe · Aufbereitung",
		kind: "coverage" as const,
		nodeType: "Bauteilgruppe",
		edgeType: "HAT_AUFBEREITUNG",
	},
];

export const topologyNetworkGraphFixture: NetworkGraphData = {
	...topologyBase,
	lenses: topologyLenses,
	statDefinitions: topologyStatDefinitions,
};

/** @emoji 🏗️ Builds a mid-size circular-economy graph (8 node types, 7 edge types) for rich Storybook previews. */
function buildCuratedNetworkGraph(): NetworkGraphData {
	const nodes: Array<{ id: string; type: string; label: string }> = [];
	const edges: Array<{ id: string; source: string; target: string; type: string }> = [];
	const node = (id: string, type: string, label: string) => nodes.push({ id, type, label });
	const link = (source: string, target: string, type: string) =>
		edges.push({ id: `${source}->${target}:${type}`, source, target, type });

	const projekt = ["Pilot A", "Retrofit B", "Campus C", "Quay D"];
	const standort = ["Berlin", "Hamburg", "Munich"];
	const gruppe = ["Batch Alpha", "Batch Beta", "Batch Gamma", "Batch Delta", "Batch Epsilon", "Batch Zeta", "Batch Eta", "Batch Theta"];
	const bauteil = ["Beam", "Column", "Slab", "Panel", "Window", "Door", "Pipe", "Duct", "Cable", "Brick", "Tile", "Frame"];
	const material = ["Steel", "Concrete", "Timber", "Glass", "Aluminium"];
	const aufbereitung = ["Cleaning", "Grinding", "Coating", "Sorting"];
	const rueckbau = ["Manual strip", "Saw cutting", "Crushing"];
	const reuse = ["Direct reuse", "Remanufacture", "Downcycle", "Recycle"];

	projekt.forEach((label, index) => node(`p${index + 1}`, "Projekt", label));
	standort.forEach((label, index) => node(`st${index + 1}`, "Standort", label));
	gruppe.forEach((label, index) => node(`bg${index + 1}`, "Bauteilgruppe", label));
	bauteil.forEach((label, index) => node(`bt${index + 1}`, "Bauteil", label));
	material.forEach((label, index) => node(`m${index + 1}`, "Material", label));
	aufbereitung.forEach((label, index) => node(`av${index + 1}`, "Aufbereitungsverfahren", label));
	rueckbau.forEach((label, index) => node(`rv${index + 1}`, "Rueckbauverfahren", label));
	reuse.forEach((label, index) => node(`wva${index + 1}`, "WiederverwendungsArt", label));

	projekt.forEach((_, index) => link(`p${index + 1}`, `st${(index % standort.length) + 1}`, "LIEGT_AN_STANDORT"));
	gruppe.forEach((_, index) => link(`p${(index % projekt.length) + 1}`, `bg${index + 1}`, "HAT_BAUTEILGRUPPE"));
	bauteil.forEach((_, index) => link(`bg${(index % gruppe.length) + 1}`, `bt${index + 1}`, "ENTHAELT_BAUTEIL"));
	bauteil.forEach((_, index) => link(`bt${index + 1}`, `m${(index % material.length) + 1}`, "BESTEHT_AUS_MATERIAL"));
	for (let index = 0; index < 6; index++) link(`bg${index + 1}`, `av${(index % aufbereitung.length) + 1}`, "HAT_AUFBEREITUNG");
	for (let index = 2; index < gruppe.length; index++) link(`bg${index + 1}`, `rv${(index % rueckbau.length) + 1}`, "HAT_RUECKBAUVERFAHREN");
	gruppe.forEach((_, index) => link(`bg${index + 1}`, `wva${(index % reuse.length) + 1}`, "HAT_WIEDERVERWENDUNGSART"));

	const nodeTypeDefs = [
		{ id: "Projekt", label: "Projekt" },
		{ id: "Standort", label: "Standort" },
		{ id: "Bauteilgruppe", label: "Bauteilgruppe" },
		{ id: "Bauteil", label: "Bauteil" },
		{ id: "Material", label: "Material" },
		{ id: "Aufbereitungsverfahren", label: "Aufbereitungsverfahren" },
		{ id: "Rueckbauverfahren", label: "Rueckbauverfahren" },
		{ id: "WiederverwendungsArt", label: "WiederverwendungsArt" },
	];
	const edgeTypeIds = [
		"LIEGT_AN_STANDORT",
		"HAT_BAUTEILGRUPPE",
		"ENTHAELT_BAUTEIL",
		"BESTEHT_AUS_MATERIAL",
		"HAT_AUFBEREITUNG",
		"HAT_RUECKBAUVERFAHREN",
		"HAT_WIEDERVERWENDUNGSART",
	];
	return {
		nodes,
		edges,
		nodeTypes: nodeTypeDefs.map((type) => ({ ...type, count: nodes.filter((entry) => entry.type === type.id).length })),
		edgeTypes: edgeTypeIds.map((id) => ({ id, label: id, count: edges.filter((entry) => entry.type === id).length })),
	};
}

const curatedNetworkGraphBase = buildCuratedNetworkGraph();

const curatedLenses: ReadonlyArray<NetworkLens> = [
	{ id: "lens-projects-sites", name: "Projects & Sites", description: "Projects and where they are located.", nodeTypes: ["Projekt", "Standort"], edgeTypes: ["LIEGT_AN_STANDORT"] },
	{ id: "lens-component-breakdown", name: "Component Breakdown", description: "Groups, parts and their materials.", nodeTypes: ["Bauteilgruppe", "Bauteil", "Material"], edgeTypes: ["ENTHAELT_BAUTEIL", "BESTEHT_AUS_MATERIAL"] },
	{ id: "lens-reuse-flow", name: "Reuse Flow", description: "Component groups and reuse modes.", nodeTypes: ["Bauteilgruppe", "WiederverwendungsArt"], edgeTypes: ["HAT_WIEDERVERWENDUNGSART"] },
	{ id: "lens-processing", name: "Processing & Deconstruction", description: "Preparation and deconstruction vocabulary.", nodeTypes: ["Bauteilgruppe", "Aufbereitungsverfahren", "Rueckbauverfahren"], edgeTypes: ["HAT_AUFBEREITUNG", "HAT_RUECKBAUVERFAHREN"] },
	{ id: "lens-material-recovery", name: "Material Recovery", description: "From parts to materials to reuse.", nodeTypes: ["Bauteil", "Material", "WiederverwendungsArt"], edgeTypes: ["BESTEHT_AUS_MATERIAL", "HAT_WIEDERVERWENDUNGSART"] },
	{ id: "lens-supply", name: "Supply Chain", description: "Projects down to individual parts.", nodeTypes: ["Projekt", "Bauteilgruppe", "Bauteil"], edgeTypes: ["HAT_BAUTEILGRUPPE", "ENTHAELT_BAUTEIL"] },
	{ id: "lens-end-to-end", name: "End to End", description: "Project through processing into reuse.", nodeTypes: ["Projekt", "Bauteilgruppe", "Aufbereitungsverfahren", "WiederverwendungsArt"], edgeTypes: ["HAT_BAUTEILGRUPPE", "HAT_AUFBEREITUNG", "HAT_WIEDERVERWENDUNGSART"] },
];

const curatedStatDefinitions = [
	{ id: "nodes-total", label: "Visible nodes", kind: "count" as const },
	{ id: "edges-total", label: "Visible edges", kind: "count" as const, numerator: "edges" },
	{ id: "density", label: "Edge density", kind: "ratio" as const },
	{ id: "degree", label: "Avg degree", kind: "degree" as const },
	{ id: "components", label: "Connected groups", kind: "components" as const },
	{ id: "isolated", label: "Isolated nodes", kind: "isolated" as const },
	{ id: "coverage-reuse", label: "Bauteilgruppe · Reuse", kind: "coverage" as const, nodeType: "Bauteilgruppe", edgeType: "HAT_WIEDERVERWENDUNGSART" },
];

/** @emoji 🧪 Mid-size curated graph (43 nodes, 8 node types, 7 lenses) for Storybook previews. */
export const curatedNetworkGraphFixture: NetworkGraphData = {
	...curatedNetworkGraphBase,
	lenses: curatedLenses,
	statDefinitions: curatedStatDefinitions,
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
