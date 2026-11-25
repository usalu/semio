#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; D=/private/tmp/claude-501/-Users-ueli-Documents-semio/8cbd0d6d-32d5-4da7-9277-09843fb54394/scratchpad
B="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/📦️packages/🦀️rust"
P="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d"
L=$1; rm -f $D/fuel-$L.txt
for i in $(seq 1 40); do
  SEMIO_DEBUG_FUEL=1 SEMIO_DEBUG_NOWALL=1 $T/fuel2-cargo.sh "$P" $L test -p semio-s-artifact-process-process3d-editor --test process3d_bearbeiten_pane_smoke -- bearbeiten_pane_loads_applies_undoes_and_closes --nocapture
  grep -q "could not compile\|exceeds its source boundary\|repository:command failed" $D/fuel-$L.txt || break
  cp $D/fuel-$L.txt $D/fuel-$L.try$i.txt; sleep 300
done
