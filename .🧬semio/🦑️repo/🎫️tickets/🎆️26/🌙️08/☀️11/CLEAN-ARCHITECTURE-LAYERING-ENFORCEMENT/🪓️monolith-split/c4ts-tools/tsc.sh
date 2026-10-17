#!/bin/zsh
# usage: tsc.sh <label> <base-tsconfig> <product-dir-rel> [extra-root...]
# tsc --noEmit over every tracked or untracked .ts/.tsx under the product dir (plus extra roots); output G/c4ts-tsc-<label>.txt, files list -files.txt
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4ts-tools/env.sh
label="$1"; base="$2"; prod="$3"; shift; shift; shift
cd "$R"
git -c core.quotepath=false -c core.fsmonitor=false ls-files --cached --others --exclude-standard -- "$prod" | /usr/bin/grep -E '\.(ts|tsx)$' | /usr/bin/grep -v -E '/node_modules/|/🗑️generated/|\.d\.ts$' | sort -u > "$G/c4ts-roots-$label.txt"
for e in "$@"; do echo "$e" >> "$G/c4ts-roots-$label.txt"; done
python3 - "$G/c4ts-tsc-$label.json" "$R" "$base" "$G/c4ts-roots-$label.txt" <<'PY'
import json,sys,os
out,R,base,roots=sys.argv[1:]
files=[os.path.join(R,l.strip()) for l in open(roots,encoding="utf8") if l.strip()]
files=[f for f in files if os.path.exists(f)]
json.dump({"extends":base,"compilerOptions":{"noEmit":True,"incremental":False,"tsBuildInfoFile":None,"composite":False},"include":[],"files":files},open(out,"w",encoding="utf8"),ensure_ascii=False,indent=1)
PY
perl -e 'alarm 1700; exec @ARGV' -- nice -n 10 node node_modules/typescript/bin/tsc --noEmit -p "$G/c4ts-tsc-$label.json" > "$G/c4ts-tsc-$label.txt" 2>&1
echo "exit=$? lines=$(wc -l < $G/c4ts-tsc-$label.txt) roots=$(wc -l < $G/c4ts-roots-$label.txt)"
