#!/bin/zsh
cd "$(dirname "$0")"
for n in 1 2 3 4 5 6; do sleep 300; ./watch.sh > watch-$n.txt 2>&1; done
