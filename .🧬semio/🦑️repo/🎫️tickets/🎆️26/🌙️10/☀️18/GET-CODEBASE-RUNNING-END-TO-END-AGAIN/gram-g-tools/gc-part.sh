#!/bin/sh
# [gram-g] post-inference RSS under three JSC heap settings on the SAME complete scratch store (records valid): A = forceRAMSize 3 GB, B = + gcMaxHeapSize 1.5 GB, C = forceRAMSize 1.5 GB. Sequential; each variant works on its own copy of the store.
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GER-CODEBASE-RUNNING-END-TO-END-AGAIN'
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
for variant in "$@"; do
 rm -rf /private/tmp/gram-g/gp$variant; cp -R /private/tmp/gram-g/cold4 /private/tmp/gram-g/gp$variant
 case $variant in A) env="BUN_JSC_forceRAMSize=3221225472";; B) env="BUN_JSC_forceRAMSize=3221225472 BUN_JSC_gcMaxHeapSize=1500000000";; C) env="BUN_JSC_forceRAMSize=1610612736";; esac
 sh "$C/gram-g-tools/run.sh" gp$variant STORE=/private/tmp/gram-g/gp$variant KEEP=1 SEMIO_GRAPH_WORKERS=0 SEMIO_GRAPH_MEMORY_MB=1792 RSS_CAP_MB=6500 WALL_S=900 PART=1 PROBES=0 SAMPLE_S=5 CHAIN_WIRE_MS=7200000 $env > /dev/null 2>&1
 rm -rf /private/tmp/gram-g/gp$variant
done
