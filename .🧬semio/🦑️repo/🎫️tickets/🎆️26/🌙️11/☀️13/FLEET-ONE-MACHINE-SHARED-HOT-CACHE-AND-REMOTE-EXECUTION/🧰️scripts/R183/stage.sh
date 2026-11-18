#!/bin/zsh
# R183: build the mirror and the map for staged runs. usage: stage.sh <mode: staged|old>
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R183
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
R="$B/🌐️remote-execution/🛰️sidecar/🚦️ready"; G="$B/🌐️remote-execution/🛰️sidecar/🕸️graph"
M=$S/mirror/sidecar/🚦️ready; rm -rf $S/mirror; mkdir -p "$M/🌲️process-tree/🧪️tests/🤝️holder-custody" "$M/🌲️process-tree/🧬️schema" "$M/🧪️tests/🤝️holder-custody" "$S/mirror/sidecar/🕸️graph"
cp $S/new/pt/law.ts "$M/🌲️process-tree/🧪️tests/🤝️holder-custody/🟦️.ts"; cp $S/new/pt/oracle.py "$M/🌲️process-tree/🧪️tests/🤝️holder-custody/🐍️.py"; cp $S/new/pt/fixture.json "$M/🌲️process-tree/🧪️tests/🤝️holder-custody/🔣️.json"
cp $S/new/rd/law.ts "$M/🧪️tests/🤝️holder-custody/🟦️.ts"; cp $S/stage/f2 "$M/🌲️process-tree/🧬️schema/🔣️.json"; cp $S/stage/f7 "$S/mirror/sidecar/🕸️graph/🟦️.ts"
python3 - "$S" "$R" "$G" "$M" "$1" <<'P'
import json,sys
S,R,G,M,mode=sys.argv[1:]
live={R+"/🌲️process-tree/🟦️.ts":"f1",R+"/🌲️process-tree/🧬️schema/🔣️.json":"f2",R+"/🌲️process-tree/🧹️containment/🟦️.ts":"f3",R+"/🌲️process-tree/🧹️containment/🧬️schema/🔣️.json":"f4",R+"/🟦️.ts":"f5",R+"/🧬️schema/🔣️.json":"f6",G+"/🟦️.ts":"f7"}
import os,shutil
ext={"f1":"ts","f2":"json","f3":"ts","f4":"json","f5":"ts","f6":"json","f7":"ts"}
run=S+"/run";shutil.rmtree(run,ignore_errors=True);os.makedirs(run)
m={}
for k,v in live.items():
    dst=run+"/"+v+"."+ext[v];shutil.copy(S+("/stage/" if mode=="staged" else "/base/")+v,dst);m[k]=dst
m[R+"/🌲️process-tree/🧪️tests/🤝️holder-custody/🟦️.ts"]=M+"/🌲️process-tree/🧪️tests/🤝️holder-custody/🟦️.ts"
m[R+"/🧪️tests/🤝️holder-custody/🟦️.ts"]=M+"/🧪️tests/🤝️holder-custody/🟦️.ts"
os.makedirs(run+"/🏃️wrapper",exist_ok=True);shutil.copy(R+"/🏃️wrapper/🟦️.ts",run+"/🏃️wrapper/🟦️.ts")
json.dump(m,open(S+"/map.json","w"),ensure_ascii=False,indent=1)
P
