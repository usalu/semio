#!/bin/sh
# 📍️ Laws of batch `jdir` after apply: sh verify.sh <root>   (repository or mirror root)
ROOT=${1:-${VERIFY_ROOT:?root required}}; HERE=$(cd "$(dirname "$0")" && pwd)
R='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'
cd "$ROOT" || exit 2
python3 -I "$HERE/stage.py" check --root "$ROOT" || exit 2
FAILED=0
law() { name=$1; shift; out=$("$@" 2>&1); code=$?; summary=$(printf '%s\n' "$out" | grep -E "^ *[0-9]+ (pass|fail)$" | tr '\n' ' '); if [ $code -eq 0 ]; then echo "GREEN $name: $summary"; else echo "RED   $name: $summary"; printf '%s\n' "$out" | grep -E "^\(fail\)|^error" | head -8; FAILED=1; fi; }
law "journal directory independent of TMPDIR (platform vectors, Python oracle, two processes)" bun test "./$R/🏡️resident/📜️journal/🧪️tests/📍️directory/🟦️.ts"
law "change journal service and client"      bun test "./$R/🏡️resident/📜️journal/🧪️tests/🟦️.ts"
law "resident protocol"                      bun test "./$R/🏡️resident/🧪️tests/📨️protocol/🟦️.ts"
law "resident command"                       bun test "./$R/🏡️resident/🧪️tests/⌨️command/🟦️.ts"
law "service grant and endpoints"            bun test "./$R/📥️caller/🛰️service/🧪️tests/🟦️.ts"
law "graph memo"                             bun test "./$R/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🟦️.ts"
law "every test file belongs to one suite"   bun test "./$R/🗂️suites/🧪️tests/🟦️.ts"
echo "(resident service law: 3 failures are pre-existing without git: run-many/dirty queries, frontier cursors, graph changes)"
exit $FAILED
