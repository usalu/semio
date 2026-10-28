#!/bin/sh
# 📦 Laws of batch `memoout` after apply: sh verify.sh <root>   (repository or mirror root; SEMIO_REPO_WATCH_BINARY=<watcher binary> also runs the REAL file watcher law)
ROOT=${1:-${VERIFY_ROOT:?root required}}; HERE=$(cd "$(dirname "$0")" && pwd)
R='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'; G="$R/🧭️invocation/🕸️graph/🧠️memo"
cd "$ROOT" || exit 2
python3 -I "$HERE/stage.py" check --root "$ROOT" || exit 2
FAILED=0
law() { name=$1; shift; out=$("$@" 2>&1); code=$?; summary=$(printf '%s\n' "$out" | grep -E "^ *[0-9]+ (pass|skip|fail)$" | tr '\n' ' '); if [ $code -eq 0 ]; then echo "GREEN $name: $summary"; else echo "RED   $name: $summary"; printf '%s\n' "$out" | grep -E "^\(fail\)|^error" | head -8; FAILED=1; fi; }
for file in "$G/🧪️tests/🟦️.ts" "$G/🧪️tests/✏️edit/🟦️.ts" "$G/🧪️tests/🎯️slice/🟦️.ts" "$G/🧪️tests/🔂️reseal/🟦️.ts" "$G/🧪️tests/🕰️verified/🟦️.ts" "$G/🧪️tests/🧳️carry/🟦️.ts" "$G/🧪️tests/🧹️retention/🟦️.ts"; do
 [ -f "$file" ] && law "memo suite ${file#$G/🧪️tests/} (real watcher law when SEMIO_REPO_WATCH_BINARY is set)" bun test "./$file"
done
law "every test file belongs to one suite"              bun test "./$R/🗂️suites/🧪️tests/🟦️.ts"
exit $FAILED
