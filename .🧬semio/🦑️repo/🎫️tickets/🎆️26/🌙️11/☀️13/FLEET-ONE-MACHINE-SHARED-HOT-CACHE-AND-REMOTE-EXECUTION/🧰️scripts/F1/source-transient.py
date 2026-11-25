import re,sys
D="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/"
def edit(path,pairs):
    s=open(path,encoding='utf-8').read()
    for a,b in pairs:
        assert s.count(a)==1,(path.split('/')[-2:],a[:60],s.count(a))
        s=s.replace(a,b)
    return s
ts=edit(D+"🟦️.ts",[
('|"store-full"|"disk-short"','|"store-full"|"source-transient"|"disk-short"'),
('nextAttemptMs:number;atMs:number;tail:readonly string[]}>','nextAttemptMs:number;atMs:number;tail:readonly string[];tree?:string}>'),
('status:Readonly<{update(patch:Partial<ReadyStatusV1>):void;heartbeat?():void}>;','tree?():string|null;root?:string;\n status:Readonly<{update(patch:Partial<ReadyStatusV1>):void;heartbeat?():void}>;'),
('repo=ports.repoPresent(),now=ports.now();','repo=ports.repoPresent(),now=ports.now(),stamp=ports.tree?.()??null;\n   if(stamp!==null&&treeSeen?.stamp!==stamp)treeSeen={stamp,changedMs:now};'),
('count:READY_TREE_POLICY_V1.stallRestartMaximum}):null;','count:READY_TREE_POLICY_V1.stallRestartMaximum,seconds:Math.round(P.sourceTransientDebounceMilliseconds/1000)}):null;'),
('const inflightStage=ports.inflight?.();','const startTree=ports.tree?.()??null,inflightStage=ports.inflight?.();'),
(':readyStoreFullV1(failedTail)!==null?"store-full":"stage-failed"',':readyStoreFullV1(failedTail)!==null?"store-full":readySourceTransientV1([...result.failure??[],...failedTail],ports.root??"")?"source-transient":"stage-failed"'),
('tail:failedTail.slice(-P.tailLines)};','tail:failedTail.slice(-P.tailLines),...startTree===null?{}:{tree:startTree}};'),
('let memoryLowSince:number|undefined,memoryWaitSaid=false,refreshStage:string|null=null;','let memoryLowSince:number|undefined,memoryWaitSaid=false,refreshStage:string|null=null,treeSeen:Readonly<{stamp:string;changedMs:number}>|null=null;'),
('failure:state.failure===null?null:{digest:state.failure.digest,nextAttemptMs:state.failure.nextAttemptMs},now});','failure:state.failure===null?null:{digest:state.failure.digest,nextAttemptMs:state.failure.nextAttemptMs,code:state.failure.code,atMs:state.failure.atMs,...state.failure.tree===undefined?{}:{tree:state.failure.tree}},tree:treeSeen,now});'),
('failure:Readonly<{digest:string;nextAttemptMs:number}>|null;now:number}>;','failure:Readonly<{digest:string;nextAttemptMs:number;code?:ReadyCodeV1;atMs?:number;tree?:string}>|null;tree?:Readonly<{stamp:string;changedMs:number}>|null;now:number}>;'),
(' if(input.failure!==null&&input.failure.digest===input.digest&&input.failure.nextAttemptMs>input.now)return act("backoff",index,null,input.failure.nextAttemptMs-input.now);',
 ' if(input.failure!==null&&input.failure.digest===input.digest){const retry=readySourceRetryV1(input.failure,input.tree??null);if(retry>input.now)return act("backoff",index,null,input.failure.code==="source-transient"?Math.min(retry-input.now,P.sourceTransientPollMilliseconds):retry-input.now);}'),
('digests:async()=>measure(plan),\n   state,','digests:async()=>measure(plan),\n   root:input.root,\n   tree:()=>{const store=input.environment.SEMIO_FLEET_STORE_ROOT;if(store===undefined||store==="")return null;try{return readdirSync(join(store,"v1",P.treeStampDirectory)).filter(name=>!name.startsWith(".")).sort().join("|");}catch{return null;}},\n   state,'),
])
anchor='export type ReadyDecisionV1=Readonly<{action:"server"|"idle"|"mark"|"observe"|"wait"|"backoff"|"run";stageIndex:number;code:ReadyCodeV1|null;waitMilliseconds:number}>;\n'
assert ts.count(anchor)==1
new=anchor+'''
/** 🧱️ Whether the evidence lines of a failed stage name a source-level cause in the checkout: a missing or refused source file, import or module (the `sourceTransientPatterns` of the schema, `{root}` standing for the checkout root). A peer's half-finished edit looks like this and heals with the next change of the replicated tree, so it is no evidence about this machine.
 * @see ./🧬️schema/🔣️.json */
export function readySourceTransientV1(lines:readonly string[],root:string):boolean{
 const escaped=root.replace(/[.*+?^${}()|[\\]\\\\]/gu,"\\\\$&"),plain=lines.map(line=>line.replace(/\\u001b\\[[0-9;]*m/gu,""));
 return P.sourceTransientPatterns.some(pattern=>{if(root===""&&pattern.includes("{root}"))return false;const expression=new RegExp(pattern.replaceAll("{root}",escaped),"u");return plain.some(line=>expression.test(line));});
}

/** ⏩️ The time the next attempt of a failed stage starts: a `source-transient` failure retries a debounce after the replicated tree changed since the stage started (the tree stamp differs from the one the failure recorded; a change during the run counts from the end of the run), never later than its back-off; any other failure, or an unchanged tree, keeps the back-off.
 * @see ./🧬️schema/🔣️.json */
export function readySourceRetryV1(failure:Readonly<{code?:ReadyCodeV1;atMs?:number;tree?:string;nextAttemptMs:number}>,tree:Readonly<{stamp:string;changedMs:number}>|null):number{
 if(failure.code!=="source-transient"||failure.tree===undefined||tree===null||tree.stamp===failure.tree)return failure.nextAttemptMs;
 return Math.min(failure.nextAttemptMs,Math.max(failure.atMs??0,tree.changedMs)+P.sourceTransientDebounceMilliseconds);
}
'''
ts=ts.replace(anchor,new)
sc=edit(D+"🧬️schema/🔣️.json",[
('"storeFullPattern": "No space left \\\\((EDQUOT|ENOSPC)\\\\) in (.+?): the disk",','"storeFullPattern": "No space left \\\\((EDQUOT|ENOSPC)\\\\) in (.+?): the disk",\n        "sourceTransientPatterns": ["ENOENT: no such file or directory, \\\\w+ \'{root}[^\']*\'", "Private Nx engine import missing: ", "(?:refused|invalid|missing) process\\\\.source\\\\.", "(?:Cannot find module|Could not resolve)[: ]+[\\"\']?(?:\\\\.|{root})"],\n        "sourceTransientDebounceMilliseconds": 15000,\n        "sourceTransientPollMilliseconds": 5000,\n        "treeStampDirectory": "heads",'),
('"refreshKeepingFailureCodes": ["stage-timeout", "stage-stall-exhausted", "store-full",','"refreshKeepingFailureCodes": ["stage-timeout", "stage-stall-exhausted", "store-full", "source-transient",'),
('          "store-full": "Stage {stage} cannot create files','          "source-transient": "Stage {stage} failed on a source file of the checkout that is missing or refused ({ending}), usually a peer\'s edit in progress; it starts again {seconds} seconds after the next change of the replicated tree, or after the back-off if the tree does not change.",\n          "store-full": "Stage {stage} cannot create files'),
('          "store-full": "Stufe {stage} kann in','          "source-transient": "Stufe {stage} scheiterte an einer Quelldatei des Arbeitsbaums, die fehlt oder abgelehnt wurde ({ending}), meist die Änderung eines Peers in Arbeit; sie startet {seconds} Sekunden nach der nächsten Änderung des replizierten Baums neu, oder nach der Wartezeit, wenn sich der Baum nicht ändert.",\n          "store-full": "Stufe {stage} kann in'),
('"stage-holder-silent", "store-full", "disk-short"','"stage-holder-silent", "store-full", "source-transient", "disk-short"'),
])
if '--write' in sys.argv:
    open(D+"🟦️.ts",'w',encoding='utf-8').write(ts);open(D+"🧬️schema/🔣️.json",'w',encoding='utf-8').write(sc);print('written')
else:print('dry ok')
