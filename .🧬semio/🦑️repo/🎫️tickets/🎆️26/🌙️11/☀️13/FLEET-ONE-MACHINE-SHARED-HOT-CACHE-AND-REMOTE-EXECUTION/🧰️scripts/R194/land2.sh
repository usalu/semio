#!/bin/zsh
# R194 landing 2: pool late refusal + hub line. Test mode: R194_ROOT=<scratch mirror root> R194_NOWINDOW=1 [R194_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R194_ROOT:-$REAL}
T=${R194_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R194
REL="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm"
PF="$ROOT/$REL"; PN="$ROOT/${REL%/*}"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("$PN/🟨️.mjs|$S/s2/pn.base|$S/s2/pn.new.mjs"
"$PN/📥️take/🟨️.mjs|$S/s2/take.base|$S/s2/take.new.mjs"
"$PN/📥️take/🟨️.d.mts|$S/s2/tdts.base|$S/s2/tdts.new"
"$PN/📥️take/🧬️schema/🔣️.json|$S/s2/tschema.base|$S/s2/tschema.new.json"
"$PN/🌐️owner-pool/🧒️child/🟨️.mjs|$S/s2/child.base|$S/s2/child.new.mjs")
h(){ shasum -a 256 < "$1"; }
if [ -z "$R194_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 240 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R194 poll-published-names $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || { echo "live differs from base, nothing written: $live"; exit 2; }; done
restore(){ for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; cp "$base" "$live"; done; echo "restored"; for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || echo "RESTORE MISMATCH $live"; done; }
for row in "${FILES[@]}"; do live=${row%%|*}; stage=${row##*|}; cp "$stage" "$live" || { restore; exit 3; }; done
[ -n "$R194_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R194_NOTESTS" ] && { echo "WRITTEN (no tests) $START $(date -u +%H:%M:%S)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
DIRS=("$PN/📥️take/🧪️tests/📣️published" "$PN/📥️take/🧪️tests" "$PN/🧪️tests/📥️take" "$PN/🧪️tests/🧾️poll-ledger" "$PN/🧪️tests/🫥️self-progress" "$PN/🌐️owner-pool/🧪️tests/🌊️early-wave" "$PN/🌐️owner-pool/🧪️tests/🏋️heavy" "$PN/🌐️owner-pool/🧪️tests/🔔️keep-alive" "$PN/🌐️owner-pool/🧪️tests/🧭️turn" "$PN/🌐️owner-pool/🧪️tests/🚪️exits")
for d in "${DIRS[@]}"; do
 cd "$d"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "$d: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
 echo "$out"|grep -q "^ *0 fail" || { echo "$out" > "$S/land2-fail.log"; echo "$out"|grep -E "^\(fail\)" ; restore; exit 3; }
done
cd "$ROOT" && $BUN -e 'await import("'"$PN"'/🟨️.mjs");await import("'"$PN"'/🌐️owner-pool/🧒️child/🟨️.mjs");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
