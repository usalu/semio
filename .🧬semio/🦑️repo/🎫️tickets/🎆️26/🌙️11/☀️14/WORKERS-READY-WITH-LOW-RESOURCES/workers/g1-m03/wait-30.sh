#!/bin/bash
# ⏳ Waits (5 s steps, at most 112) for a new "ended" line in round-30.monitor.txt, then prints the monitor tail, the newest try's key lines and the round line of ROUND.md. Usage: wait-30.sh
here=$(cd "$(dirname "$0")" && pwd); m="$here/round-30.monitor.txt"; c=$(grep -c 'ended' "$m"); n=0
until [ "$(grep -c 'ended' "$m")" -gt "$c" ] || [ $n -ge 112 ]; do sleep 5; n=$((n+1)); done
date -u +%H:%M:%SZ; tail -4 "$m" | cut -c1-260
l=$(ls -t "$HOME"/native-bootstrap-round-30*.log 2>/dev/null | head -1)
if [ -n "$l" ]; then echo "$l"; sed 's/\x1b\[[0-9;]*m//g' "$l" | grep -a -v '^\[script\] {' | grep -a -E 'owners [0-9]+ of [0-9]+|cut smaller|could not be read|unexpected failure|native_rc|^(✔|✖|>) +nx run|Failed tasks|^- |holds the dependency store' | awk '!s[substr($0,1,60)]++' | tail -8 | cut -c1-330; fi
s=$(ls -t "${TMPDIR%/}"/semio-repo/"$(id -u)"/host-*.log 2>/dev/null | head -1); grep -a -E '^graph: ' "$s" | grep -v 'memory headroom' | awk '!s[$0]++' | tail -3 | cut -c1-300
head -1 "$here/../../ROUND.md"; cat "$here/next.md" 2>/dev/null | head -5
