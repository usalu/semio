/** 📏️ Measures a release build of the quiz site against `QUIZ_SITE_BUDGET`: every script and stylesheet the document
 * links (what the budget counts) and every other chunk (lazy), raw and gzip, plus the budget problems.
 * Usage: `bun measure_site_budget.ts <dist>` */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { gzipSync } from "node:zlib";
import { QUIZ_SITE_BUDGET, siteArtifactProblems } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/🟦️.ts";

const dist = process.argv[2]!;
const html = readFileSync(join(dist, "index.html"), "utf8");
const linked = [...html.matchAll(/<(?:script|link)\b[^>]*\s(?:src|href)="\/(assets\/[^"]+)"/gu)].map((tag) => tag[1]!);
const assets = readdirSync(join(dist, "assets")).map((name) => `assets/${name}`).filter((path) => /\.(?:js|css)$/u.test(path));
const rows = assets.map((path) => {
  const bytes = readFileSync(join(dist, path));
  return { path, linked: linked.includes(path), raw: statSync(join(dist, path)).size, gzip: gzipSync(bytes).length };
});
for (const row of rows.sort((a, b) => Number(b.linked) - Number(a.linked) || b.gzip - a.gzip)) console.log(`${row.linked ? "LINKED" : "lazy  "} ${String(row.raw).padStart(9)} raw ${String(row.gzip).padStart(8)} gzip  ${row.path}`);
const sum = (extension: string, isLinked: boolean): number => rows.filter((row) => row.path.endsWith(extension) && row.linked === isLinked).reduce((total, row) => total + row.gzip, 0);
console.log(`linked scripts gzip ${sum(".js", true)} of ${QUIZ_SITE_BUDGET.scriptGzipBytes}; lazy scripts gzip ${sum(".js", false)}`);
console.log(`linked styles gzip ${sum(".css", true)} of ${QUIZ_SITE_BUDGET.styleGzipBytes}`);
console.log(`problems (any origin): ${JSON.stringify(siteArtifactProblems(dist, process.argv[3] ?? "http://127.0.0.1:8892"))}`);
