#!/bin/zsh
# usage: note.sh <short-name> <before> <class> <what>
printf '%s\t%s\t%s\t%s\n' "$1" "$2" "$3" "$4" >> "$(dirname $0)/table.tsv"
