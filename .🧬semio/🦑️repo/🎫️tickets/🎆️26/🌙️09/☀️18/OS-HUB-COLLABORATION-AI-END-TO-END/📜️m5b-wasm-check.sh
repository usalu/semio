#!/bin/zsh
# 🧱️ M5b — inside ONE wasm-mutex hold: the wasm32 compile proof of every plugin crate this slice declared on,
# then (only when it is green) the descriptor regeneration of the five authored plugins whose committed
# descriptors the `search::long` laws pin. Waits for the slice's own native cargo so it never runs two at once.
cd /Users/ueli/Documents/semio
while [ -f .tmp-ticket-0918/wp-m5b/native-cargo.pid ] && kill -0 "$(cat .tmp-ticket-0918/wp-m5b/native-cargo.pid)" 2>/dev/null; do sleep 10; done
echo "START $(date '+%H:%M:%S')"
CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket-0918/wp-m5b/target CARGO_INCREMENTAL=0 CARGO_PROFILE_WASM_DEV_DEBUG=false \
  cargo check --target wasm32-wasip2 \
  -p semio-s-plugin-draw -p semio-s-plugin-note -p semio-s-plugin-layout -p semio-s-plugin-forms -p semio-s-plugin-cad \
  -p semio-s-plugin-animate -p semio-s-plugin-architect -p semio-s-plugin-flow -p semio-s-plugin-procedural \
  -p semio-s-plugin-process -p semio-s-plugin-puzzle -p semio-s-plugin-remodel -p semio-s-plugin-shooting \
  -p semio-s-plugin-space -p semio-s-plugin-trinity -p semio-s-plugin-demonstrator -p semio-s-plugin-lowpoly
rc=$?
echo "EXIT=$rc $(date '+%H:%M:%S')"
[ $rc -eq 0 ] || exit $rc
for p in 📏️layout 🖍️draw 📋️forms 📐️cad 🗒️note; do
  echo "DESCRIBE-START $p $(date '+%H:%M:%S')"
  (cd "✏️s/🔌️plugins/$p/📦️packages/🦀️rust" && CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_INCREMENTAL=0 bun ./📜️script.ts describe) > ".tmp-ticket-0918/wp-m5b/generated/m5b-describe-${p#*️}.txt" 2>&1
  echo "DESCRIBE-END $p rc=$? $(date '+%H:%M:%S')"
done
