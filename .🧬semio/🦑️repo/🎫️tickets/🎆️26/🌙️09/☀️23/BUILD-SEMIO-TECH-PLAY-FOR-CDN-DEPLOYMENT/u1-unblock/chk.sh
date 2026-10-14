#!/bin/bash
# U1 checks: chk.sh <which: sdk|kernel|hub|ktest> ; logs to 🗑️generated/u1/<which>.log
R=/Users/ueli/Documents/semio
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
C="$R/.🧬semio/🦑️repo/⚡️cache/u1"; L="$T/🗑️generated/u1"; mkdir -p "$L"
W="$1"; shift
case "$W" in
  sdk)    K=native; A=(check --manifest-path "$R/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml" --lib);;
  kernel) K=native; A=(check --manifest-path "$R/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml" --lib);;
  hub)    K=wasm;   A=(check --manifest-path "$R/🌎️hub/Cargo.toml" --workspace --exclude semio-hub --exclude semio-hub-auth-client --target wasm32-wasip2 --profile wasm-release);;
  ktest)  K=native; A=(test --manifest-path "$R/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml" --lib);;
esac
cd "$R"
echo "START $W $(date +%H:%M:%S)" >> "$L/timestamps.txt"
CARGO_TARGET_DIR="$C/$K/target" CARGO_BUILD_BUILD_DIR="$C/$K/build" bash "$T/🚦️cargo-slot.sh" u1-$W -- cargo "${A[@]}" --keep-going --message-format=short "$@" > "$L/$W.log" 2>&1
rc=$?; echo "exit=$rc END $W $(date +%H:%M:%S)" >> "$L/timestamps.txt"
echo "$W rc=$rc errors=$(grep -c ': error' "$L/$W.log") warnings=$(grep -c ': warning' "$L/$W.log")"
