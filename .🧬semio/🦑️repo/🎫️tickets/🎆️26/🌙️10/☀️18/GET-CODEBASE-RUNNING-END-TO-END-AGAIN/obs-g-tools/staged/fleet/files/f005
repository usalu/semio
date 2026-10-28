import {afterAll,expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,relative,resolve} from "node:path";
import {validateFleetReportV1} from "../../🟦️.ts";

const scratch=mkdtempSync(join(tmpdir(),"fleet-guard-")),spy=join(import.meta.dir,"🪤️spy","🟦️.ts"),worker=join(import.meta.dir,"📥️worker","🟦️.ts"),holder=join(import.meta.dir,"🧲️holder","🟦️.ts"),bare=join(scratch,"bunfig.toml"),plugin=join(scratch,"loaded-modules.ts");
afterAll(()=>rmSync(scratch,{recursive:true,force:true}));
writeFileSync(bare,"");
writeFileSync(plugin,'import {readFileSync,writeFileSync} from "node:fs";const seen:string[]=[];Bun.plugin({name:"loaded-modules",setup(build){build.onLoad({filter:/\\.(ts|mjs)$/},args=>{seen.push(args.path);return {contents:readFileSync(args.path,"utf8"),loader:args.path.endsWith(".ts")?"ts":"js"};});}});process.on("exit",()=>{writeFileSync(process.env.LOADED_MODULES_OUT!,JSON.stringify(seen));});\n');

/** 🏖️ A sandbox workspace: its own lease directory, lease registry, capacity directory and agent session directory, so the command reads nothing of the real machine. */
const sandbox=(name:string)=>{
 const root=join(scratch,name),environment={SEMIO_LEASE_REGISTRY_DIRECTORY:join(root,"registry"),SEMIO_HOST_CAPACITY_DIRECTORY:join(root,"capacity"),SEMIO_AGENT_SESSION_DIRECTORY:join(root,"sessions"),SEMIO_LANGUAGE:"en",SEMIO_REPO_IMPLEMENTATION:"live"};
 for(const directory of [join(root,".cache","b","l","m","data"),environment.SEMIO_LEASE_REGISTRY_DIRECTORY,environment.SEMIO_HOST_CAPACITY_DIRECTORY,environment.SEMIO_AGENT_SESSION_DIRECTORY])mkdirSync(directory,{recursive:true});
 writeFileSync(join(root,"package.json"),'{"workspaces":[]}');writeFileSync(join(root,"📋️project.json"),"{}");
 return {root,environment,leases:join(root,".cache","b","l","m","data")};
};
/** 📸️ The names of every file and directory below a directory (sizes and times of a live holder's own files move on their own). */
const snapshot=(directory:string):string=>{
 const rows:string[]=[],walk=(path:string)=>{for(const name of readdirSync(path).sort()){const full=join(path,name);rows.push(relative(directory,full).replace(/\d+-\d+\.json$/u,"<entry>.json"));if(statSync(full).isDirectory())walk(full);}};
 if(existsSync(directory))walk(directory);return rows.join("\n");
};
const holdLease=async(box:ReturnType<typeof sandbox>,resource:string)=>{
 const child=spawn(process.execPath,["--config="+bare,holder,JSON.stringify({directory:box.leases,resource})],{env:{...process.env,...box.environment},stdio:["ignore","pipe","inherit"]});
 const pid=await new Promise<number>((done,fail)=>{child.once("error",fail);child.stdout!.once("data",chunk=>done(JSON.parse(String(chunk).trim()).pid as number));setTimeout(()=>fail(Error("holder did not acquire")),60_000);});
 for(let waited=0;waited<100&&!snapshot(box.environment.SEMIO_LEASE_REGISTRY_DIRECTORY).includes("<entry>.json");waited++)await new Promise(done=>setTimeout(done,100));
 return {pid,stop:()=>{child.kill("SIGKILL");}};
};
const runWorker=(box:ReturnType<typeof sandbox>,mode:"status"|"control",extra:string[]=[],environment:Record<string,string>={})=>{
 const out=join(scratch,`spy-${Math.random().toString(36).slice(2)}.jsonl`),modules=join(scratch,`modules-${Math.random().toString(36).slice(2)}.json`);
 const result=spawnSync(process.execPath,["--config="+bare,"--preload",spy,"--preload",plugin,worker,JSON.stringify({root:box.root,mode,directory:box.leases}),...extra],{encoding:"utf8",timeout:120_000,env:{...process.env,...box.environment,...environment,FLEET_SPY_OUT:out,LOADED_MODULES_OUT:modules}});
 expect(result.status,result.stderr.slice(0,600)).toBe(0);
 const events=existsSync(out)?readFileSync(out,"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line) as {op:string;subject:string}):[];
 return {answer:JSON.parse(result.stdout.trim().split("\n").pop()!) as {code:number;report:Record<string,any>},events,modules:JSON.parse(readFileSync(modules,"utf8")) as string[]};
};

test("the command writes nothing but the capacity sample cache, takes no lease and starts only the read-only host probes, while a real process holds an exclusive lease it must see",async()=>{
 const box=sandbox("readonly"),held=await holdLease(box,"build-graph-state:/w:selected:7");
 try{
  const registryBefore=snapshot(box.environment.SEMIO_LEASE_REGISTRY_DIRECTORY),leasesBefore=snapshot(box.leases);
  const {answer,events}=runWorker(box,"status");
  expect(answer.code).toBe(0);
  expect(validateFleetReportV1(answer.report)).toEqual([]);
  const seen=answer.report.queues.graph.builder;expect(seen.pid).toBe(held.pid);
  const writes=events.filter(event=>event.op!=="child"&&event.op!=="sqlite-open").map(event=>event.subject),capacity=box.environment.SEMIO_HOST_CAPACITY_DIRECTORY;
  expect(writes.filter(path=>!path.startsWith(capacity)),"writes outside the capacity sample cache").toEqual([]);
  expect(events.filter(event=>event.op==="sqlite-open").map(event=>event.subject),"a database was opened").toEqual([]);
  expect(writes.some(path=>path.startsWith(box.leases)||path.startsWith(box.environment.SEMIO_LEASE_REGISTRY_DIRECTORY)||path.includes("lease")),"a lease or registry path was written").toBe(false);
  expect(snapshot(box.leases)).toBe(leasesBefore);
  expect(snapshot(box.environment.SEMIO_LEASE_REGISTRY_DIRECTORY)).toBe(registryBefore);
  const children=events.filter(event=>event.op==="child").map(event=>event.subject.split(" ")[0]!.split("/").pop()!);
  expect(children.filter(command=>!["ps","vm_stat","sysctl","memory_pressure","df","lsof","powershell","pwsh","hdiutil","diskutil"].includes(command)),"a child that is no read-only probe").toEqual([]);
 }finally{held.stop();}
},180_000);

test("negative control: the spy sees a lease taken by the same worker, so the read-only law above cannot pass by blindness",()=>{
 const box=sandbox("control"),{events,answer}=runWorker(box,"control");
 expect(answer.code).toBe(0);
 expect(events.some(event=>event.op==="sqlite-open"&&event.subject.startsWith(box.leases))).toBe(true);
},180_000);

test("the command never loads the graph: no module of the graph group, the inputs module or a build invocation is loaded, the schema of the generation store is read as plain JSON",()=>{
 const box=sandbox("graphfree"),{modules,answer}=runWorker(box,"status");
 expect(answer.code).toBe(0);
 const forbidden=["/🏗️build/🕸️graph/","/⚡️caching/🕸️graph/","/🏗️build/📥️inputs/","/🏗️build/🧭️invocation/","/🏗️build/🧭️selection/","/🏗️build/📥️caller/","/🏗️build/🏡️resident/","/⚡️caching/🚀️bootstrap/","/🏗️build/🧾️store-receipt/","/🏃️process/🦀️cargo/"];
 expect(modules.filter(path=>forbidden.some(part=>path.includes(part)))).toEqual([]);
 expect(modules.some(path=>path.includes("/🚢️fleet-status/⌨️command/"))).toBe(true);
});

test("the sources of the module name no way to take a lease, start a build or load the graph",()=>{
 const root=resolve(import.meta.dir,"../.."),files:string[]=[];
 const walk=(directory:string)=>{for(const name of readdirSync(directory)){const path=join(directory,name),status=statSync(path);if(status.isDirectory()){if(name!=="🧪️tests")walk(path);}else if(/\.(ts|mjs)$/u.test(name))files.push(path);}};
 walk(root);
 expect(files.length).toBeGreaterThanOrEqual(3);
 const banned=/acquireResourceLease|acquireQueuedResourceLease|withResourceLease|withQueuedResourceLease|tryAcquire|loadGenerationGraphV1|withBuildGraph|runBuildCliV1|withHostCapacityV1|acquireBazelSessionV1|bazelSlotPortV1|\.claim\(|spawnSync|execSync|\bspawn\(/u;
 for(const file of files){const text=readFileSync(file,"utf8");expect(banned.test(text),file+": "+(banned.exec(text)?.[0]??"")).toBe(false);}
});
