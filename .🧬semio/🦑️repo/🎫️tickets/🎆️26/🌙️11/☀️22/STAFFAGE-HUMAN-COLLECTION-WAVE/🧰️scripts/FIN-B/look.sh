#!/bin/zsh
# 🖼️ Third-angle multiview sheets (plan, front, side) at 1:50, 1:100, 1:200, 1:500 as PNG: look.sh <slug> [pose|clip@t|-] [view] [file] [scales]   (PROFILE=id for a diagnostic print profile, OUT=subfolder)
source "${0:A:h}/env.sh"
a=$1; o="$GEN/look/$a${OUT:+/$OUT}"; mkdir -p "$o"
prof=(); [[ -n $PROFILE ]] && prof=(--profile $PROFILE)
q=(); [[ -n $2 && $2 != - ]] && { [[ $2 == *@* ]] && q=(--clip ${2%@*} --time ${2#*@}) || q=(--pose $2) }
src=(--asset $a); [[ -n $4 && $4 != - ]] && src+=(--file $4)
for sc in ${=5:-50 100 200 500}; do
  (cd "$S" && $B "🔨️modules/🎨️depiction/🖨️preview/🟦️.ts" $src --scale $sc $q --view ${3:-light-solid} --views --projection third --flow none $prof --no-png --out "$o" >/dev/null 2>"$o/[DEBUG]-err.txt") || echo "[DEBUG] fail $a $sc: $(grep -v '^ *[0-9]* |' "$o/[DEBUG]-err.txt" | head -3)"
done
rm -f "$o/[DEBUG]-err.txt"
svgs=("$o"/*.svg(N)); (( $#svgs )) && $B "$F/png.ts" ${DPI:-110} $svgs
ls "$o"
