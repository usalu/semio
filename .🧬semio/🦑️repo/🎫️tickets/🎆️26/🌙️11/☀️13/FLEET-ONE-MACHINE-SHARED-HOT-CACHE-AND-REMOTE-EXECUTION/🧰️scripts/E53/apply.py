import sys
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/"
def edit(path,old,new):
    s=open(B+path,encoding='utf8').read()
    if new in s: print("already",path); return
    assert s.count(old)==1,(path,s.count(old))
    open(B+path,'w',encoding='utf8').write(s.replace(old,new))
    print("edited",path)
edit("🌲️process-tree/🧬️schema/🔣️.json",'"staleStageMilliseconds": 1800000,','"staleStageMilliseconds": 1800000,\n        "codeQuietMilliseconds": 300000,')
doc='''/** 🪢️ What a changed code declaration does to a running stage, decided in one place: `"holder"` when the stage has waited `holderWaitRestartMilliseconds` for a named holder (it makes no progress by definition), `"stale"` when it only reads or plans (never in the `executing` phase), has been in that phase for the stale age AND has shown no progress for `codeQuietMilliseconds`, `null` otherwise: a stage that is making progress is never stopped for new code, the new code applies to its next attempt, so a landing never costs an hour of reading. A pure function.
 * @see ./🧪️tests/🟦️.ts */
export function readyCodeStopV1(input:Readonly<{holderWaitMs:number;phase:ReadyPhaseV1;phaseAgeMs:number;quietMs:number;policy?:typeof P}>):"holder"|"stale"|null{
 const policy=input.policy??P;
 if(readyHolderRestartV1(input.holderWaitMs,true,policy))return"holder";
 return readyStaleStopV1(input.phase,input.phaseAgeMs,policy)&&input.quietMs>=policy.codeQuietMilliseconds?"stale":null;
}

'''
anchor='/** 🧬️ The declared semantics of the code a stage loaded:'
edit("🌲️process-tree/🟦️.ts",anchor,doc+anchor)
edit("🟦️.ts",'if(!killed&&(waiting>=treePolicy.holderWaitRestartMilliseconds||readyStaleStopV1(phase,Date.now()-phaseAt,treePolicy))&&Date.now()-codeCheckedAt>=treePolicy.codeCheckMilliseconds){codeCheckedAt=Date.now();const holder=readyHolderRestartV1(waiting,true,treePolicy),file=readySemanticsChangedV1(input.root,semanticsAtStart,treePolicy,sha);if(file!==null&&(holder||readyStaleStopV1(phase,Date.now()-phaseAt,treePolicy))){codeFile=file;codeStale=!holder;stalled=true;kill("restart");}}',
 'const codeVerdict=():ReturnType<typeof readyCodeStopV1>=>readyCodeStopV1({holderWaitMs:waiting,phase,phaseAgeMs:Date.now()-phaseAt,quietMs:Date.now()-progressAt,policy:treePolicy});if(!killed&&codeVerdict()!==null&&Date.now()-codeCheckedAt>=treePolicy.codeCheckMilliseconds){codeCheckedAt=Date.now();const why=codeVerdict(),file=readySemanticsChangedV1(input.root,semanticsAtStart,treePolicy,sha);if(file!==null&&why!==null){codeFile=file;codeStale=why==="stale";stalled=true;kill("restart");}}')
edit("🟦️.ts",'import{readyDeadlineV1,readyForgetTreeV1,','import{readyCodeStopV1,readyDeadlineV1,readyForgetTreeV1,')
