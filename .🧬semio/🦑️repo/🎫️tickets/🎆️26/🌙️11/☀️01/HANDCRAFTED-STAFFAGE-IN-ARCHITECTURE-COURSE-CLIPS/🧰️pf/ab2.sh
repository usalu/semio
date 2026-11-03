#!/usr/bin/env bash
# A/B on single casts, one process per cast: A = pre-optimisation measure module and a thermal session per frame (no sharing), B = current tree.
set -u
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/pf"
S="🎓️teaching/🏛️architecture/🧍️staffage"
for id in "$@"; do
  rm -rf "$G/ab-a" "$G/ab-b"
  printf 'A %s: ' "$id"
  bun "$T/🧰️pf/bench.ts" "$G/orig/🔨️modules/📤️export/🎭️cast/🟦️.ts" "$G/manifest.json" "$G/ab-a" "$G/catalog" "$id" 2>&1 | tail -1
  printf 'B %s: ' "$id"
  bun "$T/🧰️pf/bench.ts" "$S/🔨️modules/📤️export/🎭️cast/🟦️.ts" "$G/manifest.json" "$G/ab-b" "$G/catalog" "$id" 2>&1 | tail -1
  python3 "$T/🧰️pf/same_modulo_digits.py" "$G/ab-a/$id" "$G/ab-b/$id" || true
done
