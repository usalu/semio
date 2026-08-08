
import { loadTaxonomy, validateTaxonomy, clearDiscoveryCache } from "/Users/ueli/Documents/semio/\ud83e\uddf0\ufe0fframework/\ud83d\udecd\ufe0fproducts/\ud83e\udd91\ufe0frepo/\ud83d\udd28\ufe0fmodules/\ud83d\udcda\ufe0flibrary/\ud83d\udd0d\ufe0fdiscovery/\ud83d\udfe6\ufe0fcomponent.ts";

clearDiscoveryCache?.();
const taxonomy = loadTaxonomy();
const problems = validateTaxonomy(taxonomy);
const expects = {
  artifactComponentDirs: ["🧬️mutations", "🔺️diff", "🗣️dsl", "🎒️pack", "🔧️op", "📡️spr", "⚙️engine"],
  artifactChildDirs: ["🧬️mutations", "🔺️diff", "🗣️dsl", "🎒️pack", "🔧️op", "📡️spr", "⚙️engine", "📚️examples"],
  mutationChildDirs: ["🦠️mutation", "🔺️diff", "↩️inverse"],
  structuralOnly: ["📚️examples"],
  leafParents: ["🧬️mutations", "🦠️mutation", "↩️inverse", "🔺️diff"],
};

function eq(a: unknown, b: unknown) {
  return JSON.stringify(a) === JSON.stringify(b);
}

const failures: string[] = [];
if (!eq(taxonomy.artifactComponentDirs, expects.artifactComponentDirs)) {
  failures.push(`artifactComponentDirs: got ${JSON.stringify(taxonomy.artifactComponentDirs)}`);
}
if (!eq(taxonomy.artifactChildDirs, expects.artifactChildDirs)) {
  failures.push(`artifactChildDirs: got ${JSON.stringify(taxonomy.artifactChildDirs)}`);
}
if (!eq(taxonomy.mutationChildDirs, expects.mutationChildDirs)) {
  failures.push(`mutationChildDirs: got ${JSON.stringify(taxonomy.mutationChildDirs)}`);
}
const structuralOnly = taxonomy.artifactChildDirs.filter((d) => !taxonomy.artifactComponentDirs.includes(d));
if (!eq(structuralOnly, expects.structuralOnly)) {
  failures.push(`structuralOnly: got ${JSON.stringify(structuralOnly)}`);
}
for (const p of expects.leafParents) {
  if (!taxonomy.taxonomyLeafParentDirs.includes(p)) failures.push(`missing leaf parent ${p}`);
}
if (problems.length) failures.push(`validateTaxonomy: ${problems.join(" | ")}`);

// mutationChildDirs validation negative cases
const brokenParents = {
  ...taxonomy,
  taxonomyLeafParentDirs: taxonomy.taxonomyLeafParentDirs.filter((d) => d !== "🦠️mutation"),
};
if (!validateTaxonomy(brokenParents).some((p) => p.includes("🦠️mutation"))) {
  failures.push("expected brokenParents to report 🦠️mutation");
}
const brokenEmpty = { ...taxonomy, mutationChildDirs: [] as string[] };
if (!validateTaxonomy(brokenEmpty).some((p) => p.includes("mutationChildDirs"))) {
  failures.push("expected empty mutationChildDirs to report");
}
const brokenMissingEngine = {
  ...taxonomy,
  artifactComponentDirs: taxonomy.artifactComponentDirs.filter((d) => d !== "⚙️engine"),
};
if (!validateTaxonomy(brokenMissingEngine).some((p) => p.includes("⚙️engine"))) {
  failures.push("expected missing engine completeness to report");
}

if (failures.length) {
  console.error("FAIL");
  for (const f of failures) console.error(" -", f);
  process.exit(1);
}
console.log("PASS wave2a taxonomy+validateTaxonomy");
console.log(JSON.stringify({
  artifactComponentDirs: taxonomy.artifactComponentDirs,
  mutationChildDirs: taxonomy.mutationChildDirs,
  validateTaxonomy: problems,
}, null, 2));
