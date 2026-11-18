#!/usr/bin/env python3
import os,sys,shutil
R="/Users/ueli/Documents/semio"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";GS=B+"🪪️graph-state/"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R93/stage1"
shutil.rmtree(S,ignore_errors=True);os.makedirs(S)
rows=[];n=[0]
def edit(rel,fn):
    live=open(R+"/"+rel,encoding="utf8").read();new=fn(live)
    i=str(n[0]);n[0]+=1
    open(S+"/"+i+".base","w",encoding="utf8").write(live);open(S+"/"+i+".new","w",encoding="utf8").write(new);rows.append((i,rel,"edit"))
def create(rel,text):
    i=str(n[0]);n[0]+=1
    open(S+"/"+i+".new","w",encoding="utf8").write(text);rows.append((i,rel,"new"))
def rep(text,old,new,count=1):
    assert text.count(old)==count,(old[:80],text.count(old));return text.replace(old,new)

def ex(t):
    t=rep(t,'''/** 🏷️ The inline description of an entry: see the schema definition `Note`. */''','''/** 🌊️ A document of an entry that was not downloaded with it: `read` downloads its blobs (verified against their names by the client) and decompresses them when asked, `keep` leaves the compressed blobs in memory (within a small budget per entry) so that the next `read` of the same document does not download them again. `hash` is the digest of the stored blob of a document that is one blob (the name the local stores know it by), `size` the compressed bytes. */
export type GraphDeferredV1=Readonly<{hash:string|undefined;size:number;read(options?:Readonly<{keep?:boolean}>):Promise<GraphDocumentV1|undefined>}>;
/** 🏷️ The inline description of an entry: see the schema definition `Note`. */''')
    t=rep(t,'export type GraphDecisionV1=Readonly<{outcome:"hit";via:"cache"|"wait";documents:readonly GraphDocumentV1[];note:GraphEntryNoteV1;waitedMs:number}>','export type GraphDecisionV1=Readonly<{outcome:"hit";via:"cache"|"wait";documents:readonly GraphDocumentV1[];deferred:ReadonlyMap<string,GraphDeferredV1>;note:GraphEntryNoteV1;waitedMs:number}>')
    t=rep(t,'''/** ⬇️ Looks the entry of a state up and downloads its documents, except the blobs `held` says the caller has. `undefined` is a miss; `"fault"` is an unreachable, denied or damaged entry (the caller computes). Every blob is verified against its name before it counts. */
export async function fetchGraphEntryV1(client:GraphFleetClientV1,state:GraphStateV1,held?:GraphHeldV1):Promise<{note:GraphEntryNoteV1;documents:readonly GraphDocumentV1[]}|undefined|"fault"> {''','''const DEFERRED_KEEP_BYTES_V1=32<<20;
/** ⬇️ Looks the entry of a state up and downloads its documents, except the blobs `held` says the caller has and the documents `defer` names: those are not downloaded and come back in `deferred`, to be read one at a time, so a caller that unpacks many large documents never holds more than it reads. `undefined` is a miss; `"fault"` is an unreachable, denied or damaged entry (the caller computes). Every blob is verified against its name before it counts. */
export async function fetchGraphEntryV1(client:GraphFleetClientV1,state:GraphStateV1,held?:GraphHeldV1,defer?:(name:string)=>boolean):Promise<{note:GraphEntryNoteV1;documents:readonly GraphDocumentV1[];deferred:ReadonlyMap<string,GraphDeferredV1>}|undefined|"fault"> {''')
    t=rep(t,''' const blobs:GraphDocumentV1[]=[];let broken=false;
 await each(described.entries,4,async entry=>{if(broken)return;if(held!==undefined&&await held({path:entry.path,hash:entry.hash,size:entry.size}))return;const bytes=await client.getBlob(entry.hash,entry.size);if(bytes===undefined){broken=true;return;}blobs.push({name:entry.path,bytes});});
 if(broken)return "fault";
 try{return {note:described.note,documents:unpackGraphDocumentsV1(blobs)};}catch{return "fault";}''',''' const blobs:GraphDocumentV1[]=[],lazy=new Map<string,{path:string;hash:string;size:number;index:number}[]>(),eager:{path:string;hash:string;size:number}[]=[];let broken=false;
 for(const entry of described.entries){
  const named=defer===undefined?null:/^(.*)\\.br(?:\\.(\\d+))?$/u.exec(entry.path);
  if(named===null||!defer!(named[1]!)){eager.push(entry);continue;}
  if(held!==undefined&&await held({path:entry.path,hash:entry.hash,size:entry.size}))continue;
  const rows=lazy.get(named[1]!)??[];rows.push({...entry,index:Number(named[2]??0)});lazy.set(named[1]!,rows);
 }
 await each(eager,4,async entry=>{if(broken)return;if(held!==undefined&&await held({path:entry.path,hash:entry.hash,size:entry.size}))return;const bytes=await client.getBlob(entry.hash,entry.size);if(bytes===undefined){broken=true;return;}blobs.push({name:entry.path,bytes});});
 if(broken)return "fault";
 const deferred=new Map<string,GraphDeferredV1>();let keptBytes=0;
 for(const [name,rows] of lazy){
  rows.sort((left,right)=>left.index-right.index);
  const size=rows.reduce((sum,row)=>sum+row.size,0);let kept:Uint8Array[]|undefined;
  deferred.set(name,Object.freeze({hash:rows.length===1?rows[0]!.hash:undefined,size,async read(options){
   let parts=kept;
   if(parts!==undefined){kept=undefined;keptBytes-=size;}
   else{parts=[];for(const row of rows){const bytes=await client.getBlob(row.hash,row.size);if(bytes===undefined)return undefined;parts.push(bytes);}}
   if(options?.keep===true&&keptBytes+size<=DEFERRED_KEEP_BYTES_V1){kept=parts;keptBytes+=size;}
   try{return unpackGraphDocumentsV1(parts.map((bytes,index)=>({name:rows.length===1?name+".br":`${name}.br.${index}`,bytes})))[0];}catch{return undefined;}
  }}));
 }
 try{return {note:described.note,documents:unpackGraphDocumentsV1(blobs),deferred};}catch{return "fault";}''')
    t=rep(t,'expectedMilliseconds?:number;waitLimitMilliseconds?:number;skip?:GraphHeldV1;now?:()=>number}>):Promise<GraphDecisionV1>{','expectedMilliseconds?:number;waitLimitMilliseconds?:number;skip?:GraphHeldV1;defer?:(name:string)=>boolean;now?:()=>number}>):Promise<GraphDecisionV1>{')
    t=rep(t,'const hit=(found:{note:GraphEntryNoteV1;documents:readonly GraphDocumentV1[]},via','const hit=(found:{note:GraphEntryNoteV1;documents:readonly GraphDocumentV1[];deferred:ReadonlyMap<string,GraphDeferredV1>},via')
    t=rep(t,'return {outcome:"hit",via,documents:found.documents,note:found.note,waitedMs:now()-started};','return {outcome:"hit",via,documents:found.documents,deferred:found.deferred,note:found.note,waitedMs:now()-started};')
    t=rep(t,'fetchGraphEntryV1(client,state,input.skip)','fetchGraphEntryV1(client,state,input.skip,input.defer)',3)
    t=rep(t,'files:found.documents.length,megabytes:Math.round(found.documents.reduce((sum,document)=>sum+document.bytes.length,0)/1048576)','files:found.documents.length+found.deferred.size,megabytes:Math.round((found.documents.reduce((sum,document)=>sum+document.bytes.length,0)+[...found.deferred.values()].reduce((sum,each)=>sum+each.size,0))/1048576)')
    return t
edit(GS+"🛰️exchange/🟦️.ts",ex)

def pr(t):
    t=rep(t,'import{decideGraphEntryV1,fetchGraphEntryV1,graphEntryKeyV1,graphFleetClientV1,neutralizeRootV1,publishGraphEntryOutcomeV1,publishGraphEntryV1,tryGraphEntryV1,type GraphEntryNoteV1,type GraphFleetClientV1,type GraphHoldV1,type GraphRefusalV1}','import{decideGraphEntryV1,fetchGraphEntryV1,graphEntryKeyV1,graphFleetClientV1,neutralizeRootV1,publishGraphEntryOutcomeV1,publishGraphEntryV1,tryGraphEntryV1,type GraphDeferredV1,type GraphEntryNoteV1,type GraphFleetClientV1,type GraphHoldV1,type GraphRefusalV1}')
    t=rep(t,'MAXIMUM_OFFERS=64,MAXIMUM_PACKED=32,','MAXIMUM_OFFERS=64,MAXIMUM_PACKED=32,CLOSURE_WINDOW=2,MAXIMUM_RECIPES=4096,isClosureDocument=(name:string)=>name.startsWith("closure-"),view=(bytes:Uint8Array)=>Buffer.from(bytes.buffer,bytes.byteOffset,bytes.byteLength),')
    old_start=t.index(' const takeClosures=async(')
    old_end=t.index(' };\nconst seedFrom=async(')+len(' };\n')
    new_take=''' const recipeMemo=new Map<string,Readonly<{recipe:unknown;owned:boolean}>>(),importing=new Map<string,Promise<number>>(),waiting:(()=>void)[]=[];let free=CLOSURE_WINDOW;
 const within=async<T>(work:()=>Promise<T>):Promise<T>=>{if(free===0)await new Promise<void>(go=>waiting.push(go));else free--;try{return await work();}finally{const next=waiting.shift();if(next!==undefined)next();else free++;}};
 const laneOf=(decision:Readonly<{documents:readonly Readonly<{name:string;bytes:Uint8Array}>[];deferred?:ReadonlyMap<string,GraphDeferredV1>}>,sources:ReadonlyMap<string,string>)=>{
  const lane=new Map<string,GraphDeferredV1>(decision.deferred??[]);
  for(const row of decision.documents)if(isClosureDocument(row.name))lane.set(row.name,{hash:sources.get(row.name),size:row.bytes.length,read:async()=>row});
  return new Map([...lane].sort((left,right)=>left[0]<right[0]?-1:1));
 };
 const recipesOf=async(lane:ReadonlyMap<string,GraphDeferredV1>)=>{
  const recipes=new Map<string,unknown>();let owned=false;
  for(const [name,entry] of lane){
   const known=entry.hash===undefined?undefined:recipeMemo.get(entry.hash);
   if(known!==undefined){recipes.set(name.slice(8),known.recipe);owned||=known.owned;continue;}
   await within(async()=>{
    const document=await entry.read({keep:true});if(document===undefined)return;
    try{
     const packed=JSON.parse(view(document.bytes).toString("utf8")) as PackedClosureV1;
     if(packed===null||typeof packed!=="object"||packed.version!==2||packed.recipe===null||typeof packed.recipe!=="object")return;
     const has=view(document.bytes).includes(portableRowsPolicyV1.ownerToken);
     recipes.set(name.slice(8),packed.recipe);owned||=has;
     if(entry.hash!==undefined){recipeMemo.delete(entry.hash);recipeMemo.set(entry.hash,{recipe:packed.recipe,owned:has});while(recipeMemo.size>MAXIMUM_RECIPES)recipeMemo.delete(recipeMemo.keys().next().value!);}
    }catch{}
   });
  }
  return{recipes,owned};
 };
 const takeClosures=async(info:ProjectRecordInfoV1,lane:ReadonlyMap<string,GraphDeferredV1>,recipes:ReadonlyMap<string,unknown>,held:ReadonlyMap<string,string>,keys:ReadonlyMap<string,string>,context:Readonly<{providers:readonly string[];generation:string|undefined}>)=>{
  if(info.closures===undefined)return;
  try{
   const all=new Map([...keys,...held,...bundleKeysV1([...recipes].map(([recipeKey,recipe])=>({recipeKey,bundle:{recipe} as unknown as ClosureBundleV1})),host,"local")]);
   for(const [name,entry] of lane){
    const hash=entry.hash,flying=hash===undefined?undefined:importing.get(hash);
    if(flying!==undefined){await flying.catch(()=>undefined);continue;}
    const work=within(async()=>{
     const document=await entry.read();if(document===undefined)return 0;
     let neutral:Readonly<{recipeKey:string;bundle:ClosureBundleV1}>;
     try{neutral={recipeKey:name.slice(8),bundle:unpackBundleV1(JSON.parse(view(document.bytes).toString("utf8")) as PackedClosureV1)};}catch{return 0;}
     const moved=localBundleV1(neutral,host,all,context);
     return moved===undefined?0:await info.closures!.import(input.root,[hash===undefined?moved:{...moved,source:hash}]);
    });
    if(hash!==undefined)importing.set(hash,work);
    try{counts.closures+=await work;}catch{}finally{if(hash!==undefined)importing.delete(hash);}
   }
  }catch{}
 };
'''
    t=t[:old_start]+new_take+t[old_end:]
    t=rep(t,'''const seedFrom=async(info:ProjectRecordInfoV1,state:GraphStateV1,decision:Readonly<{documents:readonly Readonly<{name:string;bytes:Uint8Array}>[];note:GraphEntryNoteV1;via:string}>,held:Map<string,string>):Promise<boolean>=>{''','''const seedFrom=async(info:ProjectRecordInfoV1,state:GraphStateV1,decision:Readonly<{documents:readonly Readonly<{name:string;bytes:Uint8Array}>[];deferred?:ReadonlyMap<string,GraphDeferredV1>;note:GraphEntryNoteV1;via:string}>,held:Map<string,string>):Promise<boolean>=>{''')
    t=rep(t,'''    const text=Buffer.from(document.bytes).toString("utf8"),neutral=unpackRecordV1(JSON.parse(text) as PackedRecordV1),owned=decision.documents.some(row=>Buffer.from(row.bytes).includes(portableRowsPolicyV1.ownerToken)),context={providers:owned?await providersOf():[],generation:(input.generation??dependencyGenerationV1)(input.root)},taken=localRecordV1(neutral,host,context),sources=new Map(decision.note.documents.flatMap(row=>row.name.endsWith(".br")?[[row.name.slice(0,-3),row.hash] as const]:[]));
    await takeClosures(info,decision.documents,sources,held,taken.keys,context);''','''    const text=Buffer.from(document.bytes).toString("utf8"),neutral=unpackRecordV1(JSON.parse(text) as PackedRecordV1),sources=new Map(decision.note.documents.flatMap(row=>row.name.endsWith(".br")?[[row.name.slice(0,-3),row.hash] as const]:[])),lane=info.closures===undefined?new Map<string,GraphDeferredV1>():laneOf(decision,sources),found=await recipesOf(lane);
    const owned=decision.documents.some(row=>view(row.bytes).includes(portableRowsPolicyV1.ownerToken))||found.owned,context={providers:owned?await providersOf():[],generation:(input.generation??dependencyGenerationV1)(input.root)},taken=localRecordV1(neutral,host,context);
    await takeClosures(info,lane,found.recipes,held,taken.keys,context);''')
    t=rep(t,'const found=await fetchGraphEntryV1(client,state,heldOf(info,held));','const found=await fetchGraphEntryV1(client,state,heldOf(info,held),info.closures===undefined?undefined:isClosureDocument);')
    t=rep(t,'return{seeded:await seedFrom(info,state,{documents:found.documents,note:found.note,via:"cache"},held),...nothing};','return{seeded:await seedFrom(info,state,{documents:found.documents,deferred:found.deferred,note:found.note,via:"cache"},held),...nothing};')
    t=rep(t,'decideGraphEntryV1({client,state,owner,skip:heldOf(info,held)})','decideGraphEntryV1({client,state,owner,skip:heldOf(info,held),...info.closures===undefined?{}:{defer:isClosureDocument}})')
    return t
edit(GS+"📦️project-records/🟦️.ts",pr)
T="🪪️graph-state/"
here=os.path.dirname(os.path.abspath(__file__))
create(GS+"📦️project-records/🧪️tests/🌊️seed-stream/🟦️.ts",open(here+"/law/test.ts",encoding="utf8").read())
create(GS+"📦️project-records/🧪️tests/🌊️seed-stream/👶️child.ts",open(here+"/law/child.ts",encoding="utf8").read())
G=GS[len(B):]
K=GS+"📦️project-records/🧪️tests/"
open(S+"/tests.txt","w",encoding="utf8").write("\n".join([K+"🌊️seed-stream/🟦️.ts",K+"🚚️closures/🟦️.ts",K+"🗜️packed/🟦️.ts",GS+"🧪️tests/🟦️.ts",GS+"📦️project-records/🚚️entry-lane/🧪️tests/🟦️.ts",GS+"🙋️wanted/🧪️tests/🟦️.ts",GS+"🙋️wanted/💰️cost/🧪️tests/🟦️.ts",B+"🕸️graph/📥️inference/📦️project-nodes/🧪️tests/📥️take/🟦️.ts"])+"\n")
open(S+"/before.txt","w",encoding="utf8").write(GS+"📦️project-records/🧪️tests/🌊️seed-stream/🟦️.ts\n")
open(S+"/note.txt","w",encoding="utf8").write("Package R93 (report `☀️14/WORKERS-READY-WITH-LOW-RESOURCES/result-R93.md`): a project record taken from the fleet cache streams its closure entries: `fetchGraphEntryV1`/`decideGraphEntryV1` accept `defer` and return the closure documents undownloaded in `deferred`; `project-records` reads the recipes in a first pass (one document at a time, two in flight), then unpacks, localises and imports one bundle at a time, shares bundles between owners by blob digest (in-flight map, recipe memo). Replaces the all-at-once `takeClosures`. New law `project-records/🧪️tests/🌊️seed-stream` (real child process, real cache server): peak resident set of a cold seeding. Measured on a real fleet entry (79 bundles, 19 MB compressed, 194 MB decompressed): tree footprint peak 3099 MB -> 520 MB.\n")
open(S+"/index.txt","w",encoding="utf8").write("".join("%s %s %s\n"%r for r in rows))
print(rows)
