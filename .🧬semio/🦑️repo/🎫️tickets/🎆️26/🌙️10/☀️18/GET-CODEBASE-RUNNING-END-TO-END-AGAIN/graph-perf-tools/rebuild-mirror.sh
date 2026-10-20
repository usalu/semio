#!/bin/sh
# usage: rebuild-mirror.sh [DIR]  recreates the scratch repository root (default /private/tmp/gp/mirror) from the live library and applies the staged set into it.
HERE="$(cd "$(dirname "$0")" && pwd)"
DIR="${1:-/private/tmp/gp/mirror}"
python3 "$HERE/mirror.py" "$DIR" >/dev/null && (cd "$HERE/staged" && GP_ROOT="$DIR" python3 apply.py apply --ignore-running)
find "$DIR/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library" -type f -newermt "-10 minutes" -print0 | xargs -0 touch -t 202610201200 2>/dev/null
