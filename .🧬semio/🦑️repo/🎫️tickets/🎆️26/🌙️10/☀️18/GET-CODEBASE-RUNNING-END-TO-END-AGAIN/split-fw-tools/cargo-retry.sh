#!/bin/zsh
# usage: cargo-retry.sh <log-name> <attempts> <pause-seconds> <crate...>
cd /Users/ueli/Documents/semio
C=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
log=$C/🗑️generated/split-fw/$1.log; n=$2; pause=$3; shift 3
args=(); for c in "$@"; do args+=(-p "$c"); done
: > $log
for i in $(seq 1 $n); do
  echo "== attempt $i $(date +%T)" >> $log
  perl -e 'alarm 590; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check "${args[@]}" --lib --tests > $C/🗑️generated/split-fw/$1.attempt.log 2>&1
  code=$?
  command grep -v '^\[script\]' $C/🗑️generated/split-fw/$1.attempt.log | tail -40 >> $log
  echo "== attempt $i exit $code" >> $log
  if [ $code -eq 0 ]; then echo "exit 0" >> $log; exit 0; fi
  if ! command grep -qE 'failed to select a version|cargo fetch|System image plist|Bounded unique parent|SIGALRM|cannot update the lock file|failed to load manifest|failed to parse manifest|could not find|no matching package' $C/🗑️generated/split-fw/$1.attempt.log; then echo "exit $code" >> $log; exit $code; fi
  sleep $pause
done
echo "exit gave-up" >> $log
