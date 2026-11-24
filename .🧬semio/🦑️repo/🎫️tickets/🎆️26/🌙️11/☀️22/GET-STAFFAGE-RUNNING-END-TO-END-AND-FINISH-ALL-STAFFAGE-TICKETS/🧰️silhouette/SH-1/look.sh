#!/bin/zsh
# 🖼️ Third-angle multiview sheets (plan, front, side) of a person as PNG from the product preview: look.sh <slug> [scales] [file] [tag]   (after `FIN-B/look.sh` of the human wave; output 🗑️generated/silhouette/SH-1/look/<slug>[/<tag>])
source "${0:A:h}/env.sh"
a=$1; o="$GEN/look/$a${4:+/$4}"; mkdir -p "$o"
src=(--asset $a); [[ -n $3 && $3 != - ]] && src+=(--file $3)
for sc in ${=2:-50 500}; do
  (cd "$S" && timeout 300 $B "🔨️modules/🎨️depiction/🖨️preview/🟦️.ts" $src --scale $sc --view light-solid --views --projection third --flow none --no-png --out "$o" >/dev/null 2>"$o/[DEBUG]-err.txt") || echo "[DEBUG] fail $a $sc: $(grep -v '^ *[0-9]* |' "$o/[DEBUG]-err.txt" | head -3)"
done
rm -f "$o/[DEBUG]-err.txt"
svgs=("$o"/*.svg(N)); (( $#svgs )) && $B "$W/svg-png.ts" ${DPI:-110} $svgs
ls "$o"/*.png
