import{createHash,randomBytes}from"node:crypto";
import{existsSync,readFileSync,statSync,writeFileSync}from"node:fs";
import{hostname}from"node:os";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{graphPlatformV1,type GraphStateV1}from"../🟦️.ts";
import{recordPlatformV1}from"../🌍️platform-class/🟦️.ts";
import{bundleKeysV1,dependencyGenerationV1,hostOfRootV1,localBundleV1,localRecordV1,neutralBundleV1,neutralRecordV1,portableRowsPolicyV1,type ClosureBundleV1,type ObservationRowV1,type PortableRecordV1}from"./🧳️portable/🟦️.ts";
import{decideGraphEntryV1,graphEntryKeyV1,graphFleetClientV1,neutralizeRootV1,publishGraphEntryV1,type GraphFleetClientV1,type GraphHoldV1}from"../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../🧶️nx/🟦️.ts";
import{remoteCacheFetchV1}from"../../🌐️remote-cache/🛰️fleet/🟦️.ts";

type Environment=Readonly<Record<string,string|undefined>>;
/** 📦️ What the inference seam hands over for one project: where its record lives, which project it is, the key text it was computed under, and how to read and write the record locally. */
/** 🚚️ How the closure entries a record names travel with it: bundles of the recipes it refers to, exported from and imported into the closure store of this machine. */
export type ProjectClosureTravelV1=Readonly<{export(root:string,image:string):Promise<readonly Readonly<{recipeKey:string;bundle:unknown}>[]>;import(root:string,bundles:readonly Readonly<{recipeKey:string;bundle:unknown}>[]):Promise<number>}>;
export type ProjectRecordInfoV1=Readonly<{cachePath:string;fleet:string;keyText:string;exportRecord():unknown;import(portable:unknown):Promise<boolean>;closures?:ProjectClosureTravelV1|undefined}>;
const SOURCE_ANALYSES="../../🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs";
/** 🪪️ The ids of the source analyses this process registered for its inference, in the order of their roles: a taken record names the providers of its producer by role and the id of a provider contains a hash of the module location, so the taker puts in the ids it has registered (the module is loaded only when a record is taken; a process that cannot load it takes records whose analysis rows cannot hold, and infers those projects itself). */
async function registeredProvidersV1():Promise<readonly string[]>{
 const loaded=await import(SOURCE_ANALYSES) as {prepareSourceAnalysesV1():Readonly<Record<string,{id:string}>>};
 return Object.values(loaded.prepareSourceAnalysesV1()).map(provider=>provider.id);
}

/** 🚪️ The answer of the gate: whether the record was taken from the fleet, and how to store the computed one or give the claim back. */
export type ProjectGateV1=Readonly<{seeded:boolean;publish():void;abandon():Promise<void>}>;
export type ProjectFleetPortV1=Readonly<{gate(info:ProjectRecordInfoV1):Promise<ProjectGateV1>;offer(info:ProjectRecordInfoV1):Promise<void>;flush():Promise<string>;stats():Readonly<Record<"seeded"|"published"|"offered"|"waited"|"computed"|"closures"|"refused"|"reinferred",number>>}>;

/** 🚩️ Where the fleet marker of a stored project record lives: beside the record, named after it; the closure proof of the graph store knows this name. */
export const projectRecordMarkerPathV1=(cachePath:string)=>cachePath+".fleet";
const ZERO="0".repeat(64),sha=(text:string)=>createHash("sha256").update(text).digest("hex"),MAXIMUM_OFFERS=64;

/** 🧮️ The state a project record is stored under: the digest of the record key (inference revision and the project's own identity) with the root replaced by the token, the platform tag (the neutral tag of every platform, the exact platform for a project whose key names a host-bound declaration), and one slot per project so that a newer record of a project supersedes only its own older ones. It names nothing of the machine, the generation or the module-load state of the producer: the record carries every observation it consumed, and a taker checks them against its own tree before it uses the record, so a record is reusable by every machine whose project inputs equal the producer's. A record that does not hold in the taker's tree is replaced by the taker's own. */
export function projectRecordStateV1(input:Readonly<{root:string;fleet:string;keyText:string;platform:string}>):GraphStateV1{
 const neutral=(text:string)=>Buffer.from(neutralizeRootV1(Buffer.from(text),input.root)).toString("utf8"),platform=recordPlatformV1(input.keyText,input.platform),digest=sha(JSON.stringify(["project-inference",neutral(input.keyText),platform,neutral(input.fleet)]));
 return Object.freeze({version:1,frame:Object.freeze({kind:"project-inference" as const,tooling:sha(neutral(input.fleet)),implementation:ZERO,platform,extra:Object.freeze({})}),names:ZERO,content:ZERO,files:0,contentFiles:0,digest});
}

/** 🌐️ The port of one process: it takes a missing project record from the fleet cache, claims the computation of a record nobody has (so eight machines split the projects), stores finished records and offers the records that were computed earlier. Every fault ends in "compute locally". `undefined` when no fleet cache is configured. */
export function createProjectFleetPortV1(input:Readonly<{root:string;environment:Environment;client?:GraphFleetClientV1;owner?:string;platform?:string;providers?():Promise<readonly string[]>;generation?(root:string):string|undefined;say?(line:string):void}>):ProjectFleetPortV1|undefined{
 const settings=input.client===undefined?graphFleetSettingsV1(input.root,input.environment):undefined;
 if(input.client===undefined&&settings===undefined)return undefined;
 const client=input.client??graphFleetClientV1(settings!,{fetch:remoteCacheFetchV1(input.environment,{root:input.root})}),platform=input.platform??graphPlatformV1(),owner=input.owner??`${hostname()}:${process.pid}:${randomBytes(3).toString("hex")}`,pending=new Set<Promise<unknown>>(),counts={seeded:0,published:0,offered:0,waited:0,computed:0,closures:0,refused:0,reinferred:0},host=hostOfRootV1(input.root);
 const say=input.say??((line:string)=>{try{process.stderr.write(line+"\n");}catch{}}),language=(input.environment.SEMIO_LANGUAGE==="de"?"de":"en") as "en"|"de";
 const decided=()=>{if((counts.seeded+counts.computed)%50===0)say(schema.definitions.Texts.const[language].progress.replace(/\{(\w+)\}/gu,(_all,name:string)=>String((counts as Record<string,number>)[name]??0)));};
 const stateOf=(info:ProjectRecordInfoV1)=>projectRecordStateV1({root:input.root,fleet:info.fleet,keyText:info.keyText,platform});
 const track=<T>(work:Promise<T>):Promise<T>=>{pending.add(work);void work.finally(()=>pending.delete(work)).catch(()=>undefined);return work;};
 const packed=new WeakMap<object,Readonly<{name:string;bytes:Uint8Array}>|null>();
 const providersOf=async():Promise<readonly string[]>=>{try{return await(input.providers??registeredProvidersV1)();}catch{return[];}};
 const closureDocsOf=async(info:ProjectRecordInfoV1,image:string,keys:ReadonlyMap<string,string>)=>{
  if(info.closures===undefined)return[];
  try{
   const bundles=(await info.closures.export(input.root,image)) as readonly Readonly<{recipeKey:string;bundle:ClosureBundleV1}>[],all=new Map([...keys,...bundleKeysV1(bundles,host,"neutral")]);
   return bundles.flatMap(entry=>{
    let doc=packed.get(entry.bundle);
    if(doc===undefined){const moved=neutralBundleV1(entry,host,all);doc=moved===undefined?null:{name:`closure-${moved.recipeKey}`,bytes:Buffer.from(JSON.stringify(moved.bundle))};packed.set(entry.bundle,doc);}
    return doc===null?[]:[doc];
   }).sort((left,right)=>left.name<right.name?-1:1);
  }catch{return[];}
 };
 const documentsOf=async(info:ProjectRecordInfoV1)=>{
  const portable=info.exportRecord() as PortableRecordV1|undefined;if(portable===undefined)return undefined;
  let neutral:ReturnType<typeof neutralRecordV1>;
  try{neutral=neutralRecordV1(portable,host);}catch{counts.refused++;return undefined;}
  return[{name:"record",bytes:Buffer.from(JSON.stringify(neutral.record))},...await closureDocsOf(info,portable.image,neutral.keys)];
 };
 const takeClosures=async(info:ProjectRecordInfoV1,documents:readonly Readonly<{name:string;bytes:Uint8Array}>[],keys:ReadonlyMap<string,string>,context:Readonly<{providers:readonly string[];generation:string|undefined}>)=>{
  if(info.closures===undefined)return;
  try{
   const neutral=documents.filter(row=>row.name.startsWith("closure-")).flatMap(row=>{try{return[{recipeKey:row.name.slice(8),bundle:JSON.parse(Buffer.from(row.bytes).toString("utf8")) as ClosureBundleV1}];}catch{return[];}});
   const all=new Map([...keys,...bundleKeysV1(neutral,host,"local")]),bundles=neutral.flatMap(entry=>{const moved=localBundleV1(entry,host,all,context);return moved===undefined?[]:[moved];});
   if(bundles.length>0)counts.closures+=await info.closures.import(input.root,bundles);
  }catch{}
 };
 const marker=(info:ProjectRecordInfoV1)=>projectRecordMarkerPathV1(info.cachePath);
 const versionOf=(info:ProjectRecordInfoV1,state:GraphStateV1)=>{const form=info.closures===undefined?"":":closures-1";try{const status=statSync(info.cachePath);return`${state.digest}:${status.size}:${Math.floor(status.mtimeMs)}${form}`;}catch{return state.digest+form;}};
 const mark=(info:ProjectRecordInfoV1,state:GraphStateV1)=>{try{writeFileSync(marker(info),versionOf(info,state));}catch{}};
 const publisher=(info:ProjectRecordInfoV1,state:GraphStateV1,hold:GraphHoldV1|undefined,seeded:boolean=false)=>{let done=false;return{
  publish(){if(done)return;done=true;if(seeded)counts.reinferred++;void track((async()=>{const documents=await documentsOf(info);if(documents===undefined){await hold?.release().catch(()=>undefined);return false;}const stored=await publishGraphEntryV1({client,state,owner,documents,...(hold===undefined?{}:{hold})});if(stored){counts.published++;mark(info,state);}return stored;})());},
  async abandon(){if(done)return;done=true;await hold?.release().catch(()=>undefined);}};};
 return Object.freeze({
  async gate(info){
   const state=stateOf(info),decision=await decideGraphEntryV1({client,state,owner});
   if(decision.outcome==="hit"){
    const document=decision.documents.find(row=>row.name==="record");
    try{
     if(document!==undefined){
      const text=Buffer.from(document.bytes).toString("utf8"),neutral=JSON.parse(text) as PortableRecordV1,context={providers:text.includes(portableRowsPolicyV1.ownerToken)?await providersOf():[],generation:(input.generation??dependencyGenerationV1)(input.root)},taken=localRecordV1(neutral,host,context);
      await takeClosures(info,decision.documents,taken.keys,context);
      const ok=await info.import(taken.record);
      if(ok){counts.seeded++;decided();if(decision.via==="wait")counts.waited++;mark(info,state);return{seeded:true,...publisher(info,state,undefined,true)};}
     }
    }catch{counts.refused++;}
    return{seeded:false,...publisher(info,state,undefined)};
   }
   counts.computed++;decided();
   return{seeded:false,...publisher(info,state,decision.hold)};
  },
  async offer(info){
   if(!client.settings.upload||!client.usable())return;
   while(pending.size>=MAXIMUM_OFFERS)await Promise.race([...pending].map(work=>work.catch(()=>undefined)));
   const state=stateOf(info);
   try{if(existsSync(marker(info))&&readFileSync(marker(info),"utf8")===versionOf(info,state))return;}catch{}
   void track((async()=>{
    const documents=await documentsOf(info);if(documents===undefined)return false;
    const stored=await publishGraphEntryV1({client,state,owner,documents});if(stored){counts.offered++;mark(info,state);}return stored;
   })().catch(()=>false));
  },
  async flush(){await Promise.allSettled([...pending]);const texts=schema.definitions.Texts.const;return(["en","de"] as const).map(language=>texts[language].summary.replace(/\{(\w+)\}/gu,(_all,name:string)=>String((counts as Record<string,number>)[name]??0))).join(" | ");},
  stats:()=>Object.freeze({...counts}),
 });
}
