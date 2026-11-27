#!/bin/zsh
# 🚦️ Runs one script of lane CL-T1 through the machine-wide memory queue and records its peak: q.sh <megabytes> <timeout seconds> <name> <script of this folder> <args…> writes `$GEN/log/q-<name>.log` and appends the exit code and the largest resident set to `$GEN/log/runs.txt`.
source "${0:A:h}/env.sh"
mb="$1"; t="$2"; n="$3"; shift 3; mkdir -p "$GEN/log"
cd "$R" && repo capacity run --memory="$mb" -- /usr/bin/time -l -o "$GEN/log/rss-$n.txt" timeout "$t" zsh "$W/$1" "${@:2}" > "$GEN/log/q-$n.log" 2>&1
echo "$(date +%H:%M:%S) $n exit $? $(( $(awk '/maximum resident set size/{print $1}' "$GEN/log/rss-$n.txt") / 1048576 )) MB largest process ($mb MB reserved)" >> "$GEN/log/runs.txt"
