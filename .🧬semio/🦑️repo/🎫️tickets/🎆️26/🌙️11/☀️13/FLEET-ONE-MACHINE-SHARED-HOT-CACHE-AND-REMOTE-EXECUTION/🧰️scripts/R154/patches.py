import hashlib,os,sys
ROOT="/Users/ueli/Documents/semio"
B=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
OBS=B+"👁️observation/🟨️.mjs"
LOAD=B+"🕸️graph/📥️load/🟦️.ts"
REP=B+"🧠️memory/📋️slice-report/🟨️.mjs"
NODES=B+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"
FAIL=B+"🏡️resident/🏛️service/🧯️failure/🟨️.mjs"
def read(p): return open(p,encoding="utf8").read()
def write(p,s): open(p,"w",encoding="utf8").write(s)
def sha(s): return hashlib.sha256(s.encode("utf8")).hexdigest()
def sub(s,old,new,label):
    n=s.count(old)
    if n!=1: raise SystemExit("anchor %s found %d times"%(label,n))
    return s.replace(old,new)

def obs(s):
    s=sub(s,'/** 🚧️ The declared limits of the observation module; its schema file is their single source. */',
'''/** 🧊️ The failure of an attempt whose source facts a repeated inference left as they were (a process that reads a pinned implementation file whose bytes changed on disk cannot refresh them): it ends as a stale implementation, so the outermost command starts again with the current code instead of repairing for ever. */
export function staleSourceFactsErrorV1(rows){const named=rows.slice(0,4).map(row=>row.kind+":"+row.path).join(", ");return Object.assign(staleImplementationV1("Stale source facts that a repeated inference cannot refresh; restart required: "+named+" / Veraltete Quellfakten, die eine erneute Ableitung nicht erneuern kann; Neustart erforderlich: "+named),{code:"BUILD_AUTHORITY_CHANGED",paths:rows.slice(0,8).map(row=>row.path)});}

/** 🚧️ The declared limits of the observation module; its schema file is their single source. */''',"limits-doc")
    s=sub(s,'const value=await observeInferenceComputationV1({cachePath,key,reusable,prepare,fleet,deferOffer,verdict,verdictFinal,prepareAttempts}','let own;const value=await observeInferenceComputationV1({cachePath,key,reusable,prepare,fleet,deferOffer,verdict,verdictFinal,prepareAttempts,published:found=>{own=found;}}',"call")
    s=sub(s,'const publication=originalInferencePublications.get(cachePath);\n if(!publication)throw Error("Original project inference observations required");','const publication=own??originalInferencePublications.get(cachePath);\n if(!publication)throw Object.assign(Error("Original project inference observations required for "+String(owner)+" ("+cachePath+") / Ursprüngliche Beobachtungen der Projektableitung erforderlich für "+String(owner)+" ("+cachePath+")"),{code:"BUILD_INFERENCE_PUBLICATION_LOST",semioProject:typeof owner==="string"?owner:undefined});',"publication")
    s=sub(s,'async function observeInferenceComputationV1({cachePath,key,reusable,prepare,fleet,deferOffer=false,verdict,verdictFinal=false,prepareAttempts=1},compute,nestedProject) {','async function observeInferenceComputationV1({cachePath,key,reusable,prepare,fleet,deferOffer=false,verdict,verdictFinal=false,prepareAttempts=1,published},compute,nestedProject) {',"sig")
    s=sub(s,'originalInferencePublications.set(cachePath,{path:cachePath,digest,authorityObservations:header.authorityObservations,observations:header.observations});return{value};','{const found={path:cachePath,digest,authorityObservations:header.authorityObservations,observations:header.observations};originalInferencePublications.set(cachePath,found);published?.(found);}return{value};',"set-reuse")
    s=sub(s,'if (jsonImage(result) !== undefined) originalInferencePublications.set(cachePath, {path:cachePath,digest,authorityObservations:authority,observations:rows});','if (jsonImage(result) !== undefined) { const found={path:cachePath,digest,authorityObservations:authority,observations:rows}; originalInferencePublications.set(cachePath, found); published?.(found); }',"set-compute")
    return s

def load(s):
    s=sub(s,'''  for(let repair=0;!holds&&before===after&&unchanged()&&repair<GRAPH_LIMITS_V1.repairRounds;repair++){
    const repairStarted=performance.now();process.stderr.write("[graph-repair] "+JSON.stringify({round:repair+1,stale:stale.map(row=>row.path)})+"\\n");stale.length=0;''','''  const staleKey=rows=>JSON.stringify(rows.map(row=>[row.kind,row.path,row.fact]).sort((left,right)=>(left[1] as string)<(right[1] as string)?-1:(left[1] as string)>(right[1] as string)?1:0));
  let unrefreshed=holds?"":staleKey(stale);
  for(let repair=0;!holds&&before===after&&unchanged()&&repair<GRAPH_LIMITS_V1.repairRounds;repair++){
    const repairStarted=performance.now();process.stderr.write("[graph-repair] "+JSON.stringify({round:repair+1,stale:stale.map(row=>row.path)})+"\\n");stale.length=0;''',"repair-head")
    s=sub(s,'''    holds=await buildGraphSourceFactsHoldV1(captured.observations,invocation,collect);timing("graph-repair",repairStarted,{round:repair+1,holds});
  }''','''    holds=await buildGraphSourceFactsHoldV1(captured.observations,invocation,collect);timing("graph-repair",repairStarted,{round:repair+1,holds});
    if(!holds&&stale.length>0&&staleKey(stale)===unrefreshed){process.stderr.write("[graph-retry] "+JSON.stringify({version:1,reason:"unrefreshed-facts",stale:stale.map(row=>row.path)})+"\\n");throw observation.staleSourceFactsErrorV1(stale);}
    unrefreshed=holds?"":staleKey(stale);
  }''',"repair-tail")
    return s

def rep(s):
    s=sub(s,"sources=new Map(),startedAt=performance.now();","sources=new Map(),finals=new Map(),startedAt=performance.now();","finals")
    s=sub(s,'''export function registerSliceReportSourceV1(name,read){sources.set(name,read);return()=>{if(sources.get(name)===read)sources.delete(name);};}''','''export function registerSliceReportSourceV1(name,read){sources.delete(name);finals.delete(name);sources.set(name,read);return()=>{if(sources.get(name)===read){try{finals.set(name,read());}catch{}sources.delete(name);}};}

/** 🏁️ What the source `name` answered when it was removed, or undefined while it is registered or never was. */
export function readFinalSliceReportSourceV1(name){return sources.has(name)?undefined:finals.get(name);}''',"register")
    s=sub(s,'''export function startSliceProgressV1({emit,read=readSliceReportSourceV1,milliseconds=P.progressMilliseconds}){
 let told;
 const timer=setInterval(()=>{const ledger=read("owners")?.ledger,done=Number(ledger?.done),total=Number(ledger?.total);if(!Number.isSafeInteger(done)||!Number.isSafeInteger(total)||total<=0||done===told)return;told=done;try{emit({phase:"taken",done,total});}catch{}},milliseconds);''','''export function startSliceProgressV1({emit,read=readSliceReportSourceV1,final=readFinalSliceReportSourceV1,milliseconds=P.progressMilliseconds}){
 let told,finished;
 const timer=setInterval(()=>{const live=read("owners"),ledger=(live??final("owners"))?.ledger,done=Number(ledger?.done),total=Number(ledger?.total);if(!Number.isSafeInteger(done)||!Number.isSafeInteger(total)||total<=0)return;if(live===undefined){if(finished===ledger)return;finished=ledger;told=undefined;try{emit({phase:"owners done",done,total});}catch{}return;}finished=undefined;if(done===told)return;told=done;try{emit({phase:"taken",done,total});}catch{}},milliseconds);''',"progress")
    return s

def nodes(s):
    s=sub(s,'''async function tagProjectFailureV1(root, run) {
  try { return await run(); }
  catch (error) { if (error !== null && typeof error === "object" && !Object.hasOwn(error, "semioProject") && Object.isExtensible(error)) Object.defineProperty(error, "semioProject", { value: root, configurable: true }); throw error; }
}''','''export async function tagProjectFailureV1(root, run) {
  try { return await run(); }
  catch (error) {
    if (error === null || typeof error !== "object") throw Object.assign(new Error(String(error)), { semioProject: root });
    if (Object.hasOwn(error, "semioProject")) throw error;
    if (!Object.isExtensible(error)) throw Object.assign(new Error(String(error.message ?? error), { cause: error }), { ...(typeof error.code === "string" ? { code: error.code } : {}), semioProject: root });
    Object.defineProperty(error, "semioProject", { value: root, configurable: true });
    throw error;
  }
}''',"tag")
    return s

def fail(s):
    s=sub(s,''' const text=String(error?.message??error).replace(/\\s+/gu," ").trim(),code=typeof error?.code==="string"&&!text.includes(error.code)?" ["+error.code+"]":"";
 return(String(error?.name??"Error")+code+": "+text).slice(0,FAILURE_MESSAGE_CHARACTERS_V1);''',''' const text=String(error?.message??error).replace(/\\s+/gu," ").trim(),code=typeof error?.code==="string"&&!text.includes(error.code)?" ["+error.code+"]":"",owner=typeof error?.semioProject==="string"&&!text.includes(error.semioProject)?" (project "+error.semioProject+")":"";
 return(String(error?.name??"Error")+code+owner+": "+text).slice(0,FAILURE_MESSAGE_CHARACTERS_V1);''',"message")
    return s

OBSD=B+"👁️observation/🟨️.d.mts"
REPD=B+"🧠️memory/📋️slice-report/🟨️.d.mts"
def obsd(s):
    return sub(s,"export function requireCurrentObservationV1():void;","export function requireCurrentObservationV1():void;\nexport function staleSourceFactsErrorV1(rows:readonly {kind:string;path:string}[]):Error;","d-stale")
def repd(s):
    s=sub(s,"export function readSliceReportSourceV1(name:string):unknown;","export function readSliceReportSourceV1(name:string):unknown;\nexport function readFinalSliceReportSourceV1(name:string):unknown;","d-final")
    return sub(s,"read?:(name:string)=>unknown;milliseconds?:number}","read?:(name:string)=>unknown;final?:(name:string)=>unknown;milliseconds?:number}","d-start")
TARGETS={OBS:obs,LOAD:load,REP:rep,NODES:nodes,FAIL:fail,OBSD+'':obsd,REPD+'':repd}
