#!/bin/zsh
# usage: quicktsc.sh <label> <file>...  -> G/c4p27-quick-<label>.txt (tsc over exactly these roots + their closure)
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p27-tools/env.sh
label="$1"; shift
python3 - "$G/c4p27-quick-$label.json" "$R" "$@" <<'PY'
import json,sys
out,R,*files=sys.argv[1:]
json.dump({"extends":R+"/🧰️framework/🛍️products/💻️os/tsconfig.json","compilerOptions":{"noEmit":True,"incremental":False,"tsBuildInfoFile":None},"include":[],"files":files},open(out,"w",encoding="utf8"),ensure_ascii=False,indent=1)
PY
cd "$R"
perl -e 'alarm 560; exec @ARGV' -- node node_modules/typescript/bin/tsc --noEmit -p "$G/c4p27-quick-$label.json" > "$G/c4p27-quick-$label.txt" 2>&1
echo "exit=$? lines=$(wc -l < $G/c4p27-quick-$label.txt)"
