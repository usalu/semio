#!/bin/zsh
# 🐕 Renders every pose and two instants of every clip of the labrador at one scale and list, tiles them (usage: scale list view)
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
S=${1:-50}; L=${2:-body}; V=${3:-right}
OUT="🗑️generated/rework-labrador-dog/tiles"; mkdir -p "$OUT"; : > "$OUT/list-$S-$L-$V.txt"
for P in standing-alert sitting-waiting sniffing-head-down walk-phase-0 walk-phase-1 walk-phase-2 walk-phase-3 walk-phase-4 walk-phase-5 walk-phase-6 walk-phase-7 trot-phase-0 trot-phase-1 trot-phase-2 trot-phase-3 trot-phase-4 trot-phase-5 trot-phase-6 trot-phase-7; do
  $BUN r3-sheet.ts animals/labrador-dog --scale $S --list $L --pose $P --only $V --no-labels >> "$OUT/list-$S-$L-$V.txt"
done
for C in "walk-cycle 0.2" "walk-cycle 0.55" "trot-cycle 0.1" "trot-cycle 0.33" "sniff-ground 1.3" "sniff-ground 2.7" "sit-and-pant 1.5" "sit-and-pant 4.5"; do
  $BUN r3-sheet.ts animals/labrador-dog --scale $S --list $L --clip ${C% *} --t ${C#* } --only $V --no-labels >> "$OUT/list-$S-$L-$V.txt"
done
python3 rework-p2-labrador-dog-tile.py "$OUT/list-$S-$L-$V.txt" "$OUT/tiles-$S-$L-$V"
