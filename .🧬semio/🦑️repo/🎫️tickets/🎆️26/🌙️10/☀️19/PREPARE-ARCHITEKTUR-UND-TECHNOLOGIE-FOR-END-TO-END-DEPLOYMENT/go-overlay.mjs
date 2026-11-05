import fs from "node:fs";
import path from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";

const ticket=path.dirname(fileURLToPath(import.meta.url)),root=path.resolve(ticket,"../../../../../../.."),generated=path.join(ticket,"🗑️generated"),modules={launcher:"🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go",cli:"🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go"},selected=process.argv[2];
if(!Object.hasOwn(modules,selected))throw Error("Expected launcher or cli overlay owner");
const {canonicalGoPlan}=await import(pathToFileURL(path.join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts")).href),plan=canonicalGoPlan(path.join(root,modules[selected]));
for(const [target,source] of Object.entries(plan.replacements))if(target.length>256||source.length>256)throw Error("Canonical Go input exceeds the path contract");
fs.mkdirSync(path.join(generated,"t"),{recursive:true});fs.writeFileSync(path.join(generated,selected+"-overlay.json"),JSON.stringify({Replace:plan.replacements}));
console.log("[DEBUG] Wrote canonical "+selected+" source/test overlay DATA with "+Object.keys(plan.replacements).length+" references and no source copies");
