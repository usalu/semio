#!/bin/zsh
# [DEBUG] sequential per-workspace checks of the mutation-report split under one guard slot
T=${0:A:h}; G="$T/🗑️generated/claude-test-support"; C=$T/claude-test-support-cargo.sh
CARGO_CWD="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf" $C -p semio-s-artifact-stdio-pdf-presence-lane -p semio-s-artifact-stdio-pdf-view-lane --lib --target wasm32-wasip2 > $G/s4-2.txt 2>&1; echo "2 exit $?" >> $G/seq4.txt
CARGO_CWD="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag" $C -p semio-s-artifact-dag-dag-config -p semio-s-artifact-dag-dag-dag --lib > $G/s4-3.txt 2>&1; echo "3 exit $?" >> $G/seq4.txt
CARGO_CWD="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d" $C -p semio-s-artifact-wfc-2d-config -p semio-s-artifact-wfc-2d-transient -p semio-s-artifact-wfc-2d-v1 --lib > $G/s4-4.txt 2>&1; echo "4 exit $?" >> $G/seq4.txt
