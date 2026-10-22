#!/bin/sh
# usage: cycle.sh [step] [root]  rebuilds one step against the chain overlay's state before it and re-applies it (undo first)
cd "$(dirname "$0")" || exit 1
STEP="${1:-config}"; ROOT="${2:-/private/tmp/i18n-core/t6}"
python3 apply.py --step "$STEP" --root "$ROOT" --undo --yes >/dev/null 2>&1
python3 build.py --step "$STEP" --root "$ROOT" || exit 2
python3 apply.py --step "$STEP" --root "$ROOT" --yes | tail -2
