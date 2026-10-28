"""[hold-g] one compile-atomic edit of the graph module: claims import, shared namespace writer for the builder, host.prefetch, the two new functions."""
import sys
G = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts"
s = open(G, encoding="utf8").read()
def rep(a, b, count=1):
    global s
    assert s.count(a) == count, (a[:80], s.count(a))
    s = s.replace(a, b)
rep('import{createGenerationStoreV1,createNodeFilePortV1,createFrontierV1,diffFrontiersV1,frontierHoldsV1,stableDirectoriesV1,canonicalJsonV1 as canonicalGenerationJsonV1}from"./🕰️generations/🟨️.mjs";',
    'import{GENERATION_POLICY_V1,createGenerationStoreV1,createNodeFilePortV1,createFrontierV1,diffFrontiersV1,frontierHoldsV1,readGenerationPointerV1,stableDirectoriesV1,canonicalJsonV1 as canonicalGenerationJsonV1}from"./🕰️generations/🟨️.mjs";\nimport{createClaimRegistryV1,prepareStaleProjectsV1,renderClaimMessageV1,slotCountV1,type ClaimMessageKeyV1}from"./🕰️generations/🧩️claims/🟨️.mjs";\nimport{totalmem}from"node:os";')
rep('scope===generationScopeV1?{optionalCaches:false}:undefined', 'scope===generationScopeV1?{optionalCaches:false,shared:true}:undefined')
rep('    say:(key:string,fields:Record<string,string|number>)=>{process.stderr.write(renderGenerationMessageV1(key as never,fields,resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language)+"\\n");},\n  };\n}',
    '    say:(key:string,fields:Record<string,string|number>)=>{process.stderr.write(renderGenerationMessageV1(key as never,fields,resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language)+"\\n");},\n    prefetch(request:GenerationPrefetchRequestV1){return prefetchStaleGenerationV1({root,invocation,phaseControl,registry,files:()=>state.files,request,release:()=>this.release(),language:resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language});},\n  };\n}')
anchor = '/** 🔍️ Names the first difference between a generation graph and the graph a full inference produced;'
new = '''/** 🧩️ The stale projects of a pending refresh: the generation they were planned against, their names, their roots and the implementation digest of the planner. */
type GenerationPrefetchRequestV1={readonly generation:number;readonly dirty:readonly string[];readonly roots:Readonly<Record<string,string>>;readonly implementation:string};

/** 🔗️ The lease port of the claim registry over the build lease directory: `tryHold` takes an exclusive lease at once or answers undefined, `hold` queues for it, `wait` returns when no exclusive holder is left. */
function claimLeasePortV1(directory:string,control:ResourceLeaseControl){
  const request=(resource:string,mode:"shared"|"exclusive",signal?:AbortSignal,timeoutMs?:number)=>acquireQueuedResourceLease({directory,resource,mode,owner:randomBytes(16).toString("base64url"),control,signal:signal??control.signal,...(timeoutMs!==undefined?{timeoutMs}:{})});
  return{
    async tryHold(resource:string){try{return await request(resource,"exclusive",undefined,0);}catch(error){if(/timed out/iu.test(String((error as Error).message)))return undefined;throw error;}},
    hold:(resource:string,options?:{signal?:AbortSignal})=>request(resource,"exclusive",options?.signal),
    async wait(resource:string,options?:{signal?:AbortSignal}){(await request(resource,"shared",options?.signal)).release();},
  };
}

/** 🧩️ Fills the project records of the owners whose roots are in `roots` without assembling anything: the inference the build runs for them with the options it runs it with (so the records are the ones the build finds warm), without the test and policy contributions, the edges and the environment. */
export async function prefetchBuildGraphProjectsV1(root:string,registry:BuildOwnerRegistryV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,files:readonly string[],roots:ReadonlySet<string>):Promise<number> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);registry=readBuildOwnerRegistryV1(registry);
  requireBuildOwnerRegistryAuthorityV1(registry,JSON.parse(pinnedAuthorityReadV1(join(root,"📋️build-owners.json"),()=>observationV1.observedFs.readFileSync(join(root,"📋️build-owners.json"),"utf8"))));
  const owner=await import("./📥️inference/🟨️.mjs");checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();
  const context={workspaceRoot:root,invocation,phaseControl,registry};
  const captured=await observationV1.captureObservedSourceInputsV1(async()=>{
    const options={analyzeLockfile:false,catalogProviders:registry.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})),snapshot:true,ownerRoots:roots,onOwnerObservations:()=>{}};
    return (await owner.discoverBuildProjectsV1(files.filter(file=>file.endsWith("📋️project.json")||file.endsWith("Cargo.toml")||javascriptManagerV1.lockFileNames().includes(file)||file.endsWith(".patch")),options,context)).length;
  },undefined,false);
  return captured.value;
}

/** 🤝️ Prepares the stale projects of a pending refresh together with the commands that wait for the same graph, before the exclusive build: claims a chunk of them, fills their project records in chunks that release the heap in between, joins what a peer computes, and never fails the command (the build computes what is left and reports a real failure). */
async function prefetchStaleGenerationV1(input:{root:string;invocation:ScriptInvocation;phaseControl:ResourceLeaseControl;registry:BuildOwnerRegistryV1;files:()=>readonly string[];request:GenerationPrefetchRequestV1;release:()=>Promise<void>;language:"en"|"de"}):Promise<void> {
  const{root,invocation,phaseControl,registry,request}=input;
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);requireCurrentGraphModuleV1();
  const say=(key:ClaimMessageKeyV1,fields:Record<string,string|number>)=>{process.stderr.write(renderClaimMessageV1(key,fields,input.language)+"\\n");};
  await withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,generationScopeV1,async(store,control)=>{
    const files=generationFilesV1(store),directory=join(store.root,"generations"),clock=createGraphPhaseClockV1({publish:resource=>control.publish({resource,mode:"shared",elapsedMs:0}),write:writeGraphPhaseRowV1});
    const claims=createClaimRegistryV1({root:directory,fs:files as never,lease:claimLeasePortV1(join(root,".cache/b/l/m/data"),control)});
    let begun=false;
    try{
      await prepareStaleProjectsV1({registry:claims,base:request.generation,implementation:request.implementation,wanted:request.dirty,slots:slotCountV1({memoryMegabytes:Math.floor(totalmem()/1048576),environment:process.env}),signal:invocation.control.signal,say,
        served:async()=>{const bytes=await files.read(join(directory,GENERATION_POLICY_V1.pointer)) as Uint8Array|undefined,pointer=bytes?readGenerationPointerV1(bytes):undefined;return pointer&&pointer.generation!==request.generation?pointer.generation:undefined;},
        progress:async({done,total})=>{if(!begun){begun=true;await clock.begin("prefetch",total);}await clock.advance("prefetch",done);},
        run:async owners=>{
          checkScriptInvocation(invocation);
          const roots=new Set(owners.map(name=>request.roots[name]).filter((value):value is string=>value!==undefined));
          try{if(roots.size)await prefetchBuildGraphProjectsV1(root,registry,invocation,control,input.files(),roots);}finally{await input.release();}
        }});
    }finally{if(begun)await clock.end("prefetch").catch(()=>undefined);}
  },{optionalCaches:false,shared:true});
}

'''
assert s.count(anchor) == 1
s = s.replace(anchor, new + anchor)
open(G, "w", encoding="utf8").write(s)
print("patched")
