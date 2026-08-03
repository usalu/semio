#!/usr/bin/env bash
set -euo pipefail
TICKET_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$TICKET_DIR/../../../../../.." && pwd)"
# ticket is .repo/tickets/YY/MM/DD/SLUG — 6 levels up is wrong. Use git root.
ROOT="$(git -C "$TICKET_DIR" rev-parse --show-toplevel)"
cd "$ROOT"
rebuild() {
  local label="$1"
  local script
  script=$(find "$2" -path '*/🦀️rust/📜️script.ts' | head -1)
  echo "[rebuild] $label -> $script"
  # bust stale skip so profile path is exercised
  local pkgdir
  pkgdir="$(dirname "$script")/pkg"
  rm -f "$pkgdir"/*_bg.wasm || true
  bun "$script" wasm 2>&1 | tee "$TICKET_DIR/${label}-wasm-build.txt"
}
rebuild editor "🧰️framework/🔨️module/✍️editor"
rebuild flow-core "🧰️framework/🛍️product/💻️os/🔨️module/🌊️flow/