/** 🗂️ Writes the schema-document index projection of a root (`bun regen.ts <root> <out>`); `--check` compares instead and exits 1 when the root's index differs. */
import {readFileSync,writeFileSync} from "node:fs";
const [root,out]=process.argv.slice(2) as [string,string],D=root+"/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive";
const {mutationSchemaDocumentIndexProjection}=await import(D+"/🗂️schema-index/🟦️.ts");
const text=mutationSchemaDocumentIndexProjection(root);
if(out==="--check"){const same=readFileSync(D+"/🗂️schema-document-index.json","utf8")===text;console.log(same?"schema index current":"schema index STALE");process.exit(same?0:1);}
writeFileSync(out,text);
