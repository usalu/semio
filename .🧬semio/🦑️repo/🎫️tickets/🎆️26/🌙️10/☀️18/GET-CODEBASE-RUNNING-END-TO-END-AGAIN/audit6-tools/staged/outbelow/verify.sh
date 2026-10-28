#!/bin/sh
# 📦 Laws of batch `outbelow`: sh verify.sh <root>   (mirror: python3 -I C/mirror-tools/mirror.py run <name> -- sh <this file> <mirror-root>; SEMIO_REPO_WATCH_BINARY=<semio-repo-watch> adds the REAL watcher laws incl. the 3 output-directory mutations)
# RED_ON_BASE=1 (mirror only): with the base relevance builder and the new law, exactly the output-directory law fails; then the batch is applied.
set -u
root=$(cd "${1:?usage: verify.sh <root>}" && pwd -P); here=$(cd "$(dirname "$0")" && pwd); fail=0
T=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🧪️tests
run(){ (cd "$root" && bun test "$@" 2>&1); }
counts(){ echo "$1" | grep -E "^ *[0-9]+ (pass|fail|skip)$" | tr -s ' \n' ' '; }
python3 -I "$here/stage.py" apply --root "$root" >/dev/null || exit 2
if [ "${RED_ON_BASE:-0}" = 1 ]; then
  rel="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/📤️relevance/🟦️.ts"
  file=$(python3 -I -c 'import json,sys;print(json.load(open(sys.argv[1],encoding="utf8"))[sys.argv[2]]["file"])' "$here/manifest.json" "$rel"); cp "$here/base/$file" "$root/$rel"
  out=$(run "$T/🟦️.ts" -t "output directory stays"); echo "base relevance: $(counts "$out")"; cp "$here/files/$file" "$root/$rel"
  echo "$out" | grep -qE "^ *1 fail$" || { echo "VERIFY FAIL: the output-directory law is not red on base"; exit 1; }
  echo "RED ON BASE: OK"
fi
for f in "🟦️.ts" "✏️edit/🟦️.ts" "🎯️slice/🟦️.ts" "🔂️reseal/🟦️.ts" "🕰️verified/🟦️.ts" "🧳️carry/🟦️.ts" "🧹️retention/🟦️.ts"; do
  out=$(run "$T/$f"); echo "$f: $(counts "$out")"; echo "$out" | grep -E "^\(fail\)" | cut -c1-140; echo "$out" | grep -qE "^ *0 fail$" || fail=1
done
[ $fail -eq 0 ] && echo "VERIFY outbelow: GREEN" || echo "VERIFY outbelow: RED"
exit $fail
