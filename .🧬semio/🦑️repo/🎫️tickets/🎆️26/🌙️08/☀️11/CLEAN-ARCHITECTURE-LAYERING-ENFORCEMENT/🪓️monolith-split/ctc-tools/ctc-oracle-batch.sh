#!/bin/zsh
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/ctc-tools
sed -n "$1,$2p" $T/../🗑️generated/ctc-owner-features.txt | while read o f; do
  out=$(nice -n 10 bun $T/ctc-oracle.ts $o $f 2>&1)
  echo "$out" | /usr/bin/grep -E "NEW CRATE REFS" | sed "s|^|$o $f: |"
  echo "$out" | /usr/bin/grep -E "^  error\[E0(425|412|423|432|599|603|616|624|061|277|308|422|405|407|117)\]" | /usr/bin/grep -v "for type \`!\`" | sed "s|^|$o $f: |" | cut -c1-300
done
