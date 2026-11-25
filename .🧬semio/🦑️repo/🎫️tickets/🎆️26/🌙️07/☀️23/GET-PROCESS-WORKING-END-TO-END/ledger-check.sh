#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
cd /Users/ueli/Documents/semio || exit 1
i=0
while read -r m; do
  i=$((i+1))
  $T/stale-cargo.sh ledger-check-$i check --tests --manifest-path "$m" > /dev/null
  r=$?
  e=$(grep -c '^error' "$T/🗑️generated/stale-ledger-check-$i.txt")
  echo "$i exit=$r errors=$e $m" >> $T/🗑️generated/ledger-check-summary.txt
done < $T/🗑️generated/ledger-manifests.txt
echo done >> $T/🗑️generated/ledger-check-summary.txt
