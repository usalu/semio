#!/bin/zsh
# usage: tsc.sh <label> -> G/c4p51-tsc-<label>.txt (+ -files.txt): tsc --noEmit over the quiz react closure plus every quiz test and module tree
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p51-tools/env.sh
label="$1"
python3 - "$G/c4p51-tsconfig.json" "$R" "$Q" <<'PY'
import json,sys
out,R,Q=sys.argv[1:]
P=Q+"/🎯️targets/⚛️react/📦️packages/🟦️typescript"
cfg={"extends":P+"/tsconfig.json","compilerOptions":{"noEmit":True,"incremental":False,"tsBuildInfoFile":None,"composite":False},
"include":[Q+"/🎯️targets/⚛️react/🟦️.tsx",Q+"/🎯️targets/⚛️react/🔨️modules/**/*.ts",Q+"/🎯️targets/⚛️react/🔨️modules/**/*.tsx",Q+"/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx",Q+"/🎯️targets/⚛️react/📦️packages/🟦️typescript/🎨️stylesheet/🟦️.ts",Q+"/🧪️tests/**/*.ts",Q+"/🧪️tests/**/*.tsx",Q+"/📦️packages/🟦️typescript/🟦️.ts",Q+"/🔨️modules/**/*.ts"],"exclude":["**/node_modules/**"]}
json.dump(cfg,open(out,"w",encoding="utf8"),ensure_ascii=False,indent=1)
PY
cd "$P_DIR" 2>/dev/null || cd "$Q/🎯️targets/⚛️react/📦️packages/🟦️typescript"
nice -n 10 perl -e 'alarm 1700; exec @ARGV' -- node "$R/node_modules/typescript/bin/tsc" -p "$G/c4p51-tsconfig.json" --listFilesOnly 2>&1 | /usr/bin/grep "/❓️quiz/" > "$G/c4p51-tsc-$label-files.txt"
nice -n 10 perl -e 'alarm 1700; exec @ARGV' -- node "$R/node_modules/typescript/bin/tsc" -p "$G/c4p51-tsconfig.json" > "$G/c4p51-tsc-$label.txt" 2>&1
echo "exit=$? lines=$(wc -l < $G/c4p51-tsc-$label.txt) quizfiles=$(wc -l < $G/c4p51-tsc-$label-files.txt)"
