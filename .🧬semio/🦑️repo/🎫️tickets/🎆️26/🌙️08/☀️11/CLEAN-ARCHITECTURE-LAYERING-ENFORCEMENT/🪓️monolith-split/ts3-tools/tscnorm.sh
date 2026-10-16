#!/bin/zsh
# usage: tscnorm.sh <log> -> sorted "file: TScode message" rows without line/column numbers
/usr/bin/grep 'error TS' "$1" | sed -E 's/\([0-9]+,[0-9]+\)//' | sort
