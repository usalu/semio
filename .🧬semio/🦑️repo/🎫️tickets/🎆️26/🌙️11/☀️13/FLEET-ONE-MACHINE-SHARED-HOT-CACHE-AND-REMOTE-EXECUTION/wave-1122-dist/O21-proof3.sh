#!/bin/zsh
set -u
W=/Users/ueli/Documents/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122-dist"
S=$W/.cache/scratch/o21-run L=$1 PROBE=$W/.cache/scratch/o21-a/ws
export MANIFEST_DIR=$PROBE P1_TOOLCHAIN=o21-run1
echo "[$(date +%H:%M:%S)] begin B2" >> $L/steps.log; "$T/O21-run.sh" $S/c3 $S/t7 pin:g1-m11 check --lib --message-format=short > $L/B2.log 2>&1; echo "[$(date +%H:%M:%S)] end B2 exit $?" >> $L/steps.log
