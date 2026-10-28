from make_lib import *

# ---- A: fact index stateless duplication trigger
mjs, dts, sch = G + "/🟨️.mjs", G + "/🟨️.d.mts", G + "/🧬️schema/🔣️.json"
edit(mjs, "compactionGrowth:1.25,", "compactionGrowth:1.25,compactionDuplication:1.5,")
edit(dts, "readonly compactionGrowth:number;", "readonly compactionGrowth:number;readonly compactionDuplication:number;")
edit(sch, '    "compactionGrowth": 1.25,\n', '    "compactionGrowth": 1.25,\n    "compactionDuplication": 1.5,\n')
edit(mjs, " return result.classes.length>P.compactionFactor*result.owners.length+P.compactionFloor&&result.classes.length>P.compactionGrowth*index.classes.length?refineFactIndexV1(result):result;\n",
 " const ids=result.classes.reduce((sum,cls)=>sum+cls.r.length,0);\n return result.classes.length>P.compactionFactor*result.owners.length+P.compactionFloor&&result.classes.length>P.compactionGrowth*index.classes.length||ids>P.compactionFloor&&ids>P.compactionDuplication*result.rows.length?refineFactIndexV1(result):result;\n")
edit(mjs, "prunes unreferenced rows and rebuilds the partition once classes outgrow the owners. */",
 "prunes unreferenced rows and rebuilds the partition once classes outgrow the owners or the class lists hold more than `compactionDuplication` times the rows (the duplication of an index patched generation after generation, which no comparison with the previous generation can see). */")
newfile(G + "/🧪️tests/🌊️patch-drift/🟦️.ts", "patch-drift.ts")

if False:
    write("rev3fix")
    print(len(spec["edits"]), "edits")
