import {pathToFileURL} from "node:url";
const mod = await import(pathToFileURL("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs").href);
const c = mod.nxBuildEngineClosureV1();
console.log("files", c.files.length, "packages", c.packages.length);
const hits = c.files.filter(f => /schedul|cache-gc|bazel|graph\/📥|🏗️scheduling/u.test(f));
console.log("scheduling/bazel/cache-gc/graph-inference hits", hits.length);
for (const h of hits.slice(0, 15)) console.log(" ", h);
