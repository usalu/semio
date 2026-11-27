#!/bin/sh
# usage: run-multi-retry.sh <log name> <package list, comma separated> <test filter args..>  (lib tests of several packages through the gate with a 12 GB disk guard; retries up to 6 times, 3 minutes apart, while another session's edit breaks the build)
cd "$(dirname "$0")"
name=$1; pkgs=$2; shift 2
set --  "$@"
args=""; for p in $(echo "$pkgs" | tr ',' ' '); do args="$args -p $p"; done
for attempt in 1 2 3 4 5 6 7 8 9 10; do
  [ "$(df -k /System/Volumes/Data | awk 'NR==2{print $4}')" -lt 12000000 ] && { echo "exit 99" > 🗑️generated/$name.done; exit 99; }
  sh ./run-s4.sh $name test $args --lib --no-fail-fast -- "$@" --test-threads=${TEST_THREADS:-3}
  grep -aq "could not compile" 🗑️generated/$name.log || exit 0
  sleep 180
done
