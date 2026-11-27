#!/bin/sh
# usage: resume-sweep.sh  (the held descriptor runs after the PDF pause, one after the other: emit, settle and strictly verify the crates that checked clean, then recheck the crates that failed phase 1 and the extension crates)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
E="semio-hub-space-component semio-hub-stdio-component semio-hub-stdio-media-component semio-hub-stdio-mesh-component semio-hub-stdio-office-component semio-hub-reasoning-component semio-hub-energy-component semio-hub-draw-component semio-hub-demonstrator-component semio-hub-playbook-component semio-hub-puzzle-component semio-hub-wfc-component"
sh "$here/chain-settle-verify.sh" rs1 $E -- $E semio-hub-gis-component semio-hub-cad-component semio-hub-procedural-component
sh "$here/chain-settle-verify.sh" rs2 semio-hub-flow-component semio-hub-stdio-semio-component -- semio-hub-flow-component semio-hub-stdio-semio-component
sh "$here/check-plugin-components.sh" ck18 $(cat "$here/🗑️generated/ck17.list") $(cut -d'|' -f1 "$here/plugin-descriptors-components.txt" | grep '^semio-s-plugin\|host-test')
