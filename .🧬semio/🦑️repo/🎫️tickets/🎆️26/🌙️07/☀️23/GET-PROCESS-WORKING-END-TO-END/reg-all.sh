#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
P="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d"
REG_CWD=$P $T/reg-retry.sh host test -p semio-s-artifact-process-process3d-host --no-fail-fast
REG_CWD=$P $T/reg-retry.sh smoke test -p semio-s-artifact-process-process3d-editor --test process3d_bearbeiten_pane_smoke --no-fail-fast
$T/reg-retry.sh comp test -p semio-hub-process-component --no-fail-fast
touch $T/🗑️generated/reg/all.done
