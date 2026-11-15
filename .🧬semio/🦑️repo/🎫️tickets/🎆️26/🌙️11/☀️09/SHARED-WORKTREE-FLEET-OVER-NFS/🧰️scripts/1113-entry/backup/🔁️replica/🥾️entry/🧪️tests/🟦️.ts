import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {createHash} from "node:crypto";
import {chmodSync,copyFileSync,cpSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {basename,dirname,extname,join} from "node:path";
import {codeDigestV1} from "../../../🩺️attest/🟦️.ts";
import {sealCodeV1} from "../../../🛡️resilience/🟦️.ts";
import {applyQueue,closeReplicaV1,followOnceV1,joinV1,openReplicaV1,publishOnceV1,runV1,seedV1,type ReplicaEventV1,type ReplicaOptionsV1} from "../../🟦️.ts";
import {admitEntryScriptV1,entryDirectoryV1,entryGuardedV1,entryScriptPathV1,maintainEntryV1,migrateAgentV1,readEntryStateV1,requireEntryShapeV1} from "../🟦️.ts";

setDefaultTimeout(180_000);
const sourceDirectory=join(import.meta.dirname,".."),trampoline=join(sourceDirectory,"..","📜️script.ts"),supervisorFile=join(sourceDirectory,"..","🛟️supervisor","🟦️.ts");
const relative="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree";
const scratch:string[]=[],running:Bun.Subprocess[]=[];
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`entry-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const proc of running)try{proc.kill("SIGKILL");}catch{}for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const put=(root:string,path:string,text:string):void=>{const file=join(root,...path.split("/"));mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);};
const sha=(data:string|Uint8Array):string=>createHash("sha256").update(data).digest("hex");
const until=async(condition:()=>boolean|Promise<boolean>,milliseconds=30_000):Promise<void>=>{const end=Date.now()+milliseconds;while(!await condition()){if(Date.now()>end)throw Error("Timed out");await Bun.sleep(20);}};
const fixture=JSON.parse(readFileSync(join(import.meta.dirname,"🧫️fixtures","🔣️.json"),"utf8")) as {directories:{platform:string;home:string;env:Record<string,string>;expected:string}[];hashes:{text:string;sha256:string}[];decisions:{name:string;installed:number|null;source:number;identical:boolean;damaged:boolean;selfTestBroken:boolean;expected:string}[];guards:{name:string;source:string;admitted:boolean}[]};
const program=readdirSync(sourceDirectory).find(name=>extname(name)===".mjs")!;
const cleanEnvironment=(extra:Record<string,string>={}):Record<string,string>=>{const environment:Record<string,string>={};for(const [name,value] of Object.entries(process.env))if(value!==undefined&&!name.startsWith("SEMIO_FLEET_")&&name!=="SEMIO_WORKSPACE_ROOT")environment[name]=value;return {...environment,...extra};};

const makeSource=(directory:string,options:Readonly<{version?:number;variant?:string;broken?:boolean}>={}):string=>{
 mkdirSync(directory,{recursive:true});
 for(const name of readdirSync(sourceDirectory)){
  if(![".mjs",".sh",".ps1"].includes(extname(name)))continue;
  let text=readFileSync(join(sourceDirectory,name),"utf8");
  if(extname(name)===".mjs"){
   if(options.version!==undefined)text=text.replace("export const bootVersionV1=1;",`export const bootVersionV1=${options.version};`);
   if(options.variant!==undefined)text+=`\n// ${options.variant}\n`;
   if(options.broken===true){expect(text).toContain("process.on(\"SIGTERM\",()=>{process.exit(0);});");text=text.replace("process.on(\"SIGTERM\",()=>{process.exit(0);});","process.on(\"SIGTERM\",()=>{process.exit(9);});");}
  }
  writeFileSync(join(directory,name),text);
 }
 return join(directory,program);
};
const installWith=(source:string,entry:string,worktree:string,store:string|null,...extra:string[]):{status:number;result:{action:string;reason?:string}}=>{
 const done=Bun.spawnSync([process.execPath,source,"install","--worktree",worktree,...store===null?[]:["--store",store],...extra],{env:cleanEnvironment({SEMIO_FLEET_ENTRY_DIR:entry}),stdout:"pipe",stderr:"pipe"});
 const out=done.stdout.toString().trim().split("\n").filter(line=>line.startsWith("{")).pop();
 return {status:done.exitCode??-1,result:out===undefined?{action:"none",reason:done.stderr.toString()}:JSON.parse(out) as {action:string;reason?:string}};
};

const workingSupervisor=`import {mkdirSync,writeFileSync} from "node:fs";\nimport {join} from "node:path";\nconst root=process.env.SEMIO_FLEET_BOOT_ROOT;\nmkdirSync(join(root,".cache","f","boot"),{recursive:true});\nwriteFileSync(join(root,"ran.txt"),process.env.SEMIO_FLEET_BOOT_SOURCE+" "+(process.env.SEMIO_FLEET_BOOT_BASE??""));\nwriteFileSync(join(root,".cache","f","boot","ready.json"),JSON.stringify({pid:process.pid,waiting:false}));\nprocess.on("SIGTERM",()=>{process.exit(0);});\nsetInterval(()=>{},1000);\n`;
const failingSupervisor="import \"./does-not-exist.ts\";\nexport const x=1;\n";
const writeSnapshot=(codeDirectory:string,seed:string,body:string,tamper=false):string=>{
 const digest=sha(seed),directory=join(codeDirectory,digest),path=`${relative}/🔁️replica/🛟️supervisor/🟦️.ts`;
 put(directory,`files/${path}`,tamper?`${body}// tampered\n`:body);
 writeFileSync(join(directory,"sealed.json"),JSON.stringify({version:1,digest,createdMs:Date.now(),moduleRoot:relative,files:[{path,bytes:Buffer.byteLength(body),sha256:sha(body)}]}));
 return digest;
};

type Sandbox=Readonly<{base:string;repo:string;entry:string;store:string;source:string}>;
const sandbox=(label:string,options:Readonly<{supervisor?:string|null;script?:string;store?:boolean;worktree?:boolean}>={}):Sandbox=>{
 const base=temp(label),repo=join(base,"wt"),entry=join(base,"entry"),store=join(base,"store");
 if(options.worktree!==false){
  if(options.supervisor!==null)put(repo,`${relative}/🔁️replica/🛟️supervisor/🟦️.ts`,options.supervisor??workingSupervisor);
  put(repo,`${relative}/🔁️replica/📜️script.ts`,options.script??"export const fine=1;\n");
  put(repo,".cache/f/machine.json",JSON.stringify({version:1,originId:"w1",groupId:"g1",storeRoot:store,key:"0".repeat(32)}));
 }
 if(options.store===true)mkdirSync(join(store,"v1","bootstrap","code"),{recursive:true});
 const source=makeSource(join(base,"src"));
 const installed=installWith(source,entry,repo,store);
 expect(installed.result.action,installed.result.reason).toBe("installed");
 return {base,repo,entry,store,source};
};

const startShim=(box:Sandbox,extra:Record<string,string>={}):{proc:Bun.Subprocess<"ignore","pipe","pipe">;out:()=>string}=>{
 const proc=Bun.spawn(["sh",join(box.entry,"boot.sh"),"run"],{stdout:"pipe",stderr:"pipe",cwd:box.base,env:cleanEnvironment({SEMIO_FLEET_ENTRY_DIR:box.entry,SEMIO_FLEET_ENTRY_EARLY_MS:"5000",SEMIO_FLEET_ENTRY_READY_MS:"20000",SEMIO_FLEET_ENTRY_POLL_MS:"50",SEMIO_FLEET_ENTRY_BACKOFF_MS:"100",SEMIO_FLEET_ENTRY_BACKOFF_MAX_MS:"200",SEMIO_FLEET_ENTRY_GRACE_MS:"3000",SEMIO_FLEET_ENTRY_STORE_MS:"2000",PATH:`${dirname(process.execPath)}:${process.env.PATH??""}`,...extra})});
 running.push(proc);
 let out="";
 void (async()=>{for await(const chunk of proc.stdout)out+=Buffer.from(chunk).toString();})();
 void (async()=>{for await(const chunk of proc.stderr)out+=Buffer.from(chunk).toString();})();
 return {proc,out:()=>out};
};
const stopShim=async(shim:{proc:Bun.Subprocess<"ignore","pipe","pipe">}):Promise<number|null>=>{shim.proc.kill("SIGTERM");const code=await Promise.race([shim.proc.exited,Bun.sleep(15_000).then(()=>"hung" as const)]);if(code==="hung"){shim.proc.kill("SIGKILL");throw Error("the shim did not stop");}return code;};
const ranOf=(box:Sandbox):string=>existsSync(join(box.repo,"ran.txt"))?readFileSync(join(box.repo,"ran.txt"),"utf8"):"";
const phaseOf=(box:Sandbox):string|undefined=>readEntryStateV1(box.entry)?.phase;

const moduleCopy=(label:string):{repo:string;module:string}=>{
 const repo=temp(label),module=join(repo,...relative.split("/")),live=join(import.meta.dirname,"..","..","..","..","..","..","..");
 mkdirSync(join(repo,"🧰️framework","🛍️products","🦑️repo","🔨️modules"),{recursive:true});
 symlinkSync(join(live,"📦️packages"),join(repo,"🧰️framework","📦️packages"));
 const modules=join(import.meta.dirname,"..","..","..","..");
 for(const item of readdirSync(modules))if(item!=="🤝️shared-worktree")symlinkSync(join(modules,item),join(repo,"🧰️framework","🛍️products","🦑️repo","🔨️modules",item));
 symlinkSync(join(live,"🔨️modules"),join(repo,"🧰️framework","🔨️modules"));
 cpSync(join(import.meta.dirname,"..","..",".."),module,{recursive:true,filter:source=>!source.includes("node_modules")});
 return {repo,module};
};

test("the install directory follows the language-agnostic vectors for every operating system, and the digest vectors equal sha256",()=>{
 for(const item of fixture.directories)expect(entryDirectoryV1(item.env,item.platform,item.home)).toBe(item.expected);
 for(const item of fixture.hashes)expect(sha(item.text)).toBe(item.sha256);
 const boot=readFileSync(join(sourceDirectory,program),"utf8");
 expect(boot).toContain("export function entryDirectoryV1(");
 expect(entryDirectoryV1({},"darwin","/Users/x")).not.toContain("Documents");
});

test("provisioning installs the boot program: three files, a manifest that matches the schema and the bytes, a worktree file, and verify agrees",()=>{
 const box=sandbox("install");
 for(const name of ["boot.mjs","boot.sh","boot.ps1","worktree","entry.json"])expect(existsSync(join(box.entry,name))).toBe(true);
 const manifest=JSON.parse(readFileSync(join(box.entry,"entry.json"),"utf8")) as {files:Record<string,{sha256:string;bytes:number}>;worktree:string};
 requireEntryShapeV1("EntryManifest",manifest,"manifest");
 for(const [name,item] of Object.entries(manifest.files)){expect(sha(readFileSync(join(box.entry,name)))).toBe(item.sha256);expect(readFileSync(join(box.entry,name)).length).toBe(item.bytes);}
 expect(readFileSync(join(box.entry,"worktree"),"utf8").trim()).toBe(box.repo);
 const verified=Bun.spawnSync([process.execPath,join(box.entry,"boot.mjs"),"verify"],{env:cleanEnvironment({SEMIO_FLEET_ENTRY_DIR:box.entry}),stdout:"pipe"});
 expect(JSON.parse(verified.stdout.toString())).toEqual({ok:true,problems:[]});
 writeFileSync(join(box.entry,"boot.mjs"),`${readFileSync(join(box.entry,"boot.mjs"),"utf8")}\n//x`);
 const damaged=Bun.spawnSync([process.execPath,join(box.entry,"boot.mjs"),"verify"],{env:cleanEnvironment({SEMIO_FLEET_ENTRY_DIR:box.entry}),stdout:"pipe"});
 expect(damaged.exitCode).toBe(1);expect(JSON.parse(damaged.stdout.toString()).problems).toEqual(["boot.mjs differs"]);
});

test("a scratch worktree never redirects the boot program of the machine, only provisioning takes it over",()=>{
 const box=sandbox("foreign"),other=join(box.base,"other");
 put(other,".cache/f/machine.json","{}");
 const before=readFileSync(join(box.entry,"entry.json"),"utf8");
 const refused=installWith(box.source,box.entry,other,null);
 expect(refused.result.action).toBe("refused-foreign");expect(refused.status).toBe(3);
 expect(readFileSync(join(box.entry,"entry.json"),"utf8")).toBe(before);expect(readFileSync(join(box.entry,"worktree"),"utf8").trim()).toBe(box.repo);
 const taken=installWith(box.source,box.entry,other,null,"--take-over");
 expect(taken.result.action).toBe("current");
 expect(readFileSync(join(box.entry,"worktree"),"utf8").trim()).toBe(other);
 rmSync(join(other,".cache"),{recursive:true,force:true});
 expect(installWith(box.source,box.entry,box.repo,null).result.action).toBe("current");
});

test("update decisions follow the language-agnostic table: newer installs, never a downgrade or a same-version change, a failing self-test refuses, a damaged installation is repaired",()=>{
 for(const item of fixture.decisions){
  const base=temp("decision"),entry=join(base,"entry"),repo=join(base,"wt");
  mkdirSync(repo,{recursive:true});
  if(item.installed!==null){
   const first=makeSource(join(base,"first"),{version:item.installed,variant:item.identical&&item.installed===item.source?undefined:"installed"});
   expect(installWith(first,entry,repo,null).result.action,item.name).toBe("installed");
   if(item.damaged)writeFileSync(join(entry,"boot.mjs"),`${readFileSync(join(entry,"boot.mjs"),"utf8")}\n// damaged\n`);
  }
  const second=makeSource(join(base,"second"),{version:item.source,broken:item.selfTestBroken});
  const before=existsSync(join(entry,"entry.json"))?readFileSync(join(entry,"entry.json"),"utf8"):"",bytes=existsSync(join(entry,"boot.mjs"))?readFileSync(join(entry,"boot.mjs"),"utf8"):"";
  const done=installWith(second,entry,repo,null);
  expect(done.result.action,`${item.name}: ${done.result.reason??""}`).toBe(item.expected);
  if(item.expected.startsWith("refused")){if(before!==""){expect(readFileSync(join(entry,"entry.json"),"utf8")).toBe(before);expect(readFileSync(join(entry,"boot.mjs"),"utf8")).toBe(bytes);}else expect(existsSync(join(entry,"boot.mjs"))).toBe(false);expect(done.status).toBe(3);}
  if(item.expected==="installed"&&item.installed!==null&&!item.damaged)expect(sha(readFileSync(join(entry,"boot.prev.mjs")))).toBe(sha(bytes));
 }
});

test("an update whose self-test fails is refused through the daemon call, leaves the installed program and manifest byte for byte, and a good newer one installs next with the previous kept",async()=>{
 const box=sandbox("refuse"),module=join(box.base,"module");
 const manifestBefore=readFileSync(join(box.entry,"entry.json"),"utf8"),programBefore=readFileSync(join(box.entry,"boot.mjs"),"utf8");
 const place=(directory:string,source:string):void=>{mkdirSync(directory,{recursive:true});for(const name of readdirSync(dirname(source)))if([".mjs",".sh",".ps1"].includes(extname(name)))copyFileSync(join(dirname(source),name),join(directory,name));};
 place(join(module,"🔁️replica","🥾️entry"),makeSource(join(box.base,"bad"),{version:2,broken:true}));
 const refused=await maintainEntryV1({worktree:box.repo,moduleRoot:module,storeRoot:box.store,entryDirectory:box.entry});
 expect(refused.action).toBe("refused-selftest");expect(refused.reason).toContain("scenario");
 expect(readFileSync(join(box.entry,"entry.json"),"utf8")).toBe(manifestBefore);expect(readFileSync(join(box.entry,"boot.mjs"),"utf8")).toBe(programBefore);
 rmSync(join(module,"🔁️replica","🥾️entry"),{recursive:true});
 place(join(module,"🔁️replica","🥾️entry"),makeSource(join(box.base,"good"),{version:2}));
 const installed=await maintainEntryV1({worktree:box.repo,moduleRoot:module,storeRoot:box.store,entryDirectory:box.entry});
 expect(installed.action).toBe("installed");
 const manifest=JSON.parse(readFileSync(join(box.entry,"entry.json"),"utf8")) as {bootVersion:number;previous:{sha256:string}};
 expect(manifest.bootVersion).toBe(2);expect(manifest.previous.sha256).toBe(sha(programBefore));expect(sha(readFileSync(join(box.entry,"boot.prev.mjs")))).toBe(sha(programBefore));
 const again=await maintainEntryV1({worktree:box.repo,moduleRoot:module,storeRoot:box.store,entryDirectory:box.entry});
 expect(again.action).toBe("current");
 rmSync(join(module,"🔁️replica","🥾️entry"),{recursive:true});
 place(join(module,"🔁️replica","🥾️entry"),makeSource(join(box.base,"old"),{version:1}));
 expect((await maintainEntryV1({worktree:box.repo,moduleRoot:module,storeRoot:box.store,entryDirectory:box.entry})).action).toBe("refused-downgrade");
});

test("the service shim runs the worktree supervisor although the worktree script 📜️script.ts is garbage, and stops on SIGTERM",async()=>{
 const box=sandbox("garbage",{script:"export const x=;\n@@@ not a program\n"});
 const shim=startShim(box);
 await until(()=>ranOf(box).startsWith("worktree")&&phaseOf(box)==="running");
 requireEntryShapeV1("EntryState",readEntryStateV1(box.entry),"state");
 expect(shim.proc.exitCode).toBeNull();
 const code=await stopShim(shim);
 expect([0,143]).toContain(code);
});

test("a worktree supervisor with a broken static import falls back to the newest sealed snapshot, which runs with its reason shown in both languages",async()=>{
 const box=sandbox("static",{supervisor:failingSupervisor});
 const digest=writeSnapshot(join(box.repo,".cache","f","code"),"sealed-1",workingSupervisor);
 const shim=startShim(box);
 await until(()=>ranOf(box).startsWith("snapshot")&&phaseOf(box)==="running");
 const state=readEntryStateV1(box.entry)!;
 expect(state.source).toBe("snapshot");expect(state.digest).toBe(digest);
 expect(state.attempts.map(item=>item.source)).toEqual(["worktree"]);expect(state.attempts[0]!.outcome).toStartWith("early");
 expect(shim.out()).toContain("startet nicht");
 await stopShim(shim);
});

test("a missing worktree is a visible waiting state that never exits, and the program starts the code when the worktree appears",async()=>{
 const box=sandbox("missing",{worktree:false});
 const shim=startShim(box);
 await until(()=>phaseOf(box)==="waiting");
 expect(readEntryStateV1(box.entry)!.reason).toContain("does not exist");
 await Bun.sleep(1200);
 expect(shim.proc.exitCode).toBeNull();expect(phaseOf(box)).toBe("waiting");
 put(box.repo,`${relative}/🔁️replica/🛟️supervisor/🟦️.ts`,workingSupervisor);
 put(box.repo,".cache/f/machine.json",JSON.stringify({version:1,originId:"w1",groupId:"g1",storeRoot:box.store,key:"0".repeat(32)}));
 await until(()=>ranOf(box).startsWith("worktree")&&phaseOf(box)==="running");
 await stopShim(shim);
});

test("with the store unreachable and no local code the program keeps polling with a state file, and runs the snapshot of the store as soon as the store answers",async()=>{
 const box=sandbox("unreachable",{supervisor:failingSupervisor});
 const shim=startShim(box);
 await until(()=>phaseOf(box)==="exhausted");
 const state=readEntryStateV1(box.entry)!;
 expect(state.nextMs).toBeGreaterThan(Date.now()-1000);expect(state.reason.length).toBeGreaterThan(0);
 await Bun.sleep(1200);
 expect(shim.proc.exitCode).toBeNull();
 const wall=readEntryStateV1(box.entry)!.wallMs;
 await Bun.sleep(300);
 expect(existsSync(join(box.entry,"code"))).toBe(false);
 const digest=writeSnapshot(join(box.store,"v1","bootstrap","code"),"store-1",workingSupervisor);
 await until(()=>ranOf(box).startsWith("store")&&phaseOf(box)==="running");
 expect(readEntryStateV1(box.entry)!.digest).toBe(digest);expect(readEntryStateV1(box.entry)!.wallMs).toBeGreaterThanOrEqual(wall);
 expect(existsSync(join(box.entry,"code",digest,"sealed.json"))).toBe(true);
 await stopShim(shim);
});

test("a corrupt store snapshot (hash mismatch) is never run or installed, and a valid one next to it is used",async()=>{
 const box=sandbox("corrupt",{supervisor:failingSupervisor,store:true});
 const bad=writeSnapshot(join(box.store,"v1","bootstrap","code"),"corrupt",workingSupervisor,true);
 const shim=startShim(box);
 await until(()=>phaseOf(box)==="exhausted");
 await Bun.sleep(400);
 expect(ranOf(box)).toBe("");expect(existsSync(join(box.entry,"code",bad))).toBe(false);
 expect(shim.out()).toContain("hashes differ");
 const good=writeSnapshot(join(box.store,"v1","bootstrap","code"),"valid",workingSupervisor);
 await until(()=>ranOf(box).startsWith("store")&&phaseOf(box)==="running");
 expect(readEntryStateV1(box.entry)!.digest).toBe(good);expect(existsSync(join(box.entry,"code",bad))).toBe(false);
 await stopShim(shim);
});

test("a snapshot that fails early is refused for a while and not run again, while the worktree code is tried again every round",async()=>{
 const box=sandbox("refused",{supervisor:failingSupervisor,store:true});
 const bad=writeSnapshot(join(box.repo,".cache","f","code"),"bad",failingSupervisor);
 const shim=startShim(box);
 await until(()=>(readEntryStateV1(box.entry)?.attempts.filter(item=>item.source==="worktree").length??0)>=2);
 const attempts=readEntryStateV1(box.entry)!.attempts;
 expect(attempts.filter(item=>item.digest===bad)).toHaveLength(1);
 expect(JSON.parse(readFileSync(join(box.entry,"refused.json"),"utf8")).digests[bad]).toBeGreaterThan(Date.now());
 await stopShim(shim);
});

test("the trampoline hands run over to the installed program of its own worktree, and uses the supervisor for every other command, a foreign worktree, a damaged installation and a program that fails at once",async()=>{
 const base=temp("trampoline"),repo=join(base,"wt"),entry=join(base,"entry");
 const module=join(repo,...relative.split("/")),script=join(module,"🔁️replica","📜️script.ts");
 mkdirSync(dirname(script),{recursive:true});copyFileSync(trampoline,script);
 put(module,"🔁️replica/🛟️supervisor/🟦️.ts","export async function scriptMainV1(argv:readonly string[]):Promise<number>{process.stdout.write(`supervisor ${argv.join(\" \")}\\n`);return 0;}\n");
 const install=(shimText:string,worktree:string):void=>{
  mkdirSync(entry,{recursive:true});
  const files:Record<string,string>={"boot.mjs":"// boot\n","boot.sh":shimText,"boot.ps1":"# boot\n"};
  for(const [name,text] of Object.entries(files))writeFileSync(join(entry,name),text);
  chmodSync(join(entry,"boot.sh"),0o755);
  writeFileSync(join(entry,"entry.json"),JSON.stringify({version:1,bootVersion:1,files:Object.fromEntries(Object.entries(files).map(([name,text])=>[name,{sha256:sha(text),bytes:Buffer.byteLength(text)}])),worktree,module:relative,store:null,installedMs:1}));
 };
 const run=(...args:string[]):{out:string;code:number}=>{const done=Bun.spawnSync([process.execPath,script,...args],{cwd:base,env:cleanEnvironment({SEMIO_FLEET_ENTRY_DIR:entry}),stdout:"pipe",stderr:"pipe"});return {out:done.stdout.toString(),code:done.exitCode??-1};};
 expect(run("run").out).toBe("supervisor run\n");
 install("#!/bin/sh\necho \"booted $@ $SEMIO_FLEET_ENTRY\"\nexit 0\n",repo);
 expect(run("run").out).toBe("booted run trampoline\n");
 expect(run("status").out).toBe("supervisor status\n");
 expect(run("--boot-child","run").out).toBe("supervisor --boot-child run\n");
 install("#!/bin/sh\necho \"booted $@\"\nexit 75\n",repo);
 expect(run("run")).toEqual({out:"booted run\n",code:75});
 install("#!/bin/sh\necho failing\nexit 3\n",repo);
 expect(run("run").out).toBe("failing\nsupervisor run\n");
 install("#!/bin/sh\necho \"booted\"\n",join(base,"other"));
 expect(run("run").out).toBe("supervisor run\n");
 install("#!/bin/sh\necho \"booted\"\n",repo);
 writeFileSync(join(entry,"boot.sh"),"#!/bin/sh\necho tampered\n");
 expect(run("run").out).toBe("supervisor run\n");
});

test("the guard admits a trampoline that parses and passes its self-test and refuses every other file, and the real trampoline passes it",async()=>{
 for(const item of fixture.guards){
  const file=join(temp("guard"),"script.ts");
  writeFileSync(file,item.source);
  const why=await admitEntryScriptV1(file,{timeoutMs:20_000});
  expect(why===undefined,`${item.name}: ${why??"admitted"}`).toBe(item.admitted);
 }
 expect(await admitEntryScriptV1(trampoline)).toBeUndefined();
 expect(entryGuardedV1(entryScriptPathV1)).toBe(true);expect(entryGuardedV1(`${relative}/🔁️replica/🟦️.ts`)).toBe(false);
 expect(readFileSync(trampoline,"utf8").match(/\bfrom\s*"([^"]+)"/gu)!.every(item=>item.includes("\"node:"))).toBe(true);
 expect(readFileSync(trampoline,"utf8").length).toBeLessThan(16*1024);
});

test("a daemon refuses an incoming change of the trampoline that does not pass the guard, keeps the previous content, reports the guard, and applies a good change afterwards",async()=>{
 const store=temp("store"),a=temp("a"),b=temp("b");
 const good=readFileSync(trampoline,"utf8"),path=entryScriptPathV1;
 put(a,path,good);
 const optionsA:ReplicaOptionsV1={worktree:a,storeRoot:store,originId:"a1",groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false},optionsB:ReplicaOptionsV1={worktree:b,storeRoot:store,originId:"b1",groupId:"g1",pollMilliseconds:10,persistMilliseconds:0,watch:false};
 await seedV1(optionsA);mkdirSync(b,{recursive:true});await joinV1(optionsB);
 expect(readFileSync(join(b,...path.split("/")),"utf8")).toBe(good);
 const ra=await openReplicaV1(optionsA),events:ReplicaEventV1[]=[],rb=await openReplicaV1(optionsB,event=>events.push(event));
 put(a,path,"import {x} from \"left-pad\";\nexport const broken=;\n");await publishOnceV1(ra);
 await followOnceV1(rb);await applyQueue(rb,undefined);
 expect(readFileSync(join(b,...path.split("/")),"utf8")).toBe(good);
 const refusal=events.find(event=>event.event==="refused"&&event.path===path);
 expect(refusal?.reason).toStartWith("entry-guard:");
 expect(rb.failures.summary().map(item=>item.kind)).toContain("entry-guard");
 put(a,path,`${good}// a harmless comment\n`);await publishOnceV1(ra);
 await followOnceV1(rb);await applyQueue(rb,undefined);
 expect(readFileSync(join(b,...path.split("/")),"utf8")).toBe(`${good}// a harmless comment\n`);
 await closeReplicaV1(ra);await closeReplicaV1(rb);
});

test("the LaunchAgent of a macOS account is pointed at the shim by its own daemon: only the exact rendered shape, linted, the previous file kept, and a second run changes nothing",()=>{
 const home=temp("home"),entry=join(home,"Library","Application Support","semio","fleet","entry"),file=join(home,"Library","LaunchAgents","tech.semio.fleet.replica.plist");
 const old=`<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">\n<plist version="1.0">\n<dict>\n<key>Label</key><string>tech.semio.fleet.replica</string>\n<key>LimitLoadToSessionType</key><string>Aqua</string>\n<key>WorkingDirectory</key><string>/Users/w/Documents/semio/r</string>\n<key>ProgramArguments</key><array><string>/usr/local/bin/repo</string><string>./📜️script.ts</string><string>run</string></array>\n<key>RunAtLoad</key><true/>\n<key>KeepAlive</key><true/>\n</dict>\n</plist>\n`;
 put(home,"Library/LaunchAgents/tech.semio.fleet.replica.plist",old);
 expect(migrateAgentV1({home,entryDirectory:entry,platform:"linux"})).toBe("absent");
 expect(migrateAgentV1({home,entryDirectory:entry,platform:"darwin",lint:process.platform==="darwin"})).toBe("rewritten");
 const next=readFileSync(file,"utf8");
 expect(next).toContain(`<array><string>/bin/sh</string><string>${join(entry,"boot.sh")}</string><string>run</string></array>`);
 expect(next).not.toContain("WorkingDirectory");expect(next).not.toContain("script.ts");expect(next).toContain("<key>KeepAlive</key><true/>");
 expect(readFileSync(join(entry,"agent.plist.previous"),"utf8")).toBe(old);
 if(process.platform==="darwin")expect(Bun.spawnSync(["plutil","-lint",file]).exitCode).toBe(0);
 expect(migrateAgentV1({home,entryDirectory:entry,platform:"darwin"})).toBe("current");
 writeFileSync(file,old.replace("<string>run</string>","<string>run</string><string>extra</string>"));
 expect(migrateAgentV1({home,entryDirectory:entry,platform:"darwin"})).toBe("unrecognised");
});

test("the real module end to end: a broken worktree script and a broken static import in the supervisor do not stop the daemon, which runs from its sealed snapshot and attests the fallback",async()=>{
 const {repo,module}=moduleCopy("module"),store=temp("store"),entry=join(temp("entry"),"e");
 put(repo,"a.txt","alpha\n");
 const options:ReplicaOptionsV1={worktree:repo,storeRoot:store,originId:"w1",groupId:"g1",pollMilliseconds:20,scanSeconds:5,persistMilliseconds:0};
 await seedV1(options);
 const check=Bun.spawnSync([process.execPath,join(module,"🔁️replica","📜️script.ts"),"self-check","--lang","en"],{cwd:repo,env:cleanEnvironment(),stdout:"pipe",stderr:"pipe"});
 expect(check.exitCode,check.stdout.toString()+check.stderr.toString()).toBe(0);
 const digest=await codeDigestV1(module);
 expect((await sealCodeV1({root:repo,moduleRoot:module,digest,digestOf:codeDigestV1})).created).toBe(true);
 const installed=await maintainEntryV1({worktree:repo,moduleRoot:module,storeRoot:store,entryDirectory:entry});
 expect(installed.action).toBe("installed");
 writeFileSync(join(module,"🔁️replica","📜️script.ts"),"export const x=;\n");
 writeFileSync(join(module,"🔁️replica","🛟️supervisor","🟦️.ts"),"import \"./does-not-exist.ts\";\n");
 const proc=Bun.spawn(["sh",join(entry,"boot.sh"),"run"],{cwd:repo,stdout:"pipe",stderr:"pipe",env:cleanEnvironment({SEMIO_FLEET_ENTRY_DIR:entry,PATH:`${dirname(process.execPath)}:${process.env.PATH??""}`,SEMIO_FLEET_ENTRY_EARLY_MS:"20000",SEMIO_FLEET_ENTRY_BACKOFF_MS:"200"})});
 running.push(proc);
 let out="";void (async()=>{for await(const chunk of proc.stdout)out+=Buffer.from(chunk).toString();})();void (async()=>{for await(const chunk of proc.stderr)out+=Buffer.from(chunk).toString();})();
 await until(()=>{try{const attestation=JSON.parse(readFileSync(join(store,"v1","origins","w1","attest.json"),"utf8")) as {fallback?:{guard:string;source:string}};return attestation.fallback?.guard==="code-fallback";}catch{return false;}},120_000).catch(error=>{throw Error(`${error.message}\nstate ${JSON.stringify(readEntryStateV1(entry))}\noutput ${out.slice(-3000)}`);});
 const attestation=JSON.parse(readFileSync(join(store,"v1","origins","w1","attest.json"),"utf8")) as {code:string;fallback:{source:string}};
 expect(attestation.code).toBe(digest);expect(attestation.fallback.source).toBe("snapshot");
 expect(readEntryStateV1(entry)!.source).toBe("snapshot");
 proc.kill("SIGTERM");
 expect(await Promise.race([proc.exited,Bun.sleep(30_000).then(()=>"hung")])).not.toBe("hung");
 void basename;void out;
});

test("a healthy daemon installs the boot program of the code it runs, once at start, and reports a refused update as the failure kind entry-boot",async()=>{
 const {repo,module}=moduleCopy("healthy"),store=temp("store"),entry=join(temp("entry"),"e");
 put(repo,"a.txt","alpha\n");
 const options:ReplicaOptionsV1={worktree:repo,storeRoot:store,originId:"h1",groupId:"g1",pollMilliseconds:10,scanSeconds:1,debounceMilliseconds:5,persistMilliseconds:0,attestMilliseconds:100,restartStableMilliseconds:100,restartJitterMilliseconds:0,minimumUptimeMilliseconds:0,viewMilliseconds:300,viewDebounceMilliseconds:50,logFiles:[],codeRoot:module,gitState:async()=>({present:true,head:"refs/heads/main",commit:"a".repeat(40),indexTree:"c".repeat(40)}),gitView:async()=>null,selfCheck:async()=>({ok:true}),entryDirectory:entry,entryMilliseconds:400};
 await seedV1(options);
 const controller=new AbortController(),events:ReplicaEventV1[]=[],done=runV1(options,controller.signal,event=>events.push(event));
 await until(()=>existsSync(join(entry,"entry.json")),60_000);
 requireEntryShapeV1("EntryManifest",JSON.parse(readFileSync(join(entry,"entry.json"),"utf8")),"manifest");
 await until(()=>events.some(event=>event.event==="sync"&&event.stage==="entry"),30_000);
 const source=join(module,"🔁️replica","🥾️entry"),mjs=readdirSync(source).find(name=>extname(name)===".mjs")!;
 writeFileSync(join(source,mjs),readFileSync(join(source,mjs),"utf8").replace("export const bootVersionV1=1;","export const bootVersionV1=2;").replace("process.on(\"SIGTERM\",()=>{process.exit(0);});","process.on(\"SIGTERM\",()=>{process.exit(9);});"));
 await until(()=>events.some(event=>event.event==="error"&&event.stage==="entry"),60_000);
 expect(events.find(event=>event.event==="error"&&event.stage==="entry")!.message).toContain("refused-selftest");
 expect(JSON.parse(readFileSync(join(entry,"entry.json"),"utf8")).bootVersion).toBe(1);
 controller.abort();await done;
});
