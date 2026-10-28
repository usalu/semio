import {pathToFileURL} from "node:url";
const mod = await import(pathToFileURL(process.cwd()+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs").href);
const c = mod.nxBuildEngineClosureV1();
for (const f of c.files) console.log(f);
console.error("packages", c.packages.join(","), "files", c.files.length, "rssMB", Math.round(process.memoryUsage().rss/1e6));
