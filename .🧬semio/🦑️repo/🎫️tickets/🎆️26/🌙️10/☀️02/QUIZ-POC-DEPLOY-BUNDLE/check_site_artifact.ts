/** 🔎️ Checks a built site directory as `publish` would and prints what its document's scripts and stylesheets weigh:
 * `bun check_site_artifact.ts <directory> [proctor origin]`. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { gzipSync } from "node:zlib";
import { QUIZ_PROCTOR_ORIGIN, QUIZ_SITE_BUDGET, QUIZ_SITE_ORIGIN, siteArtifactProblems } from "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/🟦️.ts";

const directory = resolve(process.argv[2] ?? ".");
const origin = process.argv[3] ?? QUIZ_PROCTOR_ORIGIN;
const html = readFileSync(join(directory, "index.html"), "utf8");
const linked = [...html.matchAll(/<(?:script|link)\b[^>]*\s(?:src|href)="\/(assets\/[^"]+)"/gu)].map((tag) => tag[1]!);
for (const path of linked) console.log(`${path}: ${readFileSync(join(directory, path)).length} bytes, ${gzipSync(readFileSync(join(directory, path))).length} gzip`);
console.log(`site ${QUIZ_SITE_ORIGIN}, proctor ${origin}, budget ${JSON.stringify(QUIZ_SITE_BUDGET)}`);
const problems = siteArtifactProblems(directory, origin);
console.log(problems.length === 0 ? "no problems" : problems.map((problem) => `- ${problem}`).join("\n"));
process.exit(problems.length === 0 ? 0 : 1);
