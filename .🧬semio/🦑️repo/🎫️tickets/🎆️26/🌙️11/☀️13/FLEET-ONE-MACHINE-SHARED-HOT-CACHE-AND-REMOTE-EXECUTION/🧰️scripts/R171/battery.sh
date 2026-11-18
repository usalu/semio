#!/bin/zsh
# usage: battery.sh [preload-file]  (env R171_* passed through); prints "<dir>: <pass>/<fail>"
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
DIRS=("🧪️tests/🕸️graph/🔄️candidate" "🧪️tests/🕸️graph/🧲️adopt-registry" "🧪️tests/🕸️graph/🌊️seal-under-churn" "🧪️tests/🕸️graph/🩹️repair-rounds" "🧪️tests/🕸️graph/🧊️stale-facts" "🕸️graph/🧪️tests/🔪️slice-read" "🕸️graph/🧪️tests/🔓️sealed-reuse" "🕸️graph/📥️load/🧠️admission-memo/🧪️tests" "🕸️graph/📥️inference/🔖️stamps/🧪️tests" "🕸️graph/📥️inference/🔖️stamps/🧪️tests/🔧️tooling-edit" "🕸️graph/📥️load/🤝️selected-exchange/🧪️tests" "🕸️graph/📥️load/🤝️selected-exchange/🧪️tests/🔌️attempt" "🕸️graph/📥️load/🤝️selected-exchange/🧪️tests/🔗️wiring" "🕸️graph/📥️load/🤝️selected-exchange/🧪️tests/🖥️real-server" "🕸️graph/📥️load/🤝️selected-exchange/📏️revision/🧪️tests")
PRE=(); [ -n "$1" ] && PRE=(--preload "$1")
for d in $DIRS; do
 [ -d "$B/$d" ] || { echo "$d: absent"; continue; }
 out=$(cd "$B/$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts $PRE --timeout 180000 2>&1)
 p=$(echo "$out" | grep -E "^ *[0-9]+ pass" | awk '{print $1}'); f=$(echo "$out" | grep -E "^ *[0-9]+ fail" | awk '{print $1}')
 echo "$d: ${p:-?}/${f:-?}"
done
