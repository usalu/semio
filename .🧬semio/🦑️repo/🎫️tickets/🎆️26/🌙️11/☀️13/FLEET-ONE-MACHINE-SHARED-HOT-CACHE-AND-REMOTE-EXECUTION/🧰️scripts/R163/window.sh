#!/bin/zsh
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R163
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R163
until m=$((10#$(date -u +%M))); [ $m -lt 5 ] || { [ $m -ge 30 ] && [ $m -lt 35 ]; }; do sleep 1; done
waited=0
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 240 ] || { echo "no lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R163 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
rm -rf $SCR/bk; python3 $D/land.py apply $SCR/bk || { python3 $D/land.py restore $SCR/bk; release; exit 2; }
if $D/tests.sh; then echo GREEN; else echo RED; python3 $D/land.py restore $SCR/bk; fi
release; echo "released $(date -u +%H:%M:%S) (started $START)"
