#!/bin/sh
# Host-target native cargo fallback (repo broker queue stalled >10 min); one build at a time, shared target dir.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"; OUT="$T/🗑️generated"
R=/Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/Users/ueli/t/src-extensions CARGO_BUILD_BUILD_DIR=/Users/ueli/t/src-extensions-b CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER= CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 SEMIO_TEST_ARTIFACT_DIR="$OUT/artifacts"
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
S="$R/✏️s/🔌️plugins/🪵️sourcing"
for e in ${EXT:-beams slabs windows oracle hub}; do
  case $e in
  beams) d="$S/🧩️extensions/🪵️beams";; slabs) d="$S/🧩️extensions/🧱️slabs";; windows) d="$S/🧩️extensions/🪟️windows";;
  oracle) d="$S/🧩️extensions/🧪️tests/🔬️unit/🔮️oracle/📦️packages/🦀️rust";; hub) d="$R/🌎️hub/🧩️compositions/🪵️sourcing/📦️packages/🦀️rust";;
  esac
  (cd "$d" && cargo test --all-targets -j 4 -- --test-threads=1) > "$OUT/ext-$e-native.txt" 2>&1
  echo "$e exit $?" >> "$OUT/ext-native-driver.log"
done
echo finished >> "$OUT/ext-native-driver.log"
