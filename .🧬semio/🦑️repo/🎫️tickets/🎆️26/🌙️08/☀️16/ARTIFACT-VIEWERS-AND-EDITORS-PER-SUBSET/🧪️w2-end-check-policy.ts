import {
  policySubsetSurfaceCompletenessBreaches,
  policyViewerPurityBreaches,
  policyContributedSurfaceTargetBreaches,
  policyOsConfigShapeBreaches,
} from "/Users/ueli/Documents/semio/📜️script.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const plugins = ["🔋️energy", "🗄️stdio", "🪐️space", "🎪️demonstrator"];

function filterFor(records: any[]) {
  return records.filter((r) => {
    const p = JSON.stringify(r);
    return plugins.some((pl) => p.includes(`🔌️plugins/${pl}`));
  });
}

const completeness = filterFor(policySubsetSurfaceCompletenessBreaches(repoRoot));
const purity = filterFor(policyViewerPurityBreaches(repoRoot));
const contributed = filterFor(policyContributedSurfaceTargetBreaches(repoRoot));
const osConfig = policyOsConfigShapeBreaches(repoRoot);

console.log("surface-completeness breaches (4 plugins):", completeness.length);
for (const r of completeness) console.log(JSON.stringify(r));
console.log("viewer-purity breaches (4 plugins):", purity.length);
for (const r of purity) console.log(JSON.stringify(r));
console.log("contributed-surface-target breaches (4 plugins):", contributed.length);
for (const r of contributed) console.log(JSON.stringify(r));
console.log("os-config-shape breaches (repo-wide, unaffected check):", osConfig.length);
