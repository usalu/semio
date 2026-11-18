const L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
export const SI=L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs",SID=L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.d.mts",EN=L+"🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs",PORT=L+"🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts",LANE=L+"🪪️graph-state/📦️project-records/🚚️entry-lane/🟦️.ts",LANES=L+"🪪️graph-state/📦️project-records/🚚️entry-lane/🧬️schema/🔣️.json",REC=L+"🪪️graph-state/📦️project-records/🟦️.ts",FPS=L+"🧠️memory/🦶️footprint/🧬️schema/🔣️.json",FPD=L+"🧠️memory/🦶️footprint/🟨️.d.mts";
const edit=(from,to)=>text=>{const at=text.indexOf(from);if(at<0||text.indexOf(from,at+1)>=0)throw Error("anchor not unique or missing: "+from.slice(0,80));return text.replace(from,()=>to);};

const headCode=`
const CLOSURE_HEAD_V1=Buffer.from('{"version":2,"recipe":'),CLOSURE_HEAD_SCAN_V1=1<<20;
const headRecipeV1=(view:Buffer):unknown=>{
 if(view.length<CLOSURE_HEAD_V1.length+2||!view.subarray(0,CLOSURE_HEAD_V1.length).equals(CLOSURE_HEAD_V1))return undefined;
 const open=CLOSURE_HEAD_V1.length,end=Math.min(view.length,open+CLOSURE_HEAD_SCAN_V1);
 if(view[open]!==123)return undefined;
 let depth=0,quoted=false;
 for(let at=open;at<end;at++){
  const byte=view[at]!;
  if(quoted){if(byte===92)at++;else if(byte===34)quoted=false;continue;}
  if(byte===34)quoted=true;
  else if(byte===123||byte===91)depth++;
  else if(byte===125||byte===93){
   depth--;
   if(depth!==0)continue;
   if(view[at+1]!==44)return undefined;
   try{const recipe=JSON.parse(view.toString("utf8",open,at+1)) as unknown;return recipe!==null&&typeof recipe==="object"?recipe:undefined;}catch{return undefined;}
  }
 }
 return undefined;
};

/** 🔖️ The recipe of a stored closure document and whether it names the owner token, read from its first bytes: the packer writes the recipe (a few hundred characters) before the rows, so naming the keys of the documents of an entry parses none of their rows (the intake parsed every document twice, once for its recipe and once to localize it, 57 s of 192 s for one owner on a cold machine). The bytes are viewed, never copied; a document that does not start the way the packer writes it is read by the full parse; undefined when it is no closure document.
 * @see ./🧪️tests/🔖️document-head */
export function closureDocumentHeadV1(bytes:Uint8Array,policy:PortableRowsPolicyV1=portableRowsPolicyV1):Readonly<{recipe:unknown;owned:boolean}>|undefined{
 const view=Buffer.from(bytes.buffer,bytes.byteOffset,bytes.byteLength),recipe=headRecipeV1(view);
 if(recipe!==undefined)return{recipe,owned:view.includes(policy.ownerToken)};
 try{const full=JSON.parse(view.toString("utf8")) as{version?:unknown;recipe?:unknown}|null;return full!==null&&typeof full==="object"&&full.version===2&&full.recipe!==null&&typeof full.recipe==="object"?{recipe:full.recipe,owned:view.includes(policy.ownerToken)}:undefined;}catch{return undefined;}
}
`;

const storeCode=`  /** 🧷️ Remembers that the entry stored under the recipe key \`key\` is the local form of the fleet document named \`neutral\` (the digest of its recipe with the root token), whatever bytes that document had when it was taken: the shared entries an entry names travel inside every entry that names them, each time with other bytes. */
  async rememberNeutral(neutral,key){await mkdir(join(directory,"travel"),{recursive:true});await write(join(directory,"travel","n-"+neutral),key);},
  /** 🧷️ The recipe key of the entry that was taken from the fleet document named \`neutral\` (or that the document was refused for, because it does not hold here) and how long ago, or undefined. */
  async travelledNeutral(neutral){try{const path=join(directory,"travel","n-"+neutral),[text,info]=await Promise.all([readFile(path,"utf8"),stat(path)]);return{key:text.trim(),ageMilliseconds:Date.now()-info.mtimeMs};}catch{return undefined;}},
  /** 🔒️ Claims the intake of the fleet document named \`name\` for the process that asks: the answer is the release of the claim, or undefined while another process holds a claim younger than \`staleMilliseconds\` (a claim of a process that died is taken over after that time); a store that cannot hold claims answers a release that does nothing, so the caller goes on and takes the document itself. */
  async claim(name,staleMilliseconds){
   const parent=join(directory,"travel"),path=join(parent,"c-"+name);
   try{await mkdir(parent,{recursive:true});}catch{return async()=>undefined;}
   for(let attempt=0;attempt<2;attempt++){
    try{await mkdir(path);return async()=>{await rm(path,{recursive:true,force:true});};}
    catch(error){
     if(error?.code!=="EEXIST")return async()=>undefined;
     const info=await stat(path).catch(()=>undefined);
     if(info!==undefined&&Date.now()-info.mtimeMs<staleMilliseconds)return undefined;
     await rm(path,{recursive:true,force:true});
    }
   }
   return undefined;
  },
  /** 🔒️ Whether a claim of the intake of the fleet document named \`name\` exists. */
  async claimed(name){return(await stat(join(directory,"travel","c-"+name)).catch(()=>undefined))!==undefined;},
`;

const oldHeld='  async held(workspaceRoot, source) {\n    const store = closureStoreOfV1(workspaceRoot);\n    if (!store || !TRAVEL_DIGEST_V1.test(source)) return undefined;\n    try {\n      const key = await store.travelled(source);\n      return key !== undefined && TRAVEL_DIGEST_V1.test(key) && await store.stamp(workspaceRoot + "\\0" + key) !== undefined ? key : undefined;\n    } catch { return undefined; }\n  },\n});';
const newHeld=`  async held(workspaceRoot, source, neutral, refusedMilliseconds = 0) {
    const store = closureStoreOfV1(workspaceRoot);
    if (!store) return undefined;
    try {
      if (TRAVEL_DIGEST_V1.test(source)) {
        const key = await store.travelled(source);
        if (key !== undefined && TRAVEL_DIGEST_V1.test(key) && await store.stamp(workspaceRoot + "\\0" + key) !== undefined) return key;
      }
      if (typeof neutral === "string" && TRAVEL_DIGEST_V1.test(neutral)) {
        const known = await store.travelledNeutral(neutral);
        if (known !== undefined && TRAVEL_DIGEST_V1.test(known.key) && (known.ageMilliseconds < refusedMilliseconds || await storedEntryHoldsV1(workspaceRoot, store, workspaceRoot + "\\0" + known.key))) return known.key;
      }
    } catch { return undefined; }
    return undefined;
  },
  async claim(workspaceRoot, neutral, staleMilliseconds) {
    const store = closureStoreOfV1(workspaceRoot);
    if (!store || !TRAVEL_DIGEST_V1.test(neutral)) return async () => undefined;
    try { return await store.claim(neutral, staleMilliseconds); } catch { return async () => undefined; }
  },
  async claimed(workspaceRoot, neutral) {
    const store = closureStoreOfV1(workspaceRoot);
    if (!store || !TRAVEL_DIGEST_V1.test(neutral)) return false;
    try { return await store.claimed(neutral); } catch { return false; }
  },
});`;

const holdsCode=`/** 🪢️ Whether the entry the store holds under \`id\` still holds in this tree (loaded once into the entries of the process, so the next question and the next take of the entry find it there): what lets an intake leave a shared document it already has alone, whatever bytes another entry brings it in. */
async function storedEntryHoldsV1(workspaceRoot, store, id) {
  const scripts = closureScriptsV1.get(workspaceRoot);
  if (!scripts) return false;
  const known = closureEntriesV1.peek(id);
  if (known) return closureEntryHoldsAsyncV1(known, scripts);
  const stored = await store.load(id);
  if (!stored) return false;
  const entry = { ...stored, layers: undefined, store, answers: new Map(), checked: null };
  if (!await closureEntryHoldsAsyncV1(entry, scripts)) return false;
  closureEntriesV1.set(id, entry);
  return true;
}
export const closureTravelV1 = Object.freeze({`;

const oldHeldOf=`const heldOf=(travel:ClosureEntryTravelV1,held:Map<string,string>)=>async(entry:Readonly<{path:string;hash:string}>)=>{
  const named=DOCUMENT.exec(entry.path);
  if(named===null||travel.held===undefined)return false;
  try{const local=await travel.held(input.root,entry.hash);if(local===undefined)return false;held.set(named[1]!,local);return true;}catch{return false;}
 };`;
const newHeldOf=`type Scope=Readonly<{held:Map<string,string>;claims:Map<string,()=>Promise<void>>}>;
 const scopeOf=():Scope=>({held:new Map(),claims:new Map()});
 const releaseOf=async(scope:Scope,names:Iterable<string>=[...scope.claims.keys()])=>{for(const name of names){const release=scope.claims.get(name);if(release===undefined)continue;scope.claims.delete(name);await release().catch(()=>undefined);}};
 const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
 const heldOf=(travel:ClosureEntryTravelV1,scope:Scope)=>async(entry:Readonly<{path:string;hash:string}>)=>{
  const named=DOCUMENT.exec(entry.path);
  if(named===null||travel.held===undefined)return false;
  const neutral=named[1]!;
  if(scope.claims.has(neutral))return false;
  try{
   let local=await travel.held(input.root,entry.hash,neutral,policy.refusedMilliseconds);
   if(local===undefined&&travel.claim!==undefined){
    const release=await travel.claim(input.root,neutral,policy.claimStaleMilliseconds);
    if(release!==undefined){scope.claims.set(neutral,release);return false;}
    const until=Date.now()+policy.claimWaitMilliseconds;
    while(travel.claimed!==undefined&&Date.now()<until&&await travel.claimed(input.root,neutral))await pause(150);
    local=await travel.held(input.root,entry.hash,neutral,policy.refusedMilliseconds);
   }
   if(local===undefined)return false;
   scope.held.set(neutral,local);return true;
  }catch{return false;}
 };`;

const oldUnpack='const unpackOf=(row:Readonly<{name:string;bytes:Uint8Array}>)=>{try{return{recipeKey:row.name.slice(8),bundle:unpackBundleV1(JSON.parse(Buffer.from(row.bytes).toString("utf8")) as PackedClosureV1)};}catch{return undefined;}};';
const newUnpack='const unpackOf=(row:Readonly<{name:string;bytes:Uint8Array}>)=>{try{return{recipeKey:row.name.slice(8),bundle:unpackBundleV1(JSON.parse(Buffer.from(row.bytes.buffer,row.bytes.byteOffset,row.bytes.byteLength).toString("utf8")) as PackedClosureV1)};}catch{return undefined;}};';

const oldIntake=`const intake=(documents:readonly Readonly<{name:string;bytes:Uint8Array}>[],note:GraphEntryNoteV1,travel:ClosureEntryTravelV1,held:ReadonlyMap<string,string>)=>serial(async()=>{
  try{
   const named=documents.filter(row=>row.name.startsWith("closure-")),found=new Map<string,string>(held);let any=false,owned=false;
   for(const row of named){const entry=unpackOf(row);if(entry===undefined)continue;any=true;for(const [from,to] of bundleKeysV1([entry],host,"local"))found.set(from,to);owned||=Buffer.from(row.bytes).includes(portableRowsPolicyV1.ownerToken);}
   if(!any)return 0;
   const context={providers:owned?await providersOf():[],generation:(input.generation??dependencyGenerationV1)(input.root)},sources=new Map(note.documents.flatMap(row=>row.name.endsWith(".br")?[[row.name.slice(0,-3),row.hash] as const]:[]));
   let taken=0;
   for(const row of named){
    const entry=unpackOf(row);if(entry===undefined)continue;
    const moved=localBundleV1(entry,host,found,context),source=sources.get("closure-"+entry.recipeKey);
    if(moved!==undefined)taken+=await travel.import(input.root,[source===undefined?moved:{...moved,source}]);
    
   }
   counts.imported+=taken;return taken;
  }catch{return 0;}
 });`;
const newIntake=`const closureName=(name:string)=>name.startsWith("closure-");
 const intake=(deferred:ReadonlyMap<string,GraphDeferredV1>,travel:ClosureEntryTravelV1,scope:Scope)=>serial(async()=>{
  const names=[...deferred.keys()].filter(closureName).sort();
  try{
   const found=new Map<string,string>(scope.held);let any=false,owned=false;
   for(const name of names){
    const document=await deferred.get(name)!.read({keep:true});if(document===undefined)continue;
    const head=closureDocumentHeadV1(document.bytes);if(head===undefined)continue;
    any=true;owned||=head.owned;
    for(const [from,to] of bundleKeysV1([{recipeKey:name.slice(8),bundle:{recipe:head.recipe} as unknown as ClosureBundleV1}],host,"local"))found.set(from,to);
   }
   if(!any)return 0;
   const context={providers:owned?await providersOf():[],generation:(input.generation??dependencyGenerationV1)(input.root)};
   let taken=0;
   for(const name of names){
    try{
     const waiting=deferred.get(name)!,document=await waiting.read();if(document===undefined)continue;
     const entry=unpackOf(document);if(entry===undefined)continue;
     const moved=localBundleV1(entry,host,found,context),source=waiting.hash;
     if(moved!==undefined)taken+=await travel.import(input.root,[{...moved,...source===undefined?{}:{source},neutral:entry.recipeKey}]);
    }finally{await releaseOf(scope,[name.slice(8)]);}
   }
   counts.imported+=taken;return taken;
  }catch{return 0;}finally{await releaseOf(scope,names.map(name=>name.slice(8)));}
 });`;

const oldTake=`  async take(recipe,travel){
   const held=new Map<string,string>();
   try{
    const decision=await decideGraphEntryV1({client,state:stateOf(recipe),owner,expectedMilliseconds:policy.expectedMilliseconds,waitLimitMilliseconds:Math.max(1,Math.min(policy.waitMilliseconds,policy.waitBudgetMilliseconds-waitedMilliseconds)),skip:heldOf(travel,held)});
    waitedMilliseconds+=decision.waitedMs;
    if(decision.outcome==="hit"){await intake(decision.documents,decision.note,travel,held);counts.fetched++;return{outcome:"taken"} as const;}
    counts.computed++;return{outcome:"compute",hold:decision.hold} as const;
   }catch{counts.computed++;return{outcome:"compute",hold:undefined} as const;}
  },`;
const newTake=`  async take(recipe,travel){
   const scope=scopeOf();
   try{
    const decision=await decideGraphEntryV1({client,state:stateOf(recipe),owner,expectedMilliseconds:policy.expectedMilliseconds,waitLimitMilliseconds:Math.max(1,Math.min(policy.waitMilliseconds,policy.waitBudgetMilliseconds-waitedMilliseconds)),skip:heldOf(travel,scope),defer:closureName});
    waitedMilliseconds+=decision.waitedMs;
    if(decision.outcome==="hit"){await intake(decision.deferred,travel,scope);counts.fetched++;return{outcome:"taken"} as const;}
    counts.computed++;return{outcome:"compute",hold:decision.hold} as const;
   }catch{counts.computed++;return{outcome:"compute",hold:undefined} as const;}finally{await releaseOf(scope);}
  },`;

const oldPrefetch=`    const held=new Map<string,string>();
    try{
     const found=await fetchGraphEntryV1(client,stateOf(recipe),heldOf(travel,held));
     if(found===undefined||found==="fault"){counts.missed++;return;}
     if(await intake(found.documents,found.note,travel,held)>0){taken++;counts.prefetched++;}
    }catch{counts.missed++;}`;
const newPrefetch=`    const scope=scopeOf();
    try{
     const found=await fetchGraphEntryV1(client,stateOf(recipe),heldOf(travel,scope),closureName);
     if(found===undefined||found==="fault"){counts.missed++;return;}
     if(await intake(found.deferred,travel,scope)>0){taken++;counts.prefetched++;}
    }catch{counts.missed++;}finally{await releaseOf(scope);}`;

const oldTravelType='export type ClosureEntryTravelV1=Readonly<{export(root:string,image:string):Promise<Bundles>;import(root:string,bundles:readonly Readonly<{recipeKey:string;bundle:unknown;source?:string}>[]):Promise<number>;held?(root:string,source:string):Promise<string|undefined>}>;';
const newTravelType='export type ClosureEntryTravelV1=Readonly<{export(root:string,image:string):Promise<Bundles>;import(root:string,bundles:readonly Readonly<{recipeKey:string;bundle:unknown;source?:string;neutral?:string}>[]):Promise<number>;held?(root:string,source:string,neutral?:string,refusedMilliseconds?:number):Promise<string|undefined>;claim?(root:string,neutral:string,staleMilliseconds:number):Promise<(()=>Promise<void>)|undefined>;claimed?(root:string,neutral:string):Promise<boolean>}>;';

export const all=[
 {file:SI,change:edit(' footprintPulseV1();releaseIfGrownV1({every:16});',' footprintPulseV1();releaseIfGrownV1(closureTakeV1.getStore() === true ? { every: 16, threshold: FOOTPRINT_POLICY_V1.takeReleaseGrowthBytes } : { every: 16 });')},
 {file:SI,change:edit('import { footprintPulseV1, releaseIfGrownV1,','import { FOOTPRINT_POLICY_V1, footprintPulseV1, releaseIfGrownV1,')},
 {file:FPS,change:edit('releaseGrowthBytes the managed growth after which releaseIfGrownV1 collects and shrinks; commandTimeoutMilliseconds of one reading.','releaseGrowthBytes the managed growth after which releaseIfGrownV1 collects and shrinks, takeReleaseGrowthBytes the same inside a take pass, which only re-reads stored rows and compiles nothing, so its collection waits for four times the growth (one collection per pressure event, never one per owner); commandTimeoutMilliseconds of one reading.')},
 {file:FPS,change:edit('releaseGrowthBytes das verwaltete Wachstum, ab dem releaseIfGrownV1 sammelt und schrumpft; commandTimeoutMilliseconds einer Messung.','releaseGrowthBytes das verwaltete Wachstum, ab dem releaseIfGrownV1 sammelt und schrumpft, takeReleaseGrowthBytes dasselbe innerhalb eines Übernahmedurchgangs, der nur gespeicherte Zeilen liest und nichts übersetzt, sodass seine Sammlung auf das Vierfache des Wachstums wartet (eine Sammlung je Druckereignis, nie eine je Projekt); commandTimeoutMilliseconds einer Messung.')},
 {file:FPD,change:edit('readonly releaseGrowthBytes:number;','readonly releaseGrowthBytes:number;readonly takeReleaseGrowthBytes:number;')},
 {file:FPS,change:edit('"releaseGrowthBytes": 268435456,\n','"releaseGrowthBytes": 268435456,\n        "takeReleaseGrowthBytes": 1073741824,\n')},
 {file:SI,change:edit('export const closureTravelV1 = Object.freeze({',holdsCode)},
 {file:SI,change:edit('for (const { recipeKey, bundle, source } of bundles) {','for (const { recipeKey, bundle, source, neutral } of bundles) {')},
 {file:SI,change:edit('        if (typeof source === "string" && TRAVEL_DIGEST_V1.test(source)) await store.remember(source, recipeKey);\n','        if (typeof source === "string" && TRAVEL_DIGEST_V1.test(source)) await store.remember(source, recipeKey);\n        if (typeof neutral === "string" && TRAVEL_DIGEST_V1.test(neutral)) await store.rememberNeutral(neutral, recipeKey);\n')},
 {file:SI,change:edit(oldHeld,newHeld)},
 {file:SID,change:edit('held(workspaceRoot: string, source: string): Promise<string | undefined> }>;','held(workspaceRoot: string, source: string, neutral?: string, refusedMilliseconds?: number): Promise<string | undefined>; claim(workspaceRoot: string, neutral: string, staleMilliseconds: number): Promise<(() => Promise<void>) | undefined>; claimed(workspaceRoot: string, neutral: string): Promise<boolean> }>;')},
 {file:SID,change:edit('bundles: readonly Readonly<{ recipeKey: string; bundle: unknown; source?: string }>[]): Promise<number>;','bundles: readonly Readonly<{ recipeKey: string; bundle: unknown; source?: string; neutral?: string }>[]): Promise<number>;')},
 {file:EN,change:edit('  /** 🧹️ Waits for the removal of the oldest records beyond',storeCode+'  /** 🧹️ Waits for the removal of the oldest records beyond')},
 {file:PORT,change:edit('/** 📖️ The neutral closure bundle of a stored form (rows and layers share their row objects); throws when the form does not hold together. */',headCode.slice(1)+'\n/** 📖️ The neutral closure bundle of a stored form (rows and layers share their row objects); throws when the form does not hold together. */')},
 {file:LANES,change:edit('"required": ["form", "waitMilliseconds", "waitBudgetMilliseconds", "expectedMilliseconds", "fetchConcurrency", "offersInFlight"],','"required": ["form", "waitMilliseconds", "waitBudgetMilliseconds", "expectedMilliseconds", "fetchConcurrency", "offersInFlight", "claimStaleMilliseconds", "claimWaitMilliseconds", "refusedMilliseconds"],')},
 {file:LANES,change:edit('"offersInFlight": { "type": "integer", "minimum": 1 }\n','"offersInFlight": { "type": "integer", "minimum": 1 },\n    "claimStaleMilliseconds": { "type": "integer", "minimum": 1 },\n    "claimWaitMilliseconds": { "type": "integer", "minimum": 1 },\n    "refusedMilliseconds": { "type": "integer", "minimum": 1 }\n')},
 {file:LANES,change:edit('"fetchConcurrency": 2, "offersInFlight": 1 }','"fetchConcurrency": 2, "offersInFlight": 1, "claimStaleMilliseconds": 120000, "claimWaitMilliseconds": 60000, "refusedMilliseconds": 600000 }')},
 {file:LANE,change:edit('import{bundleKeysV1,closureKeyV1,dependencyGenerationV1,','import{bundleKeysV1,closureDocumentHeadV1,closureKeyV1,dependencyGenerationV1,')},
 {file:LANE,change:edit('type GraphEntryNoteV1,type GraphFleetClientV1','type GraphDeferredV1,type GraphEntryNoteV1,type GraphFleetClientV1')},
 {file:LANE,change:edit(oldTravelType,newTravelType)},
 {file:LANE,change:edit(oldHeldOf,newHeldOf)},
 {file:LANE,change:edit(oldUnpack,newUnpack)},
 {file:LANE,change:edit(oldIntake,newIntake)},
 {file:LANE,change:edit(oldTake,newTake)},
 {file:LANE,change:edit(oldPrefetch,newPrefetch)},
];

const oldRecParse=`    try{
     const packed=JSON.parse(view(document.bytes).toString("utf8")) as PackedClosureV1;
     if(packed===null||typeof packed!=="object"||packed.version!==2||packed.recipe===null||typeof packed.recipe!=="object")return;
     const has=view(document.bytes).includes(portableRowsPolicyV1.ownerToken);
     recipes.set(name.slice(8),packed.recipe);owned||=has;
     if(entry.hash!==undefined){recipeMemo.delete(entry.hash);recipeMemo.set(entry.hash,{recipe:packed.recipe,owned:has});while(recipeMemo.size>MAXIMUM_RECIPES)recipeMemo.delete(recipeMemo.keys().next().value!);}
    }catch{}`;
const newRecParse=`    try{
     const head=closureDocumentHeadV1(document.bytes);
     if(head===undefined)return;
     recipes.set(name.slice(8),head.recipe);owned||=head.owned;
     if(entry.hash!==undefined){recipeMemo.delete(entry.hash);recipeMemo.set(entry.hash,{recipe:head.recipe,owned:head.owned});while(recipeMemo.size>MAXIMUM_RECIPES)recipeMemo.delete(recipeMemo.keys().next().value!);}
    }catch{}`;
all.push(
 {file:REC,change:edit('import{bundleKeysV1,consultedHoldV1,','import{bundleKeysV1,closureDocumentHeadV1,consultedHoldV1,')},
 {file:REC,change:edit('import(root:string,bundles:readonly Readonly<{recipeKey:string;bundle:unknown;source?:string}>[]):Promise<number>;held?(root:string,source:string):Promise<string|undefined>}>;','import(root:string,bundles:readonly Readonly<{recipeKey:string;bundle:unknown;source?:string;neutral?:string}>[]):Promise<number>;held?(root:string,source:string,neutral?:string,refusedMilliseconds?:number):Promise<string|undefined>}>;')},
 {file:REC,change:edit('try{const local=await info.closures.held(input.root,entry.hash);if(local===undefined)return false;held.set(named[1]!,local);return true;}catch{return false;}','try{const local=await info.closures.held(input.root,entry.hash,named[1]!);if(local===undefined)return false;held.set(named[1]!,local);return true;}catch{return false;}')},
 {file:REC,change:edit(oldRecParse,newRecParse)},
 {file:REC,change:edit('return moved===undefined?0:await info.closures!.import(input.root,[hash===undefined?moved:{...moved,source:hash}]);','return moved===undefined?0:await info.closures!.import(input.root,[{...moved,...hash===undefined?{}:{source:hash},neutral:name.slice(8)}]);')},
);

all.push(
 {file:SI,change:edit('async function bundleSupersedesStoredV1(workspaceRoot, store, stored, bundle) {\n  const scripts = closureScriptsV1.get(workspaceRoot), step = cooperativeStepV1(scripts, "build:closure-entry-rows");\n  if (!await observationRowsHoldAsyncV1(bundle.rows, step)) return false;\n  for (const layer of bundle.layers) if (!await observationLayerHoldsAsyncV1(layer, step)) return false;\n  return !await','/** 🧪️ Whether the rows and the layers of a bundle hold in this tree (true while no scripts are bound, where nothing can be decided): a bundle that does not hold is never stored, since every consumer checks the rows of a stored entry against its own tree and derives it again when they do not hold, so storing it only costs the write. */\nasync function bundleHoldsV1(workspaceRoot, bundle) {\n  const scripts = closureScriptsV1.get(workspaceRoot);\n  if (!scripts) return true;\n  const step = cooperativeStepV1(scripts, "build:closure-entry-rows");\n  if (!await observationRowsHoldAsyncV1(bundle.rows, step)) return false;\n  for (const layer of bundle.layers) if (!await observationLayerHoldsAsyncV1(layer, step)) return false;\n  return true;\n}\nasync function bundleSupersedesStoredV1(workspaceRoot, store, stored, bundle) {\n  const scripts = closureScriptsV1.get(workspaceRoot);\n  if (!await bundleHoldsV1(workspaceRoot, bundle)) return false;\n  return !await')},
 {file:SI,change:edit('if (stored === undefined || await bundleSupersedesStoredV1(workspaceRoot, store, stored, bundle)) { await store.adopt(id, bundle); taken++; }','if (stored === undefined ? await bundleHoldsV1(workspaceRoot, bundle) : await bundleSupersedesStoredV1(workspaceRoot, store, stored, bundle)) { await store.adopt(id, bundle); taken++; }')},
);

const supersedesDoc='/** 🔁️ Whether a bundle taken from the fleet replaces the entry this store already holds under its recipe: it does when the stored entry no longer holds in this tree and the bundle does, so a stale entry (derived before a change, or taken with a row that could never hold) never blocks an entry another machine derived for the same inputs; a bundle that does not hold here leaves the stored entry alone. */\n';
const holdsDoc='/** 🧪️ Whether the rows and the layers of a bundle hold in this tree (true while no scripts are bound, where nothing can be decided): a bundle that does not hold is never stored, since every consumer checks the rows of a stored entry against its own tree and derives it again when they do not hold, so storing it only costs the write. */\nasync function bundleHoldsV1(workspaceRoot, bundle) {\n  const scripts = closureScriptsV1.get(workspaceRoot);\n  if (!scripts) return true;\n  const step = cooperativeStepV1(scripts, "build:closure-entry-rows");\n  if (!await observationRowsHoldAsyncV1(bundle.rows, step)) return false;\n  for (const layer of bundle.layers) if (!await observationLayerHoldsAsyncV1(layer, step)) return false;\n  return true;\n}\nasync function bundleSupersedesStoredV1(';
const newHoldsDoc='/** 🧪️ Whether the own rows of a bundle hold in this tree (true while no scripts are bound, where nothing can be decided): a bundle whose own rows do not hold is stale and is not stored, since every consumer checks the rows of a stored entry against its own tree and derives it again when they do not hold, so storing it only costs the write; its layers are left to that check, which a bundle that holds pays anyway. */\nasync function bundleRowsHoldV1(workspaceRoot, bundle) {\n  const scripts = closureScriptsV1.get(workspaceRoot);\n  if (!scripts) return true;\n  return observationRowsHoldAsyncV1(bundle.rows, cooperativeStepV1(scripts, "build:closure-entry-rows"));\n}\n/** 🧪️ Whether the rows and the layers of a bundle hold in this tree (true while no scripts are bound, where nothing can be decided). */\nasync function bundleHoldsV1(workspaceRoot, bundle) {\n  const scripts = closureScriptsV1.get(workspaceRoot);\n  if (!scripts) return true;\n  const step = cooperativeStepV1(scripts, "build:closure-entry-rows");\n  if (!await observationRowsHoldAsyncV1(bundle.rows, step)) return false;\n  for (const layer of bundle.layers) if (!await observationLayerHoldsAsyncV1(layer, step)) return false;\n  return true;\n}\n'+supersedesDoc.trimEnd()+'\nasync function bundleSupersedesStoredV1(';
export const second=[
 {file:SI,change:edit(supersedesDoc+holdsDoc,newHoldsDoc)},
 {file:SI,change:edit('if (stored === undefined ? await bundleHoldsV1(workspaceRoot, bundle) :','if (stored === undefined ? await bundleRowsHoldV1(workspaceRoot, bundle) :')},
];
export const apply=(file,text,list=all)=>list.filter(item=>file.endsWith(item.file)).reduce((value,item)=>item.change(value),text);
