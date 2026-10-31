/** 🎥 Observe one normally admitted native lifecycle and retain the original child terminal status. */
import {mkdir,writeFile} from 'node:fs/promises';
import {openSync,writeSync,closeSync} from 'node:fs';
import {dirname,join,resolve} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
const root=process.cwd(),ticket=dirname(fileURLToPath(import.meta.url)),[topic]=process.argv.slice(2),language='en';if(!['array','color','text','native-text','vector','factory'].includes(topic))throw Error('Explicit isolation mode required');
const tools=await import(pathToFileURL(join(root,'🧰️framework/🔨️modules/🏃️process/🧰️tools/🟦️.ts')).href),policy=await import(pathToFileURL(join(root,'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts')).href);
tools.setDefaultToolAdmissionV1(tools.createToolAdmissionV1(policy.BUILD_HOST_CAPACITY_POLICY_V1,{environment:process.env,write:line=>console.error(line)}));
const stamp=new Date().toISOString().replace(/[^0-9]/g,''),out=join(ticket,'🗑️generated/native-r92/i',stamp);await mkdir(out,{recursive:true});const descriptor=openSync(join(out,'outer.log'),'wx');let bytes=0;
const args=['run','--frozen','--project','🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python','--python','3.12','--group','test','python','-B','-X','utf8','-X','faulthandler',join(ticket,'native-crash-isolate-r92.py'),topic,out],started=Date.now();
await writeFile(join(out,'outer.json'),JSON.stringify({state:'running',supervisor_pid:process.pid,topic,language,args,started_ms:started}));console.log('[DEBUG] bounded native probe',out);
let terminal;
try{const result=await tools.runToolV1('uv',args,{root,cwd:root,env:{...process.env,SEMIO_LANGUAGE:language,SEMIO_PROGRESS_DEADLINE_TRACE:'1'},trusted:true,heavy:true,check:false,stdout:'inherit',onLine:(line,stream)=>{const row='[DEBUG] '+stream+' '+line+'\n';bytes+=Buffer.byteLength(row);if(bytes<=5*1024*1024)writeSync(descriptor,row);}});terminal={state:'settled',status:result.status,signal:result.signal,duration_ms:result.durationMs,command:result.command};}
catch(error){terminal={state:'refused',status:null,signal:null,error:String(error),cause:error?.cause};}
finally{closeSync(descriptor);}
await writeFile(join(out,'outer.json'),JSON.stringify({supervisor_pid:process.pid,topic,language,args,started_ms:started,finished_ms:Date.now(),output_bytes_seen:bytes,...terminal},null,2));console.log('[DEBUG] bounded terminal',JSON.stringify(terminal));process.exitCode=terminal.status===0&&terminal.signal===null?0:1;
