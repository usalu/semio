import sys,json;sys.path.insert(0,__import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from edit import *
H=__import__("os").path.dirname(__import__("os").path.abspath(__file__))+"/"
rd=lambda n:open(H+n,encoding="utf8").read()
IN=B+"🕸️graph/📥️inference/";SI=IN+"🧭️script-inputs/";SCH=IN+"🧬️schema/🔣️.json"
s=Stage(SCR+"stage2")
s.edit(SCH,[('"layerIndexLockWaitMilliseconds": 3000,','"layerIndexLockWaitMilliseconds": 20,',1)])
mod=rd("layer-index.src.mjs").replace('"OBSERVATION"','"../../../../👁️observation/🟨️.mjs"').replace('"SCHEMA"','"../../🧬️schema/🟨️.mjs"')
s.new(SI+"🧾️layer-index/🟨️.mjs",mod)
s.new(SI+"🧾️layer-index/🟨️.d.mts",rd("layer-index.d.mts"))
s.new(SI+"🧾️layer-index/🧪️tests/🟦️.ts",rd("layer-index.unit.test.ts"))
s.new(IN+"🧪️tests/🪪️layer-ids/🟦️.ts",rd("layer-ids.law.ts"))
s.finish([SI+"🧾️layer-index/🧪️tests/🟦️.ts",IN+"🧪️tests/🪪️layer-ids/🟦️.ts",IN+"🧪️tests/🧮️pass-verdicts/🟦️.ts",SI+"💾️entries/🧪️tests/🟦️.ts"],"- Step 1b (R82 a): the row dictionary records carry the 80 bit fingerprint of their row (a restart seeds its table from the headers, no row is parsed; a row read is checked against it and refused when damaged), the deduplication key is built from kind, path and fact without a canonical rendering (cold indexing 8 ms a layer instead of 21), the lock wait of a writer is 20 ms.")
print("staged",s.S)
