#!/bin/sh
# 🧪 [audit10] one bare bun process over two files that declare the same library birth (build-inputs-storage), then each alone. usage: sh multifile.sh <root>
R="${1:?root}"; cd "$R" || exit 1
for set in "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🎛️phase/🟦️.ts ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🚪️admission/🟦️.ts" "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🚪️admission/🟦️.ts ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🎛️phase/🟦️.ts" "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🎛️phase/🟦️.ts" "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🚪️admission/🟦️.ts"; do
 echo "[DEBUG] === $set"
 env -u SEMIO_SCRIPT_PROCESS_INVOCATION -u SEMIO_TEST_ARTIFACT_DIR nice -n 10 bun test --timeout 170000 $set 2>&1 </dev/null | grep -E "^ *[0-9]+ (pass|fail)$|^\(fail\)|ENOENT|error:" | head -12
done
