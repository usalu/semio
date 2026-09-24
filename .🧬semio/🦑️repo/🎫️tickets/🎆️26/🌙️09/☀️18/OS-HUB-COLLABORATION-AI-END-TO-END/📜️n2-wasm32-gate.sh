#!/bin/zsh
# 🔐️ N2 wasm32 gate — runs INSIDE one fleet-mutex hold: `zsh 📜️fleet-mutex.sh wasm n2 -- zsh 📜️n2-wasm32-gate.sh`
# 1. bridge group (JS program-bridge document door): apply → renderer wasm32 check → keep on success, revert on failure.
# 2. relay group (shell document half): apply → evidence crate wasm32 check (kernel `sync` + the two TLS-chain
#    feature lines the frozen kernel hunk would make unnecessary) → ALWAYS revert (blocked-by-freeze).
# 3. forms wgpu dev build for the browser probe: renderer wasm (trunk), browser boot, frame worker, prepare, activate.
cd /Users/ueli/Documents/semio || exit 1
T=.tmp-ticket-0918
G=$T/wp-n2/generated
EDITS="$T/🐍️n2-relay-edits.py"
export CARGO_INCREMENTAL=0 CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false
export CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket-0918/wp-n2/target
summarize() {
  local raw="$1" out="$2" rc="$3"
  { echo "exit=$rc"; echo "warnings=$(/usr/bin/grep -c ': warning' "$raw")"; echo "errors=$(/usr/bin/grep -c -E '(^error|: error)' "$raw")";
    /usr/bin/grep -E '(^error|: error)' "$raw" | head -80; echo "--- tail"; tail -15 "$raw"; } > "$out"
  rm -f "$raw"
}

echo "== bridge $(date '+%T')"
python3 "$EDITS" apply bridge || exit 3
cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --message-format short > "$G/n2-wasm32-bridge.raw" 2>&1
BRIDGE=$?
summarize "$G/n2-wasm32-bridge.raw" "$G/n2-wasm32-bridge-check.txt" $BRIDGE
if [ $BRIDGE -ne 0 ]; then python3 "$EDITS" revert bridge; echo "bridge reverted"; exit 4; fi
echo "bridge kept $(date '+%T')"

echo "== relay evidence $(date '+%T')"
python3 "$EDITS" apply relay || { python3 "$EDITS" revert relay; exit 5; }
(cd "$T/wp-n2/relay-wasm32-probe" && cargo check --target wasm32-unknown-unknown --message-format short) > "$G/n2-wasm32-relay.raw" 2>&1
RELAY=$?
python3 "$EDITS" revert relay
summarize "$G/n2-wasm32-relay.raw" "$G/n2-wasm32-relay-evidence.txt" $RELAY
echo "relay evidence exit=$RELAY (reverted) $(date '+%T')"

echo "== forms wgpu dev build $(date '+%T')"
R="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript"
D="🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
( cd "$R" && bun ../../🏗️compiler/🌐️wasm/📜️script.ts build dev ) > "$G/n2-renderer-wasm.raw" 2>&1
WASM=$?
tail -40 "$G/n2-renderer-wasm.raw" > "$G/n2-renderer-wasm.txt"; echo "exit=$WASM" >> "$G/n2-renderer-wasm.txt"; rm -f "$G/n2-renderer-wasm.raw"
[ $WASM -ne 0 ] && exit 6
( cd "$R" && bun "../../⚙️browser-build/📜️script.ts" generate-browser-boot && bun ./📜️script.ts generate-frame-worker ) > "$G/n2-browser-bundles.txt" 2>&1 || exit 7
( cd "$D" && bun ./📜️script.ts prepare forms wgpu dev && bun ./📜️script.ts activate forms wgpu dev ) > "$G/n2-forms-activate.txt" 2>&1
echo "activate exit=$? $(date '+%T')"
