import {censusNxAuthorizerSourcesV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import {writeFile} from "node:fs/promises";
import {resolve} from "node:path";
const root=process.cwd(),entry="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts";
try{const result=await censusNxAuthorizerSourcesV1(root,entry,AbortSignal.timeout(600000),{installed:true,progress:files=>{if(files%128===0)console.error("[DEBUG] census files="+files);}});await writeFile(resolve(import.meta.dir,"🗑️generated/source-census-full.json"),JSON.stringify(result,null,2));console.log(JSON.stringify({files:result.files,sourceBytes:result.sourceBytes,snapshotBytes:result.snapshotBytes}));}catch(error){console.error(error);process.exitCode=1;}
