#!/bin/sh
# Wave 1127 H96: unpack the stage and run the four member tests against a scratch tree (no live files touched).
# usage: run-tests.sh <scratch-dir>   then, from the live checkout: repo land check <scratch-dir>/land ; repo land --wave "Wave 1127" --lane H96 --title "Feedback loop for agents" <scratch-dir>/land
set -eu
scratch=${1:?scratch directory}
repo=${SEMIO_ROOT:-$(cd "$(dirname "$0")/../../../../../../../../.." && pwd)}
mkdir -p "$scratch" && tar xzf "$(dirname "$0")/land.tgz" -C "$scratch"
dev="$scratch/dev"; mkdir -p "$dev"; cp -R "$scratch/land/tree/." "$dev/"
ln -sfn "$repo/node_modules" "$dev/node_modules"
ln -sfn "$repo/🧰️framework/🔨️modules" "$dev/🧰️framework/🔨️modules"
b="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
cd "$dev" && nice -n 10 bun test "./$b/📈️progress-rate/🧪️tests/🟦️.ts" "./$b/🧾️verdict/🧪️tests/🟦️.ts" "./$b/🔁️verdict-reuse/🧪️tests/🟦️.ts" "./$b/📋️queue-view/🧪️tests/🟦️.ts"
