#!/usr/bin/env bun
/** 📊 Ticket generator: taxonomy manifest + gallery tex. Run from repo root. */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

type Leaf = { readonly slug: string; readonly title: string; readonly kind: "mark" | "chart" | "layout" | "axis" | "scale"; readonly family: string };
type Group = { readonly title: string; readonly leaves: readonly Leaf[] };
type Section = { readonly id: string; readonly title: string; readonly groups: readonly Group[] };

const s = (slug: string, title: string, family: string, kind: Leaf["kind"] = "chart"): Leaf => ({ slug, title, kind, family });

const SECTIONS: readonly Section[] = [
  {
    id: "0",
    title: "Foundational visualizations / primitives",
    groups: [
      {
        title: "Points / marks",
        leaves: [
          s("dot", "Dot", "mark", "mark"),
          s("circle", "Circle", "mark", "mark"),
          s("square", "Square", "mark", "mark"),
          s("rectangle", "Rectangle", "mark", "mark"),
          s("triangle", "Triangle", "mark", "mark"),
          s("diamond", "Diamond", "mark", "mark"),
          s("cross", "Cross", "mark", "mark"),
          s("plus", "Plus", "mark", "mark"),
          s("star", "Star", "mark", "mark"),
          s("custom-glyph", "Custom glyph", "mark", "mark"),
          s("icon", "Icon mark", "mark", "mark"),
          s("image", "Image mark", "mark", "mark"),
          s("text", "Text mark", "mark", "mark"),
        ],
      },
      {
        title: "Lines",
        leaves: [
          s("straight-line", "Straight line", "path", "mark"),
          s("polyline", "Polyline", "path", "mark"),
          s("step-line", "Step line", "path", "mark"),
          s("curved-line", "Curved line", "path", "mark"),
          s("bezier", "Bezier curve", "path", "mark"),
          s("spline", "Spline", "path", "mark"),
          s("catmull-rom", "Catmull-Rom spline", "path", "mark"),
          s("basis-spline", "Basis spline", "path", "mark"),
          s("cardinal-spline", "Cardinal spline", "path", "mark"),
          s("monotone-spline", "Monotone spline", "path", "mark"),
          s("closed-curve", "Closed curve", "path", "mark"),
        ],
      },
      {
        title: "Areas",
        leaves: [
          s("polygon", "Polygon", "path", "mark"),
          s("filled-path", "Filled path", "path", "mark"),
          s("ribbon", "Ribbon", "path", "mark"),
          s("band", "Band", "path", "mark"),
          s("envelope", "Envelope", "path", "mark"),
        ],
      },
      {
        title: "Arcs",
        leaves: [
          s("circular-arc", "Circular arc", "path", "mark"),
          s("elliptical-arc", "Elliptical arc", "path", "mark"),
          s("annular-arc", "Annular arc", "path", "mark"),
          s("sector", "Sector", "path", "mark"),
          s("wedge", "Wedge", "path", "mark"),
        ],
      },
      {
        title: "Connections",
        leaves: [
          s("straight-connector", "Straight connector", "path", "mark"),
          s("orthogonal-connector", "Orthogonal connector", "path", "mark"),
          s("curved-connector", "Curved connector", "path", "mark"),
          s("elbow-connector", "Elbow connector", "path", "mark"),
          s("bundled-connector", "Bundled connector", "path", "mark"),
          s("arrow", "Arrow", "path", "mark"),
          s("bidirectional-arrow", "Bidirectional arrow", "path", "mark"),
        ],
      },
      {
        title: "Regions",
        leaves: [
          s("rectangular-region", "Rectangular region", "path", "mark"),
          s("circular-region", "Circular region", "path", "mark"),
          s("polygonal-region", "Polygonal region", "path", "mark"),
          s("voronoi-region", "Voronoi region", "voronoi", "mark"),
          s("convex-hull", "Convex hull", "path", "mark"),
          s("concave-hull", "Concave hull", "path", "mark"),
        ],
      },
      {
        title: "Annotation primitives",
        leaves: [
          s("label", "Label", "anno", "mark"),
          s("callout", "Callout", "anno", "mark"),
          s("leader-line", "Leader line", "anno", "mark"),
          s("bracket", "Bracket", "anno", "mark"),
          s("brace", "Brace", "anno", "mark"),
          s("highlight-region", "Highlight region", "anno", "mark"),
          s("reference-line", "Reference line", "anno", "mark"),
          s("reference-band", "Reference band", "anno", "mark"),
          s("reference-point", "Reference point", "anno", "mark"),
        ],
      },
    ],
  },
  {
    id: "1",
    title: "Categorical comparison charts",
    groups: [
      {
        title: "Bar charts",
        leaves: [
          s("vertical-bar", "Vertical bar chart", "bar"),
          s("horizontal-bar", "Horizontal bar chart", "bar"),
          s("grouped-bar", "Grouped bar chart", "bar"),
          s("clustered-bar", "Clustered bar chart", "bar"),
          s("stacked-bar", "Stacked bar chart", "bar"),
          s("percent-stacked-bar", "100% stacked bar chart", "bar"),
          s("diverging-stacked-bar", "Diverging stacked bar chart", "bar"),
          s("floating-bar", "Floating bar chart", "bar"),
          s("range-bar", "Range bar chart", "bar"),
          s("overlapping-bar", "Overlapping bar chart", "bar"),
          s("nested-bar", "Nested bar chart", "bar"),
          s("thin-bar", "Thin bar chart", "bar"),
          s("rounded-bar", "Rounded bar chart", "bar"),
          s("lollipop-bar", "Lollipop bar chart", "bar"),
          s("bullet-bar", "Bullet-style bar chart", "bullet"),
          s("paired-bar", "Paired bar chart", "bar"),
          s("mirrored-bar", "Mirrored bar chart", "bar"),
          s("butterfly", "Butterfly chart", "bar"),
          s("tornado", "Tornado chart", "bar"),
          s("population-pyramid", "Population pyramid", "bar"),
        ],
      },
      {
        title: "Column charts",
        leaves: [
          s("column", "Column chart", "bar"),
          s("grouped-column", "Grouped column chart", "bar"),
          s("stacked-column", "Stacked column chart", "bar"),
          s("percent-stacked-column", "100% stacked column chart", "bar"),
          s("floating-column", "Floating column chart", "bar"),
          s("range-column", "Range column chart", "bar"),
        ],
      },
      {
        title: "Dot-based comparison",
        leaves: [
          s("dot-plot", "Dot plot", "dot"),
          s("cleveland-dot", "Cleveland dot plot", "dot"),
          s("grouped-dot", "Grouped dot plot", "dot"),
          s("dumbbell", "Dumbbell chart", "dot"),
          s("arrow-dot", "Arrow plot", "dot"),
          s("lollipop", "Lollipop chart", "dot"),
          s("stem", "Stem chart", "dot"),
        ],
      },
    ],
  },
  {
    id: "2",
    title: "Ranking charts",
    groups: [
      {
        title: "Ordered comparison",
        leaves: [
          s("ranked-bar", "Ranked bar chart", "bar"),
          s("ranked-column", "Ranked column chart", "bar"),
          s("slope", "Slope chart", "line"),
          s("bump", "Bump chart", "line"),
          s("ranked-lollipop", "Ranked lollipop", "dot"),
          s("ordered-dot", "Ordered dot plot", "dot"),
          s("unit-chart", "Unit chart", "waffle"),
          s("pictogram", "Pictogram chart", "waffle"),
          s("isotype", "Isotype chart", "waffle"),
        ],
      },
    ],
  },
  {
    id: "3",
    title: "Distribution charts",
    groups: [
      {
        title: "Frequency",
        leaves: [
          s("histogram", "Histogram", "dist"),
          s("variable-width-histogram", "Variable-width histogram", "dist"),
          s("density", "Density plot", "dist"),
          s("kernel-density", "Kernel density estimate", "dist"),
          s("ridgeline", "Ridgeline plot", "dist"),
          s("joy", "Joy plot", "dist"),
          s("frequency-polygon", "Frequency polygon", "dist"),
          s("cumulative-freq", "Cumulative frequency", "dist"),
          s("ogive", "Ogive", "dist"),
        ],
      },
      {
        title: "Summary",
        leaves: [
          s("boxplot", "Box plot", "dist"),
          s("letter-value-box", "Letter-value box plot", "dist"),
          s("violin", "Violin plot", "dist"),
          s("bean", "Bean plot", "dist"),
          s("bee-swarm", "Beeswarm plot", "dist"),
          s("strip", "Strip plot", "dist"),
          s("jitter", "Jitter plot", "dist"),
          s("sina", "Sina plot", "dist"),
          s("raincloud", "Raincloud plot", "dist"),
          s("quantile-box", "Quantile box plot", "dist"),
        ],
      },
      {
        title: "Binned 2d",
        leaves: [
          s("hexbin", "Hexbin chart", "hexbin"),
          s("rectbin", "Rectbin chart", "heat"),
          s("contour", "Contour plot", "heat"),
          s("filled-contour", "Filled contour", "heat"),
        ],
      },
    ],
  },
  {
    id: "4",
    title: "Part-to-whole charts",
    groups: [
      {
        title: "Circular",
        leaves: [
          s("pie", "Pie chart", "pie"),
          s("donut", "Donut chart", "pie"),
          s("exploded-pie", "Exploded pie chart", "pie"),
          s("half-pie", "Half pie / gauge pie", "pie"),
          s("sunburst", "Sunburst", "pie"),
          s("multi-level-donut", "Multi-level donut", "pie"),
          s("rose", "Rose chart", "pie"),
          s("nightingale", "Nightingale rose", "pie"),
          s("polar-area", "Polar area chart", "pie"),
        ],
      },
      {
        title: "Rectangular",
        leaves: [
          s("treemap", "Treemap", "tree"),
          s("squarify-treemap", "Squarified treemap", "tree"),
          s("slice-dice-treemap", "Slice-and-dice treemap", "tree"),
          s("icicle", "Icicle chart", "tree"),
          s("mosaic", "Mosaic plot", "tree"),
          s("marimekko", "Marimekko / Mekko", "tree"),
          s("waffle", "Waffle chart", "waffle"),
          s("gridplot", "Grid plot", "waffle"),
          s("stacked-percent-area", "100% stacked area", "area"),
        ],
      },
      {
        title: "Progress",
        leaves: [
          s("funnel", "Funnel chart", "funnel"),
          s("pyramid-part", "Pyramid chart", "funnel"),
          s("gauge", "Gauge", "pie"),
          s("bullet", "Bullet chart", "bullet"),
          s("progress-bar", "Progress bar", "bar"),
          s("radial-progress", "Radial progress", "pie"),
        ],
      },
    ],
  },
  {
    id: "5",
    title: "Trend and time series charts",
    groups: [
      {
        title: "Lines",
        leaves: [
          s("line", "Line chart", "line"),
          s("multi-line", "Multi-line chart", "line"),
          s("step", "Step chart", "line"),
          s("spline-line", "Spline line chart", "line"),
          s("area", "Area chart", "area"),
          s("stacked-area", "Stacked area chart", "area"),
          s("streamgraph", "Streamgraph", "area"),
          s("difference", "Difference chart", "area"),
          s("range-area", "Range area chart", "area"),
          s("horizon", "Horizon chart", "area"),
          s("sparkline", "Sparkline", "line"),
          s("sparklines-band", "Sparkband", "area"),
        ],
      },
      {
        title: "Temporal markers",
        leaves: [
          s("connected-scatter", "Connected scatter", "line"),
          s("cycle", "Cycle plot", "line"),
          s("seasonal", "Seasonal plot", "line"),
          s("index-chart", "Index chart", "line"),
          s("fan", "Fan chart", "area"),
          s("control", "Control chart", "line"),
        ],
      },
    ],
  },
  {
    id: "6",
    title: "Correlation and relationship charts",
    groups: [
      {
        title: "Point",
        leaves: [
          s("scatter", "Scatter plot", "dot"),
          s("bubble", "Bubble chart", "dot"),
          s("scatter-matrix", "Scatterplot matrix", "dot"),
          s("qq", "Q-Q plot", "dot"),
          s("pp", "P-P plot", "dot"),
          s("residual", "Residual plot", "dot"),
          s("lag", "Lag plot", "dot"),
        ],
      },
      {
        title: "Bivariate surface",
        leaves: [
          s("heatmap", "Heatmap", "heat"),
          s("correlation-matrix", "Correlation matrix", "heat"),
          s("voronoi-scatter", "Voronoi scatter", "voronoi"),
          s("density-2d", "2d density", "heat"),
        ],
      },
    ],
  },
  {
    id: "7",
    title: "Flow charts",
    groups: [
      {
        title: "Quantity flow",
        leaves: [
          s("sankey", "Sankey diagram", "sankey"),
          s("alluvial", "Alluvial diagram", "sankey"),
          s("parallel-sets", "Parallel sets", "sankey"),
          s("chord", "Chord diagram", "chord"),
          s("arc-flow", "Arc diagram", "net"),
          s("hive", "Hive plot", "net"),
          s("od-matrix", "Origin-destination matrix", "heat"),
        ],
      },
    ],
  },
  {
    id: "8",
    title: "Hierarchy charts",
    groups: [
      {
        title: "Trees",
        leaves: [
          s("tree", "Node-link tree", "tree"),
          s("cluster-tree", "Cluster dendrogram", "tree"),
          s("tidy-tree", "Tidy tree", "tree"),
          s("radial-tree", "Radial tree", "tree"),
          s("circular-dendrogram", "Circular dendrogram", "tree"),
          s("pack", "Circle pack", "pack"),
          s("nested-circles", "Nested circles", "pack"),
          s("enclosure", "Enclosure diagram", "pack"),
          s("indented-tree", "Indented tree", "tree"),
          s("sunburst-hierarchy", "Sunburst hierarchy", "pie"),
          s("icicle-hierarchy", "Icicle hierarchy", "tree"),
        ],
      },
    ],
  },
  {
    id: "9",
    title: "Network charts",
    groups: [
      {
        title: "Graphs",
        leaves: [
          s("force", "Force-directed graph", "force"),
          s("arc", "Arc graph", "net"),
          s("adjacency", "Adjacency matrix", "heat"),
          s("radial-network", "Radial network", "net"),
          s("layered-graph", "Layered / Sugiyama graph", "net"),
          s("bipartite", "Bipartite graph", "net"),
          s("ego", "Ego network", "force"),
        ],
      },
    ],
  },
  {
    id: "10",
    title: "Geospatial charts",
    groups: [
      {
        title: "Maps",
        leaves: [
          s("choropleth", "Choropleth", "geo"),
          s("proportional-symbol", "Proportional symbol map", "geo"),
          s("dot-density-map", "Dot density map", "geo"),
          s("isoline", "Isoline map", "geo"),
          s("isopleth", "Isopleth map", "geo"),
          s("cartogram", "Cartogram", "geo"),
          s("hexbin-map", "Hexbin map", "geo"),
          s("tile-grid-map", "Tile grid map", "geo"),
          s("flow-map", "Flow map", "geo"),
          s("connection-map", "Connection map", "geo"),
        ],
      },
    ],
  },
  {
    id: "11",
    title: "Multivariate charts",
    groups: [
      {
        title: "High dimensional",
        leaves: [
          s("parallel-coordinates", "Parallel coordinates", "parallel"),
          s("radar", "Radar chart", "radar"),
          s("spider", "Spider chart", "radar"),
          s("star-plot", "Star plot", "radar"),
          s("andrews", "Andrews plot", "parallel"),
          s("chernoff", "Chernoff faces", "special"),
          s("profile", "Profile plot", "parallel"),
          s("table-lens", "Table lens", "heat"),
        ],
      },
    ],
  },
  {
    id: "12",
    title: "Uncertainty charts",
    groups: [
      {
        title: "Error and interval",
        leaves: [
          s("error-bar", "Error bar chart", "dist"),
          s("confidence-band", "Confidence band", "area"),
          s("prediction-interval", "Prediction interval", "area"),
          s("gradient-interval", "Gradient interval", "dist"),
          s("quantile-dot", "Quantile dot plot", "dot"),
          s("fan-uncertainty", "Fan uncertainty", "area"),
          s("hypothetical-outcome", "Hypothetical outcome plot", "dot"),
        ],
      },
    ],
  },
  {
    id: "13",
    title: "Statistical charts",
    groups: [
      {
        title: "Inference",
        leaves: [
          s("qq-stat", "Normal Q-Q", "dot"),
          s("residual-leverage", "Residual vs leverage", "dot"),
          s("cooks", "Cook's distance", "bar"),
          s("acf", "Autocorrelation", "bar"),
          s("pacf", "Partial autocorrelation", "bar"),
          s("roc", "ROC curve", "line"),
          s("precision-recall", "Precision-recall", "line"),
          s("forest", "Forest plot", "dot"),
          s("bland-altman", "Bland-Altman", "dot"),
        ],
      },
    ],
  },
  {
    id: "14",
    title: "Financial charts",
    groups: [
      {
        title: "Markets",
        leaves: [
          s("candlestick", "Candlestick", "bar"),
          s("ohlc", "OHLC bar", "bar"),
          s("kagi", "Kagi", "line"),
          s("renko", "Renko", "bar"),
          s("point-figure", "Point and figure", "special"),
          s("waterfall", "Waterfall chart", "bar"),
          s("volume", "Volume bars", "bar"),
          s("equity-curve", "Equity curve", "area"),
        ],
      },
    ],
  },
  {
    id: "15",
    title: "Calendar and temporal charts",
    groups: [
      {
        title: "Calendars",
        leaves: [
          s("calendar-heatmap", "Calendar heatmap", "calendar"),
          s("github-contrib", "Contribution calendar", "calendar"),
          s("gantt", "Gantt chart", "gantt"),
          s("timeline", "Timeline", "gantt"),
          s("swimlane", "Swimlane", "gantt"),
          s("spiral", "Spiral chart", "calendar"),
          s("polar-clock", "Polar clock", "calendar"),
        ],
      },
    ],
  },
  {
    id: "16",
    title: "Text and qualitative charts",
    groups: [
      {
        title: "Words",
        leaves: [
          s("word-cloud", "Word cloud", "text"),
          s("bar-of-words", "Word bar chart", "bar"),
          s("phrase-net", "Phrase net", "net"),
          s("alluvial-text", "Text alluvial", "sankey"),
        ],
      },
    ],
  },
  {
    id: "17",
    title: "Scientific charts",
    groups: [
      {
        title: "Lab",
        leaves: [
          s("function", "Function plot", "line"),
          s("parametric", "Parametric plot", "line"),
          s("polar", "Polar plot", "radar"),
          s("vector-field", "Vector field", "science"),
          s("streamline", "Streamline plot", "science"),
          s("phase", "Phase portrait", "science"),
          s("ternary", "Ternary plot", "science"),
          s("smith", "Smith chart", "science"),
          s("bode", "Bode plot", "line"),
          s("nyquist", "Nyquist plot", "line"),
        ],
      },
    ],
  },
  {
    id: "18",
    title: "Process and state charts",
    groups: [
      {
        title: "Process",
        leaves: [
          s("flowchart", "Flowchart", "process"),
          s("state-machine", "State machine", "process"),
          s("activity", "Activity diagram", "process"),
          s("sequence-diagram", "Sequence diagram", "process"),
          s("event-storm", "Event storm", "process"),
        ],
      },
    ],
  },
  {
    id: "19",
    title: "Axes, legends, and annotation charts",
    groups: [
      {
        title: "Chrome",
        leaves: [
          s("axis-linear", "Linear axis", "bar", "axis"),
          s("axis-log", "Log axis", "bar", "axis"),
          s("axis-band", "Band axis", "bar", "axis"),
          s("grid-cartesian", "Cartesian grid", "bar", "axis"),
          s("legend-swatch", "Swatch legend", "bar", "axis"),
          s("legend-gradient", "Gradient legend", "heat", "axis"),
          s("legend-size", "Size legend", "dot", "axis"),
        ],
      },
    ],
  },
];

function md(): string {
  const lines = ["# Exhaustive Visualization Taxonomy for a TikZ-Based D3 for LaTeX Library", ""];
  for (const section of SECTIONS) {
    lines.push(`## ${section.id}. ${section.title}`, "");
    for (const group of section.groups) {
      lines.push(`### ${group.title}`, "");
      for (const leaf of group.leaves) lines.push(`- ${leaf.title} \`${leaf.slug}\` ${leaf.kind}`);
      lines.push("");
    }
  }
  return `${lines.join("\n")}\n`;
}

function leaves(): { readonly id: string; readonly slug: string; readonly title: string; readonly kind: string; readonly family: string; readonly section: string }[] {
  const out = [];
  for (const section of SECTIONS) {
    for (const group of section.groups) {
      for (const leaf of group.leaves) {
        out.push({ id: `${section.id}/${leaf.slug}`, slug: leaf.slug, title: leaf.title, kind: leaf.kind, family: leaf.family, section: section.id });
      }
    }
  }
  return out;
}

function galleryTex(section: Section): string {
  const body: string[] = [];
  for (const group of section.groups) {
    body.push(`\\section{${group.title}}`);
    for (const leaf of group.leaves) {
      body.push(`% viz-covers: ${section.id}/${leaf.slug}`);
      body.push(`\\begin{VizFigure}[title={${leaf.title}}, width=80, height=40]`);
      body.push(`\\SemioVizDemo{${leaf.slug}}`);
      body.push("\\end{VizFigure}");
    }
  }
  return `\\documentclass[type=report,theme=light,language=de]{semio}
\\title{${section.title}}
\\author{Semio}
\\date{\\today}
\\begin{document}
\\chapter{${section.title}}
${body.join("\n")}
\\end{document}
`;
}

const CAT_FILE: Record<string, string> = {
  "1": "comparison",
  "2": "ranking",
  "3": "distribution",
  "4": "part",
  "5": "trend",
  "6": "correlation",
  "7": "flow",
  "8": "hierarchy",
  "9": "network",
  "10": "geo",
  "11": "multivariate",
  "12": "uncertainty",
  "13": "statistical",
  "14": "financial",
  "15": "calendar",
  "16": "text",
  "17": "science",
  "18": "process",
  "19": "chrome",
};

function chartSty(section: Section): string {
  const file = CAT_FILE[section.id];
  const lines = [
    `\\NeedsTeXFormat{LaTeX2e}`,
    `\\ProvidesPackage{semio-viz-chart-${file}}[2026/08/18 v0.1.0 semio viz ${file} charts]`,
    `%region Kinds`,
  ];
  for (const group of section.groups) {
    for (const leaf of group.leaves) lines.push(`\\SemioVizChartKind{${leaf.slug}}{${leaf.family}}{data=demo}`);
  }
  lines.push(`%endregion Kinds`);
  return `${lines.join("\n")}\n`;
}

const repo = join(import.meta.dir, "../../../../../../");
const printRoot = join(repo, "print");
const ticket = import.meta.dir;
const all = leaves();
writeFileSync(join(printRoot, "asset/viz-taxonomy.md"), md());
writeFileSync(join(printRoot, "asset/viz-taxonomy.json"), JSON.stringify(all, null, 2));
writeFileSync(join(ticket, "wp-registry.json"), JSON.stringify(all, null, 2));
writeFileSync(join(ticket, "wp-registry.md"), all.map((l) => `- \`${l.id}\` → family \`${l.family}\` (${l.kind})`).join("\n") + "\n");
mkdirSync(join(printRoot, "template/viz-gallery"), { recursive: true });
for (const section of SECTIONS) {
  const name = `viz-${section.id}.tex`;
  writeFileSync(join(printRoot, "template/viz-gallery", name), galleryTex(section));
}
for (const section of SECTIONS) {
  if (section.id === "0") continue;
  const file = CAT_FILE[section.id];
  if (!file) throw new Error(`no chart file for section ${section.id}`);
  writeFileSync(join(printRoot, "tex", `semio-viz-chart-${file}.sty`), chartSty(section));
}
console.log(`print viz taxonomy: ${all.length} leaves, ${SECTIONS.length} gallery files`);
