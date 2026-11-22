#!/bin/zsh
# 🖼️ Renders a draft or catalog asset sheet (three views) and converts it to PNG: render.sh <slug> <file|-> <scale> <pose|clip@t> [view] [outdir]
source "${0:A:h}/env.sh"
slug=$1; file=$2; scale=$3; q=$4; view=${5:-light-solid}; out=${6:-$O/png}
mkdir -p "$out"
src=(--asset $slug); [[ $file != - ]] && src+=(--file $file)
if [[ $q == *@* ]]; then qa=(--clip ${q%@*} --time ${q#*@}); else qa=(--pose $q); fi
cd "$S" && $B 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts $src --scale $scale $qa --view $view --views elevation,front,plan --projection third --no-png --out "$out" 2>&1 | grep -v "^ *at " | tail -3
cd "$out" && for f in *.svg(N); do [[ -f $f.png && $f.png -nt $f ]] || qlmanage -t -s 1600 -o . $f >/dev/null 2>&1; done
