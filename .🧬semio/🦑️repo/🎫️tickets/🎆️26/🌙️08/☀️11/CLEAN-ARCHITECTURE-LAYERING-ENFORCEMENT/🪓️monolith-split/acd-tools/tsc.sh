#!/bin/zsh
# usage: tsc.sh <label> -> G/acd-tsc-<label>.txt: tsc --noEmit over mkroots.py roots (all five module trees, their SDK wrappers, known importers)
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/acd-tools/env.sh
label="$1"
python3 "$T/mkroots.py" > "$G/acd-roots-$label.txt"
python3 - "$G/acd-tsc-$label.json" "$R" "$G/acd-roots-$label.txt" <<'PY'
import json,sys
out,R,roots=sys.argv[1:]
files=[l.strip() for l in open(roots,encoding="utf8") if l.strip()]
json.dump({"extends":R+"/🧰️framework/🛍️products/💻️os/tsconfig.json","compilerOptions":{"noEmit":True,"incremental":False,"tsBuildInfoFile":None},"include":[],"files":files},open(out,"w",encoding="utf8"),ensure_ascii=False,indent=1)
PY
cd "$R"
perl -e 'alarm 1700; exec @ARGV' -- nice -n 10 node node_modules/typescript/bin/tsc --noEmit -p "$G/acd-tsc-$label.json" > "$G/acd-tsc-$label.txt" 2>&1
echo "exit=$? lines=$(wc -l < $G/acd-tsc-$label.txt) roots=$(wc -l < $G/acd-roots-$label.txt)"
