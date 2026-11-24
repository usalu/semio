#!/bin/zsh
# regenerates the patched files from the play clone's current files (pre-images) and the live tree (the client function, the classifier, the merged action.ts) with edit-o23.py
set -e
here=$(cd "$(dirname "$0")" && pwd)
CL=${1:?clone library dir (sealed wrapper dir with the leading framework folder dropped, or <clone>/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library)}
LV=${2:?live library dir}
SC=${3:?scratch dir}
F="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B="🏗️build/🌐️remote-execution"
rm -rf "$SC"; mkdir -p "$SC"
: > "$here/list.tsv"
typeset -A copy
rels=("content.ts|$B/🗄️content/🟦️.ts" "action.ts|$B/🚀️offload/📜️action/🟦️.ts" "offload.ts|$B/🚀️offload/🟦️.ts" "client.ts|$B/🚀️offload/🔌️client/🟦️.ts" "failure.ts|$B/🚀️offload/🩺️failure/🟦️.ts" "offload-schema.json|$B/🚀️offload/🧬️schema/🔣️.json" "policy.ts|$B/🚀️offload/📐️policy/🟦️.ts")
for pair in $rels; do rel=${pair#*|}; mkdir -p "$SC/$(dirname "$rel")"; cp "$CL/$rel" "$SC/$rel"; chmod u+w "$SC/$rel"; done
cp "$here/static/content.ts" "$SC/$B/🗄️content/🟦️.ts"
cp "$here/static/action.ts" "$SC/$B/🚀️offload/📜️action/🟦️.ts"
cp "$here/static/offload.ts" "$SC/$B/🚀️offload/🟦️.ts"
cp "$LV/$B/🚀️offload/🩺️failure/🟦️.ts" "$SC/$B/🚀️offload/🩺️failure/🟦️.ts"
python3 "$here/edit-o23.py" "$SC" "$LV"
mkdir -p "$SC/$B/🚀️offload/🩺️failure/🧬️schema"; cp "$LV/$B/🚀️offload/🩺️failure/🧬️schema/🔣️.json" "$SC/$B/🚀️offload/🩺️failure/🧬️schema/🔣️.json"
cp "$SC/$B/🚀️offload/🩺️failure/🧬️schema/🔣️.json" "$here/failure-schema.json"
rows=("failure-schema.json|$B/🚀️offload/🩺️failure/🧬️schema/🔣️.json" "offload-schema.json|$B/🚀️offload/🧬️schema/🔣️.json" "policy.ts|$B/🚀️offload/📐️policy/🟦️.ts" "failure.ts|$B/🚀️offload/🩺️failure/🟦️.ts" "client.ts|$B/🚀️offload/🔌️client/🟦️.ts" "action.ts|$B/🚀️offload/📜️action/🟦️.ts" "content.ts|$B/🗄️content/🟦️.ts" "offload.ts|$B/🚀️offload/🟦️.ts")
for pair in $rows; do
 name=${pair%%|*}; rel=${pair#*|}
 [[ $name == failure-schema.json ]] || cp "$SC/$rel" "$here/$name"
 if [[ $name == failure-schema.json ]]; then pre=NEW; else pre=$(shasum -a 256 "$CL/$rel" | cut -d' ' -f1); fi
 printf '%s\t%s\t%s\t%s\n' "$name" "$F/$rel" "$pre" "$(shasum -a 256 "$here/$name" | cut -d' ' -f1)" >> "$here/list.tsv"
done
