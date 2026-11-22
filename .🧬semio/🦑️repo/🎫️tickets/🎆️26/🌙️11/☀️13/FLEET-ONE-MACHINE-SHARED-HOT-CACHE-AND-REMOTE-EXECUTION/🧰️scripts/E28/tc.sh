#!/bin/zsh
# usage: [CO=<checkout>] tc.sh <files relative to checkout...>  strict typecheck in a scratch checkout; prints errors found in the tools, testing and patches members
export CO=${CO:-/private/tmp/e28/co2}
cd "$CO" || exit 2
python3 - "$@" <<'PY'
import json,sys,os
co=os.environ["CO"]
json.dump({"extends":co+"/tsconfig.json","compilerOptions":{"noEmit":True,"strict":True,"skipLibCheck":True,"types":["bun-types","node"],"typeRoots":[co+"/node_modules",co+"/node_modules/@types"]},"include":[],"files":[co+"/"+a for a in sys.argv[1:]]},open("/private/tmp/e28/tsconfig.check.json","w"))
PY
node node_modules/typescript/bin/tsc -p /private/tmp/e28/tsconfig.check.json --pretty false 2>&1 | grep -E "error TS" | cut -c1-320 | head -${TC_MAX:-40}
echo "tsc done"
