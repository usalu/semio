#!/bin/zsh
# 🧪️ ST2 overlay job 5 (session 14c): stdio census + catalogue laws after the details-window and package-assembly fixes.
O=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-cx1-overlay
cd "$O" || exit 1
echo "== disk $(df -g / | awk 'NR==2{print $4}') GiB free $(date '+%T')"
echo "== stdio shipped_fleet + editor_catalog $(date '+%T')"; cargo test -p semio-s-plugin-stdio --test shipped_fleet --test editor_catalog --no-fail-fast --message-format=short; echo "== stdio census rc=$? $(date '+%T')"
