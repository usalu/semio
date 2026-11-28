#!/usr/bin/env bash
# 🧮️ Full v3 browser matrix: restarts the viewers on the freshly published component, runs every app one after the other, then renders feature-matrix-v3.md.
# Usage: matrix-v3-all.sh [apps...]   (default: 3d 2d viewer3d viewer2d)
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated/matrix-v3"; mkdir -p "$G"
cd "$T" || exit 1
bash matrix-v3/serve-viewers.sh stop both > "$G/all-viewers.log" 2>&1; bash matrix-v3/serve-viewers.sh start both >> "$G/all-viewers.log" 2>&1
for app in ${@:-3d 2d viewer3d viewer2d}; do
  echo "$app start $(date +%H:%M)" >> "$G/all.log"
  node matrix-v3/run.mjs "$app" > "$G/all-$app.out" 2>&1
  echo "$app rc=$? $(date +%H:%M)" >> "$G/all.log"
done
node matrix-v3/report.mjs > "$G/all-report.out" 2>&1; echo "report rc=$? $(date +%H:%M)" >> "$G/all.log"
