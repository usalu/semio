#!/usr/bin/env bun
/** 📊 Ticket generator: parse the exhaustive taxonomy into galleries and chart kinds. Run from repo root. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

type Kind = "mark" | "chart" | "layout" | "axis" | "scale";
type Leaf = { readonly slug: string; readonly title: string; readonly kind: Kind; readonly family: string };
type Group = { readonly title: string; readonly leaves: readonly Leaf[] };
type Section = { readonly id: string; readonly title: string; readonly groups: readonly Group[] };
type Node = { title: string; children: Node[] };

const FAMILIES = new Set([
  "bar", "dot", "line", "area", "pie", "dist", "tree", "net", "flow", "geo", "heat", "radar",
  "parallel", "waffle", "funnel", "gantt", "bullet", "pack", "chord", "sankey", "force",
  "voronoi", "hexbin", "calendar", "text", "science", "process", "special", "path", "mark", "anno", "chrome",
]);

function slugify(title: string): string {
  const slug = title
    .normalize("NFKD")
    .replace(/['’]/g, "")
    .replace(/[–—]/g, "-")
    .replace(/[%]/g, "percent")
    .replace(/&/g, "and")
    .replace(/\+/g, "plus")
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
  return slug || "item";
}

function stripDecor(title: string): string {
  return title.trim().replace(/^\*\*(.+)\*\*$/, "$1").replace(/^`(.+)`$/, "$1").trim();
}

function isCategoryName(title: string): boolean {
  if (title.includes(" / ")) return true;
  if (/\b(visualizations|capabilities|namespaces|encodings|transforms|infrastructure|objective|equivalents)\b/i.test(title)) return true;
  if (/(charts|plots|diagrams|maps|families|types|views|structures|representations|indicators|algorithms)$/i.test(title)) return true;
  return false;
}

function kindFor(sectionId: string, group: string, title: string): Kind {
  if (sectionId === "0") return "mark";
  if (sectionId === "75" || sectionId === "76" || sectionId === "77" || sectionId === "79") return "layout";
  if (sectionId === "74") return /position|size|shape|appearance|orientation|compound/.test(group.toLowerCase()) ? "scale" : "mark";
  if (sectionId === "78") {
    if (/^scales$/i.test(group)) return "scale";
    if (/^axes$/i.test(group)) return "axis";
    return "layout";
  }
  if (sectionId === "51" && /\baxis\b/i.test(title)) return "axis";
  if (/\b(legend|grid)\b/i.test(title) && /axis|legend|grid|chrome|guide/i.test(group)) return "axis";
  return "chart";
}

function familyFor(sectionId: string, group: string, title: string, kind: Kind): string {
  if (kind === "mark") {
    const hay = `${group} ${title}`.toLowerCase();
    if (/label|callout|leader|bracket|brace|highlight|reference|annotation/.test(hay)) return "anno";
    if (/voronoi/.test(hay)) return "voronoi";
    if (/points|marks|dot|circle|square|rectangle|triangle|diamond|cross|plus|star|glyph|icon|image/.test(hay) && !/line|arc|region|connector|path|polygon|ribbon|band/.test(title.toLowerCase())) return "mark";
    return "path";
  }
  if (kind === "axis" || kind === "scale") return "chrome";
  const hay = `${group} ${title}`.toLowerCase();
  const rules: readonly (readonly [RegExp, string])[] = [
    [/sankey/, "sankey"],
    [/alluvial|theme\s*river|streamgraph/, "area"],
    [/chord|dependency wheel/, "chord"],
    [/treemap|icicle|sunburst|circle pack|pack layout|enclosure/, "pack"],
    [/waffle|pictogram|isotype|unit chart|icon array|repeated-symbol/, "waffle"],
    [/funnel/, "funnel"],
    [/gantt|pert|roadmap|kanban|scrum|schedule|timetable|burndown|burnup/, "gantt"],
    [/bullet|gauge|kpi|progress|thermometer|speedometer|dial|scorecard|big-number/, "bullet"],
    [/pie|donut|coxcomb|nightingale|polar-area|polar area|rose diagram|hemicycle|parliament|seat chart/, "pie"],
    [/histogram|density|violin|box plot|boxen|ridgeline|joyplot|beeswarm|swarm|qq|q-q|p-p|strip plot|jitter|sina|raincloud|stem-and-leaf|ogive|kde|quantile|lorenz|ecdf/, "dist"],
    [/hexbin/, "hexbin"],
    [/heatmap|heat map|confusion matrix|correlation matrix|adjacency matrix|distance matrix|co-occurrence matrix/, "heat"],
    [/calendar|planner|contribution heatmap/, "calendar"],
    [/choropleth|cartogram|graticule|isoline|isopleth|isochrone|terrain|hillshade|floor plan|site plan|map\b|geo|cartographic/, "geo"],
    [/dendrogram|phylogen|cladogram|org(?:anization)? chart|family tree|mind map|parse tree|syntax tree|trie|rooted tree|radial tree/, "tree"],
    [/force-directed|spring layout|fruchterman|kamada|force layout/, "force"],
    [/\b(graph|network|automaton|petri|fsm|dag\b|call graph|knowledge graph)\b/, "net"],
    [/parallel coord|parallel set|andrews/, "parallel"],
    [/radar|spider|star plot|polar profile|circumplex/, "radar"],
    [/area chart|range-area|horizon chart|confidence-band|ribbon/, "area"],
    [/line chart|sparkline|slope|bump chart|cycle plot|seasonal|fan chart|control chart|equity curve/, "line"],
    [/scatter|bubble chart|dot plot|cleveland|dumbbell|lollipop|connected scatter/, "dot"],
    [/bar chart|column chart|waterfall|tornado|butterfly|population pyramid|pareto|volume bar/, "bar"],
    [/word cloud|tag cloud|concordance|kwic|topic |text |lexical|sentence diagram/, "text"],
    [/flowchart|uml|bpmn|sequence diagram|state-machine|state machine|workflow|process map|c4 |er diagram|architecture diagram/, "process"],
    [/vector field|quiver|streamline|contour|surface|bode|nyquist|phase portrait|ternary|smith chart|function plot|waveform|spectrogram|feynman/, "science"],
    [/voronoi/, "voronoi"],
    [/flow map|migration|traffic-flow|user-flow|material-flow/, "flow"],
    [/legend|grid |axis |facet|layer|overlay|inset|small multiple|dashboard|composition/, "chrome"],
    [/venn|euler|upset/, "pack"],
    [/table\b|crosstab|pivot|scorecard/, "heat"],
    [/timeline|swimlane|chronology/, "gantt"],
    [/matrix/, "heat"],
  ];
  for (const [pattern, family] of rules) {
    if (pattern.test(hay) && FAMILIES.has(family)) return family;
  }
  const bySection: Record<string, string> = {
    "1": "bar", "2": "line", "3": "dist", "4": "dot", "5": "radar", "6": "pie", "7": "tree", "8": "net",
    "9": "sankey", "10": "geo", "11": "heat", "12": "heat", "13": "text", "14": "process", "15": "process",
    "16": "gantt", "17": "line", "18": "bullet", "19": "funnel", "20": "radar", "21": "chord", "22": "dist",
    "23": "dist", "24": "heat", "25": "science", "26": "science", "27": "science", "28": "science", "29": "science",
    "30": "science", "31": "process", "32": "science", "33": "science", "34": "science", "35": "geo", "36": "process",
    "37": "geo", "38": "process", "39": "pack", "40": "gantt", "41": "dist", "42": "bar", "43": "geo", "44": "science",
    "45": "process", "46": "text", "47": "pack", "48": "chrome", "49": "chrome", "50": "anno", "51": "chrome",
    "52": "chrome", "53": "heat", "54": "calendar", "55": "pie", "56": "process", "57": "tree", "58": "net",
    "59": "geo", "60": "science", "61": "special", "62": "gantt", "63": "net", "64": "net", "65": "line",
    "66": "process", "67": "process", "68": "science", "69": "bar", "70": "geo", "71": "process", "72": "gantt",
    "73": "chrome", "74": "mark", "75": "pack", "76": "chrome", "77": "chrome", "78": "chrome", "79": "chrome",
  };
  return bySection[sectionId] ?? "special";
}

function parseTaxonomy(md: string): Section[] {
  const sections: { id: string; title: string; roots: Node[] }[] = [];
  let current: { id: string; title: string; roots: Node[] } | undefined;
  const stack: { depth: number; node: Node }[] = [];
  for (const raw of md.split(/\n/)) {
    const sectionMatch = raw.match(/^- \*\*(\d+)\.\s+(.+?)\*\*\s*$/);
    if (sectionMatch) {
      current = { id: sectionMatch[1]!, title: sectionMatch[2]!, roots: [] };
      sections.push(current);
      stack.length = 0;
      continue;
    }
    if (!current) continue;
    const bullet = raw.match(/^( *)- (.+)$/);
    if (!bullet) continue;
    const depth = Math.floor(bullet[1]!.length / 2);
    if (depth < 1) continue;
    const node: Node = { title: stripDecor(bullet[2]!), children: [] };
    while (stack.length && stack[stack.length - 1]!.depth >= depth) stack.pop();
    if (stack.length === 0) current.roots.push(node);
    else stack[stack.length - 1]!.node.children.push(node);
    stack.push({ depth, node });
  }
  return sections.map((section) => flattenSection(section));
}

function flattenSection(section: { id: string; title: string; roots: Node[] }): Section {
  const used = new Set<string>();
  const groups: Group[] = [];
  const general: Leaf[] = [];
  const toLeaf = (groupTitle: string, title: string): Leaf => {
    const kind = kindFor(section.id, groupTitle, title);
    const family = familyFor(section.id, groupTitle, title, kind);
    let slug = slugify(title);
    let n = 2;
    while (used.has(slug)) slug = `${slugify(title)}-${n++}`;
    used.add(slug);
    return { slug, title, kind, family };
  };
  const collect = (groupTitle: string, node: Node, into: Leaf[]): void => {
    if (node.children.length === 0) {
      into.push(toLeaf(groupTitle, node.title));
      return;
    }
    if (!isCategoryName(node.title)) into.push(toLeaf(groupTitle, node.title));
    for (const child of node.children) collect(groupTitle, child, into);
  };
  for (const root of section.roots) {
    if (root.children.length === 0) {
      general.push(toLeaf("General", root.title));
      continue;
    }
    const leaves: Leaf[] = [];
    if (!isCategoryName(root.title)) leaves.push(toLeaf(root.title, root.title));
    for (const child of root.children) collect(root.title, child, leaves);
    if (leaves.length > 0) groups.push({ title: root.title, leaves });
  }
  if (general.length > 0) groups.unshift({ title: "General", leaves: general });
  return { id: section.id, title: section.title, groups };
}

function md(sections: readonly Section[]): string {
  const lines = ["# Exhaustive Visualization Taxonomy for a TikZ-Based D3 for LaTeX Library", ""];
  for (const section of sections) {
    lines.push(`## ${section.id}. ${section.title}`, "");
    for (const group of section.groups) {
      lines.push(`### ${group.title}`, "");
      for (const leaf of group.leaves) lines.push(`- ${leaf.title} \`${leaf.slug}\` ${leaf.kind}`);
      lines.push("");
    }
  }
  return `${lines.join("\n")}\n`;
}

function allLeaves(sections: readonly Section[]): { readonly id: string; readonly slug: string; readonly title: string; readonly kind: string; readonly family: string; readonly section: string }[] {
  const out = [];
  for (const section of sections) {
    for (const group of section.groups) {
      for (const leaf of group.leaves) {
        out.push({ id: `${section.id}/${leaf.slug}`, slug: leaf.slug, title: leaf.title, kind: leaf.kind, family: leaf.family, section: section.id });
      }
    }
  }
  return out;
}

function texText(value: string): string {
  return value.replace(/[%#&_$]/g, (ch) => `\\${ch}`);
}

function galleryTex(section: Section): string {
  const body: string[] = [];
  for (const group of section.groups) {
    body.push(`\\section{${texText(group.title)}}`);
    for (const leaf of group.leaves) {
      body.push(`% viz-covers: ${section.id}/${leaf.slug}`);
      body.push(`\\begin{VizFigure}[title={${texText(leaf.title)}}, width=80, height=40]`);
      body.push(`\\SemioVizDemo{${leaf.slug}}`);
      body.push("\\end{VizFigure}");
    }
  }
  return `\\documentclass[type=report,theme=light,language=de]{semio}
\\title{${texText(section.title)}}
\\author{Semio}
\\date{\\today}
\\begin{document}
\\chapter{${texText(section.title)}}
${body.join("\n")}
\\end{document}
`;
}

function extraOpts(leaf: Leaf): string {
  if (leaf.family !== "chrome") return "";
  if (/gradient/.test(leaf.slug) || /gradient/.test(leaf.title.toLowerCase())) return ",legend=gradient";
  if (/legend-size|size legend/.test(`${leaf.slug} ${leaf.title.toLowerCase()}`)) return ",legend=size";
  if (leaf.kind !== "axis") return "";
  if (/log/.test(leaf.slug) || /\blog\b/.test(leaf.title.toLowerCase())) return ",scale=y-log,orient=left";
  if (/band|categorical/.test(`${leaf.slug} ${leaf.title.toLowerCase()}`)) return ",scale=x,orient=bottom";
  return ",scale=y,orient=left";
}

function kindsSty(sections: readonly Section[]): string {
  const lines = [
    `\\NeedsTeXFormat{LaTeX2e}`,
    `\\ProvidesPackage{semio-viz-chart-kinds}[2026/08/18 v0.1.0 semio viz chart kinds]`,
  ];
  for (const section of sections) {
    if (section.id === "0") continue;
    lines.push(`%region Section${section.id}`);
    for (const group of section.groups) {
      for (const leaf of group.leaves) {
        lines.push(`\\SemioVizChartKind{${leaf.slug}}{${leaf.family}}{data=demo${extraOpts(leaf)}}`);
      }
    }
    lines.push(`%endregion Section${section.id}`);
  }
  return `${lines.join("\n")}\n`;
}

const ticket = import.meta.dir;
const repo = join(ticket, "../../../../../../");
const printRoot = join(repo, "print");
const source = readFileSync(join(ticket, "exhaustive-taxonomy.md"), "utf8");
const sections = parseTaxonomy(source);
const all = allLeaves(sections);
const unknown = [...new Set(all.map((leaf) => leaf.family).filter((family) => !FAMILIES.has(family)))];
if (unknown.length > 0) throw new Error(`unknown families: ${unknown.join(", ")}`);
const ids = all.map((leaf) => leaf.id);
if (new Set(ids).size !== ids.length) throw new Error("duplicate coverage ids");
writeFileSync(join(printRoot, "asset/viz-taxonomy.md"), md(sections));
writeFileSync(join(printRoot, "asset/viz-taxonomy.json"), JSON.stringify(all, null, 2));
writeFileSync(join(ticket, "wp-registry.json"), JSON.stringify(all, null, 2));
writeFileSync(join(ticket, "wp-registry.md"), `${all.map((leaf) => `- \`${leaf.id}\` → family \`${leaf.family}\` (${leaf.kind})`).join("\n")}\n`);
mkdirSync(join(printRoot, "template/viz-gallery"), { recursive: true });
for (const section of sections) {
  writeFileSync(join(printRoot, "template/viz-gallery", `viz-${section.id}.tex`), galleryTex(section));
}
writeFileSync(join(printRoot, "tex", "semio-viz-chart-kinds.sty"), kindsSty(sections));
const counts = new Map<string, number>();
for (const leaf of all) counts.set(leaf.family, (counts.get(leaf.family) ?? 0) + 1);
console.log(`print viz taxonomy: ${all.length} leaves, ${sections.length} gallery files`);
console.log(`families: ${[...counts.entries()].map(([family, n]) => `${family}=${n}`).join(" ")}`);
