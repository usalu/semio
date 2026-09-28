/** 🔬️ R10 probe: runs the orchestration module's docstring census read-only and prints each oracle disagreement with the line. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { runDocstringCensus } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const started = Date.now();
const census = runDocstringCensus(root, new AbortController().signal, (line) => console.error(line));
const at = census.hits.filter((hit) => hit.rule === "at-emoji").length;
console.log(`files=${census.files} at-emoji=${at} no-emoji=${census.hits.length - at} disagreements=${census.disagreements.length} seconds=${Math.round((Date.now() - started) / 1000)}`);
const scanned = new Set(census.hits.filter((hit) => hit.rule === "at-emoji").map((hit) => `${hit.path}:${hit.line}`));
for (const key of census.disagreements.slice(Number(process.argv[3] ?? 0), Number(process.argv[2] ?? 40))) {
  const cut = key.lastIndexOf(":");
  const path = key.slice(0, cut);
  const line = Number(key.slice(cut + 1));
  const lines = readFileSync(join(root, path), "utf8").split("\n");
  console.log(`${scanned.has(key) ? "SCANNER-ONLY" : "ORACLE-ONLY"} ${key}`);
  for (let index = Math.max(0, line - 3); index < Math.min(lines.length, line + 1); index += 1) console.log(`  ${index + 1}${index + 1 === line ? ">" : " "} ${lines[index]!.slice(0, 140)}`);
}
