#!/bin/zsh
# 🏃️ W3-STDIO-CASES: runs `test parity exhaustive --case <case>` for every argument, one at a time, each only once the
# shared taxonomy validates (peers edit it concurrently) and fewer than 8 rustc are running. Per-case logs and one summary
# line per case land in this ticket's 🗑️generated/w3-stdio-cases/. Usage: zsh 🧪️w3-stdio-run-cases.sh <case>...
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🗑️generated/w3-stdio-cases"
mkdir -p "$G"
for c in "$@"; do
  n=0
  until bun -e 'const m=await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"); m.loadCatalogTaxonomy();' >/dev/null 2>&1 || [ $n -ge 60 ]; do n=$((n+1)); sleep 20; done
  until [ "$(pgrep -x rustc | wc -l | tr -d ' ')" -lt 8 ]; do sleep 20; done
  bun ./📜️script.ts test parity exhaustive --case "$c" > "$G/parity-$c.txt" 2>&1
  code=$?
  line=$(/usr/bin/grep "^\[test\] level=" "$G/parity-$c.txt" | tail -1)
  echo "$(date +%H:%M) exit=$code $c :: ${line:-no summary}" >> "$G/summary.txt"
done
echo "[w3-stdio] done" >> "$G/summary.txt"
