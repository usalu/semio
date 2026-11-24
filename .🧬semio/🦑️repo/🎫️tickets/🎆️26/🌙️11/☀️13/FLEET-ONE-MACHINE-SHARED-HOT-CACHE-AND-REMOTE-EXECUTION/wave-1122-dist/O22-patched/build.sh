#!/bin/zsh
# regenerates the patched files from the play clone's current sealed files (pre-images) with edit-o22.py
set -e
here=$(cd "$(dirname "$0")" && pwd)
CL=${1:?clone sealed library dir}
SC=${2:?scratch dir}
F="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
rm -rf "$SC"; mkdir -p "$SC"
: > "$here/list.tsv"
for pair in "action.ts|🏗️build/🌐️remote-execution/🚀️offload/📜️action/🟦️.ts" "lock.ts|⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts" "script.ts|⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts"; do
 name=${pair%%|*}; rel=${pair#*|}
 mkdir -p "$SC/$(dirname "$rel")"; cp "$CL/$rel" "$SC/$rel"; chmod u+w "$SC/$rel"
done
python3 "$here/edit-o22.py" "$SC"
for pair in "action.ts|🏗️build/🌐️remote-execution/🚀️offload/📜️action/🟦️.ts" "lock.ts|⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts" "script.ts|⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts"; do
 name=${pair%%|*}; rel=${pair#*|}
 cp "$SC/$rel" "$here/$name"
 printf '%s\t%s\t%s\t%s\n' "$name" "$F/$rel" "$(shasum -a 256 "$CL/$rel" | cut -d' ' -f1)" "$(shasum -a 256 "$SC/$rel" | cut -d' ' -f1)" >> "$here/list.tsv"
done
