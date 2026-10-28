#!/bin/sh
# 🧮️ Disk census for auditor aud-disk-g: sequential, low priority, read-only du walks.
R=/Users/ueli/Documents/semio
C="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/aud-disk-g"
cd "$R" || exit 1
run() { name="$1"; shift; echo "[DEBUG] $name start $(date +%H:%M:%S)" >> "$O/progress.log"; nice -n 19 "$@" > "$O/$name.txt" 2>/dev/null; echo "[DEBUG] $name done $(date +%H:%M:%S) rc=$?" >> "$O/progress.log"; }
run A-repo-cache-d1 du -skx -d1 ".🧬semio/🦑️repo/⚡️cache"
run B-native-cache-d1 du -skx -d1 ".cache"
run C-dmg-allocated du -sk .cache/b/d/fixed.dmg .cache/b/g/fixed.dmg .cache/b/p/fixed.dmg .cache/b/t/fixed.dmg .cache/b/n/fixed.dmg .cache/b/o/fixed.dmg .cache/b/l/fixed.dmg .cache/b/c .🧬semio/🦑️repo/⚡️cache/rust/fixed/fixed.dmg
run D-p-mount du -sk .cache/b/p/m
run D-d-mount du -sk .cache/b/d/m
run D-g-mount du -sk .cache/b/g/m
run D-t-mount du -sk .cache/b/t/m
run D-o-mount du -sk .cache/b/o/m
run E-rust-mount du -sk ".🧬semio/🦑️repo/⚡️cache/rust/fixed/m"
run F-repo-root-d1 du -skx -d1 "$R"
run G-tools-tree du -sk ".🧬semio/🦑️repo/⚡️cache/tools"
run H-home-caches-d1 du -skx -d1 "$HOME/Library/Caches"
run I-home-dotcache-d1 du -skx -d1 "$HOME/.cache"
run J-sinc du -sk "$HOME/.sinc"
run K-ticket-generated du -sk "$C/🗑️generated"
run L-ticket-tools du -sk "$C"/*-tools
run M-framework-generated-process du -sk "🧰️framework/🤖️generated/process"
echo "[DEBUG] census complete $(date +%H:%M:%S)" >> "$O/progress.log"
