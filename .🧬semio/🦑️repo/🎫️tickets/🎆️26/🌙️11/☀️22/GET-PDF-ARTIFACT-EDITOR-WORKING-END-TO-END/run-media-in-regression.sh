#!/bin/sh
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
log="$here/🗑️generated/$1.log"; shift
cd "$here" || exit 1
RUST_MIN_STACK=67108864 WATCHDOG_TEST_SECONDS=${WATCHDOG_TEST_SECONDS:-1500} python3 watchdog-cargo.py sh ./native-cargo.sh test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- "$@" --test-threads=1 --nocapture > "$log" 2>&1
echo "exit $?" >> "$log"
touch "$here/🗑️generated/$(basename "$log" .log).done"
