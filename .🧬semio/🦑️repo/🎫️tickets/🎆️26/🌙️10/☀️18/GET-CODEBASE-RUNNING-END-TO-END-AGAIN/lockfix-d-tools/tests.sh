#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/lockfix-d"
export BUN_BIN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies"
for t in "🔌️managers/🍞️bun/🧪️tests" "🔒️lock-sync/🧪️tests" "🔌️managers/🧪️tests"; do
  nice -n 10 $BUN_BIN test "./$B/$t/🟦️.ts" > "$G/t-$(echo $t | md5 | cut -c1-6).log" 2>&1; echo "rc=$? $t" >> "$G/tests.rc"
done
echo done >> "$G/tests.rc"
