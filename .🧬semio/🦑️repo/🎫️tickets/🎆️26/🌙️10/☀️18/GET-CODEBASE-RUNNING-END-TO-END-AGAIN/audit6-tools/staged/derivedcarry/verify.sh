#!/bin/sh
# 🧺 Laws of batch `derivedcarry`: sh verify.sh <root>   (mirror: python3 -I C/mirror-tools/mirror.py run <name> -- sh <this file> <mirror-root>; SEMIO_REPO_WATCH_BINARY=<semio-repo-watch> adds the REAL watcher laws)
# RED_ON_BASE=1 (mirror only): with the base memo core + schema and the new law, exactly the two-step law fails; then the batch is applied.
set -u
root=$(cd "${1:?usage: verify.sh <root>}" && pwd -P); here=$(cd "$(dirname "$0")" && pwd); fail=0
T=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🧪️tests
run(){ (cd "$root" && bun test "$@" 2>&1); }
counts(){ echo "$1" | grep -E "^ *[0-9]+ (pass|fail|skip)$" | tr -s ' \n' ' '; }
if [ "${RED_ON_BASE:-0}" = 1 ]; then
  python3 -I "$here/stage.py" apply --root "$root" >/dev/null || exit 2
  for rel in "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🟦️.ts" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🧬️schema/🔣️.json"; do
    file=$(python3 -I -c 'import json,sys;print(json.load(open(sys.argv[1],encoding="utf8"))[sys.argv[2]]["file"])' "$here/manifest.json" "$rel"); cp "$here/base/$file" "$root/$rel"; done
  out=$(run "$T/✏️edit/🟦️.ts"); echo "base core: $(counts "$out")"; echo "$out" | grep -E "^\(fail\)" | cut -c1-140
  echo "$out" | grep -qE "^ *1 fail$" && echo "$out" | grep -q "(fail) a derived serve leaves its stale support behind" || { echo "VERIFY FAIL: the two-step law is not red (exactly 1) on base"; exit 1; }
  echo "RED ON BASE: OK"
fi
python3 -I "$here/stage.py" apply --root "$root" >/dev/null || exit 2
for f in "✏️edit/🟦️.ts" "🟦️.ts" "🎯️slice/🟦️.ts" "🔂️reseal/🟦️.ts" "🕰️verified/🟦️.ts" "🧳️carry/🟦️.ts" "🧹️retention/🟦️.ts"; do
  out=$(run "$T/$f"); echo "$f: $(counts "$out")"; echo "$out" | grep -E "^\(fail\)" | cut -c1-140; echo "$out" | grep -qE "^ *0 fail$" || fail=1
done
[ $fail -eq 0 ] && echo "VERIFY derivedcarry: GREEN" || echo "VERIFY derivedcarry: RED"
exit $fail
