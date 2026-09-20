#!/usr/bin/env zsh
# 🧪️ Slice DS1 session 5 — clears §9 gaps 1 and 2 in one detached supervisor, ONE cargo at a time.
#   1. builds the `semio-framework --lib` test binary with `--features typegen` (one binary serves
#      both the two new manifest laws and the `exports_typescript_bindings` schema export),
#   2. runs the three law filters,
#   3. regenerates the TypeScript twin through the repo's OWN generator verb
#      (`bun ./📜️script.ts generate` in 🧰️framework/📦️packages/🦀️rust) — never a hand edit.
# Retries only while the build dies inside a PEER's crate (the shared tree is edited concurrently);
# an error in a DS1-owned file stops the supervisor so the failure is read, not looped over.
set -u
ROOT=/Users/ueli/Documents/semio
GEN="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated"
CRATE="$ROOT/🧰️framework/📦️packages/🦀️rust"
BUILD="$GEN/ds1-framework-manifest-laws.txt"
LAWS="$GEN/ds1-law-run.txt"
GENOUT="$GEN/ds1-schema-generate.txt"
export NX_DAEMON=false
# ⚡️ `.cargo/config.toml` states it in its own words: "A private `CARGO_TARGET_DIR` only diverts the
# small uplifted deliverables; intermediates stay shared." The `build-dir` (shared, `fine-grain-locking`)
# is what holds the compiled units; the ARTIFACT directory is the single exclusive lock ~30 concurrent
# fleet cargos serialize on ("Blocking waiting for file lock on artifact directory", 30 min and counting
# for DS1 on 2026-09-20). Diverting only the uplift keeps every shared intermediate and drops that wait.
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-ds1"

cd "$CRATE" || exit 1
attempt=0
while [ $attempt -lt 30 ]; do
  attempt=$((attempt + 1))
  echo "=== build attempt $attempt at $(date -Iseconds) ===" >> "$BUILD"
  cargo test -p semio-framework --features typegen --lib --no-run >> "$BUILD" 2>&1
  rc=$?
  if [ $rc -eq 0 ]; then
    echo "=== build ok at $(date -Iseconds) ===" >> "$BUILD"
    break
  fi
  echo "=== build failed (rc=$rc) at $(date -Iseconds) ===" >> "$BUILD"
  sleep 90
done
[ $rc -eq 0 ] || exit $rc

: > "$LAWS"
for filter in window_kind_actions_join_the_app_roster_without_copying_it no_action_definition_is_stored_twice_inside_one_app resolve_window_actions; do
  echo "=== $filter ===" >> "$LAWS"
  cargo test -p semio-framework --features typegen --lib "$filter" -- --nocapture >> "$LAWS" 2>&1
  echo "=== exit $? ===" >> "$LAWS"
done

: > "$GENOUT"
echo "=== bun ./📜️script.ts generate at $(date -Iseconds) ===" >> "$GENOUT"
bun ./📜️script.ts generate >> "$GENOUT" 2>&1
echo "=== exit $? ===" >> "$GENOUT"
