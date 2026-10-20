#!/bin/sh
# Round-2 apply series (see apply-all.py). Dry run by default; --yes applies; --undo <group> restores; --sim proves the chain on an overlay.
exec python3 -I "$(cd "$(dirname "$0")" && pwd)/apply-all.py" "$@"
