#!/bin/sh
# Round-6 apply series (see apply-all.py). Dry run by default; --yes applies; --undo <step> restores; --sim proves the chain on an overlay.
exec python3 -I "$(cd "$(dirname "$0")" && pwd)/apply-all.py" "$@"
