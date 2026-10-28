#!/bin/sh
cd "/Users/ueli/Documents/semio"
run(){ nice -n 10 bun run "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/bzsrv-g-tools/analysis.ts" "$1" "$2" "/Users/ueli/Documents/semio/.cache/b/p/m/bazel/workspaces/s1" 760c0f403769cd16 40 >> "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/bzsrv-g/variants.log" 2>&1; }
run v1 "-Xmx1024m"
run v2 "-Xmx768m -XX:+UseSerialGC"
run v3 "-Xmx1024m -XX:+UseSerialGC -XX:MinHeapFreeRatio=5 -XX:MaxHeapFreeRatio=20"
run v4 "-Xmx1024m -XX:G1PeriodicGCInterval=5000 -XX:G1PeriodicGCSystemLoadThreshold=0 -XX:MaxHeapFreeRatio=20"
