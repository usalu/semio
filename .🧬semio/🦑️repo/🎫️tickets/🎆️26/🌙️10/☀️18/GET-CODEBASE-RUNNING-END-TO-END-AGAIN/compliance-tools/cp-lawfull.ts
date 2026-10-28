/** 🗣️ The language law over the whole law roots of a root (live or mirror), with the allowlist of that root: CP_ROOT=<root> bun cp-lawfull.ts <out json>. */
import { readFileSync, writeFileSync } from "node:fs";
const LIVE = "/Users/ueli/Documents/semio";
const ROOT = process.env.CP_ROOT ?? LIVE;
const language = await import(`${LIVE}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts`);
const allowlist = JSON.parse(readFileSync(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🔒️allowlist/🔣️.json`, "utf8"));
const sources = language.readLanguageLawSourcesV1(ROOT);
const findings = language.languageLawFindingsV1(sources, allowlist);
writeFileSync(process.argv[2]!, JSON.stringify({ sources: sources.length, findings }, null, 1));
console.error("[DEBUG] sources", sources.length, "findings", findings.length, "rss MB", Math.round(process.memoryUsage().rss / 1048576));
