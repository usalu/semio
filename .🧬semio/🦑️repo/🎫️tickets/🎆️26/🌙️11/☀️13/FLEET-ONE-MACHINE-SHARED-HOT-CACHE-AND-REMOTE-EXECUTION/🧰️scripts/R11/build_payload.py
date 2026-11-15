"""R11: builds payload.tar (mirror/<rel> = the R11 version, base/<rel> = the live file R11 started from) from the scratch overlay and base."""
import os,sys,tarfile,filecmp
SC=sys.argv[1];OUT=sys.argv[2]
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
BOOT=B+"⚡️caching/🚀️bootstrap/"
FF=BOOT+"🛰️fleet-fetch/"
G=B+"🏗️build/"
SI=G+"🕸️graph/📥️inference/🧭️script-inputs/"
EDITED=[FF+"📦️install/🟦️.ts",FF+"🧬️schema/🔣️.json",FF+"🐍️uv/🟦️.ts",FF+"🐍️uv/🧪️tests/🟦️.ts",BOOT+"📦️dependencies/🏗️native/🟦️.ts",SI+"💾️entries/🟨️.mjs",SI+"💾️entries/🟨️.d.mts",SI+"🟨️.mjs",SI+"🟨️.d.mts",G+"👁️observation/🟨️.mjs",G+"👁️observation/🟨️.d.mts",G+"🪪️graph-state/📦️project-records/🟦️.ts",G+"🪪️graph-state/📦️project-records/🧬️schema/🔣️.json",G+"🪪️graph-state/📦️project-records/🧪️tests/🟦️.ts",G+"👁️observation/🧪️tests/🧫️fixtures/🔣️.json"]
NEW=[FF+"🧰️setup/🟦️.ts",FF+"🧰️setup/🧬️schema/🔣️.json",FF+"🧰️setup/🧪️tests/🟦️.ts",FF+"🧰️setup/🧪️tests/🔣️.json",FF+"🧰️setup/🧪️tests/🐍️.py",G+"🪪️graph-state/📦️project-records/🧪️tests/🚚️closures/🟦️.ts"]
n=0
with tarfile.open(OUT,"w") as t:
    for rel in EDITED:
        mine=SC+"/overlay/"+rel;base=SC+"/base/"+rel
        assert os.path.exists(mine) and os.path.exists(base),rel
        if filecmp.cmp(mine,base,shallow=False):print("unchanged",rel);continue
        t.add(mine,arcname="mirror/"+rel);t.add(base,arcname="base/"+rel);n+=1
    for rel in NEW:
        t.add(SC+"/overlay/"+rel,arcname="mirror/"+rel);n+=1
print(n,"files in",OUT)
