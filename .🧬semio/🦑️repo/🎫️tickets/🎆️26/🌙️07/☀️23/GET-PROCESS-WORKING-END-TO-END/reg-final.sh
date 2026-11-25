#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
P="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d"
until [ -e $T/🗑️generated/reg/comp-all.done ]; do sleep 10; done
REG_CWD=$P $T/reg-retry.sh host2 test -p semio-s-artifact-process-process3d-host --no-fail-fast
$T/reg-retry.sh hub6 test -p semio-hub-process --no-fail-fast
REG_CWD=$P $T/reg-retry.sh smoke2 test -p semio-s-artifact-process-process3d-editor --test process3d_bearbeiten_pane_smoke --test process3d_interaction_behavior --no-fail-fast
touch $T/🗑️generated/reg/final.done
