import { graphWidgetDataFromSemioLanguageGraph } from "../index";
import type { SemioLanguageGraph } from "../index";

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
