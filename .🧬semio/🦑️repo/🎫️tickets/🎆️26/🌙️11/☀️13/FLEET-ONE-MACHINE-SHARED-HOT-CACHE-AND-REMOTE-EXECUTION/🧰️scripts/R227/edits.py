import json,sys,hashlib,os
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
G=B+"/🕸️graph"
EDITS={
 G+"/🕰️generations/🟨️.mjs":[
  ('workspaceRoot,say,stepped}){\n if(typeof root!=="string"','workspaceRoot,say,stepped,afterSeal}){\n if(typeof root!=="string"'),
  ('  return{current:{pointer:outcome.pointer,manifest:done.next},built:true,pub','  if(!rest.length&&afterSeal)await profilePhaseV1("graph.build.affected",()=>Promise.resolve(afterSeal({manifest:done.next,digest:outcome.pointer.manifest,readFragment,readGlobal,signal})).catch(()=>undefined));\n  return{current:{pointer:outcome.pointer,manifest:done.next},built:true,pub')],
 G+"/🕰️generations/🟨️.d.mts":[
  ('stepped?:(step:"storing"|"indexing"|"sealing")=>void|Promise<void>;','stepped?:(step:"storing"|"indexing"|"sealing")=>void|Promise<void>;afterSeal?:(sealed:{manifest:GenerationManifestV1;digest:string;readFragment(row:GenerationOwnerRowV1):Promise<GenerationFragmentV1|undefined>;readGlobal(manifest:GenerationManifestV1):Promise<{readonly edges?:readonly {source:string;target:string}[]}>;signal?:AbortSignal})=>unknown;')],
 G+"/🕰️host/🟦️.ts":[
  ('import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from','import {affectedSealHookV1} from "../🎯️affected-index/🟦️.ts";\nimport {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from'),
  ('fs:generationFilesV1(store),lease:lease(control),','fs:generationFilesV1(store),afterSeal:affectedSealHookV1({workspace:root,root:join(store.root,"generations"),fs:generationFilesV1(store),declared:graphDeclarationV1("../🔣️policy.json").namedInputs as never}),lease:lease(control),')],
 B+"/⌨️command/⚙️runtime/🟦️.ts":[
  ('    await runTaskGraphV1(root,args,parent,phaseControl,{capabilities,selected,resolved,original,ancestor,invocation,previous});','    let routed=resolved;\n    if(command==="affected"){const{answerAffectedV1}=await import("../../🕸️graph/🎯️affected-index/🟦️.ts"),fast=await profilePhaseV1("affected.index",()=>answerAffectedV1({root,invocation,phaseControl,args:resolved.args}));if(fast?.kind==="none"){console.log("[semio] no affected tasks");return;}if(fast)routed={...resolved,args:fast.args};}\n    await runTaskGraphV1(root,args,parent,phaseControl,{capabilities,selected,resolved:routed,original,ancestor,invocation,previous});')]}
if __name__=="__main__":
 root,out=sys.argv[1],sys.argv[2]
 for path,reps in EDITS.items():
  s=open(os.path.join(root,path)).read()
  for old,new in reps:
   assert s.count(old)==1,(path,old[:60],s.count(old))
   s=s.replace(old,new)
  dest=os.path.join(out,path);os.makedirs(os.path.dirname(dest),exist_ok=True);open(dest,"w").write(s)
  print(path.split("/")[-1],hashlib.sha256(open(os.path.join(root,path),'rb').read()).hexdigest()[:12])
