#!/bin/zsh
# usage: mkov.sh <overlay name> <tools json>; rebuilds overlay with the tool rows and the code edits
HERE=${0:A:h}; S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R29
rm -rf $S/$1; mkdir -p $S/$1; python3 $HERE/tools_overlay.py $2 $S/$1 && python3 $HERE/apply_edits.py $S/$1 edits_si
