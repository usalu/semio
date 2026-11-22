import sys,re
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/"
def edit(path,pairs):
    t=open(path,encoding="utf8").read()
    for old,new,mark in pairs:
        if mark in t: continue
        assert t.count(old)==1,(path,old[:60],t.count(old))
        t=t.replace(old,new)
    open(path,"w",encoding="utf8").write(t)
phase=sys.argv[1]
if phase=="schema":
    edit(R+"🧬️schema/🔣️.json",[
     ('"binary-missing": "The repo binary is missing, starting again at stage {stage}."}','"binary-missing": "The repo binary is missing, starting again at stage {stage}.", "refreshing": "Ready, refreshing stage {stage} in the background: its inputs changed; the worker keeps serving and stays ready unless this check fails."}','"refreshing": "Ready, refreshing'),
     ('"binary-missing": "Das repo-Programm fehlt, Neustart bei Stufe {stage}."}','"binary-missing": "Das repo-Programm fehlt, Neustart bei Stufe {stage}.", "refreshing": "Bereit, Stufe {stage} wird im Hintergrund aufgefrischt: ihre Eingaben haben sich geändert; der Worker bedient weiter und bleibt bereit, außer diese Prüfung schlägt fehl."}','"refreshing": "Bereit, Stufe'),
     ('"adopted", "binary-missing"]}','"adopted", "binary-missing", "refreshing"]}','"binary-missing", "refreshing"]}'),
     ('        "readyMs": {"type": ["integer", "null"]},\n','        "readyMs": {"type": ["integer", "null"]},\n        "refreshing": {"type": "boolean"},\n','"refreshing": {"type": "boolean"}'),
    ])
if phase=="code":
    edit(R+"🟦️.ts",[
     ('|"adopted"|"binary-missing";','|"adopted"|"binary-missing"|"refreshing";','"binary-missing"|"refreshing"'),
     ('readyMs:number|null;startedMs:number;updatedMs:number};','readyMs:number|null;refreshing?:boolean;startedMs:number;updatedMs:number};','refreshing?:boolean;startedMs'),
     ('const out={...doc,updatedMs:now(),beatMilliseconds:P.beatByState[doc.state]};','const out={...doc,updatedMs:now(),beatMilliseconds:P.beatByState[doc.refreshing===true?"running":doc.state]};','doc.refreshing===true?"running"'),
     ('heartbeat(){if(!doc.ready&&(doc.state==="running"','heartbeat(){if((doc.refreshing===true||!doc.ready)&&(doc.refreshing===true||doc.state==="running"','doc.refreshing===true||!doc.ready'),
     ('||patch.ready!==undefined&&patch.ready!==doc.ready;','||patch.ready!==undefined&&patch.ready!==doc.ready||patch.refreshing!==undefined&&patch.refreshing!==doc.refreshing;','patch.refreshing!==doc.refreshing'),
     ('/** 🔒️ The one lease per machine in','''/** 🔄️ The stages a worker that was ready re-verifies in the background while it stays ready: every stage whose own input digest changed, in bootstrap order, `null` when none is stale or the worker is not in a state to stay ready (no marker or `repo` binary, never ready, or a recorded failure: a re-verification that failed leaves Ready). A pure function.
 * @see ./🧪️tests/🔄️refresh/🟦️.ts */
export function readyRefreshV1(input:Readonly<{stages:readonly Readonly<{name:string;digest:string}>[];done:Readonly<Record<string,string>>;marker:boolean;repo:boolean;readyMs:number|null;failure:unknown}>):readonly string[]|null{
 if(!input.marker||!input.repo||input.readyMs===null||input.failure!==null)return null;
 const stale=input.stages.filter(stage=>input.done[stage.name]!==stage.digest).map(stage=>stage.name);
 return stale.length===0?null:Object.freeze(stale);
}

/** 🔄️ A status patch of a stage that runs while the worker stays ready: working states say `ready` with `refreshing` and the stage, the refusal is dropped and the reason names the refresh; a recorded failure (`backoff`) passes through unchanged and every other state change clears `refreshing`. A pure function.
 * @see ./🧪️tests/🔄️refresh/🟦️.ts */
export function readyRefreshingPatchV1(patch:Partial<ReadyStatusV1>,stage:string|null):Partial<ReadyStatusV1>{
 if(patch.state===undefined)return patch;
 if(stage===null||patch.state==="ready"||patch.state==="backoff"||patch.state==="server")return{...patch,refreshing:false};
 return{...patch,state:"ready",ready:true,refreshing:true,refusal:null,reason:readyReasonV1("refreshing",patch.stage??stage)};
}

/** 🔒️ The one lease per machine in''','🔄️ The stages a worker that was ready'),
     (' let memoryLowSince:number|undefined,memoryWaitSaid=false;\n',' let memoryLowSince:number|undefined,memoryWaitSaid=false,refreshStage:string|null=null;\n const status={update:(patch:Partial<ReadyStatusV1>):void=>{ports.status.update(readyRefreshingPatchV1(patch,refreshStage));}};\n','refreshStage:string|null=null'),
     ('   const memory=ports.memory(),lease=ports.lease.peek();\n','   refreshStage=readyRefreshV1({stages,done,marker,repo,readyMs:state.readyMs,failure:state.failure})?.[0]??null;\n   const memory=ports.memory(),lease=ports.lease.peek();\n','refreshStage=readyRefreshV1('),
    ])
    t=open(R+"🟦️.ts",encoding="utf8").read()
    a=t.index("async function runReadyLoopV1"); b=t.index("const isServer=(origin")
    body=t[a:b]
    n=body.count("ports.status.update(")
    body=body.replace("ports.status.update(","status.update(")
    body=body.replace("const status={update:(patch:Partial<ReadyStatusV1>):void=>{status.update(","const status={update:(patch:Partial<ReadyStatusV1>):void=>{ports.status.update(")
    open(R+"🟦️.ts","w",encoding="utf8").write(t[:a]+body+t[b:])
    print("replaced",n)
    edit(R+"💔️silence/🟦️.ts",[
     (' if(ready!==false||state!=="running"&&state!=="waiting"&&state!=="backoff")return{state:"idle",missedBeats,ageMs};',' const refreshing=ready===true&&state==="ready"&&(row as {refreshing?:unknown}).refreshing===true;\n if(!refreshing&&(ready!==false||state!=="running"&&state!=="waiting"&&state!=="backoff"))return{state:"idle",missedBeats,ageMs};','const refreshing=ready===true'),
     ('liveness=silence.state==="silent"?"silent":silence.state==="fresh"?"working":ready?"ready":"idle";','liveness=silence.state==="silent"?"silent":silence.state==="fresh"&&!ready?"working":ready?"ready":"idle";','fresh"&&!ready?"working"'),
    ])
    edit(R+"🌲️process-tree/🟦️.ts",[
     ('const row=status as{ready?:unknown;state?:unknown;updatedMs?:unknown};\n return row.ready===false&&(row.state==="running"||row.state==="waiting"||row.state==="backoff")&&','const row=status as{ready?:unknown;state?:unknown;updatedMs?:unknown;refreshing?:unknown};\n return(row.ready===false&&(row.state==="running"||row.state==="waiting"||row.state==="backoff")||row.ready===true&&row.state==="ready"&&row.refreshing===true)&&','row.refreshing===true)&&'),
    ])
if phase=="age":
    edit(R+"🧬️schema/🔣️.json",[
     ('"leaseStaleMilliseconds": 60000,','"leaseStaleMilliseconds": 60000, "refreshMaximumAgeMilliseconds": 86400000,','"refreshMaximumAgeMilliseconds"'),
    ])
    edit(R+"🟦️.ts",[
     ('export function readyRefreshV1(input:Readonly<{stages:readonly Readonly<{name:string;digest:string}>[];done:Readonly<Record<string,string>>;marker:boolean;repo:boolean;readyMs:number|null;failure:unknown}>):readonly string[]|null{\n if(!input.marker||!input.repo||input.readyMs===null||input.failure!==null)return null;\n const stale=input.stages.filter(stage=>input.done[stage.name]!==stage.digest).map(stage=>stage.name);\n return stale.length===0?null:Object.freeze(stale);',
      'export function readyRefreshV1(input:Readonly<{stages:readonly Readonly<{name:string;digest:string}>[];done:Readonly<Record<string,string>>;doneMs:Readonly<Record<string,number>>;now:number;marker:boolean;repo:boolean;readyMs:number|null;failure:unknown}>):readonly string[]|null{\n if(!input.marker||!input.repo||input.readyMs===null||input.failure!==null)return null;\n const stale=input.stages.filter(stage=>input.done[stage.name]!==stage.digest);\n if(stale.length===0||stale.some(stage=>input.done[stage.name]===undefined||!(input.now-(input.doneMs[stage.name]??-Infinity)<=P.refreshMaximumAgeMilliseconds)))return null;\n return Object.freeze(stale.map(stage=>stage.name));','input.doneMs[stage.name]'),
     ('refreshStage=readyRefreshV1({stages,done,marker,repo,readyMs:state.readyMs,failure:state.failure})','refreshStage=readyRefreshV1({stages,done,doneMs:Object.fromEntries(Object.entries(state.stages).map(([name,row])=>[name,row.doneMs])),now,marker,repo,readyMs:state.readyMs,failure:state.failure})','doneMs:Object.fromEntries(Object.entries(state.stages)'),
     ('own input digest changed, in bootstrap order, `null` when none is stale or the worker is not in a state to stay ready (no marker or `repo` binary, never ready, or a recorded failure: a re-verification that failed leaves Ready).','own input digest changed, in bootstrap order, `null` when none is stale or the worker is not in a state to stay ready (no marker or `repo` binary, never ready, a recorded failure: a re-verification that failed leaves Ready, a stale stage that was never verified here or last verified more than `refreshMaximumAgeMilliseconds` ago: old evidence is no evidence).','more than `refreshMaximumAgeMilliseconds` ago'),
    ])
