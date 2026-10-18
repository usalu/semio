import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const reg="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";
const { verifySemanticDescriptorPairBytesV1 } = await import(reg+"/✅️catalog-verification/🟦️.ts");
const { DESCRIPTOR_JSON_REL_PATH, findPluginCargoFiles, parseComponentSourceOwnerV1 } = await import(reg+"/🔎️discovery/🟦️.ts");
const root="/Users/ueli/Documents/semio";
const manifests=findPluginCargoFiles(root); console.log("manifests",manifests.length);
let bad=0;
for (const m of manifests){ try{ const s=parseComponentSourceOwnerV1(m,root), p=resolve(root,s.cratePath,...DESCRIPTOR_JSON_REL_PATH); verifySemanticDescriptorPairBytesV1(readFileSync(p),readFileSync(join(dirname(p),"🛂️.descriptor.semio"))); }catch(e:any){ bad++; console.log("BAD",m.replace(root,""),String(e.message).slice(0,160)); } }
console.log("bad",bad);
