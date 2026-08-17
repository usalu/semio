import { policySubsetSurfaceCompletenessBreaches, policyViewerPurityBreaches, policyContributedSurfaceTargetBreaches } from "../../../../../../📜️script.ts";

const repoRoot = process.cwd();
const scope = "🎪️demonstrator";

for (const [name, fn] of [
  ["surface-completeness", policySubsetSurfaceCompletenessBreaches],
  ["viewer-purity", policyViewerPurityBreaches],
  ["contributed-surface-target", policyContributedSurfaceTargetBreaches],
] as const) {
  const all = fn(repoRoot);
  const mine = all.filter((b) => JSON.stringify(b).includes(scope));
  console.log(`${name}: total=${all.length} demonstrator=${mine.length}`);
  for (const b of mine) console.log("  ", JSON.stringify(b));
}
