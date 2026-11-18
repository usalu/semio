#!/bin/bash
# Runs the silhouette triage of the working tree over the whole catalog in parallel chunks (vegetation apart: its trees take minutes each).
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd "$(dirname "$0")"; mkdir -p 🗑️generated
CATS="$1"
NV=$(grep -v '^vegetation ' "$CATS" | awk '{print $2}')
VG=$(grep '^vegetation ' "$CATS" | awk '{print $2}')
i=0; n=0; for a in $NV; do echo "$a" >> 🗑️generated/chunk-$((i%5)).txt; i=$((i+1)); done
for k in 0 1 2 3 4; do $BUN silhouette-triage.ts current 🗑️generated/sil-current-$k.jsonl $(cat 🗑️generated/chunk-$k.txt) > 🗑️generated/sil-current-$k.log 2>&1 & done
j=0; for a in $VG; do echo "$a" >> 🗑️generated/vchunk-$((j%2)).txt; j=$((j+1)); done
for k in 0 1; do $BUN silhouette-triage.ts current 🗑️generated/sil-veg-$k.jsonl $(cat 🗑️generated/vchunk-$k.txt) > 🗑️generated/sil-veg-$k.log 2>&1 & done
wait
