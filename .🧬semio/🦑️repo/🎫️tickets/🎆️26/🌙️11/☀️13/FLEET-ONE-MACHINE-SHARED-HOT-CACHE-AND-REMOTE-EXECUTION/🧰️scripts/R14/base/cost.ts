import{createHash}from"node:crypto";
import{mkdirSync,readFileSync,renameSync,writeFileSync}from"node:fs";
import{dirname,join}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import type{GraphStateV1}from"../../🟦️.ts";
import{fetchGraphEntryV1,publishGraphEntryV1,type GraphFleetClientV1}from"../../🛰️exchange/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 💰️ The measured numbers a read decides a project's weight by: the physical footprint per managed byte, the margin a cost must fit with, the samples kept per owner, the managed cost of an owner nobody measured (by kind, else this number) and the owners measured on g1-server. See `./🧬️schema/🔣️.json`. */
export const ownerCostPolicyV1=Object.freeze({...schema.definitions.Policy.const});
export type OwnerCostPolicyV1=typeof ownerCostPolicyV1;
export type OwnerSampleV1=Readonly<{megabytes:number;atMs:number;stopped:boolean}>;
export type OwnerCostEntryV1=Readonly<{kind:string;rows:number;seconds:number;samples:readonly OwnerSampleV1[]}>;
export type OwnerCostTableV1=Readonly<{version:1;owners:Readonly<Record<string,OwnerCostEntryV1>>}>;
export type OwnerEstimateV1=Readonly<{megabytes:number;source:"recorded"|"seed"|"kind"|"unknown"}>;
export type OwnerGrantV1=Readonly<{footprintMegabytes:number;managedMegabytes:number}>;
export type HeavyOwnerV1=Readonly<{name:string;megabytes:number;source:OwnerEstimateV1["source"]}>;
export type OwnerClassesV1=Readonly<{heavy:readonly HeavyOwnerV1[];light:readonly string[]}>;
export type OwnerCostInputV1=Readonly<{name:string;kind:string;megabytes:number;seconds:number;rows:number;stopped:boolean;atMs:number}>;
const ZERO="0".repeat(64),sha=(text:string)=>createHash("sha256").update(text).digest("hex");

/** 🗣️ One message of the cost texts in one language. */
export function ownerCostTextV1(key:keyof typeof schema.definitions.Texts.const.en,language:"en"|"de",fields:Readonly<Record<string,string|number>>={}):string{
 return schema.definitions.Texts.const[language][key].replace(/\{(\w+)\}/gu,(_all,name:string)=>String(fields[name]??""));
}

/** 📋️ A table nobody recorded anything into. */
export const emptyOwnerCostTableV1=():OwnerCostTableV1=>Object.freeze({version:1 as const,owners:Object.freeze({})});

/** ✍️ The table with one more measurement of one owner: the newest `samples` are kept, so an owner is judged by what it needed lately, and only its own samples count (never the ones of its kind). */
export function recordOwnerCostV1(table:OwnerCostTableV1,input:OwnerCostInputV1,policy:OwnerCostPolicyV1=ownerCostPolicyV1):OwnerCostTableV1{
 const known=table.owners[input.name],samples=[...(known?.samples??[]),{megabytes:Math.max(0,Math.round(input.megabytes)),atMs:input.atMs,stopped:input.stopped}].sort((a,b)=>a.atMs-b.atMs).slice(-policy.samples);
 return Object.freeze({version:1 as const,owners:Object.freeze({...table.owners,[input.name]:Object.freeze({kind:input.kind,rows:Math.max(known?.rows??0,input.rows),seconds:Math.max(known?.seconds??0,Math.round(input.seconds)),samples:Object.freeze(samples)})})});
}

/** 🔀️ The union of tables (the fleet's and this machine's): per owner the samples of all tables by time, equal samples once, the newest `samples` kept. */
export function mergeOwnerCostTablesV1(tables:readonly(OwnerCostTableV1|undefined)[],policy:OwnerCostPolicyV1=ownerCostPolicyV1):OwnerCostTableV1{
 const owners:Record<string,OwnerCostEntryV1>={};
 for(const table of tables)for(const [name,entry] of Object.entries(table?.owners??{})){
  const known=owners[name],seen=new Set((known?.samples??[]).map(sample=>`${sample.atMs}:${sample.megabytes}:${sample.stopped}`)),extra=entry.samples.filter(sample=>!seen.has(`${sample.atMs}:${sample.megabytes}:${sample.stopped}`));
  owners[name]=Object.freeze({kind:entry.kind,rows:Math.max(known?.rows??0,entry.rows),seconds:Math.max(known?.seconds??0,entry.seconds),samples:Object.freeze([...(known?.samples??[]),...extra].sort((a,b)=>a.atMs-b.atMs).slice(-policy.samples))});
 }
 return Object.freeze({version:1 as const,owners:Object.freeze(owners)});
}

/** 🔮️ What inferring `name` takes in managed megabytes: its own recorded samples (the largest of the newest few: an owner is as heavy as it was lately), else the measured seed of the declared heavy owners, else the cost of its kind, else the cost of an unknown owner. A stopped sample is a lower bound and counts as measured. */
export function estimateOwnerCostV1(table:OwnerCostTableV1|undefined,name:string,kind?:string,policy:OwnerCostPolicyV1=ownerCostPolicyV1):OwnerEstimateV1{
 const entry=table?.owners[name];
 if(entry!==undefined&&entry.samples.length>0)return{megabytes:Math.max(...entry.samples.map(sample=>sample.megabytes)),source:"recorded"};
 const seed=(policy.seeds as Readonly<Record<string,number>>)[name];if(seed!==undefined)return{megabytes:seed,source:"seed"};
 const byKind=kind===undefined?undefined:(policy.kindMegabytes as Readonly<Record<string,number>>)[kind];if(byKind!==undefined)return{megabytes:byKind,source:"kind"};
 return{megabytes:policy.unknownMegabytes,source:"unknown"};
}

/** 🎁️ What one project may take on a machine admitted `admittedMegabytes` (half its memory, or the explicit budget): the physical footprint is the admitted memory less the base the parent holds before any project (the wanted policy's `baseMegabytes`), the managed megabytes are that footprint divided by the footprint factor and the margin. A serving machine (admitted at least `serveHeadroomMegabytes`) has no limit. The base is the larger of the wanted policy's `baseMegabytes` and the measured `floorMegabytes` of the parent. */
export function ownerGrantV1(admittedMegabytes:number,base:Readonly<{baseMegabytes:number;serveHeadroomMegabytes:number}>,policy:OwnerCostPolicyV1=ownerCostPolicyV1):OwnerGrantV1{
 if(!Number.isFinite(admittedMegabytes)||admittedMegabytes>=base.serveHeadroomMegabytes)return{footprintMegabytes:Number.POSITIVE_INFINITY,managedMegabytes:Number.POSITIVE_INFINITY};
 const footprint=Math.max(0,Math.floor(admittedMegabytes-Math.max(base.baseMegabytes,policy.floorMegabytes)));
 return{footprintMegabytes:footprint,managedMegabytes:Math.floor(footprint/policy.footprintFactor/(1+policy.marginPercent/100))};
}

/** 🏋️ Splits owners by weight: heavy is a MEASURED property of a named owner (its own recorded samples or the seed of a declared heavy owner) whose cost does not fit the grant; it is never inferred on this machine, however few they are. An owner nobody measured is light whatever the grant is (the cost of its kind or the unknown cost only orders the serving machine's work): measured on a 16 GB worker, a grant of 0 MB at a small booking turned all 517 stale projects into heavy ones, 820 MB of an unknown project being more than nothing. A light owner that outgrows the grant is stopped by the memory governor, recorded under its name and heavy from then on. The heavy ones come heaviest first. */
export function classifyOwnersV1(names:readonly string[],estimate:(name:string)=>OwnerEstimateV1,grant:OwnerGrantV1,policy:OwnerCostPolicyV1=ownerCostPolicyV1):OwnerClassesV1{
 const heavy:HeavyOwnerV1[]=[],light:string[]=[],measured=policy.heavySources as readonly string[];
 for(const name of names){const row=estimate(name);if(row.megabytes>grant.managedMegabytes&&measured.includes(row.source))heavy.push({name,megabytes:row.megabytes,source:row.source});else light.push(name);}
 return{heavy:heavy.sort((a,b)=>b.megabytes-a.megabytes||(a.name<b.name?-1:a.name>b.name?1:0)),light};
}

/** 🧭️ The order a serving machine infers asked-for projects in: the ones requesters call heavy first, then by cost, heaviest first (they block every requester and a long inference should start early), equal cost in the order given (older requests first). */
export function orderByCostV1(names:readonly string[],estimate:(name:string)=>OwnerEstimateV1,heavy:ReadonlySet<string>=new Set()):string[]{
 return[...new Set(names)].map((name,at)=>({name,at,heavy:heavy.has(name),megabytes:estimate(name).megabytes})).sort((a,b)=>Number(b.heavy)-Number(a.heavy)||b.megabytes-a.megabytes||a.at-b.at).map(row=>row.name);
}

/** 🗒️ Words the heavy projects in the language of the machine: the heaviest few with their cost, then how many more. */
export function describeHeavyV1(heavy:readonly HeavyOwnerV1[],language:"en"|"de",policy:OwnerCostPolicyV1=ownerCostPolicyV1):string{
 const shown=heavy.slice(0,policy.maximumNamed).map(row=>ownerCostTextV1("list",language,{name:row.name,megabytes:row.megabytes*policy.footprintFactor}));
 return heavy.length>shown.length?[...shown,ownerCostTextV1("more",language,{count:heavy.length-shown.length})].join(", "):shown.join(", ");
}

/** 🌊️ The cost a published wave proves: when exactly one project ran at the peak of the wave, the peak above what survived the collection is its cost. Several projects at the peak prove nothing about one of them, and a peak below the floor is noise. */
export function waveEvidenceV1(row:Readonly<{peakMegabytes:number;managedMegabytes:number;peakProjects:readonly string[]}>,policy:OwnerCostPolicyV1=ownerCostPolicyV1):Readonly<{name:string;megabytes:number}>|undefined{
 if(row.peakProjects.length!==1)return undefined;
 const megabytes=row.peakMegabytes-row.managedMegabytes;
 return megabytes>=policy.waveFloorMegabytes?{name:row.peakProjects[0]!,megabytes}:undefined;
}

/** 📍️ Where the table of a worktree lives: beside the other state of the machine's graph work, never in a selection directory, so every read of the machine sees it. */
export const ownerCostPathV1=(root:string,policy:OwnerCostPolicyV1=ownerCostPolicyV1):string=>join(root,policy.directory,policy.file);

/** 📖️ The table in a file; a missing, damaged or foreign file reads as an empty table. */
export function readOwnerCostFileV1(path:string):OwnerCostTableV1{
 try{
  const table=JSON.parse(readFileSync(path,"utf8")) as OwnerCostTableV1;
  return validateJsonSchemaSubset({$ref:"#/definitions/Table"},table,schema).length===0?table:emptyOwnerCostTableV1();
 }catch{return emptyOwnerCostTableV1();}
}

/** 💾️ Replaces the file atomically; a disk that cannot be written never fails a read. */
export function writeOwnerCostFileV1(path:string,table:OwnerCostTableV1):void{
 try{mkdirSync(dirname(path),{recursive:true});const temporary=`${path}.${process.pid}.tmp`;writeFileSync(temporary,JSON.stringify(table));renameSync(temporary,path);}catch{}
}

/** 📝️ Records one measurement into the file of the machine (read, add, replace: two readers writing at once may lose one sample, never the file). */
export function recordOwnerCostFileV1(path:string,input:OwnerCostInputV1):OwnerCostTableV1{
 const table=recordOwnerCostV1(readOwnerCostFileV1(path),input);writeOwnerCostFileV1(path,table);return table;
}

/** 🧾️ Records what the owners of one read took into the file of the machine: only the notable ones (stopped, at least the wave floor, or already known so that a lighter run ages the old cost out) are kept, in memory, and written merged with the file at most every `flushMilliseconds` and on `flush`, so a refresh of 700 owners writes a handful of times. A run that stopped an owner flushes at once: the next read must know it. */
export function createOwnerCostRecorderV1(input:Readonly<{path:string;rowsOf?(key:string):number;now?:()=>number;policy?:OwnerCostPolicyV1}>):Readonly<{record(row:Readonly<{key:string;kind:string;takenBytes:number;seconds:number;stopped:boolean}>):void;flush():void;table():OwnerCostTableV1}>{
 const policy=input.policy??ownerCostPolicyV1,now=input.now??Date.now;
 let table=readOwnerCostFileV1(input.path),dirty=false,flushedAt=-Infinity;
 const flush=()=>{if(!dirty)return;table=mergeOwnerCostTablesV1([readOwnerCostFileV1(input.path),table],policy);writeOwnerCostFileV1(input.path,table);dirty=false;flushedAt=now();};
 return Object.freeze({
  record(row){
   const megabytes=Math.round(row.takenBytes/1048576);
   if(!row.stopped&&megabytes<policy.waveFloorMegabytes&&table.owners[row.key]===undefined)return;
   table=recordOwnerCostV1(table,{name:row.key,kind:row.kind,megabytes,seconds:row.seconds,rows:input.rowsOf?.(row.key)??0,stopped:row.stopped,atMs:now()},policy);dirty=true;
   if(row.stopped||now()-flushedAt>=policy.flushMilliseconds)flush();
  },
  flush,
  table:()=>table,
 });
}

/** 🔑️ The exchange state the fleet's table lives under, one slot for every platform: the cost of inferring a project is a property of the project, measured by the serving machine and read by every worker. */
export function ownerCostStateV1():GraphStateV1{
 return Object.freeze({version:1,frame:Object.freeze({kind:"project-inference" as const,tooling:sha("project-costs"),implementation:sha("fleet"),platform:"neutral",extra:Object.freeze({costs:"1"})}),names:ZERO,content:ZERO,files:0,contentFiles:0,digest:sha(JSON.stringify(["project-costs"]))}) as GraphStateV1;
}

/** 📣️ Publishes the table of a serving machine, the one that measured the heavy owners, so every reader plans with it. False when the fleet cannot take it. */
export async function publishOwnerCostsV1(input:Readonly<{client:GraphFleetClientV1;server:string;table:OwnerCostTableV1}>):Promise<boolean>{
 return publishGraphEntryV1({client:input.client,state:ownerCostStateV1(),owner:input.server,documents:[{name:"costs",bytes:Buffer.from(JSON.stringify(input.table))}]});
}

/** 👂️ The fleet's table, or undefined when there is none, it is damaged or the fleet cannot answer. */
export async function readOwnerCostsV1(input:Readonly<{client:GraphFleetClientV1}>):Promise<OwnerCostTableV1|undefined>{
 const found=await fetchGraphEntryV1(input.client,ownerCostStateV1());
 if(found===undefined||found==="fault")return undefined;
 try{
  const document=found.documents.find(row=>row.name==="costs");if(document===undefined)return undefined;
  const table=JSON.parse(Buffer.from(document.bytes).toString("utf8")) as OwnerCostTableV1;
  return validateJsonSchemaSubset({$ref:"#/definitions/Table"},table,schema).length===0?table:undefined;
 }catch{return undefined;}
}
