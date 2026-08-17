import { policySubsetSurfaceCompletenessBreaches, policyViewerPurityBreaches, policyContributedSurfaceTargetBreaches, policyOsConfigShapeBreaches } from "../../../../../../📜️script.ts";

const repoRoot = process.cwd();
const all = [
  ...policySubsetSurfaceCompletenessBreaches(repoRoot),
  ...policyViewerPurityBreaches(repoRoot),
  ...policyContributedSurfaceTargetBreaches(repoRoot),
  ...policyOsConfigShapeBreaches(repoRoot),
];
const mine = all.filter((b) => b.scope.includes("🌊️flow") || b.scope.includes("🌀️procedural"));
console.log("total breaches repo-wide:", all.length);
console.log("flow+procedural breaches:", mine.length);
for (const b of mine) console.log(JSON.stringify(b, null, 2));
