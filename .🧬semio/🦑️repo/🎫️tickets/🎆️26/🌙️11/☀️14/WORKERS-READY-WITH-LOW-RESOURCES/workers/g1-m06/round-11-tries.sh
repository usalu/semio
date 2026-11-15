#!/bin/bash
# 🪜 Runs the tries <first> to <last> of round 11 on g1-m06 one after the other, appends a block per try to the report (times, exit code, refusal text, graph and fleet lines, launcher identity, pressure levels, peak resident) and refreshes the tail and the state file; it goes on after exit 75 (2 minutes), after a hard stop or a stalled graph step (3 minutes) and ends on every other outcome or when the round number changes.
# Usage: bash round-11-tries.sh <first> <last>
# @see round-11-try.sh
set -u
here=$(cd "$(dirname "$0")" && pwd)
ticket=$(cd "$here/../.." && pwd)
marker=$HOME/Documents/semio/.cache/f/native-bootstrap.done
for try in $(seq "$1" "$2"); do
  grep -q '^round: 11$' "$ticket/ROUND.md" || break
  if [ "$try" = 1 ]; then log=$HOME/native-bootstrap-round-11.log; else log=$HOME/native-bootstrap-round-11-try-$try.log; fi
  [ -f "$log" ] || bash "$here/round-11-try.sh" "$try"
  samples=${log%.log}.samples.txt
  rc=$(tail -n 1 "$log" | sed -n 's/^native_rc=//p')
  stop=$(sed -n 's/.*runner ended, hard stop: //p' "$samples")
  {
    printf '\n## Try %s\n\n' "$try"
    printf -- '- %s, %s, `native_rc=%s`, hard stop: %s.\n' "$(head -n 1 "$log")" "$(tail -n 2 "$log" | head -n 1)" "${rc:-not measured}" "${stop:-not measured}"
    printf -- '- Pressure levels of the samples (count, level): %s. Peak resident: %s KB.\n' "$(awk '/pressure_level/{print $2}' "$samples" | sort | uniq -c | tr -s ' ' | tr '\n' ';')" "$(grep -A1 '^all:' "$samples" | grep -vE 'all:|--' | sort -nr | head -n 1 | awk '{print $1}')"
    printf -- '- Launcher after the try: %s; marker %s.\n' "$(grep -A9 "try $try after" "$HOME/native-bootstrap-round-11.notes.txt" | grep -E '^inode|^Identifier|^CDHash' | tr '\n' ' ')" "$([ -f "$marker" ] && echo present || echo absent)"
    printf -- '- EPERM lines in the run log: %s.\n\n```\n' "$(grep -c EPERM "$log")"
    grep -E '^graph:|^\[capacity\]|runs under|fall' "$log" | awk '!seen[$0]++' | cut -c1-400
    grep -E '^Reading the project graph slice' "$log" | tail -n 2 | cut -c1-300
    grep -vE '^\[script\]|^Reading the project graph slice|still working|still running' "$log" | tail -n 4 | cut -c1-1500
    printf '```\n'
    [ -f "${log%.log}.stall.txt" ] && { printf '\nStall capture:\n\n```\n'; cat "${log%.log}.stall.txt"; printf '```\n'; }
  } >> "$here/round-11.md"
  tail -n 200 "$log" > "$here/round-11.tail.txt"
  ready=false; [ "$rc" = 0 ] && [ -f "$marker" ] && ready=true
  printf '{"origin":"g1-m06","round":11,"native_rc":%s,"ready":%s,"failed":[],"tries":%s,"at":"%s"}\n' "${rc:-null}" "$ready" "$try" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$here/state.json"
  if [ "$rc" = 75 ]; then sleep 120; elif [ "$stop" != none ]; then sleep 180; else break; fi
done
