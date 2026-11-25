#!/bin/zsh
# 🔍️ A coarse band drawn large: zoom.sh <file.json> <name> <scale> [pose]   puts the geometry of the scale into the 1:50 band of a copy and renders that sheet (peek.sh), so a 1:500 drawing can be read.
source "${0:A:h}/env.sh"
mkdir -p "$GEN/sheets"; tmp="$GEN/sheets/zoom-$2-$3.json"
python3 - "$1" "$3" "$tmp" <<'PY'
import json, sys
d = json.load(open(sys.argv[1])); r = d["asset"]["representations"]
r["50"]["geometry"] = r[sys.argv[2]]["geometry"]
json.dump(d, open(sys.argv[3], "w"))
PY
"$W/peek.sh" "$tmp" "$2-zoom$3" 50 $4; rm -f "$tmp"
