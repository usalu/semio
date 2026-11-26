# 👀️ Product sheets (side, front, plan; line and solid) of asset documents (sourced by q.sh): sheet.sh <tag> <label>=<file.json>:<scale>[:<pose>]…   writes $GEN/sheets/<asset>.s<scale>.<pose>.<label>.png with the r3-2d tool of the 3D ticket; the default pose when none is named.
shift
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
for t in "$@"; do
  label="${t%%=*}"; parts=("${(@s/:/)${t#*=}}"); file="${parts[1]}"; [[ "$file" == /* ]] || file="$GEN/$file"
  work="$GEN/sheets/work-$label"; mkdir -p "$work"; cp "$file" "$work/🔣️.json"
  id="$(python3 -c "import json,sys; a=json.load(open(sys.argv[1]))['asset']; print(a['id'], a['presentation']['defaultPose'])" "$work/🔣️.json")"; pose="${parts[3]:-${id#* }}"; id="${id%% *}"
  png="$(cap 600 "$T14" "$B" r3-2d.ts --file "$work/🔣️.json" --scale "${parts[2]}" --pose "$pose" 2>&1 | tail -1)"
  [[ -f "$png" ]] && mv "$png" "$GEN/sheets/$id.s${parts[2]}.$pose.$label.png" && echo "$GEN/sheets/$id.s${parts[2]}.$pose.$label.png" || echo "no sheet: $png"
  rm -rf "$work"
done
