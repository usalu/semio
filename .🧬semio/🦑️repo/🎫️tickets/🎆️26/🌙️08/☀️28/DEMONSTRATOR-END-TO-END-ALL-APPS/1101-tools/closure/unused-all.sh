#!/bin/bash
cd /Users/ueli/Documents/semio
C=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS; O=$C/🗑️generated/1101
i=0
while IFS='|' read -r d c; do i=$((i+1)); python3 $C/1101-tools/closure/unused.py $O/closure-manifests.txt "$d/Cargo.lock" $c semio-s-artifact-stdio- > $O/closure-unused-$i.txt; done <<'EOT'
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d|semio-s-artifact-process-process3d-editor
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation|semio-s-artifact-sourcing-curation
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap|semio-s-artifact-gis-gismap-gis2d-unit-tests
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad|semio-s-artifact-cad-cad
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d|semio-s-artifact-puzzle-3d-editor
✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d|semio-s-artifact-fem-3d-editor
✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model|semio-s-artifact-energy-model-editor-mounted-tests
✏️s/🧑‍💻dev/🧩️composition|semio-s-composition-laws
EOT
echo done > $O/closure-unused-all.rc
