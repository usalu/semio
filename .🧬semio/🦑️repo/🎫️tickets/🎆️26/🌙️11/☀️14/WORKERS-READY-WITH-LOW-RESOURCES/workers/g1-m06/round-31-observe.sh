#!/bin/bash
# 👁️ Observes the setup that the fleet agent drives by itself on g1-m06 (round 31) without starting anything: every 10 minutes it rewrites round-31.md with the ready record of this machine from the store, the last 30 lines of the running stage's log, load, memory pressure and the largest processes, keeps one history line per observation, and ends when the record says ready or after the given number of observations.
# Usage: bash round-31-observe.sh <observations>
# @see ../../ROUND.md
set -u
here=$(cd "$(dirname "$0")" && pwd)
store=/Volumes/semio-store/v1/origins/g1-m06
ready=$HOME/Documents/semio/.cache/f/ready
history=$HOME/round-31.history.txt
field() { python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); v=d.get(sys.argv[2]); print(json.dumps(v, ensure_ascii=False) if isinstance(v,(dict,list)) else v)' "$store/ready.json" "$1" 2>/dev/null; }
for turn in $(seq 1 "$1"); do
  now=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  stage=$(field stage)
  printf '%s state=%s stage=%s attempts=%s exit=%s ready=%s peakMegabytes=%s load=%s pressure_level=%s\n' "$now" "$(field state)" "$stage" "$(field attempts)" "$(field exit)" "$(field ready)" "$(field peakMegabytes)" "$(uptime | sed 's/.*load averages: //')" "$(sysctl -n kern.memorystatus_vm_pressure_level)" >> "$history"
  {
    printf '# g1-m06 Round 31\n\nObservation only; I start no setup command. Rewritten every 10 minutes, last at %s. Machine: Mac mini, 16 GB memory, macOS 27.2.1. Whether a macOS permission dialog waits on screen: not measured (my shell cannot see the screen); the ready record below names a refusal if the tools met one.\n\n' "$now"
    printf '## Ready Record (`origins/g1-m06/ready.json` in the store)\n\n```\nstate: %s\nstage: %s\nattempts: %s\nexit: %s\nrefusal: %s\nreason: %s\nready: %s\npeakMegabytes: %s\n```\n\n' "$(field state)" "$stage" "$(field attempts)" "$(field exit)" "$(field refusal)" "$(field reason)" "$(field ready)" "$(field peakMegabytes)"
    printf '## Last 30 Lines of the Log of Stage %s\n\n```\n' "$stage"
    tail -n 30 "$ready/$stage.log" 2>/dev/null | cut -c1-400
    printf '```\n\n## Load and Memory\n\n```\n%s\n' "$(uptime)"
    memory_pressure | tail -n 3
    printf '```\n\nLargest processes (`ps -axo pid,ppid,rss,pcpu,etime,command | sort -k3 -nr | head -25`, lines cut at 220 characters):\n\n```\n'
    ps -axo pid,ppid,rss,pcpu,etime,command | sort -k3 -nr | head -n 25 | cut -c1-220
    printf '```\n\n## History (One Line per Observation)\n\n```\n'
    cat "$history"
    printf '```\n'
  } > "$here/round-31.md.next"
  mv "$here/round-31.md.next" "$here/round-31.md"
  [ "$(field ready)" = True ] && break
  sleep 600
done
