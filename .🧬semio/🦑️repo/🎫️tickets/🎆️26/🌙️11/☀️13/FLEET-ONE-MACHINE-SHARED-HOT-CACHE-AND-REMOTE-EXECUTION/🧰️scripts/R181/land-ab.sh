#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R181
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
D="$L/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
rows=("${(@f)$(cat $S/land-ab.tsv)}")
for r in $rows; do live=${r%%$'\t'*}; rest=${r#*$'\t'}; base=${rest#*$'\t'}
  if [ -n "$base" ]; then [ "$(shasum -a 256 < $base)" = "$(shasum -a 256 < $live)" ] || { echo "live file moved: $live"; exit 2; }; else [ ! -e "$live" ] || { echo "new file exists: $live"; exit 2; }; fi; done
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 170 ] || { echo "no lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R181 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
restore(){ for r in $rows; do live=${r%%$'\t'*}; rest=${r#*$'\t'}; base=${rest#*$'\t'}; if [ -n "$base" ]; then cp "$base" "$live"; else rm -f "$live"; rmdir "$(dirname $live)" 2>/dev/null; fi; done; echo "restored $(date -u +%H:%M:%S)"; }
fail(){ echo "RED: $1"; restore; release; exit 3; }
for r in $rows; do live=${r%%$'\t'*}; rest=${r#*$'\t'}; st=${rest%%$'\t'*}; mkdir -p "$(dirname $live)"; cp "$st" "$live" || fail copy; done
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 SEMIO_TEST_ARTIFACT_DIR=$S/artifacts
run(){ (cd "$1" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test "./🟦️.ts" --timeout 120000 2>&1) }
a=$(run "$D/🧪️tests/🔖️task-semantics"); echo "$a" | grep -a "^ *[0-9]* \(pass\|fail\)" | tr '\n' ' '; echo "$a" | grep -aq "^ *0 fail" || fail "task-semantics"
b=$(run "$L/🏗️build/🧪️tests/🕸️graph/🔌️nx-engine"); echo "$b" | grep -a "^ *[0-9]* \(pass\|fail\)" | tr '\n' ' '; echo "$b" | grep -aq "^ *0 fail" || fail "nx-engine"
d=$(run "$L/🏗️build/👁️observation/🧪️tests/🔒️loaded-implementation"); echo "$d" | grep -a "^ *[0-9]* \(pass\|fail\)" | tr '\n' ' '; echo "$d" | grep -aq "^ *0 fail" || fail "loaded-implementation"
for g in "🧊️stale-facts" "🌊️seal-under-churn" "🩹️repair-rounds"; do e=$(run "$L/🏗️build/🧪️tests/🕸️graph/$g"); echo "$e" | grep -aq "^ *0 fail" || fail "$g"; done
f=$(run "$L/🏗️build/👁️observation/🧪️tests/🔂️validation-round"); echo "$f" | grep -aq "^ *0 fail" || fail "validation-round"
echo "GREEN $START $(date -u +%H:%M:%S)"; release
