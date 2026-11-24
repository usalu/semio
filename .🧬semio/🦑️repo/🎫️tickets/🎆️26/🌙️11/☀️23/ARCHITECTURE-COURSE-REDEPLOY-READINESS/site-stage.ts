/** 🚚️ Verifies a directly built release site with the publish verb's own artifact check and stages the CDN directory beside it.
 * Usage: bun site-stage.ts [proctor origin] */
import { join, resolve } from "node:path";
import { releaseProctorOrigin, siteArtifactProblems, stageCourseSiteArtifact } from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🌐️site/🟦️.ts";

const bundle = resolve(import.meta.dirname, "../../../../../../../🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript");
const origin = process.argv[2] ?? releaseProctorOrigin();
const dist = join(bundle, "dist", "release");
const problems = siteArtifactProblems(dist, origin);
console.log(problems.length ? `problems:\n- ${problems.join("\n- ")}` : `no problems for ${origin}`);
if (!problems.length) stageCourseSiteArtifact(dist, join(bundle, "dist", "pages", "course"), origin);
process.exit(problems.length ? 1 : 0);
