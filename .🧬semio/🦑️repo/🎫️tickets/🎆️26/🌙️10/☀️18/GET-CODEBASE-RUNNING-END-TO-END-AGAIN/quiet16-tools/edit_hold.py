"""🤫️ quiet16 batch `hold`, code edits by anchor (rebase route): a quiet forecast holds until work really runs. A nested invocation that inherits a quiet grant runs its body at once with the plan the holder carried (never grows it itself, never hands it the quiet row's single job); an executor asks for the growth and waits for it (planner, graph rebuild, every Bazel action birth: also in a process whose phase marks are off); a birth first tells its action it is `waiting`, which ends the 10 s handshake clock, so the growth may take as long as the fair queue needs; only births take a job slot (an action's own execution join belongs to its birth), so born Bazel actions run only as wide as the jobs the grant holds now (a partial growth never runs at the full width the tool was started with); the holder grows a quiet grant whose tree outgrows the quiet reservation (work no executor announced) without pausing the run; a run whose peak exceeded the quiet size of its class is learned as executed.
usage: python3 -I edit_hold.py <root>   (exit 1 = an anchor is missing or not unique; idempotent: an edit already present is skipped). The test-file edits are the anchored hunks in edit_hold_tests.json, the new files are copied from staged/hold/files. Rebase: a fresh mirror of the live tree (with agents16 landed), then this script, then `make.py hold <mirror>`."""
import os, stat, sys

GATE = "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts"
DOOR = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts"
MARKS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🚏️marks/🟦️.ts"

BAZEL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/"

EDITS = {
    BAZEL + "🌡️host-capacity/🟦️.ts": [
        ("""import {readHostCapacityPlanV1,type HostActionKindV1,type HostCapacityPolicyV1,type HostPlanV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";""",
         """import {readHostCapacityPlanV1,readLiveHostCapacityPlanV1,type HostActionKindV1,type HostCapacityPolicyV1,type HostPlanV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";"""),
        ("""/** 📥️ Reads the plan of the enclosing grant from the environment. */""",
         """/** 🔢️ The jobs the run's grant holds now (the carried plan followed to the grant row: after a partial or reduced growth fewer than the tool was started with, after a later growth more), undefined when the process runs under no grant. */
export function bazelLiveHostJobsV1(environment:Readonly<Record<string,string|undefined>>):number|undefined{return readLiveHostCapacityPlanV1(environment)?.jobs;}

/** 🚦️ The width of a build's real actions: a born action (its work really runs, and its birth already grew a quiet grant) starts only while fewer actions run than the jobs the grant holds now (`jobs`, read live; undefined means no grant and no bound; an idle build always starts one), so a run whose growth was partial never runs wider than its memory carries although its tool was started with the full jobs. Waiters start in arrival order when an action leaves or when `recheck` (every `pollMilliseconds` while someone waits) finds more jobs; a waiter whose key leaves (its connection closed) or whose signal aborts is refused. */
export type BazelActionWidthV1<K>=Readonly<{enter:(key:K,signal?:AbortSignal)=>Promise<void>;leave:(key:K)=>void;recheck:()=>void;running:()=>number;waiting:()=>number}>;
export function createBazelActionWidthV1<K>(jobs:()=>number|undefined,pollMilliseconds:number):BazelActionWidthV1<K>{
 const running=new Set<K>(),queue:{key:K;accept:()=>void;refuse:(error:unknown)=>void}[]=[];let timer:ReturnType<typeof setInterval>|undefined;
 const recheck=()=>{
  while(queue.length){const limit=jobs();if(limit!==undefined&&running.size>0&&running.size>=limit)break;const next=queue.shift()!;running.add(next.key);next.accept();}
  if(queue.length&&timer===undefined){timer=setInterval(recheck,pollMilliseconds);timer.unref?.();}else if(!queue.length&&timer!==undefined){clearInterval(timer);timer=undefined;}
 };
 return Object.freeze({
  enter:(key:K,signal?:AbortSignal)=>new Promise<void>((accept,refuse)=>{
   if(signal?.aborted){refuse(signal.reason);return;}
   const abort=()=>{const at=queue.indexOf(row);if(at>=0){queue.splice(at,1);recheck();refuse(signal!.reason);}},row={key,accept:()=>{signal?.removeEventListener("abort",abort);accept();},refuse:(error:unknown)=>{signal?.removeEventListener("abort",abort);refuse(error);}};
   signal?.addEventListener("abort",abort,{once:true});queue.push(row);recheck();
  }),
  leave:(key:K)=>{if(running.delete(key)){recheck();return;}const at=queue.findIndex(row=>row.key===key);if(at>=0){const[row]=queue.splice(at,1);recheck();row!.refuse(Error("Bazel action left before it started"));}},
  recheck,running:()=>running.size,waiting:()=>queue.length,
 });
}

/** 📥️ Reads the plan of the enclosing grant from the environment. */"""),
    ],
    BAZEL + "📜️script.ts": [
        ("""import {bazelGrantedWorkerHostV1,bazelHostPlanV1,bazelHostResourceSettingsV1,bazelResourceSetsV1} from "./🌡️host-capacity/🟦️.ts";""",
         """import {bazelGrantedWorkerHostV1,bazelHostPlanV1,bazelHostResourceSettingsV1,bazelLiveHostJobsV1,bazelResourceSetsV1,createBazelActionWidthV1} from "./🌡️host-capacity/🟦️.ts";"""),
        ("""const token=randomUUID(),pendingBirths=new Set<Promise<void>>(),""",
         """const token=randomUUID(),width=createBazelActionWidthV1<Socket>(()=>bazelLiveHostJobsV1(process.env),BUILD_HOST_CAPACITY_POLICY_V1.wait.pollMilliseconds),pendingBirths=new Set<Promise<void>>(),"""),
        ("""socket.once("close",()=>connections.delete(socket));""",
         """socket.once("close",()=>{connections.delete(socket);width.leave(socket);});"""),
        ("""wire=JSON.stringify(readIssuedScriptProcessEnvelopeV1(invocation));}await upgradeBuildCapacityV1(signal);const response=""",
         """wire=JSON.stringify(readIssuedScriptProcessEnvelopeV1(invocation));socket.write(JSON.stringify({version:1,event:birthSchema.definitions.Protocol.const.waitingEvent})+"\\n");await upgradeBuildCapacityV1(signal);await width.enter(socket,signal);}else await upgradeBuildCapacityV1(signal);const response="""),
    ],
    BAZEL + "🎬️executor/🟦️.ts": [
        ("""\n/** 🐣️ Obtains unchanged current authority from the live owned parent before the actual action child is born. */\nasync function launchBaze""",
         """\n/** ⏱️ The handshake clock of one action birth: at most `maximumMilliseconds` from the birth's start until the parent says it is `waiting` (the birth is accepted and now waits for the run's grant to grow and for a job of it) or that the action `started`; while it waits only the invocation's own deadline bounds it (no deadline: it waits as long as the parent is alive and the action is not cancelled), so a quiet grant's growth in the fair queue never fails the action. `settle` stops the clock. */\nexport function bazelBirthHandshakeV1(maximumMilliseconds:number,deadlineEpochMilliseconds:number|null,started:number,fail:(error:unknown)=>void):Readonly<{waiting:()=>void;settle:()=>void}>{\n const left=()=>deadlineEpochMilliseconds===null?undefined:Math.max(1,deadlineEpochMilliseconds-Date.now());\n let timer:ReturnType<typeof setTimeout>|undefined=setTimeout(()=>fail(refuseBuildV1("cancelled","bazel.birth-handshake-deadline-exhausted")),Math.max(1,Math.min(maximumMilliseconds-(performance.now()-started),left()??maximumMilliseconds)));\n return Object.freeze({waiting:()=>{clearTimeout(timer);const rest=left();timer=rest===undefined?undefined:setTimeout(()=>fail(refuseBuildV1("cancelled","bazel.birth-deadline-exhausted")),rest);},settle:()=>{clearTimeout(timer);timer=undefined;}});\n}\n\n/** 🐣️ Obtains unchanged current authority from the live owned parent before the actual action child is born; the parent may first say that it is `waiting` (for the grant's growth and a job of it), which ends the handshake clock. */\nasync function launchBaze"""),
        ("""handshake=setTimeout(()=>fail(refuseBuildV1("cancelled","bazel.birth-handshake-deadline-exhausted")),Math.max(1,Math.min(protocol.maximumHandshakeMilliseconds-(performance.now()-started),envelope.deadlineEpochMilliseconds===null?protocol.maximumHandshakeMilliseconds:envelope.deadlineEpochMilliseconds-Date.now())));""",
         """handshake=bazelBirthHandshakeV1(protocol.maximumHandshakeMilliseconds,envelope.deadlineEpochMilliseconds,started,fail);"""),
        ("""else if(message.event==="started"&&id&&!issued){issued=true;clearTimeout(handshake);acceptJoin(expected);}""",
         """else if(message.event===protocol.waitingEvent&&!id&&!issued)handshake.waiting();else if(message.event==="started"&&id&&!issued){issued=true;handshake.settle();acceptJoin(expected);}"""),
        ("""}clearTimeout(handshake);const timer=setTimeout(()=>socket.destroy(refuseBuildV1("cancelled","bazel.birth-termination-rec""",
         """}handshake.settle();const timer=setTimeout(()=>socket.destroy(refuseBuildV1("cancelled","bazel.birth-termination-rec"""),
    ],
    BAZEL + "🧬️schema/🔣️.json": [
        ("""            "maximumHandshakeMilliseconds": 10000,
            "readChunkBytes": 65536,
            "cooperativeCancellationRole": "execution"
          }""",
         """            "maximumHandshakeMilliseconds": 10000,
            "readChunkBytes": 65536,
            "cooperativeCancellationRole": "execution",
            "waitingEvent": "waiting"
          }"""),
    ],
    GATE: [
        (" A nested invocation that inherits a quiet grant first has it grown and then hands its body the grant as it is after the growth (its jobs, cores and plan), never the quiet row's single job. */",
         " A nested invocation runs its body under an inherited grant at once, with the plan the holder carried and its jobs and cores (for a quiet grant those of the full request, never the quiet row's single job: a forecast changes what a run reserves, never what it may use); it never grows a quiet grant itself, an executor of the run asks when work has to run (`requestHostCapacityUpgradeV1`). */"),
        ("""  let grant=inherited as HostGrantV1;
  if(grant.quiet===true){await(await import("../🔼️upgrade/🟦️.ts")).requestHostCapacityUpgradeV1(environment,options.control);const current=readJson(inheritedPath);if(isHostCapacityV1("Grant",current)&&(current as HostGrantV1).id===grant.id)grant=current as HostGrantV1;}
  const carried=readHostCapacityPlanV1(environment),plan=""",
         """  const grant=inherited as HostGrantV1,carried=readHostCapacityPlanV1(environment),plan="""),
        ("return body(Object.freeze({id:grant.id,jobs:grant.jobs,cpu:grant.cpu,reservedMegabytes:grant.reservedMegabytes,plan,inherited:true,",
         "return body(Object.freeze({id:grant.id,jobs:plan.jobs,cpu:plan.cpu,reservedMegabytes:grant.reservedMegabytes,plan,inherited:true,"),
        ("and its reservation follows the `resize` port at each boundary; the segments go to `learn`. */",
         "and its reservation follows the `resize` port at each boundary; the segments go to `learn`. A quiet grant whose tree holds more than its quiet reservation grows as if an executor had asked: work runs that no executor announced (the executors, which wait for the growth, are the planner, a graph rebuild and the birth of every Bazel action that really runs); the run goes on meanwhile and is learned as executed. */"),
        ("let growing=false,settled:HostGrowthSettleV1|undefined,",
         "let growing=false,outgrown=false,settled:HostGrowthSettleV1|undefined,"),
        ("publish({...state,observedMegabytes:observed});}}).catch(()=>undefined);};",
         "publish({...state,observedMegabytes:observed});if(quiet&&state.quiet===true&&observed>state.reservedMegabytes)outgrown=true;}}).catch(()=>undefined);};"),
        ("if(state.quiet!==true||upgrading||stopped||!existsSync(upgradeFile)||now()-attempted<policy.wait.pollMilliseconds)return;",
         "if(state.quiet!==true||upgrading||stopped||!outgrown&&!existsSync(upgradeFile)||now()-attempted<policy.wait.pollMilliseconds)return;"),
        ("requested:()=>existsSync(upgradeFile),",
         "requested:()=>outgrown||existsSync(upgradeFile),"),
        ("executed:state.upgradedAtMs!==undefined||existsSync(upgradeFile)},",
         "executed:state.upgradedAtMs!==undefined||outgrown||existsSync(upgradeFile)},"),
    ],
    DOOR: [
        (" The executors of a build tool call it before the first thing that is not a pure replay of earlier results: the planner, a graph rebuild, the first action that really runs.",
         " The executors of a build tool call it before the first thing that is not a pure replay of earlier results (the planner, a graph rebuild, the birth of every Bazel action that really runs, whose width then follows the jobs the grant holds), in the holder's process or in any process that inherits the grant; work that no executor announces grows the grant once the run's tree outgrows the quiet reservation, without pausing it."),
        ("then hands the body the arguments rewritten to the granted jobs and sets the environment the children inherit; light commands pass untouched. */",
         "then hands the body the arguments rewritten to the granted jobs and sets the environment the children inherit; light commands pass untouched. A run of a forecast-aware tool is learned as one that executed work when an executor asked for it or when its peak exceeded the quiet size of its class, so the quiet class learns only what quiet runs hold and the next forecast follows the run's own last outcome. */"),
        ("const forecasted=run.executed===undefined?undefined:{executed:run.executed,",
         "const forecasted=run.executed===undefined?undefined:{executed:run.executed||forecast!==undefined&&run.peakMegabytes>forecast.megabytes,"),
    ],
    MARKS: [
        ("""/** 🚏️ Says which phase the run of this process enters (`assembly`, `plan`, `rebuild`, `execution`), so the host capacity gate can cut the run into phases, re-size its reservation at the boundary and learn each phase under its own class; a process that runs under no grant, or under an inherited one, says nothing and costs nothing (the gate module loads only when a grant exists). Entering `rebuild` first grows a quiet forecast grant into its full request (a graph rebuild is work that has to run), so the rebuild is booked only after it fits. */
export async function markBuildCapacityPhaseV1(phase:"assembly"|"plan"|"rebuild"|"execution",environment:Readonly<Record<string,string|undefined>>=process.env):Promise<boolean>{
 if(environment["SEMIO_HOST_CAPACITY_GRANT"]===undefined||environment["SEMIO_HOST_CAPACITY_PHASE_MARKS"]==="off")return false;
 if(phase==="rebuild")await(await import("../🟦️.ts")).upgradeBuildCapacityV1(undefined,environment);
 return""",
         """/** 🚏️ Says which phase the run of this process enters (`assembly`, `plan`, `rebuild`, `execution`), so the host capacity gate can cut the run into phases, re-size its reservation at the boundary and learn each phase under its own class; a process that runs under no grant says nothing and costs nothing (the gate module loads only when a grant exists), one under an inherited grant (phase marks off) marks nothing. Entering `rebuild` first grows a quiet forecast grant into its full request in every process of the run, the inheriting ones included (a graph rebuild is work that has to run), so the rebuild is booked only after it fits. */
export async function markBuildCapacityPhaseV1(phase:"assembly"|"plan"|"rebuild"|"execution",environment:Readonly<Record<string,string|undefined>>=process.env):Promise<boolean>{
 if(environment["SEMIO_HOST_CAPACITY_GRANT"]===undefined)return false;
 if(phase==="rebuild")await(await import("../🟦️.ts")).upgradeBuildCapacityV1(undefined,environment);
 if(environment["SEMIO_HOST_CAPACITY_PHASE_MARKS"]==="off")return false;
 return"""),
        ("""/** ⏫️ Grows a quiet forecast grant into its full request BEFORE the graph builder custody is taken: the wait for capacity is a fair queue with no timer, so it must not happen while a custody is held that every other builder queues for, and not while other runs hold full grants and wait for that custody (each would wait for the other); a process under no grant, or one that only inherits it, asks for nothing. */
export async function upgradeBuildCapacityBeforeRebuildV1(environment:Readonly<Record<string,string|undefined>>=process.env):Promise<boolean>{
 if(environment["SEMIO_HOST_CAPACITY_GRANT"]===undefined||environment["SEMIO_HOST_CAPACITY_PHASE_MARKS"]==="off")return false;
 await(await import("../🟦️.ts")).upgradeBuildCapacityV1(undefined,environment);return true;
}""",
         """/** ⏫️ Grows a quiet forecast grant into its full request BEFORE the graph builder custody is taken: the wait for capacity is a fair queue with no timer, so it must not happen while a custody is held that every other builder queues for, and not while other runs hold full grants and wait for that custody (each would wait for the other). Every process of the run asks, the one that only inherits the grant included (an inherited quiet grant is not grown when it is inherited, so the rebuild is where it grows); answers whether a live grant was asked, a process under no grant asks for nothing. */
export async function upgradeBuildCapacityBeforeRebuildV1(environment:Readonly<Record<string,string|undefined>>=process.env):Promise<boolean>{
 if(environment["SEMIO_HOST_CAPACITY_GRANT"]===undefined)return false;
 return(await(await import("../🟦️.ts")).upgradeBuildCapacityV1(undefined,environment))!=="absent";
}"""),
    ],
}


def main():
    import json, shutil
    root = os.path.abspath(sys.argv[1])
    here = os.path.dirname(os.path.abspath(__file__))
    edits = {**EDITS, **{rel: [tuple(pair) for pair in pairs] for rel, pairs in json.load(open(os.path.join(here, "edit_hold_tests.json"), encoding="utf8")).items()}}
    for rel, row in json.load(open(os.path.join(here, "staged", "hold", "manifest.json"), encoding="utf8")).items():
        if row["base"] is None:
            os.makedirs(os.path.dirname(os.path.join(root, rel)), exist_ok=True)
            shutil.copyfile(os.path.join(here, "staged", "hold", "files", row["file"]), os.path.join(root, rel))
            print("new", rel[-70:])
    for rel, pairs in edits.items():
        path = os.path.join(root, rel)
        text = open(path, encoding="utf8").read()
        for old, new in pairs:
            if new in text and old not in text:
                continue
            if text.count(old) != 1:
                print("ANCHOR", text.count(old), rel[-60:], old[:90])
                sys.exit(1)
            text = text.replace(old, new)
        mode = stat.S_IMODE(os.stat(path).st_mode)
        temporary = path + ".quiet16.tmp"
        open(temporary, "w", encoding="utf8").write(text)
        os.chmod(temporary, mode)
        os.replace(temporary, path)
        print("edited", rel[-70:])


main()
