#!/bin/sh
# 🛰️ Staged batch `nl-reapi` (lane nolegacy, audit11 copy): the remote executor advertises REAPI 2.1-2.3, reads only `output_paths`, refuses a 2.0 command (only `output_files`/`output_directories`) and, as REAPI 2.1 demands, ignores the deprecated fields when `output_paths` is used.
# usage: VERIFY_ROOT=<repository or mirror root> sh verify.sh   (stage state, Bun parse-check via gate-g-tools/scan.ts, docstring/emoji/[DEBUG] censuses and the language law on the batch files via compliance-tools/verify_batch.py, then the batch's laws)
set -e
here=$(cd "$(dirname "$0")" && pwd)
c=$(cd "$here/../../.." && pwd)
root=${VERIFY_ROOT:?VERIFY_ROOT required}
python3 -I "$here/stage.py" check --root "$root"
python3 -I "$c/compliance-tools/verify_batch.py" "$here" "$root"
cd "$root"
bun test --timeout 120000 './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/🟦️.ts' './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/📜️scenarios/🟦️.ts'
