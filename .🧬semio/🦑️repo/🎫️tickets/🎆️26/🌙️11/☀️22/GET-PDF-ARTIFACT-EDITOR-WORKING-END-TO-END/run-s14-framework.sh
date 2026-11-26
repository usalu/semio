#!/bin/sh
# usage: run-s14-framework.sh <log name> <package..>  (slice 14: gated, watched cargo test of framework crates that cover faulting jobs and retained-command full operation; retried while a foreign crate does not compile; log in 🗑️generated)
cd "$(dirname "$0")"
root=$(CDPATH= cd -- "$(pwd)/../../../../../../.." && pwd)
export NATIVE_CARGO_DIR="$root" WATCHDOG_TEST_SECONDS=${WATCHDOG_TEST_SECONDS:-1500}
G=🗑️generated
name=$1; shift
args=""
for p in "$@"; do args="$args -p $p"; done
rm -f $G/$name.done
for attempt in 1 2 3 4 5 6; do
  python3 watchdog-cargo.py sh ./native-cargo.sh test $args --no-fail-fast -- --test-threads=2 > $G/$name.log 2>&1
  status=$?
  grep -a -q "could not compile" $G/$name.log || break
  echo "retry $attempt: $(grep -a 'could not compile' $G/$name.log | head -1)" >> $G/$name.retries
  sleep 240
done
echo "exit $status" > $G/$name.done
