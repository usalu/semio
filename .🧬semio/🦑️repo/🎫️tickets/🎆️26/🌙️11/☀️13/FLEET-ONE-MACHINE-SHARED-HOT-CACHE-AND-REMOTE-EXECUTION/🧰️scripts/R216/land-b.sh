#!/bin/zsh
# R216 landing B: adoption counters, adopt lane without footprint gate, look before a local inference of a listed owner. Test mode: R216_ROOT=<scratch root> R216_NOWINDOW=1 [R216_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R216_ROOT:-$REAL}
T=${R216_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R216
TSV=${R216_TSV:-$T/🧰️scripts/R216/land-b.tsv}
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
h(){ shasum -a 256 < "$1" | cut -c1-64; }
if [ -z "$R216_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; [ $mm -ge 5 ] && [ $mm -lt 28 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R216 adopt-before-infer $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
while IFS=$'\t' read -r kind rel; do
 live="$ROOT/$rel"
 if [ "$kind" = M ]; then [ "$(h "$S/base/$rel")" = "$(h "$live")" ] || { echo "live differs from base, nothing written: $live"; exit 2; }
 else [ ! -e "$live" ] || { echo "new file exists, nothing written: $live"; exit 2; }; fi
done < "$TSV"
PN="$ROOT/$B/🕸️graph/📥️inference/📦️project-nodes"
restore(){ while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; if [ "$kind" = M ]; then cp "$S/base/$rel" "$live"; else rm -f "$live"; fi; done < "$TSV"; rmdir "$PN/📥️take/🧪️tests/📊️adopt-counters" "$PN/📥️take/🧪️tests/🥇️before-infer" 2>/dev/null; echo "restored"; while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; if [ "$kind" = M ]; then [ "$(h "$S/base/$rel")" = "$(h "$live")" ] || echo "RESTORE MISMATCH $live"; else [ ! -e "$live" ] || echo "RESTORE MISMATCH $live"; fi; done < "$TSV"; }
while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; mkdir -p "$(dirname "$live")"; cp "$S/new/$rel" "$live" || { restore; exit 3; }; done < "$TSV"
[ -n "$R216_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
TK="$PN/📥️take/🧪️tests"; T2="$PN/🧪️tests"
FILES=("$TK/📊️adopt-counters/🟦️.ts" "$TK/🥇️before-infer/🟦️.ts" "$TK/🎣️fetch-first/🟦️.ts" "$TK/🎣️stale-backoff/🟦️.ts" "$TK/📣️published/🟦️.ts" "$TK/🔎️stale-row/🟦️.ts" "$TK/🔒️decided/🟦️.ts" "$TK/🔥️hot-lane/🟦️.ts" "$TK/🔬️stale-line/🟦️.ts" "$TK/🚀️release/🟦️.ts" "$TK/🚦️no-progress/🟦️.ts" "$TK/🟦️.ts" "$TK/🤝️two-machines/🟦️.ts" "$TK/🧐️stale-why/🟦️.ts" "$TK/🧮️stale-fleet-told/🟦️.ts" "$TK/🪧️tail-report/🟦️.ts" "$T2/🏷️owner-tag/🟦️.ts" "$T2/🐌️slow-last/🟦️.ts" "$T2/🐘️never-here/🟦️.ts" "$T2/📈️inferred/🟦️.ts" "$T2/📥️take/🟦️.ts" "$T2/🔁️retry-alone/🟦️.ts" "$T2/🔭️look-job/🟦️.ts" "$T2/🚦️governed-dispatch/🟦️.ts" "$T2/🚧️read-parent/🟦️.ts" "$T2/🚪️away/🟦️.ts" "$T2/🛑️too-heavy/🟦️.ts" "$T2/🛣️lanes/🟦️.ts" "$T2/🤝️pool-admission/🟦️.ts" "$T2/🤝️selected-fleet/🟦️.ts" "$T2/🧭️parent-routes/🟦️.ts" "$T2/🧭️stage-names/🟦️.ts" "$T2/🧮️aside-census/🟦️.ts" "$T2/🧮️known-unit/🟦️.ts" "$T2/🧵️ready-context/🟦️.ts" "$T2/🧹️parent-budget/🟦️.ts" "$T2/🧾️poll-ledger/🟦️.ts" "$T2/🩻️heavy-refusal/🟦️.ts" "$T2/🪂️set-aside/🟦️.ts" "$T2/🪘️refs-file/🟦️.ts" "$T2/🫥️self-progress/🟦️.ts" "$T2/⏳️verdict-wait/🟦️.ts" "$T2/🌍️bigger-host/🟦️.ts")
for f in "${FILES[@]}"; do
 cd "$(dirname "$f")"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "${f#$ROOT/$B/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
 echo "$out"|grep -q "^ *0 fail" || { echo "$out"|tail -30; restore; exit 3; }
done
cd "$ROOT" && $BUN -e 'await import("'"$PN"'/🟨️.mjs");await import("'"$PN"'/📥️take/🟨️.mjs");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
