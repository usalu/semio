#!/bin/sh
# builds the verification overlay and applies the set to it (never to the checkout); GROUP=1|2|all (default all)
set -e
cd "$(dirname "$0")"
OV=${1:-/private/tmp/conc-bazel/ov}
python3 mkoverlay.py "$OV" >/dev/null
python3 apply.py --root "$OV" --group "${GROUP:-all}" --write --backup "$OV/../backup-ov"
