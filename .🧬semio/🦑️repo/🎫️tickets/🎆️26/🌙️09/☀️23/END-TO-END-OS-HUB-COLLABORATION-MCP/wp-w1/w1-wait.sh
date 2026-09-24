#!/bin/zsh
# ⏳️ W1: block until a chain log grows by an END/DONE line or a request file changes (max $1 seconds).
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-w1 || exit 1
sig() { cat generated/*chain*.txt 2>/dev/null | grep -cE 'END|DONE'; ls -l requests | md5; }
a=$(sig); t=0
while [ "$(sig)" = "$a" ] && [ $t -lt ${1:-580} ]; do sleep 20; t=$((t+20)); done
date '+%T'; tail -2 generated/stage-chain.txt; tail -2 generated/catalog-c7-chain.txt; ls -l requests | tail -n +2 | awk '{print $6,$7,$8,$9}'
