#!/bin/sh
# The digest laws that need the repository envelope (new batch-snapshot law + the existing exact-source laws), through the real `repo exec`.
cd "$(dirname "$0")/../../../../../../../.."
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
D="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🧪️tests"
export SEMIO_TEST_ARTIFACT_DIR="$PWD/$C/🗑️generated/churn-g/art"
exec nice -n 10 bun repo --tool=semio exec --projects=workspace -- repo tools exec bun -- test "./$D/🔒️batch-snapshot/🟦️.ts" "./$D/🟦️.ts"
