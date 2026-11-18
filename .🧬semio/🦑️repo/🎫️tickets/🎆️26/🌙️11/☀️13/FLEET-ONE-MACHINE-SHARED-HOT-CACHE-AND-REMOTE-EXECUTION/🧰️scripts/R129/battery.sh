#!/bin/zsh
# usage: battery.sh <out file>   (R129_BASE=1 runs the unchanged tree; otherwise the staged overlay; R129_ONLY=newline list of repo-relative files)
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R129
D=${0:h}
R=/Users/ueli/Documents/semio
PH="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
FP="$PH/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests"
FILES=(${(f)"$(ls $FP/🟦️.ts $FP/*/🟦️.ts $PH/🧠️memory/🧪️tests/🎫️reservation/🟦️.ts $PH/🧠️memory/🪶️lean/🧪️tests/🟦️.ts $PH/🧠️memory/🦶️footprint/🧪️tests/🟦️.ts $PH/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🪂️set-aside/🟦️.ts $PH/🕸️graph/🧪️tests/🏊️cooperative-farm/🟦️.ts 2>/dev/null)"})
[ -n "$R129_ONLY" ] && FILES=(${(f)"$(sed "s|^|$R/|" <<< "$R129_ONLY")"})
: > "$1"
for f in "${FILES[@]}"; do
 out=$($D/run-staged.sh "$f" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$" | tr '\n' ' ')
 echo "${f#$R/} :: $out" >> "$1"
done
