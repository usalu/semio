#!/usr/bin/env bash
# 👀️ Renders the product 2D sheet (side, front, plan; line and solid view) of an asset at a scale with the r3-2d tool of the 3D ticket into 🗑️generated/matrix/MB-2/look/<id>/<tag>.<scale>[.<pose>].png and prints the path: look.sh <id> <scale> <tag> [pose]
source "$(dirname "$0")/env.sh"
T14="$U/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
F=$(asset "$1"); OUT="$G/look/$1"; WORK="$OUT/work/$1"; mkdir -p "$WORK"; cp "$F" "$WORK/🔣️.json"
POSE=${4:-$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))[\"asset\"][\"presentation\"][\"defaultPose\"])" "$F")}
cd "$T14" || exit 1
PNG=$(timeout 300 "$BUN" r3-2d.ts --file "$WORK/🔣️.json" --scale "$2" --pose "$POSE" | tail -1)
rm -rf "$OUT/work"
[[ -f "$PNG" ]] || { echo "no sheet: $PNG"; exit 1; }
mv "$PNG" "$OUT/$3.$2${4:+.$4}.png"; sips -Z ${SIZE:-1100} "$OUT/$3.$2${4:+.$4}.png" --out "$OUT/$3.$2${4:+.$4}.s.png" > /dev/null; echo "$OUT/$3.$2${4:+.$4}.s.png"
