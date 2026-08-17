const { loadTaxonomy, validateTaxonomy } = await import("./\ud83e\uddf0\ufe0fframework/\ud83d\udecd\ufe0fproducts/\ud83e\udd91\ufe0frepo/\ud83d\udd28\ufe0fmodules/\ud83d\udcda\ufe0flibrary/\ud83d\udce6\ufe0fpackages/\ud83d\udfe6\ufe0ftypescript/\ud83d\udce6\ufe0findex.ts");
const t = loadTaxonomy();
console.log("slug", t.exampleSlugPattern);
console.log("validate", JSON.stringify(validateTaxonomy()));
console.log("leafHasPacks", t.taxonomyLeafParentDirs.includes("🎒️packs"));
console.log("assetsInLeaf", t.taxonomyLeafParentDirs.includes(t.exampleAssetsDirName));
console.log("testsInLeaf", t.taxonomyLeafParentDirs.includes(t.exampleTestsDirName));
