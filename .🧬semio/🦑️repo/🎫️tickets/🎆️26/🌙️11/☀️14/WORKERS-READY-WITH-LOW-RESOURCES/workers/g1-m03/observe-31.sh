#!/bin/bash
# 👀 Round 31 observation: writes round-31.md from the fleet agent's ready.json, the running stage's log, memory pressure and the largest processes, and appends one line to round-31.history.txt. Starts nothing. Usage: observe-31.sh
here=$(cd "$(dirname "$0")" && pwd); o=/Volumes/semio-store/v1/origins/g1-m03; r="$o/ready.json"; now=$(date -u +%Y-%m-%dT%H:%M:%SZ)
f(){ python3 -I -c 'import json,sys
try:
  d=json.load(open(sys.argv[1]))
except Exception as e:
  print("unreadable: %s"%e); sys.exit()
k=sys.argv[2]
if k=="line": print("state=%s stage=%s (%s of %s) attempts=%s exit=%s refusal=%s reason=%s ready=%s peakMegabytes=%s"%(d.get("state"),d.get("stage"),d.get("stageIndex"),d.get("stageCount"),d.get("attempts"),d.get("exit"),json.dumps(d.get("refusal")),(d.get("reason") or {}).get("code"),d.get("ready"),d.get("peakMegabytes")))
elif k=="stage": print(d.get("stage") or "")
elif k=="ready": print("true" if d.get("ready") is True else "false")
else: print(json.dumps(d,indent=1,ensure_ascii=False))' "$r" "$1"; }
line=$(f line); stage=$(f stage); log="$o/ready-log-$stage.txt"
mp=$(memory_pressure | tail -1); unc=$(ps ax -o pid,command | grep -c '[U]serNotificationCenter')
echo "$now $line | $mp" >> "$here/round-31.history.txt"
{ echo "# Round 31 on g1-m03: Observation Only"; echo
  echo "Observed $now. I run no setup command. Try 5 of round 30 was stopped with one SIGINT at 20:52:36Z (see \`round-30.md\`); my runner is ended."; echo
  echo "## ready.json (\`$r\`)"; echo; echo "- $line"; echo; echo '```json'; f all; echo '```'; echo
  echo "## Last 30 lines of the running stage's log (\`ready-log-$stage.txt\`)"; echo; echo '```'; tail -30 "$log" 2>&1 | cut -c1-600; echo '```'; echo
  echo "## Memory"; echo; echo "- \`memory_pressure | tail -1\`: $mp"; echo
  echo "## Five largest processes by resident size (KiB, pid, ppid, elapsed, command)"; echo; echo '```'; ps -axo rss,pid,ppid,etime,command | sort -k1 -nr | head -5 | cut -c1-400; echo '```'; echo
  echo "## macOS permission dialog"; echo; echo "- I cannot look at the screen from this shell. Processes named UserNotificationCenter (the process that shows such dialogs): $unc. I clicked nothing."; echo
  echo "## History (one line per observation, about every 10 minutes)"; echo; echo '```'; cat "$here/round-31.history.txt" | cut -c1-400; echo '```'
} > "$here/round-31.md"
echo "$now $line"; f ready
