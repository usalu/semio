#!/bin/sh
# Landing rehearsal on a mirror (mirror --batch command): setup-g, oracle2-g, conv-g, obs-g fleet, then the flip. usage: sh rehearse.sh <mirror-root>
R="$1"; C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; cd "$R" || exit 2
SEMIO_STAGE_ROOT="$R" python3 -I "$C/setup-g-tools/staged/stage.py" apply | tail -1
python3 -I "$C/seal-g-tools/rehearse.py" "$C"
python3 -I "$C/seal-g-tools/flip.py" apply
