import sys,hashlib
LOAD_IMPORT_ANCHOR='import {readAdmissionMemoV1,writeAdmissionMemoV1} from "./🧠️admission-memo/🟦️.ts";\n'
LOAD_IMPORT_NEW=LOAD_IMPORT_ANCHOR+'import {loadSelectedThroughFleetV1} from "./🤝️selected-exchange/🔌️attempt/🟦️.ts";\n'
MEMO_ANCHOR='/** 🪄️ Answers the admission facts of one owner from the memo'
MEMO_NEW=r'''/** 🧲️ The admission facts of one scope this process remembers and the way a generation of the fleet seeds them: the key text of a fact names the scope, the implementation of this process and the question, so a fact of another machine is stored under this implementation.
 * @see ./🤝️selected-exchange/🔌️attempt/🟦️.ts */
export const selectedAdmissionFactsV1={
  entries(scope:string):readonly {suffix:string;facts:unknown;rows:readonly unknown[]}[]{const prefix=scope+"\0"+loadedGraphModuleAuthorityV1.implementation+"\0";return [...ownerFactsMemoV1].filter(([key])=>key.startsWith(prefix)).map(([key,entry])=>({suffix:key.slice(prefix.length),facts:entry.facts,rows:entry.rows}));},
  seed(scope:string,suffix:string,facts:unknown,rows:readonly unknown[]):void{rememberOwnerFactsV1(scope+"\0"+loadedGraphModuleAuthorityV1.implementation+"\0"+suffix,{facts,rows});}
};

'''+MEMO_ANCHOR
CALL_OLD='checkResourceLeaseControl(actualControl);await store.recheck();return loadBuildGraphAttemptV1(root,invocation,actualControl,options,registry,scope);'
CALL_NEW=('checkResourceLeaseControl(actualControl);await store.recheck();const attempt=()=>loadBuildGraphAttemptV1(root,invocation,actualControl,options,registry,scope);\n'
 '      return scope===undefined?attempt():selectedThroughFleetV1({root,invocation,scope,staging,attempt});')
HELPER_ANCHOR='/** 🔓️ Serves a current sealed graph to any number of shared readers'
HELPER_NEW=r'''/** 🧲️ The attempt of a selected scope offered through the fleet: a sealed generation of another machine whose owners hold here is installed, this machine's own sealed result is published.
 * @see ./🤝️selected-exchange/🔌️attempt/🟦️.ts */
function selectedThroughFleetV1<G>(input:{root:string;invocation:ScriptInvocation;scope:string;staging:string;attempt:()=>Promise<G|undefined>}):Promise<G|undefined>{
  return loadSelectedThroughFleetV1({root:input.root,scope:input.scope,path:join(input.staging,input.scope+".json"),authorityPath:join(input.staging,input.scope+".authority.json"),attempt:input.attempt,sealed:value=>value!==undefined&&!isOwnerRegistryInvalidationV1(value),holds:rows=>buildGraphSourceFactsHoldV1(rows,input.invocation),facts:selectedAdmissionFactsV1,module:loadedGraphModuleAuthorityV1,verdict:({projects,rows})=>sealVerdictV1({rounds:0,staleCount:0,stale:[],membershipMoved:false,admissionMoved:false,projects:projects as never,rows:rows as never}),signal:input.invocation?.control.signal});
}


'''+HELPER_ANCHOR
EDITS=[(LOAD_IMPORT_ANCHOR,LOAD_IMPORT_NEW),(MEMO_ANCHOR,MEMO_NEW),(HELPER_ANCHOR,HELPER_NEW),(CALL_OLD,CALL_NEW)]
def patch(text):
    for old,new in EDITS:
        if text.count(old)!=1: raise SystemExit("anchor count %d for %r"%(text.count(old),old[:60]))
        text=text.replace(old,new,1)
    return text
if __name__=="__main__":
    src,dst=sys.argv[1],sys.argv[2]
    t=open(src,encoding="utf8").read()
    open(dst,"w",encoding="utf8").write(patch(t))
    print("patched",hashlib.sha256(t.encode()).hexdigest()[:12],"->",dst)
