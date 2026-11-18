#!/bin/zsh
# R204 landing: fetch-first adoption, look detach, portable Windows spelling. Test mode: R204_ROOT=<scratch root> R204_NOWINDOW=1 R204_NOTEST=1 [R204_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R204_ROOT:-$REAL}
T=${R204_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
K=$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R204
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R204/stage
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
PN="$ROOT/$B/🕸️graph/📥️inference/📦️project-nodes"
PO="$ROOT/$B/🪪️graph-state/📦️project-records/🧳️portable"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("$PN/📥️take/🟨️.mjs|$K/take.base.mjs|$S/take.new.mjs"
"$PN/📥️take/🟨️.d.mts|$K/take.base.d.mts|$S/take.new.d.mts"
"$PN/📥️take/🧬️schema/🔣️.json|$K/schema.base.json|$S/schema.new.json"
"$PN/🔭️look-queue/🟨️.mjs|$K/look.base.mjs|$S/look.new.mjs"
"$PN/🟨️.mjs|$K/pn.base.mjs|$S/pn.new.mjs"
"$PO/🟦️.ts|$K/portable.base.ts|$S/portable.new.ts")
NEWD=("$PN/📥️take/🧪️tests/🎣️fetch-first|$K/law/fetch-first"
"$PN/🔭️look-queue/🧪️tests/🎣️detach|$K/law/detach"
"$PO/🧪️tests/🪟️windows-spelling|$K/law/windows-spelling")
h(){ shasum -a 256 < "$1"; }
if [ -z "$R204_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 1; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 1; waited=$((waited+1)); [ $waited -lt 240 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R204 fetch-first-adoption $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || { echo "live differs from base, nothing written: $live"; exit 2; }; done
for row in "${NEWD[@]}"; do d=${row%%|*}; [ -e "$d" ] && { echo "new test directory exists, nothing written: $d"; exit 2; }; done
restore(){ for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; cp "$base" "$live"; done; for row in "${NEWD[@]}"; do d=${row%%|*}; [ -d "$d" ] && rm -f "$d"/🟦️.ts "$d"/🔣️.json "$d"/🐍️.py; rmdir "$d" 2>/dev/null; done; echo "restored"; for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || echo "RESTORE MISMATCH $live"; done; for row in "${NEWD[@]}"; do d=${row%%|*}; [ -e "$d" ] && echo "RESTORE MISMATCH new test $d"; done; }
for row in "${FILES[@]}"; do live=${row%%|*}; stage=${row##*|}; cp "$stage" "$live" || { restore; exit 3; }; done
for row in "${NEWD[@]}"; do d=${row%%|*}; src=${row#*|}; mkdir -p "$d" && cp "$src"/* "$d"/ || { restore; exit 3; }; done
[ -n "$R204_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R204_NOTEST" ] && { echo "NOTEST: landed without tests (rehearsal)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
PR="$ROOT/$B/🪪️graph-state/📦️project-records"
DIRS=("$PN/📥️take/🧪️tests/🎣️fetch-first" "$PN/🔭️look-queue/🧪️tests/🎣️detach" "$PO/🧪️tests/🪟️windows-spelling" "$PN/🔭️look-queue/🧪️tests" "$PN/🔭️look-queue/🧪️tests/🧱️isolated-looks" "$PN/📥️take/🧪️tests" "$PN/📥️take/🧪️tests/🔥️hot-lane" "$PN/📥️take/🧪️tests/📣️published" "$PN/📥️take/🧪️tests/🔒️decided" "$PN/📥️take/🧪️tests/🚀️release" "$PN/📥️take/🧪️tests/🪧️tail-report" "$PN/📥️take/🧪️tests/🔬️stale-line" "$PN/🧪️tests/🧾️poll-ledger" "$PN/🧪️tests/🔭️look-job" "$PN/🧪️tests/📥️take" "$PO/🧪️tests" "$PO/🧪️tests/🌍️real" "$PO/🧪️tests/🔎️token-scan" "$PO/🧪️tests/🪶️rest-walk" "$PO/🧪️tests/🕰️stamps" "$PR/🧪️tests" "$PR/🧪️tests/🌍️two-platforms" "$PR/🧪️tests/🗜️packed" "$PR/🧪️tests/🚚️closures" "$PR/🧪️tests/🤝️pool-reader" "$ROOT/$B/🕸️graph/📥️load/🤝️selected-exchange/📏️revision/🧪️tests" "$PN/📥️take/🧪️tests/🚦️no-progress")
for d in "${DIRS[@]}"; do
 [ -f "$d/🟦️.ts" ] || { echo "missing $d"; restore; exit 3; }
 cd "$d"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "${d##*/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
 echo "$out"|grep -q "^ *0 fail" || { if [ "${d##*/}" = "🚦️no-progress" ]; then echo "retry (load-sensitive CPU sample)"; out=$(env -u SEMIO_FLEET_GROUP nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "$out"|grep -q "^ *0 fail" && continue; fi; echo "$out" > "$S/land-fail.log"; echo "$out"|grep -E "^\(fail\)"; restore; exit 3; }
done
cd "$ROOT" && $BUN -e 'await import("'"$PN"'/🟨️.mjs");await import("'"$PN"'/📥️take/🟨️.mjs");await import("'"$PN"'/🔭️look-queue/🟨️.mjs");await import("'"$PO"'/🟦️.ts");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
