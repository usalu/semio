#!/bin/sh
# 🪞 rebase-g: mirror.sh <short-root> <repo-relative path>... : builds a sparse short-root mirror of the live tree (clones of the given paths via APFS/reflink `cp -c`, symlinks for everything else), so a staged batch can be applied and its single-file laws run without touching the live tree and without exceeding the 256 character path limit. Use a root of at most ~40 characters.
cd /Users/ueli/Documents/semio || exit 1
OV="$1"; shift
rm -rf "$OV"
exec python3 -I ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rebase-g-tools/mirror.py" "$OV" "$@"
