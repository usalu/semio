#!/bin/sh
# usage: run-s14-manifest.sh <log name> <manifest path>  (slice 14: gated lib and integration tests of one crate by manifest path, retried while a foreign crate does not compile; log in 🗑️generated)
cd "$(dirname "$0")"
G=🗑️generated
name=$1; manifest=$2
rm -f $G/$name.done
for attempt in 1 2 3 4 5 6; do
  python3 watchdog-cargo.py sh ./native-cargo.sh test --manifest-path "$manifest" --no-fail-fast -- --test-threads=2 > $G/$name.log 2>&1
  status=$?
  grep -a -q "could not compile" $G/$name.log || break
  sleep 240
done
echo "exit $status" > $G/$name.done
