#!/bin/zsh
# usage: battery.sh <out file>   (R135_BASE=1 runs the unchanged tree; otherwise the staged overlay; R135_LIST=file with repo-relative paths)
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R135
D=${0:h}
R=/Users/ueli/Documents/semio
PH="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
FILES=(${(f)"$(cat ${R135_LIST:-$S/battery.list})"})
: > "$1"
for f in "${FILES[@]}"; do
 out=$($D/run-staged.sh "$R/$f" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)$" | tr '\n' ' ')
 echo "$f :: $out" >> "$1"
done
