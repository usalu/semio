#!/bin/zsh
# R186: build the mirror and the map for staged runs. usage: stage.sh <staged|old>
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R186
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
R="$B/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready"; N="$B/⚡️caching/🚀️bootstrap/📜️script.ts"
M=$S/mirror/sidecar/🚦️ready; rm -rf $S/mirror; mkdir -p "$M/🧪️tests/🧷️failure-log"
cp $S/new/law.ts "$M/🧪️tests/🧷️failure-log/🟦️.ts"
python3 - "$S" "$R" "$N" "$M" "$1" <<'P'
import json,sys,os,shutil
S,R,N,M,mode=sys.argv[1:]
live={R+"/🟦️.ts":"f1",R+"/🏃️wrapper/🟦️.ts":"f2",R+"/🧬️schema/🔣️.json":"f3",N:"f4"}
ext={"f1":"ts","f2":"ts","f3":"json","f4":"ts"}
run=S+"/run";shutil.rmtree(run,ignore_errors=True);os.makedirs(run)
m={}
os.makedirs(run+"/🏃️wrapper")
for k,v in live.items():
    dst=run+"/🏃️wrapper/🟦️.ts" if v=="f2" else run+"/"+v+"."+ext[v];shutil.copy(S+("/stage/" if mode=="staged" else "/base/")+v,dst)
    if v!="f2":m[k]=dst
m[R+"/🧪️tests/🧷️failure-log/🟦️.ts"]=M+"/🧪️tests/🧷️failure-log/🟦️.ts"
json.dump(m,open(S+"/map.json","w"),ensure_ascii=False,indent=1)
open(S+"/env.sh","w").write("export R186_MODULE='%s' R186_SCRIPT='%s'\n"%(R+"/🟦️.ts",run+"/f4.ts"))
P
