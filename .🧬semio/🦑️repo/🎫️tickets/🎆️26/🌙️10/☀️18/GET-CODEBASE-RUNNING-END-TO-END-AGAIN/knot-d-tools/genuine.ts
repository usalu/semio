import { runGenuineBunTestV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🟦️.ts";
const [file, pattern] = process.argv.slice(2) as [string, string | undefined];
await runGenuineBunTestV1({ root: "/Users/ueli/Documents/semio", file, ...(pattern ? { pattern } : {}) });
console.log("genuine run finished", file);
