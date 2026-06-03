#!/usr/bin/env bun
/** 🧭 `@widgets/react` task router: `bun ./script.ts <test|typecheck>`. */
import { strict as assert } from "node:assert";
import { BundleScript, ScriptRouter, runBundleScriptMain, runBunx } from "../repo/lib/js/src/index.ts";
import {
	buildViewGraph,
	circularGraphLayout,
	computeGraphStats,
	defaultViewState,
	forceGraphLayout,
	forceGraphLayoutPresets,
	graphLayoutRegistry,
	graphWidgetDataFromSemioLanguageGraph,
	gridGraphLayout,
	lensFromNodeTypes,
	networkGraphDataFromTopologyExport,
	normalizeGraph,
	suggestMode,
	subgraphByNodeTypes,
} from "./index.tsx";
import {
	curatedNetworkGraphFixture,
	NETWORK_GRAPH_STORY_COMBOS,
	semioLanguageGraphFixture,
	topologyNetworkGraphFixture,
} from "./fixtures/index.ts";
import topology from "./fixtures/topology.json";

class TestScript extends BundleScript {
	run(_segments: string[]): void {
		const data = graphWidgetDataFromSemioLanguageGraph(semioLanguageGraphFixture);
		assert.equal(data.nodes.length, 5);
		assert.equal(data.edges.length, 5);
		assert.throws(() =>
			graphWidgetDataFromSemioLanguageGraph({
				kind: "semio.graph",
				statements: [
					{ kind: "node", id: "kit", label: "Kit", at: [24, 32] },
					{ kind: "edge", source: "kit", target: "ghost" },
				],
			}),
		);

		const topologyData = networkGraphDataFromTopologyExport(topology);
		assert.equal(topologyData.nodes.length, 482);
		assert.equal(topologyData.edges.length, 1495);

		const curatedModel = normalizeGraph(curatedNetworkGraphFixture);
		assert.equal(curatedModel.counts.nodes, 5);
		assert.equal(curatedModel.degree.get("p1")?.out, 2);
		assert.ok(curatedModel.neighbors.get("bg1")?.has("av1"));

		const viewState = defaultViewState(curatedModel);
		const schemaView = buildViewGraph(curatedModel, { ...viewState, mode: "schema" }, { width: 400, height: 300 });
		assert.equal(schemaView.nodes.length, curatedModel.nodeTypes.length);
		assert.ok(schemaView.nodes.every((node) => node.isGroup));

		const egoView = buildViewGraph(
			curatedModel,
			{ ...viewState, mode: "ego", selectedNodeId: "p1", depth: 1, direction: "both" },
			{ width: 400, height: 300 },
		);
		assert.ok(egoView.nodes.some((node) => node.id === "p1"));

		const suggestion = suggestMode(curatedModel, viewState);
		assert.equal(suggestion.mode, "full");

		const topologyModel = normalizeGraph(topologyNetworkGraphFixture);
		const largeSuggestion = suggestMode(topologyModel, defaultViewState(topologyModel));
		assert.equal(largeSuggestion.mode, "schema");

		const combo = NETWORK_GRAPH_STORY_COMBOS[0]!;
		const comboLens = lensFromNodeTypes(topologyNetworkGraphFixture, combo.nodeTypes);
		assert.ok(comboLens.edgeTypes.includes("HAT_BAUTEILGRUPPE"));

		for (const layout of [forceGraphLayout, circularGraphLayout, gridGraphLayout]) {
			const positions = layout(curatedNetworkGraphFixture.nodes, curatedNetworkGraphFixture.edges, { width: 400, height: 300 });
			for (const node of curatedNetworkGraphFixture.nodes) {
				const position = positions.get(node.id);
				assert.ok(position);
				assert.ok(Number.isFinite(position.x));
				assert.ok(Number.isFinite(position.y));
			}
		}

		assert.ok(forceGraphLayoutPresets.every((preset) => preset.simulation != null));
		assert.ok(graphLayoutRegistry.find((entry) => entry.id === "force-balanced")?.simulation != null);
		assert.equal(graphLayoutRegistry.find((entry) => entry.id === "circular")?.simulation, undefined);

		const stats = computeGraphStats(curatedNetworkGraphFixture, {
			activeNodeTypes: new Set(curatedNetworkGraphFixture.nodeTypes.map((nodeType) => nodeType.id)),
			activeEdgeTypes: new Set(curatedNetworkGraphFixture.edgeTypes.map((edgeType) => edgeType.id)),
		});
		assert.equal(stats.find((row) => row.id === "nodes-total")?.value, "5");

		console.log("[widgets] graph mechanism smoke tests passed.");
	}
}

class TypecheckScript extends BundleScript {
	run(segments: string[]): void {
		runBunx(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root);
	}
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("typecheck", TypecheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "typecheck" });
