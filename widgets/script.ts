#!/usr/bin/env bun
/** 🧭 `@widgets/react` task router: `bun ./script.ts <test|typecheck>`. */
import { strict as assert } from "node:assert";
import { BundleScript, ScriptRouter, runBundleScriptMain, runBunx } from "../repo/lib/js/src/index.ts";
import {
	computeGraphStats,
	forceGraphLayout,
	graphWidgetDataFromSemioLanguageGraph,
	lensFromNodeTypes,
	networkGraphDataFromTopologyExport,
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
		assert.deepEqual(
			data.edges.map((edge) => `${edge.source}->${edge.target}`),
			["brief->rules", "rules->parts", "brief->layout", "layout->eval", "parts->eval"],
		);
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
		assert.equal(topologyNetworkGraphFixture.nodes.length, 482);
		assert.equal(topologyNetworkGraphFixture.lenses?.length, 3);

		const combo = NETWORK_GRAPH_STORY_COMBOS[0]!;
		const comboLens = lensFromNodeTypes(topologyNetworkGraphFixture, combo.nodeTypes);
		assert.ok(comboLens.edgeTypes.includes("HAT_BAUTEILGRUPPE"));
		const subgraph = subgraphByNodeTypes(topologyNetworkGraphFixture, combo.nodeTypes);
		assert.ok(subgraph.nodes.every((node) => combo.nodeTypes.includes(node.type)));

		const positions = forceGraphLayout(curatedNetworkGraphFixture.nodes, curatedNetworkGraphFixture.edges, {
			width: 400,
			height: 300,
		});
		for (const node of curatedNetworkGraphFixture.nodes) {
			const position = positions.get(node.id);
			assert.ok(position);
			assert.ok(Number.isFinite(position.x));
			assert.ok(Number.isFinite(position.y));
		}

		const allNodeTypes = new Set(curatedNetworkGraphFixture.nodeTypes.map((nodeType) => nodeType.id));
		const allEdgeTypes = new Set(curatedNetworkGraphFixture.edgeTypes.map((edgeType) => edgeType.id));
		const stats = computeGraphStats(curatedNetworkGraphFixture, {
			activeNodeTypes: allNodeTypes,
			activeEdgeTypes: allEdgeTypes,
		});
		const nodesTotal = stats.find((row) => row.id === "nodes-total");
		assert.equal(nodesTotal?.value, "5");
		const isolated = stats.find((row) => row.id === "isolated");
		assert.equal(isolated?.value, "0");

		console.log("[widgets] graph and network graph fixture smoke tests passed.");
	}
}

class TypecheckScript extends BundleScript {
	run(segments: string[]): void {
		runBunx(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root);
	}
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("typecheck", TypecheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "typecheck" });
