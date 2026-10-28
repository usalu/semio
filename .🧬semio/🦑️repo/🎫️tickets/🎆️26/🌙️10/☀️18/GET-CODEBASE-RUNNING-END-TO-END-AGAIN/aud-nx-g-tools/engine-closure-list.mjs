import {pathToFileURL} from "node:url";
const mod = await import(pathToFileURL("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs").href);
const c = mod.nxBuildEngineClosureV1().files;
for (const f of c) if (/cache-gc|🗓️scheduling\/🟦️\.ts|🔧️tools\/🏗️bazel\/🌡️host-capacity|🔧️tools\/🏗️bazel\/🎬️executor/u.test(f)) console.log(f);
console.log("bazel-tool files:", c.filter(f => /🔧️tools\/🏗️bazel\//u.test(f)).length);
console.log("process files:", c.filter(f => /^🧰️framework\/🔨️modules\/🏃️process\//u.test(f)).length);
console.log("bootstrap files:", c.filter(f => /⚡️caching\/🚀️bootstrap\//u.test(f)).length);
