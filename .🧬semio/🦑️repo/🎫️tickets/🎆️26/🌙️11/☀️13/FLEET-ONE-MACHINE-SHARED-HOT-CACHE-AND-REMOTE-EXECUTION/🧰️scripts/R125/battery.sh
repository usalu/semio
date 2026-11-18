#!/bin/zsh
# usage: battery.sh <out file>   (R125_BASE=1 runs the unchanged tree; otherwise the staged overlay)
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R125
D=${0:h}
R=/Users/ueli/Documents/semio
HC="$R/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests"
PH="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
FP="$PH/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests"
FILES=(${(f)"$(ls $HC/*/🟦️.ts $HC/🟦️.ts $PH/🌡️host-capacity/🧪️tests/*/🟦️.ts $PH/🌡️host-capacity/🧪️tests/🟦️.ts $FP/🟦️.ts $FP/*/🟦️.ts 2>/dev/null)"})
[ -n "$R125_ONLY" ] && FILES=(${(f)"$(sed "s|^|$R/|" <<< "$R125_ONLY")"})
: > "$1"
for f in "${FILES[@]}"; do
 out=$($D/run-staged.sh "$f" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$" | tr '\n' ' ')
 echo "${f#$R/} :: $out" >> "$1"
done
