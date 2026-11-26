#!/bin/sh
# 🪚️ Follow-up of the hub carving build: waits for the hub pass, builds the three extension finals under the same lock, then lists the measured peaks of the sourcing closure units from the lane logs and the exported wasm sizes.
# @see ./hub-carve-build.sh
set -u
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
out="$here/🗑️generated/hub-carve"
while pgrep -f "hub-carve-build.sh hub" >/dev/null; do sleep 30; done
ONLY="semio-s-plugin-sourcing-beams-component semio-s-plugin-sourcing-slabs-component semio-s-plugin-sourcing-windows-component" sh "$here/hub-carve-build.sh" ext
{ python3 "$here/hub-carve-peaks.py" "$(( $(date +%s) - 86400 ))" sourcing; ls -l "$here/🗑️generated/native-wasm"/*.wasm; } > "$out/peaks.txt" 2>&1
