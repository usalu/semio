import {createRequire} from "node:module";
import {resolve,dirname,join} from "node:path";
import {readFileSync,writeFileSync} from "node:fs";
const root=process.cwd(),path=resolve("✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/📚️office-schema-contract/🟦️.ts"),require=createRequire(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json")),ts=require("typescript"),imports=ts.preProcessFile(readFileSync(path,"utf8"),true,true).importedFiles.filter((row:any)=>row.fileName.startsWith(".")).map((row:any)=>({specifier:row.fileName,actual:Bun.resolveSync(row.fileName,dirname(path))}));
if(!imports.some((row:any)=>row.specifier.endsWith("🧬️mutations/🟦️.ts")))throw Error("Current mutation declaration missing");
const module=await import(path);module.testStdioOfficeSchemaFixtures(root);
writeFileSync(join(import.meta.dir,"../🗑️generated/office-contract-import-oracle.json"),JSON.stringify({module:path,relativeImports:imports.length,imports},null,2)+"\n");
console.log("[DEBUG] Actual Office parser/schema laws and independent TypeScript/Bun resolution passed: "+imports.length+" relative imports");
