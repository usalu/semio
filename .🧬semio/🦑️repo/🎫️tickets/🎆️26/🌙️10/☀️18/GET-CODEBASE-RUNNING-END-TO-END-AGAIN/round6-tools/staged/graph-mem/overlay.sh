#!/bin/sh
# rebuilds the scratch overlay from the live tree and the staged set: generators, ops, mirror, apply
# usage: overlay.sh [OVERLAY_DIR]   (default /private/tmp/graph-mem/overlay)
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
OVERLAY=${1:-/private/tmp/graph-mem/overlay}
python3 "$HERE/gen_memory_fixture.py" >/dev/null
for gen in "$HERE"/gen_*_fixture.py; do [ "$gen" = "$HERE/gen_memory_fixture.py" ] || python3 "$gen" >/dev/null; done
python3 "$HERE/ops_def.py"
python3 "$HERE/mirror.py" "$OVERLAY" >/dev/null
python3 "$HERE/apply.py" apply --write --root "$OVERLAY" --ignore-running | tail -1
echo "overlay ready: $OVERLAY"
