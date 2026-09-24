#!/bin/zsh
# WG8: blocks (≤ $1 s) until W2's Hub Handoff names a block-carrying catalog on 7800 and 7800 answers ready.
cd /Users/ueli/Documents/semio/.tmp-ticket
deadline=$(( $(date +%s) + ${1:-560} ))
while [ $(date +%s) -lt $deadline ]; do
  row=$(/usr/bin/grep "^| catalog |" "📓️wp-w2.md" | /usr/bin/grep -o "packages \*\*[^*]*\*\*")
  ready=$(curl -s -m 3 http://127.0.0.1:7800/readyz | /usr/bin/grep -o '"status":"ready","runId":"[0-9a-f]*"')
  if print -r -- "$row" | /usr/bin/grep -q block && [ -n "$ready" ]; then echo "CATALOG-B $row $ready $(date +%T)"; exit 0; fi
  sleep 20
done
echo "WAITING $(date +%T) row=[$row] 7800=[$ready] w2=[$(tail -1 wp-w2/generated/release-par.txt | cut -c1-120)]"
exit 1
