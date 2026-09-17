#!/bin/zsh
# 🪟️ A7 verification — packet A7 (flow, dag, vcs, note, sequence, writer, animate, imperative, remodel).
# Usage: 🐚️a7-verify.sh check|test|wasm  — one lane at a time, logs under 🗑️generated/a7/.
# Retries each crate while cargo is SIGKILLed (peers run `pkill -9 cargo rustc`) or the shared
# target lock is contended; a real compile/test failure (rc 101) stops that crate's retries.
set -u
REPO=/Users/ueli/Documents/semio
OUT="$REPO/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ARTIFACT-TREE-VIRTUALISED-STREAMING/🗑️generated/a7"
LANE="${1:-check}"
CRATES=(
  semio-s-artifact-flow-flow
  semio-s-artifact-dag-dag
  semio-s-artifact-vcs-vcs
  semio-s-artifact-note-note
  semio-s-artifact-sequence-sequence
  semio-s-artifact-writer-writer
  semio-s-artifact-animate-presentation
  semio-s-artifact-imperative-procedure
  semio-s-artifact-remodel-remodeling
)
mkdir -p "$OUT"
cd "$REPO" || exit 1
for crate in "${CRATES[@]}"; do
  log="$OUT/$LANE-$crate.txt"
  for attempt in 1 2 3 4 5 6 7 8; do
    case "$LANE" in
      check) cargo check -p "$crate" --tests > "$log" 2>&1 ;;
      test)  cargo test  -p "$crate"          > "$log" 2>&1 ;;
      wasm)  cargo check -p "$crate" --target wasm32-wasip2 > "$log" 2>&1 ;;
    esac
    rc=$?
    if [ $rc -eq 0 ] || [ $rc -eq 101 ]; then
      echo "$LANE $crate rc=$rc attempt=$attempt" >> "$OUT/$LANE-summary.txt"
      break
    fi
    echo "$LANE $crate RETRY $attempt rc=$rc" >> "$OUT/$LANE-summary.txt"
    /bin/sleep 20
  done
done
echo "$LANE lane finished" >> "$OUT/$LANE-summary.txt"
