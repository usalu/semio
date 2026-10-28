#!/bin/sh
# 🪧️ Laws of batch `memolist` after apply: sh verify.sh <root>   (repository or mirror root; SEMIO_REPO_WATCH_BINARY=<watcher binary> also runs the REAL file watcher law)
ROOT=${1:-${VERIFY_ROOT:?root required}}; HERE=$(cd "$(dirname "$0")" && pwd)
R='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'; G="$R/🧭️invocation/🕸️graph/🧠️memo"
cd "$ROOT" || exit 2
python3 -I "$HERE/stage.py" check --root "$ROOT" || exit 2
FAILED=0
law() { name=$1; shift; out=$("$@" 2>&1); code=$?; summary=$(printf '%s\n' "$out" | grep -E "^ *[0-9]+ (pass|skip|fail)$" | tr '\n' ' '); if [ $code -eq 0 ]; then echo "GREEN $name: $summary"; else echo "RED   $name: $summary"; printf '%s\n' "$out" | grep -E "^\(fail\)|^error" | head -8; FAILED=1; fi; }
law "graph memo (policy, key, model in Python, soundness against the engine, lifecycle, stamps, real watcher when SEMIO_REPO_WATCH_BINARY is set)" bun test "./$G/🧪️tests/🟦️.ts"
law "graph memo slice sidecar"                          bun test "./$G/🧪️tests/🎯️slice/🟦️.ts"
law "every test file belongs to one suite"              bun test "./$R/🗂️suites/🧪️tests/🟦️.ts"
exit $FAILED
