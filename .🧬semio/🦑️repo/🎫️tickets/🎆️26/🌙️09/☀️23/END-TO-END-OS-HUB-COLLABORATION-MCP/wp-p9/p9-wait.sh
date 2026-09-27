#!/bin/zsh
# ⏳️ Waits up to 570 s for a P9 capture to reach a marker (default ^EXIT) or for a new coordinator-log line mentioning
# the chain, window 3 or P9, then prints the capture tail, the lane owner and the coordinator-log tail.
# Usage: p9-wait.sh <capture-name> [marker-regex]
capture="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-p9-logs/$1.txt"; marker="${2:-^EXIT}"
fleet="/Users/ueli/Documents/semio/.tmp-ticket/📓️fleet-14-agents.md"
seen=$(/usr/bin/grep -c "CHAIN LAUNCHED\|WINDOW 3\|P9" "$fleet")
for i in $(seq 1 57); do
  /usr/bin/grep -q -E "$marker" "$capture" 2>/dev/null && break
  [ "$(/usr/bin/grep -c "CHAIN LAUNCHED\|WINDOW 3\|P9" "$fleet")" != "$seen" ] && { echo "COORDINATOR-LOG-CHANGED"; break; }
  /bin/sleep 10
done
echo "--- $(date +%H:%M:%S) capture tail"; tail -5 "$capture" | cut -c1-300
echo "--- overlay lane: $(cat /tmp/semio-overlay-build.lock/owner 2>/dev/null) queue=$(ls /tmp/semio-overlay-build.queue/ | wc -l | tr -d ' ')"
echo "--- coordinator log tail"; tail -3 "$fleet" | cut -c1-400
