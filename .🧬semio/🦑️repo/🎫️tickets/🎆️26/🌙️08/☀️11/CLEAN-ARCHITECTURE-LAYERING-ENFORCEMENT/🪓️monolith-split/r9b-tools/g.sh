#!/bin/zsh
# usage: g.sh <needle> [pathspec]  fixed-string git grep in the architect artifact, prefix-shortened output
cd /Users/ueli/Documents/semio
P="✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program"
S="$P/🏅️standards/🔖️1/🪆️subsets/✳️any"
git -c core.quotepath=false grep -n -F -- "$1" -- "${2:-$P}" 2>/dev/null | sed "s|$S/|A/|; s|$P/|P/|" | cut -c1-${W:-260}
