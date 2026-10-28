#!/bin/sh
# 🚀️ quietfc-g: applies the staged change to the live tree, anchor by anchor and idempotent (every edit is skipped when it is already there, and refuses when its anchor moved). Usage: sh apply.sh [check|apply] — `check` rebuilds the scratch overlay from the live tree, applies everything to it and runs the laws; `apply` then writes the live tree.
echo "SUPERSEDED (quietfc2, 10-27): the anchor route no longer carries the full-request bound; use stage.py check|apply|revert (flat batch) and verify.sh with QFC_ROOT=<mirror>" >&2; exit 2
set -e
H=$(cd "$(dirname "$0")" && pwd)
REPO=/Users/ueli/Documents/semio
case "${1:-check}" in
 check) sh "$H/rebuild.sh" && sh "$H/verify.sh" ;;
 apply) sh "$H/rebuild.sh" && python3 "$H/edit_schema.py" "$REPO" && python3 "$H/edit_code.py" "$REPO" && python3 "$H/install_new.py" "$REPO" ;;
 *) echo "usage: apply.sh [check|apply]" >&2; exit 2 ;;
esac
