import sys;sys.path.insert(0,__import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from edit import *
s=Stage(SCR+"stage5");IN=B+"🕸️graph/📥️inference/";SIC=IN+"🧭️script-inputs/"
s.edit(SIC+"💾️entries/🟨️.mjs",[
('const merged=[...held,...extra];sharedMembers.set(digest,merged);return merged;};','held.push(...extra);return held;};',1),
])
s.edit(SIC+"💾️entries/🧪️tests/🟦️.ts",[
('  expect(held[2].sidecar.members[12]).toBe(held[3].sidecar.members[5]);','  expect(held[2].sidecar.members[12]).toBe(held[3].sidecar.members[5]);\n  expect(held[0].sidecar.members[26]).toBe(held[3].sidecar.members[5]);\n  expect(new Set(held.flatMap(item=>item.sidecar.members)).size).toBeLessThanOrEqual(pool);',1),
])
s.finish([SIC+"💾️entries/🧪️tests/🟦️.ts",SIC+"🧪️tests/walk-members-merge.test.ts",SIC+"🧪️tests/closure-store-limits.test.ts",IN+"🧪️tests/🪢️walk-questions/🟦️.ts",IN+"🧪️tests/🌊️layer-stream/🟦️.ts",IN+"🧪️tests/🧺️bundle-rows/🟦️.ts",IN+"🧪️tests/🧮️pass-verdicts/🟦️.ts",SIC+"🧪️tests/closure-answer-refresh.test.ts"],"- Step 2e: the walk members of a layer digest are ONE shared array per store, extended in place with the members a later entry brings (before: a new wider copy per variant, 30 M references for 300 real entries).")
