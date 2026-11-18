#!/bin/bash
# usage: t.sh <test file path relative to the build dir or absolute> ...   runs each through the genuine-birth harness
cd /Users/ueli/Documents/semio || exit 1
BUN=$PWD/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
G=$PWD/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
BD=$G/🏗️build
for f in "$@"; do
 case "$f" in /*) p="$f";; *) p="$BD/$f";; esac
 out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN $G/🧪️testing/🌱️genuine-birth/📜️script.ts --file="$p" --birth=setup --root=/Users/ueli/Documents/semio 2>&1)
 echo "== ${p#$BD/}"; echo "$out" | grep -E "^\(fail\)|error:|^ +[0-9]+ (pass|fail)|expect|Expected|Received" | head -${LINES_MAX:-20}
done
