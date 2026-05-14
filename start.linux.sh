#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"
echo "[start.linux] setup.native.script.sh session…"
SEMIO_SESSION_START=1 bash "$ROOT/setup.native.script.sh"
