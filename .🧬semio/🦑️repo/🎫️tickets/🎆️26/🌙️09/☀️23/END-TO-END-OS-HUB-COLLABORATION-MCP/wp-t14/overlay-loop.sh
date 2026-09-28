#!/bin/zsh
# 🔁️ T14 overlay hold loop: queues `overlay-hold.sh` in the overlay lane again (back of the FIFO) after every ≤ 28-min hold until
# every step is recorded in the state file, or a hold stops red / not ready. Waits for ≤ 14 rustc before each queueing (rule 6).
# usage: overlay-loop.sh <tag-prefix> <first-number>
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-t14-logs"
T=/Users/ueli/Documents/semio/.tmp-ticket/wp-t14
export T14_STATE=$L/s14c-state.txt
STEPS=(KERNEL SDK OWNERS-1 OWNERS-1C OWNERS-1P OWNERS-1X MAP OWNERS-2 OWNERS-2R OWNERS-3P MAP2 OWNERS-V OWNERS-VR KERNEL-LAWS HOST-LAW WFC3D-LAW HUB-LAW VALUE-LAW ORPHAN-VERDICT ORPHAN-LAWS SDK-REDS TS-ORACLE N1 N2 N3 W1 W2)
n=$2
while true; do
  missing=0; for s in $STEPS; do /usr/bin/grep -q "^$s " $T14_STATE || missing=$((missing + 1)); done
  (( missing == 0 )) && { echo "ALL-STEPS-DONE $(date +%T)"; break; }
  until [ $(ps -axo command | /usr/bin/grep -c "^[^ ]*rustc ") -le 14 ]; do sleep 30; done
  echo "QUEUE $1$n missing=$missing $(date +%T)"
  zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay t14 -- nice -n 15 zsh $T/overlay-hold.sh $1$n > $L/s14c-run-$1$n.txt 2>&1
  tail -3 $L/s14c-run-$1$n.txt
  /usr/bin/grep -q -E 'STOP-RED|NOT-READY|unknown step' $L/s14c-run-$1$n.txt && { echo "STOPPED $1$n $(date +%T)"; break; }
  n=$((n + 1))
done
