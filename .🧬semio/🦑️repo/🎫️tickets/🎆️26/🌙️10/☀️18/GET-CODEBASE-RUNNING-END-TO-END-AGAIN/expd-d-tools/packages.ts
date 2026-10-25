import { writeFileSync } from "node:fs";
import { loadDependencyDirectionPolicy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🚀️bootstrap/🟦️.ts";
const snap = loadDependencyDirectionPolicy("/Users/ueli/Documents/semio");
writeFileSync(process.argv[2]!, JSON.stringify(snap.workspacePackages, null, 1));
console.log(snap.workspacePackages.length);
