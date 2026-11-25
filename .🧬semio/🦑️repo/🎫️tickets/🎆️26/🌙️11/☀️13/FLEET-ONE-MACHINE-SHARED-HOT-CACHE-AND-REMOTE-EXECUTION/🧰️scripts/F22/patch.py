import sys
D='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts'
t=open(D,encoding='utf8').read()
def once(old,new):
    global t
    assert t.count(old)==1,(old[:60],t.count(old))
    t=t.replace(old,new)
once('import {noticeLanguagesV1} from','import {awaitPartitionSlotV1,partitionFailureTextV1,partitionOperationSlotsV1,partitionRefusalV1} from "./🚦️admission/🟦️.ts";\nimport {noticeLanguagesV1} from')
once('pending:Set<Promise<unknown>>;outcomes:Set<PartitionOutcome>;closing?:Promise<void>};','pending:Set<Promise<unknown>>;outcomes:Set<PartitionOutcome>;roots:number;waiters:Set<()=>void>;closing?:Promise<void>};')
once('sequence:0,pending:new Set(),outcomes:new Set()};const partition=','sequence:0,pending:new Set(),outcomes:new Set(),roots:0,waiters:new Set()};const partition=')
a=t.index('export function withRepositoryPartitionOperationV1<T>(')
b=t.index('let phase=control;',a)
head=('export function withRepositoryPartitionOperationV1<T>(partition:NativeImageStorageV1,control:ResourceLeaseControl,run:(image:NativeImageStorageV1,actualControl:ResourceLeaseControl)=>Promise<T>):Promise<T>{'
'const admit=()=>{const scope=partitionOwners.get(partition),id=scope?.id??"unknown";if(!scope)throw partitionRefusalV1("notIssued",{id});if(!scope.live)throw partitionRefusalV1("closed",{id});if(scope.failed){const first=orderedPartitionFailures(scope.outcomes)[0];throw partitionRefusalV1("failed",{id,cause:partitionFailureTextV1(first)},first);}requireIssuedResourceControlRefinementV1(scope.control,control);const current=partitionFrames.getStore();let nested=false;for(let frame=current;frame;frame=frame.parent)if(frame.scope===scope&&frame.live&&frame.accepting)nested=true;if(!scope.accepting&&!nested||current&&(!current.live||!current.accepting))throw partitionRefusalV1("closing",{id});return{scope,current,nested};};'
'const load=(admitted:ReturnType<typeof admit>)=>Math.max(admitted.nested?0:admitted.scope.roots,admitted.current?.pending.size??0);'
'const submit=(admitted:ReturnType<typeof admit>):Promise<T>=>{const{scope,current,nested}=admitted;')
tail=t[b:]
# tail begins at 'let phase=control;' up to end of function
end=tail.index('return work;}')+len('return work;}')
body,rest=tail[:end],tail[end:]
old1='scope.pending.add(work);scope.outcomes.add(outcome);'
assert body.count(old1)==1
body=body.replace(old1,'scope.pending.add(work);if(!nested)scope.roots++;scope.outcomes.add(outcome);')
old2='void work.then(()=>{scope.pending.delete(work);'
assert body.count(old2)==1
body=body.replace(old2,'const settled=()=>{if(!nested)scope.roots--;for(const wake of[...scope.waiters])wake();};void work.then(()=>{settled();scope.pending.delete(work);')
old3='error=>{outcome.failed=true;outcome.error=error;scope.failed=true;scope.accepting=false;scope.pending.delete(work);'
assert body.count(old3)==1
body=body.replace(old3,'error=>{outcome.failed=true;outcome.error=error;scope.failed=true;scope.accepting=false;settled();scope.pending.delete(work);')
foot=('};const attempt=():Promise<T>=>{const admitted=admit();if(load(admitted)<partitionOperationSlotsV1)return submit(admitted);'
'const{scope}=admitted;return awaitPartitionSlotV1({id:scope.id,busy:()=>load(admit()),recheck:()=>{admit();},onSettled:wake=>{scope.waiters.add(wake);return()=>{scope.waiters.delete(wake);};}},control).then(attempt);};return attempt();}')
t=t[:a]+head+body+foot+rest
open(D,'w',encoding='utf8').write(t)
print("patched")
