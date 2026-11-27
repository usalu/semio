#!/bin/zsh
# 🚦️ Runs one command of lane CL-B through the machine-wide memory queue from a given folder and records its peak: q.sh <megabytes> <timeout seconds> <name> <folder> <command…> writes `$GEN/log/q-<name>.log` and appends exit code and largest resident set to `$GEN/log/runs.txt`.
source "${0:A:h}/env.sh"
mb="$1"; t="$2"; n="$3"; dir="$4"; shift 4; mkdir -p "$GEN/log"
cd "$R" && repo capacity run --memory="$mb" -- /usr/bin/time -l -o "$GEN/log/rss-$n.txt" timeout "$t" zsh -c 'cd "$1" && shift && exec "$@"' _ "$dir" "$@" > "$GEN/log/q-$n.log" 2>&1
r=$?
echo "$(date +%H:%M:%S) $n exit $r $(( $(awk '/maximum resident set size/{print $1}' "$GEN/log/rss-$n.txt") / 1048576 )) MB largest process ($mb MB reserved)" >> "$GEN/log/runs.txt"
exit $r
