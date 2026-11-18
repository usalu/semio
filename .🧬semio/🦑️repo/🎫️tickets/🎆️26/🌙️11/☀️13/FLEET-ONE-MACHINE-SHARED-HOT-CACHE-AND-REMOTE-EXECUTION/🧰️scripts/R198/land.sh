#!/bin/zsh
# R198 landing: local unit key portable + task view executor. Test mode: R198_ROOT=<fake live root> R198_NOWINDOW=1 [R198_NOTESTS=1] [R198_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R198_ROOT:-$REAL}
T=${R198_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R198
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
U="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
h(){ shasum -a 256 < "$1"; }
if [ -z "$R198_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R198 local-key+task-view-executor $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
typeset -a NEWS MODS
while IFS=$'\t' read -r kind rel base; do
 if [ $kind = new ]; then NEWS+=("$rel"); else MODS+=("$rel|$S/base/$base"); fi
done < $S/land.tsv
for rel in "${NEWS[@]}"; do [ ! -e "$ROOT/$rel" ] || { echo "new file already exists, nothing written: $rel"; exit 2; }; done
for row in "${MODS[@]}"; do live=$ROOT/${row%%|*}; base=${row#*|}; [ "$(h $base)" = "$(h $live)" ] || { echo "live differs from base, nothing written: $live"; exit 2; }; done
restore(){ for rel in "${NEWS[@]}"; do rm -f "$ROOT/$rel"; done; rmdir "$ROOT/$LIB/🏞️task-view/🏃️executor/🧬️schema" 2>/dev/null
 for row in "${MODS[@]}"; do live=$ROOT/${row%%|*}; base=${row#*|}; cp "$base" "$live"; done; echo "restored"
 for row in "${MODS[@]}"; do live=$ROOT/${row%%|*}; base=${row#*|}; [ "$(h $base)" = "$(h $live)" ] || echo "RESTORE MISMATCH $live"; done
 for rel in "${NEWS[@]}"; do [ ! -e "$ROOT/$rel" ] || echo "RESTORE LEFTOVER $rel"; done; }
for rel in "${NEWS[@]}"; do mkdir -p "$(dirname "$ROOT/$rel")"; cp "$S/mirror/$rel" "$ROOT/$rel" || { restore; exit 3; }; done
for row in "${MODS[@]}"; do rel=${row%%|*}; cp "$S/mirror/$rel" "$ROOT/$rel" || { restore; exit 3; }; done
[ -n "$R198_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R198_NOTESTS" ] && { echo "WRITTEN (no tests) $START $(date -u +%H:%M:%S)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
DIRS=("$ROOT/$LIB/🏞️task-view/🏃️executor/🧪️tests" "$ROOT/$LIB/🏞️task-view/🧪️tests" "$ROOT/$U/🛰️fleet/🪪️identity/🧪️tests/🏠️local-key" "$ROOT/$U/🛰️fleet/🪪️identity/🧪️tests" "$ROOT/$U/🔏️key/🧪️tests" "$ROOT/$U/🛰️fleet/🧪️tests/🌳️rooted" "$ROOT/$U/🛰️fleet/🧪️tests/🧯️fallback")
for d in "${DIRS[@]}"; do
 cd "$d"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "${d#$ROOT/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
 echo "$out"|grep -q "^ *0 fail" || { echo "$out"|tail -30; restore; exit 3; }
done
cd "$ROOT" && $BUN -e 'await import("'"$ROOT/$LIB"'/🏞️task-view/🏃️executor/🟦️.ts");await import("'"$ROOT/$U"'/📜️script.ts");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
