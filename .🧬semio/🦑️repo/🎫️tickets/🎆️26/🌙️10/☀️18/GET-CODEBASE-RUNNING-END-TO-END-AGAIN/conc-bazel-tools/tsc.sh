#!/bin/sh
# strict type check of the touched files in the overlay (errors inside the new sessions module and the edited adapter lines)
OV=${1:-/private/tmp/conc-bazel/ov}
cd "$OV" || exit 9
python3 - <<'PY'
import json
Z="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
files=[Z+"/📜️script.ts",Z+"/💾️storage/🟦️.ts",Z+"/⚙️operations/🟦️.ts",Z+"/🧵️sessions/🟦️.ts",Z+"/🧵️sessions/🔌️port/🟦️.ts",Z+"/🧵️sessions/🧪️tests/🟦️.ts",Z+"/🧵️sessions/🧪️tests/🔬️concurrency/🟦️.ts"]
import os
files=[f for f in files if os.path.exists(f)]
json.dump({"compilerOptions":{"target":"ESNext","module":"ESNext","moduleResolution":"bundler","allowImportingTsExtensions":True,"noEmit":True,"strict":True,"noUncheckedIndexedAccess":True,"resolveJsonModule":True,"skipLibCheck":True,"types":["bun"],"esModuleInterop":True,"lib":["ESNext"]},"files":files},open("tsconfig.cb.json","w"))
PY
/Users/ueli/Documents/semio/node_modules/.bin/tsc -p tsconfig.cb.json 2>&1 | grep -E "^🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/" | sed 's/^.*🏗️bazel\///' | cut -c1-300
