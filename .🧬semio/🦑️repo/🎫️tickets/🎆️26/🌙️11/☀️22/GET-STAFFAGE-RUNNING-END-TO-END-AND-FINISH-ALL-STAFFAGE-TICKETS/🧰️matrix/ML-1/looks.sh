#!/bin/zsh
# 👀️ Renders several product 2D sheets (side, front, plan; line and solid view; r3-2d tool of the 3D ticket) one after the other under one grant of the memory queue into 🗑️generated/matrix/ML-1/look/<tag>.<id>.<scale>.<pose or clip-t>.png and prints the paths: looks.sh <tag> <id|file.json>:<scale>:<pose>[:<clip>:<seconds>]…   (an id reads the catalog document; with a clip the pose field is ignored; PRE holds shell commands to run first under the same grant, for example the trial that writes the document a sheet is drawn from)
source "${0:A:h}/env.sh"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
tag="$1"; shift; out="$GEN/look"; mkdir -p "$out"; cmd="${PRE:+$PRE; }cd '$T14'"; n=0
for spec in "$@"; do
  parts=("${(@s/:/)spec}"); f="${parts[1]}"; id="${f:t:r}"; [[ "$f" == *.json ]] || { id="$f"; f="$(asset "$f")"; }
  n=$((n + 1)); work="$out/work-$$-$n/$id"; mkdir -p "$work"
  name="$tag.$id.${parts[2]}.${parts[4]:+${parts[4]}-t${parts[5]}}${parts[4]:-${parts[3]}}"; [[ -n "${parts[4]}" ]] && name="$tag.$id.${parts[2]}.${parts[4]}-t${parts[5]}"
  how="--pose ${parts[3]}"; [[ -n "${parts[4]}" ]] && how="--clip ${parts[4]} --t ${parts[5]}"
  cmd="$cmd; cp '$f' '$work/🔣️.json'; p=\$('$B' r3-2d.ts --file '$work/🔣️.json' --scale ${parts[2]} ${=how} 2>&1 | tail -1); if [[ -f \"\$p\" ]]; then mv \"\$p\" '$out/$name.png'; sips -Z ${SIZE:-1500} '$out/$name.png' --out '$out/$name.png' > /dev/null; echo '$out/$name.png'; else echo \"no sheet $name: \$p\"; fi"
done
qs ${MB:-2500} ${TO:-1800} "$cmd" 2>&1 | grep -v '^\[capacity\]'
rm -rf "$out"/work-$$-*
