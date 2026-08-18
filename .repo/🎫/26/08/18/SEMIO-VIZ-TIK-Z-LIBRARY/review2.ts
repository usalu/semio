#!/usr/bin/env bun
/** 🔎 Phase-4 review pass two: mark/chart slug shadowing, junk leaves, chrome kind options. */
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const ticket = import.meta.dir;
const repo = join(ticket, "../../../../../../");
const printRoot = join(repo, "print");
const texDir = join(printRoot, "tex");
const galleryDir = join(printRoot, "template/viz-gallery");

const manifest = JSON.parse(readFileSync(join(printRoot, "asset/viz-taxonomy.json"), "utf8")) as {
  id: string; slug: string; title: string; kind: string; family: string; section: string;
}[];

const markSty = readFileSync(join(texDir, "semio-viz-mark.sty"), "utf8");
const markBlock = markSty.match(/\\clist_map_inline:nn \{([\s\S]*?)\} \{ \\semio_viz_mark_kind_define:n \{#1\} \}/);
const markKinds = new Set((markBlock?.[1] ?? "").split(",").map((s) => s.trim()).filter(Boolean));

const out: string[] = [];
const say = (line = ""): void => { out.push(line); console.log(line); };

say(`mark kinds registered: ${markKinds.size}`);
const section0 = manifest.filter((leaf) => leaf.section === "0");
say(`section 0 leaves: ${section0.length}`);
say(`section 0 slugs missing from mark registry: ${section0.filter((leaf) => !markKinds.has(leaf.slug)).map((leaf) => leaf.slug).join(", ") || "none"}`);
say(`mark kinds not in section 0 (aliases): ${[...markKinds].filter((kind) => !section0.some((leaf) => leaf.slug === kind)).join(", ")}`);
say();

// chart kinds registered by loaded packages
const loader = readFileSync(join(texDir, "semio-viz-charts.sty"), "utf8");
const required = [...loader.matchAll(/\\RequirePackage\{semio-viz-chart-([^}]+)\}/g)].map((m) => m[1]!);
const registered = new Map<string, { family: string; opts: string; pkg: string }>();
for (const name of required) {
  const tex = readFileSync(join(texDir, `semio-viz-chart-${name}.sty`), "utf8");
  for (const match of tex.matchAll(/\\SemioVizChartKind\{([^}]*)\}\{([^}]*)\}\{([^}]*)\}/g)) {
    registered.set(match[1]!, { family: match[2]!, opts: match[3]!, pkg: name });
  }
}
say(`chart kinds registered: ${registered.size}`);
const shadowed = [...registered.keys()].filter((slug) => markKinds.has(slug));
say(`## chart kinds SHADOWED by a mark kind in \\SemioVizDemo dispatch (${shadowed.length})`);
for (const slug of shadowed) {
  const leaf = manifest.find((item) => item.slug === slug);
  say(`- ${slug}: taxonomy ${leaf?.id ?? "-"} "${leaf?.title ?? "-"}" kind=${leaf?.kind} family=${leaf?.family} registered-family=${registered.get(slug)!.family} pkg=${registered.get(slug)!.pkg}`);
}
say();

// registered kinds not in the manifest (aliases)
const manifestSlugs = new Set(manifest.map((leaf) => leaf.slug));
say(`## registered chart kinds absent from taxonomy: ${[...registered.keys()].filter((slug) => !manifestSlugs.has(slug)).join(", ") || "none"}`);
say();

// junk / prose leaves
const junk = manifest.filter((leaf) => /[:,.]$/.test(leaf.title) || leaf.title.split(/\s+/).length > 7);
say(`## prose-looking leaves (trailing punctuation or >7 words) (${junk.length})`);
for (const leaf of junk) say(`- ${leaf.id} "${leaf.title}" kind=${leaf.kind} family=${leaf.family}`);
say();

// section 76 namespaces
const s76 = manifest.filter((leaf) => leaf.section === "76");
say(`section 76 leaves (${s76.length}): ${s76.map((leaf) => leaf.slug).join(" ")}`);
say(`section 76 contains a leaf for the 'charts' namespace: ${s76.some((leaf) => leaf.title === "charts")}`);
say();

// collision-suffixed slugs
const suffixed = manifest.filter((leaf) => {
  const natural = leaf.title.normalize("NFKD").replace(/\p{M}/gu, "").replace(/['’]/g, "").replace(/[–—]/g, "-")
    .replace(/[%]/g, "percent").replace(/&/g, "and").replace(/\+/g, "plus").toLowerCase()
    .replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "");
  return leaf.slug !== natural;
});
say(`## collision-suffixed slugs (${suffixed.length})`);
const bad = suffixed.filter((leaf) => {
  const natural = leaf.title.normalize("NFKD").replace(/\p{M}/gu, "").replace(/['’]/g, "").replace(/[–—]/g, "-")
    .replace(/[%]/g, "percent").replace(/&/g, "and").replace(/\+/g, "plus").toLowerCase()
    .replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "");
  return leaf.slug !== `${natural}-${leaf.section}` && !new RegExp(`^${natural}-${leaf.section}-\\d+$`).test(leaf.slug);
});
say(`   not matching -{section}[-n] rule (${bad.length}): ${bad.map((leaf) => `${leaf.id}<-${leaf.title}`).join(", ") || "none"}`);
const firstOccurrenceWrong: string[] = [];
{
  const seen = new Set<string>();
  for (const leaf of manifest) {
    const natural = leaf.title.normalize("NFKD").replace(/\p{M}/gu, "").replace(/['’]/g, "").replace(/[–—]/g, "-")
      .replace(/[%]/g, "percent").replace(/&/g, "and").replace(/\+/g, "plus").toLowerCase()
      .replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "");
    if (!seen.has(natural)) {
      seen.add(natural);
      if (leaf.slug !== natural) firstOccurrenceWrong.push(`${leaf.id} natural=${natural}`);
    }
  }
}
say(`   first occurrences that did NOT keep the natural slug (${firstOccurrenceWrong.length}): ${firstOccurrenceWrong.slice(0, 10).join(", ") || "none"}`);
say();

// chrome-family option coverage
const chromeKinds = [...registered.entries()].filter(([, entry]) => entry.family === "chrome");
const chromeNoOpts = chromeKinds.filter(([, entry]) => entry.opts.trim() === "data=demo");
say(`chrome-family kinds: ${chromeKinds.length}; with only data=demo (no scale/orient/legend): ${chromeNoOpts.length}`);
const axisKinds = manifest.filter((leaf) => leaf.kind === "axis");
say(`axis-kind leaves: ${axisKinds.length}`);
const axisNoScale = axisKinds.filter((leaf) => (registered.get(leaf.slug)?.opts ?? "").indexOf("scale=") === -1);
say(`## axis-kind leaves registered without a scale= option (${axisNoScale.length}): ${axisNoScale.map((leaf) => leaf.id).join(", ") || "none"}`);
const scaleKinds = manifest.filter((leaf) => leaf.kind === "scale");
const scaleNoOpts = scaleKinds.filter((leaf) => (registered.get(leaf.slug)?.opts ?? "").trim() === "data=demo");
say(`scale-kind leaves: ${scaleKinds.length}; registered with bare data=demo: ${scaleNoOpts.length}`);
say();

// section 51 semantics
const s51 = manifest.filter((leaf) => leaf.section === "51");
say(`## section 51 kinds`);
for (const leaf of s51) say(`- ${leaf.slug} kind=${leaf.kind} family=${leaf.family} opts="${registered.get(leaf.slug)?.opts ?? "-"}"`);
say();

// section 78 subgroup kinds
const genMd = readFileSync(join(printRoot, "asset/viz-taxonomy.md"), "utf8");
const s78Block = genMd.split(/^## /m).find((block) => block.startsWith("78."));
say(`## section 78 groups + kinds`);
for (const line of (s78Block ?? "").split(/\n/)) {
  if (line.startsWith("### ")) say(`  ${line}`);
  else if (line.startsWith("- ")) say(`    ${line}`);
}
say();

// viz-api vs galleries
const galleryFiles = readdirSync(galleryDir).filter((name) => /^viz-\d+\.tex$/.test(name));
const generatedUses = new Set<string>();
for (const name of galleryFiles) {
  const tex = readFileSync(join(galleryDir, name), "utf8");
  for (const match of tex.matchAll(/\\(Semio[A-Za-z]+)/g)) generatedUses.add(match[1]!);
}
say(`commands used by generated galleries: ${[...generatedUses].sort().join(" ")}`);

Bun.write(join(ticket, "review-output-2.txt"), `${out.join("\n")}\n`);
