#!/usr/bin/env bash
# usage: rss-service-sample.sh <out> <seconds>   every 2 s: cgroup, VmRSS and VmHWM of every repo/bun process of this user
out="$1"; end=$((SECONDS+$2)); : > "$out"
while [ $SECONDS -lt $end ]; do
  for pid in $(pgrep -u "$USER" -f 'bun|repo' ); do
    [ -r /proc/$pid/status ] || continue
    rss=$(awk '/^VmRSS/{print int($2/1024)}' /proc/$pid/status); [ "${rss:-0}" -ge 150 ] || continue
    printf '%s t=%s pid=%s rss=%sMB hwm=%sMB cg=%s cmd=%s\n' "$(date +%T)" "$SECONDS" "$pid" "$rss" "$(awk '/^VmHWM/{print int($2/1024)}' /proc/$pid/status)" "$(cut -d: -f3 /proc/$pid/cgroup | head -1 | tail -c 90)" "$(tr '\0\n' '  ' < /proc/$pid/cmdline | cut -c1-100)" >> "$out"
  done
  sleep 2
done
