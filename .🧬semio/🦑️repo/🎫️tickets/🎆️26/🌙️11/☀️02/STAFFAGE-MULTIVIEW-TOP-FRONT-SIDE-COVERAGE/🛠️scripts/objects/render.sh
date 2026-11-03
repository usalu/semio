#!/bin/sh
# usage: render.sh slug scale pose|clip:id@t outdir
P=/c/git/semio/🎓️teaching/🏛️architecture/🧍️staffage
case "$3" in clip:*) c=${3#clip:}; id=${c%@*}; t=${c#*@}; q="--clip $id --time $t";; *) q="--pose $3";; esac
bun $P/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset $1 --scale $2 $q --views --projection third --out $4 2>&1 | grep -v -i "warn\|fitz"
