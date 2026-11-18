import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from lib import *
R=LIB+"/🏗️build/🏡️resident"
PROCESS=R+"/🏗️composition/🌐️host/👷️slice-pool/🧵️process/🟨️.mjs"
POOL=R+"/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs"
SERVICE=R+"/🏛️service/🌐️host/🟨️.mjs"
SCHEMA=R+"/🧬️schema/🔣️.json"
def patch_process(t):
    t=sub1(t,'stdio:["ignore","ignore","inherit","ipc"]','stdio:["ignore","ignore","pipe","ipc"]')
    t=sub1(t,' const pending=new Set();\n',' const pending=new Set();\n let tail="";\n child.stderr?.on("data",chunk=>{try{process.stderr.write(chunk);}catch{}tail=(tail+chunk.toString("utf8")).slice(-STDERR_TAIL_CHARACTERS_V1);});\n')
    t=sub1(t,'else if(event==="exit")child.on("exit",code=>{void Promise.allSettled([...pending]).then(()=>listener(code));});','else if(event==="exit")child.on("exit",code=>{void Promise.race([new Promise(done=>child.once("close",done)),new Promise(done=>setTimeout(done,STDERR_DRAIN_MILLISECONDS_V1).unref?.())]).then(()=>Promise.allSettled([...pending])).then(()=>listener(code));});')
    t=sub1(t,'  pid:child.pid,\n','  pid:child.pid,\n  stderrTail:()=>tail,\n')
    t=sub1(t,'export const SLICE_SPOOL_VARIABLE_V1="SEMIO_SLICE_SPOOL";\n','''export const SLICE_SPOOL_VARIABLE_V1="SEMIO_SLICE_SPOOL";

/** 🧾 How much of a worker's last words the pool keeps, and how long it waits for them after the exit (a farm child that inherited the pipe may hold it open). */
const STDERR_TAIL_CHARACTERS_V1=4096,STDERR_DRAIN_MILLISECONDS_V1=500,CAUSE_CHARACTERS_V1=300;

/** 🗣 The line of a worker's last words that says why it ended: the last line that names an error, else the last line, without colour codes and at most 300 characters; nothing when the worker said nothing. */
export function workerCauseV1(text){
 const lines=String(text??"").replace(/\\u001b\\[[0-9;]*m/gu,"").split("\\n").map(line=>line.trim()).filter(line=>line!=="");
 const named=[...lines].reverse().find(line=>/error|cannot find|not found|failed|unexpected|fehler/iu.test(line));
 return (named??lines.at(-1)??"").slice(0,CAUSE_CHARACTERS_V1);
}
''')
    return t
def patch_pool(t):
    t=sub1(t,'import {createProcessWorkerV1,errorOfMessageV1} from "./🧵️process/🟨️.mjs";','import {createProcessWorkerV1,errorOfMessageV1,workerCauseV1} from "./🧵️process/🟨️.mjs";')
    t=sub1(t,'totalMaximum:HOST_POLICY_V1.sliceRestartTotalMaximum}','totalMaximum:HOST_POLICY_V1.sliceRestartTotalMaximum,sameCauseMaximum:HOST_POLICY_V1.sliceRestartSameCauseMaximum}')
    t=sub1(t,'{exit:code,restart:code===HOST_POLICY_V1.sliceRestartExitCode,ranMilliseconds:Math.round(performance.now()-started)}','{exit:code,restart:code===HOST_POLICY_V1.sliceRestartExitCode,ranMilliseconds:Math.round(performance.now()-started),workerCause:typeof worker.stderrTail==="function"?workerCauseV1(worker.stderrTail()):""}')
    t=sub1(t,'let heavyHandoffs=0,totalRestarts=0,churnSince,streak=0;','let heavyHandoffs=0,totalRestarts=0,churnSince,streak=0,lastCause="",sameCause=0;')
    t=sub1(t,'    if(lasted){churnSince=undefined;streak=0;}\n','    const cause=String(error.workerCause??"");sameCause=!lasted&&cause!==""&&cause===lastCause?sameCause+1:cause!==""?1:0;lastCause=lasted?"":cause;\n    if(cause!==""&&sameCause>=(restartPolicy.sameCauseMaximum??Infinity))throw Object.assign(new Error(Object.values(renderBothV1("host-slice-worker-failing",{count:sameCause,exit:error.exit,cause})).join("\\n")),{code:"slice-worker-failing",cause,count:sameCause,exit:error.exit});\n    if(lasted){churnSince=undefined;streak=0;}\n')
    t=sub1(t,'decision={key:"restarted",used:Math.round(settleWaited/1000),budget:Math.round(restartPolicy.settleMilliseconds/1000),kind:String(error.exit),limit:Math.round(pause/1000)};','decision={key:cause===""?"restarted":"restarted-cause",used:Math.round(settleWaited/1000),budget:Math.round(restartPolicy.settleMilliseconds/1000),kind:String(error.exit),limit:Math.round(pause/1000),...cause===""?{}:{cause}};')
    return t
def patch_service(t):
    t=sub1(t,'limit:decision.limit??0}})});};','limit:decision.limit??0,cause:decision.cause??""}})});};')
    t=sub1(t,'   else if(error?.code==="slice-restarts"){','   else if(error?.code==="slice-worker-failing"){await refuse(connection,frame.id,"failed","host-slice-worker-failing",{count:error.count,exit:error.exit,cause:error.cause},"never");}\n   else if(error?.code==="slice-restarts"){')
    return t
def patch_schema(t):
    t=sub1(t,'    "sliceRestartTotalMaximum": 120,','    "sliceRestartTotalMaximum": 120,\n    "sliceRestartSameCauseMaximum": 3,')
    t=sub1(t,'      "host-slice-restarts": "The plan cannot continue','      "host-slice-owners-restarted-cause": "Reading the project graph slice ({seconds} s): the slice worker ended with exit code {kind} and said: {cause}. It is started again in {limit} s (the replica has not settled for {used} of {budget} s).",\n      "host-slice-worker-failing": "The plan cannot continue: the slice worker ended {count} times in a row with exit code {exit} and the same text: {cause}",\n      "host-slice-restarts": "The plan cannot continue')
    t=sub1(t,'      "host-slice-restarts": "Der Plan','      "host-slice-owners-restarted-cause": "Lese den Projektgraph-Ausschnitt ({seconds} s): der Ausschnitt-Worker endete mit Exit-Code {kind} und meldete: {cause}. Er wird in {limit} s neu gestartet (das Replikat ist seit {used} von {budget} s nicht zur Ruhe gekommen).",\n      "host-slice-worker-failing": "Der Plan kann nicht fortgesetzt werden: der Ausschnitt-Worker endete {count}-mal hintereinander mit Exit-Code {exit} und demselben Text: {cause}",\n      "host-slice-restarts": "Der Plan')
    return t
if __name__=="__main__":
    for name,path,fn in [("process",PROCESS,patch_process),("pool",POOL,patch_pool),("service",SERVICE,patch_service),("schema",SCHEMA,patch_schema)]:
        t=read(path);n=fn(t);print(name,sha(t),"->",sha(n))
