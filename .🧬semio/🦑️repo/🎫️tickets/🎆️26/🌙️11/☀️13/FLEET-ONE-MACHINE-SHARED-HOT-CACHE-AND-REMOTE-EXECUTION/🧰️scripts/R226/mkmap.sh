#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R226
cd $S/new && find . -type f | sed 's|^\./||' | while read -r rel; do printf '%s\t%s\n' "/Users/ueli/Documents/semio/$rel" "$S/new/$rel"; done > $S/map.tsv
wc -l $S/map.tsv
