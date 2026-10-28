#!/bin/sh
# usage: verify.sh <mirror-root>   (applies this batch to the mirror, then runs the laws)
set -eu
ROOT="$1"; HERE="$(cd "$(dirname "$0")" && pwd)"
PY=python3
"$PY" -I "$HERE/stage.py" apply --root "$ROOT"
cd "$ROOT"
"$PY" -I "$HERE/law_umlaut.py" "$ROOT" "$HERE/manifest.json"
mkdir -p "${SEMIO_TEST_ARTIFACT_DIR:?set a short artifact dir under the ticket generated folder}"
while IFS= read -r d; do [ -n "$d" ] && (cd "$ROOT/$d" && echo "### $d" && bun test --timeout 60000 ./🟦️.ts 2>&1 | grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)$' || true); done < "$HERE/tests.txt"
echo "umlaut batch: law GREEN; compare the (fail) lines above with the baseline in C/result-progress.md"
