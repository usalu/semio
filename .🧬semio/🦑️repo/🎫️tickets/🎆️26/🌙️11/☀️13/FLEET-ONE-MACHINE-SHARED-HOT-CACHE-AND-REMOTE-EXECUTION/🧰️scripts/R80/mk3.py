import sys;sys.path.insert(0,__import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from edit import *
s=Stage(SCR+"stage3");IN=B+"🕸️graph/📥️inference/";SIC=IN+"🧭️script-inputs/";SR=B+"🧠️memory/📋️slice-report/"
s.edit(SIC+"🟨️.mjs",[
('''  if (members) { let walks = walkMembersByDigestV1.get(digest); if (!walks) walkMembersByDigestV1.set(digest, walks = new Set()); for (const walk of members) walks.add(walk); }
});''','''  if (members) mergeWalkMembersV1(walkMembersByDigestV1, digest, [...members]);
});''',1),
('''    if (members) { let walks = walkMembersByDigestV1.get(digest); if (!walks) walkMembersByDigestV1.set(digest, walks = new Set()); for (const walk of members) walks.add(walk); }
    return { key: digest''','''    if (members && (!seededMembersV1.has(members) || !walkMembersByDigestV1.has(digest))) { seededMembersV1.add(members); mergeWalkMembersV1(walkMembersByDigestV1, digest, members); }
    return { key: digest''',1),
('/** 🌱️ Hands the walk facts and members of a derived entry','''const seededMembersV1 = new WeakSet();
/** 🧬️ Adds the walk members of one stored layer to the table by layer digest: the first list is held as it is (the instance the loaded entries share), a different list adds the members the table lacks; true when the table changed. A list that was merged once is never merged again (the caller remembers it), so a resolution does not walk every member of every layer of an entry again. */
export function mergeWalkMembersV1(table, digest, members) {
  const held = table.get(digest);
  if (held === undefined) { table.set(digest, members); return true; }
  if (held === members) return false;
  const have = new Set(held), extra = members.filter(walk => !have.has(walk));
  if (extra.length === 0) return false;
  table.set(digest, [...held, ...extra]);
  return true;
}
/** 🌱️ Hands the walk facts and members of a derived entry''',1),
('if (!layer.members || ![...layer.members].some(question => walked.has(question))) addLayer(layer);','if (!layer.members || !someMemberV1(layer.members, question => walked.has(question))) addLayer(layer);',1),
('const seededMembersV1 = new WeakSet();','const seededMembersV1 = new WeakSet(), someMemberV1 = (members, test) => { for (const member of members) if (test(member)) return true; return false; };',1),
('{ name: "closure:walkMembers (real)", ...spread(walkMembersByDigestV1, set => [...set]) }','{ name: "closure:walkMembers (real)", ...spread(walkMembersByDigestV1, list => [...list]) }',1),
])
s.edit(SIC+"🟨️.d.mts",[('export function closureCollectionsV1()','export function mergeWalkMembersV1(table: { get(key: string): string[] | undefined; set(key: string, value: string[]): unknown }, digest: string, members: string[]): boolean;\nexport function closureCollectionsV1()',1)])
s.new(SIC+"🧪️tests/walk-members-merge.test.ts",'''import{expect,test}from"bun:test";
import{mergeWalkMembersV1}from"../🟨️.mjs";

/** 🧬️ The walk members of a stored layer are held once per digest as the instance the loaded entries share; a different list only adds what the table lacks. */
test("the first member list of a digest is held as it is, the same list is never merged again and a different list adds only the missing members",()=>{
 const table=new Map<string,string[]>(),first=["w1","w2"],second=["w2","w3"];
 expect(mergeWalkMembersV1(table,"d",first)).toBe(true);
 expect(table.get("d")).toBe(first);
 expect(mergeWalkMembersV1(table,"d",first)).toBe(false);
 expect(mergeWalkMembersV1(table,"d",["w1"])).toBe(false);
 expect(mergeWalkMembersV1(table,"d",second)).toBe(true);
 expect(table.get("d")).toEqual(["w1","w2","w3"]);
 expect(mergeWalkMembersV1(table,"e",second)).toBe(true);
 expect(table.get("e")).toBe(second);
});
''')
s.edit(SR+"🟨️.mjs",[
(' const taken=[];let previous=0,lastCollect=-Infinity;',' const taken=[];let previous=0;',1),
('''const collect=()=>{
  const used=process.memoryUsage().heapUsed/MEGABYTE,now=performance.now();
  if(used<P.collectMinimumHeapMegabytes||now-lastCollect<P.collectEveryMilliseconds)return;
  lastCollect=now;const before=process.memoryUsage();
  try{globalThis.Bun?.gc?.(true);}catch{}
  const after=process.memoryUsage();sources.set("afterCollect",()=>({beforeHeapUsed:megabytes(before.heapUsed),beforeExternal:megabytes(before.external),heapUsed:megabytes(after.heapUsed),external:megabytes(after.external),seconds:Math.round((performance.now()-now)/100)/10}));
 };
 const timer=setInterval(()=>{probe();collect();''','''const timer=setInterval(()=>{probe();''',1),
])
s.edit(SR+"🧬️schema/🔣️.json",[('"collectionSample": 16,\n        "collectMinimumHeapMegabytes": 1000,\n        "collectEveryMilliseconds": 120000,','"collectionSample": 16,',1)])
s.finish([SIC+"🧪️tests/walk-members-merge.test.ts",SR+"🧪️tests/🟦️.ts",SIC+"🧪️tests/closure-store-limits.test.ts",SIC+"💾️entries/🧪️tests/🟦️.ts",IN+"🧪️tests/🪢️walk-questions/🟦️.ts",IN+"🧪️tests/🌊️layer-stream/🟦️.ts",IN+"🧪️tests/🧺️bundle-rows/🟦️.ts",IN+"🧪️tests/🧮️pass-verdicts/🟦️.ts",SIC+"🧪️tests/closure-answer-refresh.test.ts"],"- Step 2b: the walk members of a stored layer are held once per digest as the shared array (before: a Set per digest, 40 bytes per member, and every resolution of an entry added every member of its 274 layers again); the diagnostic collection of step 1b is removed (it blocked a worker for 65 to 94 s).")
open(s.S+"/before.txt","w",encoding="utf8").write(SIC+"🧪️tests/walk-members-merge.test.ts\n")
