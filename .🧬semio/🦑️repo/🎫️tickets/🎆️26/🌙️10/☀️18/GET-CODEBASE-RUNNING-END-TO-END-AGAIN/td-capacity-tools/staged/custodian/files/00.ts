import {writeFileSync} from "node:fs";
import {withPaidProcessCustodianV1} from "../../🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};

/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  
  if(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION!==undefined)throw new Error("Explicit authored deadline entry cannot ignore an inherited Original");
  const started=Date.now(),envelope=createScriptProcessEnvelope({version:1,owner:"test:paid-custodian-idle-deadline",maximumElapsedMilliseconds:fixture.idleDeadlineMilliseconds},{},started);let failed=false,ready=false,readyAt:number|null=null;
  try{await withScriptProcessEnvelope(envelope,original=>withPaidProcessCustodianV1(original,{version:1,coverage:"direct-child",directory:process.argv[2]!,parallelism:fixture.parallelism,jobs:[{program:process.execPath,args:["-e",""],cwd:process.cwd(),parallelism:fixture.parallelism,output:fixture.output}]},async()=>{ready=true;readyAt=Date.now();writeFileSync(process.argv[3]!,"");await new Promise<void>((_,reject)=>{const abort=()=>reject(original.control.signal.reason);original.control.signal.addEventListener("abort",abort,{once:true});if(original.control.signal.aborted)abort();});}));}catch{failed=true;}
  process.stdout.write(JSON.stringify({failed,ready,started,deadline:envelope.deadlineEpochMilliseconds,readyAt,finished:Date.now()}));
}
