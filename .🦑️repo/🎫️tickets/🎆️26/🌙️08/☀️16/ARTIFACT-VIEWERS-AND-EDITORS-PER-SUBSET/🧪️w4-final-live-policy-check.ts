import {
  policySubsetSurfaceCompletenessBreaches,
  policyViewerPurityBreaches,
  policyContributedSurfaceTargetBreaches,
  policyOsConfigShapeBreaches,
} from "/Users/ueli/Documents/semio/📜️script.ts";

const repoRoot = "/Users/ueli/Documents/semio";

const completeness = policySubsetSurfaceCompletenessBreaches(repoRoot);
const purity = policyViewerPurityBreaches(repoRoot);
const contributed = policyContributedSurfaceTargetBreaches(repoRoot);
const osConfig = policyOsConfigShapeBreaches(repoRoot);

console.log("policySubsetSurfaceCompletenessBreaches (repo-wide):", completeness.length);
for (const r of completeness) console.log(JSON.stringify(r));
console.log("policyViewerPurityBreaches (repo-wide):", purity.length);
for (const r of purity) console.log(JSON.stringify(r));
console.log("policyContributedSurfaceTargetBreaches (repo-wide):", contributed.length);
for (const r of contributed) console.log(JSON.stringify(r));
console.log("policyOsConfigShapeBreaches (repo-wide):", osConfig.length);
for (const r of osConfig) console.log(JSON.stringify(r));
