#!/bin/zsh
# 🚦️ Runs one tool process of batch ML-3 through the machine-wide memory queue (`repo capacity run`): q.sh <megabytes> <working directory> <timeout seconds> <command> <args…>. The timeout counts the run only; the peak memory of the run is appended to `$GEN/log/rss.txt`.
source "${0:A:h}/env.sh"
mb="$1"; dir="$2"; t="$3"; shift 3; mkdir -p "$GEN/log"
cd "$R" && repo capacity run --memory="$mb" -- zsh -c 'cd "$1" && shift && r="$1" && shift && /usr/bin/time -l -o "$r.$$" timeout "$@"; code=$?; echo "$(( $(awk "/maximum resident set size/{print \$1}" "$r.$$") / 1048576 )) MB $3 $4 $5" >> "$r"; rm -f "$r.$$"; exit $code' q "$dir" "$GEN/log/rss.txt" "$t" "$@"
