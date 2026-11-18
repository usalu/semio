/** 🔎️ Runs the publish verb's CDN artifact checks on the graph-free release build and optionally stages it with `_headers` into `STAGE_DIRECTORY`. */
import { join, resolve } from "node:path";
import { releaseProctorOrigin, siteArtifactProblems, stageCourseSiteArtifact } from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🌐️site/🟦️.ts";

const dist = process.env.ARTIFACT_DIRECTORY ?? join(resolve(import.meta.dir, "../../../../../../.."), "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/dist/release");
const problems = siteArtifactProblems(dist, releaseProctorOrigin());
console.log(problems.length ? problems.map(problem => "- " + problem).join("\n") : "no problems");
if (process.env.STAGE_DIRECTORY && !problems.length) stageCourseSiteArtifact(dist, process.env.STAGE_DIRECTORY, releaseProctorOrigin());
