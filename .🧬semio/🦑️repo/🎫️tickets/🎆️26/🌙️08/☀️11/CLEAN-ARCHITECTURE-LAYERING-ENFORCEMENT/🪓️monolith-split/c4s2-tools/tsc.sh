#!/bin/zsh
# usage: tsc.sh <label> <tsconfig-dir-relative-to-repo> <file...>   -> G/c4s2-tsc-<label>.txt : tsc --noEmit over exactly these root files with the project's compiler options
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4s2-tools/env.sh
label="$1"; cfgdir="$2"; shift 2
python3 - "$G/c4s2-tsconfig-$label.json" "$R/$cfgdir/tsconfig.json" "$R" "$@" <<'PY'
import json,sys,os
out,base,R=sys.argv[1:4]
files=[os.path.join(R,f) if not f.startswith('/') else f for f in sys.argv[4:]]
json.dump({"extends":base,"compilerOptions":{"noEmit":True,"incremental":False,"composite":False},"include":[],"files":files},open(out,"w",encoding="utf8"),ensure_ascii=False,indent=1)
PY
cd "$R/$cfgdir"
nice -n 10 perl -e 'alarm 1700; exec @ARGV' -- node "$R/node_modules/typescript/bin/tsc" -p "$G/c4s2-tsconfig-$label.json" > "$G/c4s2-tsc-$label.txt" 2>&1
echo "exit=$? lines=$(wc -l < $G/c4s2-tsc-$label.txt)"
