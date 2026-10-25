#!/bin/bash
# Ticket input script: repeats `five.sh` every ten minutes while the dev site cannot start for a reason of a peer's (the log says the dev site exited before it became ready); stops after a verdict of the specs or three hours.
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
log="$1"; prefix="$2"
for attempt in $(seq 1 18); do
  echo "[five-until] attempt $attempt $(date +%H:%M:%S)" >> "$log"
  bash "$here/five.sh" "$prefix$attempt-" 5 > "$log.part" 2>&1; code=$?
  cat "$log.part" >> "$log"
  if [ "$code" = 0 ]; then echo "[five-until] PASSED" >> "$log"; exit 0; fi
  /usr/bin/grep -aq "exited with status 1 before it became ready\|did not expect test()" "$log.part" || { echo "[five-until] FAILED for a reason of the specs" >> "$log"; exit 1; }
  sleep 600
done
exit 75
