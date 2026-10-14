import {dirname,resolve} from "node:path";
import {writeFile} from "node:fs/promises";

const ticket=resolve(dirname(import.meta.filename),"..");
const inspect=async(pid:string)=>{
 if(!/^\d+$/u.test(pid))throw Error("Numeric process identity required");
 const child=Bun.spawn(["ps","eww","-p",pid,"-o","command="],{stdout:"pipe",stderr:"ignore"});
 const source=await new Response(child.stdout).text(),status=await child.exited;
 const value=(key:string)=>source.match(new RegExp("(?:^|\\s)"+key+"=([^\\s]*)","u"))?.[1];
 const belongs=(key:string)=>{const path=value(key);return path===undefined?null:path===ticket||path.startsWith(ticket+"/");};
 const thread=value("CODEX_THREAD_ID");
 return {pid:Number(pid),present:status===0,artifactDirectoryBelongsToTicket:belongs("SEMIO_TEST_ARTIFACT_DIR"),ticketDirectoryBelongsToTicket:belongs("SEMIO_TICKET_DIR"),threadIdentityComparable:thread!==undefined&&process.env.CODEX_THREAD_ID!==undefined,sameThread:thread!==undefined&&process.env.CODEX_THREAD_ID!==undefined?thread===process.env.CODEX_THREAD_ID:null};
};
const results=[];
for(const pid of process.argv.slice(2))results.push(await inspect(pid));
await writeFile(resolve(ticket,"provenance.md"),"# Process Provenance\n\nRead-only process environment inspection emitted only ticket-membership and thread-equality booleans. Raw environment, process wire, tokens and thread identifiers were not printed or retained. A matching artifact directory alone is not sufficient authority to stop a process.\n\n```json\n"+JSON.stringify(results,null,2)+"\n```\n");
console.log("[DEBUG] Safe process provenance "+JSON.stringify(results));
