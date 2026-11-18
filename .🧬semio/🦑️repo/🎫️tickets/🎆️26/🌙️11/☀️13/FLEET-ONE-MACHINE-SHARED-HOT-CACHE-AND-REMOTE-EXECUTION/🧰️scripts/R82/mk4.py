import sys,json;sys.path.insert(0,__import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from edit import *
IN=B+"🕸️graph/📥️inference/";PN=IN+"📦️project-nodes/";TK=PN+"📥️take/"
s=Stage(SCR+"stage4")
s.edit(TK+"🟨️.mjs",[
('/** 📒️ What a reader knows about its owners','''/** 🛤️ The two lanes of a take pass: the owners whose record is stored here (`local`, decided by the CPU of this process) and the others (`remote`, decided by the fleet cache and the network), which run beside each other so that an owner the fleet holds does not wait behind thousands of local looks. Every name is in exactly one lane, in its order; an empty lane is left out. */
export function takeLanesV1(names, isPresent) {
  const local = [], remote = [];
  for (const name of names) (isPresent(name) ? local : remote).push(name);
  return [local, remote].filter(lane => lane.length > 0);
}

/** 📒️ What a reader knows about its owners''',1)])
s.edit(TK+"🟨️.d.mts",[("export function takePassV1(","export function takeLanesV1(names: readonly string[], isPresent: (name: string) => boolean): string[][];\nexport function takePassV1(",1)])
s.edit(PN+"🟨️.mjs",[
('import { TAKE_POLICY_V1, createTakeLedgerV1, takePassV1, takePollV1 } from "./📥️take/🟨️.mjs";','import { TAKE_POLICY_V1, createTakeLedgerV1, takeLanesV1, takePassV1, takePollV1 } from "./📥️take/🟨️.mjs";',1),
('  const inferProject=async({result,name,json},away=work=>work())=>{','  const lookPhases={authority:0,observe:0,refs:0,rows:0},lap=(key,milliseconds)=>{lookPhases[key]+=milliseconds;};\n  const inferProject=async({result,name,json},away=work=>work())=>{',1),
('const authorityMilliseconds=performance.now()-started;clock.add(name,"authority",authorityMilliseconds);','const authorityMilliseconds=performance.now()-started;lap("authority",authorityMilliseconds);clock.add(name,"authority",authorityMilliseconds);',1),
('clock.add(name,"observe",Math.max(0,Date.now()-inferStartedAt-clock.get(name,"farm")-clock.get(name,"inline")));','lap("observe",Date.now()-inferStartedAt);clock.add(name,"observe",Math.max(0,Date.now()-inferStartedAt-clock.get(name,"farm")-clock.get(name,"inline")));',1),
('finally{clock.add(name,"refs",Date.now()-startedAt);','finally{lap("refs",Date.now()-startedAt);clock.add(name,"refs",Date.now()-startedAt);',1),
('{const at=performance.now();options?.onOwnerObservations?.(name,rows,derived);clock.add(name,"observations",performance.now()-at);}','{const at=performance.now();options?.onOwnerObservations?.(name,rows,derived);lap("rows",performance.now()-at);clock.add(name,"observations",performance.now()-at);}',1),
('upTo5000:looks.buckets[2],over5000:looks.buckets[3]}','upTo5000:looks.buckets[2],over5000:looks.buckets[3],phases:Object.fromEntries(Object.entries(lookPhases).map(([key,value])=>[key,Math.round(value)]))}',1),
('      await takePassV1({names,attempt:name=>look(byName.get(name)),signal,mark:(name,outcome)=>{ledger.mark(name,outcome);if(outcome==="present"||outcome==="fetched")finished++;}});','      await Promise.all(takeLanesV1(names,name=>present.has(name)).map(lane=>takePassV1({names:lane,attempt:name=>look(byName.get(name)),signal,mark:(name,outcome)=>{ledger.mark(name,outcome);if(outcome==="present"||outcome==="fetched")finished++;}})));',1),
])
tk=open(R+"/"+TK+"🧪️tests/🟦️.ts",encoding="utf8").read()
assert "takeLanesV1" not in tk
imp='import{createTakeLedgerV1,takePassV1,takePollV1,takePollWaitV1,TAKE_POLICY_V1}'
import re
m=re.search(r'import\s*\{([^}]*)\}\s*from\s*"\.\./🟨️\.mjs";?',tk)
print("take test import:",m.group(0)[:200] if m else None)
if m:
    new=tk.replace(m.group(0),m.group(0).replace("takePassV1","takeLanesV1, takePassV1",1),1)
    new+='''
test("the lanes of a take pass hold every owner once, local records first in their own lane, and leave an empty lane out",()=>{
 const names=Array.from({length:12},(_,index)=>"o"+index),present=new Set(["o0","o3","o4","o9"]);
 const lanes=takeLanesV1(names,name=>present.has(name));
 expect(lanes).toEqual([["o0","o3","o4","o9"],["o1","o2","o5","o6","o7","o8","o10","o11"]]);
 expect(lanes.flat().sort()).toEqual([...names].sort());
 expect(takeLanesV1(names,()=>true)).toEqual([names]);expect(takeLanesV1(names,()=>false)).toEqual([names]);expect(takeLanesV1([],()=>true)).toEqual([]);
});
'''
    s.new(TK+"🧪️tests/🟦️.ts",new)
s.finish([TK+"🧪️tests/🟦️.ts",PN+"🧪️tests/📥️take/🟦️.ts",PN+"🧪️tests/📈️inferred/🟦️.ts",PN+"🧪️tests/🐌️slow-last/🟦️.ts"],"- Step 3 (R82 c): a take pass runs two lanes beside each other (`takeLanesV1`): the owners with a record stored here and the others, so an owner the fleet holds is asked at once instead of after thousands of local looks (`fetched` was 0 on every worker because the fleet was only reached after the local pass); the worker report shows where a look spends its time (`owners.looks.phases`: authority, observe, refs, rows, milliseconds summed over looks).")
