#!/bin/zsh
setopt nobgnice
# usage: battery.sh <outfile> <base|stage> <listfile> [parallel]   one line per file: name Npass Mfail
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
D=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R136
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R136
cd /Users/ueli/Documents/semio || exit 1
out=$1; mode=$2; list=$3; par=${4:-4}
one(){ f=$1; P=(); [ "$mode" = stage ] && P=(--preload $D/overlay.ts); o=$(cd "$(dirname "$f")" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN SEMIO_TEST_ARTIFACT_DIR=${ART:-$SC/art-$mode} perl -e 'alarm shift; exec @ARGV' 240 $BUN test $P ./$(basename "$f") --timeout 120000 2>&1 | tail -6); p=$(echo "$o" | grep -E '^ *[0-9]+ pass' | awk '{print $1}'); fl=$(echo "$o" | grep -E '^ *[0-9]+ fail' | awk '{print $1}'); echo "${f#/Users/ueli/Documents/semio/🧰️framework/} ${p:-?} ${fl:-?}"; }
while read f; do one "$f" & while [ $(jobs -r | wc -l) -ge $par ]; do sleep 1; done; done < $list > $out; wait
