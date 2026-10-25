import {deriveAgentClientFiles,readDeclaredTools} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🤖️agent-clients/🟦️.ts";
import {readFileSync,writeFileSync} from "node:fs";
const root="/Users/ueli/Documents/semio";
for(const f of deriveAgentClientFiles(readDeclaredTools(root))){const cur=readFileSync(root+"/"+f.path,"utf8");if(cur!==f.text){writeFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/repo-tests-c/"+f.path.replace(/[\/.]/g,"_")+".derived",f.text);console.log("DIFF",f.path,cur.length,f.text.length);}}
