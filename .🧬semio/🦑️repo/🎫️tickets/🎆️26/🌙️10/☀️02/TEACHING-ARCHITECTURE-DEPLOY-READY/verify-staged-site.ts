#!/usr/bin/env repo
/** 🔎 Checks the staged release site with the same predicate publish uses. */
import { createHash } from "node:crypto";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { gzipSync } from "node:zlib";
import { COURSE_PROCTOR_ORIGIN, COURSE_SITE_BUDGET, courseSiteTopology } from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🟦️.ts";
import { siteArtifactProblems } from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🌐️site/🟦️.ts";

const dist = join(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/dist/release/pages/course");

function files(root: string, prefix = ""): string[] {
  return readdirSync(join(root, prefix), { withFileTypes: true }).flatMap((entry) => {
    const path = prefix ? `${prefix}/${entry.name}` : entry.name;
    if (entry.isDirectory()) return files(root, path);
    return entry.isFile() ? [path] : [];
  });
}

const listed = files(dist).sort();
const total = listed.reduce((sum, path) => sum + statSync(join(dist, path)).size, 0);
const html = readFileSync(join(dist, "index.html"), "utf8");
const tags = [...html.matchAll(/<(?:script|link)\b[^>]*\s(?:src|href)="\/(assets\/[^"]+)"[^>]*>/gu)].map((tag) => ({ path: tag[1]!, idle: /\sfetchpriority="low"/u.test(tag[0]) }));
const linked = tags.filter(({ idle }) => !idle).map(({ path }) => path);
const scriptGzip = linked.filter((path) => path.endsWith(".js")).reduce((sum, path) => sum + gzipSync(readFileSync(join(dist, path))).length, 0);
const styleGzip = linked.filter((path) => path.endsWith(".css")).reduce((sum, path) => sum + gzipSync(readFileSync(join(dist, path))).length, 0);
const code = listed.filter((path) => path.endsWith(".js")).map((path) => readFileSync(join(dist, path), "utf8")).join("\n");
const problems = siteArtifactProblems(dist, COURSE_PROCTOR_ORIGIN);
const report = {
  dist,
  problems,
  fileCount: listed.length,
  totalBytes: total,
  budgetTotalBytes: COURSE_SITE_BUDGET.totalBytes,
  topology: courseSiteTopology(COURSE_PROCTOR_ORIGIN),
  scriptGzipBytes: scriptGzip,
  scriptBudget: COURSE_SITE_BUDGET.scriptGzipBytes.publish,
  styleGzipBytes: styleGzip,
  styleBudget: COURSE_SITE_BUDGET.styleGzipBytes,
  proctorOrigin: COURSE_PROCTOR_ORIGIN,
  bakesProctor: code.includes(JSON.stringify(COURSE_PROCTOR_ORIGIN)),
  hasCourseCardLink: code.includes("course-card-link"),
  hasPlainClickGuard: code.includes("metaKey") && code.includes("preventDefault"),
  indexEquals404: readFileSync(join(dist, "index.html")).equals(readFileSync(join(dist, "404.html"))),
  cname: readFileSync(join(dist, "CNAME"), "utf8").trim(),
  hasHeaders: listed.includes("_headers"),
  sha256: createHash("sha256").update(listed.join("\n")).digest("hex"),
};
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
