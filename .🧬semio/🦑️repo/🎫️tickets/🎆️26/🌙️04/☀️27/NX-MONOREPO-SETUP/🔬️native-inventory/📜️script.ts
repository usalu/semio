import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
const require = createRequire(import.meta.url);
for (const id of ["nx/src/project-graph/project-graph", "nx/src/project-graph/nx-deps-cache", "@nx/devkit"]) {
 const path = require.resolve(id), source = readFileSync(path, "utf8");
 console.log(id, path, Object.keys(require(id)).filter(key => /[Gg]raph|SourceMap/.test(key)));
 if (id.includes("project-graph/project-graph")) console.log(source.slice(source.indexOf("function readCachedProjectGraph"), source.indexOf("function readCachedProjectGraph") + 3000));
}
