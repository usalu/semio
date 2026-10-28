// sweep2-g prototype of the `imports` read: production import closure of the entries (relativeScriptInputs), minus the scheduling domains (keyed islands stay), plus the existing workspace files a closure module names as a whole path literal.
// usage: bun proto.mjs <key-list> <out-list> <entry>... ; prints counts, the added files not in the key. RSS guard.
import {readFileSync, writeFileSync, existsSync, statSync} from "node:fs";
import {join} from "node:path";
const W = "/Users/ueli/Documents/semio", LIB = join(W, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
process.chdir(W);
const inference = await import(join(LIB, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
await inference.libraryBootstrap;
const {buildSchedulingPrefixesV1, buildSchedulingKeyedPathsV1} = await import(join(LIB, "🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts"));
const [keyFile, outFile, ...entries] = process.argv.slice(2);
const prefixes = buildSchedulingPrefixesV1(), keyed = buildSchedulingKeyedPathsV1();
const scheduling = (path) => prefixes.some((prefix) => path.startsWith(prefix)) && !keyed.some((island) => path.startsWith(island));
const scripts = inference.createScriptInputCache({workspaceRoot: W});
const closure = (await inference.cacheInternals.relativeScriptInputs(entries.map((entry) => join(W, entry)), W, scripts)).map((path) => path.replace(/^\{workspaceRoot\}\//u, "")).filter((path) => !scheduling(path));
const literals = new Set();
const roots = /^(?:🧰️framework|🎓️teaching|✏️s|🌎️hub|♻️mit-bestand)\//u;
for (const file of closure) {
  if (!/\.[cm]?[jt]sx?$/u.test(file)) continue;
  let text; try { text = readFileSync(join(W, file), "utf8"); } catch { continue; }
  for (const match of text.matchAll(/["'`]([^"'`\n$\\]{4,300})["'`]/gu)) { const value = match[1]; if (roots.test(value) && !/[*?{]/u.test(value) && existsSync(join(W, value)) && statSync(join(W, value)).isFile() && !scheduling(value)) literals.add(value); }
}
const key = new Set(readFileSync(keyFile, "utf8").split("\n").filter(Boolean));
const added = [...closure, ...literals].filter((path) => !key.has(path));
for (const path of [...closure, ...literals]) if (existsSync(join(W, path))) key.add(path);
writeFileSync(outFile, [...key].sort().join("\n") + "\n");
console.log(JSON.stringify({closure: closure.length, literals: literals.size, addedToKey: new Set(added).size, rssMb: Math.round(process.memoryUsage().rss / 1048576)}));
if (process.env.SHOW) for (const path of new Set(added)) console.log("  +", path);
process.exit(0);
