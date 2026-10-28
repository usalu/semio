#!/bin/sh
# make.sh <work-root>: (re)builds every file of the envleak batch inside the work root (a mirror; idempotent) and cuts staged/ from it against the live tree.
set -e
HERE=$(cd "$(dirname "$0")" && pwd); WORK=$1
for step in build_module.py edit_witness.py edit_script.py build_tests.py edit_tests.py; do python3 -I "$HERE/$step" "$WORK"; done
R='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'; BZ="$R/🔧️tools/🏗️bazel"
rm -rf "$HERE/staged"; mkdir -p "$HERE/staged"
python3 -I "$HERE/stage-flat.py" make "$HERE/staged" "$WORK" \
 "$BZ/🌿️environment/🟦️.ts" "$BZ/🌿️environment/🧬️schema/🔣️.json" "$BZ/🌿️environment/🧪️tests/🟦️.ts" "$BZ/🌿️environment/🧪️tests/🧫️fixtures/🔣️.json" "$BZ/🌿️environment/🧪️tests/🧮️resources/🐍️oracle/🐍️.py" \
 "$BZ/📜️script.ts" "$BZ/🧪️tests/📥️fingerprint/🟦️.ts" "$BZ/🧪️tests/📥️fingerprint/🔣️ambient.json" "$BZ/🧪️tests/📥️fingerprint/🧬️ambient.json" \
 "$R/🔭️witness/🟨️.mjs" "$R/🔭️witness/🟨️.d.mts" "$R/🔭️witness/🧬️schema/🔣️.json" "$R/🔭️witness/🧪️tests/🟦️.ts" "$R/🗂️suites/🔣️.json"
rm -f "$HERE/staged/apply.sh" "$HERE/staged/check.sh" "$HERE/staged/revert.sh"
