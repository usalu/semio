#!/bin/zsh
# 🔬️ Profiles ONE plugin's owned `describe()` natively against an ALREADY-BUILT wasm component
# (slice CE2). No wasm32 cargo build runs, so this needs no fleet wasm mutex.
# Usage: 📜️ce2-profile-describe.sh <wasm-path> <label> [sample-seconds]
WASM="$1"; LABEL="$2"; SECS="${3:-90}"
TICKET="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
BIN="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target/debug/semio-framework-plugin-describe"
CORE="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target/.semio-describe-core-rwuzhe/semio_s_plugin_stdio.core.wasm"
OUT=$(mktemp -d /tmp/ce2-describe-XXXXXX)
RUN="$GEN/ce2-profile-${LABEL}-run.txt"
"$BIN" describe "$WASM" --core "$CORE" --out "$OUT" > "$RUN" 2>&1 &
PID=$!
echo "pid=$PID out=$OUT"
# ⏳️ Wait for the compile phase to end and the guest to start executing.
for i in $(seq 1 240); do
  grep -q "phase=execute fuel=[1-9]" "$RUN" 2>/dev/null && break
  kill -0 $PID 2>/dev/null || break
  /bin/sleep 1
done
grep -m1 "phase=compile" "$RUN"
sample $PID "$SECS" -f "$GEN/ce2-profile-${LABEL}-sample.txt" 2>&1 | tail -3
tail -2 "$RUN"
kill $PID 2>/dev/null
rm -rf "$OUT"
echo "DONE"
