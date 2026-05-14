#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

if ! command -v bun >/dev/null 2>&1; then
  echo "[setup.linux] Installing Bun…"
  curl -fsSL https://bun.sh/install | bash
  # shellcheck disable=SC1091
  if [[ -f "$HOME/.bun/env" ]]; then
    # shellcheck source=/dev/null
    source "$HOME/.bun/env"
  fi
  export PATH="${BUN_INSTALL:-$HOME/.bun/bin}:$PATH"
fi

echo "[setup.linux] bun install…"
bun install

echo "[setup.linux] bun setup.ts…"
bun setup.ts
