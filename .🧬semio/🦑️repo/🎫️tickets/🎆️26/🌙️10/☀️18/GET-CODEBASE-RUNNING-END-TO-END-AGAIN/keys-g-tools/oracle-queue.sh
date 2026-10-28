#!/bin/sh
# oracle-queue.sh <wait-pid> : runs the dashboard TS test oracles ONE AT A TIME after <wait-pid> ends (explicit ./ test paths: a bare path is a Bun filter that scans the whole disk)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G="$C/🗑️generated/keys-g"
D="🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard"
while kill -0 "$1" 2>/dev/null; do sleep 5; done
one() { t="$1"; shift; nice -n 10 python3 "$C/keys-g-tools/readoracle.py" --root "$PWD" --allow-list "$G/lists-after-d/_semio-tech_repo-dashboard-rs__test-$t.files" --allow-name package.json --allow-name tsconfig.json --allow-name bun.lock --cwd "." --rounds 20 --timeout 600 --out "$PWD/$G/ro-dashboard-$t" -- "$@" > "$G/ro-dashboard-$t.log" 2>&1; }
one language bun test --timeout 60000 "./$D/🧪️tests/🌐️language/🟦️.ts"
one dispatch bun test --timeout 60000 "./$D/🧪️tests/🎛️dispatch/🟦️.ts"
one source-freshness env SEMIO_TEST_ARTIFACT_DIR=/private/tmp/claude-501/keys-g-artifacts bun test "./$D/🧪️tests/🧊️execution/🟦️.ts" --timeout 120000 --test-name-pattern "source digest"
