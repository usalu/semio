#!/bin/sh
# usage: proof2.sh <name>: TWO concurrent real describe-rs builds on the shared build dir + a toucher that replaces small cargo-owned intermediates (rlib/rmeta/.d under m/b/p/prototype/build) by byte-identical copies.
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/churn-c"
name=$1
D="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe"
R=/Users/ueli/Documents/semio
B="$R/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/b/p/prototype/build"
rm -f "$G/$name".*
export SEMIO_TEST_ARTIFACT_DIR="$PWD/$G/art" SEMIO_SCRIPT_STALL_MS=14400000
mkdir -p "$G/art"
python3 "$C/churn-c-tools/toucher.py" "$G/$name.toucher.json" "$G/$name.stop" 7200 150 \
  --files "$D/📦️packages/🦀️rust/Cargo.toml" "$R/Cargo.lock" \
  --dirs "$D/📦️packages/🦀️rust" \
  --glob "$B/semio-framework-*/*/out/*.rlib" 60 4000000 &
tp=$!
nice -n 10 bun repo --tool=bazel --workers=off run @semio-tech/os-plugin-describe-rs:build:prototype --skip-cache > "$G/$name.a.log" 2>&1 &
a=$!
sleep 20
nice -n 10 bun repo --tool=bazel --workers=off run @semio-tech/os-plugin-describe-rs:build:prototype --skip-cache > "$G/$name.b.log" 2>&1 &
b=$!
echo "toucher $tp run-a $a run-b $b" > "$G/$name.pids"
wait $a; ra=$?
wait $b; rb=$?
touch "$G/$name.stop"
wait $tp
echo "a=$ra b=$rb" > "$G/$name.rc"
