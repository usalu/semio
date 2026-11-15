#!/bin/bash
# ⏳ Waits (5 s steps, at most 113) until round-2.monitor.txt has a new "try … ended" or "round ended" line, then prints the tail of the monitor and the service lines of the tries. Usage: wait-try.sh
here=$(cd "$(dirname "$0")" && pwd); m="$here/round-2.monitor.txt"; c=$(grep -c -E 'ended' "$m"); n=0
until [ "$(grep -c -E 'ended' "$m")" -gt "$c" ] || [ $n -ge 113 ]; do sleep 5; n=$((n+1)); done
tail -5 "$m" | cut -c1-200; for f in "$here"/round-2.try-*.service-lines.txt; do echo "== ${f##*/}"; tail -3 "$f" | cut -c1-280; done
