#!/bin/bash
# 📈 Samples memory pressure and the three largest processes every 30 s while the given bootstrap process lives, with the hard-stop rules of round 6 (8 GB of the run, pressure 4 twice, pressure 2 six times) and a stall rule on the whole log (no new line for 15 minutes). Usage: sample-try.sh <pid> <samples-file> <note-file> <log>
pid=$1; samples=$2; note=$3; log=$4; twos=0; fours=0; stopped=0; last=""; same=0
while kill -0 "$pid" 2>/dev/null; do
  level=$(sysctl -n kern.memorystatus_vm_pressure_level)
  { date -u +"%Y-%m-%dT%H:%M:%SZ pressure=$level"; ps -axo rss,pid,ppid,args | sort -nr | head -3 | cut -c1-600; } >> "$samples"
  if [ "$level" = 2 ]; then twos=$((twos+1)); else twos=0; fi; if [ "$level" = 4 ]; then fours=$((fours+1)); else fours=0; fi
  size=$(wc -c < "$log"); if [ "$size" = "$last" ]; then same=$((same+1)); else same=0; last=$size; fi
  if [ "$stopped" = 0 ]; then
    reason=""; top=$(ps -axo pgid,rss | awk -v g="$pid" '$1==g && $2>m {m=$2} END {print m+0}')
    [ "$top" -gt 8388608 ] && reason="process of the run above 8 GB resident ($top KiB)"
    [ "$fours" -ge 2 ] && reason="memory pressure level 4 in two samples in a row"; [ "$twos" -ge 6 ] && reason="memory pressure level 2 in six samples in a row"
    [ "$same" -ge 30 ] && reason="no new log line for 15 minutes"
    if [ -n "$reason" ]; then echo "hard stop or stall $(date -u +%Y-%m-%dT%H:%M:%SZ): $reason; one SIGINT to process group $pid" >> "$note"; kill -INT -- "-$pid"; stopped=1; fi
  fi
  sleep 30
done
echo "try 3 process ended $(date -u +%Y-%m-%dT%H:%M:%SZ) stopped=$stopped" >> "$note"
