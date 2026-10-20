#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
roots=$(python3 - <<PY
import json
i=json.load(open("$C/tsc-stage-tools/staged/index.json"))
for k in ["03-.ts","10-.ts","12-.ts","15-.ts","16-.ts","19-.ts","21-.ts","22-.ts"]: print(i[k])
PY
)
perl -e 'alarm 580; exec @ARGV' -- env NODE_OPTIONS=--max-old-space-size=8192 node "$C/tsc-stage-tools/tscheck-files.mjs" "$L/📦️packages/🟦️typescript/tsconfig.json" "$C/tsc-stage-tools/staged" ${(f)roots} 2>&1 | cut -c1-420
