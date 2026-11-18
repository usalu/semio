#!/bin/zsh
# R187: put base (red) or stage (green) copies into the mirror and write the map. usage: stage.sh base|stage
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R187
F="/Users/ueli/Documents/semio/🧰️framework/🔨️modules"; M=$S/mirror/🧰️framework/🔨️modules
O=$M/📁️filesystem/🧾️observation; P=$M/🏃️process/🏗️build/🗣️language/📣️progress
cp $S/$1/d.ts "$O/📁️descriptor/🟦️.ts"; cp $S/$1/ds.json "$O/📁️descriptor/🧬️schema/🔣️.json"; cp $S/$1/p.ts "$P/🟦️.ts"
python3 - "$F" "$M" "$S" <<'P'
import json,sys
F,M,S=sys.argv[1:]
rows=["📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts","📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json","🏃️process/🏗️build/🗣️language/📣️progress/🟦️.ts"]
json.dump({F+"/"+r:M+"/"+r for r in rows},open(S+"/map.json","w"),ensure_ascii=False,indent=1)
P
