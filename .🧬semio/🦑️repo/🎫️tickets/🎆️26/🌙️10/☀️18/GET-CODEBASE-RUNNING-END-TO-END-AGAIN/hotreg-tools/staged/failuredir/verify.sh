#!/bin/sh
# 🫥️ laws of batch `failuredir` (lane hotreg): sh verify.sh <root> [before]   (bare bun test from <root>; pass the MIRROR root, the script never resolves the live one)
#   after  (default): the executor-owned law 4/0, the key-parts law, the witness suite and the decision-cycle law keep their failure sets (none beyond the base ones)
#   before : the CODE files are restored to base, the law file stays new: the failure-folder, owned-name and file-naming laws are RED (3 of 4); the batch is applied again afterwards
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; MODE="${2:-after}"; FAILED=0
T="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; LAW="$T/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🫥️executor-owned/🟦️.ts"; PARTS="$T/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🗝️parts/🟦️.ts"; CYCLE="$T/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts"; WIT="$T/🔭️witness/🧪️tests/🟦️.ts"
ART="$ROOT/.cache/test-sandbox/hf"; mkdir -p "$ART"
python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
run() { (cd "$ROOT" && env -u SEMIO_NX_FIXED_STORAGE -u NX_CACHE_DIRECTORY -u SEMIO_SCRIPT_PROCESS_INVOCATION SEMIO_TEST_ARTIFACT_DIR="$ART" nice -n 10 bun test --timeout 170000 "$@" 2>&1); }
count() { printf '%s\n' "$1" | grep -E "^ *[0-9]+ $2\$" | tr -dc 0-9; }
expect() { p=$(count "$2" pass); f=$(count "$2" fail); if [ "${p:-0}/${f:-0}" = "$3" ]; then echo "OK    $1 $3"; else echo "FAIL  $1 expected $3 got ${p:-0}/${f:-0}"; printf '%s\n' "$2" | grep -E '^\(fail\)|error:' | head -8; FAILED=1; fi; }
code() { python3 -I -c 'import json,shutil,sys;h,r,w=sys.argv[1:];m=json.load(open(h+"/manifest.json",encoding="utf8"));[shutil.copyfile(h+"/"+w+"/"+v["file"],r+"/"+k) for k,v in m.items() if v["base"] and not k.endswith(".md")]' "$HERE" "$ROOT" "$1"; }
if [ "$MODE" = before ]; then
  code base
  expect "executor-owned law on base" "$(run "./$LAW")" 1/3
  code files; python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -qv "^applied" && { echo "FAIL  batch not re-applied"; FAILED=1; }
  [ $FAILED = 0 ] && echo "VERIFY OK failuredir (before)" || echo "VERIFY FAIL failuredir (before)"; exit $FAILED
fi
expect "executor-owned law" "$(run "./$LAW")" 4/0
expect "key-parts law" "$(run "./$PARTS")" 3/0
out=$(run "./$WIT"); p=$(count "$out" pass); f=$(count "$out" fail); echo "INFO  witness suite pass ${p:-0} fail ${f:-0}"; [ "${f:-0}" = 0 ] || { echo "FAIL  witness suite reds"; printf '%s\n' "$out" | grep -E '^\(fail\)' | head; FAILED=1; }
out=$(run "./$CYCLE" --test-name-pattern "decision cycle"); expect "decision cycle of the adapter" "$out" 1/0
[ $FAILED = 0 ] && echo "VERIFY OK failuredir" || echo "VERIFY FAIL failuredir"; exit $FAILED
