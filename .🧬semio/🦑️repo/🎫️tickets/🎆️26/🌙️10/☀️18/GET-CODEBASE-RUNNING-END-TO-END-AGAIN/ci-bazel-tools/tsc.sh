#!/bin/sh
# strict type check (noUncheckedIndexedAccess) of the staged TypeScript in the overlay built by verify.sh; errors in files
# of other owners (the budget reset module) are filtered out
OV=${OV:-/private/tmp/ci-bazel/ov}
cd "$(dirname "$0")"
B=$(python3 -c "import json;print(json.load(open('staged/targets.json'))['tokens']['@CI_REPORT_PATH@'].rsplit('/',1)[0])")
Z="${B%/*}/🔧️tools/🏗️bazel"
cat > "$OV/tsconfig.ci-bazel.json" <<EOF
{"compilerOptions":{"target":"esnext","module":"esnext","moduleResolution":"bundler","strict":true,"noUncheckedIndexedAccess":true,"allowImportingTsExtensions":true,"resolveJsonModule":true,"noEmit":true,"skipLibCheck":true,"types":["bun"],"typeRoots":["$OV/node_modules/@types"],"lib":["esnext"]},
"files":["$B/🟦️.ts","$B/📜️script.ts","$B/🧪️tests/🟦️.ts","$Z/🧾️test-results/🟦️.ts","$Z/🧾️test-results/🧪️tests/🟦️.ts","$Z/⚙️operations/🟦️.ts"]}
EOF
cd "$OV" && nice -n 10 node_modules/.bin/tsc -p tsconfig.ci-bazel.json 2>&1 | grep "error TS" | grep -v "♻️reset" | cut -c1-300
echo "tsc finished (no line above = no error in the staged files)"
