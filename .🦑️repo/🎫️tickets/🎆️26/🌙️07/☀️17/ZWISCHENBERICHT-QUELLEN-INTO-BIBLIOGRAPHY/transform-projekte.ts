#!/usr/bin/env bun
// Temporary one-off: (1) add a Quelle column + \cite to each project's Allgemein
// table, in file order (proj:01..proj:67); (2) drop the old P.Q Quellen section.
import { readFileSync, writeFileSync } from "node:fs";

const path = "/Users/ueli/Documents/semio/mit-bestand/bericht/zwischenbericht/anhang/projekte.tex";
const projToKeys: Record<string, string[]> = JSON.parse(readFileSync(`${import.meta.dir}/proj-to-keys.json`, "utf8"));

let src = readFileSync(path, "utf8");

// 1) drop the P.Q Quellen section entirely (from \appendixsection[Q]{Quellen} to end of file's SemioTableLong block, before EOF)
const qStart = src.indexOf("\\appendixsection[Q]{Quellen}");
if (qStart === -1) throw new Error("P.Q section not found");
src = src.slice(0, qStart).replace(/\n+$/, "\n");

// 2) walk Allgemein tables in order, zipping with proj:01..proj:67
const headerRe = /\\begin\{SemioTable\}\{llll\}\n(\s*)\\SemioTableHeaderRow\{Ort & Jahr & Typ & Status\}\n(\s*)\\SemioTableRow\{([^}]*)\}\n(\s*)\\end\{SemioTable\}/g;

let projIndex = 0;
const projNums = Array.from({ length: 67 }, (_, i) => String(i + 1).padStart(2, "0"));

src = src.replace(headerRe, (match, indent1, indent2, rowContent, indent3) => {
  const projNum = projNums[projIndex];
  projIndex++;
  const keys = projToKeys[projNum];
  if (!keys || keys.length === 0) throw new Error(`no source keys for proj:${projNum}`);
  const cite = `\\cite{${keys.join(",")}}`;
  return `\\begin{SemioTable}{lllll}\n${indent1}\\SemioTableHeaderRow{Ort & Jahr & Typ & Status & Quelle}\n${indent2}\\SemioTableRow{${rowContent} & ${cite}}\n${indent3}\\end{SemioTable}`;
});

if (projIndex !== 67) throw new Error(`expected 67 Allgemein tables, patched ${projIndex}`);

writeFileSync(path, src);
console.log(`patched ${projIndex} Allgemein tables, dropped P.Q section`);
