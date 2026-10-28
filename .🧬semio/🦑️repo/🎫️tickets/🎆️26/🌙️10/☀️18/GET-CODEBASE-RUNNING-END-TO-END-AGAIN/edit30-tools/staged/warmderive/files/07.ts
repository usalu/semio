/** 🏁️ The detached warm deriver process: `--root <checkout> --identity <run implementation> --endpoint <socket or pipe> --directory <private directory holding closure.json and request.json>`. It refuses to start unless every file of the run's captured closure is byte-identical on disk (checked again after its modules loaded) and no deriver serves the endpoint yet, warms the programs with the copied request, serves until idle and removes its directory.
 * @see ../🟦️.ts */
import {rmSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {GRAPH_MEMO_POLICY_V1} from "../../../🟦️.ts";
import {answerGraphMemoDerivationV1,openPlannerDeriverV1} from "../../🟦️.ts";
import {createTypeScriptProgramReuseV1,useTypeScriptProgramReuseV1} from "../../../../../../🕸️graph/📥️inference/🧭️script-inputs/♨️reuse/🟨️.mjs";
import {observationsHold,releaseInferenceMemoryV1} from "../../../../../../👁️observation/🟨️.mjs";
import {serveWarmDeriverV1,warmClosureHoldsV1,warmEndpointAliveV1,warmProgramDeriverV1,type WarmClosureV1} from "../🟦️.ts";

/** 🏃️ Runs the warm deriver of one command line; resolves when it stopped. */
export async function runWarmDeriverMainV1(argv:readonly string[]):Promise<number>{
 const option=(name:string)=>{const at=argv.indexOf(name);return at>=0?argv[at+1]:undefined;},root=option("--root"),identity=option("--identity"),endpoint=option("--endpoint"),directory=option("--directory");
 if(!root||!identity||!endpoint||!directory)return 2;
 try{
  const closure=JSON.parse(readFileSync(join(directory,"closure.json"),"utf8")) as WarmClosureV1;
  if(!warmClosureHoldsV1(closure))return 3;
  if(await warmEndpointAliveV1(endpoint))return 4;
  const store=createTypeScriptProgramReuseV1(),memory=new Map();useTypeScriptProgramReuseV1(store);
  const answer=async(request:string,signal:AbortSignal)=>{
   releaseInferenceMemoryV1();
   try{await answerGraphMemoDerivationV1(root,{[GRAPH_MEMO_POLICY_V1.derive]:request},signal,async at=>warmProgramDeriverV1(await openPlannerDeriverV1(at),store,memory,(rows,abort)=>observationsHold(rows,64,new Set(),new Set(),abort)));}
   finally{releaseInferenceMemoryV1();if(process.memoryUsage().rss>GRAPH_MEMO_POLICY_V1.warm.maximumResidentBytes)setImmediate(()=>{void server?.close();});}
  };
  let server:Awaited<ReturnType<typeof serveWarmDeriverV1>>|undefined;
  const warmup=join(directory,"request.json");
  try{await answer(warmup,new AbortController().signal);}catch{}
  if(!warmClosureHoldsV1(closure))return 3;
  let stopped:()=>void=()=>{};const done=new Promise<void>(resolve=>{stopped=resolve;});
  server=await serveWarmDeriverV1({endpoint,identity,idleMilliseconds:GRAPH_MEMO_POLICY_V1.warm.idleMilliseconds,answer,onClose:()=>stopped()});
  for(const signal of["SIGTERM","SIGINT"] as const)process.once(signal,()=>{void server?.close();});
  await done;return 0;
 }catch{return 1;}
 finally{rmSync(directory,{recursive:true,force:true});}
}

if(import.meta.main)process.exit(await runWarmDeriverMainV1(process.argv.slice(2)));
