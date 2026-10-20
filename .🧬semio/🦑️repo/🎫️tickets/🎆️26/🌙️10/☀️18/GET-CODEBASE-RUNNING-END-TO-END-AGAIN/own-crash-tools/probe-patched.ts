const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const { loadCatalogTaxonomy } = await import(`${L}/🔍️discovery/🔐️taxonomy-authority/🟦️.ts`);
const rules = loadCatalogTaxonomy().packageBoundaryRules as Record<string, { implementationAdmission?: unknown }>;
let patched = 0;
if (process.env.PATCH === "1") for (const rule of Object.values(rules)) if (rule.implementationAdmission === undefined) { rule.implementationAdmission = null; patched++; }
console.log(`[probe] rules=${Object.keys(rules).length} patched=${patched}`);
const { ownedScriptRoutes } = await import(`${L}/🏃️process/🧭️routing/🧩️contributions/🟦️.ts`);
console.log(`[probe] routes=${ownedScriptRoutes("/Users/ueli/Documents/semio").length}`);
const { policyBuildOwnerRegistryBreaches } = await import(`${L}/⚖️laws/📋️build-owner-registry/🟦️.ts`);
const breaches = policyBuildOwnerRegistryBreaches("/Users/ueli/Documents/semio");
for (const b of breaches) console.error(`[probe] breach ${b.summary}`);
console.log(`[probe] breaches=${breaches.length}`);
const { policyZeroTouchClosureBreaches } = await import(`${L}/⚖️laws/🛗️zero-touch-closure/🟦️.ts`);
const zt = policyZeroTouchClosureBreaches("/Users/ueli/Documents/semio");
for (const b of zt) console.error(`[probe] zero-touch ${b.summary}`);
console.log(`[probe] zero-touch breaches=${zt.length}`);
