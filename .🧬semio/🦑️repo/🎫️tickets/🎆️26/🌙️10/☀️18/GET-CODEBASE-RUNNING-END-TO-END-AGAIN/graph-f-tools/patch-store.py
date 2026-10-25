import os,sys,tempfile
path=sys.argv[1]
src=open(path,encoding="utf-8").read()
def sub(old,new,count=1):
    global src
    n=src.count(old)
    if n!=count: sys.exit(f"anchor count {n}!={count}: {old[:80]}")
    src=src.replace(old,new)
sub('import{canonicalJsonV1,compareCodePointsV1,decodeFactIndexV1,encodeFactIndexV1}from"./🧮️fact-codec/🟨️.mjs";',
 'import{canonicalJsonV1,compareCodePointsV1,decodeFactIndexV1,encodeFactIndexV1}from"./🧮️fact-codec/🟨️.mjs";\nimport{coverageOfV1,mergeVerifiedStampV1,readVerifiedStampV1,uncoveredOfV1}from"./🔭️coverage/🟨️.mjs";')
sub('compactionFactor:2,compactionFloor:1024,brotliQuality:5,brotliWindow:22, reasonRows:16,',
 'compactionFactor:2,compactionFloor:1024,brotliQuality:5,brotliWindow:22, reasonRows:16,\n stampVerification:false,waitNoticeMilliseconds:20000,')
sub('requireKeys(value,["version","generation","manifest","parent"],["reason"],"generation pointer");','requireKeys(value,["version","generation","manifest","parent"],["reason","verified"],"generation pointer");')
sub(' if(value.reason!==undefined)requireText(value.reason,"reason",256);\n return Object.freeze(value);',' if(value.reason!==undefined)requireText(value.reason,"reason",256);\n if(value.verified!==undefined)readVerifiedStampV1(value.verified);\n return Object.freeze(value);')
sub(' async function plan({current,identity,changes,only,signal,progress}){',' async function plan({current,identity,changes,only,skipEnvironment=false,signal,progress}){')
sub('const selection=selectFactClassesV1(index,only?new Set([...only,P.envOwner]):undefined);','const selection=selectFactClassesV1(index,only?new Set(skipEnvironment?only:[...only,P.envOwner]):undefined);')
sub(' async function publish({manifestDigest,expectedGeneration,reason,signal}){',' async function publish({manifestDigest,expectedGeneration,reason,verified,signal}){')
sub('parent:pointer?.manifest??null,...(reason?{reason}:{})};','parent:pointer?.manifest??null,...(reason?{reason}:{}),...(verified?{verified}:{})};')
sub(' async function build({identity,implementation,membership,compute,signal,progress,hint,only}){',' async function build({identity,implementation,membership,compute,signal,progress,hint,only,observedAtMs=now(),coverage}){')
sub('const found=reuse?hint.plan:await plan({current,identity,changes:previous?.frontier?live.changes:undefined,signal,progress,only});','const found=reuse?hint.plan:await plan({current,identity,changes:previous?.frontier?live.changes:undefined,only,skipEnvironment:coverage!==undefined,signal,progress});')
sub('   if(usable.membership===live.digest)return{current,built:false,published:false,plan:found,computed:0};\n   return{...await refreshMetadata({current,live,signal}),built:false,plan:found,computed:0};',
    '   if(usable.membership===live.digest)return{current,built:false,published:false,plan:found,computed:0};\n   return{...await refreshMetadata({current,live,signal}),built:false,plan:found,computed:0};')
sub('  const outcome=await publish({manifestDigest,expectedGeneration,signal});\n  if(!outcome.published)return{current:await read(signal),built:true,published:false,plan:found,computed:result.owners.length};',
    '  const verified=P.stampVerification?mergeVerifiedStampV1({owners:Object.keys(owners).sort(compareCodePointsV1),scope:cold?undefined:only,observedAtMs,carried:cold?undefined:coverage}):undefined;\n  const outcome=await publish({manifestDigest,expectedGeneration,verified,signal});\n  if(!outcome.published)return{current:await read(signal),built:true,published:false,plan:found,computed:result.owners.length};')
sub('try{const outcome=await publish({manifestDigest:digest,expectedGeneration:current.pointer.generation,signal});','try{const outcome=await publish({manifestDigest:digest,expectedGeneration:current.pointer.generation,verified:current.pointer.verified,signal});')
old_refresh=src[src.index(' async function refresh({identity,implementation,policy="fresh",only,membership,compute,signal,progress,builder}){'):src.index(' /** 🧭️ Collects the closure of `roots`')]
new_refresh=''' async function refresh({identity,implementation,policy="fresh",only,membership,compute,signal,progress,builder,requestedAt}){
  if(!P.policies.includes(policy))throw fail("Closed graph policy required");
  const started=now(),first=await read(signal),usable=first?.manifest&&first.manifest.identity===identity;
  if(policy==="current"&&usable)return{current:first,served:"current",built:false,published:false,computed:0};
  let hint,scope=only,coverage;
  if(usable&&requestedAt!==undefined&&first.pointer.verified!==undefined){
   const owners=Object.keys(first.manifest.owners).sort(compareCodePointsV1);
   coverage=coverageOfV1({stamp:first.pointer.verified,owners,implementation,manifestImplementation:first.manifest.implementation,requestedAtMs:requestedAt});
   if(coverage){const open=uncoveredOfV1({needed:only,owners,coverage});if(!open.size)return{current:first,served:"covered",built:false,published:false,computed:0};scope=open;}
  }
  if(usable){
   const previous={manifest:first.manifest,frontier:await readFrontier(first.manifest.frontier)},live=await membership(previous.frontier?previous:undefined,signal);
   const verdict=await plan({current:first,identity,changes:previous.frontier?live.changes:undefined,only:scope,skipEnvironment:coverage!==undefined,signal,progress});
   if(verdict.kind==="clean"){
    if(!scope&&live.digest!==first.manifest.membership){const refreshed=await refreshMetadata({current:first,live,signal});return{...refreshed,served:"verified",built:false,computed:0,plan:verdict};}
    return{current:first,served:"verified",built:false,published:false,computed:0,plan:verdict};
   }
   if(!scope&&verdict.kind==="delta"&&previous.frontier)hint={generation:first.pointer.generation,plan:verdict,live};
  }
  const body=()=>build({identity,implementation,membership,compute,signal,progress,hint,only:scope,observedAtMs:started,coverage});
  for(let attempt=0;attempt<8;attempt++){
   const result=await(builder?builder(body):lease?lease.exclusive(resources.builder,body,{signal}):body());
   if(result.published||!result.built)return{...result,served:"built"};
  }
  throw fail("Generation keeps changing while it is built","GENERATION_CONFLICT");
 }

'''
print(old_refresh[:200]); 
src=src.replace(old_refresh,new_refresh)
sub('async function refreshClosure({roots,neighbors,closure,identity,implementation,policy="fresh",membership,compute,signal,progress,builder}){','async function refreshClosure({roots,neighbors,closure,identity,implementation,policy="fresh",membership,compute,signal,progress,builder,requestedAt}){')
sub('let verified=names,result=await refresh({identity,implementation,policy,only:names,membership,compute,signal,progress,builder});','let verified=names,result=await refresh({identity,implementation,policy,only:names,membership,compute,signal,progress,builder,requestedAt});')
sub('result=await refresh({identity,implementation,policy,only:verified,membership,compute,signal,progress,builder});','result=await refresh({identity,implementation,policy,only:verified,membership,compute,signal,progress,builder,requestedAt});')
d=os.path.dirname(path);fd,tmp=tempfile.mkstemp(dir=d,prefix=".patch-")
os.write(fd,src.encode("utf-8"));os.close(fd);os.rename(tmp,path);print("patched")
