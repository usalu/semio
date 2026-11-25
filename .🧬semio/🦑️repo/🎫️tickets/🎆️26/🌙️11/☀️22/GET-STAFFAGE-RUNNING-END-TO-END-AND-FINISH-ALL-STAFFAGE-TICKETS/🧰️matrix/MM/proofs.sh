#!/bin/sh
# 🖨️ The proof sheets of the final run, one category at a time and resumable: proofs.sh <sheet folder> [category …]
# Writes `contact-<category>.svg` and `print-<1:n>-<category>.svg` for 500, 200, 100 and 50 with `T/🖨️proofs.ts`, one capacity request per category (the sheets of a category appear whole or not at all per file; a category whose five files exist is skipped, delete them to redo it). Run it on the frozen catalog after the full matrix, never beside it.
# Reservations (PROOF_MB, default 1500) and the seconds of one category (PROOF_S, default 7200) come from the measurements of the report; proofs.ts reads one document at a time.
set -u
R=/Users/ueli/Documents/semio
T="$R/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
case "${1:-}" in "" | -*) echo "usage: sh proofs.sh <sheet folder> [category …]" >&2; exit 2 ;; esac
mkdir -p "$1" || exit 1
out=$(cd "$1" && pwd); shift
[ $# -eq 0 ] && set -- humans animals vegetation furniture appliances objects vehicles machinery components
cd "$R" || exit 1
status=0
for category in "$@"; do
  complete=1
  for name in "contact-$category" "print-500-$category" "print-200-$category" "print-100-$category" "print-50-$category"; do [ -s "$out/$name.svg" ] || complete=0; done
  [ "$complete" -eq 1 ] && { echo "proofs: $category is complete, skipped" >&2; continue; }
  repo capacity run --memory="${PROOF_MB:-1500}" -- timeout "${PROOF_S:-7200}" "$BUN" "$T/🖨️proofs.ts" --out "$out" --categories "$category" || { echo "proofs: $category ended with $?" >&2; status=1; }
done
exit "$status"
