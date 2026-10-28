#!/bin/sh
# 🧪 [audit10] control: two files with the identical base declaration (test:leases, artifacts) in one bare bun process, then each alone. usage: sh multifile-base.sh <root>
R="${1:?root}"; cd "$R" || exit 1
for set in "./🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️claims/🟦️.ts ./🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️frontier/🟦️.ts" "./🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️claims/🟦️.ts" "./🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️frontier/🟦️.ts"; do
 echo "[DEBUG] === $set"
 env -u SEMIO_SCRIPT_PROCESS_INVOCATION -u SEMIO_TEST_ARTIFACT_DIR nice -n 10 bun test --timeout 170000 $set 2>&1 </dev/null | grep -E "^ *[0-9]+ (pass|fail)$|admission closed" | sort | uniq -c | head -6
done
