#!/bin/sh
# 🧪️ Runs one Draw native test crate locally and repeats while another edit leaves the shared sources uncompilable.
# Usage: draw-native-tests.sh <label> <crate> [cargo test arguments]
cd "$(dirname "$0")/../../../../../../.."
G="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/🗑️generated"
label="$1"; crate="$2"; shift 2
cd "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing" || exit 1
while pgrep -f "cargo.*semio-s-artifact-draw" >/dev/null; do sleep 10; done
for attempt in 1 2 3 4 5 6; do
  CARGO_INCREMENTAL=0 SEMIO_OFFLOAD=never SEMIO_BUILD_MODE=prototype SEMIO_LANGUAGE=en repo cargo --store=isolated --configuration=prototype -- test --no-fail-fast -p "$crate" "$@" > "$G/native-$label.txt" 2>&1
  code=$?
  echo "attempt $attempt exit $code $(date +%T)" >> "$G/native-$label.attempts"
  grep -aqE "^test result|^running [0-9]+ test" "$G/native-$label.txt" && break
  sleep 60
done
echo "EXIT $code" >> "$G/native-$label.txt"
