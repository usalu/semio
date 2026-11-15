#!/bin/bash
# 🔁 Runs the further tries of a round: goes on after a hard stop (3 minutes), after native_rc 75 or a fleet refusal text (2 minutes); ends on anything else so that the agent reads the result. Usage: retry-round.sh <round> <first try> <last try> <seconds to wait first>
set -u
round="$1"; k="$2"; last="$3"; pause="$4"
M="$(cd "$(dirname "$0")" && pwd)"; out="$M/round-$round.tries.txt"
while [ "$k" -le "$last" ]; do
  sleep "$pause"
  bash "$M/run-round.sh" "$round" "$k" > /dev/null 2>&1
  log="$HOME/native-bootstrap-round-$round-try-$k.log"
  rc="$(grep -a '^native_rc=' "$log" | tail -1 | cut -d= -f2 | cut -d' ' -f1)"
  stop="$(grep -c 'HARD STOP' "$M/round-$round-try-$k.monitor.txt")"
  refusal="$(grep -acE 'could not be read on this machine|graph-wanted' "$log")"
  echo "try $k ended $(date -u +%Y-%m-%dT%H:%M:%SZ) native_rc=$rc hard_stop=$stop refusal_lines=$refusal" >> "$out"
  if [ "$stop" != "0" ]; then pause=180; elif [ "$rc" = "75" ] || [ "$refusal" != "0" ]; then pause=120; else break; fi
  k=$((k + 1))
done
