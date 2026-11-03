#!/usr/bin/env bash
# Usage: run.sh <label> [ids]: one process, exports the ids (default all casts) to generated/fm/<label>, prints CPU, writes digests-<label>.txt
set -u
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/fm"
S="🎓️teaching/🏛️architecture/🧍️staffage"
rm -rf "$G/$1"
bun "$T/🧰️pf/bench.ts" "$S/🔨️modules/📤️export/🎭️cast/🟦️.ts" "$G/manifest.json" "$G/$1" "$S/🗂️catalog" "${2:-}" 2>&1 | tail -3
python3 "$T/🧰️fm/digests.py" "$G/$1" > "$G/digests-$1.txt"
wc -l "$G/digests-$1.txt"
