#!/bin/bash
# usage: prev.sh <asset> <scale> <pose|clip:ID@t> ...   -> renders the three-view sheet of each pose/clip instant
cd /c/git/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
OUT="$T/🗑️generated/M1/prev"
S="🎓️teaching/🏛️architecture/🧍️staffage"
asset="$1"; scale="$2"; shift 2
for q in "$@"; do
  case "$q" in
    clip:*) c="${q#clip:}"; id="${c%@*}"; t="${c#*@}"; args="--clip $id --time $t" ;;
    *) args="--pose $q" ;;
  esac
  repo tools exec bun -- "$S/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts" --asset "$asset" --scale "$scale" $args --views --view dark-line --dpi 300 --out "$OUT" 2>&1 | grep -v '^\[' | tail -3
done
echo "prev done"
