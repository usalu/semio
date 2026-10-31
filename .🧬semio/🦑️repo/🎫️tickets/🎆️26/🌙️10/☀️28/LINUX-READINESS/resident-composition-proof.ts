import {test,expect,afterAll} from "bun:test";
import {join} from "node:path";
import {pathToFileURL} from "node:url";
import {copyFileSync,symlinkSync,writeFileSync,existsSync} from "node:fs";
const checkout=process.cwd(),library=join(checkout,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),load=(path:string)=>import(pathToFileURL(join(library,path)).href);
const {privateRepositoryWorkspaceV1}=await load("🧪️testing/🧫️private-repository/🟦️.ts"),{withRepositoryResidentWatcherV1}=await load("⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts"),{repositoryFilesystemJournalPolicyV1}=await load("⚡️caching/🕸️graph/🔏️seal/📥️membership/📥️journal/🟦️.ts"),{createForegroundRunnerV1}=await load("🏗️build/🏡️resident/🏗️composition/🟨️.mjs"),{runResidentCommandV1}=await load("🏗️build/🏡️resident/⌨️command/🟨️.mjs"),{endpointsV1}=await load("🏗️build/🏡️resident/📨️protocol/📍️endpoint/🟨️.mjs"),{shortTempDirectory,cleanup,waitFor}=await load("🏗️build/🏡️resident/🧪️tests/🟦️.ts"),{createIssuedResourceControlV1,scriptInvocationEnvironment}=await import(pathToFileURL(join(checkout,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts")).href);
const ipc=shortTempDirectory("rc-");afterAll(()=>cleanup(ipc));
test("actual owned repository composition uses native private readiness and public status-stop",async()=>{
 const repository=await privateRepositoryWorkspaceV1();symlinkSync(join(checkout,"🧰️framework"),join(repository.root,"🧰️framework"),process.platform==="win32"?"junction":"dir");copyFileSync(join(checkout,"rust-toolchain.toml"),join(repository.root,"rust-toolchain.toml"));
 await repository.invoke((original:any)=>withRepositoryResidentWatcherV1(repository.root,original,createIssuedResourceControlV1(original,"resident:composition-proof",0),async(binary:string,control:any)=>{
  const root=await repository.scratch("composition-");writeFileSync(join(root,"visible.json"),"visible");writeFileSync(join(root,".p0123abcd"),"private");
  const environment=scriptInvocationEnvironment(original,{...process.env,XDG_RUNTIME_DIR:ipc,SEMIO_RESIDENT_EXECUTORS:"0"}),runtime=endpointsV1(root,environment),sourcePolicy=await repositoryFilesystemJournalPolicyV1(root),run=createForegroundRunnerV1({sourcePolicy,signal:control.signal,watcherBinary:binary,script:"unused",loadGraph:async()=>({version:1,projects:{},dependencies:[]}),salt:()=>"",warmSize:0,executorSize:0,environmentPaths:[],environmentPrefixes:[]});
  const running=run({root,runtime,environment});let ended=false;void running.finally(()=>{ended=true;}).catch(()=>{});
  const sink=()=>{let value="";return{write(text:string){value+=text;},text(){return value;}};};
  try{
   expect(await waitFor(()=>existsSync(runtime.descriptor)||ended,15000)).toBe(true);if(ended)await running;
   const status=sink();expect(await runResidentCommandV1({root,args:["status","--json"],languages:["en","de"],environment,stdout:status,stderr:status})).toBe(0);const value=JSON.parse(status.text());expect(value.watcher.backend).toBe(process.platform==="linux"?"inotify":process.platform==="darwin"?"fsevents":"rdcw");expect(value.watcher.trust).toBe("exact");expect(value.state.files).toBe(1);
  }finally{const output=sink();await runResidentCommandV1({root,args:["stop"],languages:["en","de"],environment,stdout:output,stderr:output});}
  expect(await running).toBe(0);expect(existsSync(binary)).toBe(true);
 }),{owner:"resident-composition-proof",maximumElapsedMilliseconds:180000});
},240000);
