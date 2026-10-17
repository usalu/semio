#!/bin/zsh
# 🔭️ Quick census of named crates (short names without the semio-s-artifact-/semio-framework- prefix are tried): cen.sh name...
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd /Users/ueli/Documents/semio
args=()
for n in "$@"; do
  m=""
  for cand in "$n" "semio-s-artifact-$n" "semio-framework-$n"; do
    m=$(/usr/bin/grep -h "^$cand," "$F/🗑️generated/hdr-crate-census.csv" | head -1 | cut -d, -f2)
    [ -n "$m" ] && break
  done
  if [ -z "$m" ]; then d=$(/usr/bin/awk -F'\t' -v n="$n" '$2==n{print $3}' "$F/hdr-tools/created.tsv" | head -1); [ -n "$d" ] && m="$d/📦️packages/🦀️rust/Cargo.toml"; fi
  if [ -z "$m" ]; then m="$n"; fi
  args+=("$m")
done
bun "$F/hdr-tools/closure.ts" "${CEN_MODE:-both}" "${args[@]}" | /usr/bin/grep "^#\|^##"
