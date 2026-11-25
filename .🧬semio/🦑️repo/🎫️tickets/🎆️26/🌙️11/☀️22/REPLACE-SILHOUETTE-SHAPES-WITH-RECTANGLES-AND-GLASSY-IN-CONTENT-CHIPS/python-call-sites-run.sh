#!/bin/sh
# usage: render-site.sh SceneName
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/REPLACE-SILHOUETTE-SHAPES-WITH-RECTANGLES-AND-GLASSY-IN-CONTENT-CHIPS"
PY="/Users/ueli/Documents/semio/.🧬semio/🎓️teaching/⚡️cache/python/bin/python"
cd /Users/ueli/Documents/semio || exit 1
mkdir -p "$T/🗑️generated/sites"
lockf -k /private/tmp/claude-501/semio-manim.lock timeout 300 env PYTHONUTF8=1 PYTHONDONTWRITEBYTECODE=1 SEMIO_TEACHING_LANGUAGE=${LANGV:-en} SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_PROFILE=final "$PY" -m manim -ql -s -r 1280,720 --disable_caching --media_dir "$T/🗑️generated/sites" -o "$1" "$T/python-call-sites.py" "$1" > "$T/🗑️generated/sites/$1.txt" 2>&1
echo "exit $?" >> "$T/🗑️generated/sites/$1.txt"
