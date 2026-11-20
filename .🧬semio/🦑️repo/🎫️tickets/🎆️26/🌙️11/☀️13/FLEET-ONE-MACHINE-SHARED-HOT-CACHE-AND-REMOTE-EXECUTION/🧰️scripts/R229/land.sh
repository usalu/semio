#!/bin/zsh
# R229 landing: measurement lines of a stage reach the origin log mirror; restore width lever (default unchanged). Test mode: R229_ROOT=<scratch root> R229_NOWINDOW=1 R229_NOTEST=1 R229_NOWAIT=1 R229_LOCKBASE=<dir> [R229_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R229_ROOT:-$REAL}
T=${R229_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
K=$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R229
RD="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready"
NB="$ROOT/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("$RD/🟦️.ts|$K/ready.base.ts.txt|$K/ready.new.ts.txt" "$RD/🧬️schema/🔣️.json|$K/ready-schema.base.json.txt|$K/ready-schema.new.json.txt" "$NB/🟦️.ts|$K/native.base.ts.txt|$K/native.new.ts.txt")
NEWF=("$RD/🧪️tests/📏️measurement-lines/🟦️.ts|$K/measurement-test.new.ts.txt" "$NB/🧱️restore-width/🟦️.ts|$K/width.new.ts.txt" "$NB/🧱️restore-width/🧬️schema/🔣️.json|$K/width-schema.new.json.txt" "$NB/🧱️restore-width/🧪️tests/🟦️.ts|$K/width-test.new.ts.txt")
h(){ shasum -a 256 < "$1" | cut -c1-64; }
inwin(){ mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; }
if [ -z "$R229_NOWINDOW" ]; then until inwin; do mm=$((10#$(date -u +%M))); [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ -n "$R229_NOWINDOW" ] || inwin || { echo "window closed while waiting for the lock"; exit 4; }; [ $waited -lt 240 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R229 measurement-lines $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
checkbases(){ for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || { echo "live differs from base, nothing (more) written: $live"; return 1; }; done; }
checkbases || exit 2
for row in "${NEWF[@]}"; do f=${row%%|*}; [ -e "$f" ] && { echo "new file exists, nothing written: $f"; exit 2; }; done
restore(){ for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || cp "$base" "$live"; done; for row in "${NEWF[@]}"; do f=${row%%|*}; rm -f "$f"; done; rmdir "$RD/🧪️tests/📏️measurement-lines" "$NB/🧱️restore-width/🧪️tests" "$NB/🧱️restore-width/🧬️schema" "$NB/🧱️restore-width" 2>/dev/null; echo "restored"; for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || echo "RESTORE MISMATCH $live"; done; }
WROTE=0
for row in "${NEWF[@]}"; do f=${row%%|*}; src=${row#*|}; mkdir -p "${f%/*}" && cp "$src" "$f" || { restore; exit 3; }; done
echo "new files written $(date -u +%H:%M:%S); waiting 120 s for the replicas"
[ -n "$R229_NOWAIT" ] || sleep 120
inwin || [ -n "$R229_NOWINDOW" ] || { for row in "${NEWF[@]}"; do rm -f "${row%%|*}"; done; rmdir "$RD/🧪️tests/📏️measurement-lines" "$NB/🧱️restore-width/🧪️tests" "$NB/🧱️restore-width/🧬️schema" "$NB/🧱️restore-width" 2>/dev/null; echo "window ended during the wait, new files removed, modified files untouched"; exit 4; }
checkbases || { restore; exit 2; }
for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; new=${rest##*|}; cp "$new" "$live" || { restore; exit 3; }; done
[ -n "$R229_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R229_NOTEST" ] && { echo "GREEN(no tests) $START $(date -u +%H:%M:%S)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
ENVU=(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN)
for d in "$RD/🧪️tests/📏️measurement-lines" "$RD/🧪️tests/🧷️failure-log" "$RD/🧪️tests/🪜️stage-plan" "$RD/🧪️tests/🎯️probes" "$NB/🧱️restore-width/🧪️tests"; do cd "$d" || { restore; exit 3; }; out=$($ENVU nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "${d##*/🌐️remote-execution/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"; echo "$out"|grep -q "^ *0 fail" || { echo "$out"|tail -25; restore; exit 3; }; done
cd "$ROOT" && $BUN build --no-bundle "$RD/🟦️.ts" >/dev/null && $BUN build --no-bundle "$NB/🟦️.ts" >/dev/null && $BUN -e 'const m=await import("'"$RD"'/🟦️.ts");console.log("import ok",Object.keys(m).length,typeof m.readyMeasurementLinesV1);const w=await import("'"$NB"'/🧱️restore-width/🟦️.ts");console.log("width ok",typeof w.restoreWidthV1)' || { echo "smoke failed"; restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
