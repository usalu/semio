#!/bin/sh
# [gram4-g] verify of batch shape-keys AFTER apply (run it inside a mirror root: `python3 -I C/mirror-tools/mirror.py run <name> -- sh <this>`): the claim law (3 tests: fixture schema, equality with the Python oracle on 60 seeds, set insertions bounded by the rows) and the existing load suite (68 tests).
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests'
BUN=${BUN:-bun}
$BUN test "./$L/🗝️claim-keys/🟦️.ts" "./$L/🟦️.ts" 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)"
