#!/bin/zsh
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
d="$1"; out="$2"; n=$(echo "$d" | md5 -q)
cd "$L/$d" || exit 1
env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN test ./🟦️.ts --timeout 180000 > "$out/$n.txt" 2>&1
echo "$d|$(grep -E '^ *[0-9]+ pass' $out/$n.txt | tr -d ' ')|$(grep -E '^ *[0-9]+ fail' $out/$n.txt | tr -d ' ')|$(grep '^(fail)' $out/$n.txt | cut -c1-90 | tr '\n' ';')"
