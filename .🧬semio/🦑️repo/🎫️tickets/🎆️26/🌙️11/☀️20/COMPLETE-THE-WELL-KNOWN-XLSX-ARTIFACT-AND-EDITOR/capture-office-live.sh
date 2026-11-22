#!/bin/sh
G="$1"; OFF=$(cat "$G/office-codec-green-offset.txt")
until [ -f "$G/office-codec-green/stdio-office-cargo-artifact.json" ]; do
  [ -f "$G/office-codec-green-build.exit" ] && [ ! -f "$G/office-codec-green/stdio-office-cargo-artifact.json" ] && sleep 20 && [ -f "$G/office-codec-green-build.exit" ] && echo "[DEBUG] build exited before capture" && exit 1
  tail -c +"$((OFF+1))" "$G/office-codec-green-build.log" > "$G/office-codec-green-slice.log"
  repo tools exec --light bun -- "$(dirname "$G")/capture-office-component.ts" "$G/office-codec-green-slice.log" "$G/office-codec-green" >/dev/null 2>&1
  sleep 1
done
echo "[DEBUG] captured"
