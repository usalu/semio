// real-closure: bun real-closure.ts <workspace-relative entry>... : the PRODUCTION command-source closure (relativeScriptInputs) of the entries.
import {join} from "node:path";
const W = "/Users/ueli/Documents/semio", LIB = join(W, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const inference: any = await import(join(LIB, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
await inference.libraryBootstrap;
const scripts = inference.createScriptInputCache({workspaceRoot: W});
const found = await inference.cacheInternals.relativeScriptInputs(process.argv.slice(2).map((entry) => join(W, entry)), W, scripts);
const files = found.map((path: string) => path.replace(/^\{workspaceRoot\}\//u, "")).sort();
console.log(JSON.stringify({count: files.length, files}));
