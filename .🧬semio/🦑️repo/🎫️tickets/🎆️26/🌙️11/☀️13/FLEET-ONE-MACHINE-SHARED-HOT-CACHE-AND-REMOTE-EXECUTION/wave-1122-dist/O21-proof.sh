#!/bin/zsh
# usage: O21-proof.sh <logDir>   (probe workspace at .cache/scratch/o21-a/ws; scratch under .cache/scratch/o21-run)
set -u
W=/Users/ueli/Documents/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122-dist"
S=$W/.cache/scratch/o21-run L=$1 PROBE=$W/.cache/scratch/o21-a/ws
export MANIFEST_DIR=$PROBE P1_TOOLCHAIN=o21-run1
mkdir -p $S $L
run(){ name=$1; shift; echo "[$(date +%H:%M:%S)] begin $name" >> $L/steps.log; "$T/O21-run.sh" "$@" check --lib --message-format=short > $L/$name.log 2>&1; echo "[$(date +%H:%M:%S)] end $name exit $?" >> $L/steps.log; }
cp "$PROBE/probe/🦀️.rs" $L/probe-v1.rs
O21_UPLOAD=true O21_ACCESS=read-write run A0 $S/c1 $S/t1 pin:g1-m06
cp -R $S/c1 $S/c1b
printf '\npub fn marker_two() -> u32 { 2 }\n' >> "$PROBE/probe/🦀️.rs"
O21_UPLOAD=true O21_ACCESS=read-write run A1 $S/c1 $S/t2 pin:g1-m06
run L $S/c1b $S/t3 never
run B $S/c2 $S/t4 pin:g1-m10
echo "[$(date +%H:%M:%S)] all done" >> $L/steps.log
