#!/bin/sh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/REPLACE-SILHOUETTE-SHAPES-WITH-RECTANGLES-AND-GLASSY-IN-CONTENT-CHIPS"
PY="/Users/ueli/Documents/semio/.🧬semio/🎓️teaching/⚡️cache/python/bin/python"
cd /Users/ueli/Documents/semio || exit 1
for scene in "$@"; do
  lockf -k /private/tmp/claude-501/semio-manim.lock timeout 400 env PYTHONUTF8=1 PYTHONDONTWRITEBYTECODE=1 "$PY" "$T/python-scene-stills.py" "$scene" en > "$T/🗑️generated/still-$scene.txt" 2>&1
  echo "exit $?" >> "$T/🗑️generated/still-$scene.txt"
done
echo CHAIN_DONE
