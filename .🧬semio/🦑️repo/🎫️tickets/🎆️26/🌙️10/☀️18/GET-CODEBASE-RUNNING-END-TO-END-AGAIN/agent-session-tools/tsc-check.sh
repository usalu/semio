#!/bin/sh
# usage: check.sh <overlay> <file-list>: type-checks the listed files (paths relative to the overlay) strictly; prints the diagnostics
OV="$1"; LIST="$2"
cd "$OV" || exit 2
python3 - "$OV" "$LIST" <<'PY'
import json,sys
ov,lst=sys.argv[1],sys.argv[2]
files=[l.strip() for l in open(lst,encoding="utf8") if l.strip()]
json.dump({"compilerOptions":{"target":"ES2023","module":"ESNext","moduleResolution":"bundler","strict":True,"noUncheckedIndexedAccess":True,"allowImportingTsExtensions":True,"resolveJsonModule":True,"noEmit":True,"skipLibCheck":True,"types":["bun","node"],"typeRoots":["/Users/ueli/Documents/semio/node_modules/@types"],"lib":["ES2023","DOM"],"noUnusedLocals":False},"files":[ov+"/"+f for f in files]},open("/private/tmp/agent-session/tsc/tsconfig.json","w"),ensure_ascii=False)
PY
nice -n 10 /Users/ueli/Documents/semio/node_modules/.bin/tsc -p /private/tmp/agent-session/tsc/tsconfig.json 2>&1 | sed "s#$OV/##" 
