#!/bin/bash
# usage: sheet.sh <asset> <pose|clip:id@t> <scales> <views> <outname> [pixel width]
S="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
G="$T/🗑️generated/vegetation"
ASSET=$1; WHAT=$2; SC=$3; VW=$4; NAME=$5; PW=${6:-1600}
D=$(dirname "$0"); FILE=$(cd "$D"; pwd)/assets/$1.json
if [[ $WHAT == clip:* ]]; then C=${WHAT#clip:}; ARGS="--clip ${C%@*} --t ${C#*@}"; else ARGS="--pose $WHAT"; fi
repo tools exec bun -- "$T/🧰️scripts/vegetation/preview.ts" --file "$FILE"  $ARGS --scales $SC --views $VW --out "$G/$NAME.svg" && rsvg-convert -w $PW "$G/$NAME.svg" -o "$G/$NAME.png" && echo "$G/$NAME.png"
