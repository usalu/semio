// #region 🧲Header
/** @emoji 🧩 `@widgets/react` — standalone React widgets styled against the semio UI token surface. */
// #endregion 🧲Header

// #region 🔌Adapters
import { forceCollide, forceLink, forceManyBody, forceSimulation, forceX, forceY } from "d3-force";
import {
	useCallback,
	useEffect,
	useMemo,
	useRef,
	useState,
	type ComponentPropsWithoutRef,
	type CSSProperties,
	type PointerEvent as ReactPointerEvent,
	type ReactNode,
	type WheelEvent as ReactWheelEvent,
} from "react";
// #endregion 🔌Adapters

// #region 🧰Types
export type WidgetTone = "neutral" | "accent" | "success" | "warning";
export type WidgetDensity = "compact" | "comfortable" | "spacious";

export interface WidgetGridProps extends ComponentPropsWithoutRef<"div"> {
	readonly minColumnWidth?: string;
}

export interface WidgetCardProps extends ComponentPropsWithoutRef<"section"> {
	readonly tone?: WidgetTone;
	readonly density?: WidgetDensity;
}

export interface WidgetHeadingProps extends Omit<ComponentPropsWithoutRef<"div">, "title"> {
	readonly eyebrow?: ReactNode;
	readonly title: ReactNode;
	readonly description?: ReactNode;
	readonly action?: ReactNode;
}

export interface WidgetMetricProps extends ComponentPropsWithoutRef<"article"> {
	readonly label: ReactNode;
	readonly value: ReactNode;
	readonly hint?: ReactNode;
	readonly tone?: WidgetTone;
}

export interface GraphWidgetNode {
	readonly id: string;
	readonly label: string;
	readonly x: number;
	readonly y: number;
	readonly radius?: number;
	readonly tone?: WidgetTone;
}

export interface GraphWidgetEdge {
	readonly source: string;
	readonly target: string;
	readonly label?: string;
	readonly tone?: WidgetTone;
}

export interface GraphWidgetProps extends Omit<ComponentPropsWithoutRef<"section">, "title"> {
	readonly title?: ReactNode;
	readonly subtitle?: ReactNode;
	readonly nodes: ReadonlyArray<GraphWidgetNode>;
	readonly edges: ReadonlyArray<GraphWidgetEdge>;
	readonly width?: number;
	readonly height?: number;
}

export interface GraphWidgetData {
	readonly nodes: ReadonlyArray<GraphWidgetNode>;
	readonly edges: ReadonlyArray<GraphWidgetEdge>;
}

export type SemioGraphStatement =
	| {
			readonly kind: "node";
			readonly id: string;
			readonly label: string;
			readonly at: readonly [number, number];
			readonly radius?: number;
			readonly tone?: WidgetTone;
	  }
	| {
			readonly kind: "edge";
			readonly source: string;
			readonly target: string;
			readonly label?: string;
			readonly tone?: WidgetTone;
	  };

export interface SemioLanguageGraph {
	readonly kind: "semio.graph";
	readonly title?: string;
	readonly statements: ReadonlyArray<SemioGraphStatement>;
}
// #endregion 🧰Types

// #region 🎨Theme
interface WidgetTonePalette {
	readonly border: string;
	readonly background: string;
	readonly foreground: string;
	readonly mutedForeground: string;
	readonly accent: string;
}

const classNames = (...values: Array<string | false | null | undefined>): string => values.filter(Boolean).join(" ");

const tonePaletteByTone: Record<WidgetTone, WidgetTonePalette> = {
	neutral: {
		border: "var(--border-window-color, #7b827d)",
		background: "color-mix(in srgb, var(--panel, #c9c8bd) 78%, white 22%)",
		foreground: "var(--foreground, #001117)",
		mutedForeground: "var(--muted-foreground, #7b827d)",
		accent: "var(--accent-secondary, #34d1bf)",
	},
	accent: {
		border: "var(--accent, #ff344f)",
		background: "color-mix(in srgb, var(--panel, #c9c8bd) 80%, var(--accent, #ff344f) 20%)",
		foreground: "var(--foreground, #001117)",
		mutedForeground: "var(--muted-foreground, #7b827d)",
		accent: "var(--accent, #ff344f)",
	},
	success: {
		border: "var(--success-border, #7eb77f)",
		background: "var(--success-bg, #dad9cb)",
		foreground: "var(--success-foreground, #001117)",
		mutedForeground: "var(--muted-foreground, #7b827d)",
		accent: "var(--success-border, #7eb77f)",
	},
	warning: {
		border: "var(--warning-border, #fccf05)",
		background: "var(--warning-bg, #ebe8d9)",
		foreground: "var(--warning-foreground, #001117)",
		mutedForeground: "var(--muted-foreground, #7b827d)",
		accent: "var(--warning-border, #fccf05)",
	},
};

const widgetGridStyle: CSSProperties = {
	display: "grid",
	gap: "1rem",
};

const widgetCardStyle: CSSProperties = {
	display: "grid",
	gap: "0.875rem",
	borderRadius: 0,
	borderStyle: "solid",
	borderWidth: 1,
	boxShadow: "0 0 0 1px color-mix(in srgb, var(--canvas, #f0ecdd) 70%, transparent)",
	fontFamily: "var(--font-sans, sans-serif)",
};

const densityStyleByDensity: Record<WidgetDensity, CSSProperties> = {
	compact: { padding: "0.875rem" },
	comfortable: { padding: "1.125rem" },
	spacious: { padding: "1.5rem" },
};

const eyebrowStyle: CSSProperties = {
	margin: 0,
	fontSize: "0.75rem",
	fontWeight: 700,
	letterSpacing: "0.08em",
	textTransform: "uppercase",
	color: "var(--muted-foreground, #7b827d)",
};

const titleStyle: CSSProperties = {
	margin: 0,
	fontSize: "1.125rem",
	fontWeight: 700,
	lineHeight: 1.2,
	color: "var(--foreground, #001117)",
};

const descriptionStyle: CSSProperties = {
	margin: 0,
	fontSize: "0.95rem",
	lineHeight: 1.5,
	color: "var(--muted-foreground, #7b827d)",
};

const metricValueStyle: CSSProperties = {
	margin: 0,
	fontSize: "clamp(1.8rem, 3vw, 2.5rem)",
	fontWeight: 800,
	lineHeight: 1,
	color: "var(--foreground, #001117)",
};

const metricLabelStyle: CSSProperties = {
	margin: 0,
	fontSize: "0.875rem",
	fontWeight: 600,
	color: "var(--muted-foreground, #7b827d)",
};

const metricHintStyle: CSSProperties = {
	margin: 0,
	fontSize: "0.8125rem",
	color: "var(--muted-foreground, #7b827d)",
};
// #endregion 🎨Theme

// #region 🧱Primitives
export function WidgetGrid({ minColumnWidth = "16rem", className, style, ...props }: WidgetGridProps) {
	return (
		<div
			{...props}
			className={classNames("widgets-grid", className)}
			style={{ ...widgetGridStyle, gridTemplateColumns: `repeat(auto-fit, minmax(${minColumnWidth}, 1fr))`, ...style }}
		/>
	);
}

export function WidgetCard({ tone = "neutral", density = "comfortable", className, style, ...props }: WidgetCardProps) {
	const palette = tonePaletteByTone[tone];
	return (
		<section
			{...props}
			data-widget-tone={tone}
			className={classNames("widget-card", className)}
			style={{
				...widgetCardStyle,
				...densityStyleByDensity[density],
				borderColor: palette.border,
				background: palette.background,
				color: palette.foreground,
				...style,
			}}
		/>
	);
}

export function WidgetHeading({ eyebrow, title, description, action, className, style, ...props }: WidgetHeadingProps) {
	return (
		<div
			{...props}
			className={classNames("widget-heading", className)}
			style={{ display: "grid", gap: "0.5rem", gridTemplateColumns: action ? "1fr auto" : undefined, alignItems: "start", ...style }}
		>
			<div style={{ display: "grid", gap: "0.35rem" }}>
				{eyebrow ? <p style={eyebrowStyle}>{eyebrow}</p> : null}
				<h2 style={titleStyle}>{title}</h2>
				{description ? <p style={descriptionStyle}>{description}</p> : null}
			</div>
			{action ? <div>{action}</div> : null}
		</div>
	);
}

export function WidgetMetric({ label, value, hint, tone = "neutral", className, style, ...props }: WidgetMetricProps) {
	const palette = tonePaletteByTone[tone];
	return (
		<article
			{...props}
			data-widget-tone={tone}
			className={classNames("widget-metric", className)}
			style={{
				...widgetCardStyle,
				...densityStyleByDensity.compact,
				borderColor: palette.border,
				background: palette.background,
				color: palette.foreground,
				...style,
			}}
		>
			<p style={metricLabelStyle}>{label}</p>
			<p style={metricValueStyle}>{value}</p>
			{hint ? <p style={metricHintStyle}>{hint}</p> : null}
		</article>
	);
}
// #endregion 🧱Primitives

// #region 🌐GraphWidget
function resolveGraphNode(nodes: ReadonlyArray<GraphWidgetNode>, id: string): GraphWidgetNode | undefined {
	return nodes.find((node) => node.id === id);
}

function resolveGraphPalette(tone: WidgetTone): WidgetTonePalette {
	return tonePaletteByTone[tone];
}

/** @emoji 🧬 Maps the lightweight semio graph language fixture into renderable widget data. */
export function graphWidgetDataFromSemioLanguageGraph(graph: SemioLanguageGraph): GraphWidgetData {
	const nodes = graph.statements
		.filter((statement): statement is Extract<SemioGraphStatement, { kind: "node" }> => statement.kind === "node")
		.map((statement) => ({
			id: statement.id,
			label: statement.label,
			x: statement.at[0],
			y: statement.at[1],
			radius: statement.radius,
			tone: statement.tone,
		}));
	const nodeIds = new Set(nodes.map((node) => node.id));
	const edges = graph.statements
		.filter((statement): statement is Extract<SemioGraphStatement, { kind: "edge" }> => statement.kind === "edge")
		.map((statement) => {
			if (!nodeIds.has(statement.source) || !nodeIds.has(statement.target)) {
				throw new Error(`semio graph edge references missing node: ${statement.source}->${statement.target}`);
			}
			return {
				source: statement.source,
				target: statement.target,
				label: statement.label,
				tone: statement.tone,
			};
		});
	return { nodes, edges };
}

/** @emoji 🌐 Graph widget styled against the semio CSS variable contract from `@ui/styling/ui.css`. */
export function GraphWidget({
	title = "Graph Widget",
	subtitle = "Setup probe wired to semio styling tokens.",
	nodes,
	edges,
	width = 480,
	height = 260,
	className,
	style,
	...props
}: GraphWidgetProps) {
	return (
		<WidgetCard
			{...props}
			tone="neutral"
			density="comfortable"
			className={classNames("graph-widget", className)}
			style={{ minHeight: height, ...style }}
		>
			<WidgetHeading title={title} description={subtitle} />
			<svg
				viewBox={`0 0 ${width} ${height}`}
				role="img"
				aria-label="Graph widget preview"
				style={{
					width: "100%",
					height,
					overflow: "visible",
					border: "1px solid var(--border-window-color, #7b827d)",
					background: "linear-gradient(180deg, var(--canvas, #f0ecdd), var(--window, #ebe8d9))",
				}}
			>
				{edges.map((edge, index) => {
					const source = resolveGraphNode(nodes, edge.source);
					const target = resolveGraphNode(nodes, edge.target);
					if (!source || !target) return null;
					const palette = resolveGraphPalette(edge.tone ?? "accent");
					const midX = (source.x + target.x) / 2;
					const midY = (source.y + target.y) / 2;
					return (
						<g key={`${edge.source}:${edge.target}:${index}`}>
							<line
								x1={source.x}
								y1={source.y}
								x2={target.x}
								y2={target.y}
								stroke={palette.accent}
								strokeWidth="2"
								strokeDasharray={edge.tone === "warning" ? "6 6" : undefined}
							/>
							{edge.label ? (
								<text
									x={midX}
									y={midY - 8}
									textAnchor="middle"
									fill={palette.mutedForeground}
									fontFamily="var(--font-mono, monospace)"
									fontSize="10"
								>
									{edge.label}
								</text>
							) : null}
						</g>
					);
				})}
				{nodes.map((node) => {
					const palette = resolveGraphPalette(node.tone ?? "neutral");
					const radius = node.radius ?? 16;
					return (
						<g key={node.id}>
							<circle cx={node.x} cy={node.y} r={radius} fill={palette.background} stroke={palette.border} strokeWidth="2" />
							<circle cx={node.x} cy={node.y} r="3" fill={palette.accent} />
							<text
								x={node.x}
								y={node.y + radius + 16}
								textAnchor="middle"
								fill={palette.foreground}
								fontFamily="var(--font-sans, sans-serif)"
								fontSize="12"
							>
								{node.label}
							</text>
						</g>
					);
				})}
			</svg>
		</WidgetCard>
	);
}
// #endregion 🌐GraphWidget

// #region 🕸NetworkGraphTypes
export interface NetworkNode {
	readonly id: string;
	readonly type: string;
	readonly label: string;
	readonly properties?: Readonly<Record<string, unknown>>;
}

export interface NetworkEdge {
	readonly id: string;
	readonly source: string;
	readonly target: string;
	readonly type: string;
}

export interface GraphNodeTypeDef {
	readonly id: string;
	readonly label: string;
	readonly color?: string;
	readonly count?: number;
}

export interface GraphEdgeTypeDef {
	readonly id: string;
	readonly label: string;
	readonly color?: string;
	readonly count?: number;
}

export interface NetworkLens {
	readonly id: string;
	readonly name: string;
	readonly description?: string;
	readonly nodeTypes: ReadonlyArray<string>;
	readonly edgeTypes: ReadonlyArray<string>;
}

export type GraphStatKind = "count" | "ratio" | "coverage" | "degree" | "isolated" | "components" | "compare";

export interface GraphStatDefinition {
	readonly id: string;
	readonly label: string;
	readonly kind: GraphStatKind;
	readonly nodeType?: string;
	readonly edgeType?: string;
	readonly nodeTypes?: ReadonlyArray<string>;
	readonly numerator?: string;
	readonly denominator?: string;
}

export interface GraphStatRow {
	readonly id: string;
	readonly label: string;
	readonly value: string;
	readonly hint?: string;
}

export interface NetworkGraphData {
	readonly nodes: ReadonlyArray<NetworkNode>;
	readonly edges: ReadonlyArray<NetworkEdge>;
	readonly nodeTypes: ReadonlyArray<GraphNodeTypeDef>;
	readonly edgeTypes: ReadonlyArray<GraphEdgeTypeDef>;
	readonly lenses?: ReadonlyArray<NetworkLens>;
	readonly statDefinitions?: ReadonlyArray<GraphStatDefinition>;
}

export interface TopologyExportNode {
	readonly elementId: string;
	readonly labels: ReadonlyArray<string>;
	readonly properties: Readonly<Record<string, unknown>>;
}

export interface TopologyExportEdge {
	readonly elementId: string;
	readonly type: string;
	readonly source: string;
	readonly target: string;
}

export interface TopologyExport {
	readonly nodetypes: ReadonlyArray<{ readonly label: string; readonly count: number }>;
	readonly edgetypes: ReadonlyArray<{ readonly type: string; readonly count: number }>;
	readonly nodes: ReadonlyArray<TopologyExportNode>;
	readonly edges: ReadonlyArray<TopologyExportEdge>;
}

export interface GraphLayoutOptions {
	readonly width?: number;
	readonly height?: number;
	readonly chargeStrength?: number;
	readonly linkDistance?: number;
	readonly collideRadius?: number;
	readonly centerStrength?: number;
}

export type GraphLayout = (
	nodes: ReadonlyArray<NetworkNode>,
	edges: ReadonlyArray<NetworkEdge>,
	options?: GraphLayoutOptions,
) => ReadonlyMap<string, { readonly x: number; readonly y: number }>;

export interface ForceGraphLayoutConfig {
	readonly chargeStrength: number;
	readonly linkDistance: number;
	readonly collideRadius: number;
	readonly centerStrength: number;
}

export const defaultForceGraphLayoutConfig: ForceGraphLayoutConfig = {
	chargeStrength: -120,
	linkDistance: 48,
	collideRadius: 14,
	centerStrength: 0.12,
};

export interface ComputeGraphStatsOptions {
	readonly activeNodeTypes: ReadonlySet<string>;
	readonly activeEdgeTypes: ReadonlySet<string>;
	readonly statDefinitions?: ReadonlyArray<GraphStatDefinition>;
	readonly selectedType?: string;
}

export interface NetworkGraphWidgetProps extends Omit<ComponentPropsWithoutRef<"section">, "title"> {
	readonly data: NetworkGraphData;
	readonly lenses?: ReadonlyArray<NetworkLens>;
	readonly statDefinitions?: ReadonlyArray<GraphStatDefinition>;
	readonly initialActiveNodeTypes?: ReadonlyArray<string>;
	readonly initialActiveEdgeTypes?: ReadonlyArray<string>;
	readonly initialLensName?: string;
	readonly initialSelectedNodeId?: string;
	readonly layout?: GraphLayout;
	readonly layoutOptions?: GraphLayoutOptions;
	readonly height?: number | string;
}
// #endregion 🕸NetworkGraphTypes

// #region 🕸NetworkGraphTheme
const NODE_TYPE_COLOR_TOKENS = [
	"var(--accent, #ff344f)",
	"var(--accent-secondary, #34d1bf)",
	"var(--accent-tertiary, #fa9500)",
	"var(--success-border, #7eb77f)",
	"var(--warning-border, #fccf05)",
	"var(--info-border, #dbbea1)",
] as const;

const CUSTOM_LENS_NAME = "Custom Lens";

const glassPanelStyle: CSSProperties = {
	background: "color-mix(in srgb, var(--panel, #c9c8bd) 62%, transparent)",
	backdropFilter: "blur(12px)",
	WebkitBackdropFilter: "blur(12px)",
	border: "1px solid var(--border-window-color, #7b827d)",
	borderRadius: 0,
	boxShadow: "none",
	fontFamily: "var(--font-sans, sans-serif)",
	color: "var(--foreground, #001117)",
};

const networkGraphShellStyle: CSSProperties = {
	position: "relative",
	display: "grid",
	width: "100%",
	minHeight: 420,
	height: "100%",
	overflow: "hidden",
	background: "linear-gradient(180deg, var(--canvas, #f0ecdd), var(--window, #ebe8d9))",
	border: "1px solid var(--border-window-color, #7b827d)",
};

const networkGraphPanelStyle: CSSProperties = {
	...glassPanelStyle,
	display: "grid",
	gap: "0.5rem",
	padding: "0.75rem",
	maxHeight: "100%",
	overflow: "auto",
	zIndex: 10,
};

const networkGraphChipStyle: CSSProperties = {
	display: "inline-flex",
	alignItems: "center",
	gap: "0.35rem",
	padding: "0.25rem 0.5rem",
	border: "1px solid var(--border-window-color, #7b827d)",
	background: "transparent",
	color: "var(--foreground, #001117)",
	fontFamily: "var(--font-sans, sans-serif)",
	fontSize: "0.75rem",
	fontWeight: 600,
	cursor: "pointer",
	borderRadius: 0,
};

function assignNodeTypeColors(nodeTypes: ReadonlyArray<GraphNodeTypeDef>): ReadonlyArray<GraphNodeTypeDef> {
	const sorted = [...nodeTypes].sort((a, b) => a.id.localeCompare(b.id));
	return nodeTypes.map((nodeType) => {
		const index = sorted.findIndex((entry) => entry.id === nodeType.id);
		return { ...nodeType, color: nodeType.color ?? NODE_TYPE_COLOR_TOKENS[index % NODE_TYPE_COLOR_TOKENS.length] };
	});
}

function nodeTypeColorMap(data: NetworkGraphData): ReadonlyMap<string, string> {
	const colored = assignNodeTypeColors(data.nodeTypes);
	return new Map(colored.map((nodeType) => [nodeType.id, nodeType.color ?? NODE_TYPE_COLOR_TOKENS[0]]));
}
// #endregion 🕸NetworkGraphTheme

// #region 🕸NetworkGraphAdapter
/** @emoji 🔌 Maps a Neo4j topology export JSON into {@link NetworkGraphData}. */
export function networkGraphDataFromTopologyExport(exportData: TopologyExport): NetworkGraphData {
	const nodes: NetworkNode[] = exportData.nodes.map((node) => {
		const type = node.labels[0];
		if (!type) throw new Error(`topology node missing label: ${node.elementId}`);
		const properties = node.properties;
		const label =
			(typeof properties.name === "string" && properties.name) ||
			(typeof properties.id === "string" && properties.id) ||
			node.elementId;
		return { id: node.elementId, type, label, properties };
	});
	const nodeIds = new Set(nodes.map((node) => node.id));
	const edges: NetworkEdge[] = exportData.edges.map((edge) => {
		if (!nodeIds.has(edge.source) || !nodeIds.has(edge.target)) {
			throw new Error(`topology edge references missing node: ${edge.source}->${edge.target}`);
		}
		return { id: edge.elementId, source: edge.source, target: edge.target, type: edge.type };
	});
	const nodeTypes = assignNodeTypeColors(
		exportData.nodetypes.map((entry) => ({ id: entry.label, label: entry.label, count: entry.count })),
	);
	const edgeTypes = exportData.edgetypes.map((entry) => ({ id: entry.type, label: entry.type, count: entry.count }));
	return { nodes, edges, nodeTypes, edgeTypes };
}

/** @emoji 🔭 Builds a lens from the selected node types and induced edge types. */
export function lensFromNodeTypes(data: NetworkGraphData, nodeTypeIds: ReadonlyArray<string>): NetworkLens {
	const nodeTypeSet = new Set(nodeTypeIds);
	const inducedEdgeTypes = new Set<string>();
	for (const edge of data.edges) {
		const source = data.nodes.find((node) => node.id === edge.source);
		const target = data.nodes.find((node) => node.id === edge.target);
		if (source && target && nodeTypeSet.has(source.type) && nodeTypeSet.has(target.type)) inducedEdgeTypes.add(edge.type);
	}
	const slug = nodeTypeIds.join("-").toLowerCase().replaceAll(/[^a-z0-9]+/g, "-");
	return {
		id: `lens-${slug}`,
		name: nodeTypeIds.join(" · "),
		nodeTypes: nodeTypeIds,
		edgeTypes: [...inducedEdgeTypes],
	};
}

/** @emoji ✂️ Returns a hard-filtered subgraph containing only the selected node types. */
export function subgraphByNodeTypes(data: NetworkGraphData, nodeTypeIds: ReadonlyArray<string>): NetworkGraphData {
	const nodeTypeSet = new Set(nodeTypeIds);
	const nodes = data.nodes.filter((node) => nodeTypeSet.has(node.type));
	const nodeIds = new Set(nodes.map((node) => node.id));
	const edges = data.edges.filter((edge) => nodeIds.has(edge.source) && nodeIds.has(edge.target));
	const nodeTypes = data.nodeTypes.filter((nodeType) => nodeTypeSet.has(nodeType.id));
	const edgeTypeIds = new Set(edges.map((edge) => edge.type));
	const edgeTypes = data.edgeTypes.filter((edgeType) => edgeTypeIds.has(edgeType.id));
	return { ...data, nodes, edges, nodeTypes, edgeTypes };
}
// #endregion 🕸NetworkGraphAdapter

// #region 🕸NetworkGraphLayout
interface ForceLayoutNode {
	readonly id: string;
	x?: number;
	y?: number;
}

interface ForceLayoutLink {
	readonly source: string;
	readonly target: string;
}

/** @emoji 🧲 Force-directed layout backed by d3-force (run-to-completion). */
export function forceGraphLayout(
	nodes: ReadonlyArray<NetworkNode>,
	edges: ReadonlyArray<NetworkEdge>,
	options: GraphLayoutOptions = {},
	config: ForceGraphLayoutConfig = defaultForceGraphLayoutConfig,
): ReadonlyMap<string, { readonly x: number; readonly y: number }> {
	if (nodes.length === 0) return new Map();
	const width = options.width ?? 800;
	const height = options.height ?? 600;
	const nodesCopy: ForceLayoutNode[] = nodes.map((node, index) => ({
		id: node.id,
		x: (index % 12) * 40 - width / 2,
		y: Math.floor(index / 12) * 40 - height / 2,
	}));
	const linksCopy: ForceLayoutLink[] = edges.map((edge) => ({ source: edge.source, target: edge.target }));
	const simulation = forceSimulation(nodesCopy)
		.force("charge", forceManyBody().strength(config.chargeStrength))
		.force(
			"link",
			forceLink<ForceLayoutNode, ForceLayoutLink>(linksCopy)
				.id((node) => node.id)
				.distance(config.linkDistance),
		)
		.force("collide", forceCollide(config.collideRadius))
		.force("x", forceX(0).strength(config.centerStrength))
		.force("y", forceY(0).strength(config.centerStrength))
		.stop();
	const numTicks = Math.ceil(Math.log(simulation.alphaMin()) / Math.log(1 - simulation.alphaDecay()));
	for (let tick = 0; tick < numTicks; tick++) simulation.tick();
	const positions = new Map<string, { readonly x: number; readonly y: number }>();
	for (const node of nodesCopy) positions.set(node.id, { x: node.x ?? 0, y: node.y ?? 0 });
	return positions;
}
// #endregion 🕸NetworkGraphLayout

// #region 🕸NetworkGraphStats
function visibleSubgraph(
	data: NetworkGraphData,
	activeNodeTypes: ReadonlySet<string>,
	activeEdgeTypes: ReadonlySet<string>,
): { nodes: NetworkNode[]; edges: NetworkEdge[] } {
	const nodes = data.nodes.filter((node) => activeNodeTypes.has(node.type));
	const nodeIds = new Set(nodes.map((node) => node.id));
	const edges = data.edges.filter(
		(edge) => activeEdgeTypes.has(edge.type) && nodeIds.has(edge.source) && nodeIds.has(edge.target),
	);
	return { nodes, edges };
}

function buildAdjacency(edges: ReadonlyArray<NetworkEdge>): Map<string, Set<string>> {
	const adjacency = new Map<string, Set<string>>();
	const touch = (id: string) => {
		if (!adjacency.has(id)) adjacency.set(id, new Set());
		return adjacency.get(id)!;
	};
	for (const edge of edges) {
		touch(edge.source).add(edge.target);
		touch(edge.target).add(edge.source);
	}
	return adjacency;
}

function countComponents(nodes: ReadonlyArray<NetworkNode>, edges: ReadonlyArray<NetworkEdge>): number {
	if (nodes.length === 0) return 0;
	const adjacency = buildAdjacency(edges);
	const visited = new Set<string>();
	let components = 0;
	const visit = (id: string) => {
		const stack = [id];
		while (stack.length > 0) {
			const current = stack.pop()!;
			if (visited.has(current)) continue;
			visited.add(current);
			for (const neighbor of adjacency.get(current) ?? []) {
				if (!visited.has(neighbor)) stack.push(neighbor);
			}
		}
	};
	for (const node of nodes) {
		if (visited.has(node.id)) continue;
		components += 1;
		visit(node.id);
	}
	return components;
}

function defaultStatDefinitions(data: NetworkGraphData): GraphStatDefinition[] {
	const defs: GraphStatDefinition[] = [
		{ id: "nodes-total", label: "Visible nodes", kind: "count" },
		{ id: "edges-total", label: "Visible edges", kind: "count", numerator: "edges" },
		{ id: "density", label: "Edge density", kind: "ratio" },
		{ id: "isolated", label: "Isolated nodes", kind: "isolated" },
		{ id: "components", label: "Connected groups", kind: "components" },
		{ id: "degree", label: "Avg degree", kind: "degree" },
	];
	for (const nodeType of data.nodeTypes) {
		defs.push({ id: `count-${nodeType.id}`, label: nodeType.label, kind: "count", nodeType: nodeType.id });
	}
	if (data.nodeTypes.length >= 2) {
		defs.push({
			id: "compare-main",
			label: "Type comparison",
			kind: "compare",
			nodeTypes: [data.nodeTypes[0]!.id, data.nodeTypes[1]!.id],
		});
	}
	const aufbereitung = data.edgeTypes.find((edgeType) => edgeType.id === "HAT_AUFBEREITUNG");
	if (aufbereitung) {
		defs.push({
			id: "coverage-aufbereitung",
			label: "Bauteilgruppe · Aufbereitung",
			kind: "coverage",
			nodeType: "Bauteilgruppe",
			edgeType: "HAT_AUFBEREITUNG",
		});
	}
	return defs;
}

function resolveStat(
	definition: GraphStatDefinition,
	subgraph: { nodes: NetworkNode[]; edges: NetworkEdge[] },
	selectedType?: string,
): GraphStatRow {
	const adjacency = buildAdjacency(subgraph.edges);
	const degrees = subgraph.nodes.map((node) => adjacency.get(node.id)?.size ?? 0);
	const countNodes = (nodeType?: string) =>
		nodeType ? subgraph.nodes.filter((node) => node.type === nodeType).length : subgraph.nodes.length;
	switch (definition.kind) {
		case "count": {
			if (definition.numerator === "edges") {
				return { id: definition.id, label: definition.label, value: String(subgraph.edges.length) };
			}
			const value = countNodes(definition.nodeType);
			return { id: definition.id, label: definition.label, value: String(value) };
		}
		case "ratio": {
			const nodes = countNodes();
			const edges = subgraph.edges.length;
			const value = nodes === 0 ? "—" : (edges / nodes).toFixed(2);
			return { id: definition.id, label: definition.label, value, hint: `${edges} / ${nodes}` };
		}
		case "coverage": {
			const nodeType = definition.nodeType;
			const edgeType = definition.edgeType;
			if (!nodeType || !edgeType) return { id: definition.id, label: definition.label, value: "—" };
			const typed = subgraph.nodes.filter((node) => node.type === nodeType);
			if (typed.length === 0) return { id: definition.id, label: definition.label, value: "0%" };
			const connected = new Set(
				subgraph.edges.filter((edge) => edge.type === edgeType).flatMap((edge) => [edge.source, edge.target]),
			);
			const covered = typed.filter((node) => connected.has(node.id)).length;
			const pct = Math.round((covered / typed.length) * 100);
			return { id: definition.id, label: definition.label, value: `${pct}%`, hint: `${covered} / ${typed.length}` };
		}
		case "degree": {
			if (degrees.length === 0) return { id: definition.id, label: definition.label, value: "0" };
			const scoped = definition.nodeType
				? subgraph.nodes
						.filter((node) => node.type === definition.nodeType)
						.map((node) => adjacency.get(node.id)?.size ?? 0)
				: degrees;
			if (scoped.length === 0) return { id: definition.id, label: definition.label, value: "0" };
			const avg = scoped.reduce((sum, degree) => sum + degree, 0) / scoped.length;
			const min = Math.min(...scoped);
			const max = Math.max(...scoped);
			return {
				id: definition.id,
				label: definition.label,
				value: avg.toFixed(1),
				hint: `min ${min} · max ${max}`,
			};
		}
		case "isolated": {
			const isolated = subgraph.nodes.filter((node) => (adjacency.get(node.id)?.size ?? 0) === 0).length;
			return { id: definition.id, label: definition.label, value: String(isolated) };
		}
		case "components": {
			return {
				id: definition.id,
				label: definition.label,
				value: String(countComponents(subgraph.nodes, subgraph.edges)),
			};
		}
		case "compare": {
			const types = definition.nodeTypes ?? [];
			if (selectedType && !types.includes(selectedType)) {
				const selectedCount = countNodes(selectedType);
				const rest = subgraph.nodes.length - selectedCount;
				return {
					id: definition.id,
					label: definition.label,
					value: `${selectedType}: ${selectedCount}`,
					hint: `other types: ${rest}`,
				};
			}
			const parts = types.map((nodeType) => `${nodeType}: ${countNodes(nodeType)}`);
			return { id: definition.id, label: definition.label, value: parts.join(" · ") || "—" };
		}
		default:
			return { id: definition.id, label: definition.label, value: "—" };
	}
}

/** @emoji 📊 Computes live graph metrics over the visible subgraph. */
export function computeGraphStats(data: NetworkGraphData, options: ComputeGraphStatsOptions): ReadonlyArray<GraphStatRow> {
	const subgraph = visibleSubgraph(data, options.activeNodeTypes, options.activeEdgeTypes);
	const definitions = options.statDefinitions ?? data.statDefinitions ?? defaultStatDefinitions(data);
	return definitions.map((definition) => resolveStat(definition, subgraph, options.selectedType));
}
// #endregion 🕸NetworkGraphStats

// #region 🕸NetworkGraphWidget
function fitTransform(
	positions: ReadonlyMap<string, { readonly x: number; readonly y: number }>,
	nodeIds: ReadonlyArray<string>,
	width: number,
	height: number,
	padding = 48,
): { x: number; y: number; k: number } {
	let minX = Infinity;
	let minY = Infinity;
	let maxX = -Infinity;
	let maxY = -Infinity;
	for (const id of nodeIds) {
		const position = positions.get(id);
		if (!position) continue;
		minX = Math.min(minX, position.x);
		minY = Math.min(minY, position.y);
		maxX = Math.max(maxX, position.x);
		maxY = Math.max(maxY, position.y);
	}
	if (!Number.isFinite(minX)) return { x: width / 2, y: height / 2, k: 1 };
	const boundsW = Math.max(maxX - minX, 1);
	const boundsH = Math.max(maxY - minY, 1);
	const k = Math.min((width - padding * 2) / boundsW, (height - padding * 2) / boundsH, 2);
	const cx = (minX + maxX) / 2;
	const cy = (minY + maxY) / 2;
	return { x: width / 2 - cx * k, y: height / 2 - cy * k, k };
}

/** @emoji 🕸 Interactive network graph with stats, lenses, and type filtering. */
export function NetworkGraphWidget({
	data,
	lenses: lensesProp,
	statDefinitions: statDefinitionsProp,
	initialActiveNodeTypes,
	initialActiveEdgeTypes,
	initialLensName,
	initialSelectedNodeId,
	layout = forceGraphLayout,
	layoutOptions,
	height = "100%",
	className,
	style,
	...props
}: NetworkGraphWidgetProps) {
	const shellRef = useRef<HTMLElement>(null);
	const [shellSize, setShellSize] = useState({ width: 800, height: 520 });
	const colors = useMemo(() => nodeTypeColorMap(data), [data]);
	const allNodeTypeIds = useMemo(() => data.nodeTypes.map((nodeType) => nodeType.id), [data.nodeTypes]);
	const allEdgeTypeIds = useMemo(() => data.edgeTypes.map((edgeType) => edgeType.id), [data.edgeTypes]);
	const lenses = lensesProp ?? data.lenses ?? [];
	const [activeNodeTypes, setActiveNodeTypes] = useState<Set<string>>(
		() => new Set(initialActiveNodeTypes ?? allNodeTypeIds),
	);
	const [activeEdgeTypes, setActiveEdgeTypes] = useState<Set<string>>(
		() => new Set(initialActiveEdgeTypes ?? allEdgeTypeIds),
	);
	const [activeLensName, setActiveLensName] = useState(initialLensName ?? lenses[0]?.name ?? CUSTOM_LENS_NAME);
	const [selectedNodeId, setSelectedNodeId] = useState<string | undefined>(initialSelectedNodeId);
	const [hoveredNodeId, setHoveredNodeId] = useState<string | undefined>();
	const [showLabels, setShowLabels] = useState(true);
	const [transform, setTransform] = useState({ x: 0, y: 0, k: 1 });
	const panRef = useRef<{ active: boolean; x: number; y: number; originX: number; originY: number }>({
		active: false,
		x: 0,
		y: 0,
		originX: 0,
		originY: 0,
	});

	useEffect(() => {
		const element = shellRef.current;
		if (!element) return;
		const observer = new ResizeObserver((entries) => {
			const entry = entries[0];
			if (!entry) return;
			setShellSize({ width: entry.contentRect.width, height: entry.contentRect.height });
		});
		observer.observe(element);
		return () => observer.disconnect();
	}, []);

	const positions = useMemo(
		() => layout(data.nodes, data.edges, { width: shellSize.width, height: shellSize.height, ...layoutOptions }),
		[data.nodes, data.edges, layout, layoutOptions, shellSize.height, shellSize.width],
	);

	const visibleNodes = useMemo(
		() => data.nodes.filter((node) => activeNodeTypes.has(node.type)),
		[data.nodes, activeNodeTypes],
	);
	const visibleNodeIds = useMemo(() => new Set(visibleNodes.map((node) => node.id)), [visibleNodes]);
	const visibleEdges = useMemo(
		() =>
			data.edges.filter(
				(edge) =>
					activeEdgeTypes.has(edge.type) && visibleNodeIds.has(edge.source) && visibleNodeIds.has(edge.target),
			),
		[data.edges, activeEdgeTypes, visibleNodeIds],
	);

	const adjacency = useMemo(() => buildAdjacency(visibleEdges), [visibleEdges]);
	const neighborIds = useMemo(() => {
		if (!selectedNodeId) return new Set<string>();
		const neighbors = adjacency.get(selectedNodeId) ?? new Set<string>();
		return new Set([selectedNodeId, ...neighbors]);
	}, [adjacency, selectedNodeId]);

	const selectedType = useMemo(
		() => (selectedNodeId ? data.nodes.find((node) => node.id === selectedNodeId)?.type : undefined),
		[data.nodes, selectedNodeId],
	);

	const stats = useMemo(
		() =>
			computeGraphStats(data, {
				activeNodeTypes,
				activeEdgeTypes,
				statDefinitions: statDefinitionsProp ?? data.statDefinitions,
				selectedType,
			}),
		[data, activeNodeTypes, activeEdgeTypes, statDefinitionsProp, selectedType],
	);

	const resetView = useCallback(() => {
		const fit = fitTransform(
			positions,
			visibleNodes.map((node) => node.id),
			shellSize.width,
			shellSize.height,
		);
		setTransform(fit);
	}, [positions, shellSize.height, shellSize.width, visibleNodes]);

	useEffect(() => {
		resetView();
	}, [resetView]);

	const applyLens = useCallback((lens: NetworkLens) => {
		setActiveNodeTypes(new Set(lens.nodeTypes));
		setActiveEdgeTypes(new Set(lens.edgeTypes.length > 0 ? lens.edgeTypes : allEdgeTypeIds));
		setActiveLensName(lens.name);
	}, [allEdgeTypeIds]);

	const toggleNodeType = useCallback(
		(nodeTypeId: string) => {
			setActiveNodeTypes((previous) => {
				const next = new Set(previous);
				if (next.has(nodeTypeId)) next.delete(nodeTypeId);
				else next.add(nodeTypeId);
				return next;
			});
			setActiveLensName(CUSTOM_LENS_NAME);
		},
		[],
	);

	const toggleEdgeType = useCallback((edgeTypeId: string) => {
		setActiveEdgeTypes((previous) => {
			const next = new Set(previous);
			if (next.has(edgeTypeId)) next.delete(edgeTypeId);
			else next.add(edgeTypeId);
			return next;
		});
		setActiveLensName(CUSTOM_LENS_NAME);
	}, []);

	const onWheel = useCallback((event: ReactWheelEvent<SVGSVGElement>) => {
		event.preventDefault();
		const rect = event.currentTarget.getBoundingClientRect();
		const px = event.clientX - rect.left;
		const py = event.clientY - rect.top;
		const factor = event.deltaY < 0 ? 1.12 : 0.88;
		setTransform((previous) => {
			const nextK = Math.min(4, Math.max(0.15, previous.k * factor));
			const graphX = (px - previous.x) / previous.k;
			const graphY = (py - previous.y) / previous.k;
			return { k: nextK, x: px - graphX * nextK, y: py - graphY * nextK };
		});
	}, []);

	const onPointerDown = useCallback((event: ReactPointerEvent<SVGSVGElement>) => {
		if (event.button !== 0) return;
		panRef.current = { active: true, x: event.clientX, y: event.clientY, originX: transform.x, originY: transform.y };
		event.currentTarget.setPointerCapture(event.pointerId);
	}, [transform.x, transform.y]);

	const onPointerMove = useCallback((event: ReactPointerEvent<SVGSVGElement>) => {
		if (!panRef.current.active) return;
		const dx = event.clientX - panRef.current.x;
		const dy = event.clientY - panRef.current.y;
		setTransform((previous) => ({ ...previous, x: panRef.current.originX + dx, y: panRef.current.originY + dy }));
	}, []);

	const onPointerUp = useCallback((event: ReactPointerEvent<SVGSVGElement>) => {
		panRef.current.active = false;
		event.currentTarget.releasePointerCapture(event.pointerId);
	}, []);

	const hoveredNode = hoveredNodeId ? data.nodes.find((node) => node.id === hoveredNodeId) : undefined;
	const labelMinZoom = 1.8;

	return (
		<section
			{...props}
			ref={shellRef}
			className={classNames("network-graph-widget", className)}
			style={{ ...networkGraphShellStyle, height, ...style }}
		>
			<aside
				style={{
					...networkGraphPanelStyle,
					position: "absolute",
					top: "0.75rem",
					left: "0.75rem",
					width: "min(14rem, calc(100% - 1.5rem))",
					maxWidth: "14rem",
				}}
			>
				<p style={eyebrowStyle}>Graph Stats</p>
				<p style={{ ...titleStyle, fontSize: "0.95rem" }}>Active Lens: {activeLensName}</p>
				<div style={{ display: "grid", gap: "0.35rem" }}>
					{stats.map((row) => (
						<div key={row.id} style={{ display: "grid", gap: "0.1rem" }}>
							<p style={{ ...metricLabelStyle, margin: 0, fontSize: "0.7rem" }}>{row.label}</p>
							<p
								style={{
									...metricValueStyle,
									margin: 0,
									fontSize: "1.1rem",
									fontFamily: "var(--font-mono, monospace)",
								}}
							>
								{row.value}
							</p>
							{row.hint ? <p style={{ ...metricHintStyle, margin: 0, fontSize: "0.65rem" }}>{row.hint}</p> : null}
						</div>
					))}
				</div>
			</aside>

			<aside
				style={{
					...networkGraphPanelStyle,
					position: "absolute",
					top: "0.75rem",
					right: "0.75rem",
					width: "min(15rem, calc(100% - 1.5rem))",
					maxWidth: "15rem",
				}}
			>
				<p style={eyebrowStyle}>Network Lenses</p>
				<div style={{ display: "grid", gap: "0.5rem" }}>
					{lenses.map((lens) => (
						<article
							key={lens.id}
							style={{
								...widgetCardStyle,
								padding: "0.5rem",
								borderColor: "var(--border-window-color, #7b827d)",
								background: "color-mix(in srgb, var(--window, #ebe8d9) 70%, transparent)",
							}}
						>
							<p style={{ ...titleStyle, fontSize: "0.85rem", margin: 0 }}>{lens.name}</p>
							{lens.description ? (
								<p style={{ ...descriptionStyle, fontSize: "0.75rem", margin: "0.25rem 0 0" }}>{lens.description}</p>
							) : null}
							<p style={{ ...metricHintStyle, margin: "0.35rem 0 0", fontSize: "0.65rem" }}>
								{lens.nodeTypes.join(", ")}
							</p>
							<button
								type="button"
								style={{
									...networkGraphChipStyle,
									marginTop: "0.35rem",
									borderColor: "var(--accent, #ff344f)",
								}}
								onClick={() => applyLens(lens)}
							>
								Apply Lens
							</button>
						</article>
					))}
				</div>
			</aside>

			<div
				style={{
					...networkGraphPanelStyle,
					position: "absolute",
					left: "0.75rem",
					right: "0.75rem",
					bottom: "0.75rem",
					gridTemplateColumns: "1fr auto",
					alignItems: "center",
					gap: "0.75rem",
					padding: "0.5rem 0.75rem",
				}}
			>
				<div style={{ display: "flex", flexWrap: "wrap", gap: "0.35rem", alignItems: "center" }}>
					<p style={{ ...eyebrowStyle, margin: 0, marginRight: "0.25rem" }}>Node Types</p>
					{data.nodeTypes.map((nodeType) => {
						const active = activeNodeTypes.has(nodeType.id);
						const color = colors.get(nodeType.id) ?? NODE_TYPE_COLOR_TOKENS[0];
						return (
							<button
								key={nodeType.id}
								type="button"
								style={{
									...networkGraphChipStyle,
									borderColor: color,
									background: active ? `color-mix(in srgb, ${color} 22%, transparent)` : "transparent",
								}}
								onClick={() => toggleNodeType(nodeType.id)}
							>
								<span
									style={{
										width: 8,
										height: 8,
										background: color,
										display: "inline-block",
									}}
								/>
								{nodeType.label}
								<span style={{ color: "var(--muted-foreground, #7b827d)" }}>{nodeType.count ?? ""}</span>
							</button>
						);
					})}
				</div>
				<div style={{ display: "flex", gap: "0.35rem", flexWrap: "wrap" }}>
					<button type="button" style={networkGraphChipStyle} onClick={() => setShowLabels((value) => !value)}>
						{showLabels ? "Hide labels" : "Show labels"}
					</button>
					<button type="button" style={networkGraphChipStyle} onClick={resetView}>
						Reset view
					</button>
				</div>
			</div>

			<div
				style={{
					position: "absolute",
					top: "0.5rem",
					left: "50%",
					transform: "translateX(-50%)",
					display: "flex",
					gap: "0.35rem",
					zIndex: 11,
				}}
			>
				{data.edgeTypes.map((edgeType) => {
					const active = activeEdgeTypes.has(edgeType.id);
					return (
						<button
							key={edgeType.id}
							type="button"
							style={{
								...networkGraphChipStyle,
								fontSize: "0.65rem",
								background: active ? "var(--window, #ebe8d9)" : "transparent",
							}}
							onClick={() => toggleEdgeType(edgeType.id)}
						>
							{edgeType.label}
						</button>
					);
				})}
			</div>

			<svg
				role="img"
				aria-label="Network graph canvas"
				style={{ width: "100%", height: "100%", cursor: panRef.current.active ? "grabbing" : "grab", touchAction: "none" }}
				onWheel={onWheel}
				onPointerDown={onPointerDown}
				onPointerMove={onPointerMove}
				onPointerUp={onPointerUp}
				onPointerLeave={onPointerUp}
			>
				<rect width="100%" height="100%" fill="transparent" />
				<g transform={`translate(${transform.x} ${transform.y}) scale(${transform.k})`}>
					{visibleEdges.map((edge) => {
						const source = positions.get(edge.source);
						const target = positions.get(edge.target);
						if (!source || !target) return null;
						const highlighted =
							!selectedNodeId || (neighborIds.has(edge.source) && neighborIds.has(edge.target));
						return (
							<line
								key={edge.id}
								x1={source.x}
								y1={source.y}
								x2={target.x}
								y2={target.y}
								stroke={highlighted ? "var(--muted-foreground, #7b827d)" : "color-mix(in srgb, var(--muted-foreground, #7b827d) 35%, transparent)"}
								strokeWidth={highlighted ? 1.5 : 1}
							/>
						);
					})}
					{visibleNodes.map((node) => {
						const position = positions.get(node.id);
						if (!position) return null;
						const color = colors.get(node.type) ?? NODE_TYPE_COLOR_TOKENS[0];
						const selected = selectedNodeId === node.id;
						const neighbor = selectedNodeId ? neighborIds.has(node.id) : true;
						const dimmed = selectedNodeId && !neighbor;
						const radius = node.type === "Projekt" ? 5 : node.type === "Bauteilgruppe" ? 4 : 6;
						return (
							<g
								key={node.id}
								style={{ cursor: "pointer" }}
								onPointerEnter={() => setHoveredNodeId(node.id)}
								onPointerLeave={() => setHoveredNodeId((current) => (current === node.id ? undefined : current))}
								onClick={(event) => {
									event.stopPropagation();
									setSelectedNodeId((current) => (current === node.id ? undefined : node.id));
								}}
							>
								<circle
									cx={position.x}
									cy={position.y}
									r={radius}
									fill={color}
									fillOpacity={dimmed ? 0.25 : 0.9}
									stroke={selected ? "var(--active-base, #ff344f)" : color}
									strokeWidth={selected ? 2.5 : 1}
								/>
								{showLabels && transform.k >= labelMinZoom ? (
									<text
										x={position.x}
										y={position.y}
										textAnchor="middle"
										dominantBaseline="central"
										fill="var(--foreground, #001117)"
										fontSize={Math.max(radius * 0.7, 6 / transform.k)}
										fontFamily="var(--font-sans, sans-serif)"
										style={{ pointerEvents: "none" }}
										opacity={dimmed ? 0.35 : 1}
									>
										{node.label.length > 12 ? `${node.label.slice(0, 11)}…` : node.label}
									</text>
								) : null}
							</g>
						);
					})}
				</g>
			</svg>

			{hoveredNode ? (
				<div
					style={{
						...glassPanelStyle,
						position: "absolute",
						bottom: "4.5rem",
						left: "50%",
						transform: "translateX(-50%)",
						padding: "0.5rem 0.75rem",
						pointerEvents: "none",
						zIndex: 12,
						fontSize: "0.8rem",
					}}
				>
					<strong>{hoveredNode.label}</strong>
					<span style={{ color: "var(--muted-foreground, #7b827d)" }}> · {hoveredNode.type}</span>
					<span style={{ fontFamily: "var(--font-mono, monospace)" }}>
						{" "}
						· deg {adjacency.get(hoveredNode.id)?.size ?? 0}
					</span>
				</div>
			) : null}
		</section>
	);
}
// #endregion 🕸NetworkGraphWidget
