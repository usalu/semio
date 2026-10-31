#!/bin/bash
# usage: pv.sh asset scale view [pose] [extra...]   -> renders zoomed and print-size into generated/NW-O/<asset>/
asset=$1; scale=$2; view=${3:-light-line}; pose=${4:-}
S=/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage
G="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-O/$asset"
mkdir -p "$G/z" "$G/p"
P=(); [ -n "$pose" ] && P=(--pose "$pose")
cd "$S"
repo tools exec bun -- 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset "$asset" --scale "$scale" --view "$view" "${P[@]}" ${PROFILE:+--profile $PROFILE} --out "$G/z" 2>&1 | grep -E "png$|rror" 
repo tools exec bun -- 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset "$asset" --scale "$scale" --view "$view" "${P[@]}" ${PROFILE:+--profile $PROFILE} --print-size --dpi 600 --out "$G/p" 2>&1 | grep -E "png$|rror"
