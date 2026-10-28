#!/bin/sh
# 🧮️ Staged batch `loader` (lane langbudget): the captured-command stream loader renders refusals from the authenticated snapshot instead of inlining the catalog; its command line stays bounded. Apply before nolegacy `lang` (langbudget `lang`).
# usage: VERIFY_ROOT=<repository or mirror root> sh verify.sh   (stage state, Bun parse-check, docstring/emoji/[DEBUG] censuses and the language law on the batch files, then the batch's laws; the argv law is RED on a state whose loader grew past the command-line bound)
set -e
here=$(cd "$(dirname "$0")" && pwd)
c=$(cd "$here/../../.." && pwd)
root=${VERIFY_ROOT:?VERIFY_ROOT required}
python3 -I "$here/stage.py" check --root "$root"
python3 -I "$c/compliance-tools/verify_batch.py" "$here" "$root"
cd "$root"
bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🧪️tests/🏷️loader-identity/🟦️.ts' './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧪️tests/🟦️.ts' './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🧭️sites/🟦️.ts' './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧾️diagnostics/🧪️tests/🟦️.ts' './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts'
