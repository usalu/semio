#!/bin/sh
# usage: run-s14-individual.sh <package..>  (slice 14: one gated cargo test per package, so one stale test crate cannot block the others; logs and exit codes per package in 🗑️generated)
cd "$(dirname "$0")"
root=$(CDPATH= cd -- "$(pwd)/../../../../../../.." && pwd)
export NATIVE_CARGO_DIR="$root" WATCHDOG_TEST_SECONDS=${WATCHDOG_TEST_SECONDS:-1500}
G=🗑️generated
for p in "$@"; do
  rm -f $G/s14-ind-$p.done
  python3 watchdog-cargo.py sh ./native-cargo.sh test -p $p --tests -- --test-threads=2 > $G/s14-ind-$p.log 2>&1
  echo "exit $?" > $G/s14-ind-$p.done
done
