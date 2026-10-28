/** 🔬️ Prints every English prose site (Error/console/stdout.write literal) per file with its line: bun prose-sites.ts <list> */
import { readFileSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
const L = `${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts`;
const language = await import(L);
const WS = "[ \\t\\n\\r\\f\\v\\u00a0]";
const LITERAL = "(\"(?:[^\"\\\\\\n]|\\\\.)*\"|'(?:[^'\\\\\\n]|\\\\.)*'|`(?:[^`\\\\]|\\\\.)*`)";
const SITES = [new RegExp(`(?:new${WS}+|throw${WS}+)?\\bError\\(${WS}*${LITERAL}`, "gu"), new RegExp(`\\bconsole\\.(?:log|error|warn|info|debug|table|group)\\(${WS}*${LITERAL}`, "gu"), new RegExp(`\\b(?:stdout|stderr)\\.write\\(${WS}*${LITERAL}`, "gu")];
for (const p of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const text = readFileSync(`${ROOT}/${p}`, "utf8");
  const lines = text.split("\n");
  lines.forEach((line, i) => {
    if (/^\s*(\/\/|\/\*|\*)/u.test(line)) return;
    for (const site of SITES) for (const m of line.matchAll(site)) {
      const lit = m[1]!;
      if (lit.startsWith("`") && lit.includes("\n")) continue;
      if (language.isProseV1(lit.slice(1, -1).replace(/\$\{[^}]*\}/gu, "XX"))) console.log(`${p.slice(-70)}:${i + 1}: ${lit.slice(0, 130)}`);
    }
  });
}
