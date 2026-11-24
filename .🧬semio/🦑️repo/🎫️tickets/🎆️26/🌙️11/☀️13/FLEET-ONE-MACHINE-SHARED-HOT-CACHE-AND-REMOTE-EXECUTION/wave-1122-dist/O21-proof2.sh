#!/bin/zsh
# usage: O21-proof2.sh <logDir>  (after O21-proof.sh: remote vs local compile of one never-uploaded source)
set -u
W=/Users/ueli/Documents/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122-dist"
S=$W/.cache/scratch/o21-run L=$1 PROBE=$W/.cache/scratch/o21-a/ws
export MANIFEST_DIR=$PROBE P1_TOOLCHAIN=o21-run1
run(){ name=$1; shift; echo "[$(date +%H:%M:%S)] begin $name" >> $L/steps.log; "$T/O21-run.sh" "$@" check --lib --message-format=short > $L/$name.log 2>&1; echo "[$(date +%H:%M:%S)] end $name exit $?" >> $L/steps.log; }
printf '\npub fn marker_three() -> u32 { 3 }\n' >> "$PROBE/probe/🦀️.rs"
run A2 $S/c1 $S/t5 pin:g1-m06
run L2 $S/c1b $S/t6 never
echo "[$(date +%H:%M:%S)] proof2 done" >> $L/steps.log
