#!/bin/sh
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
log="$here/🗑️generated/$1.log"
cd "$here" || exit 1
NATIVE_CARGO_DIR="$root" WATCHDOG_TEST_SECONDS=1800 python3 watchdog-cargo.py sh ./native-cargo.sh test --manifest-path "$root/🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/📦️packages/🦀️rust/Cargo.toml" -p semio-hub-stdio-pdf --test committed_descriptor -- --nocapture > "$log" 2>&1
echo "exit $?" >> "$log"
