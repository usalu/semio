#!/usr/bin/env python3
"""H61 staged patch: authorizer bundle edits retry in-process (build layer) and name the moved file.
usage: patch.py <tools-dir> [--write]   (dry run prints what changes; --write replaces atomically, caller holds landing.lock.bootstrap)"""
import os,sys,tempfile
tools=sys.argv[1];write="--write" in sys.argv
auth=os.path.join(tools,"🔏️authorizers","🟦️.ts");main=os.path.join(tools,"🟦️.ts")
def once(text,old,new,label):
    n=text.count(old)
    if n!=1: sys.exit(f"anchor {label}: expected 1, found {n}")
    return text.replace(old,new)
a=open(auth,encoding="utf8").read();m=open(main,encoding="utf8").read()
a=once(a,'''/** ✅️ Re-enumerates the actual runtime closure before publishing compiled authorizers. */
export async function nxAuthorizerSourcesHoldV1(root:string,original:NxAuthorizerSourcesV1,signal:AbortSignal):Promise<boolean>{
 try{return (await captureNxAuthorizerSourcesV1(root,original.manifest.entry,signal,{installed:capturedInstalled.has(original),roots:capturedInstalled.get(original)?.roots})).digest===original.digest;}catch(error){signal.throwIfAborted();return false;}
}''','''/** 🔎️ Re-enumerates the actual runtime closure and names the first source whose bytes or membership moved since `original` was captured (the entry when only a failed walk shows it); `undefined` when the closure still holds. */
export async function nxAuthorizerSourcesMovedV1(root:string,original:NxAuthorizerSourcesV1,signal:AbortSignal):Promise<string|undefined>{
 try{
  const live=await captureNxAuthorizerSourcesV1(root,original.manifest.entry,signal,{installed:capturedInstalled.has(original),roots:capturedInstalled.get(original)?.roots});
  if(live.digest===original.digest)return undefined;
  const before=new Map(original.manifest.sources.map(row=>[row.path,row.sha256])),after=new Map(live.manifest.sources.map(row=>[row.path,row.sha256]));
  for(const[path,sha256]of after)if(before.get(path)!==sha256)return path;
  for(const path of before.keys())if(!after.has(path))return path;
  return original.manifest.entry;
 }catch(error){signal.throwIfAborted();return (error as {path?:string})?.path??original.manifest.entry;}
}
/** ✅️ Re-enumerates the actual runtime closure before publishing compiled authorizers. */
export async function nxAuthorizerSourcesHoldV1(root:string,original:NxAuthorizerSourcesV1,signal:AbortSignal):Promise<boolean>{return await nxAuthorizerSourcesMovedV1(root,original,signal)===undefined;}''',"hold")
m=once(m,'import {captureNxAuthorizerSourcesV1,nxAuthorizerSourcesHoldV1} from "./🔏️authorizers/🟦️.ts";','import {captureNxAuthorizerSourcesV1,nxAuthorizerSourcesMovedV1} from "./🔏️authorizers/🟦️.ts";',"import-auth")
m=once(m,'import {concurrentEditV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";','import {concurrentEditV1,retryOnConcurrentEditV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";',"import-churn")
old_capture='''  const authorizerFiles=new Map<string,Buffer>(),authorizer=await captureNxAuthorizerSourcesV1(root,RECIPE+"/🔏️authorizers/📦️runtime/🟦️.ts",signal);
  authorizerFiles.set(".semio-authorizer-source.json",Buffer.from(JSON.stringify(authorizer.manifest)+"\\n"));
  const compiler=await captureNxAuthorizerSourcesV1(root,RECIPE+"/🔏️authorizers/📜️script.ts",signal);
  authorizerFiles.set(".semio-authorizer-compiler.json",Buffer.from(JSON.stringify({version:1,mode:"release",bun:manifest.packageManager,physicalSourceRoot:root,entryRole:"program",runtimeEntryRole:"module",resolutions:compiler.manifest.resolutions,sources:compiler.manifest.sources})+"\\n"));
  for(const name of nxAuthorizerDigestManifestPolicyV1.names)if(authorizerFiles.get(name)!.byteLength>nxAuthorizerDigestManifestPolicyV1.maximumBytes)throw Error("Bounded SDK authorizer digest manifest required: "+name);
'''
m=once(m,old_capture,'  const{authorizerFiles,authorizer,compiler}=await captureNxAuthorizerPairV1(root,manifest.packageManager,signal);\n',"capture")
helper='''/** 📥️ Captures the runtime and the compiler source closure of the Nx authorizers and the digest manifests that name them: one consistent pair, taken again whenever a peer edit moved a source while a bundle was built. */
export async function captureNxAuthorizerPairV1(root:string,packageManager:string,signal:AbortSignal){
  const authorizerFiles=new Map<string,Buffer>(),authorizer=await captureNxAuthorizerSourcesV1(root,RECIPE+"/🔏️authorizers/📦️runtime/🟦️.ts",signal);
  authorizerFiles.set(".semio-authorizer-source.json",Buffer.from(JSON.stringify(authorizer.manifest)+"\\n"));
  const compiler=await captureNxAuthorizerSourcesV1(root,RECIPE+"/🔏️authorizers/📜️script.ts",signal);
  authorizerFiles.set(".semio-authorizer-compiler.json",Buffer.from(JSON.stringify({version:1,mode:"release",bun:packageManager,physicalSourceRoot:root,entryRole:"program",runtimeEntryRole:"module",resolutions:compiler.manifest.resolutions,sources:compiler.manifest.sources})+"\\n"));
  for(const name of nxAuthorizerDigestManifestPolicyV1.names)if(authorizerFiles.get(name)!.byteLength>nxAuthorizerDigestManifestPolicyV1.maximumBytes)throw Error("Bounded SDK authorizer digest manifest required: "+name);
  return{authorizerFiles,authorizer,compiler};
}
'''
m=once(m,'export async function captureNxToolingInputsV1(workspace:string,signal:AbortSignal){',helper+'export async function captureNxToolingInputsV1(workspace:string,signal:AbortSignal){',"helper")
m=once(m,'const bundle=await ensureNxAuthorizerBundleV1(root,store,publisher,control,bun,digest,authorizer,compiler,authorizerFiles);','const bundle=await retryOnConcurrentEditV1("build",async attempt=>{const pair=attempt===1?{authorizerFiles,authorizer,compiler}:await captureNxAuthorizerPairV1(root,manifest.packageManager,signal);return ensureNxAuthorizerBundleV1(root,store,publisher,control,bun,digest,pair.authorizer,pair.compiler,pair.authorizerFiles);},{signal});',"ready")
m=once(m,'for(const[captured,name]of sources)if(!await nxAuthorizerSourcesHoldV1(root,captured,signal))return concurrentEditV1("bytes",captured.manifest.entry,','for(const[captured,name]of sources){const moved=await nxAuthorizerSourcesMovedV1(root,captured,signal);if(moved!==undefined)return concurrentEditV1("bytes",moved,',"failure-hold")
m=once(m,'it is not published","build");\n    const known','it is not published","build");}\n    const known',"failure-close")
m=once(m,'for(const[sources,name]of[[authorizer,"authorizer"],[compiler,"compiler"]]as const)if(!await nxAuthorizerSourcesHoldV1(root,sources,signal))throw concurrentEditV1("bytes",sources.manifest.entry,','for(const[sources,name]of[[authorizer,"authorizer"],[compiler,"compiler"]]as const){const moved=await nxAuthorizerSourcesMovedV1(root,sources,signal);if(moved!==undefined)throw concurrentEditV1("bytes",moved,',"ensure-hold")
m=once(m,'it is not published","build");\n    try { renameSync','it is not published","build");}\n    try { renameSync',"ensure-close")
if write:
    for path,text in((auth,a),(main,m)):
        fd,tmp=tempfile.mkstemp(dir=os.path.dirname(path));os.write(fd,text.encode("utf8"));os.close(fd);os.chmod(tmp,0o644);os.replace(tmp,path)
    print("written")
else:
    print("anchors ok (dry run)")
