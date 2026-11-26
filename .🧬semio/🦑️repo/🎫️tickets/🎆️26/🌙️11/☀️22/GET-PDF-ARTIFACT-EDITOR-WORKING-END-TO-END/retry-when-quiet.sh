#!/bin/sh
# usage: retry-when-quiet.sh <tag> [maxwait-minutes]  (gates per FILE: collects the source files named by the compiler errors of the failed runs, waits until none of them was edited in the last 15 minutes (at most the cap, default 30), then runs anyway: emits demonstrator, settles, verifies the settled plugins strictly and rechecks the plugins that failed phase 1; files still failing and unchanged for 15 minutes are mine to fix at the root)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
tag=${1:?tag}; cap=${2:-30}
g="$here/🗑️generated"
python3 "$here/error-files.py" $(ls "$g"/vf-c1/*.log "$g"/em-c1/*.log "$g"/ck4/*.log "$g"/ck6/*.log "$g"/ck7/*.log 2>/dev/null) > "$g/blocker-files-$tag.txt"
waited=0
while [ "$waited" -lt "$cap" ]; do
  recent=""
  while read -r f; do [ -n "$(find "$f" -mmin -15 2>/dev/null)" ] && recent="$f"; done < "$g/blocker-files-$tag.txt"
  [ -z "$recent" ] && break
  sleep 60; waited=$((waited+1))
done
echo "waited $waited min; ran with $(wc -l < "$g/blocker-files-$tag.txt") blocker files, still-recent: ${recent:-none}" > "$g/retry-$tag.note"
sh "$here/chain-settle-verify.sh" "$tag" semio-hub-demonstrator-component -- semio-hub-cad-component semio-hub-dag-component semio-hub-note-component semio-hub-demonstrator-component semio-hub-gis-component
rm -f "$g/ck-$tag.summary"
sh "$here/check-plugin-components.sh" "ck-$tag" semio-hub-forms-component semio-hub-imperative-component semio-hub-layout-component semio-hub-lowpoly-component semio-hub-mathematical-component semio-hub-norm-component semio-hub-norm-energy-component semio-hub-energy-component semio-hub-fem-component semio-hub-draw-component
