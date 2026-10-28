import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const D = "/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const text = brotliDecompressSync(readFileSync(D + process.argv[2])).toString("utf8"), at = text.indexOf("\n"), header = JSON.parse(text.slice(0, at)), value = JSON.parse(text.slice(at + 1));
console.log("HEADER keys", Object.keys(header), JSON.stringify(header).length);
for (const [k, v] of Object.entries(header)) console.log(" ", k, JSON.stringify(v).slice(0, 300));
console.log("VALUE keys", Object.keys(value));
for (const [key, recipe] of Object.entries(value.closureRecipes ?? {})) { const r = JSON.stringify(recipe); if (r.includes(process.argv[3] ?? "test-macro")|| r.includes("macro")) console.log("RECIPE", key.slice(0, 12), r.slice(0, 1800)); }
console.log("recipes", Object.keys(value.closureRecipes ?? {}).length);
