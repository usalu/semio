import {taxonomyImplementationFilesystemFindings} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🗂️file-kind-resolution/🟦️.ts";
const root=process.cwd();
const started=Date.now();
const findings=await taxonomyImplementationFilesystemFindings(root);
console.log("findings",findings.length,"ms",Date.now()-started);
for(const f of findings){ if(/browser-package|🫀️core|preview2-shim|flow\/📦️packages/u.test(f.path)) console.log(JSON.stringify(f).slice(0,300)); }
