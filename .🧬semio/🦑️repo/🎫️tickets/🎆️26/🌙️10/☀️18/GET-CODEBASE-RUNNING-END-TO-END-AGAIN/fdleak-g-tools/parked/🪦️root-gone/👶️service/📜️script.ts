import{mkdirSync}from"node:fs";
import{CHANGE_JOURNAL_POLICY_V1}from"../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import{runChangeJournalServiceV1}from"../../../🟦️.ts";

const[root,directory,owner]=process.argv.slice(2) as [string,string,string|undefined];
mkdirSync(root,{recursive:true});
const service=await runChangeJournalServiceV1({root,directory,fence:".fence",exclude:{rooted:[],names:[]},watcherPolicy:{},policy:{...CHANGE_JOURNAL_POLICY_V1,heartbeatMilliseconds:40},residentBytes:()=>0,...(owner?{owner:Number(owner)}:{}),createSupervisor:()=>({start:async()=>({backend:"fsevents",trust:"exact"}),stop:async()=>{}})});
process.stdout.write("READY\n");
process.stdout.write("CLOSED "+await service.closed+"\n");
process.exit(0);
