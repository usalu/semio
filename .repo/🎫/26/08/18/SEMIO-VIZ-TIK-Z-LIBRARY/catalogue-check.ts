#!/usr/bin/env bun
/** Independent catalogue fidelity check. Run from repo root. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

type Node = { title: string; children: Node[] };
const ticket = import.meta.dir;
const repo = join(ticket, "../../../../../../");
const LEAF_AND_GROUP = new Set([
  "flowchart", "bode plot", "control chart", "combination chart",
  "charts", "hierarchy", "network", "flow", "geo", "matrix", "diagram", "scientific",
  "data", "transform", "mark", "encoding", "guide",
]);
const BAD = [
  "7/trees", "6/pie-family", "6/specialized", "79/maps",
  "79/a-user-should-be-able-to-describe-essentially-any-visualization-as",
  "79/this-grammar-then-generates", "79/all-as-native-tikz-pgf-output",
];
const WANT = [
  "76/charts", "76/hierarchy", "14/flowchart", "31/bode-plot", "42/control-chart",
  "48/combination-chart", "79/data", "79/transform", "79/mark", "79/encoding", "79/guide",
];

function stripDecor(title: string): string {
  return title.trim().replace(/^\*\*(.+)\*\*$/, "$1").replace(/^`(.+)`$/, "$1").trim();
}
function slugify(title: string): string {
  return title.normalize("NFKD").replace(/\p{M}/gu, "").replace(/['’]/g, "").replace(/[–—]/g, "-")
    .replace(/[%]/g, "percent").replace(/&/g, "and").replace(/\+/g, "plus").toLowerCase()
    .replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "") || "item";
}
function isLeafAndGroup(title: string): boolean {
  return LEAF_AND_GROUP.has(stripDecor(title).toLowerCase().replace(/:$/, "").trim());
}
function isProse(title: string): boolean {
  const text = stripDecor(title);
  if (/,$/.test(text)) return true;
  if (/[.]$/.test(text) && text.split(/\s+/).length >= 4) return true;
  if (/:$/.test(text) && text.split(/\s+/).length >= 4) return true;
  return false;
}

const md = readFileSync(join(ticket, "exhaustive-taxonomy.md"), "utf8");
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

const used = new Set<string>();
const expected: { id: string; slug: string }[] = [];
const collect = (sectionId: string, node: Node): void => {
  if (isProse(node.title)) {
    for (const child of node.children) collect(sectionId, child);
    return;
  }
  if (node.children.length === 0) {
    const base = slugify(node.title);
    let slug = base;
    if (used.has(slug)) slug = `${base}-${sectionId}`;
    let n = 2;
    while (used.has(slug)) slug = `${base}-${sectionId}-${n++}`;
    used.add(slug);
    expected.push({ id: `${sectionId}/${slug}`, slug });
    return;
  }
  if (isLeafAndGroup(node.title)) {
    const base = slugify(node.title);
    let slug = base;
    if (used.has(slug)) slug = `${base}-${sectionId}`;
    let n = 2;
    while (used.has(slug)) slug = `${base}-${sectionId}-${n++}`;
    used.add(slug);
    expected.push({ id: `${sectionId}/${slug}`, slug });
  }
  for (const child of node.children) collect(sectionId, child);
};
for (const section of sections) for (const root of section.roots) collect(section.id, root);

const manifest = JSON.parse(readFileSync(join(repo, "print/asset/viz-taxonomy.json"), "utf8")) as { id: string; slug: string; section: string; family: string }[];
const galleryDir = join(repo, "print/template/viz-gallery");
const covers = new Set<string>();
const demoMismatch: string[] = [];
for (const name of readdirSync(galleryDir).filter((file) => file.endsWith(".tex") && !file.includes("-dark"))) {
  const source = readFileSync(join(galleryDir, name), "utf8");
  const coverRe = /% viz-covers:\s+(\S+)\s*\n\\begin\{VizFigure\}[^\n]*\n\\SemioVizDemo\{([^}]+)\}/g;
  for (const match of source.matchAll(coverRe)) {
    covers.add(match[1]!);
    const [section, slug] = match[1]!.split("/");
    if (slug !== match[2]) demoMismatch.push(`${name}: cover ${match[1]} demo ${match[2]}`);
    void section;
  }
}
const expectedIds = expected.map((leaf) => leaf.id);
const missingManifest = expectedIds.filter((id) => !manifest.some((leaf) => leaf.id === id));
const extraManifest = manifest.map((leaf) => leaf.id).filter((id) => !expectedIds.includes(id));
const missingCovers = manifest.filter((leaf) => !covers.has(leaf.id)).map((leaf) => leaf.id);
const badPresent = BAD.filter((id) => manifest.some((leaf) => leaf.id === id));
const wantMissing = WANT.filter((id) => !manifest.some((leaf) => leaf.id === id));
const families = new Set(manifest.map((leaf) => leaf.family));
const slugCounts = new Map<string, number>();
for (const leaf of manifest) slugCounts.set(leaf.slug, (slugCounts.get(leaf.slug) ?? 0) + 1);
const dupSlugs = [...slugCounts.entries()].filter(([, n]) => n > 1);
const sty = readdirSync(join(repo, "print/tex")).filter((name) => /^semio-viz-chart-.+\.sty$/.test(name));
const lines = [
  "# Catalogue fidelity (post-P1 independent parse)",
  "",
  `- Independent leaves: ${expected.length}`,
  `- Manifest leaves: ${manifest.length}`,
  `- Sections: ${sections.length}`,
  `- Missing from manifest: ${missingManifest.length ? missingManifest.join(", ") : "none"}`,
  `- Extra in manifest: ${extraManifest.length ? extraManifest.join(", ") : "none"}`,
  `- Missing covers: ${missingCovers.length ? missingCovers.join(", ") : "none"}`,
  `- Cover/demo slug mismatches: ${demoMismatch.length ? demoMismatch.join("; ") : "none"}`,
  `- Duplicate global slugs: ${dupSlugs.length}`,
  `- Forbidden kinds present: ${badPresent.length ? badPresent.join(", ") : "none"}`,
  `- Required kinds missing: ${wantMissing.length ? wantMissing.join(", ") : "none"}`,
  `- Families: ${[...families].sort().join(", ")}`,
  `- Chart packages on disk: ${sty.length}`,
  "",
  expected.length === 1966 && manifest.length === 1966 && missingManifest.length === 0 && extraManifest.length === 0 && missingCovers.length === 0 && demoMismatch.length === 0 && dupSlugs.length === 0 && badPresent.length === 0 && wantMissing.length === 0 && sty.length === 79
    ? "**Verdict: PASS**"
    : "**Verdict: FAIL**",
  "",
];
writeFileSync(join(ticket, "review-catalogue.md"), lines.join("\n"));
console.log(lines.join("\n"));
