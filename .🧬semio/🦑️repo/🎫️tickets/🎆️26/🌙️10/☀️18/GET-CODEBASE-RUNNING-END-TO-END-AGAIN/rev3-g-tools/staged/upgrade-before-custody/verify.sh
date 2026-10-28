#!/bin/sh
# FOR gate-g: run from the repo root AFTER `python3 C/rev3-g-tools/batch.py C/rev3-g-tools/staged/upgrade-before-custody apply .`
set -e
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
( cd "$B/🕸️graph/🧪️tests/🔼️upgrade-before-custody" && nice -n 10 bun test ./🟦️.ts )
( cd "$B/🕸️graph/🧪️tests/🕰️generation-protocol" && nice -n 10 bun test ./🟦️.ts )
( cd "$B/🌡️host-capacity/🧪️tests/🪜️phases" && nice -n 10 bun test ./🟦️.ts )
( cd "$B/🌡️host-capacity/🧪️tests/🔼️upgrade" && nice -n 10 bun test ./🟦️.ts )
