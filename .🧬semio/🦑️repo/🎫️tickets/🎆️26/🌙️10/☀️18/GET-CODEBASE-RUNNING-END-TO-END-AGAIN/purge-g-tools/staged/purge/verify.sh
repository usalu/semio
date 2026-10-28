#!/bin/sh
# Laws of the purge batch (single-file suites, graph-free). Run from anywhere AFTER `python3 -I purge-g-tools/stage.py apply`.
# Known baseline reds that are NOT this batch: ticket-protection "Windows checkout ticket paths", root-clean-scaffold-source "scaffolds through explicit physical owner paths".
set -u
R=/Users/ueli/Documents/semio; cd "$R"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
A=$(mktemp -d); export SEMIO_TEST_ARTIFACT_DIR="$A"
bad=0
for f in \
  "$L/🧼️workspace-cleanup/🔍️candidate-discovery/🧪️tests/🟦️.ts" \
  "$L/🧼️workspace-cleanup/🗑️removal/🧪️tests/🟦️.ts" \
  "$L/🧼️workspace-cleanup/🛡️protection/🧪️tests/🟦️.ts" \
  "$L/🧼️workspace-cleanup/🎮️command/🧪️tests/🟦️.ts" \
  "$L/🧪️tests/🧹️ticket-generated-purge/🟦️.ts" \
  "$L/🧪️tests/🧹️clean-ticket-runs/🟦️.ts" \
  "$L/🧪️tests/🔬️workspace-contract/🏪️managed-storage-protection/🟦️.ts" \
  "$L/🧪️tests/🔬️workspace-contract/🎫️ticket-protection/🟦️.ts" \
  "$L/🧪️tests/🧱️root-clean-scaffold-source/🟦️.ts"; do
  printf '%s\n' "== $f"; bun test "./$f" 2>&1 | grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)$' || bad=1
done
python3 -I "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧪️tests/🧹️purge-generated/🐍️.py" check || bad=1
rm -rf "$A"; exit $bad
