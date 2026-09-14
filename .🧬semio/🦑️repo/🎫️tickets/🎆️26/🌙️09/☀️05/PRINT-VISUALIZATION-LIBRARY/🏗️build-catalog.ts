#!/usr/bin/env bun
/** 🏗️ Ticket-local: builds the first version of `🖼️assets/🔣️viz-catalog.json` and the derived
 * `🖼️assets/🔣️viz-taxonomy.json` from the handcrafted taxonomy `🖼️assets/📊️viz-taxonomy.md`. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { FAMILY_DIMENSIONS, PRIMARY_SECTION, SECTIONS } from "./🏗️catalog-families.ts";
import { toGerman } from "./🏗️catalog-german.ts";

const PRODUCT = process.argv[2] ?? "C:/git/semio/🧰️framework/🛍️products/📓️print";

type Leaf = { readonly section: string; readonly group: string; readonly slug: string; readonly title: string; readonly kind: string };

function parse(md: string): Leaf[] {
  const leaves: Leaf[] = [];
  let section = "";
  let group = "General";
  for (const line of md.split(/\n/)) {
    const h2 = line.match(/^##\s+(\d+)\./);
    if (h2) { section = h2[1]!; group = "General"; continue; }
    const h3 = line.match(/^###\s+(.*?)\s*$/);
    if (h3) { group = h3[1]!; continue; }
    const leaf = line.match(/^- (.+) `([^`]+)` (mark|chart|layout|axis|scale)$/);
    if (leaf && section) leaves.push({ section, group, slug: leaf[2]!, title: leaf[1]!, kind: leaf[3]! });
  }
  return leaves;
}

const leaves = parse(readFileSync(join(PRODUCT, "🖼️assets/📊️viz-taxonomy.md"), "utf8"));

const bySlug = new Map<string, Leaf[]>();
for (const leaf of leaves) {
  const bucket = bySlug.get(leaf.slug) ?? [];
  bucket.push(leaf);
  bySlug.set(leaf.slug, bucket);
}

function spec(leaf: Leaf) {
  const section = SECTIONS[leaf.section];
  if (!section) throw new Error(`no section spec for ${leaf.section}`);
  const group = section.groups?.[leaf.group];
  return {
    owner: group?.owner ?? section.owner,
    namespace: group?.namespace ?? section.namespace,
    data: group?.data ?? section.data,
    family: group?.family ?? section.family,
  };
}

/** 🌊 Kinds whose renderer belongs to the spatial kernel rather than the geometry gallery. */
const SPATIAL_GEOMETRY = new Set(["voronoi-diagram", "delaunay-triangulation", "convex-hull", "concave-hull", "triangulation", "mesh", "tessellation"]);

const entries = [...bySlug].map(([slug, group]) => {
  const wanted = PRIMARY_SECTION[slug];
  const primary = (wanted && group.find((leaf) => leaf.section === wanted)) ?? group.slice().sort((x, y) => Number(x.section) - Number(y.section))[0]!;
  const resolved = spec(primary);
  const family = SPATIAL_GEOMETRY.has(slug) ? "spatial" : resolved.family;
  const namespace = SPATIAL_GEOMETRY.has(slug) ? "spatial" : resolved.namespace;
  const options: Record<string, string | number | boolean> = { variant: slug };
  for (const [key, dimension] of Object.entries(FAMILY_DIMENSIONS[family] ?? {})) options[key] = dimension.default;
  return {
    id: `${primary.section}/${slug}`,
    slug,
    title: { en: primary.title, de: toGerman(primary.title) },
    kind: primary.kind,
    namespace,
    family,
    options,
    data: resolved.data,
    covers: group.map((leaf) => `${leaf.section}/${leaf.slug}`).sort((x, y) => Number(x.split("/")[0]) - Number(y.split("/")[0])),
  };
});

entries.sort((x, y) => Number(x.id.split("/")[0]) - Number(y.id.split("/")[0]) || x.slug.localeCompare(y.slug));

writeFileSync(join(PRODUCT, "🖼️assets/🔣️viz-catalog.json"), `${JSON.stringify({ schemaVersion: 1, kinds: entries }, null, 2)}\n`, "utf8");

const kindBySlug = new Map(entries.map((entry) => [entry.slug, entry] as const));
const taxonomy = leaves.map((leaf) => {
  const kind = kindBySlug.get(leaf.slug)!;
  return { id: `${leaf.section}/${leaf.slug}`, slug: leaf.slug, title: leaf.title, kind: leaf.kind, family: kind.family, section: leaf.section };
});
writeFileSync(join(PRODUCT, "🖼️assets/🔣️viz-taxonomy.json"), `${JSON.stringify(taxonomy, null, 2)}\n`, "utf8");

const families = new Map<string, { owner: string; kinds: string[] }>();
for (const entry of entries) {
  const primarySection = entry.id.split("/")[0]!;
  const owner = spec(bySlug.get(entry.slug)!.find((leaf) => leaf.section === primarySection)!).owner;
  const bucket = families.get(entry.family) ?? { owner, kinds: [] };
  bucket.kinds.push(entry.slug);
  families.set(entry.family, bucket);
}
console.log(`[DEBUG] catalog: ${entries.length} kinds covering ${leaves.length} leaves in ${families.size} families`);
console.log(`[DEBUG] families: ${[...families].sort((x, y) => y[1].kinds.length - x[1].kinds.length).map(([name, value]) => `${name}(${value.owner}):${value.kinds.length}`).join(" ")}`);
const duplicate = new Map<string, string[]>();
for (const entry of entries) {
  const key = `${entry.family}|${JSON.stringify(entry.options)}`;
  const bucket = duplicate.get(key) ?? [];
  bucket.push(entry.slug);
  duplicate.set(key, bucket);
}
console.log(`[DEBUG] family+options collisions: ${[...duplicate.values()].filter((bucket) => bucket.length > 1).length}`);
