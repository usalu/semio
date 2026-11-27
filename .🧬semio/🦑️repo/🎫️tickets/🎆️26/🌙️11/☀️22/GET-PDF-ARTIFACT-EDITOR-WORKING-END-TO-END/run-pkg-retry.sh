#!/bin/sh
# usage: run-pkg-retry.sh <log name> <package> <test filter args..>  (one package's lib tests through the gate; retries up to 6 times, 3 minutes apart, while another session's edit breaks the build)
cd "$(dirname "$0")"
name=$1; pkg=$2; shift 2
for attempt in 1 2 3 4 5 6; do
  [ "$(df -k /System/Volumes/Data | awk 'NR==2{print $4}')" -lt 12000000 ] && { echo "exit 99" > 🗑️generated/$name.done; exit 99; }
  sh ./run-s4.sh $name test -p $pkg --lib -- "$@" --test-threads=3
  grep -aq "could not compile" 🗑️generated/$name.log || exit 0
  sleep 180
done
