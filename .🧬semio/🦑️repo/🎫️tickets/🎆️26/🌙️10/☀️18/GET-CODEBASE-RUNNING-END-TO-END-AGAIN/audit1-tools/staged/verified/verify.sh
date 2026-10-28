#!/bin/sh
# 🕰️ Laws of batch `verified` after apply: sh verify.sh <repository or mirror root>   (default: the root above this batch)
# RED_ON_BASE=1 (mirror only): restores the four base code files, expects the new law to fail, then re-applies.
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${1:-${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}}
cd "$root"
M=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo
python3 -I "$here/stage.py" check --root "$root"
if [ "${RED_ON_BASE:-0}" = 1 ]; then
 cp "$here/base/00.json" "$M/🧬️schema/🔣️.json"; cp "$here/base/01.ts" "$M/🟦️.ts"; cp "$here/base/02.ts" "$M/🚪️command/🟦️.ts"; cp "$here/base/03.ts" "$M/📤️relevance/🟦️.ts"
 if bun test "$M/🧪️tests/🕰️verified/🟦️.ts"; then echo "RED_ON_BASE: the law passed on base (it does not bite)"; python3 -I "$here/stage.py" apply --root "$root"; exit 1; fi
 echo "RED_ON_BASE: the law fails on base, as required"
 python3 -I "$here/stage.py" apply --root "$root"
fi
[ -n "${SEMIO_REPO_WATCH_BINARY:-}" ] || SEMIO_REPO_WATCH_BINARY=$(ls "$root"/.cache/b/t/m/repo-watch/*/semio-repo-watch 2>/dev/null | head -1)
[ -n "$SEMIO_REPO_WATCH_BINARY" ] && export SEMIO_REPO_WATCH_BINARY || echo "no compiled watcher: the REAL journal law is skipped"
I=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests
bun test --timeout 300000 "$M/🧪️tests/🕰️verified/🟦️.ts" "$M/🧪️tests/🎯️slice/🟦️.ts" "$M/🧪️tests/🟦️.ts" "$I/🧊️isolated-plan/🟦️.ts" "$I/📝️plan-file/🟦️.ts" "$I/🧊️taxonomy/🟦️.ts"
