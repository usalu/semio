#!/bin/bash
ROOT=/Users/ueli/Documents/semio
BUN=$ROOT/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
H="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts"
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R150
mkdir -p $S/art; cd $ROOT
env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN SEMIO_TEST_ARTIFACT_DIR=$S/art "$BUN" "$H" --file="${1:-$T/🧰️scripts/R150/bench-stage.test.ts}" --birth=setup --root="$ROOT" 2>&1 | grep -E "\[BENCH\]|^\(pass|^\(fail|error:|^ *[0-9]+ (pass|fail)" | cut -c1-2500
