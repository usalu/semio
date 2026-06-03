// #region 🧲Header
/** @emoji 🧩 `@widgets/react` — standalone React widgets styled against the semio UI token surface. */
// #endregion 🧲Header

// #region 🔌Adapters
import type { ComponentPropsWithoutRef, CSSProperties, ReactNode } from "react";
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
