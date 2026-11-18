import sys;sys.path.insert(0,__import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from edit import *
s=Stage(SCR+"stage4");IN=B+"🕸️graph/📥️inference/";SIC=IN+"🧭️script-inputs/"
s.edit(SIC+"💾️entries/🟨️.mjs",[
(''' const sameList=(left,right)=>left.length===right.length&&left.every((item,at)=>item===right[at]);''',''' const sameList=(left,right)=>left.length===right.length&&left.every((item,at)=>item===right[at]);
 const unionMembers=(digest,mapped)=>{const held=sharedMembers.get(digest);if(held===undefined){sharedMembers.set(digest,mapped);return mapped;}if(held===mapped||sameList(held,mapped))return held;const have=new Set(held),extra=mapped.filter(walk=>!have.has(walk));if(extra.length===0)return held;const merged=[...held,...extra];sharedMembers.set(digest,merged);return merged;};''',1),
('''const mapped=list.map(index=>table[index]),known=sharedMembers.get(digests[at]);if(known!==undefined&&sameList(known,mapped))return known;sharedMembers.set(digests[at],mapped);return mapped;});''','''return unionMembers(digests[at],list.map(index=>table[index]));});''',1),
])
s.edit(SIC+"🟨️.mjs",[
('''  const have = new Set(held), extra = members.filter(walk => !have.has(walk));
  if (extra.length === 0) return false;
  table.set(digest, [...held, ...extra]);
  return true;''','''  const wider = new Set(members);
  if (members.length >= held.length && held.every(walk => wider.has(walk))) { table.set(digest, members); return members.length > held.length; }
  const have = new Set(held), extra = members.filter(walk => !have.has(walk));
  if (extra.length === 0) return false;
  table.set(digest, [...held, ...extra]);
  return true;''',1),
])
s.edit(SIC+"🧪️tests/walk-members-merge.test.ts",[
(' expect(mergeWalkMembersV1(table,"e",second)).toBe(true);',' const wide=["w1","w2","w3","w4"];\n expect(mergeWalkMembersV1(table,"d",wide)).toBe(true);\n expect(table.get("d")).toBe(wide);\n expect(mergeWalkMembersV1(table,"e",second)).toBe(true);',1)])
s.edit(SIC+"💾️entries/🧪️tests/🟦️.ts",[
('const layer=(key:number)=>[{kind:"file",path:"/layer/"+key,fact:{size:key}}],members=(key:number)=>Array.from({length:walks},(_all,at)=>"walk:"+key+":"+at+":"+"x".repeat(40)),',
 'const layer=(key:number)=>[{kind:"file",path:"/layer/"+key,fact:{size:key}}],members=(key:number,entry=0)=>[...Array.from({length:walks},(_all,at)=>"walk:"+key+":"+at+":"+"x".repeat(40)),...entry%2===1?["extra:"+key]:[]],',1),
('sidecar:{facts:keys.map(fact),members:keys.map(members)}','sidecar:{facts:keys.map(fact),members:keys.map(key=>members(key,entry))}',1),
('  expect(held[0].sidecar.members[7]).toEqual(members(7));','  expect(held[2].sidecar.members[12]).toEqual(members(26,1));',1),

('  expect(held[0].sidecar.members[7]).toBe(held[1].sidecar.members[0]);','  expect(held[2].sidecar.members[12]).toBe(held[3].sidecar.members[5]);',1),
])
s.finish([SIC+"🧪️tests/walk-members-merge.test.ts",SIC+"💾️entries/🧪️tests/🟦️.ts",SIC+"🧪️tests/closure-store-limits.test.ts",IN+"🧪️tests/🪢️walk-questions/🟦️.ts",IN+"🧪️tests/🌊️layer-stream/🟦️.ts",IN+"🧪️tests/🧺️bundle-rows/🟦️.ts",IN+"🧪️tests/🧮️pass-verdicts/🟦️.ts",SIC+"🧪️tests/closure-answer-refresh.test.ts",SIC+"🧹️retention/🧪️tests/🟦️.ts"],"- Step 2d: loaded closure entries hold, per layer digest, ONE union list of walk members (measured on the real store: 300 entries held 68 711 distinct member arrays, 5.75 M references for 2 938 distinct walk strings, 233 KB per entry); the digest table takes the wider list as it is.")
