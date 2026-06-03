// #region 🧲Header
/** @emoji 🧩 `@widgets/react` — standalone React widgets styled against the semio UI token surface. */
// #endregion 🧲Header

// #region 🔌Adapters
import { forceCollide, forceLink, forceManyBody, forceSimulation, forceX, forceY, type Simulation } from "d3-force";
import {
	useCallback,
	useEffect,
	useMemo,
	useReducer,
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
	readonly layouts?: ReadonlyArray<NamedGraphLayout>;
	readonly initialLayoutId?: string;
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
	background: "color-mix(in srgb, var(--panel, #c9c8bd) 34%, transparent)",
	backdropFilter: "blur(14px)",
	WebkitBackdropFilter: "blur(14px)",
	border: "1px solid color-mix(in srgb, var(--border-window-color, #7b827d) 70%, transparent)",
	borderRadius: 0,
	boxShadow: "none",
	fontFamily: "var(--font-sans, sans-serif)",
	color: "var(--foreground, #001117)",
};

const networkGraphShellStyle: CSSProperties = {
	position: "relative",
	display: "flex",
	flexDirection: "column",
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
	boxSizing: "border-box",
	zIndex: 10,
};

const networkGraphChipStyle: CSSProperties = {
	display: "inline-flex",
	alignItems: "center",
	gap: "0.35rem",
	padding: "0.25rem 0.5rem",
	border: "1px solid var(--border-window-color, #7b827d)",
	background: "color-mix(in srgb, var(--panel, #c9c8bd) 30%, transparent)",
	backdropFilter: "blur(8px)",
	WebkitBackdropFilter: "blur(8px)",
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

export interface NamedGraphLayout {
	readonly id: string;
	readonly name: string;
	readonly layout: GraphLayout;
	readonly simulation?: ForceGraphLayoutConfig;
}

/** @emoji 🧲 Builds a force layout bound to a fixed {@link ForceGraphLayoutConfig}. */
export function forceGraphLayoutWithConfig(config: ForceGraphLayoutConfig): GraphLayout {
	return (nodes, edges, options) => forceGraphLayout(nodes, edges, options, config);
}

const forceGraphLayoutConfigs: ReadonlyArray<{ id: string; name: string; config: ForceGraphLayoutConfig }> = [
	{ id: "force-balanced", name: "Force · Balanced", config: defaultForceGraphLayoutConfig },
	{ id: "force-spread", name: "Force · Spread", config: { chargeStrength: -280, linkDistance: 96, collideRadius: 22, centerStrength: 0.05 } },
	{ id: "force-tight", name: "Force · Tight", config: { chargeStrength: -55, linkDistance: 26, collideRadius: 9, centerStrength: 0.22 } },
	{ id: "force-clustered", name: "Force · Clustered", config: { chargeStrength: -160, linkDistance: 30, collideRadius: 8, centerStrength: 0.02 } },
];

export const forceGraphLayoutPresets: ReadonlyArray<NamedGraphLayout> = forceGraphLayoutConfigs.map((entry) => ({
	id: entry.id,
	name: entry.name,
	layout: forceGraphLayoutWithConfig(entry.config),
	simulation: entry.config,
}));

/** @emoji 📐 Circular layout ordered by type then label. */
export function circularGraphLayout(
	nodes: ReadonlyArray<NetworkNode>,
	_edges: ReadonlyArray<NetworkEdge>,
	options: GraphLayoutOptions = {},
): ReadonlyMap<string, { readonly x: number; readonly y: number }> {
	const width = options.width ?? 800;
	const height = options.height ?? 600;
	const radius = Math.min(width, height) * 0.35;
	const sorted = [...nodes].sort((a, b) => a.type.localeCompare(b.type) || a.label.localeCompare(b.label));
	const positions = new Map<string, { x: number; y: number }>();
	sorted.forEach((node, index) => {
		const angle = (index / Math.max(sorted.length, 1)) * Math.PI * 2;
		positions.set(node.id, { x: Math.cos(angle) * radius, y: Math.sin(angle) * radius });
	});
	return positions;
}

/** @emoji 📐 Grid layout by type and label. */
export function gridGraphLayout(
	nodes: ReadonlyArray<NetworkNode>,
	_edges: ReadonlyArray<NetworkEdge>,
	options: GraphLayoutOptions = {},
): ReadonlyMap<string, { readonly x: number; readonly y: number }> {
	const width = options.width ?? 800;
	const cols = Math.ceil(Math.sqrt(nodes.length));
	const cell = Math.max(40, width / Math.max(cols, 1));
	const sorted = [...nodes].sort((a, b) => a.type.localeCompare(b.type) || a.label.localeCompare(b.label));
	const positions = new Map<string, { x: number; y: number }>();
	sorted.forEach((node, index) => {
		const col = index % cols;
		const row = Math.floor(index / cols);
		positions.set(node.id, { x: col * cell - width / 2, y: row * cell - (options.height ?? 600) / 4 });
	});
	return positions;
}

/** @emoji 📐 Radial BFS rings from highest-degree node. */
export function radialGraphLayout(
	nodes: ReadonlyArray<NetworkNode>,
	edges: ReadonlyArray<NetworkEdge>,
	options: GraphLayoutOptions = {},
): ReadonlyMap<string, { readonly x: number; readonly y: number }> {
	if (nodes.length === 0) return new Map();
	const adjacency = buildAdjacency(edges);
	const root = [...nodes].sort((a, b) => (adjacency.get(b.id)?.size ?? 0) - (adjacency.get(a.id)?.size ?? 0))[0]!;
	const levels = new Map<string, number>();
	const queue = [root.id];
	levels.set(root.id, 0);
	while (queue.length > 0) {
		const current = queue.shift()!;
		for (const neighbor of adjacency.get(current) ?? []) {
			if (levels.has(neighbor)) continue;
			levels.set(neighbor, (levels.get(current) ?? 0) + 1);
			queue.push(neighbor);
		}
	}
	const byLevel = new Map<number, string[]>();
	for (const node of nodes) {
		const level = levels.get(node.id) ?? 1;
		const bucket = byLevel.get(level) ?? [];
		bucket.push(node.id);
		byLevel.set(level, bucket);
	}
	const positions = new Map<string, { x: number; y: number }>();
	const ringGap = 55;
	for (const [level, ids] of byLevel) {
		ids.forEach((id, index) => {
			const angle = (index / Math.max(ids.length, 1)) * Math.PI * 2;
			positions.set(id, { x: Math.cos(angle) * level * ringGap, y: Math.sin(angle) * level * ringGap });
		});
	}
	return positions;
}

export const graphLayoutRegistry: ReadonlyArray<NamedGraphLayout> = [
	...forceGraphLayoutPresets,
	{ id: "circular", name: "Circular", layout: circularGraphLayout },
	{ id: "grid", name: "Grid", layout: gridGraphLayout },
	{ id: "radial", name: "Radial", layout: radialGraphLayout },
	{ id: "manual-pinned", name: "Manual · Pinned", layout: forceGraphLayout, simulation: defaultForceGraphLayoutConfig },
];
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
	const defs: GraphStatDefinition[] = [];
	if (data.nodeTypes.length >= 2) {
		defs.push({
			id: "compare-main",
			label: `${data.nodeTypes[0]!.label} vs ${data.nodeTypes[1]!.label}`,
			kind: "compare",
			nodeTypes: [data.nodeTypes[0]!.id, data.nodeTypes[1]!.id],
		});
	}
	defs.push(
		{ id: "nodes-total", label: "Visible nodes", kind: "count" },
		{ id: "edges-total", label: "Visible edges", kind: "count", numerator: "edges" },
		{ id: "density", label: "Edge density", kind: "ratio" },
		{ id: "degree", label: "Avg degree", kind: "degree" },
		{ id: "components", label: "Connected groups", kind: "components" },
		{ id: "isolated", label: "Isolated nodes", kind: "isolated" },
	);
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

// #region 🕸NetworkGraphReadability
export const NODE_READABLE_MAX = 150;
export const NODE_RENDER_MAX = 1500;
export const EDGE_READABLE_MAX = 800;
export const EDGE_RENDER_MAX = 3000;
export const LABEL_MAX = 120;
// #endregion 🕸NetworkGraphReadability

// #region 🕸NetworkGraphModel
export interface GraphDegree {
	readonly in: number;
	readonly out: number;
	readonly total: number;
}

export interface GraphCounts {
	readonly nodes: number;
	readonly edges: number;
	readonly nodeTypes: number;
	readonly edgeTypes: number;
	readonly isolated: number;
	readonly selfLoops: number;
}

export interface GraphModel {
	readonly data: NetworkGraphData;
	readonly nodes: ReadonlyArray<NetworkNode>;
	readonly edges: ReadonlyArray<NetworkEdge>;
	readonly nodeTypes: ReadonlyArray<GraphNodeTypeDef>;
	readonly edgeTypes: ReadonlyArray<GraphEdgeTypeDef>;
	readonly nodeById: ReadonlyMap<string, NetworkNode>;
	readonly edgeById: ReadonlyMap<string, NetworkEdge>;
	readonly outgoing: ReadonlyMap<string, ReadonlyArray<string>>;
	readonly incoming: ReadonlyMap<string, ReadonlyArray<string>>;
	readonly neighbors: ReadonlyMap<string, ReadonlySet<string>>;
	readonly nodesByType: ReadonlyMap<string, ReadonlyArray<NetworkNode>>;
	readonly edgesByType: ReadonlyMap<string, ReadonlyArray<NetworkEdge>>;
	readonly degree: ReadonlyMap<string, GraphDegree>;
	readonly propertyKeysByType: ReadonlyMap<string, ReadonlySet<string>>;
	readonly counts: GraphCounts;
	readonly warnings: ReadonlyArray<string>;
	readonly temporalFields: ReadonlyArray<string>;
	readonly locationFields: ReadonlyArray<string>;
	readonly colors: ReadonlyMap<string, string>;
}

const TEMPORAL_PROPERTY_KEYS = new Set(["year_completed", "exported_at", "date", "timestamp", "created_at"]);
const LOCATION_PROPERTY_KEYS = new Set(["lat", "lng", "latitude", "longitude", "location", "geo"]);

/** @emoji 🧬 Normalizes {@link NetworkGraphData} into indexed graph structures for the view pipeline. */
export function normalizeGraph(data: NetworkGraphData): GraphModel {
	const nodeTypes = assignNodeTypeColors(data.nodeTypes);
	const colors = new Map(nodeTypes.map((nodeType) => [nodeType.id, nodeType.color ?? NODE_TYPE_COLOR_TOKENS[0]]));
	const nodeById = new Map(data.nodes.map((node) => [node.id, node]));
	const edgeById = new Map(data.edges.map((edge) => [edge.id, edge]));
	const outgoing = new Map<string, string[]>();
	const incoming = new Map<string, string[]>();
	const neighbors = new Map<string, Set<string>>();
	const nodesByType = new Map<string, NetworkNode[]>();
	const edgesByType = new Map<string, NetworkEdge[]>();
	const degree = new Map<string, GraphDegree>();
	const propertyKeysByType = new Map<string, Set<string>>();
	let selfLoops = 0;
	for (const node of data.nodes) {
		degree.set(node.id, { in: 0, out: 0, total: 0 });
		const bucket = nodesByType.get(node.type) ?? [];
		bucket.push(node);
		nodesByType.set(node.type, bucket);
		const keys = propertyKeysByType.get(node.type) ?? new Set<string>();
		for (const key of Object.keys(node.properties ?? {})) keys.add(key);
		propertyKeysByType.set(node.type, keys);
	}
	for (const edge of data.edges) {
		const out = outgoing.get(edge.source) ?? [];
		out.push(edge.id);
		outgoing.set(edge.source, out);
		const inc = incoming.get(edge.target) ?? [];
		inc.push(edge.id);
		incoming.set(edge.target, inc);
		const sourceNeighbors = neighbors.get(edge.source) ?? new Set();
		sourceNeighbors.add(edge.target);
		neighbors.set(edge.source, sourceNeighbors);
		const targetNeighbors = neighbors.get(edge.target) ?? new Set();
		targetNeighbors.add(edge.source);
		neighbors.set(edge.target, targetNeighbors);
		const edgeBucket = edgesByType.get(edge.type) ?? [];
		edgeBucket.push(edge);
		edgesByType.set(edge.type, edgeBucket);
		if (edge.source === edge.target) selfLoops += 1;
		const sourceDegree = degree.get(edge.source)!;
		const targetDegree = degree.get(edge.target)!;
		degree.set(edge.source, { in: sourceDegree.in, out: sourceDegree.out + 1, total: sourceDegree.total + 1 });
		degree.set(edge.target, { in: targetDegree.in + 1, out: targetDegree.out, total: targetDegree.total + 1 });
	}
	const isolated = [...degree.values()].filter((entry) => entry.total === 0).length;
	const temporalFields = [...new Set([...propertyKeysByType.values()].flatMap((keys) => [...keys]).filter((key) => TEMPORAL_PROPERTY_KEYS.has(key)))];
	const locationFields = [...new Set([...propertyKeysByType.values()].flatMap((keys) => [...keys]).filter((key) => LOCATION_PROPERTY_KEYS.has(key)))];
	const warnings: string[] = [];
	if (isolated > 0) warnings.push(`${isolated} isolated nodes`);
	if (data.nodes.length > NODE_READABLE_MAX) warnings.push(`Large graph (${data.nodes.length} nodes); schema view recommended`);
	return {
		data,
		nodes: data.nodes,
		edges: data.edges,
		nodeTypes,
		edgeTypes: data.edgeTypes,
		nodeById,
		edgeById,
		outgoing,
		incoming,
		neighbors,
		nodesByType,
		edgesByType,
		degree,
		propertyKeysByType,
		counts: {
			nodes: data.nodes.length,
			edges: data.edges.length,
			nodeTypes: nodeTypes.length,
			edgeTypes: data.edgeTypes.length,
			isolated,
			selfLoops,
		},
		warnings,
		temporalFields,
		locationFields,
		colors,
	};
}
// #endregion 🕸NetworkGraphModel

// #region 🕸NetworkGraphState
export type VisualizationMode = "auto" | "schema" | "full" | "subgraph" | "ego" | "path" | "process" | "clustered" | "matrix";
export type NeighborhoodDirection = "in" | "out" | "both";
export type GroupingMode = "none" | "type" | "property";
export type AggregationMode = "none" | "byType" | "byGroup";
export type NodeSizeMetric = "uniform" | "degree" | "in" | "out";
export type EdgeWidthMetric = "uniform" | "count";
export type PropertyFilterOp = "eq" | "neq" | "contains" | "exists";

export interface PropertyFilter {
	readonly key: string;
	readonly op: PropertyFilterOp;
	readonly value?: string;
}

export interface GraphViewTransform {
	readonly x: number;
	readonly y: number;
	readonly k: number;
}

export interface GraphViewState {
	readonly mode: VisualizationMode;
	readonly layoutId: string;
	readonly activeNodeTypes: ReadonlySet<string>;
	readonly activeEdgeTypes: ReadonlySet<string>;
	readonly activeLensName: string;
	readonly selectedNodeId?: string;
	readonly selectedEdgeId?: string;
	readonly secondNodeId?: string;
	readonly propertyFilters: ReadonlyArray<PropertyFilter>;
	readonly searchQuery: string;
	readonly depth: number;
	readonly direction: NeighborhoodDirection;
	readonly groupingMode: GroupingMode;
	readonly aggregationMode: AggregationMode;
	readonly showLabels: boolean;
	readonly showEdges: boolean;
	readonly nodeSizeMetric: NodeSizeMetric;
	readonly edgeWidthMetric: EdgeWidthMetric;
	readonly pinnedNodeIds: ReadonlySet<string>;
	readonly collapsedGroupIds: ReadonlySet<string>;
	readonly highlightedPath: ReadonlyArray<string>;
	readonly transform: GraphViewTransform;
	readonly settingsOpen: boolean;
	readonly hoveredNodeId?: string;
}

export type GraphViewAction =
	| { readonly type: "setMode"; readonly mode: VisualizationMode }
	| { readonly type: "setLayoutId"; readonly layoutId: string }
	| { readonly type: "toggleNodeType"; readonly nodeTypeId: string }
	| { readonly type: "toggleEdgeType"; readonly edgeTypeId: string }
	| { readonly type: "setActiveNodeTypes"; readonly nodeTypeIds: ReadonlyArray<string> }
	| { readonly type: "setActiveEdgeTypes"; readonly edgeTypeIds: ReadonlyArray<string> }
	| { readonly type: "applyLens"; readonly lens: NetworkLens; readonly allEdgeTypeIds: ReadonlyArray<string> }
	| { readonly type: "selectNode"; readonly nodeId?: string }
	| { readonly type: "selectEdge"; readonly edgeId?: string }
	| { readonly type: "setSecondNode"; readonly nodeId?: string }
	| { readonly type: "setSearchQuery"; readonly searchQuery: string }
	| { readonly type: "setDepth"; readonly depth: number }
	| { readonly type: "setDirection"; readonly direction: NeighborhoodDirection }
	| { readonly type: "setGroupingMode"; readonly groupingMode: GroupingMode }
	| { readonly type: "setAggregationMode"; readonly aggregationMode: AggregationMode }
	| { readonly type: "setShowLabels"; readonly showLabels: boolean }
	| { readonly type: "setShowEdges"; readonly showEdges: boolean }
	| { readonly type: "setNodeSizeMetric"; readonly nodeSizeMetric: NodeSizeMetric }
	| { readonly type: "setEdgeWidthMetric"; readonly edgeWidthMetric: EdgeWidthMetric }
	| { readonly type: "togglePin"; readonly nodeId: string }
	| { readonly type: "toggleGroupCollapse"; readonly groupId: string }
	| { readonly type: "setHighlightedPath"; readonly path: ReadonlyArray<string> }
	| { readonly type: "setTransform"; readonly transform: GraphViewTransform }
	| { readonly type: "setSettingsOpen"; readonly settingsOpen: boolean }
	| { readonly type: "setHoveredNode"; readonly nodeId?: string }
	| { readonly type: "addPropertyFilter"; readonly filter: PropertyFilter }
	| { readonly type: "removePropertyFilter"; readonly index: number }
	| { readonly type: "resetView"; readonly model: GraphModel }
	| { readonly type: "isolateSelection"; readonly model: GraphModel };

export interface DefaultViewStateOptions {
	readonly initialActiveNodeTypes?: ReadonlyArray<string>;
	readonly initialActiveEdgeTypes?: ReadonlyArray<string>;
	readonly initialLensName?: string;
	readonly initialSelectedNodeId?: string;
	readonly initialLayoutId?: string;
}

/** @emoji 🎛️ Initial view state derived from the normalized graph model. */
export function defaultViewState(model: GraphModel, options: DefaultViewStateOptions = {}): GraphViewState {
	return {
		mode: model.counts.nodes > NODE_READABLE_MAX ? "schema" : "auto",
		layoutId: options.initialLayoutId ?? "force-balanced",
		activeNodeTypes: new Set(options.initialActiveNodeTypes ?? model.nodeTypes.map((nodeType) => nodeType.id)),
		activeEdgeTypes: new Set(options.initialActiveEdgeTypes ?? model.edgeTypes.map((edgeType) => edgeType.id)),
		activeLensName: options.initialLensName ?? model.data.lenses?.[0]?.name ?? CUSTOM_LENS_NAME,
		selectedNodeId: options.initialSelectedNodeId,
		propertyFilters: [],
		searchQuery: "",
		depth: 1,
		direction: "both",
		groupingMode: "none",
		aggregationMode: "none",
		showLabels: true,
		showEdges: true,
		nodeSizeMetric: "degree",
		edgeWidthMetric: "uniform",
		pinnedNodeIds: new Set(),
		collapsedGroupIds: new Set(),
		highlightedPath: [],
		transform: { x: 0, y: 0, k: 1 },
		settingsOpen: false,
	};
}

/** @emoji 🎛️ Reducer for {@link GraphViewState}. */
export function graphViewReducer(state: GraphViewState, action: GraphViewAction): GraphViewState {
	switch (action.type) {
		case "setMode":
			return { ...state, mode: action.mode };
		case "setLayoutId":
			return { ...state, layoutId: action.layoutId };
		case "toggleNodeType": {
			const next = new Set(state.activeNodeTypes);
			if (next.has(action.nodeTypeId)) next.delete(action.nodeTypeId);
			else next.add(action.nodeTypeId);
			return { ...state, activeNodeTypes: next, activeLensName: CUSTOM_LENS_NAME };
		}
		case "toggleEdgeType": {
			const next = new Set(state.activeEdgeTypes);
			if (next.has(action.edgeTypeId)) next.delete(action.edgeTypeId);
			else next.add(action.edgeTypeId);
			return { ...state, activeEdgeTypes: next, activeLensName: CUSTOM_LENS_NAME };
		}
		case "setActiveNodeTypes":
			return { ...state, activeNodeTypes: new Set(action.nodeTypeIds) };
		case "setActiveEdgeTypes":
			return { ...state, activeEdgeTypes: new Set(action.edgeTypeIds) };
		case "applyLens":
			return {
				...state,
				activeNodeTypes: new Set(action.lens.nodeTypes),
				activeEdgeTypes: new Set(action.lens.edgeTypes.length > 0 ? action.lens.edgeTypes : action.allEdgeTypeIds),
				activeLensName: action.lens.name,
			};
		case "selectNode":
			return { ...state, selectedNodeId: action.nodeId, selectedEdgeId: undefined, mode: action.nodeId && state.mode === "auto" ? "ego" : state.mode };
		case "selectEdge":
			return { ...state, selectedEdgeId: action.edgeId, selectedNodeId: undefined };
		case "setSecondNode":
			return { ...state, secondNodeId: action.nodeId, mode: action.nodeId && state.selectedNodeId ? "path" : state.mode };
		case "setSearchQuery":
			return { ...state, searchQuery: action.searchQuery };
		case "setDepth":
			return { ...state, depth: action.depth };
		case "setDirection":
			return { ...state, direction: action.direction };
		case "setGroupingMode":
			return { ...state, groupingMode: action.groupingMode };
		case "setAggregationMode":
			return { ...state, aggregationMode: action.aggregationMode };
		case "setShowLabels":
			return { ...state, showLabels: action.showLabels };
		case "setShowEdges":
			return { ...state, showEdges: action.showEdges };
		case "setNodeSizeMetric":
			return { ...state, nodeSizeMetric: action.nodeSizeMetric };
		case "setEdgeWidthMetric":
			return { ...state, edgeWidthMetric: action.edgeWidthMetric };
		case "togglePin": {
			const next = new Set(state.pinnedNodeIds);
			if (next.has(action.nodeId)) next.delete(action.nodeId);
			else next.add(action.nodeId);
			return { ...state, pinnedNodeIds: next, layoutId: "manual-pinned" };
		}
		case "toggleGroupCollapse": {
			const next = new Set(state.collapsedGroupIds);
			if (next.has(action.groupId)) next.delete(action.groupId);
			else next.add(action.groupId);
			return { ...state, collapsedGroupIds: next };
		}
		case "setHighlightedPath":
			return { ...state, highlightedPath: action.path };
		case "setTransform":
			return { ...state, transform: action.transform };
		case "setSettingsOpen":
			return { ...state, settingsOpen: action.settingsOpen };
		case "setHoveredNode":
			return { ...state, hoveredNodeId: action.nodeId };
		case "addPropertyFilter":
			return { ...state, propertyFilters: [...state.propertyFilters, action.filter] };
		case "removePropertyFilter":
			return { ...state, propertyFilters: state.propertyFilters.filter((_, index) => index !== action.index) };
		case "resetView":
			return defaultViewState(action.model, {
				initialActiveNodeTypes: [...state.activeNodeTypes],
				initialActiveEdgeTypes: [...state.activeEdgeTypes],
				initialLensName: state.activeLensName,
				initialLayoutId: state.layoutId,
			});
		case "isolateSelection": {
			if (!state.selectedNodeId) return state;
			const neighborIds = action.model.neighbors.get(state.selectedNodeId) ?? new Set<string>();
			const nodeIds = new Set([state.selectedNodeId, ...neighborIds]);
			const types = new Set(
				[...nodeIds]
					.map((id) => action.model.nodeById.get(id)?.type)
					.filter((type): type is string => Boolean(type)),
			);
			return { ...state, activeNodeTypes: types, mode: "subgraph", activeLensName: CUSTOM_LENS_NAME };
		}
		default:
			return state;
	}
}
// #endregion 🕸NetworkGraphState

// #region 🕸NetworkGraphModes
export interface ModeGraph {
	readonly nodes: NetworkNode[];
	readonly edges: NetworkEdge[];
	readonly schema?: boolean;
}

function filterByTypes(model: GraphModel, state: GraphViewState): ModeGraph {
	const nodes = model.nodes.filter((node) => state.activeNodeTypes.has(node.type));
	const nodeIds = new Set(nodes.map((node) => node.id));
	const edges = model.edges.filter(
		(edge) => state.activeEdgeTypes.has(edge.type) && nodeIds.has(edge.source) && nodeIds.has(edge.target),
	);
	return { nodes, edges };
}

function matchesPropertyFilters(node: NetworkNode, filters: ReadonlyArray<PropertyFilter>): boolean {
	for (const filter of filters) {
		const value = node.properties?.[filter.key];
		if (filter.op === "exists") {
			if (value === undefined || value === null || value === "") return false;
			continue;
		}
		const text = String(value ?? "");
		if (filter.op === "eq" && text !== (filter.value ?? "")) return false;
		if (filter.op === "neq" && text === (filter.value ?? "")) return false;
		if (filter.op === "contains" && !text.toLowerCase().includes((filter.value ?? "").toLowerCase())) return false;
	}
	return true;
}

function filterBySearch(nodes: NetworkNode[], query: string): NetworkNode[] {
	const trimmed = query.trim().toLowerCase();
	if (!trimmed) return nodes;
	return nodes.filter((node) => {
		if (node.label.toLowerCase().includes(trimmed) || node.id.toLowerCase().includes(trimmed)) return true;
		for (const value of Object.values(node.properties ?? {})) {
			if (String(value).toLowerCase().includes(trimmed)) return true;
		}
		return false;
	});
}

function applySchemaMode(model: GraphModel, state: GraphViewState): ModeGraph {
	const nodes: NetworkNode[] = model.nodeTypes
		.filter((nodeType) => state.activeNodeTypes.has(nodeType.id))
		.map((nodeType) => ({
			id: `schema:${nodeType.id}`,
			type: nodeType.id,
			label: nodeType.label,
			properties: { count: nodeType.count ?? model.nodesByType.get(nodeType.id)?.length ?? 0 },
		}));
	const nodeIds = new Set(nodes.map((node) => node.id));
	const edgeCounts = new Map<string, { type: string; source: string; target: string; count: number }>();
	for (const edge of model.edges) {
		if (!state.activeEdgeTypes.has(edge.type)) continue;
		const sourceNode = model.nodeById.get(edge.source);
		const targetNode = model.nodeById.get(edge.target);
		if (!sourceNode || !targetNode) continue;
		if (!state.activeNodeTypes.has(sourceNode.type) || !state.activeNodeTypes.has(targetNode.type)) continue;
		const sourceId = `schema:${sourceNode.type}`;
		const targetId = `schema:${targetNode.type}`;
		if (!nodeIds.has(sourceId) || !nodeIds.has(targetId)) continue;
		const key = `${edge.type}:${sourceId}:${targetId}`;
		const existing = edgeCounts.get(key);
		if (existing) existing.count += 1;
		else edgeCounts.set(key, { type: edge.type, source: sourceId, target: targetId, count: 1 });
	}
	const edges: NetworkEdge[] = [...edgeCounts.values()].map((entry, index) => ({
		id: `schema-edge:${index}`,
		source: entry.source,
		target: entry.target,
		type: entry.type,
	}));
	return { nodes, edges, schema: true };
}

function bfsNeighborhood(
	model: GraphModel,
	rootId: string,
	depth: number,
	direction: NeighborhoodDirection,
): Set<string> {
	const visited = new Set<string>([rootId]);
	const queue: Array<{ id: string; level: number }> = [{ id: rootId, level: 0 }];
	while (queue.length > 0) {
		const current = queue.shift()!;
		if (current.level >= depth) continue;
		if (direction === "out" || direction === "both") {
			for (const edgeId of model.outgoing.get(current.id) ?? []) {
				const edge = model.edgeById.get(edgeId);
				if (!edge) continue;
				if (!visited.has(edge.target)) {
					visited.add(edge.target);
					queue.push({ id: edge.target, level: current.level + 1 });
				}
			}
		}
		if (direction === "in" || direction === "both") {
			for (const edgeId of model.incoming.get(current.id) ?? []) {
				const edge = model.edgeById.get(edgeId);
				if (!edge) continue;
				if (!visited.has(edge.source)) {
					visited.add(edge.source);
					queue.push({ id: edge.source, level: current.level + 1 });
				}
			}
		}
	}
	return visited;
}

function shortestPathNodeIds(model: GraphModel, startId: string, endId: string): string[] {
	if (startId === endId) return [startId];
	const queue = [startId];
	const previous = new Map<string, string>();
	const visited = new Set([startId]);
	while (queue.length > 0) {
		const current = queue.shift()!;
		if (current === endId) {
			const path = [endId];
			let cursor = previous.get(endId);
			while (cursor) {
				path.unshift(cursor);
				cursor = previous.get(cursor);
			}
			return path;
		}
		for (const neighbor of model.neighbors.get(current) ?? []) {
			if (visited.has(neighbor)) continue;
			visited.add(neighbor);
			previous.set(neighbor, current);
			queue.push(neighbor);
		}
	}
	return [];
}

function applyEgoMode(model: GraphModel, state: GraphViewState, base: ModeGraph): ModeGraph {
	if (!state.selectedNodeId) return applySchemaMode(model, state);
	const allowed = bfsNeighborhood(model, state.selectedNodeId, state.depth, state.direction);
	const nodes = base.nodes.filter((node) => allowed.has(node.id));
	const nodeIds = new Set(nodes.map((node) => node.id));
	const edges = base.edges.filter((edge) => nodeIds.has(edge.source) && nodeIds.has(edge.target));
	return { nodes, edges };
}

function applyPathMode(model: GraphModel, state: GraphViewState, base: ModeGraph): ModeGraph {
	if (!state.selectedNodeId || !state.secondNodeId) return applyEgoMode(model, state, base);
	const pathIds = shortestPathNodeIds(model, state.selectedNodeId, state.secondNodeId);
	if (pathIds.length === 0) return applyEgoMode(model, state, base);
	const allowed = new Set(pathIds);
	const nodes = base.nodes.filter((node) => allowed.has(node.id));
	const nodeIds = new Set(nodes.map((node) => node.id));
	const edges = base.edges.filter((edge) => nodeIds.has(edge.source) && nodeIds.has(edge.target));
	return { nodes, edges };
}

function applyModeGraph(model: GraphModel, state: GraphViewState, mode: VisualizationMode): ModeGraph {
	let base = filterByTypes(model, state);
	base = { nodes: filterBySearch(base.nodes, state.searchQuery), edges: base.edges };
	base = {
		nodes: base.nodes.filter((node) => matchesPropertyFilters(node, state.propertyFilters)),
		edges: base.edges,
	};
	const nodeIds = new Set(base.nodes.map((node) => node.id));
	base = { nodes: base.nodes, edges: base.edges.filter((edge) => nodeIds.has(edge.source) && nodeIds.has(edge.target)) };
	switch (mode) {
		case "schema":
			return applySchemaMode(model, state);
		case "ego":
			return applyEgoMode(model, state, base);
		case "path":
			return applyPathMode(model, state, base);
		case "full":
			return base;
		case "process":
		case "clustered":
		case "matrix":
		case "subgraph":
		default:
			return base;
	}
}

/** @emoji 🎯 Suggests a visualization mode from graph size and selection. */
export function suggestMode(model: GraphModel, state: GraphViewState): { mode: VisualizationMode; reason: string } {
	if (state.selectedNodeId && state.secondNodeId) return { mode: "path", reason: "Two nodes selected" };
	if (state.selectedNodeId) return { mode: "ego", reason: "Node selected" };
	if (model.counts.nodes > NODE_READABLE_MAX) return { mode: "schema", reason: "Large graph" };
	if (model.counts.edges > EDGE_READABLE_MAX) return { mode: "matrix", reason: "Dense graph" };
	return { mode: "full", reason: "Readable overview" };
}

function resolveEffectiveMode(model: GraphModel, state: GraphViewState): VisualizationMode {
	if (state.mode !== "auto") return state.mode;
	return suggestMode(model, state).mode;
}

/** @emoji 📐 Layouts available for the active visualization mode. */
export function availableLayoutsForMode(mode: VisualizationMode, _model: GraphModel): ReadonlyArray<NamedGraphLayout> {
	if (mode === "matrix") return graphLayoutRegistry.filter((entry) => entry.id === "grid");
	if (mode === "schema") return graphLayoutRegistry.filter((entry) => ["circular", "grid", "force-balanced"].includes(entry.id));
	return graphLayoutRegistry;
}
// #endregion 🕸NetworkGraphModes

// #region 🕸NetworkGraphPipeline
export interface RenderNode {
	readonly id: string;
	readonly label: string;
	readonly type: string;
	readonly color: string;
	readonly size: number;
	readonly pinned: boolean;
	readonly dimmed: boolean;
	readonly isGroup: boolean;
	readonly memberIds?: ReadonlyArray<string>;
	readonly schema?: boolean;
}

export interface RenderEdge {
	readonly id: string;
	readonly source: string;
	readonly target: string;
	readonly type: string;
	readonly color: string;
	readonly width: number;
	readonly directed: boolean;
	readonly dashed: boolean;
	readonly dimmed: boolean;
	readonly count?: number;
}

export interface RenderLegendEntry {
	readonly id: string;
	readonly label: string;
	readonly color: string;
}

export interface RenderGraph {
	readonly nodes: ReadonlyArray<RenderNode>;
	readonly edges: ReadonlyArray<RenderEdge>;
	readonly layoutNodes: ReadonlyArray<NetworkNode>;
	readonly layoutEdges: ReadonlyArray<NetworkEdge>;
	readonly effectiveMode: VisualizationMode;
	readonly layoutId: string;
	readonly legend: {
		readonly nodeTypes: ReadonlyArray<RenderLegendEntry>;
		readonly edgeTypes: ReadonlyArray<RenderLegendEntry>;
		readonly sizeMetric: NodeSizeMetric;
		readonly widthMetric: EdgeWidthMetric;
	};
	readonly warnings: ReadonlyArray<string>;
	readonly suggestions: ReadonlyArray<{ action: string; reason: string }>;
	readonly showLabels: boolean;
	readonly matrix?: boolean;
}

function nodeSizeForMetric(model: GraphModel, node: NetworkNode, metric: NodeSizeMetric, schema?: boolean): number {
	if (schema) return 12;
	if (metric === "uniform") return 6;
	const degree = model.degree.get(node.id);
	if (!degree) return 5;
	if (metric === "in") return Math.min(14, 4 + degree.in);
	if (metric === "out") return Math.min(14, 4 + degree.out);
	return Math.min(14, 4 + Math.sqrt(degree.total));
}

function edgeWidthForMetric(metric: EdgeWidthMetric, count = 1): number {
	if (metric === "count") return Math.min(4, 1 + Math.log2(count + 1));
	return 1.5;
}

/** @emoji 🔀 Builds a render-ready view graph from model and view state. */
export function buildViewGraph(
	model: GraphModel,
	state: GraphViewState,
	viewport: { width: number; height: number },
): RenderGraph {
	const effectiveMode = resolveEffectiveMode(model, state);
	const modeGraph = applyModeGraph(model, state, effectiveMode);
	const warnings = [...model.warnings];
	const suggestions: Array<{ action: string; reason: string }> = [];
	let showLabels = state.showLabels;
	if (modeGraph.nodes.length > LABEL_MAX) {
		showLabels = false;
		warnings.push(`Labels hidden (${modeGraph.nodes.length} nodes)`);
		suggestions.push({ action: "Filter types", reason: "Too many nodes for labels" });
	}
	if (modeGraph.nodes.length > NODE_RENDER_MAX) {
		warnings.push(`Rendering capped; switch to schema view`);
		suggestions.push({ action: "Schema view", reason: "Graph too large" });
	}
	const selectedNeighbors = state.selectedNodeId
		? new Set([state.selectedNodeId, ...(model.neighbors.get(state.selectedNodeId) ?? [])])
		: null;
	const renderNodes: RenderNode[] = modeGraph.nodes.map((node) => {
		const dimmed = selectedNeighbors ? !selectedNeighbors.has(node.id) : false;
		return {
			id: node.id,
			label: node.label,
			type: node.type,
			color: model.colors.get(node.type) ?? NODE_TYPE_COLOR_TOKENS[0],
			size: nodeSizeForMetric(model, node, state.nodeSizeMetric, modeGraph.schema),
			pinned: state.pinnedNodeIds.has(node.id),
			dimmed,
			isGroup: node.id.startsWith("schema:"),
			schema: modeGraph.schema,
		};
	});
	const edgeTypeColors = new Map(model.edgeTypes.map((edgeType, index) => [edgeType.id, NODE_TYPE_COLOR_TOKENS[index % NODE_TYPE_COLOR_TOKENS.length]]));
	const renderEdges: RenderEdge[] = state.showEdges
		? modeGraph.edges.map((edge) => {
				const highlighted =
					!selectedNeighbors || (selectedNeighbors.has(edge.source) && selectedNeighbors.has(edge.target));
				const parallelKey = `${edge.type}:${edge.source}:${edge.target}`;
				return {
					id: edge.id,
					source: edge.source,
					target: edge.target,
					type: edge.type,
					color: edgeTypeColors.get(edge.type) ?? "var(--muted-foreground, #7b827d)",
					width: edgeWidthForMetric(state.edgeWidthMetric, modeGraph.schema ? 2 : 1),
					directed: true,
					dashed: Boolean(modeGraph.schema),
					dimmed: !highlighted,
					count: modeGraph.schema ? 2 : undefined,
				};
			})
		: [];
	const activeNodeTypeLegend = model.nodeTypes
		.filter((nodeType) => state.activeNodeTypes.has(nodeType.id))
		.map((nodeType) => ({ id: nodeType.id, label: nodeType.label, color: model.colors.get(nodeType.id) ?? NODE_TYPE_COLOR_TOKENS[0] }));
	const activeEdgeTypeLegend = model.edgeTypes
		.filter((edgeType) => state.activeEdgeTypes.has(edgeType.id))
		.map((edgeType) => ({ id: edgeType.id, label: edgeType.label, color: edgeTypeColors.get(edgeType.id) ?? "var(--muted-foreground, #7b827d)" }));
	return {
		nodes: renderNodes,
		edges: renderEdges,
		layoutNodes: modeGraph.nodes,
		layoutEdges: modeGraph.edges,
		effectiveMode,
		layoutId: state.layoutId,
		legend: {
			nodeTypes: activeNodeTypeLegend,
			edgeTypes: activeEdgeTypeLegend,
			sizeMetric: state.nodeSizeMetric,
			widthMetric: state.edgeWidthMetric,
		},
		warnings,
		suggestions,
		showLabels,
		matrix: effectiveMode === "matrix" && modeGraph.nodes.length <= 40,
	};
}
// #endregion 🕸NetworkGraphPipeline

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

interface SimulationNode {
	id: string;
	x: number;
	y: number;
	vx?: number;
	vy?: number;
	fx?: number | null;
	fy?: number | null;
}

interface SimulationLink {
	source: string;
	target: string;
}

interface GraphPositionsController {
	readonly positions: ReadonlyMap<string, { readonly x: number; readonly y: number }>;
	readonly version: number;
	readonly live: boolean;
	beginNodeDrag(nodeId: string, x: number, y: number): void;
	moveNodeDrag(nodeId: string, x: number, y: number): void;
	endNodeDrag(nodeId: string, keepFixed: boolean): void;
}

/** @emoji 🧲 Drives node positions through a live d3-force simulation (animated, draggable) or a static layout. */
function useGraphPositions(params: {
	nodes: ReadonlyArray<NetworkNode>;
	edges: ReadonlyArray<NetworkEdge>;
	width: number;
	height: number;
	staticLayout?: GraphLayout;
	simulationConfig?: ForceGraphLayoutConfig;
	layoutOptions?: GraphLayoutOptions;
	pinnedNodeIds: ReadonlySet<string>;
	onReady: (positions: ReadonlyMap<string, { readonly x: number; readonly y: number }>) => void;
}): GraphPositionsController {
	const { nodes, edges, width, height, staticLayout, simulationConfig, layoutOptions, pinnedNodeIds, onReady } = params;
	const positionsRef = useRef<Map<string, { x: number; y: number }>>(new Map());
	const simRef = useRef<Simulation<SimulationNode, undefined> | null>(null);
	const simNodesRef = useRef<Map<string, SimulationNode>>(new Map());
	const onReadyRef = useRef(onReady);
	onReadyRef.current = onReady;
	const [version, setVersion] = useState(0);
	const live = !staticLayout && simulationConfig != null;

	useEffect(() => {
		simRef.current?.stop();
		simRef.current = null;
		simNodesRef.current = new Map();
		if (nodes.length === 0) {
			positionsRef.current = new Map();
			setVersion((value) => value + 1);
			return;
		}
		if (!live || !simulationConfig) {
			const layoutFn = staticLayout ?? circularGraphLayout;
			const computed = layoutFn(nodes, edges, { width, height, ...layoutOptions });
			positionsRef.current = new Map([...computed].map(([id, point]) => [id, { x: point.x, y: point.y }]));
			setVersion((value) => value + 1);
			onReadyRef.current(positionsRef.current);
			return;
		}
		const previous = positionsRef.current;
		const simNodes: SimulationNode[] = nodes.map((node, index) => {
			const prior = previous.get(node.id);
			const x = prior?.x ?? (index % 12) * 40 - width / 2;
			const y = prior?.y ?? Math.floor(index / 12) * 40 - height / 2;
			const fixed = pinnedNodeIds.has(node.id);
			return { id: node.id, x, y, fx: fixed ? x : undefined, fy: fixed ? y : undefined };
		});
		const byId = new Map(simNodes.map((node) => [node.id, node]));
		simNodesRef.current = byId;
		const links: SimulationLink[] = edges
			.filter((edge) => byId.has(edge.source) && byId.has(edge.target))
			.map((edge) => ({ source: edge.source, target: edge.target }));
		const writePositions = () => {
			const next = new Map<string, { x: number; y: number }>();
			for (const node of simNodes) next.set(node.id, { x: node.x, y: node.y });
			positionsRef.current = next;
			setVersion((value) => value + 1);
		};
		writePositions();
		onReadyRef.current(positionsRef.current);
		const simulation = forceSimulation<SimulationNode>(simNodes)
			.force("charge", forceManyBody<SimulationNode>().strength(simulationConfig.chargeStrength))
			.force(
				"link",
				forceLink<SimulationNode, SimulationLink>(links)
					.id((node) => node.id)
					.distance(simulationConfig.linkDistance),
			)
			.force("collide", forceCollide<SimulationNode>(simulationConfig.collideRadius))
			.force("x", forceX<SimulationNode>(0).strength(simulationConfig.centerStrength))
			.force("y", forceY<SimulationNode>(0).strength(simulationConfig.centerStrength));
		simulation.on("tick", writePositions);
		simulation.on("end", () => {
			writePositions();
			onReadyRef.current(positionsRef.current);
		});
		simRef.current = simulation;
		return () => {
			simulation.on("tick", null);
			simulation.on("end", null);
			simulation.stop();
		};
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [nodes, edges, width, height, staticLayout, simulationConfig, layoutOptions, pinnedNodeIds, live]);

	const beginNodeDrag = useCallback((nodeId: string, x: number, y: number) => {
		const simulation = simRef.current;
		const node = simNodesRef.current.get(nodeId);
		if (!simulation || !node) return;
		node.fx = x;
		node.fy = y;
		simulation.alphaTarget(0.3).restart();
	}, []);

	const moveNodeDrag = useCallback((nodeId: string, x: number, y: number) => {
		const node = simNodesRef.current.get(nodeId);
		if (!node) return;
		node.fx = x;
		node.fy = y;
	}, []);

	const endNodeDrag = useCallback((nodeId: string, keepFixed: boolean) => {
		const simulation = simRef.current;
		const node = simNodesRef.current.get(nodeId);
		if (!simulation || !node) return;
		if (!keepFixed) {
			node.fx = undefined;
			node.fy = undefined;
		}
		simulation.alphaTarget(0);
	}, []);

	return { positions: positionsRef.current, version, live, beginNodeDrag, moveNodeDrag, endNodeDrag };
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
	layout,
	layouts = graphLayoutRegistry,
	initialLayoutId,
	layoutOptions,
	height = "100%",
	className,
	style,
	...props
}: NetworkGraphWidgetProps) {
	const shellRef = useRef<HTMLElement>(null);
	const canvasAreaRef = useRef<HTMLDivElement>(null);
	const [shellSize, setShellSize] = useState({ width: 800, height: 520 });
	const model = useMemo(() => normalizeGraph(data), [data]);
	const allEdgeTypeIds = useMemo(() => model.edgeTypes.map((edgeType) => edgeType.id), [model.edgeTypes]);
	const lenses = lensesProp ?? data.lenses ?? [];
	const [viewState, dispatch] = useReducer(
		graphViewReducer,
		undefined,
		() =>
			defaultViewState(model, {
				initialActiveNodeTypes,
				initialActiveEdgeTypes,
				initialLensName,
				initialSelectedNodeId,
				initialLayoutId,
			}),
	);
	const renderState = useMemo<GraphViewState>(
		() => ({
			...viewState,
			transform: { x: 0, y: 0, k: 1 },
			settingsOpen: false,
			hoveredNodeId: undefined,
		}),
		[
			viewState.mode,
			viewState.layoutId,
			viewState.activeNodeTypes,
			viewState.activeEdgeTypes,
			viewState.activeLensName,
			viewState.selectedNodeId,
			viewState.selectedEdgeId,
			viewState.secondNodeId,
			viewState.propertyFilters,
			viewState.searchQuery,
			viewState.depth,
			viewState.direction,
			viewState.groupingMode,
			viewState.aggregationMode,
			viewState.showLabels,
			viewState.showEdges,
			viewState.nodeSizeMetric,
			viewState.edgeWidthMetric,
			viewState.pinnedNodeIds,
			viewState.collapsedGroupIds,
			viewState.highlightedPath,
		],
	);
	const viewGraph = useMemo(() => buildViewGraph(model, renderState, shellSize), [model, renderState, shellSize]);
	const modeLayouts = useMemo(
		() => availableLayoutsForMode(viewGraph.effectiveMode, model),
		[model, viewGraph.effectiveMode],
	);
	const activeNamed = useMemo(
		() => modeLayouts.find((entry) => entry.id === viewState.layoutId) ?? modeLayouts[0] ?? graphLayoutRegistry[0]!,
		[modeLayouts, viewState.layoutId],
	);
	const staticLayoutFn = layout ?? (activeNamed.simulation ? undefined : activeNamed.layout);
	const simulationConfig = layout ? undefined : activeNamed.simulation;
	const panRef = useRef<{ active: boolean; x: number; y: number; originX: number; originY: number }>({
		active: false,
		x: 0,
		y: 0,
		originX: 0,
		originY: 0,
	});
	const svgRef = useRef<SVGSVGElement>(null);
	const draggingNodeRef = useRef<string | null>(null);
	const dragMovedRef = useRef(false);

	useEffect(() => {
		const element = canvasAreaRef.current;
		if (!element) return;
		const observer = new ResizeObserver((entries) => {
			const entry = entries[0];
			if (!entry) return;
			setShellSize({ width: entry.contentRect.width, height: entry.contentRect.height });
		});
		observer.observe(element);
		return () => observer.disconnect();
	}, []);

	const handleLayoutReady = useCallback(
		(layoutPositions: ReadonlyMap<string, { readonly x: number; readonly y: number }>) => {
			if (layoutPositions.size === 0) return;
			const fit = fitTransform(
				layoutPositions,
				viewGraph.nodes.map((node) => node.id),
				shellSize.width,
				shellSize.height,
			);
			dispatch({ type: "setTransform", transform: fit });
		},
		[viewGraph.nodes, shellSize.width, shellSize.height],
	);

	const positionsController = useGraphPositions({
		nodes: viewGraph.layoutNodes,
		edges: viewGraph.layoutEdges,
		width: shellSize.width,
		height: shellSize.height,
		staticLayout: staticLayoutFn,
		simulationConfig,
		layoutOptions,
		pinnedNodeIds: viewState.pinnedNodeIds,
		onReady: handleLayoutReady,
	});
	const positions = positionsController.positions;

	const selectedType = useMemo(
		() => (viewState.selectedNodeId ? model.nodeById.get(viewState.selectedNodeId)?.type : undefined),
		[model, viewState.selectedNodeId],
	);

	const stats = useMemo(
		() =>
			computeGraphStats(data, {
				activeNodeTypes: viewState.activeNodeTypes,
				activeEdgeTypes: viewState.activeEdgeTypes,
				statDefinitions: statDefinitionsProp ?? data.statDefinitions,
				selectedType,
			}),
		[data, viewState.activeNodeTypes, viewState.activeEdgeTypes, statDefinitionsProp, selectedType],
	);

	const resetCamera = useCallback(() => {
		const fit = fitTransform(
			positions,
			viewGraph.nodes.map((node) => node.id),
			shellSize.width,
			shellSize.height,
		);
		dispatch({ type: "setTransform", transform: fit });
	}, [positions, shellSize.height, shellSize.width, viewGraph.nodes]);

	const onWheel = useCallback((event: ReactWheelEvent<SVGSVGElement>) => {
		event.preventDefault();
		const rect = event.currentTarget.getBoundingClientRect();
		const px = event.clientX - rect.left;
		const py = event.clientY - rect.top;
		const factor = event.deltaY < 0 ? 1.12 : 0.88;
		const previous = viewState.transform;
		const nextK = Math.min(4, Math.max(0.15, previous.k * factor));
		const graphX = (px - previous.x) / previous.k;
		const graphY = (py - previous.y) / previous.k;
		dispatch({ type: "setTransform", transform: { k: nextK, x: px - graphX * nextK, y: py - graphY * nextK } });
	}, [viewState.transform]);

	const onPointerDown = useCallback(
		(event: ReactPointerEvent<SVGSVGElement>) => {
			if (event.button !== 0) return;
			panRef.current = {
				active: true,
				x: event.clientX,
				y: event.clientY,
				originX: viewState.transform.x,
				originY: viewState.transform.y,
			};
			event.currentTarget.setPointerCapture(event.pointerId);
		},
		[viewState.transform.x, viewState.transform.y],
	);

	const transformRef = useRef(viewState.transform);
	transformRef.current = viewState.transform;
	const clientToGraph = useCallback((clientX: number, clientY: number) => {
		const svg = svgRef.current;
		if (!svg) return null;
		const rect = svg.getBoundingClientRect();
		const current = transformRef.current;
		return { x: (clientX - rect.left - current.x) / current.k, y: (clientY - rect.top - current.y) / current.k };
	}, []);

	const onNodePointerDown = useCallback(
		(event: ReactPointerEvent<SVGGElement>, nodeId: string) => {
			if (event.button !== 0 || !positionsController.live) return;
			event.stopPropagation();
			draggingNodeRef.current = nodeId;
			dragMovedRef.current = false;
		},
		[positionsController.live],
	);

	const onPointerMove = useCallback(
		(event: ReactPointerEvent<SVGSVGElement>) => {
			const draggingNode = draggingNodeRef.current;
			if (draggingNode) {
				const graph = clientToGraph(event.clientX, event.clientY);
				if (!graph) return;
				if (!dragMovedRef.current) {
					dragMovedRef.current = true;
					event.currentTarget.setPointerCapture(event.pointerId);
					positionsController.beginNodeDrag(draggingNode, graph.x, graph.y);
				} else {
					positionsController.moveNodeDrag(draggingNode, graph.x, graph.y);
				}
				return;
			}
			if (!panRef.current.active) return;
			const dx = event.clientX - panRef.current.x;
			const dy = event.clientY - panRef.current.y;
			dispatch({
				type: "setTransform",
				transform: { ...transformRef.current, x: panRef.current.originX + dx, y: panRef.current.originY + dy },
			});
		},
		[positionsController, clientToGraph],
	);

	const onPointerUp = useCallback(
		(event: ReactPointerEvent<SVGSVGElement>) => {
			const draggingNode = draggingNodeRef.current;
			if (draggingNode) {
				if (dragMovedRef.current) positionsController.endNodeDrag(draggingNode, viewState.pinnedNodeIds.has(draggingNode));
				draggingNodeRef.current = null;
			}
			panRef.current.active = false;
			if (event.currentTarget.hasPointerCapture(event.pointerId)) event.currentTarget.releasePointerCapture(event.pointerId);
		},
		[positionsController, viewState.pinnedNodeIds],
	);

	const hoveredNode = viewState.hoveredNodeId ? model.nodeById.get(viewState.hoveredNodeId) : undefined;
	const selectedNode = viewState.selectedNodeId ? model.nodeById.get(viewState.selectedNodeId) : undefined;
	const selectedEdge = viewState.selectedEdgeId ? model.edgeById.get(viewState.selectedEdgeId) : undefined;
	const modeSuggestion = useMemo(() => suggestMode(model, viewState), [model, viewState]);
	const labelMinZoom = 1.8;
	const transform = viewState.transform;

	return (
		<section
			{...props}
			ref={shellRef}
			className={classNames("network-graph-widget", className)}
			style={{ ...networkGraphShellStyle, height, ...style }}
		>
			<div
				style={{
					flex: "0 0 auto",
					display: "flex",
					flexWrap: "wrap",
					alignItems: "center",
					justifyContent: "center",
					gap: "0.35rem",
					padding: "0.5rem 2.5rem",
					background: "transparent",
					zIndex: 11,
				}}
			>
				{data.edgeTypes.map((edgeType) => {
					const active = viewState.activeEdgeTypes.has(edgeType.id);
					return (
						<button
							key={edgeType.id}
							type="button"
							style={{
								...networkGraphChipStyle,
								fontSize: "0.65rem",
								background: active
									? "color-mix(in srgb, var(--window, #ebe8d9) 75%, transparent)"
									: "color-mix(in srgb, var(--panel, #c9c8bd) 30%, transparent)",
							}}
							onClick={() => dispatch({ type: "toggleEdgeType", edgeTypeId: edgeType.id })}
						>
							{edgeType.label}
						</button>
					);
				})}
			</div>

			<div ref={canvasAreaRef} style={{ position: "relative", flex: "1 1 auto", minHeight: 0, overflow: "hidden" }}>
				<svg
					ref={svgRef}
					role="img"
					aria-label="Network graph canvas"
					style={{
						position: "absolute",
						inset: 0,
						width: "100%",
						height: "100%",
						cursor: panRef.current.active ? "grabbing" : "grab",
						touchAction: "none",
					}}
					onWheel={onWheel}
					onPointerDown={onPointerDown}
					onPointerMove={onPointerMove}
					onPointerUp={onPointerUp}
					onPointerLeave={onPointerUp}
				>
					<rect width="100%" height="100%" fill="transparent" />
					<defs>
						<marker id="network-graph-arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
							<path d="M 0 0 L 10 5 L 0 10 z" fill="var(--muted-foreground, #7b827d)" />
						</marker>
					</defs>
					<g transform={`translate(${transform.x} ${transform.y}) scale(${transform.k})`}>
						{viewGraph.matrix ? (
							viewGraph.nodes.map((node, rowIndex) =>
								viewGraph.nodes.map((columnNode, columnIndex) => {
									const cellSize = 18;
									const hasEdge = viewGraph.edges.some(
										(edge) => edge.source === node.id && edge.target === columnNode.id,
									);
									if (!hasEdge) return null;
									return (
										<rect
											key={`${node.id}:${columnNode.id}`}
											x={columnIndex * cellSize - (viewGraph.nodes.length * cellSize) / 2}
											y={rowIndex * cellSize - (viewGraph.nodes.length * cellSize) / 2}
											width={cellSize - 2}
											height={cellSize - 2}
											fill="color-mix(in srgb, var(--accent-secondary, #34d1bf) 55%, transparent)"
										/>
									);
								}),
							)
						) : (
							<>
								{viewGraph.edges.map((edge) => {
									const source = positions.get(edge.source);
									const target = positions.get(edge.target);
									if (!source || !target) return null;
									return (
										<line
											key={edge.id}
											x1={source.x}
											y1={source.y}
											x2={target.x}
											y2={target.y}
											stroke={edge.dimmed ? "color-mix(in srgb, var(--muted-foreground, #7b827d) 35%, transparent)" : edge.color}
											strokeWidth={edge.width}
											strokeDasharray={edge.dashed ? "4 4" : undefined}
											markerEnd={edge.directed ? "url(#network-graph-arrow)" : undefined}
										/>
									);
								})}
								{viewGraph.nodes.map((node) => {
									const position = positions.get(node.id);
									if (!position) return null;
									const selected = viewState.selectedNodeId === node.id;
									return (
										<g
											key={node.id}
											style={{ cursor: positionsController.live ? "grab" : "pointer" }}
											onPointerEnter={() => dispatch({ type: "setHoveredNode", nodeId: node.id })}
											onPointerLeave={() => dispatch({ type: "setHoveredNode", nodeId: undefined })}
											onPointerDown={(event) => onNodePointerDown(event, node.id)}
											onClick={(event) => {
												event.stopPropagation();
												if (dragMovedRef.current) {
													dragMovedRef.current = false;
													return;
												}
												dispatch({
													type: "selectNode",
													nodeId: viewState.selectedNodeId === node.id ? undefined : node.id,
												});
											}}
										>
											<circle
												cx={position.x}
												cy={position.y}
												r={node.size}
												fill={node.color}
												fillOpacity={node.dimmed ? 0.25 : 0.9}
												stroke={selected ? "var(--active-base, #ff344f)" : node.color}
												strokeWidth={selected ? 2.5 : 1}
											/>
											{viewGraph.showLabels && viewState.showLabels && transform.k >= labelMinZoom ? (
												<text
													x={position.x}
													y={position.y}
													textAnchor="middle"
													dominantBaseline="central"
													fill="var(--foreground, #001117)"
													fontSize={Math.max(node.size * 0.7, 6 / transform.k)}
													fontFamily="var(--font-sans, sans-serif)"
													style={{ pointerEvents: "none" }}
													opacity={node.dimmed ? 0.35 : 1}
												>
													{node.label.length > 12 ? `${node.label.slice(0, 11)}…` : node.label}
												</text>
											) : null}
										</g>
									);
								})}
							</>
						)}
					</g>
				</svg>

				<aside
					style={{
						...networkGraphPanelStyle,
						position: "absolute",
						top: "0.75rem",
						bottom: "0.75rem",
						left: "0.75rem",
						width: "min(15rem, calc(100% - 1.5rem))",
						maxWidth: "15rem",
						maxHeight: "none",
						alignContent: "start",
					}}
				>
					<p style={eyebrowStyle}>Graph Stats</p>
					<p style={{ ...metricHintStyle, margin: 0, fontSize: "0.65rem" }}>
						Mode: {viewGraph.effectiveMode}
						{viewState.mode === "auto" ? ` (${modeSuggestion.reason})` : ""}
					</p>
					{viewGraph.warnings.map((warning) => (
						<p key={warning} style={{ ...metricHintStyle, margin: 0, fontSize: "0.65rem", color: "var(--warning-border, #fccf05)" }}>
							{warning}
						</p>
					))}
					{selectedNode ? (
						<div style={{ display: "grid", gap: "0.25rem", paddingBottom: "0.35rem", borderBottom: "1px solid color-mix(in srgb, var(--border-window-color, #7b827d) 45%, transparent)" }}>
							<p style={{ ...titleStyle, fontSize: "0.8rem", margin: 0 }}>{selectedNode.label}</p>
							<p style={{ ...metricHintStyle, margin: 0, fontSize: "0.65rem" }}>
								{selectedNode.type} · in {model.degree.get(selectedNode.id)?.in ?? 0} · out {model.degree.get(selectedNode.id)?.out ?? 0}
							</p>
							<div style={{ display: "flex", flexWrap: "wrap", gap: "0.25rem" }}>
								<button type="button" style={networkGraphChipStyle} onClick={() => dispatch({ type: "isolateSelection", model })}>
									Isolate
								</button>
								<button type="button" style={networkGraphChipStyle} onClick={() => dispatch({ type: "setMode", mode: "ego" })}>
									Ego
								</button>
								<button type="button" style={networkGraphChipStyle} onClick={() => dispatch({ type: "togglePin", nodeId: selectedNode.id })}>
									{viewState.pinnedNodeIds.has(selectedNode.id) ? "Unpin" : "Pin"}
								</button>
							</div>
						</div>
					) : null}
					{selectedEdge ? (
						<div style={{ display: "grid", gap: "0.25rem", paddingBottom: "0.35rem", borderBottom: "1px solid color-mix(in srgb, var(--border-window-color, #7b827d) 45%, transparent)" }}>
							<p style={{ ...titleStyle, fontSize: "0.8rem", margin: 0 }}>{selectedEdge.type}</p>
							<p style={{ ...metricHintStyle, margin: 0, fontSize: "0.65rem" }}>
								{model.nodeById.get(selectedEdge.source)?.label ?? selectedEdge.source} → {model.nodeById.get(selectedEdge.target)?.label ?? selectedEdge.target}
							</p>
						</div>
					) : null}
					<div style={{ display: "grid" }}>
						{stats.map((row, index) => (
							<div
								key={row.id}
								style={{
									display: "grid",
									gap: "0.1rem",
									padding: "0.4rem 0",
									borderTop:
										index === 0
											? "none"
											: "1px solid color-mix(in srgb, var(--border-window-color, #7b827d) 45%, transparent)",
								}}
							>
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
						bottom: "0.75rem",
						right: "0.75rem",
						width: "min(15rem, calc(100% - 1.5rem))",
						maxWidth: "15rem",
						maxHeight: "none",
						alignContent: "start",
					}}
				>
					<p style={eyebrowStyle}>Network Lenses</p>
					<p style={{ ...titleStyle, fontSize: "0.95rem", margin: 0 }}>Active Lens: {viewState.activeLensName}</p>
					<div style={{ display: "grid", gap: "0.5rem" }}>
						{lenses.map((lens) => (
							<article
								key={lens.id}
								style={{
									...widgetCardStyle,
									padding: "0.5rem",
									borderColor: "var(--border-window-color, #7b827d)",
									background: "color-mix(in srgb, var(--window, #ebe8d9) 45%, transparent)",
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
									onClick={() => dispatch({ type: "applyLens", lens, allEdgeTypeIds })}
								>
									Apply Lens
								</button>
							</article>
						))}
					</div>
				</aside>

				{hoveredNode ? (
					<div
						style={{
							...glassPanelStyle,
							position: "absolute",
							bottom: "0.75rem",
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
							· deg {model.degree.get(hoveredNode.id)?.total ?? 0}
						</span>
					</div>
				) : null}
			</div>

			<div
				style={{
					flex: "0 0 auto",
					display: "flex",
					flexWrap: "wrap",
					alignItems: "center",
					justifyContent: "center",
					gap: "0.35rem",
					padding: "0.5rem 0.6rem",
					background: "transparent",
					zIndex: 11,
				}}
			>
				{data.nodeTypes.map((nodeType) => {
					const active = viewState.activeNodeTypes.has(nodeType.id);
					const color = model.colors.get(nodeType.id) ?? NODE_TYPE_COLOR_TOKENS[0];
					return (
						<button
							key={nodeType.id}
							type="button"
							style={{
								...networkGraphChipStyle,
								borderColor: color,
								background: active
									? `color-mix(in srgb, ${color} 22%, transparent)`
									: "color-mix(in srgb, var(--panel, #c9c8bd) 30%, transparent)",
							}}
							onClick={() => dispatch({ type: "toggleNodeType", nodeTypeId: nodeType.id })}
						>
							<span style={{ width: 8, height: 8, background: color, display: "inline-block" }} />
							{nodeType.label}
							<span style={{ color: "var(--muted-foreground, #7b827d)" }}>{nodeType.count ?? ""}</span>
						</button>
					);
				})}
			</div>

			<div style={{ position: "absolute", top: "0.5rem", right: "0.6rem", zIndex: 13, display: "grid", gap: "0.35rem", justifyItems: "end" }}>
				<button
					type="button"
					aria-label="Settings"
					aria-expanded={viewState.settingsOpen}
					style={{
						...networkGraphChipStyle,
						padding: "0.25rem 0.45rem",
						background: viewState.settingsOpen
							? "color-mix(in srgb, var(--window, #ebe8d9) 75%, transparent)"
							: "color-mix(in srgb, var(--panel, #c9c8bd) 30%, transparent)",
					}}
					onClick={() => dispatch({ type: "setSettingsOpen", settingsOpen: !viewState.settingsOpen })}
				>
					⚙ Settings
				</button>
				{viewState.settingsOpen ? (
					<div style={{ ...networkGraphPanelStyle, padding: "0.6rem", width: "13rem", gap: "0.45rem", maxHeight: "calc(100vh - 4rem)", overflow: "auto" }}>
						<p style={eyebrowStyle}>Visualization</p>
						<select
							value={viewState.mode}
							onChange={(event) => dispatch({ type: "setMode", mode: event.target.value as VisualizationMode })}
							style={{ ...networkGraphChipStyle, width: "100%", cursor: "pointer" }}
						>
							{["auto", "schema", "full", "subgraph", "ego", "path", "process", "clustered", "matrix"].map((mode) => (
								<option key={mode} value={mode}>
									{mode}
								</option>
							))}
						</select>
						{!layout ? (
							<select
								value={viewState.layoutId}
								onChange={(event) => dispatch({ type: "setLayoutId", layoutId: event.target.value })}
								style={{ ...networkGraphChipStyle, width: "100%", cursor: "pointer" }}
							>
								{modeLayouts.map((entry) => (
									<option key={entry.id} value={entry.id}>
										{entry.name}
									</option>
								))}
							</select>
						) : null}
						<p style={eyebrowStyle}>Neighborhood</p>
						<input
							type="range"
							min={1}
							max={4}
							value={viewState.depth}
							onChange={(event) => dispatch({ type: "setDepth", depth: Number(event.target.value) })}
							style={{ width: "100%" }}
						/>
						<select
							value={viewState.direction}
							onChange={(event) => dispatch({ type: "setDirection", direction: event.target.value as NeighborhoodDirection })}
							style={{ ...networkGraphChipStyle, width: "100%", cursor: "pointer" }}
						>
							<option value="both">Both</option>
							<option value="in">Incoming</option>
							<option value="out">Outgoing</option>
						</select>
						<p style={eyebrowStyle}>Search</p>
						<input
							type="search"
							value={viewState.searchQuery}
							onChange={(event) => dispatch({ type: "setSearchQuery", searchQuery: event.target.value })}
							style={{ ...networkGraphChipStyle, width: "100%" }}
							placeholder="Node label or property"
						/>
						<p style={eyebrowStyle}>Encoding</p>
						<select
							value={viewState.nodeSizeMetric}
							onChange={(event) => dispatch({ type: "setNodeSizeMetric", nodeSizeMetric: event.target.value as NodeSizeMetric })}
							style={{ ...networkGraphChipStyle, width: "100%", cursor: "pointer" }}
						>
							<option value="uniform">Uniform size</option>
							<option value="degree">Degree</option>
							<option value="in">In-degree</option>
							<option value="out">Out-degree</option>
						</select>
						<p style={eyebrowStyle}>View</p>
						<button type="button" style={networkGraphChipStyle} onClick={() => dispatch({ type: "setShowLabels", showLabels: !viewState.showLabels })}>
							{viewState.showLabels ? "Hide labels" : "Show labels"}
						</button>
						<button type="button" style={networkGraphChipStyle} onClick={() => dispatch({ type: "setShowEdges", showEdges: !viewState.showEdges })}>
							{viewState.showEdges ? "Hide edges" : "Show edges"}
						</button>
						<button type="button" style={networkGraphChipStyle} onClick={resetCamera}>
							Reset camera
						</button>
						<button type="button" style={networkGraphChipStyle} onClick={() => dispatch({ type: "resetView", model })}>
							Reset all
						</button>
					</div>
				) : null}
			</div>
		</section>
	);
}
// #endregion 🕸NetworkGraphWidget
