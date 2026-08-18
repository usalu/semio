#!/usr/bin/env bun
/** 🔎 Independent phase-4 review: re-derive the taxonomy and diff it against generated artefacts. */
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const ticket = import.meta.dir;
const repo = join(ticket, "../../../../../../");
const printRoot = join(repo, "print");
const galleryDir = join(printRoot, "template/viz-gallery");

type Node = { title: string; children: Node[]; depth: number; line: number; section: string };

function stripDecor(title: string): string {
  return title.trim().replace(/^\*\*(.+)\*\*$/, "$1").replace(/^`(.+)`$/, "$1").trim();
}

const source = readFileSync(join(ticket, "exhaustive-taxonomy.md"), "utf8");
const sections: { id: string; title: string; roots: Node[] }[] = [];
{
  let current: { id: string; title: string; roots: Node[] } | undefined;
  const stack: Node[] = [];
  let lineNo = 0;
  for (const raw of source.split(/\n/)) {
    lineNo += 1;
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
    const node: Node = { title: stripDecor(bullet[2]!), children: [], depth, line: lineNo, section: current.id };
    while (stack.length && stack[stack.length - 1]!.depth >= depth) stack.pop();
    if (stack.length === 0) current.roots.push(node);
    else stack[stack.length - 1]!.children.push(node);
    stack.push(node);
  }
}

const allNodes: Node[] = [];
const walk = (node: Node): void => {
  allNodes.push(node);
  for (const child of node.children) walk(child);
};
for (const section of sections) for (const root of section.roots) walk(root);

const parents = allNodes.filter((node) => node.children.length > 0);

// generated manifest
const manifest = JSON.parse(readFileSync(join(printRoot, "asset/viz-taxonomy.json"), "utf8")) as {
  id: string; slug: string; title: string; kind: string; family: string; section: string;
}[];
const manifestByKey = new Map(manifest.map((leaf) => [leaf.id, leaf]));
const manifestTitlesBySection = new Map<string, Set<string>>();
for (const leaf of manifest) {
  const set = manifestTitlesBySection.get(leaf.section) ?? new Set<string>();
  set.add(leaf.title);
  manifestTitlesBySection.set(leaf.section, set);
}

const out: string[] = [];
const say = (line = ""): void => { out.push(line); console.log(line); };

say(`sections parsed: ${sections.length}`);
say(`bullets total (depth>=1): ${allNodes.length}`);
say(`parent bullets (have children): ${parents.length}`);
say(`terminal bullets (no children): ${allNodes.length - parents.length}`);
say(`manifest leaves: ${manifest.length}`);
say();

// --- 1. which parents were promoted to leaves, which were dropped
const promoted: Node[] = [];
const dropped: Node[] = [];
for (const node of parents) {
  const titles = manifestTitlesBySection.get(node.section);
  if (titles?.has(node.title)) promoted.push(node); else dropped.push(node);
}
say(`## parents promoted to leaves (${promoted.length})`);
for (const node of promoted) say(`- ${node.section} L${node.line} d${node.depth} "${node.title}" (children=${node.children.length})`);
say();
say(`## parents dropped as categories (${dropped.length})`);
for (const node of dropped) say(`- ${node.section} L${node.line} d${node.depth} "${node.title}" (children=${node.children.length})`);
say();

// --- 2. terminal bullets missing from manifest
const missingTerminals: Node[] = [];
for (const node of allNodes) {
  if (node.children.length > 0) continue;
  const titles = manifestTitlesBySection.get(node.section);
  if (!titles?.has(node.title)) missingTerminals.push(node);
}
say(`## terminal bullets absent from manifest (${missingTerminals.length})`);
for (const node of missingTerminals) say(`- ${node.section} L${node.line} "${node.title}"`);
say();

// --- 3. duplicate titles inside one section (lost leaves)
const dupTitles: string[] = [];
for (const section of sections) {
  const seen = new Map<string, number>();
  const collect = (node: Node): void => {
    seen.set(node.title, (seen.get(node.title) ?? 0) + 1);
    for (const child of node.children) collect(child);
  };
  for (const root of section.roots) collect(root);
  for (const [title, n] of seen) if (n > 1) dupTitles.push(`${section.id}: "${title}" x${n}`);
}
say(`## repeated titles within the same section (${dupTitles.length})`);
for (const line of dupTitles) say(`- ${line}`);
say();

// --- 4. slug uniqueness + collision-suffix rule
const slugCounts = new Map<string, number>();
for (const leaf of manifest) slugCounts.set(leaf.slug, (slugCounts.get(leaf.slug) ?? 0) + 1);
const dupSlugs = [...slugCounts.entries()].filter(([, n]) => n > 1);
say(`## duplicate global slugs (${dupSlugs.length})`);
for (const [slug, n] of dupSlugs) say(`- ${slug} x${n}`);
const badSuffix: string[] = [];
for (const leaf of manifest) {
  if (!/-\d+(-\d+)?$/.test(leaf.slug)) continue;
  const m = leaf.slug.match(/^(.*)-(\d+)$/);
  if (!m) continue;
  if (m[2] !== leaf.section && !leaf.slug.endsWith(`-${leaf.section}-2`)) badSuffix.push(`${leaf.id} slug=${leaf.slug} title="${leaf.title}"`);
}
say(`## slugs with numeric tail not equal to own section (may be natural digits) (${badSuffix.length})`);
for (const line of badSuffix) say(`- ${line}`);
say();

// --- 5. kinds per spec
const kindIssues: string[] = [];
for (const leaf of manifest) {
  const s = leaf.section;
  const expect = (allowed: string[]): void => {
    if (!allowed.includes(leaf.kind)) kindIssues.push(`${leaf.id} kind=${leaf.kind} expected ${allowed.join("|")}`);
  };
  if (s === "0") expect(["mark"]);
  else if (s === "74") expect(["mark", "scale"]);
  else if (["75", "76", "77", "79"].includes(s)) expect(["layout"]);
  else if (s === "78") expect(["scale", "axis", "layout"]);
  else if (s === "51") expect(["chart", "axis"]);
  else expect(["chart", "axis"]);
}
say(`## kind rule violations (${kindIssues.length})`);
for (const line of kindIssues.slice(0, 40)) say(`- ${line}`);
say();
const kindCounts = new Map<string, number>();
for (const leaf of manifest) kindCounts.set(leaf.kind, (kindCounts.get(leaf.kind) ?? 0) + 1);
say(`kinds: ${[...kindCounts].map(([k, n]) => `${k}=${n}`).join(" ")}`);
const s51 = manifest.filter((leaf) => leaf.section === "51");
say(`section 51: ${s51.map((leaf) => `${leaf.slug}:${leaf.kind}`).join(" ")}`);
say();

// --- 6. gallery covers vs manifest
const covers = new Set<string>();
const demos = new Map<string, string[]>();
const galleryFiles = readdirSync(galleryDir).filter((name) => name.endsWith(".tex"));
for (const name of galleryFiles) {
  const tex = readFileSync(join(galleryDir, name), "utf8");
  for (const match of tex.matchAll(/% viz-covers:\s+(\S+)/g)) covers.add(match[1]!);
  for (const match of tex.matchAll(/\\SemioVizDemo\{([^}]*)\}/g)) {
    const list = demos.get(name) ?? [];
    list.push(match[1]!);
    demos.set(name, list);
  }
}
say(`gallery files: ${galleryFiles.length} (${galleryFiles.filter((n) => /^viz-\d+\.tex$/.test(n)).length} numbered)`);
const missingCovers = manifest.filter((leaf) => !covers.has(leaf.id)).map((leaf) => leaf.id);
const extraCovers = [...covers].filter((key) => !manifestByKey.has(key));
say(`## manifest leaves without cover (${missingCovers.length}): ${missingCovers.slice(0, 20).join(", ")}`);
say(`## covers without manifest leaf (${extraCovers.length}): ${extraCovers.slice(0, 20).join(", ")}`);

// cover/demo pairing per file
const pairIssues: string[] = [];
for (const name of galleryFiles) {
  const tex = readFileSync(join(galleryDir, name), "utf8");
  const pairs = [...tex.matchAll(/% viz-covers:\s+(\S+)[\s\S]*?\\SemioVizDemo\{([^}]*)\}/g)];
  for (const pair of pairs) {
    const key = pair[1]!;
    const slug = pair[2]!;
    if (key.split("/").slice(1).join("/") !== slug) pairIssues.push(`${name}: ${key} -> demo ${slug}`);
  }
}
say(`## cover key / demo slug mismatches (${pairIssues.length})`);
for (const line of pairIssues.slice(0, 20)) say(`- ${line}`);
say();

// --- 7. chart packages / loader
const texDir = join(printRoot, "tex");
const loader = readFileSync(join(texDir, "semio-viz-charts.sty"), "utf8");
const required = [...loader.matchAll(/\\RequirePackage\{semio-viz-chart-([^}]+)\}/g)].map((m) => m[1]!);
const onDisk = readdirSync(texDir)
  .filter((name) => /^semio-viz-chart-.+\.sty$/.test(name))
  .map((name) => name.replace(/^semio-viz-chart-/, "").replace(/\.sty$/, ""));
say(`loader requires ${required.length} chart packages; ${onDisk.length} chart .sty files on disk`);
const staleFiles = onDisk.filter((name) => !required.includes(name));
const missingFiles = required.filter((name) => !onDisk.includes(name));
say(`## stale chart packages not required by loader (${staleFiles.length}): ${staleFiles.join(", ")}`);
say(`## required chart packages missing on disk (${missingFiles.length}): ${missingFiles.join(", ")}`);

// registered kinds
const registered = new Map<string, { family: string; pkg: string }[]>();
for (const name of onDisk) {
  if (!required.includes(name)) continue;
  const tex = readFileSync(join(texDir, `semio-viz-chart-${name}.sty`), "utf8");
  for (const match of tex.matchAll(/\\SemioVizChartKind\{([^}]*)\}\{([^}]*)\}\{([^}]*)\}/g)) {
    const list = registered.get(match[1]!) ?? [];
    list.push({ family: match[2]!, pkg: name });
    registered.set(match[1]!, list);
  }
}
say(`registered chart kinds (required packages only): ${[...registered.keys()].length}`);
const dupRegistered = [...registered.entries()].filter(([, list]) => list.length > 1);
say(`## chart kinds registered more than once (${dupRegistered.length})`);
for (const [slug, list] of dupRegistered.slice(0, 20)) say(`- ${slug}: ${list.map((entry) => `${entry.pkg}/${entry.family}`).join(", ")}`);

const markSty = readFileSync(join(texDir, "semio-viz-mark.sty"), "utf8");
const markKinds = new Set([...markSty.matchAll(/\\semio_viz_mark_kind_define:nn+\s*\{([^}]*)\}/g)].map((m) => m[1]!));
const markRegistered = new Set([...markSty.matchAll(/\{\s*semio_viz_mark_(?:kind|path)[^}]*\}/g)].map((m) => m[0]!));
say(`mark kinds discovered in semio-viz-mark.sty via kind_define: ${markKinds.size}`);

// every demo slug resolvable?
const allDemoSlugs = [...new Set([...demos.values()].flat())];
const unresolved = allDemoSlugs.filter((slug) => !registered.has(slug) && !markKinds.has(slug));
say(`demo slugs total ${allDemoSlugs.length}; unresolved against chart kinds + detected mark kinds: ${unresolved.length}`);
say(`  first unresolved: ${unresolved.slice(0, 25).join(", ")}`);
say();

// --- 8. families
const familyCounts = new Map<string, number>();
for (const leaf of manifest) familyCounts.set(leaf.family, (familyCounts.get(leaf.family) ?? 0) + 1);
say(`families: ${[...familyCounts].sort((a, b) => b[1] - a[1]).map(([f, n]) => `${f}=${n}`).join(" ")}`);
const layoutSty = readFileSync(join(texDir, "semio-viz-layout.sty"), "utf8");
const layoutFamilies = new Set([...layoutSty.matchAll(/semio_viz_layout_([a-z]+):n\s/g)].map((m) => m[1]!));
say(`layout families implemented (heuristic): ${[...layoutFamilies].sort().join(" ")}`);
const missingFamilies = [...familyCounts.keys()].filter((family) => !layoutFamilies.has(family));
say(`## families used by kinds but not found as layout impl (${missingFamilies.length}): ${missingFamilies.join(", ")}`);
say();

// --- 9. viz-api coverage
const apiPath = join(galleryDir, "viz-api.tex");
say(`viz-api.tex exists: ${existsSync(apiPath)}`);
const apiTex = existsSync(apiPath) ? readFileSync(apiPath, "utf8") : "";
const commands = ["\\SemioVizChart", "\\SemioVizMark", "\\SemioVizPath", "\\SemioVizLayout", "\\SemioVizAxis", "\\SemioVizGrid", "\\SemioVizLegend", "\\SemioVizTable", "\\SemioVizRow", "\\SemioVizScale"];
for (const command of commands) say(`  ${command}: api=${apiTex.includes(command)}`);
const declared = new Set<string>();
for (const name of readdirSync(texDir).filter((n) => n.startsWith("semio-viz") && n.endsWith(".sty"))) {
  const tex = readFileSync(join(texDir, name), "utf8");
  for (const match of tex.matchAll(/\\(?:New|Renew|Provide)DocumentCommand\s*\\?(\\?[A-Za-z@]+)/g)) declared.add(match[1]!.replace(/^\\/, ""));
  for (const match of tex.matchAll(/\\NewDocumentEnvironment\s*\{\s*([A-Za-z]+)\s*\}/g)) declared.add(`env:${match[1]!}`);
}
say(`public commands declared across semio-viz*.sty: ${[...declared].sort().join(" ")}`);
const publicOnly = [...declared].filter((name) => name.startsWith("SemioViz"));
const notExercised = publicOnly.filter((name) => !apiTex.includes(`\\${name}`));
say(`## public SemioViz* commands NOT exercised in viz-api.tex (${notExercised.length}): ${notExercised.join(", ")}`);
say();

// --- 10. sanity: taxonomy md vs json
const genMd = readFileSync(join(printRoot, "asset/viz-taxonomy.md"), "utf8");
const mdLeaves: string[] = [];
{
  let section = "";
  for (const line of genMd.split(/\n/)) {
    const s = line.match(/^## (\d+)\./);
    if (s) { section = s[1]!; continue; }
    const leaf = line.match(/^- .*`([^`]+)`\s+(\S+)\s*$/);
    if (leaf && section) mdLeaves.push(`${section}/${leaf[1]!}`);
  }
}
say(`taxonomy.md leaves: ${mdLeaves.length}; json leaves: ${manifest.length}; equal sets: ${mdLeaves.length === manifest.length && mdLeaves.every((key, i) => key === manifest[i]!.id)}`);
const wpRegistry = readFileSync(join(ticket, "wp-registry.json"), "utf8");
say(`wp-registry.json identical to viz-taxonomy.json: ${wpRegistry === readFileSync(join(printRoot, "asset/viz-taxonomy.json"), "utf8")}`);

Bun.write(join(ticket, "review-output.txt"), `${out.join("\n")}\n`);
