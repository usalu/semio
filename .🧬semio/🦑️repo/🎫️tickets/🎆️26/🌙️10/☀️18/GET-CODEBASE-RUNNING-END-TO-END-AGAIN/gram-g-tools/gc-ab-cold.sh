#!/bin/sh
# [gram-g] cold A/B of the JSC heap cap: fresh copy of the closure stores without project records (every project is inferred), 8 concurrent like production, governor budget of a 16 GB machine.
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
EX="EXCLUDE=✏️s/🧑‍💻dev/📋️project.json;=📋️project.json;=Cargo.toml"
for variant in B A; do
 rm -rf /private/tmp/gram-g/cold$variant; mkdir -p /private/tmp/gram-g/cold$variant/inference; cp -R /private/tmp/gram-g/cold/build-input-closures /private/tmp/gram-g/cold/build-lock-closures /private/tmp/gram-g/cold$variant/; cp -R /private/tmp/gram-g/cold/inference/ip /private/tmp/gram-g/cold/inference/rr /private/tmp/gram-g/cold$variant/inference/
 if [ $variant = B ]; then cap="BUN_JSC_gcMaxHeapSize=1500000000"; else cap="GRAM_NOCAP=1"; fi
 sh "$C/gram-g-tools/run.sh" cold$variant STORE=/private/tmp/gram-g/cold$variant KEEP=1 SEMIO_GRAPH_WORKERS=0 SEMIO_GRAPH_MEMORY_MB=1792 RSS_CAP_MB=6000 LIMIT=120 CONC=8 PER_PROJECT=1 PER_EVERY=10 PER_PROJECT_GC=0 PROBES=0 SAMPLE_S=10 "$EX" BUN_JSC_forceRAMSize=3221225472 $cap > /dev/null 2>&1
 rm -rf /private/tmp/gram-g/cold$variant
done
