#!/bin/zsh
# R210 landing: selected show project + sealed-only whole questions. Test mode: R210_ROOT=<mirror> R210_NOWINDOW=1 [R210_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R210_ROOT:-$REAL}
T=${R210_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R210
B="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; G="$B/🕸️graph"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("$G/🕰️generations/🔌️load/🟨️.mjs|$S/stage/load.base|$S/stage/load.new.mjs"
"$G/🕰️generations/🔌️load/🟨️.d.mts|$S/stage/loaddts.base|$S/stage/loaddts.new.mts"
"$G/🕰️generations/🧬️schema/🔣️.json|$S/stage/schema.base|$S/stage/schema.new.json"
"$G/🚪️admission/🟦️.ts|$S/stage/adm.base|$S/stage/adm.new.ts"
"$B/⌨️command/⚙️runtime/🟦️.ts|$S/stage/rt.base|$S/stage/rt.new.ts"
"$G/🕰️host/🟦️.ts|$S/stage/host.base|$S/stage/host.new.ts")
h(){ shasum -a 256 < "$1"; }
if [ -z "$R210_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R210 show-project-selected $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || { echo "live differs from base, nothing written: $live"; exit 2; }; done
restore(){ for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; cp "$base" "$live"; done; echo "restored"; for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || echo "RESTORE DIFFERS $live"; done; }
for row in "${FILES[@]}"; do live=${row%%|*}; stage=${row##*|}; cp "$stage" "$live" || { restore; exit 3; }; done
[ -n "$R210_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
DIRS=("$G/🚪️admission/🧪️tests/🔎️show-project-scope|0" "$G/🕰️generations/🔌️load/🧪️tests/🔒️current-never-builds|0" "$G/🕰️generations/🔌️load/🧪️tests|1" "$G/🚪️admission/🧪️tests|2" "$G/🕰️generations/🧪️tests|0" "$G/🕰️generations/🔌️load/🧪️tests/🪑idle-builder|2" "$G/🕰️generations/🔌️load/🧪️tests/🚧️withheld-dependency|0" "$B/🏡️resident/🧪️tests/🤝️slice-share|0")
for row in "${DIRS[@]}"; do d=${row%%|*}; want=${row##*|}
 cd "$d"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1)
 echo "${d#$B/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr '\n' ' ')"
 echo "$out"|grep -qE "^ *$want fail" || { echo "$out"|tail -30; restore; exit 3; }
done
cd "$ROOT" && $BUN -e 'await import("'"$G"'/🕰️generations/🔌️load/🟨️.mjs");await import("'"$G"'/🚪️admission/🟦️.ts");await import("'"$G"'/🕰️host/🟦️.ts");await import("'"$B"'/⌨️command/⚙️runtime/🟦️.ts");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
