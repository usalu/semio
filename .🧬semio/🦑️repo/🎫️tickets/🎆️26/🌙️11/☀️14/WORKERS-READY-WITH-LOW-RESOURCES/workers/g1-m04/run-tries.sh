#!/bin/bash
# 🔂 Runs a round of the worker protocol on g1-m04 with its retries: after a try that ends with the designed refusal of the fleet (the log names `graph-wanted` or a wait for the fleet) it waits and tries again, by default five minutes and six tries; any other end, a hard stop or success ends the round. Usage: run-tries.sh <round> [<tries> [<pause seconds>]]
set -u
round="$1"; max="${2:-6}"; pause="${3:-300}"; M="$(cd "$(dirname "$0")" && pwd)"; out="$M/round-$round.tries.txt"; : > "$out"
for try in $(seq 1 "$max"); do
  suffix=""; [ "$try" -gt 1 ] && suffix="-try-$try"
  log="$HOME/native-bootstrap-round-$round$suffix.log"
  bash "$M/run-round.sh" "$round" "$try"
  rc="$(grep -a '^native_rc=' "$log" | tail -1)"
  refused="$(sed $'s/\x1b\\[[0-9;]*m//g' "$log" | grep -aEic 'graph-wanted|wait(ed|ing) for the fleet|^native_rc=75')"
  echo "try $try: $(sed -n 1p "$log") $(grep -a '^end ' "$log" | tail -1) ${rc:-native_rc=none} fleet_lines=$refused $(tail -1 "$M/round-$round$suffix.monitor.txt")" >> "$out"
  case "$rc" in "native_rc=0"*) break;; esac
  grep -q 'HARD STOP' "$M/round-$round$suffix.monitor.txt" && break
  [ "$refused" -gt 0 ] || break
  [ "$try" -lt "$max" ] && sleep "$pause"
done
echo "round $round finished at $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$out"
