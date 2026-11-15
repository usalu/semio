#!/bin/bash
# 🧗 Runs the tries <first> to <last> of round 18 on g1-m06 one after the other, appends a block per try to the report (times, exit code, graph, fleet and capacity lines, setup task lists, first error with the lines before it, launcher identity, pressure levels, peaks of both samplers) and refreshes the tail and the state file after every try; it goes on 3 minutes after exit 75, a hard stop or exit 4 (with the EPERM lines and the service processes, at most three in a row) and ends on every other outcome.
# Usage: bash round-18-tries.sh <first> <last>
# @see round-18-try.sh
set -u
here=$(cd "$(dirname "$0")" && pwd)
marker=$HOME/Documents/semio/.cache/f/native-bootstrap.done
denied=0
for try in $(seq "$1" "$2"); do
  if [ "$try" = 1 ]; then log=$HOME/native-bootstrap-round-18.log; else log=$HOME/native-bootstrap-round-18-try-$try.log; fi
  [ -f "$log" ] || bash "$here/round-18-try.sh" "$try"
  samples=${log%.log}.samples.txt
  p12=${log%.log}.p12.txt
  rc=$(tail -n 1 "$log" | sed -n 's/^native_rc=//p')
  stop=$(sed -n 's/.*runner ended, hard stop: //p' "$samples")
  failed=$(grep -E '^✖' "$log" | sed -E 's/^✖ +nx run //; s/:prototype.*//' | sort -u | tr '\n' ' ')
  first=$(grep -nE 'error(\[E[0-9]+\])?:' "$log" | head -n 1 | cut -d: -f1)
  {
    printf '\n## Try %s\n\n' "$try"
    printf -- '- %s, %s, `native_rc=%s`, hard stop: %s.\n' "$(head -n 1 "$log")" "$(tail -n 2 "$log" | head -n 1 | cut -c1-80)" "${rc:-not measured}" "${stop:-not measured}"
    printf -- '- Pressure levels of the 30 s samples (count, level): %s. Peak resident of any process: %s KB.\n' "$(awk '/pressure_level/{print $2}' "$samples" | sort | uniq -c | tr -s ' ' | tr '\n' ';')" "$(grep -A1 '^all:' "$samples" | grep -vE 'all:|--' | sort -nr | head -n 1 | awk '{print $1}')"
    printf -- '- Slice worker sampler (5 s), row with the largest resident size: `%s`; %s; %s.\n' "$(grep 'rss=' "$p12" | sed -E 's/.*rss=([0-9]+)MB.*/\1 &/' | sort -nr | head -n 1 | cut -d' ' -f2- | cut -c1-330)" "$(grep 'closure entries before' "$p12")" "$(grep 'closure entries after' "$p12")"
    printf -- '- Launcher after the try: %s; marker %s.\n' "$(grep -A9 "try $try after" "$HOME/native-bootstrap-round-18.notes.txt" | grep -E '^inode|^Identifier|^CDHash' | tr '\n' ' ')" "$([ -f "$marker" ] && echo present || echo absent)"
    printf -- '- Setup tasks: succeeded %s, failed %s (%s).\n\n```\n' "$(grep -cE '^✔' "$log")" "$(grep -cE '^✖' "$log")" "${failed:-none}"
    grep -E '^graph:|^\[capacity\]|restored .* from a state|runs under|falls back|too heavy|taken from the fleet|inferred here' "$log" | awk '!seen[$0]++' | cut -c1-500
    grep -E '^Reading the project graph slice' "$log" | tail -n 2 | cut -c1-300
    grep -E 'NX   |^✔|^✖' "$log" | cut -c1-200
    grep -vE '^\[script\]|^Reading the project graph slice|still working|still running' "$log" | tail -n 4 | cut -c1-1500
    printf '```\n'
    [ -n "$first" ] && { printf '\nFirst error line (log line %s) and the 15 lines before it:\n\n```\n' "$first"; sed -n "$((first > 15 ? first - 15 : 1)),${first}p" "$log" | cut -c1-300; printf '```\n'; }
    [ -f "${log%.log}.stall.txt" ] && { printf '\nCapture at the stop:\n\n```\n'; cat "${log%.log}.stall.txt"; printf '```\n'; }
    if [ "$rc" = 4 ]; then
      printf '\nEPERM lines of the newest graph service log with the 10 lines before, and the service processes:\n\n```\n'
      grep -B10 -E 'EPERM|where: root=' "$(ls -t "$TMPDIR"semio-repo/"$(id -u)"/host-*.log | head -n 1)" | tail -n 60 | cut -c1-400
      ps -axo pid,ppid,etime,command | grep 'service host' | grep -v grep | cut -c1-300
      printf '```\n'
    fi
  } >> "$here/round-18.md"
  tail -n 200 "$log" > "$here/round-18.tail.txt"
  ready=false; [ "$rc" = 0 ] && [ -f "$marker" ] && ready=true
  list=$(printf '%s' "$failed" | awk '{for(i=1;i<=NF;i++) printf "%s\"%s\"", (i>1?",":""), $i}')
  printf '{"origin":"g1-m06","round":18,"native_rc":%s,"ready":%s,"failed":[%s],"tries":%s,"at":"%s"}\n' "${rc:-null}" "$ready" "$list" "$try" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$here/state.json"
  if [ "$rc" = 4 ]; then denied=$((denied+1)); else denied=0; fi
  [ "$denied" -ge 3 ] && break
  if [ "$rc" = 75 ] || [ "$rc" = 4 ] || [ "$stop" != none ]; then sleep 180; else break; fi
done
