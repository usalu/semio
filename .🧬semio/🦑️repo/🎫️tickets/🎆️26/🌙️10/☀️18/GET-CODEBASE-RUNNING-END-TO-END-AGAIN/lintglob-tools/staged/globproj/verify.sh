#!/bin/sh
# 🪠️ laws of batch `globproj` (lane lintglob): sh verify.sh <root> [before]
#   after (default): the closure laws (33 scenarios incl. the 14 pattern scenarios, plain and over the slice view), the Python oracle agreement, project-name selection == minimatch, and the real workspace aggregates (lint, lint-repo, build, test) all green
#   before         : the CODE files (mjs, d.mts) are restored to base, the law files stay new: the pattern scenarios, the minimatch law and lint-repo/lint/build/test aggregates are RED; the batch is applied again afterwards
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; MODE="${2:-after}"; FAILED=0
rel() { python3 -I -c 'import json,sys;m=json.load(open(sys.argv[1]+"/manifest.json",encoding="utf8"));print([k for k in m if k.endswith(sys.argv[2])][0])' "$HERE" "$1"; }
TEST="$(rel .ts)"; ART="$ROOT/.cache/test-sandbox/lg"; mkdir -p "$ART"
python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
run() { (cd "$ROOT" && env -u SEMIO_NX_FIXED_STORAGE -u NX_CACHE_DIRECTORY -u SEMIO_SCRIPT_PROCESS_INVOCATION SEMIO_TEST_ARTIFACT_DIR="$ART" nice -n 10 bun test --timeout 120000 "./$TEST" --test-name-pattern "independent model|slice view of the fragments|project name patterns|workspace aggregate|python oracle|fixture is closed" 2>&1); }
count() { printf '%s\n' "$1" | grep -E "^ *[0-9]+ $2\$" | tr -dc 0-9; }
code() { python3 -I -c 'import json,shutil,sys;h,r,w=sys.argv[1:];m=json.load(open(h+"/manifest.json",encoding="utf8"));[shutil.copyfile(h+"/"+w+"/"+v["file"],r+"/"+k) for k,v in m.items() if v["base"] and not k.endswith((".ts",".json",".py"))]' "$HERE" "$ROOT" "$1"; }
if [ "$MODE" = before ]; then
  code base; out=$(run); p=$(count "$out" pass); f=$(count "$out" fail)
  if [ "${f:-0}" -gt 0 ]; then echo "OK    on base: ${p:-0} pass, ${f} RED"; printf '%s\n' "$out" | grep -E '^\(fail\)' | sed 's/ \[[0-9.]*ms\]$//' | cut -c1-150 | head -40; else echo "FAIL  on base the laws are not red"; FAILED=1; fi
  code files; python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -qv "^applied" && { echo "FAIL  batch not re-applied"; FAILED=1; }
  [ $FAILED = 0 ] && echo "VERIFY OK globproj (before)" || echo "VERIFY FAIL globproj (before)"; exit $FAILED
fi
out=$(run); p=$(count "$out" pass); f=$(count "$out" fail)
if [ "${f:-0}" = 0 ] && [ "${p:-0}" -ge 70 ]; then echo "OK    ${p} pass, 0 fail"; else echo "FAIL  ${p:-0} pass, ${f:-?} fail"; printf '%s\n' "$out" | grep -E '^\(fail\)|error:' | head -12; FAILED=1; fi
[ $FAILED = 0 ] && echo "VERIFY OK globproj" || echo "VERIFY FAIL globproj"; exit $FAILED
