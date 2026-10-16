#!/bin/zsh
# usage: fails.sh <log> -> sorted unique FAIL lines (hash/no ansi)
sed -E $'s/\x1b\\[[0-9;]*m//g' "$1" | grep -E '^ FAIL ' | sed -E 's/ +$//' | sort -u
