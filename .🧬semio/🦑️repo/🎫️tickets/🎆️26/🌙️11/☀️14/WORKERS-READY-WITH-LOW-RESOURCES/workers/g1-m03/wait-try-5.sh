#!/bin/bash
# ⏳ Waits (5 s steps, at most 113) until round-5.monitor.txt has a new "ended" line, then prints the monitor tail, the newest service lines and the round line of ROUND.md. Usage: wait-try-5.sh
here=$(cd "$(dirname "$0")" && pwd); m="$here/round-5.monitor.txt"; c=$(grep -c 'ended' "$m"); n=0
until [ "$(grep -c 'ended' "$m")" -gt "$c" ] || [ $n -ge 113 ]; do sleep 5; n=$((n+1)); done
tail -4 "$m" | cut -c1-200; f=$(ls -t "$here"/round-5.try-*.service.txt 2>/dev/null | head -1); [ -n "$f" ] && { echo "== ${f##*/}"; tail -6 "$f" | cut -c1-330; }
l=$(ls -t "$HOME"/native-bootstrap-round-5*.log | head -1); sed 's/\x1b\[[0-9;]*m//g' "$l" | grep -a -E 'fleet|wanted|arrived|serving|restored|Restarting|adopted|native_rc' | grep -v -e 'fleet status' -e '(used by:' -e 'fleet load table' | tail -6 | cut -c1-330
head -1 "$here/../../ROUND.md"
