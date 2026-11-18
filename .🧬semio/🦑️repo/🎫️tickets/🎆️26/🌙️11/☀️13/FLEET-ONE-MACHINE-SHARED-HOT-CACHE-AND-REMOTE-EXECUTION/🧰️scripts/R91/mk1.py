#!/usr/bin/env python3
import os,sys,shutil
R="/Users/ueli/Documents/semio"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";INF=B+"🕸️graph/📥️inference/"
SI=INF+"🧭️script-inputs/";PN=INF+"📦️project-nodes/";TK=PN+"📥️take/"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R91/stage1"
shutil.rmtree(S,ignore_errors=True);os.makedirs(S)
rows=[];n=[0]
def edit(rel,fn):
    live=open(R+"/"+rel,encoding="utf8").read();new=fn(live)
    i=str(n[0]);n[0]+=1
    open(S+"/"+i+".base","w",encoding="utf8").write(live);open(S+"/"+i+".new","w",encoding="utf8").write(new);rows.append((i,rel,"edit"))
def create(rel,text):
    i=str(n[0]);n[0]+=1
    open(S+"/"+i+".new","w",encoding="utf8").write(text);rows.append((i,rel,"new"))
def rep(text,old,new,count=1):
    assert text.count(old)==count,(old[:80],text.count(old));return text.replace(old,new)

def si(t):
    t=rep(t,'import { createBoundedCacheV1 } from "../../../🧊️bounded/🟨️.mjs";','import { AsyncLocalStorage } from "node:async_hooks";\nimport { createBoundedCacheV1 } from "../../../🧊️bounded/🟨️.mjs";')
    t=rep(t,'/** ♻️ The entry of one recipe from the cache, a flight in progress, or a new detached computation;','''const closureTakeV1 = new AsyncLocalStorage();

/** 🪤️ Runs a look at a stored owner record (the take pass) so that no closure entry is derived inside it: an entry that is not stored or no longer holds ends the look with `BUILD_TAKE_MISS` (the owner is then inferred by the governed owner loop, with its own memory limit, one hand-off to the fleet and the plan guard), instead of the ungoverned walk of hundreds of recipes in the parent that grew the slice worker of a 16 GB machine by gigabytes in seconds.
 * @see ./🧪️tests/🪤️take-no-derive */
export function runClosureTakeV1(body) { return closureTakeV1.run(true, body); }

/** 📖️ The stored entry of one recipe for a take: loaded and checked like a derivation's first step, never derived. */
async function takenClosureEntryV1(workspaceRoot, id) {
  const scripts = closureScriptsV1.get(workspaceRoot), store = closureStoreOfV1(workspaceRoot);
  if (!scripts) throw refuseBuildV1("missing", "closure.scripts-unbound", { workspace: String(workspaceRoot) });
  const stored = store ? await store.load(id) : undefined, entry = stored && { ...stored, layers: undefined, store, answers: new Map(), checked: null };
  if (entry && await closureEntryHoldsAsyncV1(entry, scripts)) { closureCountsV1.loaded++; closureEntriesV1.set(id, entry); return entry; }
  if (entry) closureCountsV1.stale++;
  throw Object.assign(new Error("A closure entry is not stored here or no longer holds: the owner is inferred by the owner loop, not taken"), { code: "BUILD_TAKE_MISS" });
}

/** ♻️ The entry of one recipe from the cache, a flight in progress, or a new detached computation;''')
    t=rep(t,'  let flight = closureInflightV1.get(id);\n  if (!flight) {\n    const scripts = closureScriptsV1.get(workspaceRoot);','  if (closureTakeV1.getStore() === true && !closureInflightV1.has(id)) return takenClosureEntryV1(workspaceRoot, id);\n  let flight = closureInflightV1.get(id);\n  if (!flight) {\n    const scripts = closureScriptsV1.get(workspaceRoot);')
    return t
edit(SI+"🟨️.mjs",si)
def sid(t):
    return rep(t,'export function releaseAfterTakeLookV1(scripts: ScriptInputsValueV1 | undefined,','/** 🪤️ Runs a take look in which no closure entry is derived: a missing or stale entry ends it with `BUILD_TAKE_MISS`. */\nexport function runClosureTakeV1<T>(body: () => Promise<T>): Promise<T>;\nexport function releaseAfterTakeLookV1(scripts: ScriptInputsValueV1 | undefined,')
edit(SI+"🟨️.d.mts",sid)

def pn(t):
    t=rep(t,'let releaseAfterTakeLookV1, withClosurePassV1,','let runClosureTakeV1, releaseAfterTakeLookV1, withClosurePassV1,')
    t=rep(t,'({ releaseAfterTakeLookV1, withClosurePassV1,','({ runClosureTakeV1, releaseAfterTakeLookV1, withClosurePassV1,')
    t=rep(t,'try{await lookProject(job,takeAway);}catch(error){if(error?.code==="BUILD_TAKE_MISS")','try{await runClosureTakeV1(()=>lookProject(job,takeAway));}catch(error){if(error?.code==="BUILD_TAKE_MISS")')
    t=rep(t,'const looks={count:0,milliseconds:0,maximum:0,buckets:[0,0,0,0]};','const looks={count:0,milliseconds:0,maximum:0,buckets:[0,0,0,0]},lookActive=new Map(),lookSlowest=[];')
    t=rep(t,'const lookedAt=performance.now();\n    const work=(async()=>{','const lookedAt=performance.now();lookActive.set(job.name,lookedAt);\n    const work=(async()=>{')
    t=rep(t,'finally{const took=performance.now()-lookedAt;looks.count++;','finally{const took=performance.now()-lookedAt;lookActive.delete(job.name);lookSlowest.push({name:String(job.name).slice(0,80),milliseconds:Math.round(took)});lookSlowest.sort((a,b)=>b.milliseconds-a.milliseconds);lookSlowest.length=Math.min(lookSlowest.length,5);looks.count++;')
    t=rep(t,'over5000:looks.buckets[3],phases:','over5000:looks.buckets[3],active:[...lookActive].slice(0,6).map(([name,at])=>({name:String(name).slice(0,80),seconds:Math.round((performance.now()-at)/1000)})),slowest:lookSlowest,phases:')
    t=rep(t,'orderOwnerJobsV1(jobs,{alone,slow:new Set(enforcement.slow?.(jobs.map(job=>job.name))??[]),rank,present});','const slowNames=new Set(enforcement.slow?.(jobs.map(job=>job.name))??[]);orderOwnerJobsV1(jobs,{alone,slow:slowNames,rank,present});')
    old='await Promise.all(takeLanesV1(names,name=>present.has(name)).map(lane=>takePassV1({names:lane,concurrency:takeConcurrencyV1(graphCutConcurrencyV1(process.env)),attempt:takeAdmittedV1(name=>look(byName.get(name)),footprintGateV1,signal),signal,mark:(name,outcome)=>{ledger.mark(name,outcome);if(outcome==="present"||outcome==="fetched")finished++;}})));'
    new='const split=takeSplitV1(names,name=>alone.has(name)||slowNames.has(name)),pass=(lane,concurrency)=>takePassV1({names:lane,concurrency,attempt:takeAdmittedV1(name=>look(byName.get(name)),footprintGateV1,signal),signal,mark:(name,outcome)=>{ledger.mark(name,outcome);if(outcome==="present"||outcome==="fetched")finished++;}});\n      await Promise.all(takeLanesV1(split.light,name=>present.has(name)).map(lane=>pass(lane,takeConcurrencyV1(graphCutConcurrencyV1(process.env)))));\n      await pass(split.heavy,1);'
    t=rep(t,old,new)
    t=rep(t,'takeConcurrencyV1, takeLanesV1, takePassV1, takePollV1 } from "./📥️take/🟨️.mjs";','takeConcurrencyV1, takeLanesV1, takePassV1, takePollV1, takeSplitV1 } from "./📥️take/🟨️.mjs";')
    return t
edit(PN+"🟨️.mjs",pn)
exec(open(os.path.dirname(os.path.abspath(__file__))+'/mk1b.py',encoding='utf8').read())
