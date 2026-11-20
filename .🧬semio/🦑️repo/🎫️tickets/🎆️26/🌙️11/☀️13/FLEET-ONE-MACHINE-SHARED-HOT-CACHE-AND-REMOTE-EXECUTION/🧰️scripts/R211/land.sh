#!/bin/zsh
# R211 landing: refresh read names policy fresh; analysis rows judged by parser role+revision; one machine-local predicate for every fs row kind. Test mode: R211_ROOT=<mirror> R211_NOWINDOW=1 [R211_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R211_ROOT:-$REAL}
T=${R211_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R211
B="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; O="$B/👁️observation"; RD="$B/🌐️remote-execution/🛰️sidecar"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("$O/🟨️.mjs|$S/stage/obs.base.mjs|$S/stage/obs.new.mjs"
"$O/📂️source-names/🟨️.mjs|$S/stage/sn.base.mjs|$S/stage/sn.new.mjs"
"$O/📂️source-names/🧬️schema/🔣️.json|$S/stage/sch.base.json|$S/stage/sch.new.json"
"$RD/🕸️graph/🔭️read/🟦️.ts|$S/stage/read.base.ts|$S/stage/read.new.ts"
"$O/🧪️tests/🧐️analysis-reason/🟦️.ts|$S/stage/ar.base.ts|$S/stage/ar.new.ts")
NEWF=("$O/🧪️tests/🪪️parser-identity/🟦️.ts|$S/law/parser-identity/test.ts"
"$O/📂️source-names/🧪️tests/🤖️machine-local-paths/🟦️.ts|$S/law/machine-local-paths/test.ts"
"$RD/🧪️tests/🕸️graph/🌱️fresh-policy/🟦️.ts|$S/law/fresh-policy/test.ts")
h(){ shasum -a 256 < "$1"; }
if [ -z "$R211_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 1; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R211 fresh-policy-parser-identity-machine-local $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || { echo "live differs from base, nothing written: $live"; exit 2; }; done
for row in "${NEWF[@]}"; do live=${row%%|*}; [ ! -e "$live" ] || { echo "new file exists, nothing written: $live"; exit 2; }; done
restore(){ for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; cp "$base" "$live"; done; for row in "${NEWF[@]}"; do live=${row%%|*}; rm -f "$live"; rmdir "$(dirname $live)" 2>/dev/null; done; echo "restored"; for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || echo "RESTORE DIFFERS $live"; done; }
for row in "${FILES[@]}"; do live=${row%%|*}; stage=${row##*|}; cp "$stage" "$live" || { restore; exit 3; }; done
for row in "${NEWF[@]}"; do live=${row%%|*}; src=${row##*|}; mkdir -p "$(dirname $live)"; cp "$src" "$live" || { restore; exit 3; }; done
[ -n "$R211_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
DIRS=("$O/🧪️tests/🪪️parser-identity|0" "$O/📂️source-names/🧪️tests/🤖️machine-local-paths|0" "$RD/🧪️tests/🕸️graph/🌱️fresh-policy|0" "$O/🧪️tests/🧐️analysis-reason|0" "$O/🧪️tests/🔎️stale-row|0" "$O/🧪️tests/🤝️adopted-verdict|0" "$O/🧪️tests/⏱️analysis-stamp|0" "$O/🧪️tests/🚧️final-verdict|0" "$O/🧪️tests/🔎️lookup-holds|0" "$O/🧪️tests/🧩️field|2" "$O/🧪️tests/🧾️record-header|0" "$O/📂️source-names/🧪️tests/🕳️hollow|0" "$O/📂️source-names/🧪️tests/🤖️machine-local-dirs|0" "$O/🧪️tests/🔂️validation-round|0" "$O/🧪️tests/📥️reuse|0" "$O/🧪️tests/⚡️incremental|1" "$O/🧪️tests/🆔️record-revision|1" "$O/🧪️tests/🌊️owner-scale|1" "$O/🧪️tests/📦️projects|2" "$RD/🧪️tests/🕸️graph/🔭️read|0")
for row in "${DIRS[@]}"; do d=${row%%|*}; want=${row##*|}
 cd "$d"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1)
 echo "${d#$B/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr '\n' ' ')"
 echo "$out"|grep -qE "^ *$want fail" || { echo "$out"|tail -30; restore; exit 3; }
done
cd "$ROOT" && $BUN -e 'await import("'"$O"'/🟨️.mjs");await import("'"$O"'/📂️source-names/🟨️.mjs");await import("'"$RD"'/🕸️graph/🔭️read/🟦️.ts");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
