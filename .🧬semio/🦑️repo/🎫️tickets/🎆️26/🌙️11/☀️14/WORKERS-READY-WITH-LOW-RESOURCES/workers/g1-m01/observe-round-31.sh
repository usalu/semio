#!/bin/bash
# 🔭 Observes the fleet agent's `ready` child on g1-m01 for round 31 every 10 minutes (read-only) and appends one section per observation to round-31.md; ends when ready.json says ready or after 24 more hours. Usage: observe-round-31.sh
set -u
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
store="/Volumes/semio-store/v1/origins/g1-m01"; local_ready="$HOME/Documents/semio/.cache/f/ready"; out="$here/round-31.part-3.md"
field() { python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); r=d.get("refusal") or {}; print("state=%s stage=%s (%s of %s) try=%s exit=%s ready=%s refusal=%s reason=%s peakMegabytes=%s pid=%s" % (d.get("state"),d.get("stage"),d.get("stageIndex"),d.get("stageCount"),d.get("attempts"),d.get("exit"),d.get("ready"),(r.get("code"),r.get("en")) if r else None,d.get("reason"),d.get("peakMegabytes"),d.get("pid")))' "$1" 2>&1; }
n=0
while [ "$n" -lt 144 ]; do
  n=$((n+1)); stage=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("stage") or "")' "$store/ready.json" 2>/dev/null)
  {
    printf '\n## Observation %s (%s)\n\n' "$n" "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    printf -- '- `ready.json` (store, written %s): `%s`\n' "$(date -u -r "$store/ready.json" +%H:%M:%SZ 2>/dev/null)" "$(field "$store/ready.json")"
    printf -- '- `memory_pressure | tail -1`: `%s`; pressure level %s\n' "$(memory_pressure | tail -1)" "$(sysctl -n kern.memorystatus_vm_pressure_level)"
    printf -- '- Permission dialog: `UserNotificationCenter` process %s (the screen itself was not looked at)\n' "$(pgrep -x UserNotificationCenter >/dev/null && echo "is running, a dialog may be waiting" || echo "is not running, so no system dialog is waiting")"
    printf -- '- Five largest processes (resident KiB, pid, command):\n\n```\n%s\n```\n' "$(ps -axo rss,pid,args | sort -nr | head -5 | cut -c1-260)"
    printf -- '- Last 30 lines of the log of stage `%s` (`%s.log`, written %s):\n\n```\n%s\n```\n' "$stage" "$stage" "$(date -u -r "$local_ready/$stage.log" +%H:%M:%SZ 2>/dev/null)" "$(tail -30 "$local_ready/$stage.log" 2>/dev/null | LC_ALL=C sed $'s/\x1b\\[[0-9;]*[A-Za-z]//g' | cut -c1-400)"
  } >> "$out"
  python3 -c 'import json,sys; sys.exit(0 if json.load(open(sys.argv[1])).get("ready") is True else 1)' "$store/ready.json" 2>/dev/null && { echo "READY"; exit 0; }
  sleep 600
done
echo "NOT READY after 24 more hours"
