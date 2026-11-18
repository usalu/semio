import sys
def patch(text):
    def once(old, new):
        nonlocal text
        assert text.count(old) == 1, (text.count(old), old[:80])
        text = text.replace(old, new)
    once('/** 🔒️ The question a taker puts to the `closure` rows of a record',
'''/** 🧬️ Whether one closure entry a stored record carries still holds on this tree, the verdict a taker gives the entries it imported: the rows of the entry and of its layers probed against the files, listings and analyses of the tree, in steps that give the event loop its turns. A record whose entries walked files that changed since is stale for every reader although its own rows hold, so a pool that counted it as stored never refreshed it.
 * @see ./🧪️tests/🤝️pool-reader/🟦️.ts */
export async function closureBundleHoldsV1(bundle:Readonly<{rows:readonly unknown[];layers:Iterable<readonly unknown[]>}>):Promise<boolean>{
 const observation=await import(OBSERVATION) as {observationRowsHoldAsyncV1(rows:readonly unknown[],step:()=>unknown):Promise<boolean>;observationLayerHoldsAsyncV1(layer:readonly unknown[],step:()=>unknown):Promise<boolean>};
 let turns=0;const step=()=>++turns%2048===0?new Promise<void>(done=>setImmediate(done)):undefined;
 if(!await observation.observationRowsHoldAsyncV1([...bundle.rows],step))return false;
 for(const layer of bundle.layers)if(!await observation.observationLayerHoldsAsyncV1(layer,step))return false;
 return true;
}

/** 🔒️ The question a taker puts to the `closure` rows of a record''')
    once('const staleNoteOf=async(state:GraphStateV1):Promise<string|undefined>=>{',
'''const closureVerdicts=new Map<string,Readonly<{holds:boolean;atMs:number}>>();
 const closuresHoldOf=async(decision:Readonly<{documents:readonly Readonly<{name:string;bytes:Uint8Array}>[];deferred:ReadonlyMap<string,GraphDeferredV1>;note:GraphEntryNoteV1}>,keys:ReadonlyMap<string,string>,owned:boolean):Promise<boolean>=>{
  const lane=laneOf(decision,new Map(decision.note.documents.flatMap(row=>row.name.endsWith(".br")?[[row.name.slice(0,-3),row.hash] as const]:[])));
  if(lane.size===0)return true;
  const found=await recipesOf(lane),context={providers:owned||found.owned?await providersOf():[],generation:(input.generation??dependencyGenerationV1)(input.root)},all=new Map([...keys,...bundleKeysV1([...found.recipes].map(([recipeKey,recipe])=>({recipeKey,bundle:{recipe} as unknown as ClosureBundleV1})),host,"local")]);
  for(const [name,entry] of lane){
   const known=entry.hash===undefined?undefined:closureVerdicts.get(entry.hash);
   if(known!==undefined&&Date.now()-known.atMs<FRESH_POLICY_V1.keptMilliseconds){if(!known.holds)return false;continue;}
   const holds=await within(async()=>{
    const document=await entry.read({keep:true});if(document===undefined)return true;
    try{
     const moved=localBundleV1({recipeKey:name.slice(8),bundle:unpackBundleV1(JSON.parse(view(document.bytes).toString("utf8")) as PackedClosureV1)},host,all,context);
     if(moved===undefined)return false;
     return await closureBundleHoldsV1(moved.bundle);
    }catch{return false;}
   });
   if(entry.hash!==undefined){closureVerdicts.delete(entry.hash);closureVerdicts.set(entry.hash,{holds,atMs:Date.now()});while(closureVerdicts.size>FRESH_POLICY_V1.verdictsKept)closureVerdicts.delete(closureVerdicts.keys().next().value!);}
   if(!holds)return false;
  }
  return true;
 };
 const staleNoteOf=async(state:GraphStateV1):Promise<string|undefined>=>{''')
    once('const found=await fetchGraphEntryV1(client,state,entry=>entry.path!=="record.br");',
         'const found=await fetchGraphEntryV1(client,state,entry=>entry.path!=="record.br"&&!isClosureDocument(entry.path),isClosureDocument);')
    once('''   if(verdict.fresh)keepFresh(noteHash);verdicts.set(noteHash,{fresh:verdict.fresh,atMs:Date.now()});''',
'''   const fresh=verdict.fresh&&(found.deferred.size===0&&!found.documents.some(row=>isClosureDocument(row.name))||clock()-found.note.createdMs<FRESH_POLICY_V1.minimumAgeMilliseconds||await closuresHoldOf(found,taken.keys,text.includes(portableRowsPolicyV1.ownerToken)));
   if(fresh)keepFresh(noteHash);verdicts.set(noteHash,{fresh,atMs:Date.now()});''')
    once('   return verdict.fresh?undefined:noteHash;','   return fresh?undefined:noteHash;')
    return text
if __name__=="__main__":
    src,dst=sys.argv[1],sys.argv[2]
    open(dst,"w").write(patch(open(src).read()))
