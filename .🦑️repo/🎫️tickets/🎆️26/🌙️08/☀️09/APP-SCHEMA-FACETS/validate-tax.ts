const m = await import("file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%A6%91%EF%B8%8Frepo/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%93%9A%EF%B8%8Flibrary/%F0%9F%93%A6%EF%B8%8Fpackages/%F0%9F%9F%A6%EF%B8%8Ftypescript/%F0%9F%93%A6%EF%B8%8Findex.ts");
const p = m.validateTaxonomy();
console.log("problems:", p.length);
for (const x of p) console.log(" -", x);
