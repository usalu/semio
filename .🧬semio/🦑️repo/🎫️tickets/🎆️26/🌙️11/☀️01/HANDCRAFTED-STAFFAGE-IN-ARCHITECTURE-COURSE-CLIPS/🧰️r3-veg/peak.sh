#!/usr/bin/env bash
# Usage: peak.sh <manifest> <id>...: exports each cast in its own bun process and prints wall time and the exporter's [peak] line.
set -u
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/r3-veg"
S="🎓️teaching/🏛️architecture/🧍️staffage"
manifest="$1"
shift
for id in "$@"; do
  rm -rf "$G/peak/$id"
  start=$(date +%s)
  line=$(bun "$S/🔨️modules/📤️export/🎭️cast/🟦️.ts" --manifest "$manifest" --ids "$id" --out "$G/peak" 2>&1 | grep -E "peak|rror|issue|/casts/" | tr '\n' ' ')
  echo "$id $(( $(date +%s) - start )) s $line"
done
