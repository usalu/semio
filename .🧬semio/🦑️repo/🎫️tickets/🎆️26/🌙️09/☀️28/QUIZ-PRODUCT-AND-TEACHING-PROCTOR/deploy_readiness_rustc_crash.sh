#!/bin/sh
# 🧨️ Deploy-readiness probe: how often does the release build of the `proctor` binary crash rustc inside the builder
# stage, with the workspace rustflags (`-Z threads=8`) and without them? Runs inside the builder image:
#   docker run --rm --volume <this file>:/probe.sh:ro <builder image> sh /probe.sh <rounds>
# Every round links the binary again from warm dependencies (only the final crate is recompiled), which is the step
# that crashed (LLVM IPSCCP during thin LTO).
rounds="${1:-4}"
cd /src || exit 2
export CARGO_TARGET_DIR=/tmp/out
cargo build --release --locked --package teaching-proctor --bin proctor > /tmp/warm.log 2>&1
echo "[DEBUG] warm build exit=$?"
for flags in workspace none; do
  crashes=0
  round=1
  while [ "$round" -le "$rounds" ]; do
    touch "🎓️teaching/🛂️proctor/🏗️bootstrap/🦀️.rs" "🎓️teaching/🛂️proctor/📦️packages/🦀️rust/🦀️.rs"
    echo "// probe $flags $round" >> "🎓️teaching/🛂️proctor/🏗️bootstrap/🦀️.rs"
    echo "// probe $flags $round" >> "🎓️teaching/🛂️proctor/📦️packages/🦀️rust/🦀️.rs"
    started=$(date +%s)
    if [ "$flags" = none ]; then
      RUSTFLAGS="" cargo build --release --locked --package teaching-proctor --bin proctor > "/tmp/$flags-$round.log" 2>&1
    else
      cargo build --release --locked --package teaching-proctor --bin proctor > "/tmp/$flags-$round.log" 2>&1
    fi
    code=$?
    segv=$(grep -c "SIGSEGV" "/tmp/$flags-$round.log")
    [ "$code" -ne 0 ] && crashes=$((crashes + 1))
    echo "[DEBUG] flags=$flags round=$round exit=$code sigsegv=$segv seconds=$(( $(date +%s) - started ))"
    round=$((round + 1))
  done
  echo "[DEBUG] flags=$flags crashes=$crashes/$rounds"
done
