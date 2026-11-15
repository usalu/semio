#!/bin/bash
# 🔁 Runs round 5 of the worker protocol on g1-m04: up to ten tries of the native bootstrap; two minutes of pause after a refusal of the fleet or a code change under the run, three minutes after a hard stop; after every try the lines of the graph service's log that the round asks for are kept. Usage: run-tries-5.sh
set -u
round=5; M="$(cd "$(dirname "$0")" && pwd)"; out="$M/round-$round.tries.txt"; : > "$out"
strip() { sed $'s/\x1b\\[[0-9;]*m//g' "$@"; }
for try in 1 2 3 4 5 6 7 8 9 10; do
  suffix=""; [ "$try" -gt 1 ] && suffix="-try-$try"
  log="$HOME/native-bootstrap-round-$round$suffix.log"; began="$(date +%s)"
  bash "$M/run-round.sh" "$round" "$try"
  rc="$(grep -a '^native_rc=' "$log" | tail -1)"
  refused="$(strip "$log" | grep -aEic 'graph-wanted|wait(ed|ing) for the fleet|the fleet was asked|^native_rc=75')"
  changed="$(strip "$log" | grep -aEic 'code changed|Restarting with the current implementation|implementation changed')"
  echo "try $try: $(sed -n 1p "$log") $(grep -a '^end ' "$log" | tail -1) ${rc:-native_rc=none} fleet_lines=$refused code_change_lines=$changed $(tail -1 "$M/round-$round$suffix.monitor.txt")" >> "$out"
  service="$M/round-$round$suffix.service.txt"; : > "$service"
  for host in /run/user/"$(id -u)"/semio-repo/"$(id -u)"/host-*.log; do
    [ "$(stat -c %Y "$host")" -ge "$began" ] || continue
    { echo "## $host"; strip "$host" | grep -aE 'waiting for the fleet|taken from the fleet|inferred here|refreshing [0-9]+ projects|asking the host for|stale facts' | cut -c1-600 | awk '{k=$0; gsub(/[0-9]+ s\b/,"N s",k)} !s[k]++'; echo "## last 30 lines"; strip "$host" | tail -n 30 | cut -c1-600; } >> "$service"
  done
  case "$rc" in "native_rc=0"*) break;; esac
  if grep -q 'HARD STOP' "$M/round-$round$suffix.monitor.txt"; then [ "$try" -lt 10 ] && sleep 180; continue; fi
  [ "$refused" -gt 0 ] || [ "$changed" -gt 0 ] || break
  [ "$try" -lt 10 ] && sleep 120
done
echo "round $round finished at $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$out"
