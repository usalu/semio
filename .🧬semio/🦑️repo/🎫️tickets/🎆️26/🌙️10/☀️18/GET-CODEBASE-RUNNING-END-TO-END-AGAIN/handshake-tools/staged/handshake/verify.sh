#!/bin/sh
# 🤝 laws of batch `handshake` (lane handshake): sh verify.sh <root> [before]
#   after (default): handshake work-clock laws 12/12 (fixture from the independent Python oracle `oracle.py`), the scope closure laws and the neighbouring Bazel executor suites keep their base failure sets
#   before         : the executor is restored to base while the law files stay new: the handshake laws are RED; the batch is applied again afterwards
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; STAGE="$HERE"; MODE="${2:-after}"; FAILED=0
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; PM="🧰️framework/🔨️modules/🏃️process"; B="$L/🏗️build/🔧️tools/🏗️bazel"; LAW="$B/🧪️tests/📥️handshake/🟦️.ts"
ART="${SEMIO_TEST_ARTIFACT_DIR:-$HERE/../../../🗑️generated/handshake/art}"; mkdir -p "$ART"
python3 -I "$STAGE/stage.py" check --root "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
run() { (cd "$ROOT" && env -u SEMIO_NX_FIXED_STORAGE -u NX_CACHE_DIRECTORY -u SEMIO_SCRIPT_PROCESS_INVOCATION SEMIO_TEST_ARTIFACT_DIR="$ART" nice -n 10 bun test --timeout 170000 "$@" 2>&1); }
count() { printf '%s\n' "$1" | grep -E "^ *[0-9]+ $2\$" | tr -dc 0-9; }
expect() { p=$(count "$2" pass); f=$(count "$2" fail); if [ "${p:-0}/${f:-0}" = "$3" ]; then echo "OK    $1 $3"; else echo "FAIL  $1 expected $3 got ${p:-0}/${f:-0}"; printf '%s\n' "$2" | grep -E '^\(fail\)|error:' | head -8; FAILED=1; fi; }
code() { python3 -I -c 'import json,shutil,sys;h,r,w=sys.argv[1:];m=json.load(open(h+"/manifest.json",encoding="utf8"));[shutil.copyfile(h+"/"+w+"/"+v["file"],r+"/"+k) for k,v in m.items() if v["base"] and "/🧪️tests/" not in k]' "$STAGE" "$ROOT" "$1"; }
if [ "$MODE" = before ]; then
  code base
  out=$(run "./$LAW"); p=$(count "$out" pass); f=$(count "$out" fail)
  if [ "${f:-0}" -ge 1 ] && printf '%s\n' "$out" | grep -q "(fail) the action birth polls the handshake"; then echo "OK    laws RED on base (${p:-0} pass / ${f:-0} fail, incl. the second-wall-clock law)"; else echo "FAIL  laws not red on base (${p:-0}/${f:-0})"; FAILED=1; fi
  code files; python3 -I "$STAGE/stage.py" check --root "$ROOT" | grep -qv "^applied" && { echo "FAIL  batch not re-applied"; FAILED=1; }
  [ $FAILED = 0 ] && echo "VERIFY OK handshake (before)" || echo "VERIFY FAIL handshake (before)"; exit $FAILED
fi
expect "handshake work-clock laws" "$(run "./$LAW")" 12/0
SUITES="$B/🎬️scopes/🧪️tests/🟦️.ts $B/🧪️tests/📥️framing/🟦️.ts $B/🧪️tests/📥️control/🟦️.ts $B/🧪️tests/📥️purpose/🟦️.ts $B/🧪️tests/📥️mode/🟦️.ts $L/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts $PM/🧪️testing/🎚️budget/⏲️contention/🧪️tests/🟦️.ts"
sets() { for suite in $SUITES; do printf '%s\n' "## $suite"; run "./$suite" | grep -E '^\(fail\)|^ *[0-9]+ pass$' | sed 's/ \[[0-9.]*ms\]$//' | sort; done; }
AFTER=$(sets); code base; BEFORE=$(sets); code files
python3 -I "$STAGE/stage.py" check --root "$ROOT" | grep -qv "^applied" && { echo "FAIL  batch not re-applied"; FAILED=1; }
REDS=$(printf '%s\n' "$AFTER" | grep -c '^.fail')
if [ "$AFTER" = "$BEFORE" ]; then echo "OK    failure sets of the scopes, framing, control, purpose, mode, scheduling and contention suites equal base ($REDS pre-existing reds)"; else echo "FAIL  failure sets differ from base:"; printf '%s\n' "$BEFORE" > "$ART/[DEBUG] sets.base"; printf '%s\n' "$AFTER" > "$ART/[DEBUG] sets.after"; diff "$ART/[DEBUG] sets.base" "$ART/[DEBUG] sets.after"; FAILED=1; fi
[ $FAILED = 0 ] && echo "VERIFY OK handshake" || echo "VERIFY FAIL handshake"; exit $FAILED
