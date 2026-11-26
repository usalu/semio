#!/bin/sh
# usage: retry-after-files-quiet.sh <tag> <cap-minutes> <file…>  (waits until none of the files was edited for 15 minutes, at most the cap, then emits reasoning, energy, draw and demonstrator, settles every emitted pair and verifies every settled plugin strictly)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
tag=${1:?tag}; cap=${2:?cap}; shift 2
waited=0
while [ "$waited" -lt "$cap" ]; do
  recent=""; for f in "$@"; do [ -n "$(find "$f" -mmin -15 2>/dev/null)" ] && recent="$f"; done
  [ -z "$recent" ] && break
  sleep 60; waited=$((waited+1))
done
echo "waited $waited min, still-recent: ${recent:-none}" > "$here/🗑️generated/retry-$tag.note"
sh "$here/chain-settle-verify.sh" "$tag" semio-hub-reasoning-component semio-hub-energy-component semio-hub-draw-component semio-hub-demonstrator-component -- semio-hub-dag-component semio-hub-note-component semio-hub-procedural-component semio-hub-reasoning-component semio-hub-energy-component semio-hub-draw-component semio-hub-demonstrator-component semio-hub-cad-component semio-hub-gis-component
