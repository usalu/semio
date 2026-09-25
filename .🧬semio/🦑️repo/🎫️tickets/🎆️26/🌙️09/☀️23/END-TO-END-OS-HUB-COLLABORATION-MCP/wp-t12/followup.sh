#!/bin/zsh
# 🔁️ T12 afternoon follow-up: every check the 13:xx–15:xx edits owe, sequential, one cargo at a time.
# Each step logs to `.🧬semio/🌐hub/s11-t12-captures/followup-<step>.txt` with `EXIT=<code> SECONDS=<wall>`;
# progress goes to `followup.txt` beside them. usage: zsh followup.sh [first-step]
cd /Users/ueli/Documents/semio || exit 1
OUT=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s11-t12-captures
T12=/Users/ueli/Documents/semio/.tmp-ticket/wp-t12
PRIVATE=$T12/target
export CARGO_INCREMENTAL=0 NX_DAEMON=false
from=${1:-quick-gis}; go=0

step() {
  local name=$1; shift
  [ "$name" = "$from" ] && go=1
  [ $go -eq 1 ] || return 0
  echo "START $name $(date '+%T')" >> "$OUT/followup.txt"
  local start=$(date +%s)
  "$@" > "$OUT/followup-$name.txt" 2>&1
  local code=$?
  echo "EXIT=$code SECONDS=$(( $(date +%s) - start ))" >> "$OUT/followup-$name.txt"
  echo "END $name EXIT=$code $(date '+%T')" >> "$OUT/followup.txt"
}

lib() { env CARGO_TARGET_DIR="$PRIVATE" cargo test -p "$1" --lib --no-fail-fast; }
case_run() { env SEMIO_TEST_LEVEL=exhaustive bun nx run "@semio-tech/repo-test-domain:$1" --outputStyle=stream -- --case "$2" ${3:+--implementation} ${3:-}; }
engine=✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.toml
pairs=$T12/generated/pdf-regen

step quick-gis zsh -c "cd ✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust && CARGO_TARGET_DIR='$PRIVATE' bun ./📜️script.ts test quick"
step lib-curation lib semio-s-artifact-sourcing-curation
step lib-writer lib semio-s-artifact-writer-writer
step lib-gisterrain lib semio-s-artifact-gis-gisterrain
step lib-space lib semio-s-artifact-space-space
step lib-layout lib semio-s-artifact-layout-layout
step case-layout case_run test-parity 📐️mutate-layout-1
step case-ui-preferences case_run test-parity 🎨️mutate-os-config-ui-preferences
step case-identity case_run test-parity 🎚️mutate-os-config-identity
step case-remodel case_run test-parity 📸️mutate-remodeling-1
step pdf-engine zsh -c "env CARGO_TARGET_DIR='$PRIVATE' cargo build --release --offline --manifest-path '$engine' && rm -rf '$pairs' && mkdir -p '$pairs' && '$PRIVATE/release/generate' '$pairs' && diff -rq '$pairs' ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧫️fixtures | /usr/bin/grep -v 'Only in ✏️s' ; echo reproducible-check-done"
step inventory bun nx run @semio-tech/repo-test-domain:test-inventory --outputStyle=stream
step contract bun nx run @semio-tech/repo-test-domain:test-contract --outputStyle=stream
step contract-rows cp .🧬semio/🦑️repo/⚡️cache/breaches/testing.json "$OUT/followup-contract-rows.json"
echo "ALL_DONE $(date '+%T')" >> "$OUT/followup.txt"
