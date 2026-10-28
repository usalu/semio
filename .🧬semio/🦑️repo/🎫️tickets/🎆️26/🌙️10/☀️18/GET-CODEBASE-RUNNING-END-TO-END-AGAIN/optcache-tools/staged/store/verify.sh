#!/bin/sh
# ✅ laws of batch `store` (lane optcache): sh verify.sh <root> [before]; BARE bun test (no parent envelope), artifacts <root>/.cache/test-sandbox/ocv; the private stores live below $SEMIO_PRIVATE_GRAPH_WORKSPACE_PARENT or <root>/.cache/test-sandbox (a long <root> needs a short parent, e.g. /private/tmp/oc-pw)
#   default : suite 10 pass / 1 skip / 3 fail where the 3 are the documented maintenance-worker laws (writers-serialize child, nested maintenance, observation publication); no private repository left behind; a killed run's private repository is reaped by the next run
#   before  : the batch's support helpers + the isolation law on the BASE suite (restored for the run, the batch is applied again afterwards): the law is RED
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; MODE="${2:-after}"
G="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph"; SUITE="$G/🧊️optional-cache/🟦️.ts"
ART="$ROOT/.cache/test-sandbox/ocv"; PARENT="${SEMIO_PRIVATE_GRAPH_WORKSPACE_PARENT:-$ROOT/.cache/test-sandbox}"; mkdir -p "$ART" "$PARENT"; FAILED=0
python3 -I "$HERE/stage.py" check "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
run() { (cd "$ROOT" && env -u SEMIO_NX_FIXED_STORAGE -u NX_CACHE_DIRECTORY -u SEMIO_SCRIPT_PROCESS_INVOCATION SEMIO_TEST_ARTIFACT_DIR="$ART" SEMIO_PRIVATE_GRAPH_WORKSPACE_PARENT="$PARENT" nice -n 10 bun test --timeout 170000 "./$SUITE" "$@" 2>&1); }
count() { printf '%s\n' "$1" | grep -E "^ *[0-9]+ $2\$" | tr -dc 0-9; }
left() { ls "$PARENT" | grep -c '^g[A-Za-z0-9]\{6\}$'; }
if [ "$MODE" = before ]; then
  python3 -I "$HERE/stage.py" revert "$ROOT" >/dev/null || exit 2
  python3 -I "$HERE/../../edit_optcache.py" "$ROOT" --law-only >/dev/null || { python3 -I "$HERE/stage.py" apply "$ROOT" >/dev/null; exit 2; }
  out=$(run --test-name-pattern "touches no live")
  python3 -I -c 'import json,shutil,sys;h,r=sys.argv[1:];m=json.load(open(h+"/manifest.json",encoding="utf8"));[shutil.copyfile(h+"/base/"+v["file"],r+"/"+k) for k,v in m.items()]' "$HERE" "$ROOT"; python3 -I "$HERE/stage.py" apply "$ROOT" >/dev/null
  p=$(count "$out" pass); f=$(count "$out" fail)
  if [ "${p:-0}" = 0 ] && [ "${f:-0}" = 1 ]; then echo "OK    RED on base: isolation law 0/1"; else echo "FAIL  expected 0/1 on base, got ${p:-0}/${f:-0}"; FAILED=1; fi
  [ $FAILED = 0 ] && echo "VERIFY OK store (before)" || echo "VERIFY FAIL store (before)"; exit $FAILED
fi
out=$(run); p=$(count "$out" pass); f=$(count "$out" fail); s=$(count "$out" skip)
names=$(printf '%s\n' "$out" | grep -E '^\(fail\)' | sed 's/ \[[0-9.]*ms\]$//' | sort)
want=$(printf '%s\n' "(fail) actual observation publication reads its persisted full-digest block under the same genuine writer" "(fail) nested graph writers defer optional pressure maintenance while an original immutable pin remains held" "(fail) optional graph writers serialize distinct originals and reuse only the exact active invocation" | sort)
if [ "${p:-0}/${f:-0}/${s:-0}" = 10/3/1 ] && [ "$names" = "$want" ]; then echo "OK    10/3/1 and the 3 documented maintenance-worker laws"; else echo "FAIL  got ${p:-0}/${f:-0}/${s:-0}"; printf '%s\n' "$names"; FAILED=1; fi
if [ "$(left)" = 0 ]; then echo "OK    no private repository left"; else echo "FAIL  left behind: $(ls "$PARENT" | grep '^g[A-Za-z0-9]\{6\}$')"; FAILED=1; fi
( cd "$ROOT" && exec env -u SEMIO_NX_FIXED_STORAGE -u NX_CACHE_DIRECTORY -u SEMIO_SCRIPT_PROCESS_INVOCATION SEMIO_TEST_ARTIFACT_DIR="$ART" SEMIO_PRIVATE_GRAPH_WORKSPACE_PARENT="$PARENT" nice -n 10 bun test --timeout 170000 "./$SUITE" --test-name-pattern "touches no live" ) > "$ART/killed.log" 2>&1 &
KILLED=$!; n=0; while [ $n -lt 150 ] && ! ls "$PARENT"/g*/.owner >/dev/null 2>&1; do sleep 1; n=$((n+1)); done; sleep 10
kill -KILL "$KILLED" 2>/dev/null; wait "$KILLED" 2>/dev/null; sleep 2
if [ "$(left)" = 1 ]; then echo "OK    a killed run leaves exactly one private repository"; else echo "FAIL  expected 1 leftover after the kill, got $(left)"; FAILED=1; fi
out=$(run --test-name-pattern "touches no live"); p=$(count "$out" pass)
if [ "${p:-0}" = 1 ] && [ "$(left)" = 0 ]; then echo "OK    the next run reaped it"; else echo "FAIL  reap: pass ${p:-0}, left $(left)"; FAILED=1; fi
rm -f "$ART/killed.log"
[ $FAILED = 0 ] && echo "VERIFY OK store" || echo "VERIFY FAIL store"; exit $FAILED
