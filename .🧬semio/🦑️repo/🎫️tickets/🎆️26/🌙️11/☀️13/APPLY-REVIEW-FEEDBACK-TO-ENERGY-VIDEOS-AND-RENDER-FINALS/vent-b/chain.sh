#!/bin/bash
O="$1"
for ch in Ventilation_05_MechanischeLueftung Ventilation_06_Uebergabe Ventilation_04_NatuerlicheLueftung; do
  repo tools exec --light bun -- '🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts' render --language de --video ventilation --chapter $ch --profile preview --audience speaker --jobs 2 --no-play --output-root "$O" > "$O/render-$ch.log" 2>&1
  echo "FINISHED $ch" >> "$O/chain.log"
done
echo ALLDONE >> "$O/chain.log"
