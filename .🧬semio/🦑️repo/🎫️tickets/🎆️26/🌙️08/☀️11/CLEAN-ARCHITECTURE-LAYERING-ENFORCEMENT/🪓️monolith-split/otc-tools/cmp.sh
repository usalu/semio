#!/bin/zsh
# usage: cmp.sh <before> <after> <label...> ; compares status+test-title sets ignoring the file path column
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$T/🗑️generated"; B="$1"; A="$2"; shift 2
for l in "$@"; do
  node "$T/taf-tools/vsum.cjs" "$G/otc-vt-$B-$l.json" "$G/otc-vt-$B-$l.rows" > /dev/null 2>&1
  node "$T/taf-tools/vsum.cjs" "$G/otc-vt-$A-$l.json" "$G/otc-vt-$A-$l.rows"
  # compare: status + test title only (file column dropped)
  if diff <(cut -f1,3- "$G/otc-vt-$B-$l.rows" | sort) <(cut -f1,3- "$G/otc-vt-$A-$l.rows" | sort) > "$G/otc-cmp-$B-$A-$l.diff"; then echo "$l: status+title sets IDENTICAL"; else echo "$l: DIFFERENT ($(wc -l < "$G/otc-cmp-$B-$A-$l.diff") diff lines)"; fi
done
