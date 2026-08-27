import { testTaxonomy, testFilenameForKind } from "/Users/ueli/Documents/semio/./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const tx: any = testTaxonomy(process.cwd());
console.log("contributionFileKindId:", tx.testContributionFileKindId);
console.log("filename:", JSON.stringify(testFilenameForKind(tx, tx.testContributionFileKindId)));
console.log("featureFileKind:", tx.testFeatureFileKindId, JSON.stringify(testFilenameForKind(tx, tx.testFeatureFileKindId)));
console.log("testsDirName:", tx.testsDirName);
