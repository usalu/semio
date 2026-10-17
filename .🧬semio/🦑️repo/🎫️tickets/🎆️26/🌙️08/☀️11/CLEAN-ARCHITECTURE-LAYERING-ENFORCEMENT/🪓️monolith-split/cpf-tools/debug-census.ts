import { runDebugTagCensus } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts";
const r = runDebugTagCensus("/Users/ueli/Documents/semio", new AbortController().signal, (l) => console.error(l));
for (const h of r.hits) console.log(`${h.path}:${h.line}\t${h.text}`);
console.error("files", r.files, "hits", r.hits.length, "disagreements", r.disagreements.length);
