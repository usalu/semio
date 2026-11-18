import sys,re
def once(t,old,new,what=""):
    n=t.count(old)
    if n!=1: raise SystemExit("anchor %s found %d times: %s"%(what,n,old[:90]))
    return t.replace(old,new)
def patch(t):
    t=once(t,'import {buildGraphRetryDiagnosticV1} from "../🔁retry/🟨️.mjs";','import {buildGraphRetryDiagnosticV1,sealVerdictV1,reportSealV1} from "../🔁retry/🟨️.mjs";',"import")
    old_initial='''  let initial:Awaited<ReturnType<typeof inventory>>;try{initial=await inventory();}catch(error){if(buildGraphErrorCodeV1(error)==="BUILD_MEMBERSHIP_CHANGED")return invalidated("membership");throw error;}'''
    new_initial='''  const settledInventory=async(orNothing:boolean):Promise<Awaited<ReturnType<typeof inventory>>|undefined>=>{for(let tries=0;;tries++){try{return await inventory();}catch(error){if(buildGraphErrorCodeV1(error)!=="BUILD_MEMBERSHIP_CHANGED"||tries>=GRAPH_LIMITS_V1.repairRounds)if(orNothing&&buildGraphErrorCodeV1(error)==="BUILD_MEMBERSHIP_CHANGED")return undefined;else throw error;}}};
  const initial=(await settledInventory(false))!;'''
    t=once(t,old_initial,new_initial,"initial")
    t=once(t,'''if(await buildGraphSourceFactsHoldV1([...initial.observations,...cachedReceipt.observations],invocation))return graph;return undefined;}''','''if(await buildGraphSourceFactsHoldV1([...initial.observations,...cachedReceipt.observations],invocation))return graph;}''',"cached-undefined")
    a=t.index("  let captured:{value:BuildGraphV1|OwnerRegistryInvalidationV1;observations:readonly unknown[]},failedFacts")
    marker='''  if(before!==after||!holds)return invalidated(before!==after?"membership":"observed-facts",{before,after,stale});\n'''
    b=t.index(marker)+len(marker)
    block=t[a:b]
    # reuse the self-reading lines of discoverOnce unchanged
    d0=block.index("  const discoverOnce=()=>observation.captureObservedSourceInputsV1(async()=>{")
    d1=block.index("  const discoveryFailure=")
    discover=block[d0:d1]
    discover=once(discover,"const discoverOnce=()=>","const discoverOnce=(on:Basis)=>","d1")
    discover=once(discover,"discoverBuildGraphV1(root,options.selection,registry!,invocation,phaseControl,files,initial.admission,true);","discoverBuildGraphV1(root,options.selection,registry!,invocation,phaseControl,on.files,on.admission,true);","d2")
    new_block='''  let captured!:{value:BuildGraphV1;observations:readonly unknown[]};
  let basis=initial as Basis,failedFacts:unknown[]=[],failedAuthority:unknown[]=[];
  phaseStarted=performance.now();
'''+discover+'''  type Basis=Exclude<Awaited<ReturnType<typeof inventory>>,OwnerRegistryInvalidationV1>;
  const stale:{kind:string;path:string;fact:unknown}[]=[];let staleCount=0,rounds=0,previousStale="",membershipMoved=false,admissionMoved=false;
  const collect=(row:{kind:string;path:string;fact:unknown})=>{staleCount++;if(stale.length<GRAPH_LIMITS_V1.sealStalePaths)stale.push(row);},sameAdmission=(left:Basis,right:Basis)=>!selected||JSON.stringify(left.admission)===JSON.stringify(right.admission);
  const followed=(current:Awaited<ReturnType<typeof inventory>>|undefined)=>{rounds++;if(current!==undefined&&!isOwnerRegistryInvalidationV1(current))basis=current;};
  for(;;){
    stale.length=0;staleCount=0;
    let produced:{value:BuildGraphV1|OwnerRegistryInvalidationV1;observations:readonly unknown[]};
    try{produced=await discoverOnce(basis);}catch(error){
      if(invocation)checkScriptInvocation(invocation);if(buildGraphErrorCodeV1(error)==="BUILD_AUTHORITY_CHANGED")throw error;
      const current=await settledInventory(true);if(current!==undefined&&isOwnerRegistryInvalidationV1(current))throw error;
      const holds=await buildGraphSourceFactsHoldV1(failedFacts,invocation,collect);
      if(holds&&current!==undefined&&current.digest===basis.digest&&sameAdmission(basis,current)||rounds>=GRAPH_LIMITS_V1.repairRounds)throw error;
      process.stderr.write("[graph-repair] "+JSON.stringify({round:rounds+1,failure:true,stale:stale.map(row=>row.path)})+"\\n");followed(current);continue;
    }
    if(isOwnerRegistryInvalidationV1(produced.value)){invalidated("owner-registry",{before:basis.digest});return produced.value;}
    if(rounds===0)timing("graph-original-inference",phaseStarted,{observations:produced.observations.length});
    const current=await settledInventory(true);
    if(current!==undefined&&isOwnerRegistryInvalidationV1(current)){invalidated("owner-registry",{before:basis.digest});return current;}
    const observed=[...(current?.observations??[]),...produced.observations],holds=await buildGraphSourceFactsHoldV1(observed,invocation,collect);
    membershipMoved=current===undefined||current.digest!==basis.digest;admissionMoved=current!==undefined&&!sameAdmission(basis,current);
    captured={value:produced.value as BuildGraphV1,observations:observed};
    if(holds&&!membershipMoved&&!admissionMoved)break;
    const signature=JSON.stringify(stale.map(row=>[row.path,row.fact]).sort());
    if(staleCount>0&&!membershipMoved&&!admissionMoved&&signature===previousStale){process.stderr.write("[graph-retry] "+JSON.stringify({version:1,reason:"unrefreshed-facts",stale:stale.map(row=>row.path)})+"\\n");throw observation.staleSourceFactsErrorV1(stale);}
    if(rounds>=GRAPH_LIMITS_V1.repairRounds)break;
    previousStale=signature;const repairStarted=performance.now();process.stderr.write("[graph-repair] "+JSON.stringify({round:rounds+1,stale:stale.slice(0,8).map(row=>row.path),staleCount,membershipMoved,admissionMoved})+"\\n");followed(current);timing("graph-repair",repairStarted,{round:rounds,holds});
  }
  const verdict=sealVerdictV1({rounds,staleCount,stale,membershipMoved,admissionMoved,projects:captured.value.projects as never,rows:captured.observations as never});
  reportSealV1(verdict);process.stderr.write("[graph-seal] "+JSON.stringify({version:1,mode:verdict.mode,rounds,sealedWithLaterChanges:verdict.sealedWithLaterChanges,laterChangedOwners:verdict.laterChangedOwners,membershipMoved,admissionMoved,owners:Object.keys(verdict.owners).length})+"\\n");
'''
    t=t[:a]+new_block+t[b:]
    t=once(t,'authority=JSON.stringify({version:3,module:loadedGraphModuleAuthorityV1,membership:after,graph:digest,observations:captured.observations});','authority=JSON.stringify({version:3,module:loadedGraphModuleAuthorityV1,membership:basis.digest,graph:digest,seal:verdict,observations:captured.observations});',"authority")
    t=once(t,'''if(invocation)checkScriptInvocation(invocation);if(!await buildGraphSourceFactsHoldV1(captured.observations,invocation,row=>{if(stale.length<8)stale.push(row);}))return invalidated("publication-facts",{before,after,stale});timing("graph-publication"''','''if(invocation)checkScriptInvocation(invocation);timing("graph-publication"''',"publication")
    t=once(t,"const publishStarted=performance.now(),compact=captured.value as BuildGraphV1,","const publishStarted=performance.now(),compact=captured.value,","compact")
    return t
if __name__=="__main__":
    src=open(sys.argv[1],encoding="utf8").read();open(sys.argv[2],"w",encoding="utf8").write(patch(src))
