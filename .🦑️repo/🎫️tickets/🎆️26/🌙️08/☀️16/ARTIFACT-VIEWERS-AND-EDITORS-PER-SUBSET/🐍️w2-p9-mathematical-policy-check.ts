import { policySubsetSurfaceCompletenessBreaches, policyViewerPurityBreaches, policyContributedSurfaceTargetBreaches, policyOsConfigShapeBreaches } from "../../../../../../📜️script.ts";

const repoRoot = process.cwd();
const all = [
  ...policySubsetSurfaceCompletenessBreaches(repoRoot),
  ...policyViewerPurityBreaches(repoRoot),
  ...policyContributedSurfaceTargetBreaches(repoRoot),
  ...policyOsConfigShapeBreaches(repoRoot),
];
const mathematicalOnly = all.filter((b) => b.scope.includes("➗️mathematical"));
console.log("total breaches repo-wide:", all.length);
console.log("mathematical breaches:", mathematicalOnly.length);
for (const b of mathematicalOnly) console.log(JSON.stringify(b, null, 2));
