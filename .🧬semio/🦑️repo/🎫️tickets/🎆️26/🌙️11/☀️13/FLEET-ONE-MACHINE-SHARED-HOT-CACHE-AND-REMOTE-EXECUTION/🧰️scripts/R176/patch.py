import sys,re
S=sys.argv[1]
def sub(path,old,new,count=1):
    t=open(path,encoding="utf-8").read()
    assert t.count(old)==count,(path,old[:60],t.count(old))
    open(path,"w",encoding="utf-8").write(t.replace(old,new))
j=S+"/tree.json"
sub(j,'"codeSkipDirectories": ["node_modules", "target", ".cache", "🗑️generated", "📖️texts", "🧪️tests"],','"codeSkipDirectories": ["node_modules", "target", ".cache", "🗑️generated", "📖️texts", "🧪️tests", "🧪️testing"],\n        "readingPatterns": ["Reading the project graph", "Loading the project graph", "Planning every target", "Lese den Projektgraph", "Lade den Projektgraph"],\n        "executingPatterns": ["Running target \\\\S+ for (?:project|\\\\d+ projects)", "^\\\\s*> nx run \\\\S+", "graph: plan answered", "Graph: Plan beantwortet"],')
t=S+"/tree.ts"
sub(t,'''/** 🧬️ The first file below `codeRoots`''','''/** 🧭️ The phase of a stage run as its log shows it: `executing` once a line says the plan is answered or a task runs (`executingPatterns`), `reading` while a line says the project graph is read or planned (`readingPatterns`); the last such line of `lines` decides (an executing line wins on one line), and `previous` stays when no line says either (a stage that never says is `null` for good). A pure function.
 * @see ./🧪️tests/🕰️stale-stage */
export type ReadyPhaseV1="reading"|"executing"|null;
export function readyStagePhaseV1(lines:readonly string[],previous:ReadyPhaseV1,policy:typeof P=P):ReadyPhaseV1{
 const executing=policy.executingPatterns.map(pattern=>new RegExp(pattern,"u")),reading=policy.readingPatterns.map(pattern=>new RegExp(pattern,"u"));
 for(let index=lines.length-1;index>=0;index--){const line=lines[index]!;if(executing.some(expression=>expression.test(line)))return"executing";if(reading.some(expression=>expression.test(line)))return"reading";}
 return previous;
}

/** 🛑️ Whether the stale-code rule may stop a stage now: only while what it computes is not sealed, that is never in the `executing` phase (its tasks are planned, their hashes fix their inputs and what finishes is stored, the next stage run uses the new code), and then only after the stale age (`staleStageMilliseconds`, already scaled by the machine speed factor) in the phase; a stage that never reports a phase keeps the old rule. A pure function.
 * @see ./🧪️tests/🕰️stale-stage */
export function readyStaleStopV1(phase:ReadyPhaseV1,phaseAgeMs:number,policy:typeof P=P):boolean{return phase!=="executing"&&readyStaleRestartV1(phaseAgeMs,true,policy);}

/** 🧬️ The first file below `codeRoots`''')
r=S+"/ready.ts"
sub(r,"readyStaleRestartV1,","readyStaleStopV1,readyStagePhaseV1,type ReadyPhaseV1,")
sub(r,"codeFile:string|undefined,codeStale=false,codeCheckedAt=0,","codeFile:string|undefined,codeStale=false,codeCheckedAt=0,phase:ReadyPhaseV1=null,phaseAt=inflight.startedMs,")
sub(r,"const waiting=holder.observe(recent,Date.now());","const waiting=holder.observe(recent,Date.now()),seenPhase=readyStagePhaseV1(recent,phase,treePolicy);if(seenPhase!==phase){if(phase!==null||seenPhase!==\"reading\")phaseAt=Date.now();phase=seenPhase;}")
sub(r,"readyStaleRestartV1(Date.now()-inflight.startedMs,true,treePolicy))&&Date.now()-codeCheckedAt","readyStaleStopV1(phase,Date.now()-phaseAt,treePolicy))&&Date.now()-codeCheckedAt")
sub(r,"const file=readyCodeChangedSinceV1(input.root,inflight.startedMs,treePolicy),holder=readyHolderRestartV1(waiting,true,treePolicy);if(file!==null&&(holder||readyStaleRestartV1(Date.now()-inflight.startedMs,true,treePolicy)))","const holder=readyHolderRestartV1(waiting,true,treePolicy),file=readyCodeChangedSinceV1(input.root,holder?inflight.startedMs:phaseAt,treePolicy);if(file!==null&&(holder||readyStaleStopV1(phase,Date.now()-phaseAt,treePolicy)))")
