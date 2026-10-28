#!/bin/sh
# 🧮️ Second census pass (after 19:35): repo root depth 2, Bazel products area, Nx tooling area, repo cache loose trees.
R=/Users/ueli/Documents/semio
C="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/aud-disk-g"
cd "$R" || exit 1
echo "[DEBUG] census2 start $(date +%H:%M:%S)" >> "$O/progress.log"
nice -n 19 du -kx -d 2 "$R" > "$O/F2-repo-root-d2.txt" 2>/dev/null
echo "[DEBUG] F2 done $(date +%H:%M:%S)" >> "$O/progress.log"
nice -n 19 du -kx -d 1 ".cache/b/p/m/bazel" > "$O/P1-bazel-products.txt" 2>/dev/null
echo "[DEBUG] P1 done $(date +%H:%M:%S)" >> "$O/progress.log"
nice -n 19 du -kx -d 1 ".cache/b/t/m" > "$O/T1-tools-image.txt" 2>/dev/null
echo "[DEBUG] T1 done $(date +%H:%M:%S)" >> "$O/progress.log"
echo "[DEBUG] census2 complete $(date +%H:%M:%S)" >> "$O/progress.log"
