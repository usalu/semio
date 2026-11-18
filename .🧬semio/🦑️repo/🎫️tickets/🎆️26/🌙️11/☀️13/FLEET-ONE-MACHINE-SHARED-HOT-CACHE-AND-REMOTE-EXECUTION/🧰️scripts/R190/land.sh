#!/bin/zsh
# R190 landing: leases (bounded queue reclaim, typed admission refusal) + ready process-tree (spare the owner of a spared service, host descriptors)
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R190
F=/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process
P=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree
if [ -n "$R190_DRY" ]; then F=$S/dry/F; P=$S/dry/P; T=$S/dry/T; fi
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
LIVE=("$F/🔒️leases/🟦️.ts" "$F/🔒️leases/⏳️wait/🧬️schema/📜️policy.json" "$F/📖️texts/🔣️.json" "$P/🟦️.ts" "$P/🧹️containment/🟦️.ts" "$P/🧹️containment/🧬️schema/🔣️.json" "$P/🧪️tests/🔣️.json" "$P/🧪️tests/🐍️.py" "$P/🧹️containment/🧪️tests/🟦️.ts")
BASE=("$S/base/leases.ts" "$S/base/policy.json" "$S/base/texts.json" "$S/base/pt/tree.ts" "$S/base/pt/containment.ts" "$S/base/pt/containment-schema.json" "$S/base/pt/tree-fixture.json" "$S/base/pt/tree-oracle.py" "$S/base/pt/containment-test.ts")
STAGE=("$S/stage/leases.ts" "$S/stage/policy.json" "$S/stage/texts.json" "$S/stage/pt/tree.ts" "$S/stage/pt/containment.ts" "$S/stage/pt/containment-schema.json" "$S/stage/pt/tree-fixture.json" "$S/stage/pt/tree-oracle.py" "$S/stage/pt/containment-test.ts")
NEWDIR="$F/🔒️leases/🧪️tests/🪦️stale-queue"
sha(){ shasum -a 256 < "$1" | cut -c1-64; }
checkbase(){ for i in {1..9}; do [ "$(sha $LIVE[$i])" = "$(sha $BASE[$i])" ] || { echo "live moved: $LIVE[$i]"; return 1; }; done; [ ! -e "$NEWDIR" ] || { echo "new dir exists"; return 1; }; }
checkbase || exit 2
[ -n "$R190_DRY" ] || { mm=$(date -u +%M); { [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; } || { echo "outside window $mm"; exit 4; }; }
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R190 leases-bounded-reclaim+ready-spare-owner $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
trap release EXIT
restore(){ for i in {1..9}; do cp $BASE[$i] $LIVE[$i]; done; rm -rf "$NEWDIR"; local bad=0; for i in {1..9}; do [ "$(sha $LIVE[$i])" = "$(sha $BASE[$i])" ] || bad=1; done; [ ! -e "$NEWDIR" ] || bad=1; echo "restored $(date -u +%H:%M:%S) exact=$((1-bad))"; }
fail(){ echo "RED: $1"; restore; exit 3; }
checkbase || { echo "NOTHING WRITTEN: live differs from base after taking the lock"; exit 5; }
mkdir -p "$NEWDIR" || fail mkdir
for i in {1..9}; do cp $STAGE[$i] $LIVE[$i] || fail "copy $i"; done
cp "$S/stage/test/stale.test.ts" "$NEWDIR/🟦️.ts" || fail copy-test
for i in {1..9}; do [ "$(sha $LIVE[$i])" = "$(sha $STAGE[$i])" ] || fail "sha $i"; done
[ -z "$R190_FORCE_RED" ] || fail "forced red"
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 SEMIO_TEST_ARTIFACT_DIR=$S/art SEMIO_LEASE_REGISTRY_DIRECTORY=$S/reg
run(){ local dir=$1 file=$2; shift 2; local out; out=$(cd "$dir" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK nice -n 10 $BUN test ./$file "$@" --timeout 120000 2>&1); local p=$(echo "$out" | grep -E '^ *[0-9]+ pass$' | tr -d ' '); local f=$(echo "$out" | grep -E '^ *[0-9]+ fail$' | tr -d ' '); echo "$dir/$file: $p $f"; [ "$f" = "0fail" ] && [ -n "$p" ] || { echo "$out" | grep -E "^\(fail\)|error" | head -10; return 1; }; }
run "$NEWDIR" 🟦️.ts -t "wait policy|budget|dead holder|live slow" || fail stale-law
run "$F/📖️texts/🧪️tests" 🟦️.ts || fail texts
run "$F/🔒️leases/⏳️wait/🧪️tests" 🟦️.ts || fail wait
run "$P/🧪️tests" 🟦️.ts || fail tree
run "$P/🧹️containment/🧪️tests" 🟦️.ts || fail containment
END=$(date -u +%H:%M:%S); echo "GREEN $START $END"
