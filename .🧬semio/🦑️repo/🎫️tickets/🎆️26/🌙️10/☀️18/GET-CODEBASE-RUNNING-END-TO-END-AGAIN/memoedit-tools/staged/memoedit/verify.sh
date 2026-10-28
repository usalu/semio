#!/bin/sh
# ✏️ Laws of batch `memoedit` after apply: sh verify.sh <root>   (root = repository or mirror; SEMIO_REPO_WATCH_BINARY=<semio-repo-watch> adds the REAL journal laws)
# RED_ON_BASE=1 (mirror only): restores the base memo core, expects the edit law to fail (the base refuses every touched path), then re-applies.
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${1:?root}
cd "$root"
M=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo
I=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests
python3 -I "$here/stage.py" check --root "$root"
[ -n "${SEMIO_REPO_WATCH_BINARY:-}" ] || SEMIO_REPO_WATCH_BINARY=$(ls -t "$root"/.cache/b/t/m/repo-watch/*/semio-repo-watch 2>/dev/null | head -1)
[ -n "$SEMIO_REPO_WATCH_BINARY" ] && export SEMIO_REPO_WATCH_BINARY || echo "no compiled watcher: the REAL journal laws are skipped"
if [ "${RED_ON_BASE:-0}" = 1 ]; then
 for n in 00 01 02 03; do rel=$(python3 -I -c 'import json,sys;print([k for k,v in json.load(open(sys.argv[1])).items() if v["file"].startswith(sys.argv[2])][0])' "$here/manifest.json" $n); cp "$here/base/$n."* "$root/$rel"; done
 if bun test --timeout 300000 "$M/🧪️tests/✏️edit/🟦️.ts"; then python3 -I "$here/stage.py" apply --root "$root" >/dev/null; echo "RED_ON_BASE: the law passed on base (it does not bite)"; exit 1; fi
 echo "RED_ON_BASE: the edit law fails on base, as required"
 for n in 00 01 02 03; do rel=$(python3 -I -c 'import json,sys;print([k for k,v in json.load(open(sys.argv[1])).items() if v["file"].startswith(sys.argv[2])][0])' "$here/manifest.json" $n); cp "$here/files/$n."* "$root/$rel"; done
fi
bun test --timeout 300000 "$M/🧪️tests/✏️edit/🟦️.ts" "$M/🧪️tests/🟦️.ts" "$M/🧪️tests/🎯️slice/🟦️.ts" "$M/🧪️tests/🔂️reseal/🟦️.ts" "$M/🧪️tests/🕰️verified/🟦️.ts" "$M/🧪️tests/🧳️carry/🟦️.ts" "$M/🧪️tests/🧹️retention/🟦️.ts" "$I/🧊️isolated-plan/🟦️.ts" "$I/📝️plan-file/🟦️.ts"
