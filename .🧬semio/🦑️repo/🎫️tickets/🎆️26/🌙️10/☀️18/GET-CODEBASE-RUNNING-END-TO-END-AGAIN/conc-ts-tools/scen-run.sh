#!/bin/sh
# usage: scen-run.sh <session> : one cold/warm check of the overlay plan with niceness 10, one tsc at a time
cd "$(dirname "$0")"
F="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/tsconfig.json"
nice -n 10 bun units-cli.ts check /private/tmp/conc-ts/ov-lib "$F" --out /private/tmp/conc-ts/run-lib6 --session "$1" --concurrency 1 --quiet 2>&1 | cut -c1-330
