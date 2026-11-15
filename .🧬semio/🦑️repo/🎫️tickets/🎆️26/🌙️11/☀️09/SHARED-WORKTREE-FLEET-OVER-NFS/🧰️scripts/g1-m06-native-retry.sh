#!/bin/bash
# 🔁 Retries the native bootstrap of the fleet worktree as the worker account until the completion marker exists, so a run that waits for memory headroom or meets a build closure another machine changed is repeated without a session.
# Usage: nohup bash g1-m06-native-retry.sh [attempts] >/dev/null 2>&1 &
# @see ../../../☀️13/FINISH-WORKER-SETUP-AND-MERGE-PRE-FLEET-FOLDER-ON-MAC-MINI-02/native-bootstrap.sh
set -u
worktree=$HOME/Documents/semio
marker=$worktree/.cache/f/native-bootstrap.done
record=$worktree/.cache/f/native-retry.log
step=$worktree/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FINISH-WORKER-SETUP-AND-MERGE-PRE-FLEET-FOLDER-ON-MAC-MINI-02/native-bootstrap.sh
attempts=${1:-24}
for attempt in $(seq 1 "$attempts"); do
  [ -f "$marker" ] && break
  printf '%s attempt %s of %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$attempt" "$attempts" >> "$record"
  bash "$step" "$worktree/.cache/f/native-attempt.log"
  tail -n 1 "$worktree/.cache/f/native-attempt.log" >> "$record"
  [ -f "$marker" ] && break
  sleep 120
done
if [ -f "$marker" ]; then printf '%s done\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$record"; else printf '%s gave up after %s attempts\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$attempts" >> "$record"; fi
