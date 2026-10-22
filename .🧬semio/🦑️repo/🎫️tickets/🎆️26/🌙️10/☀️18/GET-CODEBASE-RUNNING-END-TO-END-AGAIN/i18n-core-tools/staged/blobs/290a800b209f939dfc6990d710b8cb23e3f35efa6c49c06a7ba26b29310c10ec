import {expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {copyFileSync,cpSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import scenario from "./🧫️scenario.json";
import {bazelCacheFlagsV1,bazelCacheLayoutV1,bazelServerBaseV1} from "../../../💾️storage/🟦️.ts";
import {acquireBazelSessionV1,bazelRepositoryContentsV1,bazelSessionCommandV1,bazelSessionResourcesV1,bazelSessionStartupV1,bazelSessionWorkspaceV1,planBazelServersV1,withBazelSessionV1} from "../../🟦️.ts";
import {bazelSlotPortV1,removeBazelTreeV1} from "../../🔌️port/🟦️.ts";

const binary=process.env.SEMIO_TEST_BAZEL_BINARY;
type Run={status:number|null;stdout:string;stderr:string;ms:number};
const key=(index:number)=>(0xc0ffee00+index).toString(16).padStart(16,"0");

/** 🔎️ Finds the prepared fixed module catalog (lock and repository cache) of the checkout; an explicit directory wins. */
function catalogOf():string|undefined{
 const explicit=process.env.SEMIO_TEST_BAZEL_CATALOG;if(explicit)return explicit;
 const base=join(process.cwd(),".cache/b/d/m/bazel/modules");if(!existsSync(base))return undefined;
 for(const name of readdirSync(base)){const directory=join(base,name);if(existsSync(join(directory,"workspace/MODULE.bazel.lock"))&&existsSync(join(directory,"repositories")))return directory;}
 return undefined;
}

test.skipIf(!binary||catalogOf()===undefined)("concurrent sessions build disjoint targets without a lock wait, share every result and a fourth session rebuilds one of them entirely from the shared cache",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"cb-")),catalog=catalogOf()!,layout=bazelCacheLayoutV1({tools:join(root,"t"),dependencies:join(root,"d"),products:join(root,"p")}),markers=join(root,"markers"),install=join(root,"install"),repositories=join(root,"catalog/repositories");
 const base=planBazelServersV1({cores:10,memoryMegabytes:32768},{floorBasisPoints:2000,floorMinimumMegabytes:4096,floorMaximumMegabytes:8192}),plan={...base,slots:scenario.servers,heapMegabytes:1024},directories={leases:join(root,"l"),server:layout.server,workspaces:layout.workspace},lines:string[]=[],port=bazelSlotPortV1(directories,{write:line=>{lines.push(line);}}),signal=new AbortController().signal;
 const rc=join(process.cwd(),".bazelrc"),lock=join(catalog,"workspace/MODULE.bazel.lock");
 mkdirSync(markers,{recursive:true});mkdirSync(join(root,"catalog"),{recursive:true});cpSync(join(catalog,"repositories"),repositories,{recursive:true});
 const startup=(slot:string)=>[...bazelSessionStartupV1(plan,120),`--install_base=${install}`,"--nosystem_rc","--nohome_rc","--noworkspace_rc",`--bazelrc=${rc}`,...bazelCacheFlagsV1(layout,slot,{environment:{}}).startup];
 const command=(slot:string)=>[...bazelCacheFlagsV1(layout,slot,{environment:{}}).command,...bazelSessionCommandV1(bazelRepositoryContentsV1(layout.outputs)),...bazelSessionResourcesV1(plan),`--repository_cache=${repositories}`,"--repository_disable_download","--lockfile_mode=error","--incompatible_autoload_externally=","--nobuild_runfile_links","--jobs=2"];
 const run=(args:string[],cwd:string):Promise<Run>=>new Promise(resolve=>{const started=Date.now(),child=spawn(binary!,args,{cwd,stdio:["ignore","pipe","pipe"]});let stdout="",stderr="";child.stdout.setEncoding("utf8");child.stderr.setEncoding("utf8");child.stdout.on("data",text=>{stdout+=text;});child.stderr.on("data",text=>{stderr+=text;});child.on("close",status=>resolve({status,stdout,stderr,ms:Date.now()-started}));});
 const generate=(workspace:string,targets:readonly string[])=>{
  mkdirSync(workspace,{recursive:true});writeFileSync(join(workspace,"MODULE.bazel"),'module(name = "semio_tasks", version = "0.0.0")\n');copyFileSync(lock,join(workspace,"MODULE.bazel.lock"));
  writeFileSync(join(workspace,"BUILD.bazel"),targets.map(name=>`genrule(\n    name = "${name}",\n    outs = ["${name}.out"],\n    cmd = "echo x >> ${markers.replaceAll("\\","/")}/${name}; sleep ${scenario.sleepSeconds}; echo ${name} > $@",\n    tags = ["no-sandbox"],\n)\n`).join(""));
 };
 const spawns=(text:string)=>{const line=/INFO: \d+ processes?: ([^\n]*)\./u.exec(text)?.[1]??"";return line.split(", ").map(part=>part.replace(/^\d+ /u,"")).filter(Boolean);};
 const lockWait=(text:string)=>scenario.lockWaitMarkers.some(marker=>text.includes(marker));
 const executions=(name:string)=>existsSync(join(markers,name))?readFileSync(join(markers,name),"utf8").split("\n").filter(Boolean).length:0;
 const report:Record<string,unknown>={};
 const serverMegabytes=()=>process.platform==="win32"?[]:spawnSync("ps",["-eo","rss=,args="],{encoding:"utf8"}).stdout.split("\n").filter(line=>line.includes("--output_base="+layout.server)||line.includes(layout.server+"/s")).map(line=>Math.round(Number(line.trim().split(/\s+/u)[0])/1024)).filter(value=>value>50);
 const stopAll=async()=>{for(const slot of ["s0","s1","s2","s3"])if(existsSync(bazelServerBaseV1(layout,slot)))spawnSync(binary!,[`--install_base=${install}`,"--nosystem_rc","--nohome_rc","--noworkspace_rc",`--output_base=${bazelServerBaseV1(layout,slot)}`,"shutdown"],{cwd:root,encoding:"utf8"});};
 const pinned:Awaited<ReturnType<typeof acquireBazelSessionV1>>[]=[];
 try{
  const prime=spawnSync(binary!,["--batch",`--install_base=${install}`,`--output_base=${join(root,"prime")}`,"--nosystem_rc","--nohome_rc","--noworkspace_rc","version"],{cwd:root,encoding:"utf8"});expect(prime.status,prime.stderr).toBe(0);

  for(let index=0;index<scenario.pinnedServices;index++)pinned.push(await acquireBazelSessionV1(port,plan,{language:"en",key:key(200+index),signal,kind:"service",timeoutMilliseconds:0}));
  expect(pinned.map(row=>row.slot),"dev servers hold service slots, not build slots").toEqual(Array.from({length:scenario.pinnedServices},(_,index)=>"d"+index));
  const started=Date.now(),sessions=await Promise.all(scenario.sessions.map((name,index)=>withBazelSessionV1(port,plan,{language:"en",key:key(index),signal},async session=>{
   const workspace=bazelSessionWorkspaceV1(layout.workspace,session.slot,key(index));generate(workspace,[name]);
   const result=await run([...startup(session.slot),"build",...command(session.slot),"//:"+name],workspace);return {name,slot:session.slot,mode:session.mode,result,workspace};
  })));
  const wall=Date.now()-started;report.servers=serverMegabytes();report.parallel={wallMs:wall,sessions:sessions.map(row=>({name:row.name,slot:row.slot,ms:row.result.ms,spawns:spawns(row.result.stderr)}))};
  expect(lines.filter(line=>line.includes("waiting")),"pinned dev servers never make a build wait").toEqual([]);
  expect(new Set(sessions.map(row=>row.slot)).size,"every session holds its own slot").toBe(scenario.sessions.length);
  for(const row of sessions){expect(row.result.status,row.result.stderr).toBe(0);expect(lockWait(row.result.stderr),`${row.name} waited for a Bazel server lock`).toBe(false);expect(spawns(row.result.stderr),row.name).toContain("local");expect(executions(row.name),row.name).toBe(1);expect(row.mode).toBe("fresh");}
  expect(wall,`parallel wall ${wall} ms against the sum ${sessions.reduce((sum,row)=>sum+row.result.ms,0)} ms`).toBeLessThan(sessions.reduce((sum,row)=>sum+row.result.ms,0)*0.75);
  await stopAll();

  const fourthPlan={...plan,slots:scenario.servers+1},first=sessions[scenario.warmIndex]!;
  const fourth=await withBazelSessionV1(port,fourthPlan,{language:"en",key:key(100),signal},async session=>{
   const workspace=bazelSessionWorkspaceV1(layout.workspace,session.slot,key(100));generate(workspace,[first.name]);
   return {slot:session.slot,mode:session.mode,workspace,result:await run([...startup(session.slot),"build",...command(session.slot),"//:"+first.name],workspace)};
  });
  expect(fourth.slot,"a never used slot, hence a fresh output base").toBe("s"+scenario.servers);expect(fourth.mode).toBe("fresh");expect(fourth.workspace).not.toBe(first.workspace);
  expect(fourth.result.status,fourth.result.stderr).toBe(0);expect(spawns(fourth.result.stderr)).toContain("disk cache hit");for(const runner of spawns(fourth.result.stderr))expect(scenario.allowedSpawnRunners,"no local spawn: "+runner).toContain(runner);
  expect(executions(first.name),"the action was not executed again").toBe(1);report.fourth={slot:fourth.slot,ms:fourth.result.ms,spawns:spawns(fourth.result.stderr)};
  await stopAll();

  const again=await withBazelSessionV1(port,fourthPlan,{language:"en",key:key(scenario.warmIndex),signal},async session=>({slot:session.slot,mode:session.mode,result:await run([...startup(session.slot),"build",...command(session.slot),"//:"+first.name],bazelSessionWorkspaceV1(layout.workspace,session.slot,key(scenario.warmIndex)))}));
  report.warm={slot:again.slot,mode:again.mode,ms:again.result.ms,spawns:spawns(again.result.stderr)};
  expect(again.slot,"the selection returns to the slot of its warm package").toBe(first.slot);expect(again.mode).toBe("warm");expect(again.result.status,again.result.stderr).toBe(0);expect(executions(first.name)).toBe(1);for(const runner of spawns(again.result.stderr))expect(scenario.allowedSpawnRunners).toContain(runner);
  await stopAll();

  const shared=join(root,"p/control");generate(shared,[...scenario.controlTargets]);const slot="s0";
  const [left,right]=await Promise.all(scenario.controlTargets.map(name=>run([...startup(slot),"build",...command(slot),"//:"+name],shared)));
  report.control={leftMs:left!.ms,rightMs:right!.ms,lockWait:lockWait(left!.stderr)||lockWait(right!.stderr)};
  expect(left!.status,left!.stderr).toBe(0);expect(right!.status,right!.stderr).toBe(0);expect(lockWait(left!.stderr)||lockWait(right!.stderr),"control: two commands in one output base wait for each other").toBe(true);
 }finally{for(const row of pinned)await row.release();if(process.env.SEMIO_TEST_REPORT)writeFileSync(process.env.SEMIO_TEST_REPORT,JSON.stringify(report,null,1));await stopAll();await removeBazelTreeV1(root);}
},900000);
