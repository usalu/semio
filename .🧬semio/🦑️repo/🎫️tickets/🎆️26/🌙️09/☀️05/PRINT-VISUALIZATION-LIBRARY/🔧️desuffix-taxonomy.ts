#!/usr/bin/env bun
/** 🔧 Ticket-local one-shot: strips section suffixes from taxonomy leaf slugs and applies the handcrafted disambiguations. */
import { readFileSync, writeFileSync } from "node:fs";

const path = process.argv[2]!;
const handcrafted: Record<string, string> = {
  "74/width-74": "line-width",
  "74/rotation-74-2": "text-rotation",
  "76/matrix": "network-matrix",
  "78/quantile-78": "quantile-transform",
  "79/text": "text-mark",
};

const lines = readFileSync(path, "utf8").split(/\n/);
let section = "";
let changed = 0;
const out = lines.map((line) => {
  const h2 = line.match(/^##\s+(\d+)\./);
  if (h2) { section = h2[1]!; return line; }
  const m = line.match(/^(- .+ `)([^`]+)(` (?:mark|chart|layout|axis|scale))$/);
  if (!m || !section) return line;
  const slug = m[2]!;
  const next = handcrafted[`${section}/${slug}`] ?? slug.replace(/-\d+(-\d+)?$/, "");
  if (next === slug) return line;
  changed += 1;
  return `${m[1]}${next}${m[3]}`;
});
writeFileSync(path, out.join("\n"), "utf8");
console.log(`[DEBUG] desuffix: rewrote ${changed} leaf slugs`);
