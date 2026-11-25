#!/bin/zsh
# 👀️ Renders product 2D sheets (side, front, plan; line and solid view) of asset documents with the r3-2d tool of the 3D ticket, one process after the other, into $GEN/look/<tag>.<id>.<scale>.<pose>.png: look.sh <tag> <dir with <id>.json | catalog> <id>:<scale>:<pose> …   (pose `-` is the default pose; run it through q.sh with 2500 MB)
source "${0:h}/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
tag="$1"; src="$2"; shift 2
out="$GEN/look"; mkdir -p "$out"
for item in "$@"; do
  a="${item%%:*}"; rest="${item#*:}"; scale="${rest%%:*}"; pose="${rest#*:}"
  f="$src/$a.json"; [[ "$src" == catalog ]] && f="$(asset "$a")"
  [[ "$pose" == "-" ]] && pose="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$f")"
  work="$out/work-$$/$a"; mkdir -p "$work"; cp "$f" "$work/🔣️.json"
  png="$(cd "$T14" && timeout 300 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$scale" --pose "$pose" 2>&1 | tail -1)"
  rm -rf "$out/work-$$"
  if [[ -f "$png" ]]; then mv "$png" "$out/$tag.$a.$scale.$pose.png"; sips -Z ${SIZE:-1500} "$out/$tag.$a.$scale.$pose.png" --out "$out/$tag.$a.$scale.$pose.png" > /dev/null; echo "$out/$tag.$a.$scale.$pose.png"; else echo "no sheet for $item: $png" | cut -c1-300; fi
done
