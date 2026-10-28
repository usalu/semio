#!/bin/sh
# 🪠️ laws of batch `bazelglob` (lane lintglob): sh verify.sh <root> [before]   (needs batch globproj applied at <root>)
#   after (default): the Bazel plan law (pattern lists over scoped names == minimatch under the Nx rules) and the existing plan laws of the suite are green
#   before         : the CODE file (script.ts) is restored to base, the law file stays new: the pattern law is RED (`*` matches no scoped name, only-`!` lists select nothing); the batch is applied again afterwards
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; MODE="${2:-after}"; FAILED=0
rel() { python3 -I -c 'import json,sys;m=json.load(open(sys.argv[1]+"/manifest.json",encoding="utf8"));print([k for k in m if k.endswith(sys.argv[2])][0])' "$HERE" "$1"; }
TEST="$(rel tests/🟦️.ts)"; ART="$ROOT/.cache/test-sandbox/lgb"; mkdir -p "$ART"
python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
run() { (cd "$ROOT" && env -u SEMIO_NX_FIXED_STORAGE -u NX_CACHE_DIRECTORY -u SEMIO_SCRIPT_PROCESS_INVOCATION SEMIO_TEST_ARTIFACT_DIR="$ART" nice -n 10 bun test --timeout 120000 "./$TEST" --test-name-pattern "dependency project|explicit selections" 2>&1); }
count() { printf '%s\n' "$1" | grep -E "^ *[0-9]+ $2\$" | tr -dc 0-9; }
code() { python3 -I -c 'import json,shutil,sys;h,r,w=sys.argv[1:];m=json.load(open(h+"/manifest.json",encoding="utf8"));[shutil.copyfile(h+"/"+w+"/"+v["file"],r+"/"+k) for k,v in m.items() if v["base"] and not k.endswith("tests/🟦️.ts")]' "$HERE" "$ROOT" "$1"; }
if [ "$MODE" = before ]; then
  code base; out=$(run); p=$(count "$out" pass); f=$(count "$out" fail)
  if [ "${f:-0}" -gt 0 ]; then echo "OK    on base: ${p:-0} pass, ${f} RED"; printf '%s\n' "$out" | grep -E '^\(fail\)' | sed 's/ \[[0-9.]*ms\]$//' | cut -c1-150; else echo "FAIL  on base the law is not red"; FAILED=1; fi
  code files; python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -qv "^applied" && { echo "FAIL  batch not re-applied"; FAILED=1; }
  [ $FAILED = 0 ] && echo "VERIFY OK bazelglob (before)" || echo "VERIFY FAIL bazelglob (before)"; exit $FAILED
fi
out=$(run); p=$(count "$out" pass); f=$(count "$out" fail)
if [ "${f:-0}" = 0 ] && [ "${p:-0}" -ge 2 ]; then echo "OK    ${p} pass, 0 fail"; else echo "FAIL  ${p:-0} pass, ${f:-?} fail"; printf '%s\n' "$out" | grep -E '^\(fail\)|error:' | head -12; FAILED=1; fi
[ $FAILED = 0 ] && echo "VERIFY OK bazelglob" || echo "VERIFY FAIL bazelglob"; exit $FAILED
