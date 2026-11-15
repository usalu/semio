#!/bin/zsh
# usage: runtests.sh <root>
R=$1
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
B="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
for d in "🏡️resident/🧪️tests/📐️booking" "🏡️resident/🧪️tests/🛡️plan-guard" "🏡️resident/🧪️tests/🪓️plan-limit" "🏡️resident/🧪️tests/🌐️host" "🏡️resident/🧪️tests/🛑️cancel" "🏡️resident/🧪️tests/🛟️reservation" "🧠️memory/🧪️tests" "🧱️job-limits/🧪️tests" "🪪️graph-state/🙋️wanted/🧪️tests/🐧️linux-host" "🏡️resident/🧪️tests/🐧️leaf-ceiling"; do
  [ -d "$B/$d" ] || { echo "missing $d"; continue; }
  cd "$B/$d"
  for f in *.ts; do
   out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 $BUN test ./$f --timeout 180000 2>&1); rc=$?
   echo "rc=$rc $d/$f | $(echo "$out" | grep -aE '^ *[0-9]+ (pass|fail)' | tr '\n' ' ')"; echo "$out" | grep -a '^(fail)' | cut -c1-200
  done
done
