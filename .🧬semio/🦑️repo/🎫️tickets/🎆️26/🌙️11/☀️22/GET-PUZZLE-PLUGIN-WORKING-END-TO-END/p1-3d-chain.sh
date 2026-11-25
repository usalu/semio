#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END"
export BUILDER_MEM=2500 BUILDER_TIMEOUT=10000
"$T/builder.sh" exec "bash $T/p1-3d-exec-split.sh"
"$T/builder.sh" exec "bash $T/p1-3d-exec4.sh"
bash "$T/p1-3d-pkgs.sh"
