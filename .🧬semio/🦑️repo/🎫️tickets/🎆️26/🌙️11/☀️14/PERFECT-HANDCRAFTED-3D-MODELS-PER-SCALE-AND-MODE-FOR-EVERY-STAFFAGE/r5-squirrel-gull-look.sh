#!/bin/sh
# 🔭 Round 5 red squirrel and herring gull: renders every sheet judged in the round-5 report from the candidate documents (body of every scale, structure and cut thermal of 1:50 and 1:500, the poses and one clip frame).
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
SQ="🗑️generated/r3-red-squirrel/🔣️.json"
GU="🗑️generated/r3-herring-gull-full/🔣️.json"
sheet() { "$BUN" r3-herring-gull-sheet.ts "$@" 2>&1 | tail -1; }
for s in 50 100 200 500; do
  sheet animals/red-squirrel --file "$SQ" --scale $s --no-rig
  sheet animals/herring-gull --file "$GU" --scale $s --no-rig
done
for s in 50 500; do
  for a in red-squirrel:"$SQ" herring-gull:"$GU"; do
    sheet animals/${a%%:*} --file "${a#*:}" --scale $s --list structure --no-rig
    sheet animals/${a%%:*} --file "${a#*:}" --scale $s --list thermal --cut --no-rig
  done
done
sheet animals/red-squirrel --file "$SQ" --scale 50 --pose running --no-rig
sheet animals/red-squirrel --file "$SQ" --scale 50 --pose climbing --no-rig
sheet animals/red-squirrel --file "$SQ" --scale 50 --clip bound --t 0.2 --no-rig
sheet animals/herring-gull --file "$GU" --scale 50 --pose gliding --no-rig
sheet animals/herring-gull --file "$GU" --scale 50 --clip landing --t 1.6 --no-rig
