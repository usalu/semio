#!/bin/zsh
# usage: frevert-all.sh <id> <ext> : reverts a phase split in the overlay: children, root, and every consumer copy listed in specs/p-<id>.cons
W=/private/tmp/fnsplit4a-finish/work; TD=${0:A:h}
$TD/frevert.sh $W/specs/p-$1.json $W/orig/p-$1.$2
[[ -f $W/specs/p-$1.cons ]] && while IFS= read -r c; do [[ -n "$c" ]] && cp "/Users/ueli/Documents/semio/$c" "/private/tmp/fnsplit4a-finish/ov/$c"; done < $W/specs/p-$1.cons
