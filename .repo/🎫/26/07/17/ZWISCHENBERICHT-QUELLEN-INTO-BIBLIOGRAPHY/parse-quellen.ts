#!/usr/bin/env bun
// Temporary one-off parser: turns the P.Q Quellen table rows into bib entries
// + a proj:NN -> cite-key(s) mapping. Ticket-scoped, not a permanent script.
import { readFileSync, writeFileSync } from "node:fs";

const projektePath = "/Users/ueli/Documents/semio/mit-bestand/bericht/zwischenbericht/anhang/projekte.tex";
const src = readFileSync(projektePath, "utf8");
const lines = src.split("\n");

const rowRe = /^\s*\\SemioTableRow\{(S\d+) & (.+?) & (.+?)\}$/;

type Row = { id: string; herausgeber: string; title: string; url: string; projs: string[] };
const rows: Row[] = [];

for (let i = 1784; i <= 1845; i++) {
  const line = lines[i];
  const m = line.match(rowRe);
  if (!m) {
    console.error(`NOMATCH line ${i + 1}: ${line}`);
    continue;
  }
  const [, id, mid, refs] = m;
  const midMatch = mid.match(/^(.+?) \\textperiodcentered\\ (.+?)\\\\\{\\footnotesize\\url\{(.+?)\}\}$/);
  if (!midMatch) {
    console.error(`MID-NOMATCH ${id}: ${mid}`);
    continue;
  }
  const [, herausgeber, title, url] = midMatch;
  const projs = [...refs.matchAll(/\\ref\{proj:(\d+)\}/g)].map((mm) => mm[1]);
  rows.push({ id, herausgeber, title, url, projs });
}

console.log(`parsed ${rows.length} rows`);
writeFileSync(`${import.meta.dir}/rows.json`, JSON.stringify(rows, null, 2));
