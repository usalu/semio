#!/bin/sh
# 🧳 Laws of batch `carry` after apply: VERIFY_ROOT=<repository or mirror root> sh verify.sh   (default: the root above this batch)
# RED_ON_BASE=1 (mirror only): restores the base memo core, expects the carry law to fail, then re-applies.
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
cd "$root"
M=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo
python3 -I "$here/stage.py" check --root "$root"
if [ "${RED_ON_BASE:-0}" = 1 ]; then
 cp "$here/base/00.ts" "$M/🟦️.ts"
 if bun test "$M/🧪️tests/🧳️carry/🟦️.ts"; then cp "$here/files/00.ts" "$M/🟦️.ts"; echo "RED_ON_BASE: the law passed on base (it does not bite)"; exit 1; fi
 echo "RED_ON_BASE: the law fails on base, as required"
 cp "$here/files/00.ts" "$M/🟦️.ts"
fi
[ -n "${SEMIO_REPO_WATCH_BINARY:-}" ] || SEMIO_REPO_WATCH_BINARY=$(ls "$root"/.cache/b/t/m/repo-watch/*/semio-repo-watch 2>/dev/null | head -1)
[ -n "$SEMIO_REPO_WATCH_BINARY" ] && export SEMIO_REPO_WATCH_BINARY || echo "no compiled watcher: the REAL journal law is skipped"
I=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests
bun test --timeout 300000 "$M/🧪️tests/🧳️carry/🟦️.ts" "$M/🧪️tests/🎯️slice/🟦️.ts" "$M/🧪️tests/🕰️verified/🟦️.ts" "$M/🧪️tests/🟦️.ts" "$I/🧊️isolated-plan/🟦️.ts" "$I/📝️plan-file/🟦️.ts"
