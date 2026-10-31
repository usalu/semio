/** 🎥 Observe one normally admitted native lifecycle and retain the original child terminal status. */
import {mkdir,writeFile,readFile} from 'node:fs/promises';
import {openSync,writeSync,closeSync} from 'node:fs';
import {dirname,join,resolve} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
const root=process.cwd(),ticket=dirname(fileURLToPath(import.meta.url)),[topic,language]=process.argv.slice(2);if(!topic||!['en','de'].includes(language))throw Error('Explicit topic/language required');
const tools=await import(pathToFileURL(join(root,'🧰️framework/🔨️modules/🏃️process/🧰️tools/🟦️.ts')).href),policy=await import(pathToFileURL(join(root,'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts')).href);
tools.setDefaultToolAdmissionV1(tools.createToolAdmissionV1(policy.BUILD_HOST_CAPACITY_POLICY_V1,{environment:process.env,write:line=>console.error(line)}));
const model=JSON.parse(await readFile(join(root,'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json'),'utf8')),cases=JSON.parse(await readFile(join(root,'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🎭️presentation/🔣️.cases.json'),'utf8')),beats=cases.chapters.find(row=>row.topic===topic)?.keys.length;if(!beats)throw Error('Unknown lifecycle topic');const cells=model.elements.reduce((sum,row)=>sum+(row.geometry?.parts?.length??0),0),seconds=Math.ceil(600+240*beats+0.02*cells),budget={seconds,beats,source_cells:cells,formula:'ceil(600+240*beats+0.02*source_cells)'};
const stamp=new Date().toISOString().replace(/[^0-9]/g,''),out=join(ticket,'🗑️generated/native-r92/p',stamp);await mkdir(out,{recursive:true});const descriptor=openSync(join(out,'outer.log'),'wx');let bytes=0;
const args=['run','--frozen','--project','🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python','--python','3.12','--group','test','python','-B','-X','utf8','-X','faulthandler',join(ticket,'native-lifecycle-probe-r92.py'),topic,language,out],started=Date.now();
await writeFile(join(out,'outer.json'),JSON.stringify({state:'running',supervisor_pid:process.pid,topic,language,args,budget,started_ms:started}));console.log('[DEBUG] bounded native probe',out);
let terminal;
try{const result=await tools.runToolV1('uv',args,{root,cwd:root,env:{...process.env,SEMIO_LANGUAGE:language,SEMIO_PROGRESS_DEADLINE_TRACE:'1',SEMIO_NATIVE_DIAGNOSTIC_SECONDS:String(seconds)},trusted:true,heavy:true,check:false,stdout:'inherit',timeout:(seconds+60)*1000,onLine:(line,stream)=>{const row='[DEBUG] '+stream+' '+line+'\n';bytes+=Buffer.byteLength(row);if(bytes<=5*1024*1024)writeSync(descriptor,row);}});terminal={state:'settled',status:result.status,signal:result.signal,duration_ms:result.durationMs,command:result.command};}
catch(error){terminal={state:'refused',status:null,signal:null,error:String(error),cause:error?.cause};}
finally{closeSync(descriptor);}
await writeFile(join(out,'outer.json'),JSON.stringify({supervisor_pid:process.pid,topic,language,args,budget,started_ms:started,finished_ms:Date.now(),output_bytes_seen:bytes,...terminal},null,2));console.log('[DEBUG] bounded terminal',JSON.stringify(terminal));process.exitCode=terminal.status===0&&terminal.signal===null?0:1;
