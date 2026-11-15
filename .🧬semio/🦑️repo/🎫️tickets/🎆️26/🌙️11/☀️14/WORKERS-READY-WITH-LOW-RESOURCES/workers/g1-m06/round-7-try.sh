#!/bin/bash
# 🎬 Runs one try of round 7 on g1-m06 with the runner, records the line counts of the graph service logs before and after it and, when the try ends with the graph service timeout, starts the registered service jobs with launchctl kickstart as round 7 allows, recording their launchd state before and after.
# Usage: bash round-7-try.sh <try>
# @see round-run.sh
set -u
here=$(cd "$(dirname "$0")" && pwd)
try=$1
if [ "$try" = 1 ]; then log=$HOME/native-bootstrap-round-7.log; else log=$HOME/native-bootstrap-round-7-try-$try.log; fi
notes=$HOME/native-bootstrap-round-7.notes.txt
counts() { for host in "$TMPDIR"semio-repo/"$(id -u)"/host-*.log; do printf 'try %s %s %s %s\n' "$try" "$1" "$(wc -l < "$host" | tr -d ' ')" "$host"; done >> "$notes"; }
state() { for job in $(launchctl list | awk '/tech.semio.repo.service/{print $3}'); do printf 'try %s %s %s\n' "$try" "$1" "$job" >> "$notes"; launchctl print "gui/$(id -u)/$job" | grep -E 'state =|runs =|pended nondemand spawn|last exit code' | head -4 >> "$notes"; done; }
counts before
bash "$here/round-run.sh" "$log"
counts after
if tail -n 5 "$log" | grep -q 'The graph service could not answer (timeout)'; then
  state before-kickstart
  for job in $(launchctl list | awk '/tech.semio.repo.service/{print $3}'); do launchctl kickstart "gui/$(id -u)/$job"; printf 'try %s kickstart %s %s\n' "$try" "$job" "$?" >> "$notes"; done
  sleep 10
  state after-kickstart
fi
