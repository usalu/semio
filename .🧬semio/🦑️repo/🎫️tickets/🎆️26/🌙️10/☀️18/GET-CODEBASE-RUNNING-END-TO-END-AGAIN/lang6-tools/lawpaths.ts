/** 🧮️ The language-law findings of one root restricted to the paths of a staged manifest: CP_ROOT=<root> bun lawpaths.ts <manifest.json>; exit 1 when any remains. */
import { readFileSync } from "node:fs";
const LIVE = "/Users/ueli/Documents/semio", ROOT = process.env.CP_ROOT ?? LIVE;
const language = await import(`${LIVE}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts`);
const allowlist = JSON.parse(readFileSync(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🔒️allowlist/🔣️.json`, "utf8"));
const paths = new Set(Object.keys(JSON.parse(readFileSync(process.argv[2]!, "utf8"))));
const findings = language.languageLawFindingsV1(language.readLanguageLawSourcesV1(ROOT), allowlist).filter((f: { path: string }) => paths.has(f.path));
for (const f of findings) console.log(f.rule, f.detail, f.path.slice(-70));
console.log(`[DEBUG] batch-path findings ${findings.length}`);
process.exit(findings.length ? 1 : 0);
