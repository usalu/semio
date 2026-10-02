#!/bin/zsh
# 🧹️ Every 5 min: kills cargo/cargo-nextest processes orphaned by a cut agent (parent reparented to launchd, ppid 1) that sat
# at 0 % CPU with no child process on two consecutive sweeps — they hold the shared build-dir locks and stall every live build.
log="${0:A:h}/🗑️generated/coord/orphan-cargo-guard.txt"
mkdir -p "${log:h}"
typeset -A seen
while true; do
  typeset -A now
  for line in "${(@f)$(ps -Ao pid=,ppid=,pcpu=,comm= | awk '$2==1 && ($4 ~ /\/cargo$/ || $4 ~ /cargo-nextest$/) && $3+0 < 0.5 {print $1}')}"; do
    [ -z "$line" ] && continue
    pgrep -P "$line" >/dev/null && continue
    now[$line]=1
    if [ -n "${seen[$line]}" ]; then
      echo "$(date '+%F %T') kill orphan cargo $line: $(ps -o command= -p $line | cut -c1-160)" >> "$log"
      kill -KILL "$line" 2>/dev/null
    fi
  done
  seen=("${(@kv)now}")
  sleep 300
done
