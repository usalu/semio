#!/bin/zsh
# usage: gg.sh <fixed-needle> [pathspec...]   (default pathspec = the renderer engine tree); prints path (relative to E, shortened) + char-truncated line
. /private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/env.sh
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
