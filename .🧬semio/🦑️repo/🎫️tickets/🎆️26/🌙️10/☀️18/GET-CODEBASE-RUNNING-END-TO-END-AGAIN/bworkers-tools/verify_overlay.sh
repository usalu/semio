#!/bin/bash
# ✅ Runs the new and the affected test files against the overlay, one file at a time (never a directory), with the real Bazel cross-check enabled.
cd "$(dirname "$0")" || exit 2
R=${BW_OVERLAY:-/private/tmp/bworkers/r}
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B="$L/🏗️build"
python3 - "$R" <<'PY'
import re,sys
root=sys.argv[1]
p=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/👷️workers/🧪️tests/🟦️.ts"
s=open(p,encoding="utf8").read()
s=re.sub(r'const root=join\(import\.meta\.dir,"[./]+"\)\.replace\(/\[\\\\/\]\+\$/u,""\)','const root="/Users/ueli/Documents/semio"',s)
open(p,"w",encoding="utf8").write(s)
PY
export SEMIO_BAZEL_INTEGRATION=1 SEMIO_BAZEL_BINARY=/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel
./runtests.sh "$R" \
 "$B/👷️workers/🧪️tests/🟦️.ts" \
 "$B/🔧️tools/🏗️bazel/👷️workers/🧪️tests/🟦️.ts" \
 "$L/⚡️caching/🚀️bootstrap/🛠️tools/👷️workers/🧪️tests/🟦️.ts" \
 "$B/🔧️tools/🏗️bazel/🧪️tests/📥️actions/🟦️.ts" \
 "$B/🔧️tools/🏗️bazel/🧪️tests/🔏️implementation/🟦️.ts" \
 "$B/🔧️tools/🏗️bazel/🧪️tests/📥️sources/🟦️.ts" \
 "$B/🔧️tools/🏗️bazel/🧪️tests/📥️fingerprint/🟦️.ts" \
 "$B/🧭️selection/🧪️tests/🟦️.ts" \
 "$B/🧭️invocation/🧪️tests/🟦️.ts" \
 "$B/⚙️operations/🧪️tests/🟦️.ts"
