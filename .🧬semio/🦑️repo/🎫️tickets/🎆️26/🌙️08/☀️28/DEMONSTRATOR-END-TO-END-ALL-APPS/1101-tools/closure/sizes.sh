#!/bin/bash
cd /Users/ueli/Documents/semio
C=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS; O=$C/🗑️generated/1101; M=$O/closure-manifests.txt
while IFS='|' read -r n d c; do echo "$n | $c | lock-closure $(python3 $C/1101-tools/closure/real.py $M "$d/Cargo.lock" $c $1 | cut -d' ' -f1) | $(python3 $C/1101-tools/closure/real.py $M "$d/Cargo.lock" $c $1 | sed 's/.*ws \([0-9]*\).*{\(.*\)}/ws=\1 {\2}/')"; done <<'EOT'
bearbeiten|✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d|semio-s-artifact-process-process3d-editor
aussuchen|✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation|semio-s-artifact-sourcing-curation
verfolgen|✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap|semio-s-artifact-gis-gismap-gis2d-unit-tests
koordinator|✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad|semio-s-artifact-cad-cad
aggregator|✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d|semio-s-artifact-puzzle-3d-editor
statik|✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d|semio-s-artifact-fem-3d-editor
energie|✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model|semio-s-artifact-energy-model-editor-mounted-tests
generator|✏️s/🧑‍💻dev/🧩️composition|semio-s-composition-laws
EOT
