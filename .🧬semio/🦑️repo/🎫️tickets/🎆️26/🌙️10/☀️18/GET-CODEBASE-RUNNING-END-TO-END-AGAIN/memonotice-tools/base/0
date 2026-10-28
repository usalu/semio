import {join} from "node:path";
import {GRAPH_MEMO_POLICY_V1,graphMemoEnabledV1,graphMemoKeyV1,openGraphMemoV1,renderGraphMemoTextV1,type GraphMemoV1,type MemoAnswerV1} from "../🟦️.ts";
import {canonicalRootV1} from "../../../../🏡️resident/📨️protocol/📍️endpoint/🟨️.mjs";
import {changeJournalDirectoryV1,connectChangeJournalV1} from "../../../../🏡️resident/📜️journal/🟦️.ts";
declare global{interface ImportMeta{readonly semioCapturedImplementationIdentity?:string;readonly semioCapturedLoaderIdentity?:string}}

/** 🧠️ Opens the graph memo of one command in this process: off when the switch says so or when this process does not run a captured implementation (the identity of the implementation is part of the key). The journal cursor is taken here, before any planner child starts.
 * @see ../🟦️.ts the policy and the lookup */
export async function openGraphMemoForCommandV1(root:string,args:readonly string[],capabilities:Readonly<{tool?:string;buildMode?:string}>,signal?:AbortSignal,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<GraphMemoV1|undefined>{
 if(!graphMemoEnabledV1(environment))return undefined;
 const implementation=import.meta.semioCapturedImplementationIdentity;
 if(typeof implementation!=="string")return undefined;
 const canonical=canonicalRootV1(root);
 return openGraphMemoV1({root,directory:join(changeJournalDirectoryV1(canonical,environment),GRAPH_MEMO_POLICY_V1.directory),client:connectChangeJournalV1(canonical,environment),key:graphMemoKeyV1({root,args,tool:capabilities.tool,mode:capabilities.buildMode,implementation:implementation+":"+(import.meta.semioCapturedLoaderIdentity??""),environment}),...(signal?{signal}:{}),environment,
  holds:async(rows,abort)=>{const {observationsHold}=await import("../../../../👁️observation/🟨️.mjs");return observationsHold(rows,64,new Set(),true,abort);}});
}

/** 🗣️ The one line a served graph prints, in the language of the command. */
export async function renderGraphMemoServedV1(answer:Extract<MemoAnswerV1,{hit:true}>,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<string>{
 const {resolveLanguageV1}=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts"),{CATALOG_LANGUAGES_V1}=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts");
 return renderGraphMemoTextV1("served",{projects:answer.projects,seconds:Math.round(answer.ageMilliseconds/1000),rows:answer.rows,rechecked:answer.rechecked},resolveLanguageV1(CATALOG_LANGUAGES_V1,[],environment as NodeJS.ProcessEnv).language);
}

/** 🧰️ What the planner of one request needs from its caller: the memo of the request, where the child leaves its relevance, how to read the slice, where to record the pin, where to print and mark, and the planner child itself. */
export type GraphMemoPlanInputV1<T extends Readonly<{projects:Readonly<Record<string,unknown>>}>>=Readonly<{
 memo:GraphMemoV1|undefined;stem:string;parse:(text:string)=>T;record:(pin:string|undefined)=>void;notice:(text:string)=>void;mark:(name:string,detail:string)=>void;environment?:Readonly<Record<string,string|undefined>>;
 plan:(sidecar:Readonly<Record<string,string>>|undefined)=>Promise<Readonly<{plan:string;pin:string|undefined}>>;
}>;

/** 🗺️ Serves a request from its memo when the journal proves that nothing the generation read has changed; otherwise runs the planner child (asking it for its relevance when a light record names the generation it will serve) and keeps what it produced. A request without a memo is planned as before.
 * @see ../🟦️.ts the policy and the lookup */
export async function planGraphWithMemoV1<T extends Readonly<{projects:Readonly<Record<string,unknown>>}>>(input:GraphMemoPlanInputV1<T>):Promise<Readonly<{graph:T;served:boolean}>>{
 const {memo,stem}=input,served=await memo?.lookup();
 input.mark("graph.memo",served===undefined?"off":served.hit?"hit":"miss:"+served.reason);
 if(served?.hit){const graph=input.parse(served.plan);input.record(served.pin);input.notice(await renderGraphMemoServedV1(served,input.environment));return{graph,served:true};}
 const expect=memo?.cursor===undefined?undefined:memo.expected(),produced=await input.plan(expect===undefined?undefined:{[GRAPH_MEMO_POLICY_V1.sidecar]:stem+".json",[GRAPH_MEMO_POLICY_V1.expect]:expect}),graph=input.parse(produced.plan);
 input.record(produced.pin);
 if(memo)input.mark("graph.memo-seal",await memo.seal({plan:produced.plan,pin:produced.pin,projects:Object.keys(graph.projects).length,sidecar:stem+".json"}));
 return{graph,served:false};
}
