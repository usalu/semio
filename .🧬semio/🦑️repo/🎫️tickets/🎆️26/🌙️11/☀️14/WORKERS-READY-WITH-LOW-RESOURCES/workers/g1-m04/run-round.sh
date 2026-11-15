#!/bin/bash
# 🛫 Runs one round of the worker protocol on g1-m04 (Linux): the native bootstrap in its own process group, a sample of the three largest processes and the memory pressure every 30 s, and one SIGINT when a hard stop of the round is met. From round 2 on, an inference above 300 projects is a stop only when no line about the fleet precedes it. Usage: run-round.sh <round> [<try>]
set -u
round="$1"; try="${2:-1}"; suffix=""; [ "$try" -gt 1 ] && suffix="-try-$try"
W="$(cd "$(dirname "$0")/../.." && pwd)"; M="$W/workers/g1-m04"
log="$HOME/native-bootstrap-round-$round$suffix.log"; samples="$M/round-$round$suffix.samples.txt"; monitor="$M/round-$round$suffix.monitor.txt"
: > "$samples"; : > "$monitor"
set -m
bash "$W/native-bootstrap.sh" "$log" &
pid=$!
set +m
echo "started pid=$pid at $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$monitor"
stopped=""; peak=0; high=0
while kill -0 "$pid" 2>/dev/null; do
  now="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  top="$(ps -eo rss,pid,ppid,args --sort=-rss | sed -n 2,4p | cut -c1-400 | tr '\n' '\r' | sed 's/\r/\n/g')"
  ps -eo rss,pid,ppid,comm --sort=-rss | awk '$4=="git" && $1>1000000 {print $2}' | while read -r g; do echo "## $now git above 1 GB, chain:" >> "$M/round-$round$suffix.chain.txt"; c="$g"; while [ -n "$c" ] && [ "$c" -gt 0 ]; do ps -o pid=,ppid=,args= -p "$c" | cut -c1-300 >> "$M/round-$round$suffix.chain.txt"; c="$(ps -o ppid= -p "$c" | tr -d ' ')"; done; done
  full="$(awk '/^full/{sub("avg10=","",$2);print $2}' /proc/pressure/memory 2>/dev/null || echo ?)"
  { echo "## $now psi_memory_full_avg10=$full $(awk '/MemAvailable|SwapFree/{printf "%s%s kB ",$1,$2}' /proc/meminfo)"; echo "$top"; } >> "$samples"
  first="$(echo "$top" | head -1 | awk '{print $1}')"
  [ "${first:-0}" -gt "$peak" ] && peak="$first"
  reason=""
  [ "${first:-0}" -gt 8388608 ] && reason="process above 8 GB resident (${first} KiB)"
  if [ "$full" != "?" ] && [ "${full%.*}" -ge 20 ]; then high=$((high + 1)); else high=0; fi
  [ "$high" -ge 2 ] && reason="${reason:+$reason; }memory pressure (PSI full avg10 $full %, second sample in a row)"
  if [ -n "$reason" ] && [ -z "$stopped" ]; then
    stopped="$reason"; echo "HARD STOP at $now: $reason; sending one SIGINT to process group $pid" >> "$monitor"
    kill -INT -- "-$pid" 2>/dev/null
    sleep 60
    echo "one minute after SIGINT: alive=$(kill -0 "$pid" 2>/dev/null && echo yes || echo no)" >> "$monitor"
    break
  fi
  sleep 30
done
wait "$pid" 2>/dev/null
echo "ended at $(date -u +%Y-%m-%dT%H:%M:%SZ) wait_rc=$? peak_rss_kib=$peak hard_stop=${stopped:-none}" >> "$monitor"
