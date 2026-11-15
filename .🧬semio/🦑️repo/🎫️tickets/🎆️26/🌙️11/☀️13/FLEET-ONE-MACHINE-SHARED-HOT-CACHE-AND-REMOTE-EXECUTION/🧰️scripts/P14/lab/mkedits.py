#!/usr/bin/env python3
"""P14 edit generator: reads the LIVE files, cuts every old segment by its start and end marker (each must occur exactly once), pairs it with its new text and writes ../edits.json (the anchored replacements land.py applies, every old text exactly the stated number of times) together with ../baseline.json (the sha256 of every file the payload was made from). Run again whenever a live file moved; the payload is the json, not this script.
usage: mkedits.py [--root <worktree>]"""
import os,sys,json,hashlib,re
HERE=os.path.dirname(os.path.abspath(__file__));OUT=os.path.dirname(HERE)
def arg(name):return sys.argv[sys.argv.index(name)+1] if name in sys.argv else None
ROOT=arg("--root") or next(c for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"] if os.path.exists(c+"/AGENTS.md"))
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
GS=LIB+"/🪪️graph-state";W=GS+"/🙋️wanted";PR=GS+"/📦️project-records"
EDITS=[];BASE={}
def read(rel):
    raw=open(ROOT+"/"+rel,"rb").read();BASE[rel]=hashlib.sha256(raw).hexdigest();return raw.decode("utf8")
def lit(rel,old,new,count=1):
    text=read(rel)
    if text.count(old)!=count:sys.exit("%s: old text occurs %d times, expected %d: %r"%(rel,text.count(old),count,old[:90]))
    EDITS.append({"path":rel,"kind":"literal","old":old,"new":new,"count":count})
def seg(rel,start,end,new):
    text=read(rel)
    if text.count(start)!=1:sys.exit("%s: start occurs %d times: %r"%(rel,text.count(start),start[:90]))
    i=text.index(start);j=text.find(end,i)
    if j<0:sys.exit("%s: end not found after %r"%(rel,start[:60]))
    lit(rel,text[i:j+len(end)],new)
def rx(rel,pattern,repl,count,live=None):
    text=read(rel);n=len(re.findall(pattern,text,flags=re.M))
    if n!=(count if live is None else live):sys.exit("%s: pattern matches %d times, expected %d: %s"%(rel,n,count,pattern))
    EDITS.append({"path":rel,"kind":"regex","old":pattern,"new":repl,"count":count})

WT=W+"/🟦️.ts"
lit(WT,'import{fetchGraphEntryV1,publishGraphEntryV1,type GraphFleetClientV1}from"../🛰️exchange/🟦️.ts";','import{fetchGraphEntryV1,publishGraphEntryV1,type GraphFleetClientV1}from"../🛰️exchange/🟦️.ts";\nimport{servesPlatformV1}from"../🌍️platform-class/🟦️.ts";')
seg(WT,"/** 🔑️ The exchange state a machine's request lives under",'as GraphStateV1;\n}','''/** 🔑️ The exchange state a machine's request lives under: one slot per requester, whatever its platform (the request carries the platform), so a newer request replaces the older one and every serving machine finds it by the requester alone; a slot per platform was read by machines of one platform only, and nobody read the slot of a Linux worker among macOS machines. */
export function wantedStateV1(requester:string):GraphStateV1{
 return Object.freeze({version:1,frame:Object.freeze({kind:"project-inference" as const,tooling:sha("project-wanted"),implementation:sha(requester),platform:"neutral",extra:Object.freeze({wanted:"1"})}),names:ZERO,content:ZERO,files:0,contentFiles:0,digest:sha(JSON.stringify(["project-wanted",requester]))}) as GraphStateV1;
}''')
lit(WT,'state:wantedStateV1(input.requester,input.platform)','state:wantedStateV1(input.requester)')
seg(WT,'export async function readWantedV1(input:Readonly<{client:GraphFleetClientV1;requester:string;platform:string;now?:()=>number}>)','wantedStateV1(input.requester,input.platform));','''export async function readWantedV1(input:Readonly<{client:GraphFleetClientV1;requester:string;now?:()=>number}>):Promise<WantedRequestV1|undefined>{
 const found=await fetchGraphEntryV1(input.client,wantedStateV1(input.requester));''')
lit(WT,'request.requester===input.requester&&request.platform===input.platform&&request.expiresMs','request.requester===input.requester&&request.expiresMs')
lit(WT,'known(key:string):boolean;graphKey?():Promise<string>;','known(key:string):boolean;graphKey?():Promise<string>;foreign?(request:WantedRequestV1):void;')
lit(WT,'''  const request=await readWantedV1({client:input.client,requester,platform:input.platform,...(input.now===undefined?{}:{now:input.now})});
  if(request===undefined||(input.graphKey!==undefined&&request.graphKey!==await input.graphKey()))continue;''','''  const request=await readWantedV1({client:input.client,requester,...(input.now===undefined?{}:{now:input.now})});
  if(request===undefined)continue;
  if(!servesPlatformV1(input.platform,request.platform)){input.foreign?.(request);continue;}
  if(input.graphKey!==undefined&&request.graphKey!==await input.graphKey())continue;''')
lit(WT,'the live request of the same graph state (when `graphKey` is given) whose claim','the live request of a machine of its own platform class (a request of another class goes to `foreign` and is left to a machine of that class) of the same graph state (when `graphKey` is given) whose claim')
seg(WT,'/** 🔑️ The exchange state of the answer to one requester','as GraphStateV1;\n}','''/** 🔑️ The exchange state of the answer to one requester, whatever its platform: the projects the serving machine finished for it, replaced by every newer answer. */
export function servedStateV1(requester:string):GraphStateV1{
 return Object.freeze({version:1,frame:Object.freeze({kind:"project-inference" as const,tooling:sha("project-served"),implementation:sha(requester),platform:"neutral",extra:Object.freeze({served:"1"})}),names:ZERO,content:ZERO,files:0,contentFiles:0,digest:sha(JSON.stringify(["project-served",requester]))}) as GraphStateV1;
}''')
lit(WT,'state:servedStateV1(input.request.requester,input.request.platform)','state:servedStateV1(input.request.requester)')
seg(WT,'export async function readServedV1(input:Readonly<{client:GraphFleetClientV1;requester:string;platform:string}>)','servedStateV1(input.requester,input.platform));','''export async function readServedV1(input:Readonly<{client:GraphFleetClientV1;requester:string}>):Promise<WantedServedV1|undefined>{
 const found=await fetchGraphEntryV1(input.client,servedStateV1(input.requester));''')
lit(WT,'return served.requester===input.requester&&served.platform===input.platform?served:undefined;','return served.requester===input.requester?served:undefined;')
lit(WT,'/** 🗣️ One message of the texts table in one language. */','''/** 🎚️ The memory a machine can give to inferring projects: half of its memory, less when less is available now (less the floor the others keep), and never what the small first booking of the process that asks says: the inference itself is admitted by the capacity gate later (measured: a 16 GB worker asked with 2048 MB, had a grant of 0 MB per project and a wave of 0 projects, so every project was refused), and a project that outgrows its grant is stopped by the memory governor. Swap is neither added nor subtracted: the available memory of a Linux host is `MemAvailable`, which has no swap in it. */
export function fleetAdmittedMegabytesV1(input:Readonly<{hostMegabytes:number;availableMegabytes?:number}>,policy:WantedPolicyV1=wantedPolicyV1):number{
 const half=Math.floor(input.hostMegabytes/2);
 return input.availableMegabytes===undefined||!Number.isFinite(input.availableMegabytes)?half:Math.max(0,Math.min(half,Math.floor(input.availableMegabytes-policy.availableFloorMegabytes)));
}

/** 🗣️ One message of the texts table in one language. */''')

WS=W+"/🧬️schema/🔣️.json"
lit(WS,'''        "serveHeadroomMegabytes": 9400
      }''','''        "serveHeadroomMegabytes": 9400,
        "availableFloorMegabytes": 2048
      }''')
lit(WS,'"alive": "graph-server: alive, {requesters} machines seen, {held} requests held, read {reading}",','''"alive": "graph-server: alive, {requesters} machines seen, {held} requests held, {foreign} of other platform classes left to others, read {reading}",
          "foreign": "graph-server: {count} stale projects of {requester} ({platform}) are left to a machine of the class {class}; this machine serves the class {own}",
          "class": "graph: this machine is {platform} (class {class}); only machines of that class serve its request, and their graph server logs 'serving' for it",''')
lit(WS,'"alive": "Graph-Server: lebt, {requesters} Maschinen gesehen, {held} Anfragen gehalten, Lesen {reading}",','''"alive": "Graph-Server: lebt, {requesters} Maschinen gesehen, {held} Anfragen gehalten, {foreign} anderer Plattformklassen anderen überlassen, Lesen {reading}",
          "foreign": "Graph-Server: {count} veraltete Projekte von {requester} ({platform}) bleiben einer Maschine der Klasse {class} überlassen; diese Maschine bedient die Klasse {own}",
          "class": "Graph: diese Maschine ist {platform} (Klasse {class}); ihre Anfrage bedienen nur Maschinen dieser Klasse, deren Graph-Server meldet dafür 'bediene'",''')

SV=W+"/🛠️server/🟦️.ts"
lit(SV,'import schema from"../🧬️schema/🔣️.json"with{type:"json"};','import schema from"../🧬️schema/🔣️.json"with{type:"json"};\nimport{platformClassV1}from"../../🌍️platform-class/🟦️.ts";')
lit(SV,'tick=0,seen=0,costsKey="",costsAt=-Infinity;','tick=0,seen=0,costsKey="",costsAt=-Infinity;\n const foreignNow=new Map<string,string>(),toldForeign=new Set<string>(),noteForeign=(request:WantedRequestV1)=>{foreignNow.set(request.requester,request.platform);const key=wantedClaimKeyV1(request);if(toldForeign.has(key))return;toldForeign.add(key);input.say(wantedTextV1("foreign",language,{count:request.projects.length,requester:request.requester,platform:request.platform,class:platformClassV1(request.platform),own:platformClassV1(input.platform)}));};')
lit(SV,'   if(now()>=holdUntil&&(input.mayServe===undefined||await input.mayServe())){','   foreignNow.clear();\n   if(now()>=holdUntil&&(input.mayServe===undefined||await input.mayServe())){')
lit(SV,'platform:input.platform,requesters:await input.requesters()','platform:input.platform,foreign:noteForeign,requesters:await input.requesters()')
lit(SV,'held:held.size,reading:running===undefined?"none":running.reason}','held:held.size,foreign:foreignNow.size,reading:running===undefined?"none":running.reason}')
lit(SV,'publishOwnerCostsV1({client:input.client,server:input.owner,platform:input.platform,table})','publishOwnerCostsV1({client:input.client,server:input.owner,table})')
lit(SV,'a machine with headroom. A machine that may not serve','a machine with headroom. It serves the requests of every machine of its platform class (a slot is per requester, the request names the platform) and counts the others. A machine that may not serve')

RD=W+"/📖️read/🟦️.ts"
lit(RD,'import schema from"../🧬️schema/🔣️.json"with{type:"json"};','import schema from"../🧬️schema/🔣️.json"with{type:"json"};\nimport{platformClassV1}from"../../🌍️platform-class/🟦️.ts";')
lit(RD,'readServedV1({client:input.client!,requester:input.requester,platform:input.platform})','readServedV1({client:input.client!,requester:input.requester})',2)
lit(RD,'  input.say(heavyMissing.length>0?ownerCostTextV1("refused"','  input.say(wantedTextV1("class",language,{platform:input.platform,class:platformClassV1(input.platform)}));\n  input.say(heavyMissing.length>0?ownerCostTextV1("refused"')

CT=W+"/💰️cost/🟦️.ts"
seg(CT,"/** 🔑️ The exchange state the fleet's table lives under, one slot per platform. */",'as GraphStateV1;\n}','''/** 🔑️ The exchange state the fleet's table lives under, one slot for every platform: the cost of inferring a project is a property of the project, measured by the serving machine and read by every worker. */
export function ownerCostStateV1():GraphStateV1{
 return Object.freeze({version:1,frame:Object.freeze({kind:"project-inference" as const,tooling:sha("project-costs"),implementation:sha("fleet"),platform:"neutral",extra:Object.freeze({costs:"1"})}),names:ZERO,content:ZERO,files:0,contentFiles:0,digest:sha(JSON.stringify(["project-costs"]))}) as GraphStateV1;
}''')
lit(CT,'export async function publishOwnerCostsV1(input:Readonly<{client:GraphFleetClientV1;server:string;platform:string;table:OwnerCostTableV1}>)','export async function publishOwnerCostsV1(input:Readonly<{client:GraphFleetClientV1;server:string;table:OwnerCostTableV1}>)')
lit(CT,'state:ownerCostStateV1(input.platform)','state:ownerCostStateV1()')
lit(CT,'export async function readOwnerCostsV1(input:Readonly<{client:GraphFleetClientV1;platform:string}>)','export async function readOwnerCostsV1(input:Readonly<{client:GraphFleetClientV1}>)')
lit(CT,'fetchGraphEntryV1(input.client,ownerCostStateV1(input.platform))','fetchGraphEntryV1(input.client,ownerCostStateV1())')
seg(CT,'/** 🏋️ Splits owners by weight','return{heavy:heavy.sort((a,b)=>b.megabytes-a.megabytes||(a.name<b.name?-1:a.name>b.name?1:0)),light};\n}','''/** 🏋️ Splits owners by weight: heavy is a MEASURED property of a named owner (its own recorded samples or the seed of a declared heavy owner) whose cost does not fit the grant; it is never inferred on this machine, however few they are. An owner nobody measured is light whatever the grant is (the cost of its kind or the unknown cost only orders the serving machine's work): measured on a 16 GB worker, a grant of 0 MB at a small booking turned all 517 stale projects into heavy ones, 820 MB of an unknown project being more than nothing. A light owner that outgrows the grant is stopped by the memory governor, recorded under its name and heavy from then on. The heavy ones come heaviest first. */
export function classifyOwnersV1(names:readonly string[],estimate:(name:string)=>OwnerEstimateV1,grant:OwnerGrantV1,policy:OwnerCostPolicyV1=ownerCostPolicyV1):OwnerClassesV1{
 const heavy:HeavyOwnerV1[]=[],light:string[]=[],measured=policy.heavySources as readonly string[];
 for(const name of names){const row=estimate(name);if(row.megabytes>grant.managedMegabytes&&measured.includes(row.source))heavy.push({name,megabytes:row.megabytes,source:row.source});else light.push(name);}
 return{heavy:heavy.sort((a,b)=>b.megabytes-a.megabytes||(a.name<b.name?-1:a.name>b.name?1:0)),light};
}''')

CS=W+"/💰️cost/🧬️schema/🔣️.json"
lit(CS,'''          "@semio-tech/framework-os-kernel": 3838
        },''','''          "@semio-tech/framework-os-kernel": 3838,
          "workspace": 3100
        },''')
lit(CS,'''        "flushMilliseconds": 2000
      }''','''        "flushMilliseconds": 2000,
        "heavySources": ["recorded", "seed"]
      }''')

PF=LIB+"/🕸️graph/🤝️prefetch/🟦️.ts"
lit(PF,'const half=Math.floor(memory.hostMemoryMegabytesV1()/2),explicit=Number(environment[memory.GRAPH_MEMORY_POLICY_V1.budgetEnvironment]),admittedMegabytes=Number.isFinite(explicit)&&explicit>0?Math.min(half,Math.floor(explicit/memory.GRAPH_MEMORY_POLICY_V1.grantShare)):half;','const{fleetAdmittedMegabytesV1}=await import("../../🪪️graph-state/🙋️wanted/🟦️.ts"),{hostAvailableBytesV1}=await import("../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts"),available=(input.available??hostAvailableBytesV1)(),admittedMegabytes=fleetAdmittedMegabytesV1({hostMegabytes:memory.hostMemoryMegabytesV1(),...available===undefined?{}:{availableMegabytes:Math.floor(available/1048576)}});')
lit(PF,'export async function fleetReadStaleGenerationV1(input:{root:string;request:GenerationPrefetchRequestV1;invocation:ScriptInvocation;language:"en"|"de";environment?:NodeJS.ProcessEnv}):Promise<void> {','export async function fleetReadStaleGenerationV1(input:{root:string;request:GenerationPrefetchRequestV1;invocation:ScriptInvocation;language:"en"|"de";environment?:NodeJS.ProcessEnv;available?:()=>number|undefined}):Promise<void> {')
lit(PF,'cost.readOwnerCostsV1({client,platform:graphPlatformV1()})','cost.readOwnerCostsV1({client})')
lit(PF,'The admitted memory is half of what the host has (a declared smaller machine counts as declared), lowered to the grant a slice pool load runs under when it has one (the quiet forecast grant of a command says nothing about what the machine can give):','The admitted memory is half of what the host has (a declared smaller machine counts as declared), lowered to what is available now less a floor (`fleetAdmittedMegabytesV1`), never to the grant of the slice pool load that asks (a 2048 MB booking left a 16 GB worker a grant of 0 MB per project and a wave of 0 projects):')

PRM=PR+"/🟦️.ts"
lit(PRM,'import{graphPlatformV1,type GraphStateV1}from"../🟦️.ts";','import{graphPlatformV1,type GraphStateV1}from"../🟦️.ts";\nimport{recordPlatformV1}from"../🌍️platform-class/🟦️.ts";\nimport{dependencyGenerationV1,localRowsV1,portableRowsV1,type ObservationRowV1}from"./🧳️portable/🟦️.ts";')
lit(PRM,'the digest of the record key (inference revision and the project\'s own identity) with the root replaced by the token, the platform, and one slot per project','the digest of the record key (inference revision and the project\'s own identity) with the root replaced by the token, the platform class (`posix-shared` for macOS and Linux, the exact platform for Windows and for a project whose key names a host-bound declaration), and one slot per project')
lit(PRM,'''digest=sha(JSON.stringify(["project-inference",neutral(input.keyText),input.platform,neutral(input.fleet)]));''','''platform=recordPlatformV1(input.keyText,input.platform),digest=sha(JSON.stringify(["project-inference",neutral(input.keyText),platform,neutral(input.fleet)]));''')
lit(PRM,'implementation:ZERO,platform:input.platform,extra:Object.freeze({})','implementation:ZERO,platform,extra:Object.freeze({})')
lit(PRM,'const portable=info.exportRecord();if(portable===undefined)return undefined;return[{name:"record",bytes:neutralizeRootV1(Buffer.from(JSON.stringify(portable)),input.root)}];','const portable=info.exportRecord() as {observations:readonly ObservationRowV1[]}|undefined;if(portable===undefined)return undefined;return[{name:"record",bytes:neutralizeRootV1(Buffer.from(JSON.stringify({...portable,observations:portableRowsV1(portable.observations,input.root)})),input.root)}];')
lit(PRM,'const ok=await info.import(JSON.parse(Buffer.from(expandRootV1(document.bytes,input.root)).toString("utf8")));','const shared=JSON.parse(Buffer.from(expandRootV1(document.bytes,input.root)).toString("utf8")) as {observations:readonly ObservationRowV1[]},ok=await info.import({...shared,observations:localRowsV1(shared.observations,dependencyGenerationV1(input.root))});')

# tests that exist and change their platform arguments
TESTS=W+"/🧪️tests"
lit(TESTS+"/🟦️.ts",'  expect(await readWantedV1({client:reader,requester:"g1-m03",platform:"linux-x64"})).toBeUndefined();\n','  expect((await readWantedV1({client:reader,requester:"g1-m03"}))?.platform).toBe("darwin-arm64");\n')
lit(TESTS+"/🟦️.ts",'a request is read back by requester and platform,','a request is read back by its requester alone whatever the platform of the reader,')
lit(TESTS+"/🛠️server/🟦️.ts",'expect(lines).toEqual([wantedTextV1("alive","en",{requesters:2,held:0,reading:"none"}),wantedTextV1("alive","en",{requesters:2,held:0,reading:"none"})]);','expect(lines).toEqual([wantedTextV1("alive","en",{requesters:2,held:0,foreign:0,reading:"none"}),wantedTextV1("alive","en",{requesters:2,held:0,foreign:0,reading:"none"})]);')
for rel in [TESTS+"/🟦️.ts",TESTS+"/📖️read/🟦️.ts",TESTS+"/🛠️server/🟦️.ts",TESTS+"/🛠️server/💰️priority/🟦️.ts"]:
    n=len(re.findall(r'(readWantedV1|readServedV1)\(\{([^}]*?),platform:(?:"[^"]*"|PLATFORM)',read(rel),flags=re.M))
    if n:rx(rel,r'((?:readWantedV1|readServedV1)\(\{[^}]*?),platform:(?:"[^"]*"|PLATFORM)',r'\1',n-1 if rel==TESTS+"/🟦️.ts" else n,n)
rx(TESTS+"/🛠️server/💰️priority/🟦️.ts",r'readOwnerCostsV1\(\{client:reader,platform:PLATFORM\}','readOwnerCostsV1({client:reader}',2)
CTT=W+"/💰️cost/🧪️tests/🟦️.ts"
lit(CTT,'readOwnerCostsV1({client,platform:"darwin-arm64"})','readOwnerCostsV1({client})',2)
lit(CTT,'publishOwnerCostsV1({client,server:"g1-server",platform:"darwin-arm64",table})','publishOwnerCostsV1({client,server:"g1-server",table})')
lit(CTT,'  expect(await readOwnerCostsV1({client,platform:"linux-x64"})).toBeUndefined();\n','')
lit(CTT,'expect(estimateOwnerCostV1(table,"workspace","root:language:cargo")).toEqual({megabytes:3100,source:"kind"});','expect(estimateOwnerCostV1(table,"workspace","root:language:cargo")).toEqual({megabytes:3100,source:"seed"});\n expect(estimateOwnerCostV1(table,"another-root","root:language:cargo")).toEqual({megabytes:3100,source:"kind"});')
PRT=PR+"/🧪️tests/🟦️.ts"
lit(PRT,'expect(projectRecordStateV1({...edit,platform:"linux-x64"}).digest).not.toBe(a.digest);','expect(projectRecordStateV1({...edit,platform:"linux-x64"}).digest).toBe(a.digest);expect(projectRecordStateV1({...edit,platform:"win32-x64"}).digest).not.toBe(a.digest);')
lit(PRT,'differs when the key, the platform or the project differs','differs when the key, the platform class or the project differs')

FR=LIB+"/🕸️graph/🧪️tests/🙋️fleet-read/🟦️.ts"
lit(FR,'prepare?:(root:string)=>void){','prepare?:(root:string)=>void,available?:number){')
lit(FR,'...environment}});return{lines,error:undefined};}','...environment},available:()=>available});return{lines,error:undefined};}')
json.dump(EDITS,open(OUT+"/edits.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
json.dump(BASE,open(OUT+"/baseline.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
print(len(EDITS),"edits over",len(BASE),"files")
