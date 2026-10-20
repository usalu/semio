#!/bin/sh
# usage: sh verify.sh   (rebuilds fixtures + staged set, rebuilds the overlay, applies, runs the suites one at a time)
set -e
cd "$(dirname "$0")"
ROOT=${ROOT:-$(cd ../../../../../../../.. && pwd)}
OV=${OV:-/private/tmp/ci-bazel/ov}
python3 mkfixture.py
python3 build.py --root "$ROOT"
python3 mkoverlay.py --root "$ROOT" --out "$OV"
python3 apply.py --root "$OV" --group all --write --backup "$OV-backup" | tail -3
B=$(python3 -c "import json;print(json.load(open('staged/targets.json'))['tokens']['@CI_REPORT_PATH@'].rsplit('/',1)[0])")
cd "$OV"
nice -n 10 bun test "./$B/🧪️tests/🟦️.ts"
nice -n 10 bun test "./${B%/*}/🔧️tools/🏗️bazel/🧾️test-results/🧪️tests/🟦️.ts"
