#!/bin/zsh
# 🧱️ FP13: one foreground check → link → copy of the `semio-framework-plugin` lib unittests binary.
set -u
ROOT="/Users/ueli/Documents/semio"
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
OUT="$TICKET/🗑️generated"
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-fp13"
export CARGO_INCREMENTAL=0
export CARGO_BUILD_JOBS=4
export RUST_MIN_STACK=67108864
mkdir -p "$OUT" "$CARGO_TARGET_DIR/bin"
cd "$ROOT" || exit 1
echo "== check $(date +%H:%M:%S) load=$(uptime | sed 's/.*averages: //')"
cargo check -p semio-framework-plugin --lib --profile test 2>&1 | tail -40 | tee "$OUT/fp13-check-$1.txt"
grep -c "^error" "$OUT/fp13-check-$1.txt"
echo "== link $(date +%H:%M:%S)"
cargo test -p semio-framework-plugin --lib --no-run --message-format=json 2>"$OUT/fp13-link-err-$1.txt" \
  | python3 -c 'import sys,json
for line in sys.stdin:
    try: m=json.loads(line)
    except Exception: continue
    if m.get("reason")=="compiler-artifact" and m.get("executable") and m.get("target",{}).get("name")=="semio_framework_plugin":
        print(m["executable"])' | tail -1 > "$OUT/fp13-bin-path-$1.txt"
tail -6 "$OUT/fp13-link-err-$1.txt"
BIN=$(cat "$OUT/fp13-bin-path-$1.txt")
echo "binary: $BIN"
if [ -n "$BIN" ] && [ -x "$BIN" ]; then
  cp "$BIN" "$CARGO_TARGET_DIR/bin/fp13-lib" && echo "copied to $CARGO_TARGET_DIR/bin/fp13-lib"
fi
ls -l "$CARGO_TARGET_DIR/bin/fp13-lib" 2>/dev/null
echo "== done $(date +%H:%M:%S)"
