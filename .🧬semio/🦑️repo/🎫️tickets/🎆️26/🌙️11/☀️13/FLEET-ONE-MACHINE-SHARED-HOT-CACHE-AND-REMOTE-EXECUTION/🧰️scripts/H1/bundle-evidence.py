import os
p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts"
s=open(p,encoding="utf8").read()
a="/** 🔖️ Provisions the compiled authorizer bundle beside one dependency generation"
assert s.count(a)==1
helper='''/** 🔬️ Names, for every captured module a failed bundle message points at, the digest it was captured with and the digest on disk now, so a stale or divergent capture shows in the log instead of staying a bare bundler error. */
export function authorizerBundleEvidenceV1(message:string,root:string,sources:ReadonlyMap<string,string>,observed:(path:string)=>string|null):string{
  const paths=[...new Set([...message.matchAll(/semio-captured-source:([^"\\s]+)/gu)].map(row=>relative(root,row[1]!).replaceAll("\\\\","/")))].slice(0,4);
  return paths.map(path=>path+" captured sha256 "+(sources.get(path)??"absent")+", on disk "+(observed(path)??"absent")).join("; ");
}
/** 🧯️ A bundler failure over captured sources is a concurrent edit when either capture no longer holds against the tree, otherwise it keeps its message plus the digest evidence of the modules it names. */
async function authorizerBundleFailureV1(root:string,sources:readonly(readonly[Awaited<ReturnType<typeof captureNxAuthorizerSourcesV1>>,string])[],error:unknown,signal:AbortSignal):Promise<unknown>{
  const message=String((error as Error)?.message??error);
  if(!/Bundle failed|compilation refused/u.test(message)||signal.aborted)return error;
  try{
    for(const[captured,name]of sources)if(!await nxAuthorizerSourcesHoldV1(root,captured,signal))return concurrentEditV1("bytes",captured.manifest.entry,"Authorizer sources changed while the "+name+" bundle was built, it is not published","build");
    const known=new Map(sources.flatMap(([captured])=>captured.manifest.sources.map(row=>[row.path,row.sha256] as const)));
    const evidence=authorizerBundleEvidenceV1(message,root,known,path=>{try{return createHash("sha256").update(readFileSync(join(root,path))).digest("hex");}catch{return null;}});
    return evidence===""?error:new Error(message+" [captured against disk: "+evidence+"]",{cause:error});
  }catch{return error;}
}
'''
s=s.replace(a,helper+a)
b="  } catch(error){stagingFailed=true;throw error;}\n  finally {try{rmSync(staging,{recursive:true,force:true});}catch(error){if(!stagingFailed)throw error;try{console.error(\"[nx] Authorizer staging cleanup failed"
assert s.count(b)==1
s=s.replace(b,b.replace("throw error;}\n  finally","throw await authorizerBundleFailureV1(root,[[authorizer,\"authorizer\"],[compiler,\"compiler\"]],error,signal);}\n  finally",1))
open(p+".t","w",encoding="utf8").write(s);os.replace(p+".t",p)
