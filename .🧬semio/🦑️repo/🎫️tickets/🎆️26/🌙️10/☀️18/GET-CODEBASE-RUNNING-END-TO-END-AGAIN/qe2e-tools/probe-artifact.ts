import { receiveScriptProcessInvocation } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { siteArtifactProblems, withQuizSiteArtifact } from "../../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/🟦️.ts";
const root = process.env.SEMIO_REPO_ROOT!;
const bundle = root + "/🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript";
await receiveScriptProcessInvocation(process.env, async (original) => {
  await withQuizSiteArtifact(original, root, bundle, async (artifact) => {
    console.log("[DEBUG] path", artifact.path, "files", artifact.files.length);
    console.log("[DEBUG] problems", JSON.stringify(siteArtifactProblems(artifact.path, process.env.PROBE_ORIGIN!), null, 1));
  });
});
