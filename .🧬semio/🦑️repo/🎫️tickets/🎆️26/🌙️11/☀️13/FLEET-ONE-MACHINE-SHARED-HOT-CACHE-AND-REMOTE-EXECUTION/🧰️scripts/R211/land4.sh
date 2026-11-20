#!/bin/zsh
# R211 landing 3: machine-local rows judged by the verifier (precedence fix), .store directory machine-local. Test mode: R211D_ROOT=<mirror> R211D_NOWINDOW=1 [R211D_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R211D_ROOT:-$REAL}
T=${R211D_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R211
B="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; O="$B/👁️observation"; RD="$B/🌐️remote-execution/🛰️sidecar"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
N="$B/🕸️graph/📥️inference/📦️project-nodes"
FILES=("$N/🟨️.mjs|$S/stage/pn4.base.mjs|$S/stage/pn4.new.mjs"
"$N/📥️take/🧪️tests/🧮️stale-fleet-told/🟦️.ts|$S/stage/sft4.base.ts|$S/stage/sft4.new.ts")
NEWF=()
h(){ shasum -a 256 < "$1"; }
if [ -z "$R211D_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 1; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R211 restore-reusable-shape $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || { echo "live differs from base, nothing written: $live"; exit 2; }; done
for row in "${NEWF[@]}"; do live=${row%%|*}; [ ! -e "$live" ] || { echo "new file exists, nothing written: $live"; exit 2; }; done
restore(){ for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; cp "$base" "$live"; done; for row in "${NEWF[@]}"; do live=${row%%|*}; rm -f "$live"; rmdir "$(dirname $live)" 2>/dev/null; done; echo "restored"; for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || echo "RESTORE DIFFERS $live"; done; }
for row in "${FILES[@]}"; do live=${row%%|*}; stage=${row##*|}; cp "$stage" "$live" || { restore; exit 3; }; done
for row in "${NEWF[@]}"; do live=${row%%|*}; src=${row##*|}; mkdir -p "$(dirname $live)"; cp "$src" "$live" || { restore; exit 3; }; done
[ -n "$R211D_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
DIRS=("$O/🧪️tests/📥️reuse|0" "$N/📥️take/🧪️tests/🧮️stale-fleet-told|0" "$N/📥️take/🧪️tests/🧐️stale-why|0" "$N/📥️take/🧪️tests/🔎️stale-row|0" "$N/📥️take/🧪️tests/🔬️stale-line|0" "$N/📥️take/🧪️tests/🔒️decided|0" "$N/📥️take/🧪️tests/🎣️fetch-first|0" "$N/📥️take/🧪️tests|0")
for row in "${DIRS[@]}"; do d=${row%%|*}; want=${row##*|}
 cd "$d"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1)
 echo "${d#$B/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr '\n' ' ')"
 echo "$out"|grep -qE "^ *$want fail" || { echo "$out"|tail -30; restore; exit 3; }
done
cd "$ROOT" && $BUN -e 'await import("'"$N"'/🟨️.mjs");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
