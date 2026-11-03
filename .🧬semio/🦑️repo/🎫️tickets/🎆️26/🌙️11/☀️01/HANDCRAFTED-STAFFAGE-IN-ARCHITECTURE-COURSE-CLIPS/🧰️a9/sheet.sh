#!/bin/bash
# usage: sheet.sh asset out.png height pose:scale:view [pose:scale:view ...]  (frame 0000 of each cast)
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
A=$1; G="$T/🗑️generated/a9/$A"; out="$G/$2"; h=$3; shift 3
files=()
for spec in "$@"; do IFS=: read p s v <<< "$spec"; files+=("$G/cast/$A-$p-$s-$v/0000.svg"); done
python -W ignore "$T/🧰️a9/raster.py" "$h" "$out" "${files[@]}" 2>&1 | tail -1
