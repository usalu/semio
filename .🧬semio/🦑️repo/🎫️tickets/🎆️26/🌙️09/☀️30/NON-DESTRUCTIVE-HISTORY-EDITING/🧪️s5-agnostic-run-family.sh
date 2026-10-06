#!/bin/zsh
# 👪️ S5-AGNOSTIC (coordinator 16:4x: one cargo at a time, private target + build dir, disk floor 18 GiB): runs the cross-plugin
# acceptance laws of ONE plugin family — `history_edit_acceptance_law!` (`history_edits_end_to_end`, `history_edit_inputs_resolve`),
# `composed_reload_law!` (`documents_reload_identically`), `composed_child_history_law!` (`child_history_edits_end_to_end`) and the
# derive-emitted payload laws (`semio_payload_law_*`) — in ONE private cargo folder, `🗑️generated/s5-agnostic/target`, as BOTH
# `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` (rule 66: never a private target dir over the shared build dir), with
# `CARGO_BUILD_JOBS=4`, `CARGO_INCREMENTAL=0`, `RUST_MIN_STACK=268435456`. Nothing starts while an activation builds
# (`🗑️generated/coord/activation.flag`, rule 68) or with fewer than 12 GiB free (coordinator 17:5x; exit 5: OWED, end the turn — no
# waiting). 2026-10-06 00:25 (coordinator, new goal: every editor): floor 8 GiB (`S5_FLOOR`), watchdog 5 GiB, folder dropped
# below 9 GiB free — the peers' builds keep the volume at 6–12 GiB.
# The folder is kept between my consecutive families (the framework closure is built once: 3 GiB, 15 min cold for the puzzle
# family) and deleted after a family when it exceeds 12 GiB, when fewer than 12 GiB are free, or with `S5_DROP_DIR=1`.
# Step 1 builds the family's test targets with `cargo build --tests --keep-going` (`cargo test --no-run --no-fail-fast` stops the whole
# build at the first crate whose lib-test does not compile: measured 10:37, 28 min lost); step 2 runs the filtered laws of the crates
# without a `could not compile … (lib…)` line in ONE `cargo test` (re-issued without a crate that turns red there, at most 3 rounds).
# Output: 🗑️generated/s5-agnostic/family-<name>.{build,test}.txt, one row per crate appended to acceptance-results.tsv
# (time, crate, exit, class, G12, inputs, reload, child, payload pass/fail, summary) and the report table refreshed.
# Single-flight and re-issuable: a call that finds a family running waits for it (a Bash call is capped at 10 min) and reports; while
# it waits it is the disk watchdog — below 8 GiB free it stops this family's cargo (exit 4, the folder is deleted, nothing recorded).
# A call stopped by the 40-min cap of a Bash call is simply issued again (finished units stay in the folder).
# A build that fails in a shared `semio-framework-*` crate (a peer's wave in flight) is not this family's verdict: exit 5, one line in
# `batch.events`, the batch re-issues the family later.
# Self-healing harness: when a cargo step reports a compile error INSIDE `🧪️history-edit-acceptance/🦀️.rs` and
# `🗑️generated/s5-agnostic/harness-restore.rs` exists (the copy saved before the last harness landing), the runner restores it under
# the `landing` lock (not in an activation window), writes a train line and a batch event, and repeats that step once.
# Usage: <family-name> <crate>…
cd /Users/ueli/Documents/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING"
G="$T/🗑️generated/s5-agnostic"
mkdir -p "$G/run"
pidf="$G/run/family.pid"
private="/Users/ueli/Documents/semio/$G/target"
alive() { [ -f "$pidf" ] && kill -0 "$(cat "$pidf")" 2>/dev/null }
free() { df -g /System/Volumes/Data | awk 'NR==2{print $4}' }
rows() { awk -F'\t' '{print $1, $2, $4, $5 "/" $6 "/" $7 "/" $8, $9}' "$G/acceptance-results.tsv" 2>/dev/null | tail -${1:-8} }
mine() {
  local pid
  for pid in $(pgrep -x cargo) $(pgrep -x rustc); do
    ps eww -o command= -p "$pid" 2>/dev/null | /usr/bin/grep -q "CARGO_BUILD_BUILD_DIR=$private" && echo "$pid"
  done
}
if alive; then
  waited=0
  while alive && [ "$waited" -lt 540 ]; do
    if [ "$(free)" -lt 5 ]; then
      echo "DISK $(free) GiB < 8 GiB at $(date '+%T'): stopping this family's cargo" | tee "$G/run/family.disk"
      for pid in $(mine); do kill "$pid" 2>/dev/null; done
    fi
    sleep 10; waited=$((waited + 10))
  done
  if alive; then echo "FAMILY STILL RUNNING: $(cat "$G/run/family.current" 2>/dev/null); free $(free) GiB, folder $(du -sg "$private" 2>/dev/null | awk '{print $1}') GiB"; exit 3; fi
  echo "family finished: $(cat "$G/run/family.end" 2>/dev/null)"; rows 20; exit 0
fi
family="$1"; shift
[ -n "$family" ] && [ $# -gt 0 ] || { echo "usage: <family-name> <crate>…"; exit 2; }
echo $$ > "$pidf"; rm -f "$G/run/family.end" "$G/run/family.disk"
build="$G/family-$family.build.txt"; test="$G/family-$family.test.txt"
finish() { echo "$family: $1 $(date '+%T')" | tee "$G/run/family.end"; rm -f "$pidf" "$G/run/family.current"; }
floor="${S5_FLOOR:-8}"
ready() { [ ! -e "$T/🗑️generated/coord/activation.flag" ] && [ "$(free)" -ge "$floor" ] }
closed() { echo "$([ -e "$T/🗑️generated/coord/activation.flag" ] && echo 'an activation builds (activation.flag)' || echo "$(free) GiB free < $floor GiB")" }
disk_stop() {
  [ -e "$G/run/family.disk" ] || return 1
  rm -rf "$private"
  finish "STOPPED by the disk watchdog ($(cat "$G/run/family.disk")) — nothing recorded, folder deleted (OWED)"
  exit 4
}
feature_of() {
  local manifest=$(/usr/bin/grep -l "^name = \"$1\"" ✏️s/🔌️plugins/*/🗿️artifacts/*/📦️packages/🦀️rust/Cargo.toml | head -1)
  [ -n "$manifest" ] && /usr/bin/grep -q "^component-app-assembly" "$manifest" && echo "$1/component-app-assembly"
}
cargo_args() {
  local packages=() features=() crate feature
  for crate in "$@"; do
    packages+=(-p "$crate")
    feature=$(feature_of "$crate"); [ -n "$feature" ] && features+=("$feature")
  done
  [ ${#features[@]} -gt 0 ] && packages+=(--features "${(j:,:)features}")
  print -r -- "${(@q)packages}"
}
red() { /usr/bin/grep -q "could not compile \`$1\` (lib" "$2" }
HARNESS="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs"
heal() {
  /usr/bin/grep -q "🧪️history-edit-acceptance/🦀️.rs:[0-9]*:[0-9]*: error" "$1" || return 1
  [ -f "$G/harness-restore.rs" ] && [ ! -e "$G/run/harness-healed" ] || return 1
  local first="$(/usr/bin/grep -m1 "🧪️history-edit-acceptance/🦀️.rs:[0-9]*:[0-9]*: error" "$1" | sed 's|.*🧪️history-edit-acceptance/||' | cut -c1-300)"
  if [ -e "$T/🗑️generated/coord/activation.flag" ] || ! zsh "$T/🔐️lock.sh" acquire landing S5-AGNOSTIC 300 > /dev/null 2>&1; then
    echo "$(date '+%F %T') HARNESS RED in $family, NOT restored (activation window or landing lock): $first" >> "$G/batch.events"
    return 1
  fi
  cp "$G/harness-restore.rs" "$HARNESS"
  zsh "$T/🔐️lock.sh" release landing S5-AGNOSTIC > /dev/null 2>&1
  touch "$G/run/harness-healed"
  echo "$(date '+%T') S5-AGNOSTIC harness-auto-restore (the last harness landing did not compile in family $family; the runner restored the copy saved before it) 1 file restore: -" >> "$T/🗑️generated/coord/train.txt"
  echo "$(date '+%F %T') HARNESS RED in $family, restored the harness of before the last landing: $first" >> "$G/batch.events"
  return 0
}
export CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 RUST_MIN_STACK=268435456 CARGO_TARGET_DIR="$private" CARGO_BUILD_BUILD_DIR="$private"
: > "$build"
echo "$family build ($# crates) since $(date '+%T')" > "$G/run/family.current"
ready || { finish "NOT STARTED: $(closed) — nothing ran (OWED)"; exit 5; }
echo "== build $(date '+%F %T') free=$(free)GiB: $*" >> "$build"
eval "cargo build --manifest-path ✏️s/Cargo.toml $(cargo_args "$@") --tests --keep-going --message-format=short" >> "$build" 2>&1
echo "== build exit=$? $(date '+%F %T') free=$(free)GiB folder=$(du -sg "$private" 2>/dev/null | awk '{print $1}')GiB" >> "$build"
if heal "$build"; then
  mv "$build" "$build.harness-red.txt"
  echo "== build again on the restored harness $(date '+%F %T') free=$(free)GiB: $*" > "$build"
  eval "cargo build --manifest-path ✏️s/Cargo.toml $(cargo_args "$@") --tests --keep-going --message-format=short" >> "$build" 2>&1
  echo "== build exit=$? $(date '+%F %T') free=$(free)GiB" >> "$build"
fi
disk_stop
foundation="$(/usr/bin/grep -m1 -o "could not compile \`semio-framework[a-z-]*\`" "$build")"
if [ -n "$foundation" ]; then
  first="$(/usr/bin/grep -m1 ': error' "$build" | sed 's|/Users/ueli/Documents/semio/||' | cut -c1-260)"
  echo "$(date '+%F %T') FOUNDATION RED in $family ($foundation): $first" >> "$G/batch.events"
  finish "NOT RUN: $foundation — a shared framework crate is red, not this family ($first); retry (OWED)"
  exit 5
fi
candidates=()
for crate in "$@"; do red "$crate" "$build" || candidates+=("$crate"); done
code=-
: > "$test"
round=0
while [ ${#candidates[@]} -gt 0 ] && [ "$round" -lt 3 ]; do
  round=$((round + 1))
  echo "$family test round $round (${#candidates[@]} of $# crates) since $(date '+%T')" > "$G/run/family.current"
  ready || { finish "NOT CONTINUED before test round $round: $(closed) — ${#candidates[@]} crates built, laws not run (OWED)"; exit 5; }
  echo "== test round $round $(date '+%F %T'): ${candidates[*]}" >> "$test"
  eval "cargo test --manifest-path ✏️s/Cargo.toml $(cargo_args "${candidates[@]}") --lib --no-fail-fast --message-format=short -- history_edits_end_to_end history_edit_inputs_resolve documents_reload_identically semio_payload_law --nocapture --test-threads=1" >> "$test" 2>&1
  code=$?
  echo "== test round $round exit=$code $(date '+%F %T')" >> "$test"
  if heal "$test"; then
    mv "$test" "$test.harness-red.txt"
    echo "== test round $round again on the restored harness $(date '+%F %T'): ${candidates[*]}" > "$test"
    eval "cargo test --manifest-path ✏️s/Cargo.toml $(cargo_args "${candidates[@]}") --lib --no-fail-fast --message-format=short -- history_edits_end_to_end history_edit_inputs_resolve documents_reload_identically semio_payload_law --nocapture --test-threads=1" >> "$test" 2>&1
    code=$?
    echo "== test round $round exit=$code $(date '+%F %T')" >> "$test"
  fi
  disk_stop
  /usr/bin/grep -q "Running unittests" "$test" && break
  next=()
  for crate in "${candidates[@]}"; do red "$crate" "$test" || next+=("$crate"); done
  [ ${#next[@]} -eq ${#candidates[@]} ] && break
  candidates=("${next[@]}")
done
python3 "$T/🧪️s5-agnostic-family-rows.py" "$family" "$build" "$test" "$code" 1 "$@"
python3 "$T/🧪️s5-agnostic-table.py" > /dev/null 2>&1
size=$(du -sg "$private" 2>/dev/null | awk '{print $1}')
if [ "${S5_DROP_DIR:-0}" = 1 ] || [ "${size:-0}" -gt 12 ] || [ "$(free)" -lt 9 ]; then rm -rf "$private"; echo "private cargo folder (${size} GiB) deleted; $(free) GiB free" | tee -a "$test"; else echo "private cargo folder kept for the next family: ${size} GiB, $(free) GiB free" | tee -a "$test"; fi
finish "done (${#candidates[@]} of $# crates reached the law run)"
rows $#
