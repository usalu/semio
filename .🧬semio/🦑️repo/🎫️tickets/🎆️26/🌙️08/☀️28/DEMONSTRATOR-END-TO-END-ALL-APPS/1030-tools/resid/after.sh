#!/bin/bash
D="$(dirname "$0")"; cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
until [ -f "$C/🗑️generated/1030/extfix-run-x1.rc" ]; do sleep 20; done
"$D/gis.sh" 8 9
bash "$D/run-retry.sh" x2 flow-bim flow-brep flow-dictionary flow-draw flow-list flow-logic flow-math flow-primitive flow-text cad-aec-building cad-aec-energy cad-aec-structure cad-spatial-shape process-concrete process-metal process-robotic process-wood sourcing-beams sourcing-slabs sourcing-windows
