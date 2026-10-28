# [graphlat-g] writes staged/reuse-once/edits.json + BASE.txt from the CURRENT live files (anchors asserted unique). usage: python3 -I make-reuse-once.py
import hashlib, json, os, sys
ROOT = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "staged", "reuse-once")
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/"
F = {
    "claims": L + "🕰️generations/🧩️claims/🟨️.mjs",
    "claims.d": L + "🕰️generations/🧩️claims/🟨️.d.mts",
    "load": L + "🕰️generations/🔌️load/🟨️.mjs",
    "graph": L + "🟦️.ts",
    "nodes": L + "📥️inference/📦️project-nodes/🟨️.mjs",
}
edits = []


def edit(key, name, old, new):
    text = open(os.path.join(ROOT, F[key]), encoding="utf8").read()
    if text.count(old) != 1:
        sys.exit("anchor %s in %s: %d occurrences" % (name, key, text.count(old)))
    edits.append({"file": F[key], "name": name, "old": old, "new": new})


# ---- claims: idle()
edit("claims", "claims-idle-function",
     " /** 🧹️ Removes every claim document that is not running (a finished plan leaves them behind), the claims of a live command stay. */",
     """ /** 💤️ Whether a command may build at once: its builder custody `builder` is free right now and no live running claim of this plan covers a wanted owner. A prefetch before such a build computes every stale owner for nobody (the build that follows would still pay its own whole-workspace setup), so the caller skips it; any live claim or a taken custody answers false and the prefetch runs. */
 async function idle({base,implementation,wanted,builder,signal}){
  const probe=await lease.tryHold(builder);if(!probe)return false;probe.release();
  const names=new Set(wanted);
  return!(await annotate(await readClaims(signal),signal)).some(claim=>claim.state==="running"&&claim.live&&claim.base===base&&claim.implementation===implementation&&claim.owners.some(name=>names.has(name)));
 }

 /** 🧹️ Removes every claim document that is not running (a finished plan leaves them behind), the claims of a live command stay. */""")
edit("claims", "claims-idle-export",
     "return Object.freeze({prefetch,sweep,list:async signal=>annotate(await readClaims(signal),signal),resource});",
     "return Object.freeze({prefetch,idle,sweep,list:async signal=>annotate(await readClaims(signal),signal),resource});")
edit("claims.d", "claims-idle-type",
     "export interface ClaimRegistryV1{prefetch(options:ClaimPrefetchOptionsV1):Promise<{computed:number;served:boolean}>;",
     "export interface ClaimRegistryV1{prefetch(options:ClaimPrefetchOptionsV1):Promise<{computed:number;served:boolean}>;idle(options:{base:number;implementation:string;wanted:readonly string[];builder:string;signal?:AbortSignal}):Promise<boolean>;")

# ---- load: skip the prefetch when idle, carry the closure entries through it
edit("load", "load-idle-prefetch",
     " if(host.prefetch&&first.pending?.dirty.length)await host.prefetch({generation:first.pending.generation,dirty:first.pending.dirty,roots:first.pending.roots,implementation});",
     """ const request=first.pending?.dirty.length?{generation:first.pending.generation,dirty:first.pending.dirty,roots:first.pending.roots,implementation}:undefined;
 if(host.prefetch&&request&&!await host.builderIdle?.(request)){host.carryClosures?.(true);try{await host.prefetch(request);}finally{host.carryClosures?.(false);}}""")

# ---- graph host
edit("graph", "host-export",
     "function generationHostV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,registry:BuildOwnerRegistryV1,compact=false){",
     "export function generationHostV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,registry:BuildOwnerRegistryV1,compact=false){")
edit("graph", "host-state-carry",
     "state={files:[] as string[],stable:new Set<string>()},leaseDirectory=",
     "state={files:[] as string[],stable:new Set<string>(),carry:false},leaseDirectory=")
edit("graph", "host-release-carry",
     """    release:async()=>{observationV1.releaseInferenceMemoryV1();(await import("../📥️inputs/🟨️.mjs")).releaseInputMemoryV1();(await import("./📥️inference/🧭️script-inputs/🟨️.mjs")).resetClosureEntriesV1();graphStringInternerV1.clear();createMemoryPortsV1().collect();},""",
     """    release:async()=>{observationV1.releaseInferenceMemoryV1();(await import("../📥️inputs/🟨️.mjs")).releaseInputMemoryV1();if(!state.carry){(await import("./📥️inference/🧭️script-inputs/🟨️.mjs")).resetClosureEntriesV1();setupSharesV1.delete(invocation);}graphStringInternerV1.clear();createMemoryPortsV1().collect();},
    carryClosures:(on:boolean)=>{state.carry=on;},
    builderIdle:(request:GenerationPrefetchRequestV1)=>builderIdleV1({root,invocation,phaseControl,request}),""")
edit("graph", "host-idle-and-share",
     "/** 🧵️ The owners of the previous generation that keep their fragment and only get their edges again when the environment moved",
     """/** 🧺️ What one command's prefetch and build share of the whole-workspace setup of the inference (the native owners, the catalogue and the config bytes behind a validated observation, see `emojiProjectJsonNodes`): one per command, handed along in the inference context, dropped by the final release of the command; `computed` and `reused` count the setups. */
const setupSharesV1=new WeakMap<object,{readonly cache:Map<string,unknown>;computed:number;reused:number}>();
function setupShareV1(invocation:object):{readonly cache:Map<string,unknown>;computed:number;reused:number}{
  let share=setupSharesV1.get(invocation);if(!share)setupSharesV1.set(invocation,share={cache:new Map(),computed:0,reused:0});
  return share;
}

/** 💤️ Whether this command may build at once: the builder custody is free and no live peer claim covers a stale owner of its plan, so a prefetch first would compute the owners twice. Any doubt or failure answers false and the prefetch runs as before. */
async function builderIdleV1(input:{root:string;invocation:ScriptInvocation;phaseControl:ResourceLeaseControl;request:GenerationPrefetchRequestV1}):Promise<boolean> {
  const{root,invocation,phaseControl,request}=input;
  try{
    requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);requireCurrentGraphModuleV1();
    return await withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,generationScopeV1,async(store,control)=>{
      const claims=createClaimRegistryV1({root:join(store.root,"generations"),fs:generationFilesV1(store) as never,lease:createQueuedClaimLeasePortV1({acquire:acquireQueuedResourceLease as never,directory:join(root,".cache/b/l/m/data"),control})});
      return claims.idle({base:request.generation,implementation:request.implementation,wanted:request.dirty,builder:buildGraphPublicationWriterResourceV1(store.root,generationScopeV1),signal:invocation.control.signal});
    },{optionalCaches:false,shared:true});
  }catch(error){if(invocation.control.signal.aborted)throw error;return false;}
}

/** 🧵️ The owners of the previous generation that keep their fragment and only get their edges again when the environment moved""")
edit("graph", "prefetch-context-share",
     "const context={workspaceRoot:root,invocation,phaseControl,registry};\n  const captured=await observationV1.captureObservedSourceInputsV1(async()=>{\n    const options={analyzeLockfile:false,",
     "const context={workspaceRoot:root,invocation,phaseControl,registry,setupShare:setupShareV1(invocation)};\n  const captured=await observationV1.captureObservedSourceInputsV1(async()=>{\n    const options={analyzeLockfile:false,")
edit("graph", "part-context-share",
     "context={workspaceRoot:root,invocation,phaseControl,registry},stubs=Object.entries(part.stubs)",
     "context={workspaceRoot:root,invocation,phaseControl,registry,setupShare:setupShareV1(invocation)},stubs=Object.entries(part.stubs)")

# ---- project nodes: the shared setup
edit("nodes", "nodes-import",
     "import { detachedFailureV1, observeInferenceSnapshotV1, observeProjectInferenceV1, observedFs, projectInferenceObservationsV1 } from",
     "import { detachedFailureV1, memoizedObservedAsyncComputationV1, observeInferenceSnapshotV1, observeProjectInferenceV1, observedFs, projectInferenceObservationsV1 } from")
edit("nodes", "nodes-setup-share",
     """  const catalogContributions=await catalogBuildContributionsV1(workspaceRoot,options?.catalogProviders??[]);
  const owners=options?.ownerAdmission?{files:inferenceCandidates(configFiles,workspaceRoot),nativeCache:new Map(),nativeClosures:new Map()}:await discoverNativeBuildOwnersV1(configFiles,workspaceRoot,context),{nativeCache,nativeClosures}=owners;configFiles=owners.files;
""",
     """  const isAuthoredProjectConfigV1 = configFile => configFile.endsWith(PROJECT_BASENAME) && !configFile.includes("\\uFFFD") && !configFile.includes(".🧬semio") && !POLICY.generatedDirectories.some((name) => configFile.split("/").includes(name));
  const sharedSetup=!options?.ownerAdmission&&context.setupShare?.cache instanceof Map?context.setupShare:undefined;
  const setupPass=async()=>{
    const catalog=await catalogBuildContributionsV1(workspaceRoot,options?.catalogProviders??[]);
    const found=options?.ownerAdmission?{files:inferenceCandidates(configFiles,workspaceRoot),nativeCache:new Map(),nativeClosures:new Map()}:await discoverNativeBuildOwnersV1(configFiles,workspaceRoot,context),texts=new Map();
    for(const file of found.files.filter(isAuthoredProjectConfigV1)){const abs=join(workspaceRoot,file);texts.set(file,existsSync(abs)?readFileSync(abs):null);}
    return{catalog,files:found.files,nativeCache:found.nativeCache,nativeClosures:found.nativeClosures,playground:collectPlaygroundCatalog(found.files,workspaceRoot,catalog),texts};
  };
  let setupSteps=0;
  const setup=sharedSetup?await sharedSetupV1(sharedSetup,JSON.stringify([workspaceRoot,options?.catalogProviders??[],createHash("sha256").update(configFiles.join("\\0")).digest("hex")]),setupPass,(force=false)=>yieldInferenceV1(context,"build:cargo-owners",++setupSteps,force)):await setupPass();
  const catalogContributions=setup.catalog,nativeCache=new Map(setup.nativeCache),nativeClosures=new Map(setup.nativeClosures);configFiles=setup.files;
""")
edit("nodes", "nodes-playground",
     "  const playgroundCatalog = collectPlaygroundCatalog(configFiles, workspaceRoot,catalogContributions);\n",
     "  const playgroundCatalog = setup.playground;\n")
edit("nodes", "nodes-config-bytes",
     """  const results = (await Promise.all(configFiles
    .filter((file) => file.endsWith(PROJECT_BASENAME))
    .filter((configFile) => {
      if (configFile.includes("\\uFFFD")) return false;
      if (configFile.includes(".🧬semio") || POLICY.generatedDirectories.some((name) => configFile.split("/").includes(name))) {
        return false;
      }
      return true;
    })
    .map(async (configFile) => {
      const abs = join(workspaceRoot, configFile);
      if (!existsSync(abs)) return null;
      const bytes = readFileSync(abs);
""",
     """  const results = (await Promise.all(configFiles
    .filter(isAuthoredProjectConfigV1)
    .map(async (configFile) => {
      const abs = join(workspaceRoot, configFile), bytes = setup.texts.get(configFile);
      if (!bytes) return null;
""")
edit("nodes", "nodes-shared-setup-helper",
     "/** 🛡️ Infers the project nodes of every authored emoji-named project config.",
     """/** 🧺️ Runs the whole-workspace setup of an inference once per share: a later call of the same command (the build after the prefetch) reuses it while every observation it was made from still holds, replays those observations into its own capture, and counts `computed` and `reused` on the share. */
export async function sharedSetupV1(share, key, compute, checkpoint) {
  const before = share.computed;
  const value = await memoizedObservedAsyncComputationV1(share.cache, key, async () => { share.computed++; return compute(); }, checkpoint);
  if (share.computed === before) share.reused++;
  return value;
}

/** 🛡️ Infers the project nodes of every authored emoji-named project config.""")

# counting 'reused' needs the hit to be visible: computed counts compute runs, reused is derived by the caller (see the law)
spec = {"edits": [{k: v for k, v in e.items()} for e in edits], "newFiles": []}
os.makedirs(OUT, exist_ok=True)
json.dump(spec, open(os.path.join(OUT, "edits.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
base = []
for key, rel in F.items():
    base.append("%s  %s" % (hashlib.sha256(open(os.path.join(ROOT, rel), "rb").read()).hexdigest(), rel))
open(os.path.join(OUT, "BASE.txt"), "w", encoding="utf8").write("base sha256 of every touched file (graphlat-g, reuse-once):\n" + "\n".join(base) + "\n")
print("edits", len(edits))
