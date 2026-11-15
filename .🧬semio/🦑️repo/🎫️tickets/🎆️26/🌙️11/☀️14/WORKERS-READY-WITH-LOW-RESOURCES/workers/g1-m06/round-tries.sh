#!/bin/bash
# 🔂 Runs one round on g1-m06: one bootstrap run and, while a run ends with `native_rc=75`, a fleet refusal text or a hard stop, another one after the pause for that ending, up to the given number of runs; the line count of every graph service log before and after each run is recorded so its lines can be attributed to the run.
# Usage: bash round-tries.sh <round> <tries> <pause after refusal> <pause after hard stop>
# @see round-run.sh
set -u
here=$(cd "$(dirname "$0")" && pwd)
marker=$HOME/Documents/semio/.cache/f/native-bootstrap.done
marks=$HOME/native-bootstrap-round-$1.marks.txt
counts() { for host in "$TMPDIR"semio-repo/"$(id -u)"/host-*.log; do printf '%s %s %s %s\n' "$1" "$2" "$(wc -l < "$host" | tr -d ' ')" "$host"; done >> "$marks"; }
: > "$marks"
for try in $(seq 1 "$2"); do
  if [ "$try" = 1 ]; then log=$HOME/native-bootstrap-round-$1.log; else log=$HOME/native-bootstrap-round-$1-try-$try.log; fi
  counts "$try" before
  bash "$here/round-run.sh" "$log"
  counts "$try" after
  [ -f "$marker" ] && break
  [ "$try" = "$2" ] && break
  if grep -q 'HARD STOP' "${log%.log}.samples.txt"; then sleep "$4"; continue; fi
  tail -n 1 "$log" | grep -q 'native_rc=75' || tail -n 12 "$log" | grep -aqiE 'graph-wanted|the fleet was asked|unexpected failure|code changed|Restarting' || break
  sleep "$3"
done
