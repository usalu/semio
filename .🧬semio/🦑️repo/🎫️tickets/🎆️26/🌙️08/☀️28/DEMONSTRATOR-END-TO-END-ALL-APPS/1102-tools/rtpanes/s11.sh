#!/bin/bash
D=$(cd "$(dirname "$0")" && pwd); C=$(cd "$D/../.." && pwd)
while [ ! -f "$C/🗑️generated/1102/rtpanes-seq-s10.rc" ]; do sleep 20; done
"$D/run.sh" statik "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" -p semio-s-artifact-fem-3d-editor --test statik_pane_smoke
"$D/run.sh" bearbeiten "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d" -p semio-s-artifact-process-process3d-editor --test process3d_editor_unit -- bearbeiten_pane
"$D/run.sh" koordinator "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad" -p semio-s-artifact-cad-cad --test cad_editor_unit -- koordinator_pane
echo done > "$C/🗑️generated/1102/rtpanes-seq-s11.rc"
