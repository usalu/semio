#!/bin/bash
# usage: sheet.sh asset height out.png poses views scales   (comma lists) -> one row of first frames
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/g2/$1/cast"
files=()
IFS=',' read -ra P <<< "$4"; IFS=',' read -ra V <<< "$5"; IFS=',' read -ra C <<< "$6"
for p in "${P[@]}"; do for v in "${V[@]}"; do for c in "${C[@]}"; do
  f="$G/$1-$p-$c-$v/0000.svg"; [ -f "$f" ] && files+=("$f")
done; done; done
python "$T/🧰️g2/raster.py" "$2" "$T/🗑️generated/g2/$1/$3" "${files[@]}"
