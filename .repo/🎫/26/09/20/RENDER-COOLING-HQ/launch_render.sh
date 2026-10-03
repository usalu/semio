#!/bin/zsh
set -euo pipefail
ROOT="/Users/niloufarghandehariyoon/Documents/Master LUH/Hiwi/semio"
LOG="$ROOT/.repo/🎫/26/09/20/RENDER-COOLING-HQ/render.log"
export PATH="/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin"
export PYTHONUNBUFFERED=1
cd "$ROOT"
"$ROOT/.venv/bin/python" -u "$ROOT/tutorial/energy/demand/Cooling/full_cooling_video.py" -q h --no-play --force 2>&1 | tee -a "$LOG"
exit ${pipestatus[1]}
