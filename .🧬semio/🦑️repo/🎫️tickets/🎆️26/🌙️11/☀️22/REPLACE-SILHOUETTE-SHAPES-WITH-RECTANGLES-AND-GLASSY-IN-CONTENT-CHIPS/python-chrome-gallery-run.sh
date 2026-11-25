#!/bin/sh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/REPLACE-SILHOUETTE-SHAPES-WITH-RECTANGLES-AND-GLASSY-IN-CONTENT-CHIPS"
PY="/Users/ueli/Documents/semio/.🧬semio/🎓️teaching/⚡️cache/python/bin/python"
cd /Users/ueli/Documents/semio || exit 1
while ! grep -q CHAIN_DONE "$T/🗑️generated/chain.txt"; do sleep 10; done
lockf -k /private/tmp/claude-501/semio-manim.lock timeout 400 env PYTHONUTF8=1 PYTHONDONTWRITEBYTECODE=1 "$PY" -m manim -ql -s --disable_caching --media_dir "$T/🗑️generated/gallery" -o chrome_gallery "$T/python-chrome-gallery.py" Gallery > "$T/🗑️generated/still-chrome-gallery.txt" 2>&1
echo "exit $?" >> "$T/🗑️generated/still-chrome-gallery.txt"
echo GALLERY_DONE >> "$T/🗑️generated/chain.txt"
