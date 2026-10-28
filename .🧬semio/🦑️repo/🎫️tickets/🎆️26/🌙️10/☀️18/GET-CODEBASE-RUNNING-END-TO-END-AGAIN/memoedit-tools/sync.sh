#!/bin/sh
# 🔄 Copies every file the live tree changed since the mirror was made into the mirror, except the files of the batches memoedit and derived: sh sync.sh <mirror name> <mirror root>
here=$(cd "$(dirname "$0")" && pwd); C=$(dirname "$here")
python3 -I "$C/mirror-tools/mirror.py" diff "$1" 2>/dev/null | grep '^M ' | cut -c3- | grep -v '🧠️memo/\(🟦️.ts\|📤️relevance/🟦️.ts\|🚪️command/🟦️.ts\|🧬️schema/🔣️.json\|🧪️tests/✏️edit/🟦️.ts\|🧮️derive/🟦️.ts\)$' | grep -v '🏗️build/\(🧭️invocation/🕸️graph/🟦️.ts\|⌨️command/⚙️runtime/🟦️.ts\)$' | while IFS= read -r rel; do cp -p "/Users/ueli/Documents/semio/$rel" "$2/$rel" && echo "synced $rel"; done | wc -l
