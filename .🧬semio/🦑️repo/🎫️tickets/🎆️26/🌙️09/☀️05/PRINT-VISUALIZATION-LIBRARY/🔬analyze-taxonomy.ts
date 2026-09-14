#!/usr/bin/env bun
/** 🔬 Ticket-local analysis of the print viz taxonomy: leaf counts, slug collisions, suffixes. */
import { readFileSync } from "node:fs";

const md = readFileSync(process.argv[2]!, "utf8");
type Leaf = { section: string; sectionTitle: string; group: string; slug: string; title: string; kind: string };
const leaves: Leaf[] = [];
let section = "", sectionTitle = "", group = "";
for (const line of md.split(/\n/)) {
  const h2 = line.match(/^##\s+(\d+)\.\s+(.*)$/);
  if (h2) { section = h2[1]!; sectionTitle = h2[2]!; group = ""; continue; }
  const h3 = line.match(/^###\s+(.*)$/);
  if (h3) { group = h3[1]!; continue; }
  const m = line.match(/^- (.+) `([^`]+)` (mark|chart|layout|axis|scale)$/);
  if (m && section) leaves.push({ section, sectionTitle, group, slug: m[2]!, title: m[1]!, kind: m[3]! });
}
console.log("total leaves", leaves.length);
const bySection = new Map<string, number>();
for (const l of leaves) bySection.set(l.section, (bySection.get(l.section) ?? 0) + 1);
console.log("sections", bySection.size);
console.log([...bySection].map(([s, n]) => `${s}:${n}`).join(" "));
const bySlug = new Map<string, Leaf[]>();
for (const l of leaves) { const a = bySlug.get(l.slug) ?? []; a.push(l); bySlug.set(l.slug, a); }
const dupes = [...bySlug].filter(([, a]) => a.length > 1);
console.log("distinct slugs", bySlug.size, "colliding slugs", dupes.length, "extra entries", leaves.length - bySlug.size);
console.log("suffixed slugs", leaves.filter((l) => /-\d+$/.test(l.slug)).length);
console.log("kinds", JSON.stringify([...new Set(leaves.map((l) => l.kind))]));
console.log("--- top collisions ---");
for (const [s, a] of dupes.sort((x, y) => y[1].length - x[1].length).slice(0, 25)) console.log(s, a.map((l) => l.section).join(","), "|", a.map((l) => l.title)[0]);
console.log("--- suffixed sample ---");
console.log(leaves.filter((l) => /-\d+$/.test(l.slug)).slice(0, 20).map((l) => `${l.section}/${l.slug} = ${l.title}`).join("\n"));
