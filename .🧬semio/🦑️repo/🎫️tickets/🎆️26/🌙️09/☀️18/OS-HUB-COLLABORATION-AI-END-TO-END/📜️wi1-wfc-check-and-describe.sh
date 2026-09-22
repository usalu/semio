#!/usr/bin/env zsh
# 🀄️ Slice WI1 — ONE fleet-wasm-mutex hold that (1) type-checks the whole 🀄️wfc guest for
# `wasm32-wasip2` (a NATIVE check cannot: the native feature set links 🌊️flow, which a peer had
# red at 17:51) and, only if that is green, (2) rebuilds the component and re-emits the descriptor
# so the published payload contracts reach `contributions.inferenceServices[].payload`.
# Usage: 📜️wi1-wfc-check-and-describe.sh   (run INSIDE 📜️mutex-ordered.sh)
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
LEDGER="$GEN/wi1-describe-ledger.txt"
owner="🀄️wfc"
base="$ROOT/✏️s/🔌️plugins/$owner"
dir="$base/📦️packages/🦀️rust"
staged="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev/semio_s_plugin_wfc.wasm"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true

check_log="$GEN/wi1-wasm-check.txt"
echo "=== wasm32-wasip2 check of $owner at $(date -Iseconds) ===" > "$check_log"
( cd "$ROOT" && CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-wi1" cargo check --target wasm32-wasip2 \
    -p semio-s-plugin-wfc -p semio-s-artifact-wfc-bitmap -p semio-s-artifact-wfc-2d -p semio-s-artifact-wfc-3d -p semio-s-artifact-wfc-grid2d -p semio-s-artifact-wfc-grid3d \
    --features semio-s-artifact-wfc-bitmap/component-app-assembly,semio-s-artifact-wfc-2d/component-app-assembly,semio-s-artifact-wfc-3d/component-app-assembly,semio-s-artifact-wfc-grid2d/component-app-assembly,semio-s-artifact-wfc-grid3d/component-app-assembly ) >> "$check_log" 2>&1
crc=$?
echo "=== check exit $crc at $(date -Iseconds) ===" >> "$check_log"
if [ $crc -ne 0 ]; then
  printf '%s\tcheck\trc=%s\tDESCRIBE SKIPPED\t%s\n' "$owner" "$crc" "$(date -Iseconds)" >> "$LEDGER"
  exit $crc
fi

before_json=$([ -f "$base/🔣️.json" ] && stat -f %z "$base/🔣️.json" || echo 0)
before_pack=$([ -f "$base/🛂️.descriptor.semio" ] && stat -f %z "$base/🛂️.descriptor.semio" || echo 0)
before_wasm=$([ -f "$staged" ] && stat -f %z "$staged" || echo 0)
before_sha=$([ -f "$staged" ] && shasum -a 256 "$staged" | cut -c1-16 || echo none)
log="$GEN/wi1-describe-wfc.txt"
start=$(date +%s)
echo "=== describe $owner at $(date -Iseconds) (json=$before_json pack=$before_pack wasm=$before_wasm/$before_sha) ===" > "$log"
( cd "$dir" && bun ./📜️script.ts describe ) >> "$log" 2>&1
rc=$?
end=$(date +%s)
echo "=== exit $rc after $((end-start))s ===" >> "$log"
after_json=$([ -f "$base/🔣️.json" ] && stat -f %z "$base/🔣️.json" || echo 0)
after_pack=$([ -f "$base/🛂️.descriptor.semio" ] && stat -f %z "$base/🛂️.descriptor.semio" || echo 0)
after_wasm=$([ -f "$staged" ] && stat -f %z "$staged" || echo 0)
after_sha=$([ -f "$staged" ] && shasum -a 256 "$staged" | cut -c1-16 || echo none)
printf '%s\tdescribe\trc=%s\t%ss\tjson %s -> %s\tpack %s -> %s\twasm %s -> %s\tsha %s -> %s\t%s\n' \
  "$owner" "$rc" "$((end-start))" "$before_json" "$after_json" "$before_pack" "$after_pack" "$before_wasm" "$after_wasm" "$before_sha" "$after_sha" "$(date -Iseconds)" >> "$LEDGER"
exit $rc
