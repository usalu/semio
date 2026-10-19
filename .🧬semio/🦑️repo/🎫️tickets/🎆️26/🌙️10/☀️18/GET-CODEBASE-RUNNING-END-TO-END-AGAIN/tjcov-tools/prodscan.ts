import { toolJobPluginProductionSources, toolJobProofCatalogFailures } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import { toolJobProofs, toolJobStaticRows, toolJobDispositions } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
const t = Date.now();
const files = toolJobPluginProductionSources("/Users/ueli/Documents/semio");
console.log("files", files.size, "ms", Date.now() - t);
const proofs = toolJobProofs(files);
console.log("proofs", proofs.length, "owners", new Set(proofs.map((p) => p.ownerFile)).size);
const bad = proofs.filter((p) => p.sourceFile !== p.ownerFile);
console.log("sourceFile!=ownerFile", bad.length, [...new Set(bad.map((p) => p.sourceFile.slice(-70) + " => " + p.ownerFile.slice(-60)))].slice(0, 6));
const keys = [...files.keys()].filter((k) => /\/✏️editor\/🦀️\.rs$/.test(k));
console.log("editor roots in map", keys.length);
const failures = toolJobProofCatalogFailures(files, toolJobStaticRows(files), toolJobDispositions(files), proofs);
const cat = new Map<string, number>(); for (const x of failures) { const k = x.split(" ").slice(0, 5).join(" "); cat.set(k, (cat.get(k) ?? 0) + 1); }
console.log([...cat]);
for (const pre of ["extra bounded reducer proof lacks","forged bounded reducer factory or","forged bounded reducer owner/file authority","forged bounded reducer document schema","Migrated declaration has 0 exact"]) { const s = failures.filter((x:string)=>x.startsWith(pre)); console.log("##", pre, s.length); for (const x of s.slice(0,4)) console.log("  ", x.slice(0,400)); }
await Bun.write(process.env.OUTJSON!, JSON.stringify(failures));
