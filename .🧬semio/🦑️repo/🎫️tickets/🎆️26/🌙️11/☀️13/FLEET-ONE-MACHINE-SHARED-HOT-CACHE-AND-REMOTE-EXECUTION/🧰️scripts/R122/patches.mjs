const L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
export const OB=L+"👁️observation/🟨️.mjs",OBD=L+"👁️observation/🟨️.d.mts",SI=L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs",AS=L+"🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs",PN=L+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs",FM=L+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs",FMD=L+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.d.mts";
const edit=(from,to)=>text=>{const at=text.indexOf(from);if(at<0)throw Error("anchor missing: "+from.slice(0,80));if(text.indexOf(from,at+1)>=0)throw Error("anchor not unique: "+from.slice(0,80));return text.replace(from,()=>to);};
const between=(from,to,replacement)=>text=>{const a=text.indexOf(from);if(a<0)throw Error("start missing: "+from.slice(0,80));if(text.indexOf(from,a+1)>=0)throw Error("start not unique: "+from.slice(0,80));const b=text.indexOf(to,a+from.length);if(b<0)throw Error("end missing: "+to.slice(0,80));return text.slice(0,a)+replacement+text.slice(b);};
import{readFileSync}from"node:fs";
export const FMT=L+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🟦️.ts";
const NEWTEST="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R122/farm-main-test.new.ts";

const TD=L+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/",SCT="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R122/tests-new/";
export const TESTS=["🥇️first-child","🪆️captured","🌊️wave-tally","🏘️hub"];
const PTD=L+"🕸️graph/📥️inference/📦️project-nodes/🧪️tests/";
export const PTESTS=["⏳️verdict-wait","🪘️refs-file"];
const bt="`";
const GUARD="🚧️read-parent/🟨️.mjs";

const farmJobFailed=`/** 🛑️ The refusal of a job that no farm child could do after its tries: the parent does not infer it in their place. */
const jobFailedV1 = (project, reason, tries) => { const text = language => language === "de" ? ${bt}Farm-Kindprozesse konnten ${"$"}{project} in ${"$"}{tries} Versuchen nicht ableiten (${"$"}{reason}); der Elternprozess leitet es nicht ab${bt} : ${bt}farm children could not infer ${"$"}{project} in ${"$"}{tries} tries (${"$"}{reason}); the parent does not infer it${bt}; return Object.assign(new Error(text("en") + " / " + text("de")), { code: "graph.farm-job-failed", semioProject: project }); };
const EMPTY_ATTEMPTS_V1 = 5, QUEUE_TELL_MILLISECONDS_V1 = 60000;
`;

const farmDefs=`  if (hub.state === "idle" && (engage === true || immediate === true)) open();
  else if (hub.state !== "idle" && pools.some(pool => pool.ready)) say("reused", { projects });
  const askChild = job => warmThroughV1({
    pick: () => { const at = choosePoolV1(pools.map(pool => ({ ready: pool.ready, usable: pool.farm.usable, idle: pool.farm.idle, pending: pool.farm.pending, width: pool.farm.width }))); return at < 0 ? undefined : pools[at]; },
    run: async pool => { const work = pool.farm.run(job, { signal }); pool.work.add(work); try { const outcome = await work; if (outcome.overrun === undefined) return outcome; counts.overruns++; const error = new GraphFarmOverrunErrorV1({ ...outcome.overrun, project: job.name }, environment); return { failure: { name: error.name, message: error.message, code: error.code } }; } finally { pool.work.delete(work); } },
    hopeless: () => warmGaveUp ? "no child came before" : hub.state === "off" || hub.closing ? "farm-" + (hub.closing ? "closing" : "off") : signal?.aborted ? "aborted" : pools.some(pool => !pool.closing && pool.farm.usable) || hub.waiting !== undefined || hub.state === "waiting" && pools.length === 0 ? undefined : "no-usable-child",
    wait: milliseconds => readyWithin(milliseconds),
  });
  const exists = () => hub.state !== "off" && !hub.closing && !call.closed && signal?.aborted !== true && !(hub.unavailable && hub.waiting === undefined && !pools.some(pool => !pool.ready && !pool.closing && pool.farm.usable));
  const waitForChild = async name => {
    let told = -Infinity;
    for (;;) {
      if (hub.state === "off" || hub.closing || call.closed || signal?.aborted) return false;
      if (pools.some(pool => pool.ready && pool.farm.usable)) return true;
      const granting = hub.waiting !== undefined, booting = pools.some(pool => !pool.ready && !pool.closing && pool.farm.usable);
      if (!granting && !booting && hub.state !== "idle") { hub.unavailable = true; return false; }
      if (!counts.holding) { counts.holding = true; say("holding", { seconds: Math.round((booting ? policy.holdMilliseconds : policy.grantWaitMilliseconds) / 1000) }); }
      if (performance.now() - told >= QUEUE_TELL_MILLISECONDS_V1) { told = performance.now(); write(queuedJobTextV1(name, granting ? "the host capacity governor holds the first child" : booting ? "the first child is starting" : "no child is ready")); }
      await readyWithin(1000);
    }
  };
`;

const farmPrepare=`    async prepare(job) {
      if (hub.state === "idle") { counts.misses++; if (immediate === true || counts.misses >= policy.minimumProjects && counts.misses * 10000 >= processed() * policy.missBasisPoints) open(); if (immediate !== true) return false; }
      if (hub.state === "off" || hub.closing) return false;
      await refreshOwned();
      const note = {};
      for (let attempt = 0; ; attempt++) {
        if (!(await waitForChild(job.name))) return false;
        const at = choosePoolV1(pools.map(pool => ({ ready: pool.ready, usable: pool.farm.usable, idle: pool.farm.idle, pending: pool.farm.pending, width: pool.farm.width })));
        if (at < 0) continue;
        const pool = pools[at], work = dispatch(pool, job, note);
        pool.work.add(work);
        let answer;
        try { answer = await work; } finally { pool.work.delete(work); }
        if (answer !== AGAIN) return answer;
        if (hub.state === "off" || hub.closing) return false;
        const empty = note.reason === "nothing adopted";
        if (empty ? attempt >= EMPTY_ATTEMPTS_V1 : attempt >= policy.maximumAttempts) throw jobFailedV1(job.name, note.reason ?? "no reason", attempt + 1);
        if (empty) await readyWithin(2500);
      }
    },
`;

const farmObserve=`    exists() { return exists(); },
    async observe(job) {
      let failures = 0, lost = 0, told = -Infinity;
      for (;;) {
        warmGaveUp = false;
        const result = await askChild(job);
        if (result.reply !== undefined) { counts.workerCpuMilliseconds += result.reply.cpuMilliseconds ?? 0; return result.reply; }
        if (!exists()) return undefined;
        const why = String(result.why);
        if (why.startsWith("failure:") && ++failures > policy.maximumAttempts) throw jobFailedV1(job.name, why, failures);
        if (why.startsWith("lost:") && ++lost > EMPTY_ATTEMPTS_V1) throw jobFailedV1(job.name, why, lost);
        if (performance.now() - told >= QUEUE_TELL_MILLISECONDS_V1) { told = performance.now(); write(queuedJobTextV1(job.name, why)); }
        await readyWithin(1000);
      }
    },
    async warm(job) {
      const result = await askChild(job);
`;

export const all=[
 /* guard module: observation */
 {file:OB,apply:edit(`export async function observeProjectInferenceV1({cachePath,key,owner,reusable,stamps=[],prepare,fleet,deferOffer=false,verdict},compute){`,`export async function observeProjectInferenceV1({cachePath,key,owner,reusable,stamps=[],prepare,fleet,deferOffer=false,verdict,verdictFinal=false,prepareAttempts=1},compute){`)},
 {file:OB,apply:edit(`observeInferenceComputationV1({cachePath,key,reusable,prepare,fleet,deferOffer,verdict},async()=>{projectComputations`,`observeInferenceComputationV1({cachePath,key,reusable,prepare,fleet,deferOffer,verdict,verdictFinal,prepareAttempts},async()=>{projectComputations`)},
 {file:OB,apply:edit(`async function observeInferenceComputationV1({cachePath,key,reusable,prepare,fleet,deferOffer=false,verdict},compute,nestedProject) {`,`async function observeInferenceComputationV1({cachePath,key,reusable,prepare,fleet,deferOffer=false,verdict,verdictFinal=false,prepareAttempts=1},compute,nestedProject) {`)},
 {file:OB,apply:edit(`if(said?.verdict==="holds"&&said.headerDigest===headerDigest)candidate={value:record.value,header,headerDigest,adopted:true};else{`,`if(said?.verdict==="holds"&&said.headerDigest===headerDigest)candidate={value:record.value,header,headerDigest,adopted:true};else if(verdictFinal&&(said?.verdict==="stale"||said?.verdict==="other"||said?.verdict==="missing")){}else{`)},
 {file:OB,apply:edit(`if(!reused&&prepare!==undefined&&await prepare()){computedElsewhere=true;reused=await lookup();}`,`for(let attempt=0;!reused&&prepare!==undefined&&attempt<prepareAttempts&&await prepare();attempt++){computedElsewhere=true;reused=await lookup();}`)},
 /* script-inputs: guards and the claim of a derivation */
 {file:SI,apply:edit(`import { mkdirSync as vMkdirV1,`,`import { assertNotReadParentV1 } from "../../../👁️observation/${GUARD}";\nimport { mkdirSync as vMkdirV1,`)},
 {file:SI,apply:edit(`export async function deriveClosureFactV1(row, cache, pass = {}) {\n  cache.pass = pass;`,`export async function deriveClosureFactV1(row, cache, pass = {}) {\n  assertNotReadParentV1("walk", row?.path ?? row?.kind ?? "");\n  cache.pass = pass;`)},
 {file:SI,apply:edit(`export function resolutionConfigsV1(compiler, host, workspaceRoot, modules) {\n  const root = resolve(workspaceRoot)`,`export function resolutionConfigsV1(compiler, host, workspaceRoot, modules) {\n  assertNotReadParentV1("resolve", workspaceRoot);\n  const root = resolve(workspaceRoot)`)},
 {file:SI,apply:edit(`async function produceClosureEntryV1(workspaceRoot, recipe) {`,`/** 🪧️ The claim on the first derivation of one shared closure entry: the process that takes it derives the entry, the others wait for the stored entry and reuse it; undefined when the read has no verdict directory or no store. A claim is released when the derivation ends, so a later derivation of a stale entry is claimed anew. */
export async function claimDerivationV1(id, store, scripts) {
  const directory = closureVerdictScopeV1();
  if (directory === undefined || !store) return undefined;
  const file = join(directory, "claim-" + createHash("sha256").update(id).digest("hex").slice(0, 32));
  for (let tick = 0; tick < 1200; tick++) {
    let mine = false;
    try { verdictDirMadeV1(directory); vWriteV1(file, String(process.pid), { flag: "wx" }); mine = true; } catch {}
    if (mine) return { release: () => { try { vRmV1(file, { force: true }); } catch {} } };
    const stored = await store.load(id), entry = stored && { ...stored, layers: undefined, store, answers: new Map(), checked: null };
    if (entry && await closureEntryHoldsAsyncV1(entry, scripts)) return { entry };
    await new Promise(done => setTimeout(done, 100));
  }
  return undefined;
}
async function produceClosureEntryV1(workspaceRoot, recipe) {`)},
 {file:SI,apply:edit(`const store = closureStoreOfV1(workspaceRoot), derive = () => walkClosureRecipeV1(workspaceRoot, recipe, scripts).then(`,`const store = closureStoreOfV1(workspaceRoot), derive = () => { assertNotReadParentV1("derive", recipe?.fn ?? ""); return walkClosureRecipeV1(workspaceRoot, recipe, scripts); }, deriveAndKeep = () => derive().then(`)},
 {file:SI,apply:edit(`if (entry) closureCountsV1.stale++;
      const lane = store ? await closureFleetLaneV1(workspaceRoot) : undefined;
      if (!lane) return derive();`,`if (entry) closureCountsV1.stale++;
      const claim = await claimDerivationV1(id, store, scripts);
      if (claim?.entry) { closureCountsV1.loaded++; closureEntriesV1.set(id, claim.entry); return claim.entry; }
      try {
      const lane = store ? await closureFleetLaneV1(workspaceRoot) : undefined;
      if (!lane) return await deriveAndKeep();`)},
 {file:SI,apply:edit(`let derived; try { derived = await derive(); } catch (error) { await taken?.hold?.release().catch(() => undefined); throw error; }`,`let derived; try { derived = await deriveAndKeep(); } catch (error) { await taken?.hold?.release().catch(() => undefined); throw error; }`)},
 {file:SI,apply:edit(`else await taken?.hold?.release().catch(() => undefined);
      return derived; }).then(entry => entry,`,`else await taken?.hold?.release().catch(() => undefined);
      return derived;
      } finally { claim?.release?.(); } }).then(entry => entry,`)},
 /* project-assembly */
 {file:AS,apply:edit(`export async function resolveClosureRefsV1(project, workspaceRoot, scripts, sourceRepresentation = "hash", log = undefined) {`,`export async function resolveClosureRefsV1(project, workspaceRoot, scripts, sourceRepresentation = "hash", log = undefined) {\n  assertNotReadParentV1("refs", project?.name ?? "");`)},
 {file:AS,apply:edit(`import { createHash } from "node:crypto";`,`import { createHash } from "node:crypto";\nimport { assertNotReadParentV1 } from "../../../../🏗️build/👁️observation/${GUARD}";`)},
 /* farm */
 {file:FM,apply:edit(`import { warmThroughV1 } from "../🔱️warm/🟨️.mjs";`,`import { warmThroughV1 } from "../🔱️warm/🟨️.mjs";\nimport { queuedJobTextV1 } from "../../../../../🏗️build/👁️observation/${GUARD}";`)},
 {file:FM,apply:edit(`const ORIGIN = physicalSourceOriginV1(`,farmJobFailed+`const ORIGIN = physicalSourceOriginV1(`)},
 {file:FM,apply:edit(`engage = false, releaseCaches = undefined,`,`engage = false, immediate = false, releaseCaches = undefined,`)},
 {file:FM,apply:edit(`  if (hub.state === "idle" && engage === true) open();\n  else if (hub.state !== "idle" && pools.some(pool => pool.ready)) say("reused", { projects });\n`,farmDefs)},
 {file:FM,apply:edit(`const dispatch = async (pool, job, retry) => {`,`const dispatch = async (pool, job, note = {}) => {`)},
 {file:FM,apply:edit(`reason: outcome.failure.message.slice(0, 200) }); return retry ? AGAIN : false; }`,`reason: outcome.failure.message.slice(0, 200) }); note.reason = outcome.failure.message.slice(0, 200); return AGAIN; }`)},
 {file:FM,apply:edit(`(counts.lostReasons[outcome.lost] ?? 0) + 1; return false; }`,`(counts.lostReasons[outcome.lost] ?? 0) + 1; note.reason = "lost: " + outcome.lost; return AGAIN; }`)},
 {file:FM,apply:edit(`if (adoption.published + adoption.present === 0) return false;`,`if (adoption.published + adoption.present === 0) { note.reason = "nothing adopted"; return AGAIN; }`)},
 {file:FM,apply:between(`    async observe(job) { return this.warm(job); },\n`,`      if (result.reply === undefined) { if (result.why.startsWith`,farmObserve)},
 {file:FM,apply:between(`    async prepare(job) {\n      if (hub.state === "idle") {`,`    async close() {`,farmPrepare)},
 /* project-nodes */
 {file:PN,apply:edit(`import {refuseBuildV1} from "../../../📖️texts/🟦️.ts";`,`import {refuseBuildV1} from "../../../📖️texts/🟦️.ts";\nimport { assertNotReadParentV1, markReadParentV1 } from "../../../../🏗️build/👁️observation/${GUARD}";`)},
 {file:PN,apply:edit(`{ name, root, json, native, pipeline, selectedTargets, selectedInputs } = job, began = performance.now();\n  try {`,`{ name, root, json, native, pipeline, selectedTargets, selectedInputs } = job, began = performance.now();\n  assertNotReadParentV1("infer", name);\n  try {`)},
 {file:PN,apply:edit(`observeProjectInferenceV1({ cachePath, key: projectRecordKeyV1(job.head, job.root, job.json, undefined, job.selectedInputs), owner: job.root, stamps: projectStampPathsV1({ native: job.native }), reusable: projectRecordReusableV1 }, () => inferProjectBodyV1(environment, job, undefined)))); }`,`observeProjectInferenceV1({ cachePath, key: projectRecordKeyV1(job.head, job.root, job.json, undefined, job.selectedInputs), owner: job.root, stamps: projectStampPathsV1({ native: job.native }), reusable: projectRecordReusableV1 }, () => inferProjectBodyV1(environment, job, undefined))), { verdicts: job.verdicts }); }`)},
 {file:PN,apply:edit(`catch { refs = undefined; } } return said; }, { verdicts: job.verdicts });`,`catch (error) { refs = refsFailureV1(error); } } return said; }, { verdicts: job.verdicts });`)},
 {file:PN,apply:edit(`/** 🪘️ What the parent of a read did with the closure references of its owners:`,`/** 🪘️ What a farm child answers instead of a reference when it could not resolve the closure references of an owner: a source that does not parse (the parent withholds the owner from it) or any other failure (named, the parent asks again). */
export function refsFailureV1(error) {
  if (error?.code === "BUILD_SOURCE_UNPARSEABLE") return { unparseable: { source: String(error.source), sha256: String(error.sha256), message: String(error.message).slice(0, 2048) } };
  return { failed: String(error?.message ?? error).replace(/\\s+/gu, " ").slice(0, 300) };
}

/** 🪘️ Asks farm children for the closure references of one owner until one answers a reference or a withheld source, up to \`attempts\` times; undefined when no farm exists any more (the parent then resolves them, as it always did without a farm), a refusal naming the owner and the last failure when every try failed. @param {{ observe: (job: object) => Promise<object | undefined>, exists: () => boolean }} farm */
export async function askOwnerRefsV1({ farm, job, headerDigest, attempts = 3 }) {
  let last = "no answer";
  for (let attempt = 0; attempt < attempts; attempt++) {
    if (!farm.exists()) return undefined;
    const reply = await farm.observe({ ...job, observe: true, refs: true });
    if (reply === undefined) return undefined;
    if (reply.verdict === "holds" && reply.headerDigest === headerDigest && reply.refs !== undefined && reply.refs.failed === undefined) return { refs: reply.refs, headerDigest };
    last = reply.refs?.failed ?? "verdict " + reply.verdict;
    if (reply.verdict === "stale" || reply.verdict === "other" || reply.verdict === "missing") break;
  }
  throw Object.assign(new Error("farm children could not resolve the closure references of " + job.name + " (" + last + "); the parent does not resolve them / Farm-Kindprozesse konnten die Closure-Referenzen von " + job.name + " nicht auflösen (" + last + "); der Elternprozess löst sie nicht auf"), { code: "graph.farm-job-failed", semioProject: job.root });
}

/** 🪘️ What the parent of a read did with the closure references of its owners:`)},

 {file:OBD,apply:edit(`fleet?:string;verdict?:(asked:{readonly headerDigest:string})=>PromiseLike<{readonly verdict?:string;readonly headerDigest?:string}|undefined>}>,compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>;`,`fleet?:string;verdict?:(asked:{readonly headerDigest:string})=>PromiseLike<{readonly verdict?:string;readonly headerDigest?:string}|undefined>;verdictFinal?:boolean;prepareAttempts?:number}>,compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>;`)},
 {file:FMD,apply:edit(`prepare(job:ProjectFarmJobV1):Promise<boolean>;engage():void;`,`prepare(job:ProjectFarmJobV1):Promise<boolean>;exists():boolean;engage():void;`)},
 {file:FMD,apply:edit(`maximumWorkers?:number;engage?:boolean;`,`maximumWorkers?:number;engage?:boolean;immediate?:boolean;`)},
 {file:PN,apply:edit(`let offered;const inferStartedAt=Date.now();stageV1(name,STAGE_INFER_V1);`,`let offered,seenHeader;const observeJob=farm===undefined?undefined:{name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey,cachePath,observe:true,refs:late&&options.rowLog!==undefined&&closureVerdictScopeV1()!==undefined,closureStore:closureStoreDirectoryV1(workspaceRoot),verdicts:closureVerdictScopeV1()};const inferStartedAt=Date.now();stageV1(name,STAGE_INFER_V1);`)},
 {file:PN,apply:edit(`farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey})`,`farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey,verdicts:closureVerdictScopeV1()})`)},
 {file:PN,apply:between(`verdict:away.take===true&&farm?.width?.()>0?`,`},()=>{missed.add(name);return inferProjectBodyV1(projectEnvironment`,`verdict:observeJob===undefined?undefined:({headerDigest})=>{seenHeader=headerDigest;return farm.observe(observeJob).then(reply=>{if(reply?.verdict==="holds"&&reply.headerDigest===headerDigest&&reply.refs!==undefined&&reply.refs.failed===undefined)offered={refs:reply.refs,headerDigest};return reply;});},verdictFinal:farm!==undefined,prepareAttempts:farm!==undefined?3:1`)},
 {file:PN,apply:edit(`try{const adopted=offered===undefined?undefined:adoptOwnerRefsV1(offered.refs,offered.headerDigest,workspaceRoot,scripts,options.rowLog);if(adopted!==undefined){ownerRefsCountsV1.adopted++;return adopted;}ownerRefsCountsV1.resolved++;`,`try{let adopted;for(let round=0;round<2&&adopted===undefined;round++){let from=round===0?offered:undefined;if(from===undefined&&observeJob?.refs===true&&farm.exists()){const headerDigest=seenHeader??recordHeaderDigestV1(cachePath);from=headerDigest===undefined?undefined:await askOwnerRefsV1({farm,job:observeJob,headerDigest});}if(from===undefined)break;if(from.refs.unparseable!==undefined){const detail=from.refs.unparseable;throw Object.assign(Error(detail.message),{code:"BUILD_SOURCE_UNPARSEABLE",source:detail.source,sha256:detail.sha256});}adopted=adoptOwnerRefsV1(from.refs,from.headerDigest,workspaceRoot,scripts,options.rowLog);}if(adopted!==undefined){ownerRefsCountsV1.adopted++;return adopted;}ownerRefsCountsV1.resolved++;`)},
 {file:PN,apply:edit(`export function refsFailureV1(error) {`,`export function recordHeaderDigestV1(cachePath) {
  try { const text = refsBrotliV1(rawReadFileSync(cachePath)).toString("utf8"), cut = text.indexOf("\\n"); return cut < 0 ? undefined : createHash("sha256").update(text.slice(0, cut)).digest("hex"); } catch { return undefined; }
}

/** 🪘️ What a farm child answers instead of a reference (second form). */
export function refsFailureV1(error) {`)},
 {file:PN,apply:edit(`engage:options?.coldInference===true||wantedRoots!==undefined,releaseCaches:`,`engage:options?.coldInference===true||wantedRoots!==undefined,immediate:true,releaseCaches:`)},
 {file:PN,apply:edit(`if(late){const since=performance.now();let shown;try{await primeSharedClosuresV1(workspaceRoot,scripts,{signal:context.phaseControl?.signal,progress:async({done,total})=>{active.delete(shown);shown="shared closures "+done+"/"+total;active.set(shown,{since});await publishRunning();}});}finally{active.delete(shown);}}`,`if(late){const since=performance.now(),storeDirectory=closureStoreDirectoryV1(workspaceRoot);let shown;try{if(storeDirectory!==undefined&&farm!==undefined){shown="shared closures in a child";active.set(shown,{since});await publishRunning();await primeSharedClosuresInChildV1({workspaceRoot,storeDirectory,signal:context.phaseControl?.signal});}else await primeSharedClosuresV1(workspaceRoot,scripts,{signal:context.phaseControl?.signal,progress:async({done,total})=>{active.delete(shown);shown="shared closures "+done+"/"+total;active.set(shown,{since});await publishRunning();}});}finally{active.delete(shown);}}`)},
 {file:PN,apply:edit(`  const workers=Array.from({length:limit},()=>loop(lightEnd)),grow=`,`  const unmarkParent=farm===undefined?undefined:markReadParentV1(()=>farm.exists());\n  const workers=Array.from({length:limit},()=>loop(lightEnd)),grow=`)},
 {file:PN,apply:edit(`governor?.dispose();await farm?.close();`,`governor?.dispose();unmarkParent?.();await farm?.close();`)},
 {file:OB,apply:edit(`export async function observeInference({ cachePath, key, reusable, verdict }, compute) {
  requireCurrentObservationV1();requireInferenceReuseV1(reusable);
  return observeInferenceComputationV1({cachePath,key,reusable,verdict},compute,false);`,`export async function observeInference({ cachePath, key, reusable, verdict, verdictFinal = false, prepare, prepareAttempts = 1 }, compute) {
  requireCurrentObservationV1();requireInferenceReuseV1(reusable);
  return observeInferenceComputationV1({cachePath,key,reusable,verdict,verdictFinal,prepare,prepareAttempts},compute,false);`)},
 {file:OBD,apply:edit(`export function observeInference<T>(options:Readonly<{cachePath:string;key:unknown;reusable?:(value:unknown)=>boolean|PromiseLike<boolean>;verdict?:`,`export function observeInference<T>(options:Readonly<{cachePath:string;key:unknown;verdictFinal?:boolean;prepare?:()=>boolean|PromiseLike<boolean>;prepareAttempts?:number;reusable?:(value:unknown)=>boolean|PromiseLike<boolean>;verdict?:`)},
 {file:FMT,apply:()=>readFileSync(NEWTEST,"utf8")},
 ...PTESTS.map(name=>({file:PTD+name+"/🟦️.ts",apply:()=>readFileSync(SCT+"pn-"+name+".ts","utf8")})),
 ...TESTS.map(name=>({file:TD+name+"/🟦️.ts",apply:()=>readFileSync(SCT+name+".ts","utf8")})),
];
export const apply=(file,text,list=all)=>{for(const item of list)if(item.file===file)text=item.apply(text);return text;};
