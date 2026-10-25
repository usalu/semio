#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; O="$C/🗑️generated/dtest-d"
A="🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts"
SEMIO_TEST_ARTIFACT_DIR="$PWD/$O/art" nice -n 10 bun test "./$A" --timeout 300000 > "$O/$1.log" 2>&1
echo "rc=$?" >> "$O/$1.log"
