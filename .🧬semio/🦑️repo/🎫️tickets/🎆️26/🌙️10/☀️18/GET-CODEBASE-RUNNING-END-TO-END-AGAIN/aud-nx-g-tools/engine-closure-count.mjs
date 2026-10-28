import {pathToFileURL} from "node:url";
const mod = await import(pathToFileURL("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs").href);
const c = mod.nxBuildEngineClosureV1().files;
const cats = {"build/bazel tool": /🔧️tools\/🏗️bazel\//u, "build/scheduling (closure)": /🗓️scheduling\//u, "cache-gc": /🧹️cache-gc\//u, "build/graph inference": /🕸️graph\//u, "caching/bootstrap": /⚡️caching\/🚀️bootstrap\//u, "caching/storage": /⚡️caching\/💾️storage\//u, "framework/process": /^🧰️framework\/🔨️modules\/🏃️process\//u, "tests": /🧪️tests\/|\/tests\//u};
for (const [k, re] of Object.entries(cats)) console.log(k.padEnd(30), c.filter(f => re.test(f)).length);
console.log("total", c.length);
