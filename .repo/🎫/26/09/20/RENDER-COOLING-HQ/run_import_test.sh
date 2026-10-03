#!/bin/zsh
set -euo pipefail
ROOT="/Users/niloufarghandehariyoon/Documents/Master LUH/Hiwi/semio"
cd "$ROOT"
export PATH="/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin"
export PYTHONUNBUFFERED=1
SCRIPT=$(ls -1d "$ROOT"/.repo/*/26/09/20/RENDER-COOLING-HQ/import_test.py | head -n 1)
exec "$ROOT/.venv/bin/python" -u "$SCRIPT"
