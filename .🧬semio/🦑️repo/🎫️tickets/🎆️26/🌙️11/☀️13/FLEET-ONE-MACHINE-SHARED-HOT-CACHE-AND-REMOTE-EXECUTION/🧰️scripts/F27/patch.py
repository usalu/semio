import sys
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/"
def edit(path,pairs):
    t=open(path,encoding="utf8").read()
    for old,new,count in pairs:
        if t.count(old)!=count: sys.exit("anchor mismatch (%d != %d): %s"%(t.count(old),count,old[:80]))
        t=t.replace(old,new)
    open(path,"w",encoding="utf8").write(t)
edit(B+"💾️owner-checkpoint/🟦️.ts",[
 (" check():void;\n yieldContinuation"," check():void;\n refused?(rows:readonly unknown[],storedAt:number):boolean;\n yieldContinuation",1),
 ("return read(location.batch.path,location.batch.stamp,async file=>{","return read(location.batch.path,location.batch.stamp,async(file,stamp)=>{",1),
 ("return rows.length?{facts:record.facts,rows}:undefined;","return rows.length&&!port.refused?.(rows,stamp.mtimeMs)?{facts:record.facts,rows}:undefined;",1),
])
edit(B+"🟦️.ts",[
 ('import {openOwnerCheckpointV1} from "./💾️owner-checkpoint/🟦️.ts";','import {openOwnerCheckpointV1} from "./💾️owner-checkpoint/🟦️.ts";\nimport {forgetPersistedPathsV1,readForgottenPathsV1,rowsForgottenV1} from "./🪦️forgotten-paths/🟦️.ts";',1),
 ('  const memoPath=scope?join(dirname(path),scope+".admission.json"):"";\n','  const memoPath=scope?join(dirname(path),scope+".admission.json"):"",forgottenFile=scope?join(dirname(path),scope+".forgotten.json"):"";\n',1),
 ("{check,yieldContinuation:()=>phaseControl.yieldContinuation(),publish:","{check,refused:(rows,storedAt)=>rowsForgottenV1(readForgottenPathsV1(forgottenFile),rows,storedAt),yieldContinuation:()=>phaseControl.yieldContinuation(),publish:",1),
 ("    let lent=selected&&scope&&!followUp?await readAdmissionMemoV1(","    const forgotten=selected&&scope&&!followUp?readForgottenPathsV1(forgottenFile):undefined,memoStoredAt=forgotten!==undefined&&forgotten.paths.size>0?await lstat(memoPath).then(status=>status.mtimeMs,()=>0):0;\n    let lent=selected&&scope&&!followUp?await readAdmissionMemoV1(",1),
 ("try{for await(const owner of ownersOfAdmissionMemoV1(lent,known))selectedAdmissionFactsV1.seed(scope,owner.suffix,owner.facts,owner.rows);}","try{for await(const owner of ownersOfAdmissionMemoV1(lent,known)){if(forgotten!==undefined&&rowsForgottenV1(forgotten,owner.rows,memoStoredAt))continue;selectedAdmissionFactsV1.seed(scope,owner.suffix,owner.facts,owner.rows);}}",1),
 ("const remembered=lent!==undefined&&await buildGraphSourceFactsHoldV1(","const remembered=lent!==undefined&&!(forgotten!==undefined&&rowsForgottenV1(forgotten,lent.observations,memoStoredAt))&&await buildGraphSourceFactsHoldV1(",1),
 ("if(scope)evictOwnerFactsReferencingV1(scope,moved);","if(scope){evictOwnerFactsReferencingV1(scope,moved);await forgetPersistedPathsV1(forgottenFile,moved);}",3),
])
print("patched")
