#!/bin/zsh
# usage: battery.sh OUTDIR LIST ; runs every directory of LIST (all test files inside) and writes summary.txt
OUT=$1; LIST=$2; mkdir -p $OUT; : > $OUT/summary.txt
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
while IFS= read -r d; do
  [ -z "$d" ] && continue
  dir=/Users/ueli/Documents/semio/$d
  for f in $(cd "$dir" && ls | grep -E '\.(ts|mjs)$' | grep -v '\.d\.'); do
    out=$(cd "$dir" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN test ./$f --timeout 180000 2>&1)
    p=$(echo "$out" | grep -E '^ *[0-9]+ pass$' | tail -1 | awk '{print $1}'); fl=$(echo "$out" | grep -E '^ *[0-9]+ fail$' | tail -1 | awk '{print $1}')
    echo "${d##*/build/}/$f pass=${p:-0} fail=${fl:-0}" >> $OUT/summary.txt
  done
done < $LIST
cat $OUT/summary.txt
