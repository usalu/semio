#!/bin/zsh
# ⛽️ Measures how much fuel 🧩️puzzle's owned `describe()` ACTUALLY needs (slice CE3). Runs the
# emitter built into CE3's PRIVATE target dir — that build carries a 32 G probe cap while the tree
# keeps the shipped 8 G — against the ALREADY-BUILT wasm component, so no wasm32 cargo build runs
# and no fleet wasm mutex is taken. The descriptor it writes goes to a temp dir and is thrown away;
# only the terminal `fuel=` observation is the result.
R="/Users/ueli/Documents/semio"
G="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated"
BIN="$R/.🧬semio/🦑️repo/⚡️cache/cargo/target-ce3/debug/semio-framework-plugin-describe"
WASM="$R/.🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev/semio_s_plugin_puzzle.wasm"
CORE="$R/.🧬semio/🦑️repo/⚡️cache/cargo/target/.semio-describe-core-tP2VF1/semio_s_plugin_puzzle.core.wasm"
OUT=$(mktemp -d /tmp/ce3-puzzle-XXXXXX)
export RUST_MIN_STACK=67108864
START=$(date +%s)
"$BIN" describe "$WASM" --core "$CORE" --out "$OUT" > "$G/ce3-puzzle-fuel-probe.txt" 2>&1
RC=$?
echo "=== rc=$RC after $(( $(date +%s) - START ))s at $(date -Iseconds) ===" >> "$G/ce3-puzzle-fuel-probe.txt"
ls -la "$OUT" >> "$G/ce3-puzzle-fuel-probe.txt" 2>&1
rm -rf "$OUT"
