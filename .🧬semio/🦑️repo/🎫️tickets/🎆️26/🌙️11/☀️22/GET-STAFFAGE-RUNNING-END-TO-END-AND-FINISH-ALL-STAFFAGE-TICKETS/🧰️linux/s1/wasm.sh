#!/usr/bin/env bash
# 🖊️ Native fallback for `@teaching/staffage-dwg-rs:wasm`: compiles the dependency-free DWG writer for wasm32-unknown-unknown with the pinned rustc in the tmpfs namespace of the puzzle ticket's `native-cargo.sh` and installs it at the declared output next to `🖊️dwg/🟦️.ts`. rustc is called directly because cargo resolves the dev-dependencies (acadrust, ezdwg-core) against the registry even for `--lib`, and they are not in the offline cargo home. Usage: wasm.sh
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../../.." && pwd)"
DWG="$ROOT/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🖊️dwg"
export DWG_OUT="$DWG/staffage_dwg.wasm"
LOWER_DIR=/nonexistent TMPFS_SIZE=300m \
  "$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END/native-cargo.sh" \
  "$DWG/📦️packages/🦀️rust" \
  'rustc --edition 2021 --crate-name staffage_dwg --crate-type cdylib --target wasm32-unknown-unknown -C opt-level=3 -C panic=abort -C strip=symbols -o /tmp/build/t/staffage_dwg.wasm 🦀️.rs && { cmp -s /tmp/build/t/staffage_dwg.wasm "$DWG_OUT" || cp /tmp/build/t/staffage_dwg.wasm "$DWG_OUT"; }'
ls -la "$DWG_OUT"
