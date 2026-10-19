#!/bin/zsh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/lockref"
while IFS= read -r m; do
  command grep -qF -- "$m"$'\t' "$G/results.tsv" 2>/dev/null && continue
  "$C/lockref-tools/one.sh" "$m"
done < "$G/order.txt"
touch "$G/done"
