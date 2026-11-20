#!/bin/zsh
# R209 landing: row log spills known to the closure proof. Test mode: R209_ROOT=<scratch root> R209_NOWINDOW=1 R209_NOTEST=1 [R209_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R209_ROOT:-$REAL}
T=${R209_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
K=$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R209
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R209/stage
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
PN="$ROOT/$B/🕸️graph/📥️inference/📦️project-nodes"
OB="$ROOT/$B/👁️observation"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
LIBB="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
CL="$LIBB/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures"
GN="$LIBB/🏗️build/🕸️graph/🕰️generations"
FILES=("$CL/🟦️.ts|$K/closures.base.ts|$S/closures.new.ts"
"$GN/🗃️row-log/🟨️.mjs|$K/rowlog.base.mjs|$S/rowlog.new.mjs"
"$GN/🗃️row-log/🟨️.d.mts|$K/rowlog.base.d.mts|$S/rowlog.new.d.mts"
"$LIBB/🏗️build/🕸️graph/🧭️discovery/🟦️.ts|$K/discovery.base.ts|$S/discovery.new.ts")
NEWD=("$CL/🧪️tests/🌀️row-log-spill|$K/law/row-log-spill")
h(){ shasum -a 256 < "$1"; }
if [ -z "$R209_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 1; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 1; waited=$((waited+1)); [ $waited -lt 240 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R209 row-log-spill $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || { echo "live differs from base, nothing written: $live"; exit 2; }; done
for row in "${NEWD[@]}"; do d=${row%%|*}; [ -e "$d" ] && { echo "new test directory exists, nothing written: $d"; exit 2; }; done
restore(){ for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; cp "$base" "$live"; done; for row in "${NEWD[@]}"; do d=${row%%|*}; [ -d "$d" ] && rm -f "$d"/🟦️.ts "$d"/🔣️.json "$d"/🐍️.py; rmdir "$d" 2>/dev/null; done; echo "restored"; for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || echo "RESTORE MISMATCH $live"; done; for row in "${NEWD[@]}"; do d=${row%%|*}; [ -e "$d" ] && echo "RESTORE MISMATCH new test $d"; done; }
for row in "${FILES[@]}"; do live=${row%%|*}; stage=${row##*|}; cp "$stage" "$live" || { restore; exit 3; }; done
for row in "${NEWD[@]}"; do d=${row%%|*}; src=${row#*|}; mkdir -p "$d" && cp "$src"/* "$d"/ || { restore; exit 3; }; done
[ -n "$R209_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R209_NOTEST" ] && { echo "NOTEST: landed without tests (rehearsal)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
DIRS=("$CL/🧪️tests/🌀️row-log-spill" "$GN/🧪️tests/🗃️row-log" "$GN/🧪️tests/🌊️flat-memory" "$GN/🧪️tests/🌊️stream" "$CL/🧪️tests/🚩️fleet-marker" "$CL/🧪️tests/🕰️generation" "$CL/🧪️tests/📥️inference")
for d in "${DIRS[@]}"; do
 [ -f "$d/🟦️.ts" ] || { echo "missing $d"; restore; exit 3; }
 cd "$d"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "${d##*/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
 echo "$out"|grep -q "^ *0 fail" || { if [ "${d##*/}" = "🚦️no-progress" ]; then echo "retry (load-sensitive CPU sample)"; out=$(env -u SEMIO_FLEET_GROUP nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "$out"|grep -q "^ *0 fail" && continue; fi; echo "$out" > "$S/land-fail.log"; echo "$out"|grep -E "^\(fail\)"; restore; exit 3; }
done
cd "$ROOT" && $BUN -e 'await import("'"$GN/🗃️row-log/🟨️.mjs"'");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
