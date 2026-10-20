import {createHash} from "node:crypto";
import {createRequire} from "node:module";
import {lstatSync,readFileSync,readdirSync,readlinkSync,realpathSync} from "node:fs";
import {cpus,loadavg} from "node:os";
import {dirname,join,relative,resolve} from "node:path";

const[corePath,cjsPath,root,storeRoot,closureListPath,implementationRoot=root]=process.argv.slice(2) as [string,string,string,string,string,string?];
const core=await import(corePath),glue=await import(corePath.replace(/[^/]*$/u,"")+"💾️storage/"+corePath.replace(/.*\//u,"")),cjs=createRequire(import.meta.url)(cjsPath) as {withCurrentBunWorkspaceOwnershipV1:Function;readListingProjection:(root:string,path:string)=>readonly {name:string;kind:string}[]};
const ms=(start:number)=>Math.round((performance.now()-start)*10)/10,median=(values:number[])=>[...values].sort((a,b)=>a-b)[Math.floor(values.length/2)]!;
const out:Record<string,unknown>={host:{cpus:cpus().length,loadavgAtStart:loadavg().map(value=>Math.round(value*10)/10)},root,storeRoot};
const sha=(data:string|Uint8Array)=>createHash("sha256").update(data).digest("hex");

const abort=new AbortController();let operations=0;const counts:Record<string,number>={};
let started=performance.now();
const walkOnce=()=>cjs.withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},{signal:abort.signal,advance:async(progress:{operation:string})=>{operations++;counts[progress.operation]=(counts[progress.operation]??0)+1;}},async(custody:{scopes:{manifest:string;packages:string[]}[];observed:()=>unknown})=>({scopes:custody.scopes,observed:custody.observed()}));
const first=await walkOnce() as {scopes:{manifest:string;packages:string[]}[];observed:{paths:{path:string;state:unknown[];list?:unknown[]}[];texts:{path:string;sha256:string}[];root:string}};
const walkMs=ms(started);
out.oldMembershipWalk={milliseconds:walkMs,operations,operationsByKind:counts,note:"one withCurrentBunWorkspaceOwnershipV1 (initial derivation, fence, final validate) with an empty callback; ensureBuildEnvironmentV1 performs this twice"};
const observed=first.observed;
out.retained={directories:observed.paths.filter(row=>row.list).length,stateRows:observed.paths.filter(row=>!row.list).length,texts:observed.texts.length,scopes:first.scopes.length,packages:first.scopes.reduce((total,scope)=>total+scope.packages.length,0)};

const files:string[]=[],directories=new Set<string>();
for(const scope of first.scopes){const base=dirname(resolve(root,scope.manifest));directories.add(base);files.push(join(base,"package.json"),join(base,"bun.lock"));for(const path of scope.packages){files.push(resolve(root,path,"package.json"));directories.add(resolve(root,path));}}
const unique=[...new Set(files)].filter(path=>{try{return lstatSync(path).isFile();}catch{return false;}});
const sources=unique.map(path=>{const facts=lstatSync(path);return{path,kind:"file" as const,dev:String(facts.dev),ino:String(facts.ino),mode:facts.mode,bytes:readFileSync(path)};});
out.inputs={files:sources.length,bytes:sources.reduce((total,row)=>total+row.bytes.length,0)};

const pairs:{sourceDirectory:string;targetDirectory:string}[]=[];
for(const directory of directories){const link=join(directory,"node_modules");try{if(lstatSync(link).isSymbolicLink()){const target=realpathSync(link);if(target.startsWith(storeRoot))pairs.push({sourceDirectory:link,targetDirectory:target});}}catch{}}
const observe=(selected:readonly {sourceDirectory:string;targetDirectory:string}[])=>JSON.stringify(selected.map(pair=>{const rows:unknown[]=[];let directory=storeRoot;for(const part of relative(storeRoot,pair.targetDirectory).split("/")){directory=join(directory,part);const facts=lstatSync(directory);rows.push([directory,String(facts.dev),String(facts.ino),facts.mode]);}const link=lstatSync(pair.sourceDirectory);if(!link.isSymbolicLink()||realpathSync(pair.sourceDirectory)!==pair.targetDirectory)throw Error("binding");rows.push([pair.sourceDirectory,String(link.dev),String(link.ino),link.mode,pair.targetDirectory]);return rows;}));
const generationOf=(path:string)=>/^(.*\/j\/[a-f0-9]{20})(?:\/|$)/u.exec(path)?.[1];
const generations=[...new Set(pairs.map(pair=>generationOf(pair.targetDirectory)).filter((value):value is string=>!!value))];
const fingerprint=(path:string)=>{const facts=lstatSync(path,{bigint:true});return[facts.size,facts.ino,sha(readFileSync(path))].join(":");};
const readyPaths=generations.map(generation=>join(generation,"bindings.json")).filter(path=>{try{return lstatSync(path).isFile();}catch{return false;}});
out.bindings={pairs:pairs.length,generationRoots:generations.length,readyFiles:readyPaths.length};

const keys=["dev","ino","mode","nlink","size","mtimeNs","ctimeNs"] as const;
const rowOf=(path:string,full:boolean)=>{const facts=lstatSync(path,{bigint:true,throwIfNoEntry:false});const kind=!facts?"missing":facts.isSymbolicLink()?"link":facts.isDirectory()?"directory":facts.isFile()?"file":"foreign";return{path,kind,anchor:facts?core.generationAnchorV1(facts):null,stamp:facts?core.generationStampV1(facts):null,target:kind==="link"?readlinkSync(path):null,full:full||kind==="link"};};
const providers:string[]=[];const limit=131072;
const add=(path:string)=>{if(providers.length<limit)providers.push(path);};
const generationRoots=new Set<string>(generations);
outer:for(const generation of generationRoots){const roots=[join(generation,"node_modules")];try{for(const slot of readdirSync(join(generation,"w")))roots.push(join(generation,"w",slot,"node_modules"));}catch{}
 for(const modules of roots){let names:string[]=[];try{names=readdirSync(modules);}catch{continue;}for(const name of names){if(name.startsWith(".")&&name!==".bin")continue;add(join(modules,name));if(name.startsWith("@")){try{for(const child of readdirSync(join(modules,name))){add(join(modules,name,child));add(join(modules,name,child,"package.json"));}}catch{}}else add(join(modules,name,"package.json"));if(providers.length>=limit)break outer;}}}
const observations=providers.map(path=>rowOf(path,path.endsWith("package.json")));
out.execution={observedPaths:observations.length,populationNote:"every entry and manifest of the isolated provider roots of the current generations, capped at the schema bound maximumObservations (131072); the real walk records a subset of this population"};

const control=(counter:{ticks:number;checks:number})=>({signal:abort.signal,check:()=>{counter.checks++;},tick:async()=>{counter.ticks++;}});
const runtime={bun:String(process.versions.bun),revision:String((globalThis as {Bun?:{revision?:string}}).Bun?.revision??""),node:process.versions.node,platform:process.platform,arch:process.arch,sha256:"a".repeat(64)};
const closurePaths=JSON.parse(readFileSync(closureListPath,"utf8")) as string[],implementationFile=glue.javascriptDependencyGenerationImplementationFileV1(implementationRoot);
const closureFiles=closurePaths.map(path=>{const facts=lstatSync(join(implementationRoot,path),{bigint:true,throwIfNoEntry:false});const live=implementationFile(path);return facts&&live?{path,sha256:live.sha256,bytes:live.bytes,mtimeNs:String(facts.mtimeNs),ctimeNs:String(facts.ctimeNs)}:undefined;}).filter((row):row is NonNullable<typeof row>=>row!==undefined);
const implementation={entries:core.generationProtocolV1.implementation.entries,loadedAtMs:Date.now()+60000,files:closureFiles};
const ambient={env:process.env,home:process.env.HOME??"/",config:process.env.XDG_CONFIG_HOME??join(process.env.HOME??"/",".config")};
const producerOf=()=>glue.observeJavascriptDependencyGenerationProducerV1(root,storeRoot,["."],ambient);
out.implementationClosure={files:closureFiles.length,bytes:closureFiles.reduce((total,row)=>total+row.bytes,0)};
const scopeRead="0".repeat(64);
started=performance.now();
const counter={ticks:0,checks:0};
const captured=await core.captureGenerationReceiptV1({workspace:root,storeRoot,scope:".",startedAtMs:Date.now()+60000,implementation,runtime,producer:producerOf(),recipeSha256:"c".repeat(64),ownershipSha256:"d".repeat(64),required:["."],inputs:[{scope:".",sources}],membership:observed,bindings:[{scope:".",pairs,observation:observe(pairs)}],ready:readyPaths.slice(0,1).map(path=>({scope:".",generation:"e".repeat(64),path,fingerprint:fingerprint(path)})),execution:{scopes:[{scope:".",sha256:"f".repeat(64)}],runtimeSha256:runtime.sha256,observations},scopeReadSha256:scopeRead,control:control(counter)});
const captureMs=ms(started);
if(captured.status!=="captured"){out.capture={status:captured.status,reason:captured.reason,path:captured.path};console.log(JSON.stringify(out,null,1));process.exit(0);}
const evidence=new Map<string,Uint8Array>(captured.files.map((file:{name:string;bytes:Uint8Array})=>[file.name,file.bytes]));
out.capture={milliseconds:captureMs,frontierBytes:evidence.get(captured.receipt.membership.frontier.name)!.length,executionBytes:evidence.get(captured.receipt.execution.evidence.name)!.length,receiptBytes:Buffer.byteLength(JSON.stringify(captured.receipt)),frontierRows:captured.receipt.membership.frontier.rows,executionRows:captured.receipt.execution.evidence.rows,unsettledDirectories:new TextDecoder().decode(evidence.get(captured.receipt.membership.frontier.name)!).split("\n").filter(Boolean).filter(line=>line.startsWith('["d"')&&line.includes(',0,"')).length};

const ports=(receipt:Record<string,any>,counter:{ticks:number;checks:number})=>({workspace:root,storeRoot,receipt,readEvidence:(name:string)=>evidence.get(name)!,implementationEntries:implementation.entries,implementationFile,producer:()=>receipt.producer.files.length||receipt.producer.environment.length?producerOf():receipt.producer,runtime:()=>runtime,conditionsClosed:()=>true,project:(path:string)=>cjs.readListingProjection(root,path),observeBindings:(_scope:string,selected:readonly {sourceDirectory:string;targetDirectory:string}[])=>observe(selected),readReady:fingerprint,control:control(counter)});
const base=captured.receipt as Record<string,any>;
const emptyEvidence=(name:string)=>({name,sha256:sha(""),bytes:0,rows:0});
for(const suffix of["frontier","execution","closure"])evidence.set("0".repeat(64)+"."+suffix+".ndjson",new Uint8Array());
const bare=(receipt:Record<string,any>)=>({...receipt,bindings:[],ready:[],inputs:[],required:[],producer:{files:[],environment:[]},implementation:{entries:receipt.implementation.entries,evidence:emptyEvidence("0".repeat(64)+".closure.ndjson")},membership:{frontier:emptyEvidence("0".repeat(64)+".frontier.ndjson"),texts:[]},execution:{...receipt.execution,evidence:emptyEvidence("0".repeat(64)+".execution.ndjson")}});
const tiers:Record<string,(receipt:Record<string,any>)=>Record<string,any>>={
 base:bare,
 implementation:receipt=>({...bare(receipt),implementation:receipt.implementation}),
 producer:receipt=>({...bare(receipt),producer:receipt.producer}),
 bindings:receipt=>({...bare(receipt),bindings:receipt.bindings}),
 ready:receipt=>({...bare(receipt),ready:receipt.ready}),
 inputs:receipt=>({...bare(receipt),inputs:receipt.inputs}),
 texts:receipt=>({...bare(receipt),membership:{frontier:emptyEvidence("0".repeat(64)+".frontier.ndjson"),texts:receipt.membership.texts}}),
 frontier:receipt=>({...bare(receipt),membership:{frontier:receipt.membership.frontier,texts:[]}}),
 execution:receipt=>({...bare(receipt),execution:receipt.execution}),
 all:receipt=>receipt,
};
const results:Record<string,unknown>={};
for(const[name,build]of Object.entries(tiers)){
 const samples:number[]=[];let status="";
 for(let index=0;index<3;index++){const tick={ticks:0,checks:0},start=performance.now(),verdict=await core.verifyGenerationReceiptV1(ports(build(base),tick));samples.push(ms(start));status=verdict.status==="match"?"match":verdict.class+(verdict.path?" "+String(verdict.path).slice(0,80):"");}
 results[name]={medianMilliseconds:median(samples),samples,status};
}
out.verifyTiers=results;
const totalSamples=(results.all as {samples:number[]}).samples;
out.verifySummary={medianMilliseconds:median(totalSamples),minMilliseconds:Math.min(...totalSamples),oldMembershipWalkMilliseconds:walkMs,loadavgAtEnd:loadavg().map(value=>Math.round(value*10)/10)};
let dirsStart=performance.now();let dirCount=0;for(const row of observed.paths)if(row.list){lstatSync(row.path,{bigint:true,throwIfNoEntry:false});dirCount++;}
out.rawLstatPass={directories:dirCount,milliseconds:ms(dirsStart)};
void keys;
console.log(JSON.stringify(out,null,1));
