#!/bin/zsh
# 🚦️ Runs one tool process of batch MV-L1 through the machine-wide memory queue (`repo capacity run`): q.sh <megabytes> <working directory> <timeout seconds> <command> <args…>. The timeout counts the run only. Inside a reservation a chain script already holds (INSIDE=1) the command runs at once; its peak memory is appended to `$MEMLOG` (`/usr/bin/time -l`) in both cases.
mb="$1"; dir="$2"; t="$3"; shift 3; export MEMLOG="${MEMLOG:-/dev/null}"
if [[ -n "$INSIDE" ]]; then cd "$dir" && { echo "== $mb MB reserved: ${@[2,4]}" >> "$MEMLOG"; exec /usr/bin/time -l -a -o "$MEMLOG" timeout "$t" "$@"; }; fi
cd /Users/ueli/Documents/semio && repo capacity run --memory="$mb" -- zsh -c 'cd "$1" && shift && exec /usr/bin/time -l -a -o "$MEMLOG" timeout "$@"' q "$dir" "$t" "$@"
