#!/bin/zsh
# usage: pos-eval.sh <family>...  prints, per family and cfg, the error totals and the diagnostics that mention a reader macro or import
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
for fam in "$@"; do
  for cfg in lib test; do
    f="frd-$fam-run-cur-$cfg-final.err"
    [ -f "$f" ] || continue
    total=$(/usr/bin/grep -c '"level":"error"' "$f")
    mine=$(/usr/bin/grep '"level":"error"' "$f" | /usr/bin/grep -c -E 'fixture_(text|bytes|path)|manifest_dir|repo_root!|repo_text|repo_bytes|test_fixtures|semio_framework_test_fixtures')
    echo "$fam $cfg errors=$total mentioning-the-reader=$mine"
  done
done
