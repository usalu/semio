#!/bin/zsh
# 🚦️ Runs one tool process of batch MV-1 through the machine-wide memory queue (binding rule of the coordinator of 2026-11-24: every bun or node run goes through `repo capacity run`): q.sh <megabytes> <working directory> <timeout seconds> <command> <args…>. The command waits in the queue until the host can carry it; the timeout counts the run only. Inside a reservation that `finish.sh` already holds (INSIDE=1) the command runs at once, one after the other, and its peak memory is appended to `$MEMLOG` (`/usr/bin/time -l`), as it is for a queued run when MEMLOG is set.
mb="$1"; dir="$2"; t="$3"; shift 3; export MEMLOG="${MEMLOG:-/dev/null}"
if [[ -n "$INSIDE" ]]; then cd "$dir" && { echo "== $mb MB reserved: ${@[2,4]}" >> "$MEMLOG"; exec /usr/bin/time -l -a -o "$MEMLOG" timeout "$t" "$@"; }; fi
cd /Users/ueli/Documents/semio && repo capacity run --memory="$mb" -- zsh -c 'cd "$1" && shift && exec /usr/bin/time -l -a -o "$MEMLOG" timeout "$@"' q "$dir" "$t" "$@"
