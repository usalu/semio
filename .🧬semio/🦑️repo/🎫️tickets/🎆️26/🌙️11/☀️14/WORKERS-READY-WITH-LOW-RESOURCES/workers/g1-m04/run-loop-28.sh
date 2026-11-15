#!/bin/bash
# 🎢 Runs round 28 on g1-m04: tries five minutes apart without a try limit (round 28), each through run-try-24.sh; `state.json` and the service lines after every try; ends on success, when ROUND.md names a higher round, or after six hours. Usage: run-loop-28.sh
set -u
round=28; M="$(cd "$(dirname "$0")" && pwd)"; W="$(cd "$M/../.." && pwd)"; out="$M/round-$round.tries.txt"; : > "$out"; deadline=$(( $(date +%s) + 21600 ))
strip() { sed $'s/\x1b\\[[0-9;]*m//g' "$@"; }
try=1
while :; do
  suffix=""; [ "$try" -gt 1 ] && suffix="-try-$try"
  log="$HOME/native-bootstrap-round-$round$suffix.log"; began="$(date +%s)"
  bash "$M/run-try-24.sh" "$round" "$try"
  rc="$(grep -a '^native_rc=' "$log" | tail -1)"; code="$(echo "${rc:-native_rc=1}" | grep -o '[0-9]*' | head -1)"
  echo "try $try: $(sed -n 1p "$log") $(grep -a '^end ' "$log" | tail -1) ${rc:-native_rc=none} $(tail -1 "$M/round-$round$suffix.monitor.txt")" >> "$out"
  ready=false; [ "$code" = 0 ] && [ -e "$HOME/semio/.cache/f/native-bootstrap.done" ] && ready=true
  printf '{"origin":"g1-m04","round":%s,"try":%s,"native_rc":%s,"ready":%s,"failed":[%s],"at":"%s"}\n' "$round" "$try" "$code" "$ready" "$([ "$code" = 0 ] || echo '"workspace:setup"')" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$M/state.json"
  service="$M/round-$round$suffix.service.txt"; : > "$service"
  for host in /run/user/"$(id -u)"/semio-repo/"$(id -u)"/host-*.log; do
    [ "$(stat -c %Y "$host")" -ge "$began" ] || continue
    { echo "## $host"; strip "$host" | grep -aE '^graph: |^graph-plan|\[graph\] closure-missing|access-proof|unexpected failure|request-failed' | cut -c1-1500 | awk '{k=$0; gsub(/[0-9]+/,"N",k)} !s[k]++'; echo "## last 30 lines"; strip "$host" | tail -n 30 | cut -c1-600; } >> "$service"
  done
  { echo "## last 40 lines of $log"; strip "$log" | grep -av 'Reading the project graph slice' | tail -n 40 | cut -c1-900; } > "$M/round-$round$suffix.bootstrap-last40.txt"
  [ "$code" = 0 ] && break
  now="$(grep -m1 '^round:' "$W/ROUND.md" | grep -o '[0-9]*')"; [ "${now:-$round}" -gt "$round" ] && { echo "ROUND.md names round $now" >> "$out"; break; }
  [ "$(date +%s)" -ge "$deadline" ] && { echo "six hours reached" >> "$out"; break; }
  try=$((try + 1)); sleep 300
done
echo "round $round finished at $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$out"
