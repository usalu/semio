#!/bin/bash
# 🧪️ Builds the native `--lib` test binaries of the given packages (private E4 dirs, slot-gated) and runs every test of
# each in its own process through `🧪️each-test.sh`; results land in `⚡️cache/e4/tests/<tag>/<package>/results.tsv`.
# usage: 🧪️lib-tests.sh <tag> <manifest> <package>... (env PAR=6 ALARM=300)
TAG=$1; MANIFEST=$2; shift 2
HERE="$(cd "$(dirname "$0")" && pwd)"; REPO=/Users/ueli/Documents/semio; E4="$REPO/.🧬semio/🦑️repo/⚡️cache/e4"
mkdir -p "$E4/tests/$TAG"; LOG="$E4/logs/test-$TAG.log"; rm -f "$LOG"
ARGS=(); for p in "$@"; do ARGS+=(-p "$p"); done
bash "$HERE/🏗️cargo.sh" "test-$TAG" 3000 test --offline --manifest-path "$MANIFEST" "${ARGS[@]}" --lib --no-run --message-format json-render-diagnostics || { grep -m5 -A5 '^error' "$LOG"; exit 1; }
for p in "$@"; do
  u=$(echo "$p" | tr - _)
  BIN=$(grep '^{"reason":"compiler-artifact"' "$LOG" | python3 -c "import sys,json
for l in sys.stdin:
    m=json.loads(l)
    if m['target']['name']=='$u' and m['profile']['test'] and m.get('executable'): print(m['executable'])" | tail -1)
  DIR=$(python3 -c "import json,subprocess;print([x for x in json.loads(subprocess.run(['cargo','metadata','--offline','--no-deps','--format-version','1','--manifest-path','$REPO/$MANIFEST'],capture_output=True,text=True).stdout)['packages'] if x['name']=='$p'][0]['manifest_path'].rsplit('/',1)[0])")
  echo "== $p ($(basename "$BIN"))"
  bash "$HERE/🧪️each-test.sh" "$BIN" "$DIR" "$E4/tests/$TAG/$p" "${PAR:-6}" "${ALARM:-300}"
done
