#!/usr/bin/env bash
# Usage: p3-5d-builder.sh <logfile> [pkg...]   (no pkg = all packages of 🗑️generated/p3-pkgs.txt)
# Queues one builder.sh exec run through builder.sh (it takes the lock and memory itself, BUILDER_MEM default 4500); one heavy run at a time.
set -uo pipefail
T="$(cd "$(dirname "$0")" && pwd)"; R="$(cd "$T/../../../../../../.." && pwd)"
LOG="$1"; shift
pk="${*:-$(cat "$T/🗑️generated/p3-pkgs.txt")}"
snippet="cd ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d || exit 9
for p in $pk; do
  extra=(); [ \"\$p\" = semio-s-artifact-puzzle-5d ] && extra=(--features component-app-assembly)
  echo \"=== TEST \$p ===\"
  cargo test -p \"\$p\" --all-targets \"\${extra[@]}\" --no-fail-fast --jobs 2 --offline
  echo \"TEST \$p:\$?\"
done
echo '=== TEST snapshot-sqlite ==='
cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib hoisted_snapshot_2 --no-fail-fast --jobs 2 --offline; echo \"TEST snapshot-sqlite:\$?\"
echo '=== TEST host-admission ==='
cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib puzzle5d_host_admission_ --no-fail-fast --jobs 2 --offline; echo \"TEST host-admission-root:\$?\"
cargo test -p semio-s-artifact-puzzle-5d-host-admission-tests --lib puzzle5d_host_admission_ --no-fail-fast --jobs 2 --offline; echo \"TEST host-admission-crate:\$?\"
for t in optional_part_and_grip_fields_round_trip_against_independent_json inline_import_matches_independent_json_capacity_boundary catalog_extensions_round_trip_against_independent_json created_part_board_and_world_edits_reopen_with_history import_identity_matches_neutral_fixtures_and_independent_json background_click_clears_the_world_pane_selection; do
  cargo test -p semio-s-artifact-puzzle-5d-v1-unit-tests --offline --jobs 2 --lib \"\$t\" -- --nocapture; echo \"VERIFY \$t:\$?\"
done
cargo test -p semio-s-artifact-puzzle-5d-host-admission-tests --offline --jobs 2 --lib a_catalogue_row_payload_is_admitted_by_the_board_and_the_world_ghost; echo \"VERIFY catalogue_row:\$?\""
cd "$R" && exec "$T/builder.sh" exec "$snippet" > "$LOG" 2>&1
