#!/bin/sh
# 🪈️ laws of batch `buildscript` (lane lintglob): sh verify.sh <root> [before]
#   after (default): the mounted-sources corpus (19 crates incl. 3 environment-only build scripts resolved == files rustc reads, 6 refused script shapes) and the real renderer crate of this repository are green
#   before         : the CODE file (mjs) is restored to base, the law files stay new: the three resolved script crates and the real renderer crate are RED (refused `build-script`); the batch is applied again afterwards
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; MODE="${2:-after}"; FAILED=0
rel() { python3 -I -c 'import json,sys;m=json.load(open(sys.argv[1]+"/manifest.json",encoding="utf8"));print([k for k in m if k.endswith(sys.argv[2])][0])' "$HERE" "$1"; }
TEST="$(rel .ts)"; ART="$ROOT/.cache/test-sandbox/lgs"; mkdir -p "$ART"
python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
run() { (cd "$ROOT" && env -u SEMIO_NX_FIXED_STORAGE -u NX_CACHE_DIRECTORY -u SEMIO_SCRIPT_PROCESS_INVOCATION SEMIO_TEST_ARTIFACT_DIR="$ART" nice -n 10 bun test --timeout 120000 "./$TEST" 2>&1); }
count() { printf '%s\n' "$1" | grep -E "^ *[0-9]+ $2\$" | tr -dc 0-9; }
code() { python3 -I -c 'import json,shutil,sys;h,r,w=sys.argv[1:];m=json.load(open(h+"/manifest.json",encoding="utf8"));[shutil.copyfile(h+"/"+w+"/"+v["file"],r+"/"+k) for k,v in m.items() if v["base"] and k.endswith(".mjs")]' "$HERE" "$ROOT" "$1"; }
if [ "$MODE" = before ]; then
  code base; out=$(run); p=$(count "$out" pass); f=$(count "$out" fail)
  if [ "${f:-0}" = 4 ]; then echo "OK    on base: ${p:-0} pass, 4 RED"; printf '%s\n' "$out" | grep -E '^\(fail\)' | sed 's/ \[[0-9.]*ms\]$//' | cut -c1-150; else echo "FAIL  on base expected 4 red, got ${f:-0}"; printf '%s\n' "$out" | grep -E '^\(fail\)' | cut -c1-150; FAILED=1; fi
  code files; python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -qv "^applied" && { echo "FAIL  batch not re-applied"; FAILED=1; }
  [ $FAILED = 0 ] && echo "VERIFY OK buildscript (before)" || echo "VERIFY FAIL buildscript (before)"; exit $FAILED
fi
out=$(run); p=$(count "$out" pass); f=$(count "$out" fail)
if [ "${f:-0}" = 0 ] && [ "${p:-0}" -ge 23 ]; then echo "OK    ${p} pass, 0 fail"; else echo "FAIL  ${p:-0} pass, ${f:-?} fail"; printf '%s\n' "$out" | grep -E '^\(fail\)|error:' | head -12; FAILED=1; fi
[ $FAILED = 0 ] && echo "VERIFY OK buildscript" || echo "VERIFY FAIL buildscript"; exit $FAILED
