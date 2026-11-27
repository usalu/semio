#!/usr/bin/env bash
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END/🗑️generated"
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
until grep -q '^done' "$G/lock-refresh-status.txt"; do sleep 20; done
grep -E '^(124|1|[2-9][0-9]*) ' "$G/lock-refresh-status.txt" | sed -E 's/^[0-9]+ [0-9]+s //' > "$G/stale-locks-requeue.txt"
[ -s "$G/stale-locks-requeue.txt" ] && tr '\n' '\0' < "$G/stale-locks-requeue.txt" | xargs -0 -n1 -P4 "$T/plugin-units-lock-one.sh"
echo requeue-done >> "$G/lock-refresh-status.txt"
