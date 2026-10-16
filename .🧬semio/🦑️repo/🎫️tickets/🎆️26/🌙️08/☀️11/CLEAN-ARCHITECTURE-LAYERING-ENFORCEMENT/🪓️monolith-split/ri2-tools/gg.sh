#!/bin/zsh
# usage: gg.sh <fixed-needle> [pathspec...]   (default pathspec = the renderer engine tree); prints path (relative to E, shortened) + char-truncated line
. /private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/env.sh
needle="$1"; shift
if [ $# -eq 0 ]; then set -- "$E"; fi
cd $ROOT
git -c core.quotepath=false grep -n -F -e "$needle" -- "$@" 2>/dev/null | python3 -c '
import sys
E="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/"
W=int(__import__("os").environ.get("GGW","170"))
for l in sys.stdin:
    l=l.rstrip("\n").replace(E,"E/").replace("🎯️targets/🧊️wgpu/","T/").replace("🧱️elements/","el/")
    print(l[:W])
'
