#!/bin/sh
# ♻️ laws of batch `quota` (lane cachegc): sh verify.sh <root> [before]   (bare bun test; private graph stores below <root>/.cache/test-sandbox, a long <root> needs SEMIO_PRIVATE_GRAPH_WORKSPACE_PARENT=<short dir>)
#   after  (default): LRU retirement 3/3 (incl. 120 cases == Python model), gc quota 1/1, writer admission model 1/1, insertion into a full optional cache 1/1, and the optional-cache suite keeps only its 3 documented maintenance-worker reds
#   before : the CODE files are restored to base, the law files stay new: the LRU, quota and insertion laws are RED; the batch is applied again afterwards
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; MODE="${2:-after}"; FAILED=0
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; LRU="$L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/♻️lru/🟦️.ts"; QUOTA="$L/🏗️build/🧹️cache-gc/🧪️tests/♻️quota/🟦️.ts"; SUITE="$L/🏗️build/🧪️tests/🕸️graph/🧊️optional-cache/🟦️.ts"; ADMISSION="$L/🏗️build/📥️inputs/💾️storage/🧪️tests/🚪️admission/🟦️.ts"
ART="$ROOT/.cache/test-sandbox/cgv"; PARENT="${SEMIO_PRIVATE_GRAPH_WORKSPACE_PARENT:-$ROOT/.cache/test-sandbox}"; mkdir -p "$ART" "$PARENT"
python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
run() { (cd "$ROOT" && env -u SEMIO_NX_FIXED_STORAGE -u NX_CACHE_DIRECTORY -u SEMIO_SCRIPT_PROCESS_INVOCATION SEMIO_TEST_ARTIFACT_DIR="$ART" SEMIO_PRIVATE_GRAPH_WORKSPACE_PARENT="$PARENT" nice -n 10 bun test --timeout 170000 "$@" 2>&1); }
count() { printf '%s\n' "$1" | grep -E "^ *[0-9]+ $2\$" | tr -dc 0-9; }
expect() { p=$(count "$2" pass); f=$(count "$2" fail); if [ "${p:-0}/${f:-0}" = "$3" ]; then echo "OK    $1 $3"; else echo "FAIL  $1 expected $3 got ${p:-0}/${f:-0}"; printf '%s\n' "$2" | grep -E '^\(fail\)|error:' | head -8; FAILED=1; fi; }
code() { python3 -I -c 'import json,shutil,sys;h,r,w=sys.argv[1:];m=json.load(open(h+"/manifest.json",encoding="utf8"));[shutil.copyfile(h+"/"+w+"/"+v["file"],r+"/"+k) for k,v in m.items() if v["base"] and "/🧪️tests/" not in k]' "$HERE" "$ROOT" "$1"; }
if [ "$MODE" = before ]; then
  code base
  expect "LRU retirement on base" "$(run "./$LRU")" 0/3
  expect "gc quota on base" "$(run "./$QUOTA")" 0/1
  expect "insertion into a full cache on base" "$(run "./$SUITE" --test-name-pattern "insertion into a full")" 0/1
  code files; python3 -I "$HERE/stage.py" check --root "$ROOT" | grep -qv "^applied" && { echo "FAIL  batch not re-applied"; FAILED=1; }
  [ $FAILED = 0 ] && echo "VERIFY OK quota (before)" || echo "VERIFY FAIL quota (before)"; exit $FAILED
fi
expect "LRU retirement" "$(run "./$LRU")" 3/0
expect "gc quota" "$(run "./$QUOTA")" 1/0
expect "writer admission model (eviction before the pressure path)" "$(run "./$ADMISSION")" 1/0
out=$(run "./$SUITE"); names=$(printf '%s\n' "$out" | grep -E '^\(fail\)' | sed 's/ \[[0-9.]*ms\]$//' | sort)
want=$(printf '%s\n' "(fail) actual observation publication reads its persisted full-digest block under the same genuine writer" "(fail) nested graph writers defer optional pressure maintenance while an original immutable pin remains held" "(fail) optional graph writers serialize distinct originals and reuse only the exact active invocation" | sort)
p=$(count "$out" pass); if [ "${p:-0}" = 11 ] && ! printf '%s\n' "$names" | grep -q "insertion into a full"; then echo "OK    optional-cache suite 11 pass incl. the insertion law"; else echo "FAIL  optional-cache suite pass ${p:-0} (expected 11 incl. the insertion law)"; FAILED=1; fi
extra=$(printf '%s\n' "$names" | grep . | grep -vxF "$want")
if [ -z "$extra" ]; then echo "OK    optional-cache suite: only documented reds ($(printf '%s\n' "$names" | grep -c .) of 3)"; else echo "FAIL  optional-cache suite reds beyond the documented 3:"; printf '%s\n' "$extra"; FAILED=1; fi
[ $FAILED = 0 ] && echo "VERIFY OK quota" || echo "VERIFY FAIL quota"; exit $FAILED
