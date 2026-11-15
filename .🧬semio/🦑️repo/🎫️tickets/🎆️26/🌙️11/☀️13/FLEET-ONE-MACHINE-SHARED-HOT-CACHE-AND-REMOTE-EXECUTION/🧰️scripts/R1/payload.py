import os,json
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
PUB=LIB+"/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication"
FUNDING=PUB+"/⚙️setup/💰️funding/🟦️.ts"
SETUP=PUB+"/⚙️setup/🟦️.ts"
TESTFILE=PUB+"/⚙️setup/🧪️tests/⏳️busy-funding/🟦️.ts"
TEXTS=LIB+"/⚡️caching/📖️texts/🔣️.json"
TESTS=[TESTFILE,LIB+"/⚡️caching/📖️texts/🧪️tests/🟦️.ts"]
REGISTRY="../../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts"

FUNDING_IMPORTS_OLD='import{openRepositoryCachePartitionV1}from"../../../♻️reset/🟦️.ts";'
FUNDING_IMPORTS_NEW='import{spawnSync}from"node:child_process";\nimport{readdirSync,readFileSync,rmSync}from"node:fs";\nimport{leaseProcessStartedAtMsV1,leaseRegistryFolderV1,leaseRegistryRootV1,type LeaseRegistryEntryV1}from"'+REGISTRY+'";\nimport{repositoryCacheImagePublicationOptionsV1}from"../../🟦️.ts";\n'+FUNDING_IMPORTS_OLD

CONST_OLD='progressMilliseconds:10000,limitMilliseconds:dependencyInstallSchema.$defs.SourceOwnerStoreSyncProtocol.const.maximumQueueWaitMilliseconds});'
CONST_NEW='progressMilliseconds:10000,ancestorGraceMilliseconds:15000,limitMilliseconds:dependencyInstallSchema.$defs.SourceOwnerStoreSyncProtocol.const.maximumQueueWaitMilliseconds});'

PORTS_OLD='export type DependencyStoreWaitPortsV1=Readonly<{now():number;sleep(milliseconds:number,signal:AbortSignal):Promise<void>;say(seconds:number,limit:number):void;limitMilliseconds:number}>;'
PORTS_NEW='''/** 🪪️ One process that holds the dependency store lease, as the lease registry published it: who (`pid`, `command`), since when (`sinceMs`), in which mode, and whether it is this process or one of its ancestors, whose release this process can never wait for. */
export type DependencyStoreHolderV1=Readonly<{pid:number;command:string;sinceMs:number;mode:"shared"|"exclusive";ancestor:boolean}>;
/** 👁️ What one look at the registry found: the holders that are alive, and those whose process is gone and whose entry was removed by that look. */
export type DependencyStoreHoldersV1=Readonly<{live:readonly DependencyStoreHolderV1[];gone:readonly DependencyStoreHolderV1[]}>;
export type DependencyStoreWaitPortsV1=Readonly<{now():number;sleep(milliseconds:number,signal:AbortSignal):Promise<void>;say(seconds:number,limit:number,holder?:DependencyStoreHolderV1):void;limitMilliseconds:number;observe?():DependencyStoreHoldersV1;sayGone?(holder:DependencyStoreHolderV1):void}>;
/** 🔌️ What the holder lookup touches outside its decisions: the registry entries of one lease directory with their files, process liveness, the ancestors of this process, the removal of an entry and the process start time. */
export type DependencyStoreRegistryPortsV1=Readonly<{entries(directory:string):readonly Readonly<{path:string;entry:LeaseRegistryEntryV1}>[];alive(pid:number):boolean;ancestors():readonly number[];remove(path:string):void;pid:number;startedAtMs:number}>;
/** 🧬️ Walks the parents of a process upward, nearest first, until a root, a repeat or the bound; a parent that cannot be read ends the walk. */
export function processAncestorsV1(parentOf:(pid:number)=>number|undefined,start:number,limit=64):readonly number[]{
  const found:number[]=[];
  for(let pid=start;Number.isSafeInteger(pid)&&pid>1&&!found.includes(pid)&&found.length<limit;pid=parentOf(pid)??0)found.push(pid);
  return Object.freeze(found);
}
const parentOfProcessV1=(pid:number):number|undefined=>{
  const run=process.platform==="win32"?spawnSync("powershell",["-NoProfile","-Command","(Get-CimInstance Win32_Process -Filter \\"ProcessId="+pid+"\\").ParentProcessId"],{encoding:"utf8"}):spawnSync("ps",["-o","ppid=","-p",String(pid)],{encoding:"utf8",env:{...process.env,LC_ALL:"C"}}),value=Number.parseInt(run.stdout,10);
  return run.status===0&&Number.isSafeInteger(value)?value:undefined;
};
const processAliveV1=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};
const registryPortsV1:DependencyStoreRegistryPortsV1=Object.freeze({
  entries:directory=>{
    const folder=leaseRegistryFolderV1(leaseRegistryRootV1(),directory),rows:Array<Readonly<{path:string;entry:LeaseRegistryEntryV1}>>=[];
    let names:string[];try{names=readdirSync(folder).filter(name=>/^\\d+-\\d+\\.json$/u.test(name));}catch{return rows;}
    for(const name of names)try{const path=join(folder,name),entry=JSON.parse(readFileSync(path,"utf8")) as LeaseRegistryEntryV1;if(entry?.version===1&&Number.isSafeInteger(entry.pid)&&Array.isArray(entry.holds)&&entry.directory===directory)rows.push(Object.freeze({path,entry}));}catch{}
    return rows;
  },
  alive:processAliveV1,
  ancestors:()=>processAncestorsV1(parentOfProcessV1,process.ppid),
  remove:path=>{try{rmSync(path,{force:true});}catch{}},
  pid:process.pid,
  startedAtMs:leaseProcessStartedAtMsV1,
});
/** 🕵️ Reads who holds one lease from the registry: a holder is an ancestor when its process is this process or one of its parents and it started before this process (a reused pid of a dead holder is not an ancestor); the entry of a holder whose process is gone is removed at once, so a claim that nobody can release does not outlive its process and is reported by name. */
export function observeDependencyStoreHoldersV1(directory:string,resource:string,ports:DependencyStoreRegistryPortsV1=registryPortsV1):DependencyStoreHoldersV1{
  const parents=new Set<number>(ports.ancestors()),live:DependencyStoreHolderV1[]=[],gone:DependencyStoreHolderV1[]=[];
  for(const{path,entry}of ports.entries(directory)){
    const hold=entry.holds.find(row=>row.resource===resource);if(!hold)continue;
    const self=entry.pid===ports.pid,holder=(ancestor:boolean):DependencyStoreHolderV1=>Object.freeze({pid:entry.pid,command:entry.command.slice(0,160),sinceMs:hold.sinceMs,mode:hold.mode,ancestor});
    if(!self&&!ports.alive(entry.pid)){ports.remove(path);gone.push(holder(false));continue;}
    live.push(holder(self||parents.has(entry.pid)&&entry.startedAtMs<=ports.startedAtMs));
  }
  return Object.freeze({live:Object.freeze(live.sort((left,right)=>left.sinceMs-right.sinceMs)),gone:Object.freeze(gone)});
}
const holderValuesV1=(holder:DependencyStoreHolderV1)=>({pid:holder.pid,command:holder.command,since:new Date(holder.sinceMs).toISOString()});'''

DEFAULT_OLD='say:(seconds,limit)=>sayCachingV1("dependency-store.waiting",{seconds,limit}),limitMilliseconds:DEPENDENCY_STORE_WAIT_V1.limitMilliseconds});'
DEFAULT_NEW='''say:(seconds,limit,holder)=>holder?sayCachingV1("dependency-store.waiting-for-holder",{seconds,limit,...holderValuesV1(holder)}):sayCachingV1("dependency-store.waiting",{seconds,limit}),sayGone:holder=>sayCachingV1("dependency-store.holder-gone",holderValuesV1(holder),"warn"),limitMilliseconds:DEPENDENCY_STORE_WAIT_V1.limitMilliseconds});
/** 🧭️ The wait ports for one dependency store: the defaults plus a look at the registry for the holder of its image lease; the lease is named lazily, so a lookup that cannot even name it simply finds no holder. */
export function dependencyStoreWaitPortsForV1(lease:()=>Readonly<{directory:string;resource:string}>,registry:DependencyStoreRegistryPortsV1=registryPortsV1):DependencyStoreWaitPortsV1{
  return Object.freeze({...dependencyStoreWaitPorts,observe:()=>{try{const named=lease();return observeDependencyStoreHoldersV1(named.directory,named.resource,registry);}catch{return Object.freeze({live:Object.freeze([]),gone:Object.freeze([])});}}});
}'''

LOOP_OLD='''  const started=ports.now();let step=DEPENDENCY_STORE_WAIT_V1.firstMilliseconds,said:number|undefined;
  for(;;){
    checkResourceLeaseControl(control);const selected=await attempt();if(selected.acquired)return selected.value;
    const waited=ports.now()-started;if(waited>=ports.limitMilliseconds)throw refuseCachingV1("timeout","dependency-store.busy-timeout",{seconds:Math.round(waited/1000)});
    if(said===undefined||waited-said>=DEPENDENCY_STORE_WAIT_V1.progressMilliseconds){ports.say(Math.round(waited/1000),Math.round(ports.limitMilliseconds/1000));said=waited;}'''
LOOP_NEW='''  const started=ports.now();let step=DEPENDENCY_STORE_WAIT_V1.firstMilliseconds,said:number|undefined,ancestorSince:number|undefined;
  for(;;){
    checkResourceLeaseControl(control);const selected=await attempt();if(selected.acquired)return selected.value;
    const now=ports.now(),waited=now-started,seen=ports.observe?.(),live=seen?.live??[];
    for(const holder of seen?.gone??[])ports.sayGone?.(holder);
    if(live.length>0&&live.every(holder=>holder.ancestor)){ancestorSince??=now;if(now-ancestorSince>=DEPENDENCY_STORE_WAIT_V1.ancestorGraceMilliseconds)throw refuseCachingV1("refused","dependency-store.ancestor-holds",{seconds:Math.round((now-ancestorSince)/1000),...holderValuesV1(live[0]!)});}else ancestorSince=undefined;
    if(waited>=ports.limitMilliseconds)throw refuseCachingV1("timeout","dependency-store.busy-timeout",{seconds:Math.round(waited/1000)});
    if(said===undefined||waited-said>=DEPENDENCY_STORE_WAIT_V1.progressMilliseconds){ports.say(Math.round(waited/1000),Math.round(ports.limitMilliseconds/1000),live.find(holder=>!holder.ancestor)??live[0]);said=waited;}'''

PARENT_OLD='return retryWhileDependencyStoreBusyV1(control,()=>attemptFundingParentV1(invocation,control,body));}'
PARENT_NEW='return retryWhileDependencyStoreBusyV1(control,()=>attemptFundingParentV1(invocation,control,body),dependencyStoreWaitPortsForV1(()=>{const options=repositoryCacheImagePublicationOptionsV1(invocation,control,"dependencies");return{directory:options.leaseDirectory,resource:"native-fixed-image:"+options.directory};}));}'

SETUP_IMPORT_OLD='retryWhileDependencyStoreBusyV1}from"./💰️funding/🟦️.ts";'
SETUP_IMPORT_NEW='retryWhileDependencyStoreBusyV1,dependencyStoreWaitPortsForV1}from"./💰️funding/🟦️.ts";'
SETUP_CALL_OLD='return prepared.acquired===true?Object.freeze({acquired:true as const,value:prepared}):Object.freeze({acquired:false as const});});checkResourceLeaseControl(control);'
SETUP_CALL_NEW='return prepared.acquired===true?Object.freeze({acquired:true as const,value:prepared}):Object.freeze({acquired:false as const});},dependencyStoreWaitPortsForV1(()=>({directory:options.leaseDirectory,resource:"native-fixed-image:"+options.directory})));checkResourceLeaseControl(control);'

TEXTS_OLD='''    "dependency-store.busy-timeout": {'''
TEXTS_NEW='''    "dependency-store.ancestor-holds": {
      "en": "Process {pid} ({command}) started this command and has held the dependency store since {since}; it cannot let go while this command runs, so waiting for it would never end ({seconds} s). Run the setup of the dependency store outside that process",
      "de": "Der Prozess {pid} ({command}) hat diesen Befehl gestartet und hält den Abhängigkeitsspeicher seit {since}; er kann ihn nicht freigeben, solange dieser Befehl läuft, das Warten darauf würde nie enden ({seconds} s). Die Einrichtung des Abhängigkeitsspeichers muss außerhalb dieses Prozesses laufen"
    },
    "dependency-store.busy-timeout": {'''
TEXTS2_OLD='''    "dependency-store.waiting": {'''
TEXTS2_NEW='''    "dependency-store.holder-gone": {
      "en": "Process {pid} ({command}) held the dependency store since {since} but no longer exists; its claim was removed",
      "de": "Der Prozess {pid} ({command}) hielt den Abhängigkeitsspeicher seit {since}, existiert aber nicht mehr; sein Anspruch wurde entfernt"
    },
    "dependency-store.waiting": {'''
TEXTS3_OLD='''    "headroom.retention-ledger-invalid": {'''
TEXTS3_NEW='''    "dependency-store.waiting-for-holder": {
      "en": "Process {pid} ({command}) holds the dependency store since {since}; waiting for it ({seconds} s of at most {limit} s)",
      "de": "Der Prozess {pid} ({command}) hält den Abhängigkeitsspeicher seit {since}; es wird darauf gewartet ({seconds} s von höchstens {limit} s)"
    },
    "headroom.retention-ledger-invalid": {'''

TEST_OLD='expect(text).toContain("retryWhileDependencyStoreBusyV1(control,()=>attemptFundingParentV1(invocation,control,body))");'
TEST_NEW='expect(text).toContain("retryWhileDependencyStoreBusyV1(control,()=>attemptFundingParentV1(invocation,control,body),dependencyStoreWaitPortsForV1(");expect(setup).toContain("dependencyStoreWaitPortsForV1(()=>({directory:options.leaseDirectory,resource:\\"native-fixed-image:\\"+options.directory}))");'

EDITS=[
 (FUNDING,FUNDING_IMPORTS_OLD,FUNDING_IMPORTS_NEW),
 (FUNDING,CONST_OLD,CONST_NEW),
 (FUNDING,PORTS_OLD,PORTS_NEW),
 (FUNDING,DEFAULT_OLD,DEFAULT_NEW),
 (FUNDING,LOOP_OLD,LOOP_NEW),
 (FUNDING,PARENT_OLD,PARENT_NEW),
 (SETUP,SETUP_IMPORT_OLD,SETUP_IMPORT_NEW),
 (SETUP,SETUP_CALL_OLD,SETUP_CALL_NEW),
 (TEXTS,TEXTS_OLD,TEXTS_NEW),
 (TEXTS,TEXTS2_OLD,TEXTS2_NEW),
 (TEXTS,TEXTS3_OLD,TEXTS3_NEW),
 (TESTFILE,TEST_OLD,TEST_NEW),
]

TEST_FS_OLD='import{mkdirSync,mkdtempSync,realpathSync,rmSync}from"node:fs";'
TEST_FS_NEW='import{existsSync,mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";\nimport{execSync,spawn}from"node:child_process";\nimport{leaseRegistryFolderV1}from"../../../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts";'
TEST_IMPORT_OLD='import{DEPENDENCY_STORE_WAIT_V1,retryWhileDependencyStoreBusyV1,type DependencyStoreWaitPortsV1}from"../../💰️funding/🟦️.ts";'
TEST_IMPORT_NEW='import{DEPENDENCY_STORE_WAIT_V1,dependencyStoreWaitPortsForV1,observeDependencyStoreHoldersV1,processAncestorsV1,retryWhileDependencyStoreBusyV1,type DependencyStoreHolderV1,type DependencyStoreHoldersV1,type DependencyStoreRegistryPortsV1,type DependencyStoreWaitPortsV1}from"../../💰️funding/🟦️.ts";'
TEST_END_OLD='expect(DEPENDENCY_STORE_WAIT_V1.limitMilliseconds).toBe(21600000);});'
TEST_END_NEW=TEST_END_OLD+r'''
const who=(over:Partial<DependencyStoreHolderV1>={}):DependencyStoreHolderV1=>({pid:4242,command:"bun script.ts repo run workspace:setup",sinceMs:Date.UTC(2026,10,15,1,0,0),mode:"shared",ancestor:false,...over});
function watched(rows:(call:number)=>DependencyStoreHoldersV1){const fake=clock(),gone:DependencyStoreHolderV1[]=[],named:Array<DependencyStoreHolderV1|undefined>=[];let call=0;const ports:DependencyStoreWaitPortsV1={...fake.ports,say:(_seconds,_limit,holder)=>{named.push(holder);},observe:()=>rows(call++),sayGone:holder=>{gone.push(holder);}};return{ports,fake,gone,named};}
async function caught(run:()=>Promise<unknown>):Promise<{key:string;code:string;values:Record<string,string|number>}>{try{await run();}catch(error){expect(isRepoErrorV1(error)).toBe(true);return error as never;}throw Error("a refusal was expected");}
test("an ancestor that holds the store is never waited for: after the grace the wait ends with a bilingual refusal that names the ancestor",async()=>{const w=watched(()=>({live:[who({ancestor:true})],gone:[]}));const error=await caught(()=>retryWhileDependencyStoreBusyV1(control(),async()=>busy,w.ports));expect(error.key).toBe("caching.dependency-store.ancestor-holds");expect(error.code).toBe("refused");expect(error.values.pid).toBe(4242);expect(error.values.command).toBe("bun script.ts repo run workspace:setup");expect(error.values.since).toBe("2026-11-15T01:00:00.000Z");expect(w.fake.at()).toBeGreaterThanOrEqual(DEPENDENCY_STORE_WAIT_V1.ancestorGraceMilliseconds);expect(w.fake.at()).toBeLessThan(DEPENDENCY_STORE_WAIT_V1.ancestorGraceMilliseconds+3*DEPENDENCY_STORE_WAIT_V1.maximumMilliseconds);for(const [language,text] of [["en","started this command"],["de","hat diesen Befehl gestartet"]] as const){const line=renderRepoErrorV1(error as never,language);expect(line).toContain("4242");expect(line).toContain("bun script.ts repo run workspace:setup");expect(line).toContain(text);}});
test("an ancestor that lets go within the grace is waited for like any other holder",async()=>{const w=watched(call=>({live:call<5?[who({ancestor:true})]:[],gone:[]}));let calls=0;expect(await retryWhileDependencyStoreBusyV1(control(),async()=>++calls<=6?busy:got("done"),w.ports)).toBe("done");expect(w.fake.at()).toBeLessThan(DEPENDENCY_STORE_WAIT_V1.ancestorGraceMilliseconds);});
test("a sibling next to an ancestor keeps the wait alive past the grace and is the holder the progress line names",async()=>{const sibling=who({pid:777,command:"bun script.ts repo sync"}),w=watched(()=>({live:[who({ancestor:true}),sibling],gone:[]}));let last=0;const value=await retryWhileDependencyStoreBusyV1(control(),async()=>{last=w.fake.at();return last>=2*DEPENDENCY_STORE_WAIT_V1.ancestorGraceMilliseconds?got("done"):busy;},w.ports);expect(value).toBe("done");expect(w.named.length).toBeGreaterThanOrEqual(2);for(const holder of w.named)expect(holder?.pid).toBe(777);});
test("the wait line names the holder by pid, command and start time in both languages",async()=>{const events:ProgressEventV1[]=[],registry:DependencyStoreRegistryPortsV1={entries:()=>[{path:"/registry/4242-1.json",entry:{version:1,pid:4242,startedAtMs:1,updatedAtMs:1,command:"bun script.ts repo sync --all",directory:"/leases",holds:[{resource:"native-fixed-image:/img",mode:"shared",sinceMs:Date.UTC(2026,10,15,1,0,0),task:1}],waits:[],standing:{version:1}as never}}],alive:()=>true,ancestors:()=>[],remove:()=>{throw Error("a live holder must stay");},pid:1,startedAtMs:10};let calls=0;await withProgressSinkV1(event=>events.push(event),()=>retryWhileDependencyStoreBusyV1(control(),async()=>++calls<2?busy:got(1),dependencyStoreWaitPortsForV1(()=>({directory:"/leases",resource:"native-fixed-image:/img"}),registry)));expect(events).toHaveLength(1);const event=events[0]!;expect(event.key).toBe("caching.dependency-store.waiting-for-holder");expect(event.values).toMatchObject({pid:4242,command:"bun script.ts repo sync --all",since:"2026-11-15T01:00:00.000Z",seconds:0});expect(renderProgressV1(event,"en")).toContain("Process 4242 (bun script.ts repo sync --all) holds the dependency store since 2026-11-15T01:00:00.000Z");expect(renderProgressV1(event,"de")).toContain("Der Prozess 4242 (bun script.ts repo sync --all) hält den Abhängigkeitsspeicher seit 2026-11-15T01:00:00.000Z");});
test("a holder whose process is gone is reclaimed at once: its entry is removed and the next attempt is not delayed by it",async()=>{const events:ProgressEventV1[]=[],removed:string[]=[];let present=true;const registry:DependencyStoreRegistryPortsV1={entries:()=>present?[{path:"/registry/9-1.json",entry:{version:1,pid:9,startedAtMs:1,updatedAtMs:1,command:"bun script.ts repo tools setup all",directory:"/leases",holds:[{resource:"native-fixed-image:/img",mode:"exclusive",sinceMs:Date.UTC(2026,10,14,23,0,0),task:1}],waits:[],standing:{version:1}as never}}]:[],alive:()=>false,ancestors:()=>[],remove:path=>{removed.push(path);present=false;},pid:1,startedAtMs:10};let calls=0;await withProgressSinkV1(event=>events.push(event),()=>retryWhileDependencyStoreBusyV1(control(),async()=>++calls<2?busy:got(1),dependencyStoreWaitPortsForV1(()=>({directory:"/leases",resource:"native-fixed-image:/img"}),registry)));expect(removed).toEqual(["/registry/9-1.json"]);const gone=events.filter(event=>event.key==="caching.dependency-store.holder-gone");expect(gone).toHaveLength(1);expect(gone[0]!.level).toBe("warn");expect(renderProgressV1(gone[0]!,"en")).toContain("Process 9 (bun script.ts repo tools setup all) held the dependency store since 2026-11-14T23:00:00.000Z but no longer exists");expect(renderProgressV1(gone[0]!,"de")).toContain("existiert aber nicht mehr");expect(events.filter(event=>event.key.startsWith("caching.dependency-store.waiting"))).toHaveLength(1);expect(events.find(event=>event.key==="caching.dependency-store.waiting")).toBeDefined();});
test("the ancestors of a process are its parents nearest first, and the walk stops at a root, a loop, an unreadable parent and the bound",()=>{const parents:Record<number,number>={10:9,9:8,8:1};expect(processAncestorsV1(pid=>parents[pid],10)).toEqual([10,9,8]);expect(processAncestorsV1(pid=>({3:4,4:3}as Record<number,number>)[pid],3)).toEqual([3,4]);expect(processAncestorsV1(()=>undefined,50)).toEqual([50]);expect(processAncestorsV1(pid=>pid+1,100,5)).toEqual([100,101,102,103,104]);});
test("the real process ancestry equals an independent walk with ps, and the real registry classifies an ancestor, a stranger, this process and a dead holder",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"r1-holders-")),previous=process.env.SEMIO_LEASE_REGISTRY_DIRECTORY,directory=join(root,"leases"),resource="native-fixed-image:"+join(root,"image"),stranger=spawn("sleep",["30"],{stdio:"ignore"}),dead=spawn("true",[],{stdio:"ignore"});
 try{
  await new Promise(done=>dead.once("exit",done));expect(process.ppid).toBeGreaterThan(1);
  const independent:number[]=[];for(let pid=process.ppid;pid>1&&!independent.includes(pid)&&independent.length<64;pid=Number.parseInt(execSync("ps -o ppid= -p "+pid,{encoding:"utf8"}),10))independent.push(pid);
  expect(processAncestorsV1(pid=>{try{return Number.parseInt(execSync("ps -o ppid= -p "+pid,{encoding:"utf8"}),10);}catch{return undefined;}},process.ppid)).toEqual(independent);
  process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=join(root,"registry");const folder=leaseRegistryFolderV1(join(root,"registry"),directory);mkdirSync(folder,{recursive:true});
  const write=(pid:number,startedAtMs:number,command:string,mode:"shared"|"exclusive")=>writeFileSync(join(folder,pid+"-"+startedAtMs+".json"),JSON.stringify({version:1,pid,startedAtMs,updatedAtMs:Date.now(),command,directory,standing:{version:1},holds:[{resource,mode,sinceMs:1000+pid,task:1}],waits:[]}));
  write(process.ppid,1,"the parent","shared");write(stranger.pid!,1,"the stranger","shared");write(process.pid,1,"this process","shared");write(dead.pid!,1,"the dead","exclusive");
  const first=observeDependencyStoreHoldersV1(directory,resource);
  expect(first.live.map(row=>[row.command,row.ancestor]).sort()).toEqual([["the parent",true],["the stranger",false],["this process",true]].sort());
  expect(first.gone.map(row=>[row.pid,row.command,row.mode])).toEqual([[dead.pid,"the dead","exclusive"]]);
  expect(existsSync(join(folder,dead.pid+"-1.json"))).toBe(false);expect(existsSync(join(folder,process.ppid+"-1.json"))).toBe(true);
  expect(observeDependencyStoreHoldersV1(directory,resource).gone).toEqual([]);
  write(process.ppid,Date.now()+3600000,"a later process with the pid of the parent","shared");
  expect(observeDependencyStoreHoldersV1(directory,resource).live.find(row=>row.command.startsWith("a later"))?.ancestor).toBe(false);
  expect(observeDependencyStoreHoldersV1(directory,"native-fixed-image:/other").live).toEqual([]);
 }finally{stranger.kill();if(previous===undefined)delete process.env.SEMIO_LEASE_REGISTRY_DIRECTORY;else process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=previous;rmSync(root,{recursive:true,force:true});}
});'''
EDITS+=[
 (TESTFILE,TEST_FS_OLD,TEST_FS_NEW),
 (TESTFILE,TEST_IMPORT_OLD,TEST_IMPORT_NEW),
 (TESTFILE,TEST_END_OLD,TEST_END_NEW),
]
