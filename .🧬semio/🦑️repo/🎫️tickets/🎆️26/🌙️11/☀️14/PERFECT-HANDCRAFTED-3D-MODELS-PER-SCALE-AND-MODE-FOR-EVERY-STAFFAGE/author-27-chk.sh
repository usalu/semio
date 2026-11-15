#!/bin/zsh
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
slug=$1; src=$2; shift 2
sc=("$@"); if [ ${#sc[@]} -eq 0 ]; then sc=(50 100 200 500); fi
for s in $sc; do echo "== $s"; $BUN volumes-check.ts humans/$slug --scale $s --source $src 2>&1 | grep -E '^(FAIL|note)' | sed -E 's/info:[^ ]+ //g' | cut -c1-1500; done
