#!/bin/bash
# 🔭 Observes the fleet agent's `ready` child on g1-m04 every 10 minutes (round 31) without starting any setup command: appends `ready.json`, the last 30 lines of the running stage's log, MemAvailable and the five largest processes to round-31.md; ends when `ready` is true, when ROUND.md names a higher round, or after six hours. Usage: observe-31.sh
set -u
M="$(cd "$(dirname "$0")" && pwd)"; W="$(cd "$M/../.." && pwd)"; out="$M/round-31.md"; store=/mnt/semio-store/v1/origins/g1-m04; deadline=$(( $(date +%s) + 21600 ))
[ -e "$out" ] || printf '# g1-m04 Round 31 (Linux, 16 GB, SteamOS): Observation Only\n\nNo setup command is started by this session from 23:57 UTC on 2026-11-15. Snapshots every 10 minutes by `observe-31.sh`. No macOS dialog applies (Linux).\n' > "$out"
while :; do
  json="$(cat "$store/ready.json" 2>/dev/null)"; stage="$(echo "$json" | grep -o '"stage":"[^"]*"' | cut -d'"' -f4)"
  { printf '\n## %s\n\n`ready.json`: `%s`\n\n' "$(date -u +%Y-%m-%dT%H:%MZ)" "$(echo "$json" | cut -c1-1400 | tr '`' "'")"
    printf 'MemAvailable %s MB, SwapFree %s MB, load %s\n\nLast 30 lines of `%s.log`:\n\n```\n' "$(awk '/MemAvailable/{printf "%d", $2/1024}' /proc/meminfo)" "$(awk '/SwapFree/{printf "%d", $2/1024}' /proc/meminfo)" "$(cut -d' ' -f1-3 /proc/loadavg)" "$stage"
    sed $'s/\x1b\\[[0-9;]*m//g' "$HOME/semio/.cache/f/ready/$stage.log" 2>/dev/null | tail -n 30 | cut -c1-400
    printf '```\n\nFive largest processes (rss KiB, pid, ppid, command):\n\n```\n'; ps -eo rss,pid,ppid,args --sort=-rss | sed -n 2,6p | cut -c1-260; printf '```\n'; } >> "$out"
  echo "$json" | grep -q '"ready":true' && { echo "ready" ; break; }
  now="$(grep -m1 '^round:' "$W/ROUND.md" | grep -o '[0-9]*')"; [ "${now:-31}" -gt 31 ] && { echo "ROUND.md names round $now"; break; }
  [ "$(date +%s)" -ge "$deadline" ] && { echo "six hours reached"; break; }
  sleep 600
done
