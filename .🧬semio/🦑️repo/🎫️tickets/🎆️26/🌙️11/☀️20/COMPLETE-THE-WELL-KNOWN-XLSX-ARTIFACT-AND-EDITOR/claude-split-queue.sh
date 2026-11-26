#!/bin/zsh
# [DEBUG] runs the guarded split jobs one at a time: descriptors, tests, then the measured build of stdio and stdio-office
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; L=$T/🗑️generated/claude-split
until grep -q "^exit " $L/check1.log; do sleep 30; done
MIN_DISK_GIB=20 $T/claude-resource-guard.sh $T/claude-split-descriptors.sh > $L/descriptors.log 2>&1; echo "exit $?" >> $L/descriptors.log
MIN_DISK_GIB=20 $T/claude-resource-guard.sh $T/claude-split-tests.sh > $L/tests.log 2>&1; echo "exit $?" >> $L/tests.log
