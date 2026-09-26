#!/bin/zsh
# 🏎️ H10: lands Q1 (owned-interpreter speedup, `patches/q1-interpreter-speed.py`) in ONE command, for the post-publish
# window. Each step gates the next; on any failure the interpreter file is restored byte-for-byte from the copy taken
# before the apply (a file copy, no git) and the command exits non-zero:
#   1. dry run (every hunk found exactly once on the current tree)          4. interpreter unit laws (incl. wasmtime oracles)
#   2. apply                                                                 5. identity sweep over catalog B2's codec rows,
#   3. cargo check -p semio-framework-plugin-host --tests                       saved-before vs landed interpreter
#                                                                            6. semio-hub `artifact_authority::` laws
# usage: zsh q1-land.sh [--skip-sweep]        capture: .🧬semio/🌐hub/s12-h10-logs/q1-land-<time>.txt
set -u
ROOT=/Users/ueli/Documents/semio
H10=$ROOT/.tmp-ticket/wp-h10
FILE="$ROOT/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs"
STAMP=$(date +%H%M%S)
OUT="$ROOT/.🧬semio/🌐hub/s12-h10-logs/q1-land-$STAMP.txt"
SAVED="$ROOT/.🧬semio/🌐hub/s12-h10-logs/q1-interpreter-before-$STAMP.rs"
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=$H10/target
step() { echo "=== $(date +%T) $*" | tee -a "$OUT"; }
fail() { step "FAILED: $* — restoring the interpreter"; cp "$SAVED" "$FILE"; step "restored; tree unchanged; see $OUT"; exit 1; }
cd "$ROOT"
step "dry run"; python3 "$H10/patches/q1-interpreter-speed.py" >> "$OUT" 2>&1 || { step "dry run refused; nothing applied"; exit 1; }
cp "$FILE" "$SAVED"
step "apply"; python3 "$H10/patches/q1-interpreter-speed.py" --apply >> "$OUT" 2>&1 || fail "apply"
step "check"; nice -n 15 cargo check -p semio-framework-plugin-host --tests --message-format short >> "$OUT" 2>&1 || fail "cargo check"
step "interpreter laws"; nice -n 15 cargo test -p semio-framework-plugin-host --lib --no-fail-fast -- interpreter >> "$OUT" 2>&1 || fail "interpreter laws"
if [[ "${1:-}" != "--skip-sweep" ]]; then
  step "identity sweep"; python3 "$H10/patches/q1-interpreter-speed.py" --sweep "$SAVED" "$FILE" >> "$OUT" 2>&1 || fail "identity sweep"
fi
step "hub trusted-catalog laws"; nice -n 15 cargo test -p semio-hub --lib --no-fail-fast -- artifact_authority:: >> "$OUT" 2>&1 || fail "hub laws"
step "LANDED. owned_engine_identity() changed with the interpreter source: every hub re-verifies its guests once on its next boot."
