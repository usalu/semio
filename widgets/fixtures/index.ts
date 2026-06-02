import type { GraphWidgetEdge, GraphWidgetNode } from "../index";

export const semioGraphWidgetFixture = {
	nodes: [
		{ id: "brief", label: "Brief", x: 84, y: 84, tone: "accent" },
		{ id: "rules", label: "Rules", x: 240, y: 58, tone: "neutral" },
		{ id: "parts", label: "Parts", x: 400, y: 84, tone: "success" },
		{ id: "layout", label: "Layout", x: 156, y: 184, tone: "neutral" },
		{ id: "eval", label: "Eval", x: 324, y: 184, tone: "warning" },
	] satisfies ReadonlyArray<GraphWidgetNode>,
	edges: [
		{ source: "brief", target: "rules", label: "drives", tone: "accent" },
		{ source: "rules", target: "parts", label: "selects", tone: "neutral" },
		{ source: "brief", target: "layout", label: "frames", tone: "success" },
		{ source: "layout", target: "eval", label: "measures", tone: "warning" },
		{ source: "parts", target: "eval", label: "checks", tone: "accent" },
	] satisfies ReadonlyArray<GraphWidgetEdge>,
} as const;