def stages(L,B,C,FF):
    s={}
    pin_old="""const hit=await fleetBlobOpenV1(context,pin.sha256,pin.bytes);
 if(hit){
  const stream=Readable.toWeb(createReadStream(hit.path)) as unknown as ReadableStream<Uint8Array>,closing=new TransformStream<Uint8Array,Uint8Array>({flush:async()=>{await hit.dispose();}});
  return new Response(stream.pipeThrough(closing),{status:200,headers:{"content-length":String(hit.bytes),"content-type":"application/octet-stream","x-semio-fleet-source":hit.source}});
 }
 const started=context.now(),response=await context.fetch(pin.url,init);
 if(!response.ok||!response.body||(context.storeRoot===undefined&&context.remote===undefined))return response;
"""
    pin_new="""const found:{blob?:FleetBlobV1}={blob:await fleetBlobOpenV1(context,pin.sha256,pin.bytes)};
 const claim=found.blob?undefined:await claimFleetWorkV1(context,{kind:"blob",identity:pin.sha256,ready:async()=>(found.blob??=await fleetBlobOpenV1(context,pin.sha256,pin.bytes))!==undefined});
 const hit=found.blob;
 if(hit){
  const stream=Readable.toWeb(createReadStream(hit.path)) as unknown as ReadableStream<Uint8Array>,closing=new TransformStream<Uint8Array,Uint8Array>({flush:async()=>{await hit.dispose();}});
  return new Response(stream.pipeThrough(closing),{status:200,headers:{"content-length":String(hit.bytes),"content-type":"application/octet-stream","x-semio-fleet-source":hit.source}});
 }
 const started=context.now();
 let response:Response;
 try{response=await context.fetch(pin.url,init);}catch(error){await claim?.release();throw error;}
 if(!response.ok||!response.body||(context.storeRoot===undefined&&context.remote===undefined)){await claim?.release();return response;}
"""
    flush_old="""   }finally{await rm(staging,{force:true}).catch(()=>{});}
  }
 });
 return new Response(response.body.pipeThrough(transform)"""
    flush_new="""   }finally{await rm(staging,{force:true}).catch(()=>{});await claim?.release();}
  }
 });
 return new Response(response.body.pipeThrough(transform)"""
    imp_old='import schema from "./🧬️schema/🔣️.json" with {type:"json"};\n'
    imp_new=imp_old+'import {claimFleetWorkV1} from "./🎟️work-claim/🟦️.ts";\n'
    doc_old="The caller keeps verifying the pin itself."
    doc_new="The caller keeps verifying the pin itself. On a fleet miss the download is claimed first (`claimFleetWorkV1`, kind `blob`, identity the pin), so of any number of machines asking at once exactly one downloads while the others wait and are served the published blob; a claim that cannot be had means the download runs here as before."
    crates_imp_old='import {fleetBlobOpenV1,fleetBlobPublishFileV1,fleetBlobStorePathV1,hashFleetFileV1,type FleetFetchContextV1} from "../🟦️.ts";\n'
    crates_imp_new=crates_imp_old+'import {claimFleetWorkV1,type WorkClaimV1} from "../🎟️work-claim/🟦️.ts";\n'
    crates_old=""" let pins:readonly CratePinV1[]=[];
 try{const lock=await findCargoLockV1(request.manifest,request.workspace);if(lock)pins=cargoLockRegistryCratesV1(await readFile(lock,"utf8"));if(pins.length)await fleetCratesRestoreV1(context,request.home,pins);}catch{}
 const result=await fetch();
 try{if(pins.length)await fleetCratesPublishV1(context,request.home,pins);}catch{}
 return result;
}"""
    crates_new=""" let pins:readonly CratePinV1[]=[],claim:WorkClaimV1|undefined;
 try{const lock=await findCargoLockV1(request.manifest,request.workspace);if(lock)pins=cargoLockRegistryCratesV1(await readFile(lock,"utf8"));if(pins.length)claim=await claimFleetWorkV1(context,{kind:"crates",identity:pins.map(pin=>pin.sha256).sort().join("\\n"),ready:async()=>(await fleetCratesRestoreV1(context,request.home,pins)).missing===0});}catch{}
 try{
  const result=await fetch();
  try{if(pins.length)await fleetCratesPublishV1(context,request.home,pins);}catch{}
  return result;
 }finally{await claim?.release().catch(()=>{});}
}"""
    crates_doc_old="Every failure of the fleet is swallowed; the fetch itself runs exactly as before."
    crates_doc_new="The restore runs under the fleet claim of the lockfile's crate set (`claimFleetWorkV1`, kind `crates`): of any number of machines with one lockfile exactly one downloads while the others wait and restore what it published. Every failure of the fleet is swallowed; the fetch itself runs exactly as before."
    uv_imp_old='import {setupClassV1,setupHostV1,setupTextV1,type SetupHostV1} from "../🧰️setup/🟦️.ts";\n'
    uv_imp_new=uv_imp_old+'import {claimFleetWorkV1,type WorkClaimV1} from "../🎟️work-claim/🟦️.ts";\n'
    uv_old="""/** 🔌️ The fleet seam of the uv manager"""
    uv_new="""/** 🎟️ Takes the fleet claim of one uv seed (the lock and the host class): `ready` as soon as a seed manifest for them is in the group store, `granted` to exactly one machine that then syncs and publishes the seed before it releases, `alone` without a group store or a reachable claim route. */
export async function fleetUvClaimV1(context:FleetFetchContextV1,lock:string,host:SetupHostV1=setupHostV1(),options:Readonly<{check?:()=>void}>={}):Promise<WorkClaimV1>{
 const directory=directoryOf(context),lockSha256=createHash("sha256").update(lock).digest("hex");
 if(directory===undefined)return Object.freeze({outcome:"alone",key:"",waitedMilliseconds:0,release:async()=>{}});
 const manifest=join(directory,fleetUvManifestNameV1(lockSha256,host));
 return claimFleetWorkV1(context,{kind:"uv",identity:`${lockSha256}\\n${setupClassV1("host",host)?.tag??"unclassified"}`,ready:async()=>existsSync(manifest),...(options.check===undefined?{}:{check:options.check})});
}

/** 🔌️ The fleet seam of the uv manager"""
    uv_port_old=""" const tag=setupClassV1("host",host)?.tag??"unclassified";
 return {
  before:async cache=>{const cacheDirectory=cache.UV_CACHE_DIR;if(cacheDirectory===undefined)return;const count=await fleetUvRestoreV1(context,{cacheDirectory,lock,host}).catch(()=>0);say(count>0?setupTextV1("uvRestored",{count},environment):setupTextV1("uvMiss",{class:tag},environment));},
  after:async cache=>{const cacheDirectory=cache.UV_CACHE_DIR;if(cacheDirectory===undefined)return;const count=await fleetUvPublishV1(context,{cacheDirectory,lock,host}).catch(()=>undefined);if(count!==undefined&&count>0)say(setupTextV1("uvPublished",{count,class:tag},environment));},
 };"""
    uv_port_new=""" const tag=setupClassV1("host",host)?.tag??"unclassified";
 let claim:WorkClaimV1|undefined;
 return {
  before:async cache=>{const cacheDirectory=cache.UV_CACHE_DIR;if(cacheDirectory===undefined)return;claim=await fleetUvClaimV1(context,lock,host).catch(()=>undefined);const count=await fleetUvRestoreV1(context,{cacheDirectory,lock,host}).catch(()=>0);say(count>0?setupTextV1("uvRestored",{count},environment):setupTextV1("uvMiss",{class:tag},environment));},
  after:async cache=>{const cacheDirectory=cache.UV_CACHE_DIR;try{if(cacheDirectory===undefined)return;const count=await fleetUvPublishV1(context,{cacheDirectory,lock,host}).catch(()=>undefined);if(count!==undefined&&count>0)say(setupTextV1("uvPublished",{count,class:tag},environment));}finally{const held=claim;claim=undefined;await held?.release().catch(()=>{});}},
 };"""
    s["claims"]={"title":"fleet work claim for pinned downloads, crate files and uv seeds (one machine fetches, the others are served)",
     "new":[FF+"🎟️work-claim/🟦️.ts",FF+"🎟️work-claim/🧬️schema/🔣️.json",FF+"🎟️work-claim/🧪️tests/🟦️.ts",FF+"🎟️work-claim/🧪️tests/🔗️wiring/🟦️.ts"],
     "edits":{FF+"🟦️.ts":[(imp_old,imp_new),(pin_old,pin_new),(flush_old,flush_new),(doc_old,doc_new)],
              FF+"🦀️crates/🟦️.ts":[(crates_imp_old,crates_imp_new),(crates_old,crates_new),(crates_doc_old,crates_doc_new)],
              FF+"🐍️uv/🟦️.ts":[(uv_imp_old,uv_imp_new),(uv_old,uv_new),(uv_port_old,uv_port_new)]},
     "tests":[FF+"🎟️work-claim/🧪️tests",FF+"🎟️work-claim/🧪️tests/🔗️wiring",FF+"🧪️tests",FF+"🦀️crates/🧪️tests",FF+"🐍️uv/🧪️tests",FF+"📦️install/🧪️tests"]}

    s["nxplatform"]={"title":"Nx task hashes of platform-independent kinds no longer carry the operating system and architecture",
     "new":[B+"🧮️task-model/🧭️platform-class/🟦️.ts",B+"🧮️task-model/🧭️platform-class/🧬️schema/🔣️.json",B+"🧮️task-model/🧭️platform-class/🧪️tests/🟦️.ts"],
     "edits":{L+"🟨️.mjs":[('import {remoteGuardInputsV1} from "./🏗️build/🧮️task-model/🛡️remote-guard/🟦️.ts";\n','import {remoteGuardInputsV1} from "./🏗️build/🧮️task-model/🛡️remote-guard/🟦️.ts";\nimport {splitNxPlatformInputsV1} from "./🏗️build/🧮️task-model/🧭️platform-class/🟦️.ts";\n'),
        ("return {projects:shareNxProjectInputsV1(liftNxNamedInputReferencesV1(projects)),externalNodes:graph.externalNodes??{}};","return {projects:shareNxProjectInputsV1(splitNxPlatformInputsV1(liftNxNamedInputReferencesV1(projects))),externalNodes:graph.externalNodes??{}};")]},
     "tests":[B+"🧮️task-model/🧭️platform-class/🧪️tests",B+"🧮️task-model/🛡️remote-guard/🧪️tests",B+"🧮️task-model/⚖️cache-law/🧪️tests",B+"🧮️task-model/🪢️task-slot/🧪️tests",C+"🧪️tests/🎚️build-mode",C+"🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧪️tests",B+"🧪️tests/🕸️graph/🔌️nx-engine",B+"🧮️task-model/🧭️platform-class/🧪️tests/🧰️nx-e2e"]}

    old='   counter.hits++;const headers={"content-type":"application/octet-stream","content-length":String(entry.size)};\n'
    new='   if(method==="GET"&&address.kind==="nx")options.provenance?.consume(address.namespace,"nx",address.key,guard.access===undefined?"anonymous":guard.access.role==="static"?"static":guard.access.origin);\n'+old
    s["provenance"]={"title":"Nx task results and install states read from the cache count as one consumption per producer>consumer origin pair, in the record R55 keeps for Rust units",
     "new":[B+"🌐️remote-cache/🧾️provenance/🧪️tests/🔀️nx/🟦️.ts"],
     "edits":{B+"🌐️remote-cache/🖥️server/🟦️.ts":[(old,new)]},
     "tests":[B+"🌐️remote-cache/🧾️provenance/🧪️tests/🔀️nx",B+"🌐️remote-cache/🧾️provenance/🧪️tests",B+"🌐️remote-cache/🖥️server/🧪️tests",B+"🌐️remote-cache/🧪️tests",B+"🌐️remote-cache/🧶️nx/🚦️single-flight/🧪️tests"]}
    return s
