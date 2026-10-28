#!/bin/sh
# [gram-g] A/B of the JSC heap cap on the same warm scratch store: label-A forceRAMSize only, label-B + gcMaxHeapSize. Sequential (the pack store is single-writer).
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
common="STORE=/private/tmp/gram-g/warm KEEP=1 SEMIO_GRAPH_WORKERS=0 RSS_CAP_MB=6000 LIMIT=250 CONC=8 PER_PROJECT=1 PER_EVERY=25 PER_PROJECT_GC=0 PROBES=0 SAMPLE_S=10"
EX="EXCLUDE=✏️s/🧑‍💻dev/📋️project.json;=📋️project.json;=Cargo.toml"
sh "$C/gram-g-tools/run.sh" abB $common "$EX" BUN_JSC_forceRAMSize=3221225472 BUN_JSC_gcMaxHeapSize=1200000000 > /dev/null 2>&1
sh "$C/gram-g-tools/run.sh" abA $common "$EX" BUN_JSC_forceRAMSize=3221225472 > /dev/null 2>&1
