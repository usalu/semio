import { readFileSync } from "node:fs";
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts";
const law = await import(L);
const WS = "[ \\t\\n\\r\\f\\v\\u00a0]";
const LITERAL = "(\"(?:[^\"\\\\\\n]|\\\\.)*\"|'(?:[^'\\\\\\n]|\\\\.)*'|`(?:[^`\\\\]|\\\\.)*`)";
const SITES = [
  new RegExp(`(?:new${WS}+|throw${WS}+)?\\bError\\(${WS}*${LITERAL}`, "gu"),
  new RegExp(`\\bconsole\\.(?:log|error|warn|info|debug|table|group)\\(${WS}*${LITERAL}`, "gu"),
  new RegExp(`\\b(?:stdout|stderr)\\.write\\(${WS}*${LITERAL}`, "gu"),
];
const file = process.argv[2]!;
const text = readFileSync(file, "utf8");
text.split("\n").forEach((line, i) => {
  if (/^\s*(?:\/\/|\/\*|\*)/.test(line)) return;
  for (const site of SITES) for (const m of line.matchAll(site)) {
    const lit = m[1]!;
    if (lit.startsWith("`") && lit.includes("\n")) continue;
    if (law.isProseV1(lit.slice(1, -1).replace(/\$\{[^}]*\}/gu, "XX"))) console.log(`${i + 1}@${m.index}: ${line.slice(m.index!, m.index! + Number(process.argv[3] ?? 150))}`);
  }
});
