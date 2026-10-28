#!/bin/sh
# FOR gate-g: run from the repo root AFTER `python3 C/rev3-g-tools/batch.py C/rev3-g-tools/staged/win-withheld apply .`
set -e
G="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🚧️unfounded-withheld"
( cd "$G/🪟️windows" && nice -n 10 bun test ./🟦️.ts )
( cd "$G" && nice -n 10 bun test ./🟦️.ts )
