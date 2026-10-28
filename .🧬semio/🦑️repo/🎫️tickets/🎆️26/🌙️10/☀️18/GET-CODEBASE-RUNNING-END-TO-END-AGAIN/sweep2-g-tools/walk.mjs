// sweep2-g: the REAL command-closure walk (relativeScriptInputs) of the inference for entry files, graph-free; prints the closure (workspace-relative) and compares it with a key list. usage: bun walk.mjs <key-list|-> <entry>... ; RSS guard 1.5 GB.
import {join} from "node:path";
import {readFileSync} from "node:fs";
const W = "/Users/ueli/Documents/semio", LIB = join(W, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
process.chdir(W);
const inference = await import(join(LIB, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
await inference.libraryBootstrap;
const [list, ...entries] = process.argv.slice(2);
const scripts = inference.createScriptInputCache({workspaceRoot: W});
const closure = (await inference.cacheInternals.relativeScriptInputs(entries.map((entry) => join(W, entry)), W, scripts)).map((path) => path.replace(/^\{workspaceRoot\}\//u, ""));
const key = list === "-" ? new Set() : new Set(readFileSync(list, "utf8").split("\n").filter(Boolean));
const {buildSchedulingPrefixesV1, buildSchedulingKeyedPathsV1} = await import(join(LIB, "🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts"));
const prefixes = buildSchedulingPrefixesV1(), keyed = buildSchedulingKeyedPathsV1();
const scheduling = (path) => prefixes.some((prefix) => path.startsWith(prefix)) && !keyed.some((island) => path.startsWith(island));
const missing = closure.filter((path) => !key.has(path) && !scheduling(path));
console.log(JSON.stringify({closure: closure.length, missingFromKey: missing.length, rss: Math.round(process.memoryUsage().rss / 1048576)}));
for (const path of missing) console.log(path);
process.exit(0);
