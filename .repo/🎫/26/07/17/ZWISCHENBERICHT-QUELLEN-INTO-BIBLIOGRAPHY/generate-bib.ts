#!/usr/bin/env bun
// Temporary one-off: turns rows.json into references.bib text + a proj:NN -> cite-key(s) map.
import { readFileSync, writeFileSync } from "node:fs";

type Row = { id: string; herausgeber: string; title: string; url: string; projs: string[] };
const rows: Row[] = JSON.parse(readFileSync(`${import.meta.dir}/rows.json`, "utf8"));

function deTeX(s: string): string {
  return s
    .replace(/\\&/g, "&")
    .replace(/\\_/g, "_")
    .replace(/\\textperiodcentered\\?/g, "·")
    .replace(/\\\\/g, "");
}

function slugify(s: string, maxLen = 60): string {
  const ligatures: Record<string, string> = { Æ: "AE", æ: "ae", Œ: "OE", œ: "oe", ß: "ss", Ø: "O", ø: "o" };
  const delig = [...deTeX(s)].map((ch) => ligatures[ch] ?? ch).join("");
  let slug = delig
    .toLowerCase()
    .normalize("NFKD")
    .replace(/[̀-ͯ]/g, "")
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
  if (slug.length > maxLen) {
    slug = slug.slice(0, maxLen).replace(/-[^-]*$/, "");
  }
  return slug;
}

const MANUAL_KEYS: Record<string, string> = {
  S071: "sciencedirect-structural-component-reuse-case-studies",
};

const usedKeys = new Set<string>();
function keyFor(row: Row): string {
  if (MANUAL_KEYS[row.id]) {
    usedKeys.add(MANUAL_KEYS[row.id]);
    return MANUAL_KEYS[row.id];
  }
  // manual short project slug per row (2nd half of title after the em-dash, else title)
  const parts = deTeX(row.title).split("—");
  const projectPart = parts.length > 1 ? parts.slice(1).join("—").trim() : parts[0].trim();
  let base = `${slugify(row.herausgeber, 30)}-${slugify(projectPart, 40)}`;
  base = base.replace(/-+/g, "-");
  let key = base;
  let n = 2;
  while (usedKeys.has(key)) {
    key = `${base}-${n}`;
    n++;
  }
  usedKeys.add(key);
  return key;
}

// bib field values are compiled as LaTeX text (unlike slugs) — keep existing TeX escaping (\&, \_) intact
function escapeBibField(s: string): string {
  return s;
}

// rows whose herausgeber is just a duplicate of the title (no real author/publisher known)
const NO_AUTHOR_IDS = new Set(["S071"]);

const bibEntries: string[] = [];
const projToKeys = new Map<string, string[]>();

for (const row of rows) {
  const key = keyFor(row);
  const title = escapeBibField(row.title);
  const authorField = NO_AUTHOR_IDS.has(row.id) ? "" : `  author = {{${escapeBibField(row.herausgeber)}}},\n`;
  bibEntries.push(`@online{${key},\n${authorField}  title = {${title}},\n  url = {${row.url}},\n}`);
  for (const proj of row.projs) {
    const arr = projToKeys.get(proj) ?? [];
    arr.push(key);
    projToKeys.set(proj, arr);
  }
}

const bibHeader = "% References for the mit-bestand Zwischenbericht, migrated from the former P.Q Quellen table.\n\n";
writeFileSync("/Users/ueli/Documents/semio/mit-bestand/bericht/zwischenbericht/references.bib", bibHeader + bibEntries.join("\n\n") + "\n");

const projMap = Object.fromEntries([...projToKeys.entries()].sort((a, b) => a[0].localeCompare(b[0])));
writeFileSync(`${import.meta.dir}/proj-to-keys.json`, JSON.stringify(projMap, null, 2));

console.log(`wrote ${bibEntries.length} bib entries, ${projToKeys.size} projects mapped`);
