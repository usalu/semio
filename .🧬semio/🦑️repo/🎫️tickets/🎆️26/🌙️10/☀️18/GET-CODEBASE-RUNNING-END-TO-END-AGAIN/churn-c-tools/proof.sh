#!/bin/sh
# usage: proof.sh <name> [extra repo run args...]
# Runs ONE real describe-rs build and, from the moment the compiler phase starts, churns its inputs with toucher.py until it ends.
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/churn-c"
name=$1; shift
D="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe"
R=/Users/ueli/Documents/semio
mkdir -p "$G/art"
rm -f "$G/$name.stop" "$G/$name.run.log" "$G/$name.toucher.json"
export SEMIO_TEST_ARTIFACT_DIR="$PWD/$G/art" SEMIO_SCRIPT_STALL_MS=14400000
nice -n 10 bun repo --tool=bazel --workers=off run @semio-tech/os-plugin-describe-rs:build:prototype --skip-cache "$@" > "$G/$name.run.log" 2>&1 &
run=$!
echo "run pid $run" > "$G/$name.pids"
started=0
while kill -0 "$run" 2>/dev/null; do
  if [ "$started" = 0 ] && grep -q -E "describe-rs:build:prototype; [0-9]+s|Compiling|\[nx-native\]|cargo-target-recipe|\[cargo-provenance\]" "$G/$name.run.log" 2>/dev/null; then
    python3 "$C/churn-c-tools/toucher.py" "$G/$name.toucher.json" "$G/$name.stop" 7200 80 \
      --files "$D/📦️packages/🦀️rust/Cargo.toml" "$D/📦️packages/🦀️rust/💾️binary/🦀️.rs" "$D/🛂️descriptor-emission/🦀️.rs" "$R/Cargo.lock" \
      --dirs "$D/📦️packages/🦀️rust" "$D/📦️packages/🦀️rust/💾️binary" "$D/🛂️descriptor-emission" "$R/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust" "$R/🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust" &
    echo "toucher pid $!" >> "$G/$name.pids"
    started=1
    date +%T > "$G/$name.toucher.started"
  fi
  sleep 1
done
wait "$run"; rc=$?
touch "$G/$name.stop"
wait
echo "rc=$rc" > "$G/$name.rc"
