#!/bin/zsh
# 🧊️ Every `checkVolumes` rule except `volumes-silhouette` (the guard measures the silhouettes) for people, one log per person in 🗑️generated/REP-B/<dir>: volall.sh <dir> <slug>…   (ALL=1 includes `volumes-silhouette`)
source "${0:A:h}/env.sh"
d="$GEN/$1"; shift; mkdir -p "$d"
cd "$S/📦️packages/🟦️typescript"
for a in "$@"; do
  if [[ -n "$ALL" ]]; then "$B" "$W/vol.ts" "$a" > "$d/$a.log" 2>&1; else "$B" "$W/vol.ts" "$a" --no-silhouette > "$d/$a.log" 2>&1; fi
  echo "$(tail -1 "$d/$a.log")"
done
