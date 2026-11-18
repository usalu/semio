#!/bin/bash
# 🛰️ Round 31 observer for g1-m08: starts no repo command; every 10 minutes it appends to round-31.md the machine's ready.json, the last 30 lines of the running stage's log, the memory pressure, the five largest processes and what is known about a waiting permission dialog. Ends when ready.json says ready, when ROUND.md or next.md changes, or after the given hours. Usage: round-31-observe.sh [hours]
set -u
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
ready="$HOME/Documents/semio/.cache/f/ready"
status="/Volumes/semio-store/v1/origins/g1-m08/ready.json"
report="$here/round-31.md"
round=$(head -1 "$here/../../ROUND.md")
next=$(cksum < "$here/next.md")
deadline=$(( $(date +%s) + ${1:-6} * 3600 ))
field() { sed -nE "s/.*\"$1\":(\"[^\"]*\"|[^,}]*).*/\1/p" "$status" 2>/dev/null | tr -d '"' | head -1; }
outcome="time limit"
while [ "$(date +%s)" -lt "$deadline" ]; do
  stage=$(field stage)
  {
    printf '\n## Observation %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    printf -- '- `ready.json`: state `%s`, stage `%s` (%s of %s), try (`attempts`) %s, exit `%s`, refusal `%s`, ready `%s`, peak %s MB\n' "$(field state)" "$stage" "$(field stageIndex)" "$(field stageCount)" "$(field attempts)" "$(field exit)" "$(field refusal)" "$(field ready)" "$(field peakMegabytes)"
    printf -- '- Memory pressure: `%s`, level %s\n' "$(memory_pressure | tail -1)" "$(sysctl -n kern.memorystatus_vm_pressure_level)"
    printf -- '- Permission dialog on screen: not measured (this session has no view of the screen); `UserNotificationCenter` processes: %s\n' "$(pgrep -x UserNotificationCenter | wc -l | tr -d ' ')"
    printf '\n`ready.json`:\n\n```json\n%s\n```\n' "$(cut -c1-2500 "$status")"
    printf '\nLast 30 lines of `%s.log`:\n\n```text\n%s\n```\n' "$stage" "$(tail -30 "$ready/$stage.log" 2>/dev/null | cut -c1-300)"
    printf '\nFive largest processes (rss KB, pid, ppid, command):\n\n```text\n%s\n```\n' "$(ps -axo rss=,pid=,ppid=,args= | sort -nr | head -5 | cut -c1-400)"
  } >> "$report"
  if [ "$(field ready)" = true ]; then outcome="ready"; break; fi
  if [ "$(head -1 "$here/../../ROUND.md")" != "$round" ]; then outcome="round changed"; break; fi
  if [ "$(cksum < "$here/next.md")" != "$next" ]; then outcome="next.md changed"; break; fi
  for _ in $(seq 20); do
    sleep 30
    [ "$(field ready)" = true ] && break
    [ "$(head -1 "$here/../../ROUND.md")" != "$round" ] && break
    [ "$(cksum < "$here/next.md")" != "$next" ] && break
  done
done
echo "[DEBUG] round 31 observer ended: $outcome at $(date -u +%FT%TZ)"; cut -c1-1500 "$status"; head -3 "$here/../../ROUND.md"; cat "$here/next.md"
