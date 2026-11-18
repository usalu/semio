#!/bin/zsh
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes"
bad=0
for d in $B/🏊️project-farm/🧪️tests/{🧊️alone-fresh,🧯️cap,🥇️first-child} $B/🧪️tests/{🐘️never-here,🩻️heavy-refusal,🪂️set-aside}; do
  out=$(cd $d && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST $BUN test ./🟦️.ts --timeout 180000 2>&1 | grep -E "^ *[0-9]+ fail")
  echo "$(basename $d): $out"; case "$out" in *" 0 fail"*) ;; *) bad=1;; esac
done
exit $bad
