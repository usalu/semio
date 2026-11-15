#!/bin/bash
# 🧪 Runs one try of a round of the worker protocol on g1-m04 from round 14 on: the native bootstrap in its own process group; every 5 s the slice worker's resident size beside the newest progress line of the graph service (the sampler of ROUND-NOTE-P12 for Linux); every 30 s the three largest processes; one SIGINT on a hard stop (a process above 8 GB resident, MemAvailable under 1 GB in two samples in a row), on 15 minutes without a new log line, or on 5 minutes of waiting for the dependency store. Usage: run-try-14.sh <round> <try>
set -u
round="$1"; try="$2"; suffix=""; [ "$try" -gt 1 ] && suffix="-try-$try"
W="$(cd "$(dirname "$0")/../.." && pwd)"; M="$W/workers/g1-m04"; root="$HOME/semio"; uid="$(id -u)"
log="$HOME/native-bootstrap-round-$round$suffix.log"; samples="$M/round-$round$suffix.samples.txt"; monitor="$M/round-$round$suffix.monitor.txt"; p12="$M/round-$round$suffix.p12.txt"
: > "$samples"; : > "$monitor"; : > "$p12"
strip() { sed $'s/\x1b\\[[0-9;]*m//g' "$@"; }
entries() { find "$root"/.cache/b/g/m/inference/selections/*/ce/e -type f 2>/dev/null | wc -l; }
echo "closure entries before: $(entries)" >> "$p12"
set -m
bash "$W/native-bootstrap.sh" "$log" &
pid=$!
set +m
echo "started pid=$pid at $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$monitor"
stopped=""; peak=0; low=0; tick=0; storeSince=0; slicePeak=0; slicePeakLine=""
while kill -0 "$pid" 2>/dev/null; do
  now="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  host="$(ls -t /run/user/"$uid"/semio-repo/"$uid"/host-*.log 2>/dev/null | head -1)"
  row="$(ps -eo pid=,rss=,args= | grep -F slice-pool | grep -F worker.ts | grep -v grep | sort -k2 -nr | head -1)"
  if [ -n "$row" ]; then
    spid="$(echo "$row" | awk '{print $1}')"; smb="$(echo "$row" | awk '{printf "%d", $2/1024}')"
    last="$(tail -n 600 "$host" 2>/dev/null | strip | grep -aE 'Reading the project graph slice|graph: plan stage|^graph: ' | tail -1 | cut -c1-220)"
    echo "$now pid=$spid rss=${smb}MB swap=$(awk '/VmSwap/{printf "%d", $2/1024}' /proc/"$spid"/status 2>/dev/null)MB last=[$last]" >> "$p12"
    if [ "$smb" -gt "$slicePeak" ]; then slicePeak="$smb"; slicePeakLine="$last"; fi
  else echo "$now no slice worker" >> "$p12"; fi
  if [ $((tick % 6)) = 0 ]; then
    top="$(ps -eo rss,pid,ppid,args --sort=-rss | sed -n 2,4p | cut -c1-300)"
    available="$(awk '/MemAvailable/{print $2}' /proc/meminfo)"
    { echo "## $now MemAvailable=${available}kB $(awk '/SwapFree/{print "SwapFree=" $2 "kB"}' /proc/meminfo) psi_full_avg10=$(awk '/^full/{sub("avg10=","",$2);print $2}' /proc/pressure/memory)"; echo "$top"; } >> "$samples"
    first="$(echo "$top" | head -1 | awk '{print $1}')"; [ "${first:-0}" -gt "$peak" ] && peak="$first"
    if [ "$available" -lt 1048576 ]; then low=$((low + 1)); else low=0; fi
    reason=""
    [ "${first:-0}" -gt 8388608 ] && reason="process above 8 GB resident (${first} KiB)"
    [ "$low" -ge 2 ] && reason="${reason:+$reason; }MemAvailable under 1 GB in two samples in a row (${available} kB)"
    idle=$(( $(date +%s) - $(stat -c %Y "$log" 2>/dev/null || date +%s) ))
    [ "$idle" -ge 900 ] && reason="${reason:+$reason; }no new log line for $idle s"
    if strip "$log" | tail -n 3 | grep -aq 'Another process holds the dependency store'; then [ "$storeSince" = 0 ] && storeSince="$(date +%s)"; else storeSince=0; fi
    if [ "$storeSince" != 0 ] && [ $(( $(date +%s) - storeSince )) -ge 300 ]; then
      reason="${reason:+$reason; }dependency store wait above 5 minutes"
      { echo "## dependency store wait at $now"; strip "$log" | grep -a 'Another process holds the dependency store' | tail -1; ps -eo pid,ppid,etime,args | grep -E 'bun|nx|repo' | grep -v grep | cut -c1-260; } > "$M/round-$round$suffix.store-wait.txt"
    fi
    if [ -n "$reason" ]; then
      stopped="$reason"; echo "STOP at $now: $reason; sending one SIGINT to process group $pid" >> "$monitor"
      [ -n "$host" ] && strip "$host" | tail -n 60 | cut -c1-600 > "$M/round-$round$suffix.service-last60.txt"
      kill -INT -- "-$pid" 2>/dev/null; sleep 60
      echo "one minute after SIGINT: alive=$(kill -0 "$pid" 2>/dev/null && echo yes || echo no)" >> "$monitor"
      break
    fi
  fi
  tick=$((tick + 1)); sleep 5
done
wait "$pid" 2>/dev/null
echo "closure entries after: $(entries)" >> "$p12"
echo "slice worker peak: ${slicePeak} MB at [$slicePeakLine]" >> "$p12"
echo "ended at $(date -u +%Y-%m-%dT%H:%M:%SZ) peak_rss_kib=$peak slice_peak_mb=$slicePeak stop=${stopped:-none}" >> "$monitor"
