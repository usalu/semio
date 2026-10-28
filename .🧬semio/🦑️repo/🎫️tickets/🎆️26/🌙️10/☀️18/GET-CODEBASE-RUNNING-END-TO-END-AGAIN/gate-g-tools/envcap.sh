#!/bin/sh
# Samples the SEMIO_ environment of every running bun process every 2 s until <stopfile> exists; one file per pid in <outdir>.
out=$1; stop=$2
while [ ! -f "$stop" ]; do
  for pid in $(pgrep -u "$(id -u)" bun); do
    ps eww -o command= -p "$pid" 2>/dev/null | tr ' ' '\n' | grep -E '^SEMIO_(BUILD|GRAPH|REPO)_' | sort > "$out/$pid.tmp" && [ -s "$out/$pid.tmp" ] && mv "$out/$pid.tmp" "$out/$pid.env" || rm -f "$out/$pid.tmp"
  done
  sleep 2
done
