import type { CommandDescriptor } from "./types.ts";

//#region MediaGraphTypes
export const OS_MEDIA_FLOW_MODULE_ID = "os-media";
export type MediaGraphPort = {
	readonly id: string;
	readonly resourceKind?: string;
};

export type MediaGraphNode = {
	readonly id: string;
	readonly instanceId: string;
	readonly x: number;
	readonly y: number;
	readonly width: number;
	readonly height: number;
	readonly inputs?: readonly MediaGraphPort[];
	readonly outputs?: readonly MediaGraphPort[];
};

export type MediaGraphEdge = {
	readonly id: string;
	readonly sourceNodeId: string;
	readonly sourcePortId: string;
	readonly targetNodeId: string;
	readonly targetPortId: string;
};

export type MediaGraph = {
	readonly nodes: readonly MediaGraphNode[];
	readonly edges: readonly MediaGraphEdge[];
};
//#endregion MediaGraphTypes

//#region FixtureApply
function edgeKey(source: string, target: string): string {
	return `${source}→${target}`;
}

/** @emoji 🔁 Applies structural flow fixture edits as s-play media graph commands. */
export function applyFlowFixtureJsonToMediaGraphCommands(
	graph: MediaGraph,
	fixtureJson: string,
	controllerId: string,
): CommandDescriptor[] {
	const fixture = JSON.parse(fixtureJson) as {
		readonly widgets?: readonly { readonly id?: string; readonly kind?: string }[];
		readonly layout?: Record<string, { readonly x?: number; readonly y?: number }>;
		readonly synapses?: readonly {
			readonly id: string;
			readonly from: string;
			readonly to: string;
			readonly fromPort?: string;
			readonly toPort?: string;
		}[];
	};
	const commands: CommandDescriptor[] = [];
	const neuronWidgetIds = new Set(
		(fixture.widgets ?? [])
			.filter((widget) => widget.kind === "neuron")
			.map((widget) => String(widget.id ?? ""))
			.filter(Boolean),
	);
	for (const node of graph.nodes) {
		if (!neuronWidgetIds.has(node.id)) {
			commands.push({ controllerId, command: "removeAppInstance", args: { instanceId: node.instanceId } });
		}
	}
	const layout = fixture.layout ?? {};
	for (const node of graph.nodes) {
		const position = layout[node.id];
		if (!position || position.x == null || position.y == null) continue;
		const x = position.x - node.width / 2;
		const y = position.y - node.height / 2;
		if (Math.abs(node.x - x) > 0.5 || Math.abs(node.y - y) > 0.5) {
			commands.push({ controllerId, command: "moveMediaNode", args: { nodeId: node.id, x, y } });
		}
	}
	const beforeKeys = new Set(
		graph.edges.map((edge) => edgeKey(`${edge.sourceNodeId}:${edge.sourcePortId}`, `${edge.targetNodeId}:${edge.targetPortId}`)),
	);
	const afterKeys = new Set(
		(fixture.synapses ?? []).map((synapse) =>
			edgeKey(`${synapse.from}:${synapse.fromPort ?? ""}`, `${synapse.to}:${synapse.toPort ?? ""}`),
		),
	);
	for (const synapse of fixture.synapses ?? []) {
		const key = edgeKey(`${synapse.from}:${synapse.fromPort ?? ""}`, `${synapse.to}:${synapse.toPort ?? ""}`);
		if (beforeKeys.has(key)) continue;
		if (!synapse.fromPort || !synapse.toPort) continue;
		commands.push({
			controllerId,
			command: "connectMediaPorts",
			args: {
				sourceNodeId: synapse.from,
				sourcePortId: synapse.fromPort,
				targetNodeId: synapse.to,
				targetPortId: synapse.toPort,
			},
		});
	}
	for (const edge of graph.edges) {
		const key = edgeKey(`${edge.sourceNodeId}:${edge.sourcePortId}`, `${edge.targetNodeId}:${edge.targetPortId}`);
		if (!afterKeys.has(key)) {
			commands.push({ controllerId, command: "disconnectMediaEdge", args: { edgeId: edge.id } });
		}
	}
	return commands;
}

export function parseMediaGraphFromFixture(fixtureJson: string): MediaGraph {
	const fixture = JSON.parse(fixtureJson) as {
		readonly widgets?: readonly {
			readonly id?: string;
			readonly kind?: string;
			readonly params?: { readonly instanceId?: string };
		}[];
		readonly layout?: Record<string, { readonly x?: number; readonly y?: number }>;
		readonly synapses?: readonly {
			readonly id: string;
			readonly from: string;
			readonly to: string;
			readonly fromPort?: string;
			readonly toPort?: string;
		}[];
	};
	const nodes: MediaGraphNode[] = (fixture.widgets ?? [])
		.filter((widget) => widget.kind === "neuron" && widget.id && widget.params?.instanceId)
		.map((widget) => {
			const layout = fixture.layout?.[widget.id!] ?? { x: 80, y: 80 };
			return {
				id: widget.id!,
				instanceId: widget.params!.instanceId!,
				x: (layout.x ?? 80) - 90,
				y: (layout.y ?? 80) - 36,
				width: 180,
				height: 72,
			};
		});
	const edges: MediaGraphEdge[] = (fixture.synapses ?? []).map((synapse) => ({
		id: synapse.id,
		sourceNodeId: synapse.from,
		sourcePortId: synapse.fromPort ?? "",
		targetNodeId: synapse.to,
		targetPortId: synapse.toPort ?? "",
	}));
	return { nodes, edges };
}
//#endregion FixtureApply
