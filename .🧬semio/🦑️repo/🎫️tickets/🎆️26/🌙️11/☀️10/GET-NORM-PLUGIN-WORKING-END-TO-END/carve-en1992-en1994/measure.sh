#!/usr/bin/env bash
# usage: measure.sh <log-prefix> <crate-dir-for-family> <package>...   (runs inside the lane; one build per package, retried when the warden cancels; samples the rustc resident size of the package every 3 s)
T="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
P="$1"; D="$2"; shift 2
cd "$D" || exit 1
for C in "$@"; do
  L="$T/🗑️generated/$P-$C.log"
  U="$(echo "$C" | tr '-' '_')"
  for try in 1 2 3; do
    : > "$L.rss"
    ( while true; do ps -axww -o rss=,command= | grep -- "--crate-name $U " | grep -v grep | awk -v t="$(date +%s)" '{ if ($1>m) m=$1 } END { if (m>0) print t, int(m/1024) }' >> "$L.rss"; sleep 3; done ) &
    S=$!
    repo cargo --store=shared --configuration=prototype -- build -p "$C" --target wasm32-wasip2 --offline > "$L" 2>&1
    rc=$?; echo "EXIT $rc try $try" >> "$L"
    kill $S 2>/dev/null; wait $S 2>/dev/null
    grep -q "SIGKILL\|warden\|SIGTERM" "$L" || break
    sleep 30
  done
  echo "[measure] $C rc=$rc ledger=$(cat /Users/ueli/Documents/semio/.cache/b/r/m/u/c/$U.peak 2>/dev/null) rssPeakMB=$(sort -k2 -n "$L.rss" | tail -1 | awk '{print $2}') samples=$(wc -l < "$L.rss") $(grep -o 'peaked at [0-9]* MB' "$L" | head -3 | tr '\n' ' ')" >> "$T/🗑️generated/$P-summary.txt"
done
