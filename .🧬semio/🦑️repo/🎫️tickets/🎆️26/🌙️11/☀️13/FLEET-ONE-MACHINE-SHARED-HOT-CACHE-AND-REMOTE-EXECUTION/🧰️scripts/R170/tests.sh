#!/bin/zsh
# usage: STAGED=<staged dir> [PRELOAD=1] tests.sh <test file under TESTDIR>...
D="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R170"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
O="${TESTDIR:-/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🧪️tests}"
cd "$O" || exit 1
pre=(); envs=(BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 "R170_STAGED=$STAGED")
[ -n "$PRELOAD" ] && { pre=(--preload "$D/preload.ts"); envs+=("SEMIO_TEST_CHILD_BUN_ARGUMENTS=--preload|$D/preload.ts"); }
for t in "$@"; do echo "== $t ${PRELOAD:+(staged)}"; env -u SEMIO_FLEET_GROUP $envs nice -n 10 $BUN test $pre $t --timeout 180000 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)|^error:" | head -20; done
