#!/bin/bash
# 🎡 Runs a round of the worker protocol on g1-m04 from round 24 on (exit 4 ends the series only after three in a row; the last 40 bootstrap lines are kept after exit 1 or 4): up to twelve tries three minutes apart; after every try `state.json` and the asked lines of the graph service's log are written; it goes on after the refusal 75, a stop of the runner, a task failure with `source-differs`, `closure.moving` or retry wording, or `closure-missing`, and ends on success, on exit 4, or after two tries in a row that failed otherwise. Usage: run-tries-24.sh <round> [<first try>]
set -u
round="$1"; first="${2:-1}"; M="$(cd "$(dirname "$0")" && pwd)"; out="$M/round-$round.tries.txt"; [ "$first" = 1 ] && : > "$out"
strip() { sed $'s/\x1b\\[[0-9;]*m//g' "$@"; }
other=0; four=0
for try in $(seq "$first" 12); do
  suffix=""; [ "$try" -gt 1 ] && suffix="-try-$try"
  log="$HOME/native-bootstrap-round-$round$suffix.log"; began="$(date +%s)"
  bash "$M/run-try-24.sh" "$round" "$try"
  rc="$(grep -a '^native_rc=' "$log" | tail -1)"; code="$(echo "${rc:-native_rc=1}" | grep -o '[0-9]*' | head -1)"
  again="$(strip "$log" | grep -aEic 'source-differs|closure\.moving|closure-missing|try again|retry')"
  echo "try $try: $(sed -n 1p "$log") $(grep -a '^end ' "$log" | tail -1) ${rc:-native_rc=none} retry_lines=$again $(tail -1 "$M/round-$round$suffix.monitor.txt")" >> "$out"
  ready=false; [ "$code" = 0 ] && [ -e "$HOME/semio/.cache/f/native-bootstrap.done" ] && ready=true
  printf '{"origin":"g1-m04","round":%s,"native_rc":%s,"ready":%s,"failed":[%s],"at":"%s"}\n' "$round" "$code" "$ready" "$([ "$code" = 0 ] || echo '"workspace:setup"')" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$M/state.json"
  service="$M/round-$round$suffix.service.txt"; : > "$service"
  for host in /run/user/"$(id -u)"/semio-repo/"$(id -u)"/host-*.log; do
    [ "$(stat -c %Y "$host")" -ge "$began" ] || continue
    { echo "## $host"; strip "$host" | grep -aE '^graph: |^graph-plan-too-heavy|\[graph\] closure-missing|\[graph-phase\]|access-proof|fstatat failed|where: root=|launchd route|serving' | cut -c1-1500 | awk '{k=$0; if (k ~ /waiting for the fleet/) gsub(/[0-9]+ of/,"N of",k)} !s[k]++'; echo "## last waiting line: $(strip "$host" | grep -a 'waiting for the fleet' | tail -1)"; echo "## last 30 lines"; strip "$host" | tail -n 30 | cut -c1-600; } >> "$service"
  done
  [ "$code" = 0 ] && break
  if [ "$code" = 1 ] || [ "$code" = 4 ]; then { echo "## last 40 lines of $log"; strip "$log" | tail -n 40 | cut -c1-600; echo "## service host processes"; ps -eo pid,ppid,etime,args | grep "service host" | grep -v grep | cut -c1-260; } > "$M/round-$round$suffix.bootstrap-last40.txt"; fi
  if [ "$code" = 4 ]; then four=$((four + 1)); [ "$four" -ge 3 ] && break; else four=0; fi
  if [ "$code" = 75 ] || [ "$code" = 4 ] || grep -q '^STOP' "$M/round-$round$suffix.monitor.txt" || [ "$again" -gt 0 ]; then other=0; else other=$((other + 1)); [ "$other" -ge 2 ] && break; fi
  [ "$try" -lt 12 ] && sleep 180
done
echo "round $round finished at $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$out"
