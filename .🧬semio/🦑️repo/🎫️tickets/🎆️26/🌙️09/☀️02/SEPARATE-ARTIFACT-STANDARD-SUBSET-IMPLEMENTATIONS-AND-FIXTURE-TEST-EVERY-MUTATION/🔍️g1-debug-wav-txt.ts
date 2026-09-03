import { findRepoRoot } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { loadOracleRegistry, isQualifyingOracleKind } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = findRepoRoot("/Users/ueli/Documents/semio");
const registry = loadOracleRegistry(repoRoot);
for (const owner of [
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any",
]) {
  const contribution = registry.contributions.find((c: any) => c.owner === owner);
  console.log("===", owner, "===");
  if (!contribution) { console.log("NOT FOUND"); continue; }
  console.log("comparisonPipelines:", JSON.stringify(contribution.comparisonPipelines));
  console.log("probes:", JSON.stringify(contribution.probes));
  console.log("qualifying:", contribution.oracles.filter((o:any)=>isQualifyingOracleKind(o.kind)).map((o:any)=>o.id));
}
