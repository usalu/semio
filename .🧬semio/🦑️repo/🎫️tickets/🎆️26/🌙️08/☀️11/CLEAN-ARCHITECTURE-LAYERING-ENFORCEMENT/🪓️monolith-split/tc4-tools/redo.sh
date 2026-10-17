#!/bin/zsh
# 🔁️ Regenerates the split, the name lists, the registry and runs the oracle diff. usage: redo.sh
cd /Users/ueli/Documents/semio || exit 1
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d"
ulimit -n 65536; set -o pipefail
python3 "$F/tc4-tools/restructure.py" | tail -1 || exit 1
: > "$F/🗑️generated/tc4-names-new.txt"; : > "$F/🗑️generated/tc4-names-new.err"
for d in 🔭️viewer-laws 🪟️editor-mode-laws 🧭️editor-structure-laws 🎮️editor-command-laws 🧮️editor-unit-laws 🕹️editor-surface-laws; do
  nice -n 10 bun "$F/tc4-tools/names.ts" "$G/$d/🦀️.rs" > "$F/🗑️generated/tc4-names-new-$d.txt" 2>>"$F/🗑️generated/tc4-names-new.err" || exit 1
  cat "$F/🗑️generated/tc4-names-new-$d.txt" >> "$F/🗑️generated/tc4-names-new.txt"
done
python3 "$F/tc4-tools/registry.py" | tail -1 || exit 1
python3 "$F/tc4-tools/oracle.py" 2>&1 | grep -E "errors|base" | tail -8
python3 "$F/tc4-tools/oracle-diff.py"
